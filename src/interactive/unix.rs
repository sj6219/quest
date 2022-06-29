use std::io::{self, Write};
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

pub struct Interactive {
    old: Termios,
}

impl Interactive {
    pub fn start() -> io::Result<Self> {
        let old = Termios::from_fd(0).unwrap();
        let mut new = old.clone();
        new.c_lflag &= !(ICANON | ECHO);
        tcsetattr(0, TCSANOW, &new).unwrap();

        Ok(Self { old })
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
        tcsetattr(0, TCSANOW, &self.old).unwrap();
    }
}
