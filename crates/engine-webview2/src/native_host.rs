//! Host-owned Windows window: Win32 controls and WebView2 share the UI STA.
#![allow(unsafe_code)]
use browser_engine::BrowserCommand;
use browser_windows::toolbar::{self, Toolbar};
use std::cell::RefCell;
use std::rc::Rc;
use windows_sys::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx, CoUninitialize};
use windows_webview::WebViewHost;

const WM_COMMAND: u32 = 0x0111;

pub fn run(start_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // SAFETY: CoInitializeEx operates on the current UI thread.
    let initialized = unsafe { CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED as u32) };
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
    let move_host = Rc::clone(&host);
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
                if let Some(host) = event_host.borrow().as_ref()
                    && let Err(error) = super::dispatch_webview_command(host.webview(), command)
                {
                    tracing::warn!(%error, "Toolbar navigation failed");
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
                let _ = host.controller().set_bounds(
                    0,
                    toolbar::HEIGHT,
                    width,
                    height.max(toolbar::HEIGHT),
                );
            }
        })
        .on_move(move || {
            if let Some(host) = move_host.borrow().as_ref()
                && let Err(error) = host.controller().notify_parent_window_position_changed()
            {
                tracing::warn!(%error, "WebView2 parent window move notification failed");
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
    let completed_toolbar = Rc::clone(&toolbar);
    let shortcut_toolbar = Rc::clone(&toolbar);
    windows_webview::WebViewHost::builder()
        .create_for_hwnd(window.hwnd(), move |result| {
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
            if let Err(error) =
                super::dispatch_webview_command(view, BrowserCommand::Navigate(target))
            {
                *error_slot.borrow_mut() = Some(error.to_string());
                windows_window::quit();
                return;
            }
            let completed_view = view.clone();
            let navigation_toolbar = Rc::clone(&completed_toolbar);
            let title_view = view.clone();
            let registrations = [
                view.on_navigation_completed(move |args| {
                    let url = completed_view.source();
                    if let Some(controls) = navigation_toolbar.borrow().as_ref() {
                        controls.sync_address(&url);
                    }
                    tracing::info!(url = %url, success = args.is_success(), "Navigation completed");
                }),
                view.on_document_title_changed(move |title| {
                    tracing::info!(%title, "Page title changed");
                }),
                view.on_process_failed(move |args| {
                    tracing::error!(kind = ?args.kind(), "WebView2 process failed");
                }),
            ];
            for registration in registrations {
                match registration {
                    Ok(registration) => instance.retain(registration),
                    Err(error) => tracing::error!(%error, "Registering browser event failed"),
                }
            }
            let shortcut_view = title_view;
            match instance
                .controller()
                .on_accelerator_key_pressed(move |args| {
                    use windows_webview::KeyEventKind;
                    if matches!(args.key_event_kind(), KeyEventKind::KeyDown)
                        && args.virtual_key() == 0x4C
                        && unsafe {
                            windows_sys::Win32::UI::Input::KeyboardAndMouse::GetKeyState(0x11)
                        } < 0
                    {
                        if let Some(controls) = shortcut_toolbar.borrow().as_ref() {
                            controls.focus_address();
                        }
                        let _ = args.set_handled(true);
                        return;
                    }
                    let command = match (args.key_event_kind(), args.virtual_key()) {
                        (KeyEventKind::KeyDown, 0x74) => Some(BrowserCommand::Reload),
                        (KeyEventKind::KeyDown, 0x1B) => Some(BrowserCommand::Stop),
                        (KeyEventKind::SystemKeyDown, 0x25) => Some(BrowserCommand::Back),
                        (KeyEventKind::SystemKeyDown, 0x27) => Some(BrowserCommand::Forward),
                        _ => None,
                    };
                    if let Some(command) = command {
                        if let Err(error) = super::dispatch_webview_command(&shortcut_view, command)
                        {
                            tracing::warn!(%error, "Keyboard navigation failed");
                        }
                        let _ = args.set_handled(true);
                    }
                }) {
                Ok(registration) => instance.retain(registration),
                Err(error) => tracing::error!(%error, "Registering keyboard shortcuts failed"),
            }
            if let Some(controls) = completed_toolbar.borrow().as_ref() {
                controls.set_address(&view.source());
            }
            *created_host.borrow_mut() = Some(instance);
        })
        .map_err(|error| format!("Creating WebView2 host failed: {error}"))?;

    windows_window::run();
    host.borrow_mut().take();
    if let Some(message) = failure.borrow_mut().take() {
        return Err(message.into());
    }
    Ok(())
}
