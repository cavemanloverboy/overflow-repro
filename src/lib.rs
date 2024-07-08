use core::mem::size_of;

extern "C" {
    fn sol_log_(message: *const u8, len: u64);
    fn sol_set_return_data(data: *const u8, length: u64);
}

fn sol_log(s: &str) {
    unsafe { sol_log_(s.as_bytes().as_ptr(), s.as_bytes().len() as u64) }
}

fn set_return(d: &[u8]) {
    unsafe { sol_set_return_data(d.as_ptr(), d.len() as u64) };
}

macro_rules! log {
    ($($arg:tt)*) => (sol_log(&format!($($arg)*)))
}

#[no_mangle]
pub unsafe extern "C" fn entrypoint(mut input: *mut u8) -> u32 {
    // In this simple repoducer, there are no input accounts
    input = input.add(size_of::<u64>());

    // Instruction data length should be 3
    assert_eq!(*input.cast::<u64>(), 3, "unexpected input length");
    input = input.add(size_of::<u64>());

    // Get instruction data
    let data = core::slice::from_raw_parts(input, 3);

    // Copy data to stack and corrupt
    let data_on_stack = [data[0], data[1], data[2]];
    corrupt();

    // Log and set as return
    log!("data {:?}", &data_on_stack);
    set_return(&data_on_stack);

    0
}

#[inline(never)]
fn corrupt() {
    let mut oops = [0_u8; 8192];
    oops[0..8192].fill(5);
    core::hint::black_box(oops);
}
