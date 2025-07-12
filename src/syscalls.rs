// Removed: use std::ptr;
use winapi::um::memoryapi::VirtualProtect;
use winapi::shared::minwindef::DWORD;

pub fn stealth_virtual_protect(addr: *mut u8, size: usize, new_protect: DWORD) {
    unsafe {
        let mut old_protect = 0;
        let func: extern "system" fn(*mut u8, usize, DWORD, *mut DWORD) -> i32 = std::mem::transmute(VirtualProtect as usize);
        func(addr, size, new_protect, &mut old_protect);
    }
}