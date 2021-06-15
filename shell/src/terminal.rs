use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

/// Starts the main terminal process.
pub fn start_main_process() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}Ctrl-C to exit.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();

    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::CurrentLine
    )
    .unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Alt(c) => {
                if c as u32 == 127 {
                    write!(stdout, "BACKSPACE ALT").unwrap();
                }
            }
            _ => {}
        };
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
