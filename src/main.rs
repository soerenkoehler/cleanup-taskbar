#![no_std]
#![no_main]

use core::{
    mem::{size_of, zeroed},
    panic::PanicInfo,
};

use windows_sys::Win32::{
    Storage::FileSystem::SearchPathW,
    System::{
        Environment::GetCommandLineW,
        Threading::{
            CREATE_NO_WINDOW, CreateProcessW, ExitProcess, PROCESS_INFORMATION,
            STARTF_USESHOWWINDOW, STARTUPINFOW,
        },
    },
    UI::WindowsAndMessaging::SW_HIDE,
};

#[unsafe(no_mangle)]
pub extern "C" fn WinMain() -> ! {
    create_process(
        search_path("powershell.exe")
            .append_str(" ")
            .append_ptr_u16(get_arguments())
            .as_mut_ptr(),
    );
    // Exit without returning (since we are the entry point)
    unsafe { ExitProcess(0) };
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { ExitProcess(1) };
}

fn get_arguments() -> *const u16 {
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

fn create_process(cmd: *mut u16) {
    let mut si: STARTUPINFOW = unsafe { zeroed() };
    si.cb = size_of::<STARTUPINFOW>() as u32;
    si.dwFlags = STARTF_USESHOWWINDOW;
    si.wShowWindow = SW_HIDE as u16;
    let mut pi: PROCESS_INFORMATION = unsafe { zeroed() };

    unsafe {
        CreateProcessW(
            core::ptr::null(),
            cmd,
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

fn search_path(s: &str) -> ZeroTerminatedBuffer<260> {
    let mut full_path = [0u16; 260]; // MAX_PATH is 260
    let mut file_part = core::ptr::null_mut();

    let result = unsafe {
        SearchPathW(
            core::ptr::null(), // Use default search path (includes %PATH%)
            ZeroTerminatedBuffer::<256>::new(s).as_ptr(), // The file to search for
            core::ptr::null(), // No specific extension (it's already in the name)
            full_path.len() as u32,
            full_path.as_mut_ptr(),
            &mut file_part, // Receives pointer to the filename part of the path
        ) as usize
    };

    if result == 0 || result > full_path.len() {
        panic!("File search error");
    }

    ZeroTerminatedBuffer {
        data: full_path,
        len: result,
    }
}

struct ZeroTerminatedBuffer<const CAPACITY: usize> {
    data: [u16; CAPACITY],
    len: usize,
}

impl<const CAPACITY: usize> ZeroTerminatedBuffer<CAPACITY> {
    fn new(s: &str) -> Self {
        ZeroTerminatedBuffer {
            data: [0u16; CAPACITY],
            len: 0,
        }
        .append_str(s)
        .append_u16(0)
    }

    fn append_str(self, s: &str) -> Self {
        self.append_iter_u16(s.encode_utf16())
    }

    fn append_iter_u16<T: Iterator<Item = u16>>(self, mut iter: T) -> Self {
        match iter.next() {
            Some(c) => self.append_u16(c).append_iter_u16(iter),
            None => self,
        }
    }

    fn append_ptr_u16(mut self, mut s: *const u16) -> Self {
        unsafe {
            while *s != 0 {
                self = self.append_u16(*s);
                s = s.add(1);
            }
        }
        self
    }

    fn append_u16(mut self, c: u16) -> Self {
        if self.len >= CAPACITY {
            panic!("Buffer overflow");
        }

        // last char in buffer is always zero
        self.data[self.len] = if self.len == CAPACITY - 1 { 0 } else { c };

        // increase size if not a zero
        if self.data[self.len] != 0 {
            self.len += 1;
        }

        self
    }

    fn as_ptr(self) -> *const u16 {
        self.data.as_ptr()
    }

    fn as_mut_ptr(mut self) -> *mut u16 {
        self.data.as_mut_ptr()
    }
}
