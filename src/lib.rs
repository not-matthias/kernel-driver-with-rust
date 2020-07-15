#![no_std]

use crate::include::MmIsAddressValid;
use crate::process::Process;
use crate::string::create_unicode_string;
use core::panic::PanicInfo;

pub mod include;
pub mod log;
pub mod process;
pub mod string;

/// Explanation can be found here: https://github.com/Trantect/win_driver_example/issues/4
#[used]
#[no_mangle]
static _fltused: i32 = 0;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "system" fn driver_entry() -> u32 {
    // MmIsAddressValid
    //
    let is_valid = unsafe { MmIsAddressValid(0 as _) };
    log!("MmIsAddressValid(0) returned %i", is_valid as u64);

    // String
    //
    let string = create_unicode_string(obfstr::wide!("Hello World!\0"));
    log!("String: %ws", string.Buffer);

    // Process
    //
    let process = Process::by_id(4 as _);
    log!("Process found: %i", process.is_some() as u64);

    0 /* STATUS_SUCCESS */
}
