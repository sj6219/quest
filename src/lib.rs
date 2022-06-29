#![deny(missing_docs)]

//! Command-line input utilities.

#[macro_use] extern crate cfg_if;
extern crate rpassword;
extern crate tempfile;
#[cfg(unix)] extern crate termios;
#[cfg(windows)] extern crate winapi;

use interactive::Interactive;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use std::{env, io};
use tempfile::TempDir;

mod interactive;
mod util;

/// The ASCII escape character.
const ESC: u8 = 0x1B;

/// Print a question, in bold, without creating a new line.
pub fn ask(q: &str) {
    print!("\u{1B}[1m{}\u{1B}[0m", q);
    io::stdout().flush().unwrap();
}

/// Print a message of success, with a newline.
pub fn success(s: &str) {
    println!("\u{1B}[1;92m{}\u{1B}[0m", s);
}

/// Print an error message, with a newline.
pub fn error(s: &str) {
    println!("\u{1B}[1;91m{}\u{1B}[0m", s);
}

/// Ask for a password (the password will not be visible).
pub fn password() -> io::Result<String> {
    rpassword::read_password()
}

/// Ask for a line of text.
pub fn text() -> io::Result<String> {
    // Read up to the first newline or EOF.

    let mut out = String::new();
    io::stdin().read_line(&mut out)?;

    // Only capture up to the first newline.

    if let Some(mut newline) = out.find('\n') {
        if newline > 0 && out.as_bytes()[newline - 1] == b'\r' { newline -= 1; }
        out.truncate(newline);
    }

    Ok(out)
}

/// Ask a yes-or-no question.
///
/// `None` indicates an invalid response.
pub fn yesno(default: bool) -> io::Result<Option<bool>> {
    let s = text()?.to_lowercase();
    Ok(if s.is_empty() {
        Some(default)
    } else if "yes".starts_with(&s) {
        Some(true)
    } else if "no".starts_with(&s) {
        Some(false)
    } else {
        None
    })
}

/// Ask the user to enter some text through their editor.
///
/// We'll check the `VISUAL` environment variable, then `EDITOR`, and then
/// finally default to `vi`. The message will be the initial contents of the
/// file, and the result will be the final contents of the file, after the user
/// has quit their editor.
///
/// On Windows, the editor defaults to `notepad`.
pub fn editor(name: &str, message: &[u8]) -> io::Result<String> {
    // Create a temporary file with the message.

    let dir = TempDir::new()?;
    let path = dir.path().join(name);
    File::create(&path)?.write_all(message)?;

    // Get the editor command from the environment.

    let editor = env::var_os("VISUAL").or_else(|| env::var_os("EDITOR"));

    let editor = match editor {
        Some(ref editor) => editor,
        None => OsStr::new(
            #[cfg(windows)] "notepad",
            #[cfg(unix)] "vi"),
    };

    // Call the editor.

    Command::new(editor).arg(&path).spawn()?.wait()?;

    // Read the file back.

    let mut out = String::new();
    File::open(&path)?.read_to_string(&mut out)?;
    Ok(out)
}

/// The text to use when indicating whether an item is selected.
#[derive(Clone, Copy, Debug)]
pub struct Boxes<'a> {
    /// The text to use when an item is selected.
    pub on: &'a str,
    /// The text to use when an item is not selected.
    pub off: &'a str,
}

impl<'a> Default for Boxes<'a> {
    fn default() -> Self {
        Self {
            on: ">",
            off: " ",
        }
    }
}

/// Ask the user to choose exactly one option from a list.
pub fn choose<S: AsRef<str>>(boxes: Boxes, items: &[S]) -> io::Result<usize> {
    assert!(items.len() > 0);

    let stdin = io::stdin();
    let mut stdin = stdin.bytes();
    let mut selected = 0;

    let interactive = Interactive::start()?;

    loop {
        for (i, item) in items.iter().enumerate() {
            println!(
                "{} {}",
                if i == selected { boxes.on } else { boxes.off },
                item.as_ref(),
            );
        }

        match util::or2ro(stdin.next())? {
            Some(ESC) => match util::or2ro(stdin.next())? {
                Some(b'[') => match util::or2ro(stdin.next())? {
                    Some(b'A') => {
                        selected = selected.saturating_sub(1);
                    }
                    Some(b'B') => {
                        selected = selected.saturating_add(1).min(items.len() - 1);
                    }
                    None => break,
                    Some(_) => (),
                },
                None => break,
                Some(_) => (),
            },
            Some(b'\r') | Some(b'\n') => break,
            Some(b'k') => {
                selected = selected.saturating_sub(1);
            }
            Some(b'j') => {
                selected = selected.saturating_add(1).min(items.len() - 1);
            }
            None => break,
            Some(_) => (),
        }

        interactive.up(items.len());
        interactive.clear_right();
    }

    Ok(selected)
}
