use std::io::{self, Write};
use winapi::{
    shared::minwindef::{DWORD, FALSE, LPDWORD},
    um::{
        consoleapi::{GetConsoleMode, SetConsoleMode},
        handleapi::INVALID_HANDLE_VALUE,
        processenv::GetStdHandle,
        winbase::STD_INPUT_HANDLE,
        wincon::*,
        winnt::HANDLE,
    }
};

pub struct Interactive {
    handle: HANDLE,
    old_mode: DWORD,
}

impl Interactive {
    pub fn start() -> io::Result<Self> {
        // Retrieve a handle to the stdin.

        let handle = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
        if handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }

        // Retrieve the current input mode to restore it later.

        let mut old_mode = 0 as DWORD;
        if unsafe { GetConsoleMode(handle, &mut old_mode as LPDWORD) } == FALSE {
            return Err(io::Error::last_os_error());
        }

        // Set the new input mode.
        // ENABLE_PROCESSED_INPUT delegates Ctrl+C processing to system.
        // ENABLE_VIRTUAL_TERMINAL_INPUT allows to process arrow keys in the app code.

        let mode = ENABLE_PROCESSED_INPUT | ENABLE_VIRTUAL_TERMINAL_INPUT;
        if unsafe { SetConsoleMode(handle, mode) } == FALSE {
            return Err(io::Error::last_os_error());
        }

        Ok(Interactive { handle, old_mode })
    }

    pub fn up(&self, n: usize) {
        print!("\u{1B}[{}A", n);
        let _ = io::stdout().flush();
    }

    pub fn clear_right(&self) {
        print!("\u{1B}[J");
        let _ = io::stdout().flush();
    }
}

impl Drop for Interactive {
    fn drop(&mut self) {
        // Restore the previous console mode.

        unsafe { SetConsoleMode(self.handle, self.old_mode) };
    }
}
