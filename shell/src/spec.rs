use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::CurrentLine
    )
    .unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => write!(stdout, "{}", c).unwrap(),
            Key::Alt(c) => {
                write!(stdout, "^").unwrap();
                if c as u32 == 127 {
                    write!(stdout, "BACKSPACE ALT").unwrap();
                }
            }
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {}
        };
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
