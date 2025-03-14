use brocproc::{tree};
use broccli::errors::Error;

fn main() -> Result<(), Error> {
    tree!(
        "Hello, World!",
        {
            "Hello, World 2!";
            "Last One Here";
        },
        {
            "Hello, World 3!";
            {
                "Nested Block 1"
            }
            {
                "Nested Block 2"
            }
        },
        "Goodbye, World!"
    );


    //let test = xprintln!("test {:?} {}" => Color::Crimson, vec => Color::Blue, "test2" => Color::White);

/*
    let interface = Interface::new();

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
    }*/

    Ok(())
}
