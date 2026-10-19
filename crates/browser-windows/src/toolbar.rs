//! Native Win32 navigation toolbar attached to a parent browser window.
#![allow(unsafe_code)]

use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, GetWindowTextLengthW, GetWindowTextW, MoveWindow, SetWindowTextW,
    WS_BORDER, WS_CHILD, WS_VISIBLE, ES_AUTOHSCROLL,
};

pub const HEIGHT: i32 = 44;
pub const BACK: usize = 101;
pub const FORWARD: usize = 102;
pub const RELOAD: usize = 103;
pub const STOP: usize = 104;
pub const ADDRESS: usize = 105;
pub const GO: usize = 106;

fn wide(input: &str) -> Vec<u16> {
    input.encode_utf16().chain(std::iter::once(0)).collect()
}

fn child(parent: HWND, class: &str, title: &str, id: usize, style: u32) -> Result<HWND, &'static str> {
    let class = wide(class);
    let title = wide(title);
    // SAFETY: All UTF-16 inputs are terminated and parent must be a live HWND on this thread.
    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class.as_ptr(),
            title.as_ptr(),
            WS_CHILD | WS_VISIBLE | style,
            0,
            0,
            0,
            0,
            parent,
            id as *mut core::ffi::c_void,
            core::ptr::null_mut(),
            core::ptr::null(),
        )
    };
    if hwnd.is_null() {
        return Err("failed to create native toolbar control");
    }
    Ok(hwnd)
}

/// Child windows are owned and destroyed by their parent HWND.
pub struct Toolbar {
    back: HWND,
    forward: HWND,
    reload: HWND,
    stop: HWND,
    address: HWND,
    go: HWND,
}

impl Toolbar {
    pub fn new(parent: HWND, initial_url: &str) -> Result<Self, &'static str> {
        Ok(Self {
            back: child(parent, "BUTTON", "Back", BACK, 0)?,
            forward: child(parent, "BUTTON", "Forward", FORWARD, 0)?,
            reload: child(parent, "BUTTON", "Reload", RELOAD, 0)?,
            stop: child(parent, "BUTTON", "Stop", STOP, 0)?,
            address: child(parent, "EDIT", initial_url, ADDRESS, WS_BORDER | ES_AUTOHSCROLL as u32)?,
            go: child(parent, "BUTTON", "Go", GO, 0)?,
        })
    }

    pub fn resize(&self, width: i32) {
        let mut left = 8;
        for (hwnd, size) in [(self.back, 62), (self.forward, 74), (self.reload, 70), (self.stop, 58)] {
            // SAFETY: HWNDs belong to the live parent; layout runs on the UI thread.
            unsafe { MoveWindow(hwnd, left, 8, size, 28, 1) };
            left += size + 5;
        }
        let go_width = 48;
        let address_width = (width - left - go_width - 20).max(70);
        // SAFETY: HWNDs belong to the live parent; layout runs on the UI thread.
        unsafe {
            MoveWindow(self.address, left, 8, address_width, 28, 1);
            MoveWindow(self.go, left + address_width + 5, 8, go_width, 28, 1);
        }
    }

    pub fn address(&self) -> String {
        // SAFETY: The edit control is a live HWND on the UI thread.
        unsafe {
            let size = GetWindowTextLengthW(self.address);
            let mut buffer = vec![0u16; (size + 1) as usize];
            let copied = GetWindowTextW(self.address, buffer.as_mut_ptr(), size + 1);
            String::from_utf16_lossy(&buffer[..copied as usize])
        }
    }

    pub fn set_address(&self, url: &str) {
        let value = wide(url);
        // SAFETY: The pointer is terminated and valid throughout the synchronous call.
        unsafe { SetWindowTextW(self.address, value.as_ptr()) };
    }
}
