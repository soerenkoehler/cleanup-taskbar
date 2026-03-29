use core::mem::zeroed;

use windows_sys::Win32::{
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

use crate::buffer::ZeroTerminatedBuffer;

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

pub fn search_path(s: &str) -> ZeroTerminatedBuffer<260> {
    let mut full_path = [0u16; 260]; // MAX_PATH is 260
    let mut file_part = core::ptr::null_mut();

    let result = unsafe {
        SearchPathW(
            core::ptr::null(), // Use default search path (includes %PATH%)
            ZeroTerminatedBuffer::<256>::new().append_str(s).as_ptr(), // The file to search for
            core::ptr::null(), // No specific extension (it's already in the name)
            full_path.len() as u32,
            full_path.as_mut_ptr(),
            &mut file_part, // Receives pointer to the filename part of the path
        ) as usize
    };

    if result == 0 || result >= full_path.len() {
        panic!("File search error");
    }

    ZeroTerminatedBuffer::new().append_iter_u16(full_path.into_iter())
}

pub fn create_process<const CAPACITY: usize>(cmd: ZeroTerminatedBuffer<CAPACITY>) {
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
