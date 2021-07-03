#![no_std]
#![no_main]

#![feature(asm)]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;

#[no_mangle]
extern "efiapi" fn kernel_main(frame_buffer_base: u64, frame_buffer_size: u64) -> ! {
    let frame_buffer_start = frame_buffer_base as *mut u8;
    let frame_buffer = unsafe { core::slice::from_raw_parts_mut(frame_buffer_start, frame_buffer_size as usize)};
    
    for i in 0 .. frame_buffer_size as usize {
        frame_buffer[i] = (i as u8) % 255;
    }
    
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
