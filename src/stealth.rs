use winapi::um::debugapi::CheckRemoteDebuggerPresent;
use winapi::um::winnt::HANDLE;
use std::ptr;

pub fn disable_callbacks() {
    let mut is_debugger_present = 0;
    unsafe {
        CheckRemoteDebuggerPresent(ptr::null_mut() as HANDLE, &mut is_debugger_present);
    }

    if is_debugger_present != 0 {
        panic!("Debugger detected! Exiting.");
    }
}
