#![no_std]
#![no_main]

mod buffer;
mod fs;
mod process;

use core::panic::PanicInfo;

use windows_sys::Win32::System::Threading::ExitProcess;

use crate::process::{create_process, get_arguments, search_path};

#[unsafe(no_mangle)]
pub extern "C" fn WinMain() -> ! {
    create_process(
        search_path("powershell.exe")
            .append_str(" ")
            .append_ptr_u16(get_arguments()),
    );
    // Exit without returning (since we are the entry point)
    unsafe { ExitProcess(0) };
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { ExitProcess(1) };
}
