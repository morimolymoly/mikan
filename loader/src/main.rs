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

use common::{PixelFormat, FrameBufferConfig};

#[allow(dead_code)]
struct MemoryMap<'a> {
    buffer_size: usize,
    buffer: &'a mut [u8],
    map_size: usize,
    map_key: i64,
    descriptor_size: usize,
    descriptor_version: usize,
}

fn write_frame_buffer_and_create_frame_buffer_config(st: &mut SystemTable<Boot>) ->  FrameBufferConfig{
    let go = st.boot_services().locate_protocol::<GraphicsOutput>().unwrap().unwrap();
    let go = unsafe {&mut *go.get()};
    let (x, y) = go.current_mode_info().resolution();
    writeln!(st.stdout(), "Resolution: {} x {}", x, y).unwrap();
    let frame_buffer_size = go.frame_buffer().size();
    let frame_buffer = go.frame_buffer().as_mut_ptr();
    let frame_buffer = unsafe { core::slice::from_raw_parts_mut(frame_buffer, frame_buffer_size as usize)};
    for i in 0..go.frame_buffer().size() {
        frame_buffer[i] = 255;
    }

    let frame_buffer_adress = go.frame_buffer().as_mut_ptr() as u64;
    let frame_buffer_size = go.frame_buffer().size() as u64;

    use uefi::proto::console::gop::PixelFormat::{Rgb, Bgr, Bitmask, BltOnly};

    let pixel_format = match go.current_mode_info().pixel_format() {
        Rgb => PixelFormat::RGBResv8BitPerColor,
        Bgr => PixelFormat::BGRResv8BitPerColor,
        Bitmask => {panic!()},
        BltOnly => {panic!()},
    };

    let frame_buffer_config = FrameBufferConfig {
        frame_buffer: go.frame_buffer().as_mut_ptr(),
        frame_buffer_size: go.frame_buffer().size(),
        pixels_per_scan_line: go.current_mode_info().stride() as u32,
        horizontal_resolution: x as u32,
        vertical_resolution: y as u32,
        pixel_format: pixel_format,
    };
    return frame_buffer_config;
}

fn get_memmap(st: &mut SystemTable<Boot>) {
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
}

fn get_kernel_entry_point_address(st: &mut SystemTable<Boot>) -> u64{
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
    let _ = st.boot_services().allocate_pages(AllocateType::Address(kernel_base_addr), 
    MemoryType::LOADER_DATA, 
    num_pages
    ).unwrap_success();

    let kernel_mem_start = kernel_base_addr as *mut u8;
    let kernel_mem = unsafe { core::slice::from_raw_parts_mut(kernel_mem_start, kernel_size as usize)};
    let _read_size = kernel_file.read(kernel_mem).unwrap().unwrap();

    let kernel_mem_p = (kernel_base_addr + 24) as *mut u8;
    let entry_point_address_buf = unsafe { core::slice::from_raw_parts(kernel_mem_p, 8)};
    let entry_point_address = byteorder::LittleEndian::read_u64(entry_point_address_buf);
    writeln!(st.stdout(), "entry point address: ={:x}", entry_point_address).unwrap();

    return entry_point_address;
}

#[entry]
fn efi_main(handle: Handle, mut st: SystemTable<Boot>) -> Status {
    writeln!(st.stdout(), "Hello, World!").unwrap();

    // get frame buffer config
    let frame_buffer_config = write_frame_buffer_and_create_frame_buffer_config(&mut st);
    
    // get memory maps
    get_memmap(&mut st);


    // load kernel and get entrypoint of that and jump to it
    let entry_point_address = get_kernel_entry_point_address(&mut st);
    let kernel_main = unsafe {
        let f: extern "efiapi" fn(FrameBufferConfig) -> ! = core::mem::transmute(entry_point_address);
        f
    };

    writeln!(st.stdout(), "jump to kernel!").unwrap();

    let mut buffer_exit_boot_service = [0;10000];
    st.exit_boot_services(handle, &mut buffer_exit_boot_service).unwrap_success();

    kernel_main(frame_buffer_config);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}