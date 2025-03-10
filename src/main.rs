use brocproc::{tokenize, xformat_args};
use broccli::errors::Error;
use broccli::interface::Interface;
use broccli::xprintln;
use broccolor::Color;

fn main() -> Result<(), Error> {
    let interface = Interface::new();

    let test = tokenize!(
        "test" => Color::Crimson,
        "test",
        "test2" => Color::White,
        {
            {
                "ali"
            }
        },
        "test",
    );


    let vec = vec!["test"];
    let test = xprintln!("test {:?} {}" => Color::Crimson, vec => Color::Blue, "test2" => Color::White);

/*    interface.clear()?;

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
    }*/

    Ok(())
}
