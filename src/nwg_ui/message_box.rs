
use std::ptr::null_mut as NULL;

use winapi::um::winuser;

#[allow(dead_code)]
pub fn message_box(title: &str, message: &str) {
    let mut title_term = title.to_string();
    title_term.push('\0');
    let title_wide: Vec<u16> =  title_term.encode_utf16().collect();
    let mut message_term = message.to_string();
    message_term.push('\0');
    let message_wide: Vec<u16> = message_term.encode_utf16().collect();
    unsafe {
        winuser::MessageBoxW(
            NULL(),
            message_wide.as_ptr(),
            title_wide.as_ptr(),
            winuser::MB_OK | winuser::MB_ICONINFORMATION
        );
    }
}

#[allow(dead_code)]
pub fn message_box_debug(message: String) {
    message_box("Debug", &message);
}
