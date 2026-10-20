//! Native Win32 navigation toolbar attached to a parent browser window.
#![allow(unsafe_code)]

use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::UI::Controls::{DefSubclassProc, SetWindowSubclass};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, EM_SETSEL, ES_AUTOHSCROLL, GetParent, GetWindowTextLengthW, GetWindowTextW,
    MoveWindow, SendMessageW, SetFocus, SetWindowTextW, WS_BORDER, WS_CHILD, WS_VISIBLE,
};

pub const HEIGHT: i32 = 44;
pub const BACK: usize = 101;
pub const FORWARD: usize = 102;
pub const RELOAD: usize = 103;
pub const STOP: usize = 104;
pub const ADDRESS: usize = 105;
pub const GO: usize = 106;

const WM_KEYDOWN: u32 = 0x0100;
const VK_RETURN: usize = 0x0D;
const WM_COMMAND: u32 = 0x0111;

unsafe extern "system" fn address_key_handler(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
    _subclass_id: usize,
    _ref_data: usize,
) -> LRESULT {
    if message == WM_KEYDOWN && wparam == VK_RETURN {
        // SAFETY: This control belongs to its parent HWND, and messages are
        // dispatched synchronously on the owning UI thread.
        let parent = unsafe { GetParent(hwnd) };
        if !parent.is_null() {
            unsafe { SendMessageW(parent, WM_COMMAND, GO, 0) };
        }
        return 0;
    }
    unsafe { DefSubclassProc(hwnd, message, wparam, lparam) }
}

fn wide(input: &str) -> Vec<u16> {
    input.encode_utf16().chain(std::iter::once(0)).collect()
}

fn child(
    parent: HWND,
    class: &str,
    title: &str,
    id: usize,
    style: u32,
) -> Result<HWND, &'static str> {
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
        let address = child(
            parent,
            "EDIT",
            initial_url,
            ADDRESS,
            WS_BORDER | ES_AUTOHSCROLL as u32,
        )?;
        // SAFETY: The callback has a static lifetime and does not capture state.
        if unsafe { SetWindowSubclass(address, Some(address_key_handler), ADDRESS, 0) } == 0 {
            return Err("failed to attach address Enter-key handler");
        }
        Ok(Self {
            back: child(parent, "BUTTON", "Back", BACK, 0)?,
            forward: child(parent, "BUTTON", "Forward", FORWARD, 0)?,
            reload: child(parent, "BUTTON", "Reload", RELOAD, 0)?,
            stop: child(parent, "BUTTON", "Stop", STOP, 0)?,
            address,
            go: child(parent, "BUTTON", "Go", GO, 0)?,
        })
    }

    pub fn resize(&self, width: i32) {
        let mut left = 8;
        for (hwnd, size) in [
            (self.back, 62),
            (self.forward, 74),
            (self.reload, 70),
            (self.stop, 58),
        ] {
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

    pub fn focus_address(&self) {
        // SAFETY: The edit HWND remains owned by the current live browser window.
        unsafe {
            SetFocus(self.address);
            SendMessageW(self.address, EM_SETSEL, 0, -1);
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
