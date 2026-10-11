// byc_runtime::panic — Panic handler

pub fn by_panic(message: &str) -> ! {
    eprintln!("BYLE PANIC: {}", message);
    std::process::abort();
}
