use std::sync::Once;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::System::Diagnostics::Debug::OutputDebugStringW;

pub fn debug_writeln(message: &str) {
    unsafe {
        OutputDebugStringW(PCWSTR(HSTRING::from(message.to_owned() + "\n").as_ptr()));
    }
}

static mut TRACE: bool = false;
static TRACE_SET: Once = Once::new();
pub fn trace() -> bool {
    TRACE_SET.call_once(|| unsafe {
        TRACE = std::env::var("HIDE_RDP_TRACE").map_or(false, |v| v == "1" || v == "true")
    });
    unsafe { TRACE }
}