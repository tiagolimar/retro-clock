use std::thread;
use std::time::Duration;

const ALERT_BEEP_COUNT: u8 = 8;
const ALERT_BEEP_INTERVAL: Duration = Duration::from_millis(280);

pub fn trigger() {
    let _ = thread::spawn(|| {
        for beep in 0..ALERT_BEEP_COUNT {
            native_beep();

            if beep + 1 < ALERT_BEEP_COUNT {
                thread::sleep(ALERT_BEEP_INTERVAL);
            }
        }
    });
}

fn native_beep() {
    #[cfg(target_os = "linux")]
    trigger_linux();

    #[cfg(windows)]
    trigger_windows();
}

#[cfg(target_os = "linux")]
fn trigger_linux() {
    use std::ptr;
    use x11_dl::xlib;

    let Ok(xlib) = xlib::Xlib::open() else {
        return;
    };

    unsafe {
        let display = (xlib.XOpenDisplay)(ptr::null());
        if display.is_null() {
            return;
        }

        (xlib.XBell)(display, 100);
        (xlib.XFlush)(display);
        (xlib.XCloseDisplay)(display);
    }
}

#[cfg(windows)]
fn trigger_windows() {
    use winapi::um::winuser::{MessageBeep, MB_ICONEXCLAMATION};

    unsafe {
        MessageBeep(MB_ICONEXCLAMATION);
    }
}
