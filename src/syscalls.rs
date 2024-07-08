#[cfg(target_os = "solana")]
extern "C" {
    fn sol_log_(message: *const u8, len: u64);
    fn sol_set_return_data(data: *const u8, length: u64);
}

pub fn sol_log(_s: &str) {
    #[cfg(target_os = "solana")]
    unsafe {
        sol_log_(_s.as_bytes().as_ptr(), _s.as_bytes().len() as u64)
    }
    #[cfg(not(target_os = "solana"))]
    println!("{}", _s);
}

pub fn set_return(_d: &[u8]) {
    #[cfg(target_os = "solana")]
    unsafe {
        sol_set_return_data(_d.as_ptr(), _d.len() as u64)
    };
}
