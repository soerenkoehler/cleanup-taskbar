#![no_std]
#![no_main]

mod buffer;
mod fs;
mod process;

use core::{panic::PanicInfo, sync::atomic::{AtomicPtr, Ordering}};

use windows_sys::Win32::System::Threading::ExitProcess;

use crate::{fs::File, process::{create_process, get_arguments, search_path}};

static LOG: AtomicPtr<File> = AtomicPtr::new(core::ptr::null_mut::<File>());

#[unsafe(no_mangle)]
pub extern "C" fn WinMain() -> ! {
    LOG.store(&mut File::open_stdout() as *mut File, Ordering::SeqCst);

    create_process(
        search_path("powershell.exe")
            .append_str(" ")
            .append_ptr_u16(get_arguments()),
    );

    unsafe { ExitProcess(0) };
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { ExitProcess(1) };
}
