mod errors;
use std::io::{stdout, Stdout};
use crate::errors::Error;
use std::io::Write;

fn main() -> Result<(), Error> {
    let mut interface = Interface::new();
    let config = BoxConfig {
        x: 10,
        y: 5,
        width: 30,
        height: 10,
        border_style: BorderStyle::Rounded,
    };
    interface.clear()?;

    if let Ok((cols, rows)) = get_terminal_size() {
        let config = BoxConfig {
            x: 0, // Start from the first column
            y: 0, // Start from the first row
            width: cols, // Use the full width of the terminal
            height: rows, // Use the full height of the terminal
            border_style: BorderStyle::Rounded,
        };

        interface.draw_box(config)?;
    }

    Ok(())
}

struct Interface {
    cursor_pos: [i32; 2],
    stdout: Stdout,
}

use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};
fn get_terminal_size() -> Result<(u16, u16), std::io::Error> {
    let mut size: winsize = unsafe { std::mem::zeroed() };

    if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) } == -1 {
        return Err(std::io::Error::last_os_error());
    }

    Ok((size.ws_col, size.ws_row))
}

impl Interface {
    fn new() -> Self {
        Self {
            cursor_pos: [0, 0],
            stdout: stdout(),
        }
    }

    fn write_escape(&mut self, escape_sequence: &str) -> Result<(), Error> {
        let mut handle = self.stdout.lock();
        write!(handle, "{}", escape_sequence).map_err(|_| Error::CursorMove)?;
        handle.flush().map_err(|e| Error::Flush(e))?;
        Ok(())
    }

    fn move_cursor(&mut self, x: i32, y: i32) -> Result<(), Error> {
        let escape = format!("\x1B[{};{}H", y + 1, x + 1);
        self.write_escape(&escape)?;
        self.cursor_pos = [x, y];
        Ok(())
    }

    fn move_cursor_up(&mut self, n: i32) -> Result<(), Error> {
        let escape = format!("\x1B[{}A", n);
        self.write_escape(&escape)?;
        self.cursor_pos[1] += n;
        Ok(())
    }

    fn move_cursor_down(&mut self, n: i32) -> Result<(), Error> {
        let escape = format!("\x1B[{}B", n);
        self.write_escape(&escape)?;
        self.cursor_pos[1] -= n;
        Ok(())
    }

    fn move_cursor_forward(&mut self, n: i32) -> Result<(), Error> {
        let escape = format!("\x1B[{}C", n);
        self.write_escape(&escape)?;
        self.cursor_pos[0] += n;
        Ok(())
    }

    fn move_cursor_backward(&mut self, n: i32) -> Result<(), Error> {
        let escape = format!("\x1B[{}D", n);
        self.write_escape(&escape)?;
        self.cursor_pos[0] -= n;
        Ok(())
    }

    fn save_cursor(&mut self) -> Result<(), Error> {
        self.write_escape("\x1B[s")
    }

    fn restore_cursor(&mut self) -> Result<(), Error> {
        self.write_escape("\x1B[u")
    }

    fn scroll_up(&mut self, n: i32) -> Result<(), Error> {
        let escape = format!("\x1B[{}S", n);
        self.write_escape(&escape)
    }

    fn scroll_down(&mut self, n: i32) -> Result<(), Error> {
        let escape = format!("\x1B[{}T", n);
        self.write_escape(&escape)
    }

    fn clear(&mut self) -> Result<(), Error> {
        self.write_escape("\x1B[2J\x1B[H")?;
        self.cursor_pos = [0, 0];
        Ok(())
    }

    fn clear_line(&mut self) -> Result<(), Error> {
        self.write_escape("\x1B[2K")
    }

    fn clear_from_cursor_to_end(&mut self) -> Result<(), Error> {
        self.write_escape("\x1B[0J")
    }

    fn clear_from_cursor_to_begin(&mut self) -> Result<(), Error> {
        self.write_escape("\x1B[1J")
    }
    fn draw_box(&mut self, config: BoxConfig) -> Result<(), Error> {
        let (x, y, width, height) = (config.x as i32, config.y as i32, config.width as i32, config.height as i32);
        let border = match config.border_style {
            BorderStyle::Single => ('┌', '─', '┐', '│', '└', '┘'),
            BorderStyle::Double => ('╔', '═', '╗', '║', '╚', '╝'),
            BorderStyle::Rounded => ('╭', '─', '╮', '│', '╰', '╯'),
            BorderStyle::Custom(c) => (c, c, c, c, c, c),
        };

        self.move_cursor(x, y)?;
        write!(self.stdout, "{}{}{}", border.0, border.1.to_string().repeat((width - 2) as usize), border.2).map_err(|e| Error::WriteError)?;

        for i in 1..height - 1 {
            self.move_cursor(x, y + i)?;
            write!(self.stdout, "{}", border.3).map_err(|e| Error::WriteError)?;
            self.move_cursor(x + width - 1, y + i)?;
            write!(self.stdout, "{}", border.3).map_err(|e| Error::WriteError)?;
        }

        self.move_cursor(x, y + height - 1)?;
        write!(self.stdout, "{}{}{}", border.4, border.1.to_string().repeat((width - 2) as usize), border.5).map_err(|e| Error::WriteError)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoxConfig {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub border_style: BorderStyle,
}

#[derive(Debug, Clone, Copy)]
pub enum BorderStyle {
    Single,
    Double,
    Rounded,
    Custom(char),
}