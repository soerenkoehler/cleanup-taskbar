use core::{mem::zeroed, sync::atomic::Ordering};

use windows_sys::Win32::{
    Foundation::{self, GetLastError},
    Storage::FileSystem::SearchPathW,
    System::{
        Environment::GetCommandLineW,
        Threading::{
            CREATE_NO_WINDOW, CreateProcessW, PROCESS_INFORMATION, STARTF_USESHOWWINDOW,
            STARTUPINFOW,
        },
    },
    UI::WindowsAndMessaging::SW_HIDE,
};

use crate::{LOG, buffer::String};

const MAX_PATH: usize = Foundation::MAX_PATH as usize;

pub fn get_arguments() -> *const u16 {
    const SPACE: u16 = b' ' as u16;
    const QUOTE: u16 = b'"' as u16;

    let mut cmd_line: *const u16 = unsafe { GetCommandLineW() };
    let mut in_quotes = false;

    // Iterate until we find the end of the first argument
    loop {
        let c = unsafe { *cmd_line };

        if c == 0 {
            break;
        } else if c == SPACE && !in_quotes {
            break;
        } else if c == QUOTE {
            in_quotes = !in_quotes;
        }

        unsafe { cmd_line = cmd_line.add(1) };
    }

    // Move past any trailing spaces to reach the actual start of the arguments
    unsafe {
        while *cmd_line == b' ' as u16 {
            cmd_line = cmd_line.add(1);
        }
    }

    cmd_line
}

pub fn search_path(s: &str) -> String<MAX_PATH> {
    let log = unsafe { &*LOG.load(Ordering::SeqCst) };

    let mut full_path = [0u16; MAX_PATH];
    let mut file_part = core::ptr::null_mut();

    let lpfilename = String::<MAX_PATH>::new().append_str(s);
    log.write_str("filename: ");
    log.write_iter_u16(
        String::<MAX_PATH>::new()
            .append_iter_u16(lpfilename.as_iter())
            .as_iter(),
    );
    log.write_str("\r\n");
    log.write_str("filename: ");
    log.write_iter_u16(
        String::<MAX_PATH>::new()
            .append_iter_u16(lpfilename.as_iter())
            .as_iter(),
    );
    log.write_str("\r\n");

    let result = unsafe {
        SearchPathW(
            core::ptr::null(),
            lpfilename.as_ptr(),
            core::ptr::null(),
            full_path.len() as u32,
            full_path.as_mut_ptr(),
            &mut file_part,
        ) as usize
    };

    if result == 0 || result >= full_path.len() {
        let error = unsafe { GetLastError() };
        log.write_str("error: ");
        log.write_str(itoa::Buffer::new().format(error));
        log.write_str("\r\n");
        panic!("File search error");
    }

    log.write_str("result: ");
    log.write_iter_u16(
        String::<MAX_PATH>::new()
            .append_iter_u16(full_path[..result].into_iter().cloned())
            .as_iter(),
    );
    log.write_str("\r\n");

    String::new().append_iter_u16(full_path[..result].into_iter().cloned())
}

pub fn create_process<const CAPACITY: usize>(cmd: String<CAPACITY>) {
    let mut si: STARTUPINFOW = unsafe { zeroed() };
    si.cb = size_of::<STARTUPINFOW>() as u32;
    si.dwFlags = STARTF_USESHOWWINDOW;
    si.wShowWindow = SW_HIDE as u16;
    let mut pi: PROCESS_INFORMATION = unsafe { zeroed() };

    unsafe {
        CreateProcessW(
            core::ptr::null(),
            cmd.as_mut_ptr(),
            core::ptr::null(),
            core::ptr::null(),
            0,
            CREATE_NO_WINDOW,
            core::ptr::null(),
            core::ptr::null(),
            &si,
            &mut pi,
        );
    }
}
