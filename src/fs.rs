use core::{char::decode_utf16, ffi::c_void};

use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
    Storage::FileSystem::{
        CreateFileW, FILE_APPEND_DATA, FILE_ATTRIBUTE_NORMAL, FILE_END, FILE_SHARE_READ,
        OPEN_ALWAYS, SetFilePointer, WriteFile,
    },
    System::Console::{ATTACH_PARENT_PROCESS, AttachConsole, GetStdHandle, STD_OUTPUT_HANDLE},
};

pub struct File {
    handle: *mut c_void,
}

impl File {
    pub fn open_stdout() -> Self {
        File {
            handle: unsafe {
                match AttachConsole(ATTACH_PARENT_PROCESS) {
                    0 => INVALID_HANDLE_VALUE,
                    _ => GetStdHandle(STD_OUTPUT_HANDLE),
                }
            },
        }
    }

    pub fn open_write(file: *const u16) -> Self {
        File {
            handle: unsafe {
                CreateFileW(
                    file,
                    FILE_APPEND_DATA,
                    FILE_SHARE_READ,
                    core::ptr::null_mut(),
                    OPEN_ALWAYS,
                    FILE_ATTRIBUTE_NORMAL,
                    0 as HANDLE,
                )
            },
        }
    }

    pub fn close(self) {
        if self.handle == INVALID_HANDLE_VALUE {
            return;
        }
        unsafe { CloseHandle(self.handle) };
    }

    pub fn write_bytes(&self, message: &[u8]) {
        if self.handle == INVALID_HANDLE_VALUE {
            return;
        }
        let mut written: u32 = 0;
        unsafe {
            SetFilePointer(self.handle, 0, core::ptr::null_mut(), FILE_END);
            WriteFile(
                self.handle,
                message.as_ptr(),
                message.len() as u32,
                &mut written,
                core::ptr::null_mut(),
            );
        }
    }

    pub fn write_str(&self, message: &str) {
        self.write_bytes(message.as_bytes());
    }

    pub fn write_iter_u16<T: Iterator<Item = u16>>(&self, message: T) {
        let mut utf8_buf = [0u8; 4];
        decode_utf16(message).for_each(|result| {
            self.write_bytes(
                result
                    .unwrap_or(core::char::REPLACEMENT_CHARACTER)
                    .encode_utf8(&mut utf8_buf)
                    .as_bytes(),
            )
        });
    }
}
