#![no_std]
#![no_main]

#![feature(abi_efiapi)]
#![feature(asm)]

use byteorder::ByteOrder;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::media::file::File;
use uefi::table::boot::{AllocateType, MemoryType};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::media::file::{FileMode, FileAttribute, RegularFile, FileInfo};
use core::mem::size_of_val;
use core::panic::PanicInfo;
use core::fmt::Write;

#[allow(dead_code)]
struct MemoryMap<'a> {
    buffer_size: usize,
    buffer: &'a mut [u8],
    map_size: usize,
    map_key: i64,
    descriptor_size: usize,
    descriptor_version: usize,
}

#[entry]
fn efi_main(_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    writeln!(st.stdout(), "Hello, World!").unwrap();

    // Getting Memory map
    let mut memmap_buf = [0; 4096*4];
    let  memmap = MemoryMap {
        buffer_size: size_of_val(&memmap_buf),
        buffer: &mut memmap_buf,
        map_size: 0,
        map_key: 0,
        descriptor_size: 0,
        descriptor_version: 0
    };
    let _mem_map_key = st.boot_services().memory_map(memmap.buffer).unwrap_success();

    // open root dir
    let fs = st.boot_services().locate_protocol::<SimpleFileSystem>().unwrap_success();
    let fs = unsafe { &mut *fs.get() };
    let mut root = fs.open_volume().unwrap_success();

    // read kernel
    let kernel_file_handle = root.open("kernel", FileMode::Read, FileAttribute::empty()).unwrap_success();
    let mut kernel_file: RegularFile;
    unsafe {
        kernel_file = RegularFile::new(kernel_file_handle);
    }
    
    const FILE_INFO_SIZE: usize = 1000;
    let mut buffer_kernel_info = [0;FILE_INFO_SIZE];
    let kernel_info: &mut FileInfo = kernel_file.get_info(&mut buffer_kernel_info).unwrap_success();
    let kernel_size = kernel_info.file_size();
    let num_pages = (kernel_size as usize +  0xfff) / 0x1000;

    let kernel_base_addr = 0x100000;
    let num_pages_actually = st.boot_services().allocate_pages(AllocateType::Address(kernel_base_addr), 
    MemoryType::LOADER_DATA, 
    num_pages
    ).unwrap().unwrap();

    if num_pages_actually != num_pages as u64 {
        writeln!(st.stdout(), "not the same").unwrap();
        unsafe {
            asm!("hlt");
        }
    }

    let kernel_mem_start = kernel_base_addr as *mut u8;
    let kernel_mem = unsafe { core::slice::from_raw_parts_mut(kernel_mem_start, kernel_size as usize)};
    let _read_size = kernel_file.read(kernel_mem).unwrap().unwrap();

    let kernel_mem_p = (kernel_base_addr + 24) as *mut u8;
    let entry_point_address_buf = unsafe { core::slice::from_raw_parts(kernel_mem_p, 8)};
    let entry_point_address = byteorder::LittleEndian::read_u64(entry_point_address_buf);
    writeln!(st.stdout(), "entry point address: ={:x}", entry_point_address).unwrap();

    let kernel_main = unsafe {
        let f: extern "efiapi" fn(u64, u64) -> ! = core::mem::transmute(entry_point_address);
        f
    };

    let go = st.boot_services().locate_protocol::<GraphicsOutput>().unwrap().unwrap();
    let go = unsafe {&mut *go.get()};
    let frame_buffer_adress = go.frame_buffer().as_mut_ptr() as u64;
    let frame_buffer_size = go.frame_buffer().size() as u64;

    let _st_runtime = st.exit_boot_services(_handle, &mut buffer_kernel_info).unwrap();
    
    kernel_main(frame_buffer_adress, frame_buffer_size);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}