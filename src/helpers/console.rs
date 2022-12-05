#[cfg(target_os = "windows")]
pub fn enable_ansi_support() {
    use std::os::windows::prelude::AsRawHandle;

    const ENABLE_V_TERM_PROCESSING: u32 = 0x0004;

    let stdout = std::io::stdout();
    let handle = stdout.lock();

    let mut mode = 0;

    unsafe {
        let handle = handle.as_raw_handle();
        let result = kernel32::GetConsoleMode(handle, &mut mode);
        if result == 0 {
            return;
        }

        mode |= ENABLE_V_TERM_PROCESSING;

        let result = kernel32::SetConsoleMode(handle, mode);
        if result == 0 {
            return;
        }
    }
}