//! Host-owned Windows window: Win32 controls and WebView2 share the UI STA.
use browser_engine::BrowserCommand;
use browser_windows::toolbar::{self, Toolbar};
use std::cell::RefCell;
use std::rc::Rc;
use windows_sys::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};
use windows_webview::WebViewHost;

const WM_COMMAND: u32 = 0x0111;

pub fn run(start_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: CoInitializeEx operates on the current UI thread.
    let initialized = unsafe { CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED) };
    if initialized < 0 {
        return Err(format!("COM STA initialization failed: {initialized}").into());
    }
    let outcome = run_sta(start_url);
    // SAFETY: This is called on the same thread for every successful COM initialization.
    unsafe { CoUninitialize() };
    outcome
}

fn run_sta(start_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let host = Rc::new(RefCell::new(None::<WebViewHost>));
    let toolbar = Rc::new(RefCell::new(None::<Toolbar>));
    let event_host = Rc::clone(&host);
    let resize_host = Rc::clone(&host);
    let resize_toolbar = Rc::clone(&toolbar);
    let button_toolbar = Rc::clone(&toolbar);
    let close_host = Rc::clone(&host);
    let failure = Rc::new(RefCell::new(None::<String>));
    let error_slot = Rc::clone(&failure);
    let window = windows_window::Window::new("RustBrowser")
        .size(1200, 800)
        .on_message(move |_hwnd, message, wparam, _lparam| {
            if message != WM_COMMAND {
                return None;
            }
            let id = wparam & 0xffff;
            let command = match id {
                toolbar::BACK => Some(BrowserCommand::Back),
                toolbar::FORWARD => Some(BrowserCommand::Forward),
                toolbar::RELOAD => Some(BrowserCommand::Reload),
                toolbar::STOP => Some(BrowserCommand::Stop),
                toolbar::GO => button_toolbar.borrow().as_ref().and_then(|toolbar| {
                    match browser_navigation::resolve_address(&toolbar.address()) {
                        Ok(url) => Some(BrowserCommand::Navigate(url)),
                        Err(error) => {
                            tracing::warn!(%error, "Invalid address input");
                            None
                        }
                    }
                }),
                _ => None,
            };
            if let Some(command) = command {
                if let Some(host) = event_host.borrow().as_ref() {
                    if let Err(error) = super::dispatch_webview_command(host.webview(), command) {
                        tracing::warn!(%error, "Toolbar navigation failed");
                    }
                }
                return Some(0);
            }
            None
        })
        .on_resize(move |width, height| {
            if let Some(toolbar) = resize_toolbar.borrow().as_ref() {
                toolbar.resize(width);
            }
            if let Some(host) = resize_host.borrow().as_ref() {
                let _ = host
                    .controller()
                    .set_bounds(0, toolbar::HEIGHT, width, height);
            }
        })
        .on_close(move || {
            if let Some(host) = close_host.borrow_mut().take() {
                let _ = host.close();
            }
        })
        .create()
        .map_err(|error| format!("Creating Windows browser window failed: {error}"))?;

    let controls = Toolbar::new(window.hwnd(), start_url)
        .map_err(|error| format!("Creating browser toolbar failed: {error}"))?;
    let (width, height) = window.client_size();
    controls.resize(width);
    *toolbar.borrow_mut() = Some(controls);

    let target = start_url.to_owned();
    let created_host = Rc::clone(&host);
    windows_webview::WebViewHost::builder().create_for_hwnd(window.hwnd(), move |result| {
        let instance = match result {
            Ok(host) => host,
            Err(error) => {
                *error_slot.borrow_mut() = Some(error.to_string());
                windows_window::quit();
                return;
            }
        };
        let view = instance.webview();
        if let Err(error) = instance
            .controller()
            .set_bounds(0, toolbar::HEIGHT, width, height)
        {
            *error_slot.borrow_mut() = Some(error.to_string());
            windows_window::quit();
            return;
        }
        if let Err(error) = super::dispatch_webview_command(view, BrowserCommand::Navigate(target))
        {
            *error_slot.borrow_mut() = Some(error.to_string());
            windows_window::quit();
            return;
        }
        *created_host.borrow_mut() = Some(instance);
    })?;

    windows_window::run();
    host.borrow_mut().take();
    if let Some(message) = failure.borrow_mut().take() {
        return Err(message.into());
    }
    Ok(())
}

