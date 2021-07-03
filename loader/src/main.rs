#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use byteorder::ByteOrder;
use uefi::prelude::*;
use uefi::proto::media::file::File;
use uefi::table::boot::{AllocateType, MemoryType};
use uefi::table::boot;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::media::file::{Directory, FileHandle, FileMode, FileAttribute, RegularFile, FileInfo, FileProtocolInfo};
use core::mem::size_of;
use core::mem::size_of_val;
use core::panic::PanicInfo;
use core::fmt::Write;

struct MemoryMap<'a> {
    buffer_size: usize,
    buffer: &'a mut [u8],
    map_size: usize,
    map_key: i64,
    descriptor_size: usize,
    descriptor_version: usize,
}

fn save_memmap(file: &mut RegularFile, _memmap: MemoryMap) {
    file.flush().unwrap_success();
    let header: &[u8] = "aiueo".as_bytes();
    file.write(header).unwrap_success();
    file.flush().unwrap_success();
}

#[entry]
fn efi_main(_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    writeln!(st.stdout(), "Hello, World!").unwrap();

    // Getting Memory map
    let mut memmap_buf: [u8; 4096*4] = [0; 4096*4];
    let  memmap = MemoryMap {
        buffer_size: size_of_val(&memmap_buf),
        buffer: &mut memmap_buf,
        map_size: 0,
        map_key: 0,
        descriptor_size: 0,
        descriptor_version: 0
    };
    st.boot_services().memory_map(memmap.buffer);

    // save memmap
    let fs = st.boot_services().locate_protocol::<SimpleFileSystem>().unwrap_success();
    let fs = unsafe { &mut *fs.get() };
    let mut root = fs.open_volume().unwrap_success();
    
    let memmap_handle = root.open("memmap",  FileMode::CreateReadWrite, FileAttribute::empty()).unwrap_success();
    let mut memmap_file: RegularFile;
    unsafe {
        memmap_file = RegularFile::new(memmap_handle);
    }

    save_memmap(&mut memmap_file, memmap);
    memmap_file.close();

    // read kernel
    let kernel_file_handle = root.open("kernel", FileMode::Read, FileAttribute::empty()).unwrap_success();
    let mut kernel_file: RegularFile;
    unsafe {
        kernel_file = RegularFile::new(kernel_file_handle);
    }
    
    const file_info_size: usize = 1000;
    let mut buffer_kernel_info = [0;file_info_size];
    let kernel_info: &mut FileInfo = kernel_file.get_info(&mut buffer_kernel_info).unwrap_success();
    let kernel_size = kernel_info.file_size();
    let num_pages = (kernel_size as usize +  0xfff) / 0x1000;

    let kernel_base_addr = 0x100000;
    st.boot_services().allocate_pages(AllocateType::Address(kernel_base_addr), 
    MemoryType::LOADER_DATA, 
    num_pages
    );

    let kernel_mem_start = kernel_base_addr as *mut u8;
    let kernel_mem = unsafe { core::slice::from_raw_parts_mut(kernel_mem_start, kernel_size as usize)};
    kernel_file.read(kernel_mem);

    let kernel_mem_p = (kernel_base_addr + 24) as *mut u8;
    let entry_point_address_buf = unsafe { core::slice::from_raw_parts(kernel_mem_p, 8)};
    let entry_point_address = byteorder::LittleEndian::read_u64(entry_point_address_buf);
    writeln!(st.stdout(), "entry point address: ={:x}", entry_point_address).unwrap();

    kernel_file.read(kernel_mem);


    st.exit_boot_services(_handle, &mut buffer_kernel_info);

    let kernel_main = unsafe {
        let f: extern "efiapi" fn() -> ! = core::mem::transmute(entry_point_address);
        f
    };
    kernel_main();
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}