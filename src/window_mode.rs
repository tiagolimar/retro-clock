use minifb::Window;

#[cfg(target_os = "linux")]
fn set_native_fullscreen(window: &Window, fullscreen: bool) -> bool {
    use minifb::HasWindowHandle;
    use raw_window_handle::{HasDisplayHandle, RawDisplayHandle, RawWindowHandle};
    use std::ffi::CString;
    use std::mem;
    use std::os::raw::c_long;
    use x11_dl::xlib;

    let Ok(window_handle) = window.window_handle() else {
        return false;
    };
    let RawWindowHandle::Xlib(window_handle) = window_handle.as_raw() else {
        return false;
    };
    let Ok(display_handle) = window.display_handle() else {
        return false;
    };
    let RawDisplayHandle::Xlib(display_handle) = display_handle.as_raw() else {
        return false;
    };
    let Some(display) = display_handle.display else {
        return false;
    };

    let Ok(xlib) = x11_dl::xlib::Xlib::open() else {
        return false;
    };
    let display = display.as_ptr() as *mut xlib::Display;
    let state_name = CString::new("_NET_WM_STATE").unwrap();
    let fullscreen_name = CString::new("_NET_WM_STATE_FULLSCREEN").unwrap();

    unsafe {
        let state = (xlib.XInternAtom)(display, state_name.as_ptr(), xlib::False);
        let fullscreen_atom = (xlib.XInternAtom)(display, fullscreen_name.as_ptr(), xlib::False);
        let root = (xlib.XDefaultRootWindow)(display);
        let mut event: xlib::XEvent = mem::zeroed();
        event.client_message = xlib::XClientMessageEvent {
            type_: xlib::ClientMessage,
            serial: 0,
            send_event: xlib::True,
            display,
            window: window_handle.window,
            message_type: state,
            format: 32,
            data: xlib::ClientMessageData::from([
                if fullscreen { 1 } else { 0 },
                fullscreen_atom as c_long,
                0,
                0,
                0,
            ]),
        };

        let sent = (xlib.XSendEvent)(
            display,
            root,
            xlib::False,
            xlib::SubstructureNotifyMask | xlib::SubstructureRedirectMask,
            &mut event,
        );
        (xlib.XFlush)(display);
        sent != 0
    }
}

#[cfg(windows)]
fn set_native_fullscreen(window: &Window, fullscreen: bool) -> bool {
    use std::ffi::c_void;
    use winapi::um::winuser::{ShowWindow, SW_MAXIMIZE, SW_RESTORE};

    unsafe {
        let handle = window.get_window_handle() as *mut c_void;
        ShowWindow(handle, if fullscreen { SW_MAXIMIZE } else { SW_RESTORE }) != 0
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
fn set_native_fullscreen(_window: &Window, _fullscreen: bool) -> bool {
    false
}

pub fn set_fullscreen(window: &Window, fullscreen: bool) -> bool {
    set_native_fullscreen(window, fullscreen)
}
