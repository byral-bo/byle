use std::sync::atomic::{AtomicI64, Ordering};

#[repr(C)]
struct Header {
    refcount: AtomicI64,
    size: i64,
}

const HEADER_SIZE: usize = std::mem::size_of::<Header>();

#[unsafe(no_mangle)]
pub extern "C" fn by_alloc(size: i64) -> *mut u8 {
    if size <= 0 {
        return std::ptr::null_mut();
    }
    let Ok(size) = usize::try_from(size) else {
        return std::ptr::null_mut();
    };
    let Some(total) = size.checked_add(HEADER_SIZE) else {
        return std::ptr::null_mut();
    };
    // Ensure alignment for Header (AtomicI64 requires 8-byte alignment on aarch64)
    let Ok(layout) = std::alloc::Layout::from_size_align(total, std::mem::align_of::<Header>())
    else {
        return std::ptr::null_mut();
    };
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
        return ptr;
    }
    let header = ptr as *mut Header;
    unsafe {
        (*header).refcount = AtomicI64::new(1);
        (*header).size = size as i64;
    }
    unsafe { ptr.add(HEADER_SIZE) }
}

#[unsafe(no_mangle)]
pub extern "C" fn by_free(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let header_ptr = ptr.sub(HEADER_SIZE) as *mut Header;
        let Ok(size) = usize::try_from((*header_ptr).size) else {
            return;
        };
        let Some(total) = size.checked_add(HEADER_SIZE) else {
            return;
        };
        if let Ok(layout) =
            std::alloc::Layout::from_size_align(total, std::mem::align_of::<Header>())
        {
            std::alloc::dealloc(header_ptr as *mut u8, layout);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn by_retain(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let header = ptr.sub(HEADER_SIZE) as *mut Header;
        (*header).refcount.fetch_add(1, Ordering::Relaxed);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn by_release(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let header = ptr.sub(HEADER_SIZE) as *mut Header;
        let prev = (*header).refcount.fetch_sub(1, Ordering::Release);
        if prev == 1 {
            std::sync::atomic::fence(Ordering::Acquire);
            by_free(ptr);
        }
    }
}
