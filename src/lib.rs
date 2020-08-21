#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

use crate::{include::MmIsAddressValid, process::Process, string::create_unicode_string};
use core::panic::PanicInfo;

pub mod include;
pub mod log;
pub mod process;
pub mod string;

/// When using the alloc crate it seems like it does some unwinding. Adding this
/// export satisfies the compiler but may introduce undefined behaviour when a
/// panic occurs.
#[no_mangle]
pub extern "system" fn __CxxFrameHandler3(_: *mut u8, _: *mut u8, _: *mut u8, _: *mut u8) -> i32 { unimplemented!() }

#[global_allocator]
static GLOBAL: kernel_alloc::KernelAlloc = kernel_alloc::KernelAlloc;

/// Explanation can be found here: https://github.com/Trantect/win_driver_example/issues/4
#[export_name = "_fltused"]
static _FLTUSED: i32 = 0;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

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

    // Logger
    //

    kernel_print::kernel_println!("BEFORE");
    kernel_print::kernel_dbg!(2 + 2);
    kernel_print::kernel_println!("{} + {} = {}", 2, 2, 2 + 2);
    kernel_print::kernel_print!("{} + {} = {}\n", 2, 2, 2 + 2);
    kernel_print::kernel_println!("AFTER");

    0 /* STATUS_SUCCESS */
}
