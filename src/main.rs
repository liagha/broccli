mod errors;
mod colors;
mod macros;
mod interface;
mod terminal;
mod widgets;

use crate::errors::Error;
use crate::colors::Color;
use crate::interface::Interface;
use std::io::Write;
use crate::terminal::get_terminal_size;
use crate::widgets::{BoxConfig, LineStyle};

fn main() -> Result<(), Error> {
    let mut interface = Interface::new();

    interface.clear()?;

    xprintln!("{} hey" => Color::Red, "hey");

    if let Ok((cols, rows)) = get_terminal_size() {
        let config = BoxConfig {
            x: 0,
            y: 0,
            width: cols,
            height: rows,
            border_style: LineStyle::Single,
            color: Color::Transparent,
            background: Color::Transparent,
        };

        interface.draw_box(config)?;
    }

    Ok(())
}
