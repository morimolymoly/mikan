#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;

#[no_mangle]
extern "efiapi" fn kernel_main() -> ! {
    loop{
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
