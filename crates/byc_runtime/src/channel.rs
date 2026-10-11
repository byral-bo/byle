use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};

use crate::error::KlError;

static NEXT_CHANNEL_ID: AtomicU64 = AtomicU64::new(1);

struct ChannelInner<T> {
    buffer: VecDeque<T>,
    capacity: usize,
    closed: bool,
}

pub struct Channel<T> {
    id: u64,
    capacity: usize,
    inner: Arc<Mutex<ChannelInner<T>>>,
    send_notify: Arc<Condvar>,
    recv_notify: Arc<Condvar>,
    closed: Arc<AtomicBool>,
}

impl<T> Channel<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            id: NEXT_CHANNEL_ID.fetch_add(1, Ordering::Relaxed),
            capacity,
            inner: Arc::new(Mutex::new(ChannelInner {
                buffer: VecDeque::with_capacity(capacity),
                capacity,
                closed: false,
            })),
            send_notify: Arc::new(Condvar::new()),
            recv_notify: Arc::new(Condvar::new()),
            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn send(&self, value: T) -> Result<(), KlError> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| KlError::new("channel lock poisoned"))?;
        while inner.buffer.len() >= inner.capacity {
            if inner.closed {
                return Err(KlError::new("channel closed"));
            }
            inner = self
                .send_notify
                .wait(inner)
                .map_err(|_| KlError::new("channel wait poisoned"))?;
        }
        inner.buffer.push_back(value);
        self.recv_notify.notify_one();
        Ok(())
    }

    pub fn recv(&self) -> Result<T, KlError> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| KlError::new("channel lock poisoned"))?;
        loop {
            if let Some(value) = inner.buffer.pop_front() {
                self.send_notify.notify_one();
                return Ok(value);
            }
            if inner.closed {
                return Err(KlError::new("channel closed"));
            }
            inner = self
                .recv_notify
                .wait(inner)
                .map_err(|_| KlError::new("channel wait poisoned"))?;
        }
    }

    pub fn try_send(&self, value: T) -> Result<(), KlError> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| KlError::new("channel lock poisoned"))?;
        if inner.closed {
            return Err(KlError::new("channel closed"));
        }
        if inner.buffer.len() >= inner.capacity {
            return Err(KlError::with_code("channel full", 1));
        }
        inner.buffer.push_back(value);
        self.recv_notify.notify_one();
        Ok(())
    }

    pub fn try_recv(&self) -> Result<T, KlError> {
        let mut inner = self
            .inner
            .lock()
            .map_err(|_| KlError::new("channel lock poisoned"))?;
        if let Some(value) = inner.buffer.pop_front() {
            self.send_notify.notify_one();
            return Ok(value);
        }
        if inner.closed {
            return Err(KlError::new("channel closed"));
        }
        Err(KlError::with_code("channel empty", 2))
    }

    pub fn close(&self) {
        self.closed.store(true, Ordering::Relaxed);
        {
            let mut inner = self
                .inner
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            inner.closed = true;
        }
        self.send_notify.notify_all();
        self.recv_notify.notify_all();
    }

    pub fn len(&self) -> usize {
        self.inner.lock().map(|i| i.buffer.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Relaxed)
    }
}

impl<T> Clone for Channel<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            capacity: self.capacity,
            inner: Arc::clone(&self.inner),
            send_notify: Arc::clone(&self.send_notify),
            recv_notify: Arc::clone(&self.recv_notify),
            closed: Arc::clone(&self.closed),
        }
    }
}

unsafe impl<T: Send> Send for Channel<T> {}
unsafe impl<T: Send> Sync for Channel<T> {}

// --- C-compatible wrappers for Byle FFI ---

#[unsafe(no_mangle)]
pub extern "C" fn by_channel_new(capacity: i64) -> i64 {
    // This implementation is buffered; avoid a permanent send deadlock and
    // invalid huge allocations for zero or negative FFI values.
    let capacity = usize::try_from(capacity).unwrap_or(1).max(1);
    let chan = Channel::<i64>::new(capacity);
    Box::into_raw(Box::new(chan)) as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn by_channel_send(chan_ptr: i64, val: i64) -> i64 {
    let chan = unsafe { &*(chan_ptr as *const Channel<i64>) };
    match chan.send(val) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn by_channel_recv(chan_ptr: i64) -> i64 {
    let chan = unsafe { &*(chan_ptr as *const Channel<i64>) };
    chan.recv().unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn by_channel_close(chan_ptr: i64) {
    let chan = unsafe { &*(chan_ptr as *const Channel<i64>) };
    chan.close();
}

#[unsafe(no_mangle)]
pub extern "C" fn by_channel_len(chan_ptr: i64) -> i64 {
    let chan = unsafe { &*(chan_ptr as *const Channel<i64>) };
    chan.len() as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn by_channel_free(chan_ptr: i64) {
    if chan_ptr != 0 {
        unsafe {
            drop(Box::from_raw(chan_ptr as *mut Channel<i64>));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{by_channel_free, by_channel_new, by_channel_send};

    #[test]
    fn ffi_channel_normalizes_invalid_capacity() {
        for capacity in [0, -1] {
            let channel = by_channel_new(capacity);
            assert_ne!(channel, 0);
            assert_eq!(by_channel_send(channel, 42), 0);
            by_channel_free(channel);
        }
    }
}
