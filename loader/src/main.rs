#![feature(abi_efiapi)]
#![no_std]
#![no_main]

use uefi::prelude::*;
use core::panic::PanicInfo;
use core::fmt::Write;

#[entry]
fn efi_main(_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    writeln!(st.stdout(), "Hello, World!").unwrap();
    
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}