#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

use crate::{include::MmIsAddressValid, process::Process, string::create_unicode_string};
use kernel_log::KernelLogger;
use log::LevelFilter;

pub mod include;
pub mod process;
pub mod string;

#[global_allocator]
static GLOBAL: kernel_alloc::KernelAlloc = kernel_alloc::KernelAlloc;

#[no_mangle]
pub extern "system" fn driver_entry() -> u32 {
    KernelLogger::init(LevelFilter::Info).unwrap();

    // MmIsAddressValid
    //
    let is_valid = unsafe { MmIsAddressValid(0 as _) };
    log::info!("MmIsAddressValid(0) returned {}", is_valid as u64);

    // String
    //
    let string = create_unicode_string(obfstr::wide!("Hello World!\0"));
    log::info("String buffer is at: {:x}", string.Buffer);

    // Process
    //
    let process = Process::by_id(4 as _);
    log::info!("Process found: {}", process.is_some() as u64);

    0 /* STATUS_SUCCESS */
}
