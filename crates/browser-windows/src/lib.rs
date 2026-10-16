//! Windows-specific functionality is isolated in this crate.

pub struct PlatformInfo;

impl PlatformInfo {
    #[must_use]
    pub const fn current() -> Self {
        Self
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        if cfg!(target_os = "windows") {
            "Windows"
        } else {
            "non-Windows development host"
        }
    }
}

#[derive(Debug)]
pub struct WindowError(pub &'static str);

impl std::fmt::Display for WindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl std::error::Error for WindowError {}

#[cfg(windows)]
#[allow(unsafe_code)]
mod native {
    use super::WindowError;
    use std::ptr::{null, null_mut};
    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::HiDpi::{
        DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, SetProcessDpiAwarenessContext,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyWindow,
        DispatchMessageW, GetMessageW, IDC_ARROW, LoadCursorW, MSG, PostQuitMessage,
        RegisterClassW, SW_SHOW, SWP_NOACTIVATE, SWP_NOZORDER, SetWindowPos, ShowWindow,
        TranslateMessage, WM_DESTROY, WM_DPICHANGED, WNDCLASSW, WS_OVERLAPPEDWINDOW,
    };

    const CLASS: &[u16] = &[
        82, 117, 115, 116, 66, 114, 111, 119, 115, 101, 114, 87, 105, 110, 100, 111, 119, 0,
    ]; // RustBrowserWindow
    const TITLE: &[u16] = &[82, 117, 115, 116, 66, 114, 111, 119, 115, 101, 114, 0]; // RustBrowser

    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match message {
            WM_DESTROY => {
                unsafe { PostQuitMessage(0) };
                0
            }
            WM_DPICHANGED => {
                let suggested = lparam as *const RECT;
                if !suggested.is_null() {
                    let rect = unsafe { *suggested };
                    unsafe {
                        SetWindowPos(
                            hwnd,
                            null_mut(),
                            rect.left,
                            rect.top,
                            rect.right - rect.left,
                            rect.bottom - rect.top,
                            SWP_NOACTIVATE | SWP_NOZORDER,
                        );
                    }
                }
                0
            }
            _ => unsafe { DefWindowProcW(hwnd, message, wparam, lparam) },
        }
    }

    pub fn run() -> Result<(), WindowError> {
        // Only this native module is allowed to call Win32 unsafe APIs.
        unsafe {
            let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
            let instance = GetModuleHandleW(null());
            if instance.is_null() {
                return Err(WindowError("GetModuleHandleW failed"));
            }

            let class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_proc),
                hInstance: instance,
                hCursor: LoadCursorW(null_mut(), IDC_ARROW),
                lpszClassName: CLASS.as_ptr(),
                ..std::mem::zeroed()
            };
            if RegisterClassW(&class) == 0 {
                return Err(WindowError("RegisterClassW failed"));
            }

            let window = CreateWindowExW(
                0,
                CLASS.as_ptr(),
                TITLE.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                1200,
                800,
                null_mut(),
                null_mut(),
                instance,
                null(),
            );
            if window.is_null() {
                return Err(WindowError("CreateWindowExW failed"));
            }
            let _ = ShowWindow(window, SW_SHOW);

            let mut message: MSG = std::mem::zeroed();
            loop {
                let status = GetMessageW(&mut message, null_mut(), 0, 0);
                if status == -1 {
                    DestroyWindow(window);
                    return Err(WindowError("GetMessageW failed"));
                }
                if status == 0 {
                    break;
                }
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
        Ok(())
    }
}

/// Launches a native window on Windows; unavailable on other operating systems.
pub fn run_native_window() -> Result<(), WindowError> {
    #[cfg(windows)]
    {
        native::run()
    }
    #[cfg(not(windows))]
    {
        Err(WindowError("RustBrowser requires Windows"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_name_is_nonempty() {
        assert!(!PlatformInfo::current().name().is_empty());
    }
}
