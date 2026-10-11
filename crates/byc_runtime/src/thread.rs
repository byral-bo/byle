use std::thread;

/// Spawn a thread running `func(arg)`. Returns an opaque handle (i64 pointer).
/// `func` must be an `extern "C" fn(i64) -> i64` — guaranteed by codegen.
#[unsafe(no_mangle)]
pub extern "C" fn by_spawn_thread(func: Option<unsafe extern "C" fn(i64) -> i64>, arg: i64) -> i64 {
    let Some(func) = func else {
        return 0;
    };
    let handle = thread::spawn(move || unsafe { func(arg) });
    Box::into_raw(Box::new(handle)) as i64
}

/// Join a thread spawned by `by_spawn_thread`. Returns the i64 result.
#[unsafe(no_mangle)]
pub extern "C" fn by_join_thread(handle_ptr: i64) -> i64 {
    if handle_ptr == 0 {
        return 0;
    }
    let handle = unsafe { Box::from_raw(handle_ptr as *mut thread::JoinHandle<i64>) };
    handle.join().unwrap_or(0)
}
