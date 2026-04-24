use crate::buffer::Buffer;
use crate::style::Modifier;
use broccolor::{Color, ColorConversion};
use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::io::{stdout, Stdout, Write};

pub struct Terminal {
    out: Stdout,
    curr: Buffer,
    prev: Buffer,
}

impl Terminal {
    pub fn new() -> std::io::Result<Self> {
        let mut out = stdout();
        terminal::enable_raw_mode()?;
        out.execute(terminal::EnterAlternateScreen)?;
        out.execute(cursor::Hide)?;

        let size = terminal::size()?;
        let curr = Buffer::new(size.0, size.1);
        let prev = Buffer::new(size.0, size.1);

        Ok(Self { out, curr, prev })
    }

    pub fn size() -> std::io::Result<(u16, u16)> {
        terminal::size()
    }

    pub fn clear(&mut self) -> std::io::Result<()> {
        self.out.queue(terminal::Clear(terminal::ClearType::All))?;
        self.out.flush()
    }

    pub fn draw<F>(&mut self, build: F) -> std::io::Result<()>
    where
        F: FnOnce(&mut Buffer),
    {
        self.curr.reset();
        build(&mut self.curr);
        self.render()?;
        self.prev = self.curr.clone();
        Ok(())
    }

    fn render(&mut self) -> std::io::Result<()> {
        for y in 0..self.curr.height {
            for x in 0..self.curr.width {
                let index = (y as usize) * (self.curr.width as usize) + (x as usize);
                let curr = self.curr.content[index];
                let prev = self.prev.content[index];

                if curr != prev {
                    self.out.queue(cursor::MoveTo(x, y))?;

                    let fg = curr.style.fg.unwrap_or(Color::White).to_ansi_code();
                    let bg = curr.style.bg.unwrap_or(Color::Transparent).to_background_ansi_code();

                    let mut mods = String::new();
                    if curr.style.modifier.contains(Modifier::BOLD) { mods.push_str(Color::bold()); }
                    if curr.style.modifier.contains(Modifier::ITALIC) { mods.push_str(Color::italic()); }
                    if curr.style.modifier.contains(Modifier::UNDERLINE) { mods.push_str(Color::underline()); }
                    if curr.style.modifier.contains(Modifier::STRIKE) { mods.push_str(Color::strikethrough()); }

                    let reset = Color::reset();

                    write!(self.out, "{}{}{}{}{}", mods, fg, bg, curr.symbol, reset)?;
                }
            }
        }
        self.out.flush()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = self.out.execute(cursor::Show);
        let _ = self.out.execute(terminal::LeaveAlternateScreen);
    }
}
