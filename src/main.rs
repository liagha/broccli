use broccli::colors::Color;
use broccli::errors::Error;
use broccli::interface::Interface;
use broccli::terminal::get_terminal_size;
use broccli::widgets::{BoxConfig, LineStyle};

fn main() -> Result<(), Error> {
    let mut interface = Interface::new();

    interface.clear()?;

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
