#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use uefi::prelude::*;
use uefi::proto::media::file::File;
use uefi::table::boot;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::media::file::{Directory, FileHandle, FileMode, FileAttribute, RegularFile};
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
    let header: &[u8] = "test".as_bytes();
    file.write(header).unwrap_success();
    //file.flush().unwrap_success();
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

    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}