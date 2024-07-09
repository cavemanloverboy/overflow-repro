use core::mem::size_of;

mod syscalls;
use syscalls::*;

macro_rules! log {
    ($($arg:tt)*) => (sol_log(&format!($($arg)*)))
}

#[no_mangle]
pub unsafe extern "C" fn entrypoint(mut input: *mut u8) -> u32 {
    // In this simple reproducer, there are no input accounts
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
#[no_mangle]
fn corrupt() {
    let mut oops = [0_u8; 4243];
    oops[0] = 69;
    oops[1] = 42;
    oops[2] = 88;
    core::hint::black_box(oops);
}
