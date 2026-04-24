use broccolor::Color;
use crate::buffer::{Buffer, Cell};

pub trait Widget {
    fn render(self, area: Rect, buf: &mut Buffer);
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub line: Line,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Debug, Clone, Copy)]
pub enum Line {
    Single,
    Double,
    Rounded,
    Dashed,
    Dotted,
    Thick,
    Custom {
        tl: char,
        h: char,
        tr: char,
        v: char,
        br: char,
        bl: char,
    },
}

impl Widget for Block {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border = match self.line {
            Line::Single => ('┌', '─', '┐', '│', '└', '┘'),
            Line::Double => ('╔', '═', '╗', '║', '╚', '╝'),
            Line::Rounded => ('╭', '─', '╮', '│', '╰', '╯'),
            Line::Dashed => ('┌', '╌', '┐', '╎', '└', '┘'),
            Line::Dotted => ('┌', '┄', '┐', '┆', '└', '┘'),
            Line::Thick => ('▛', '▀', '▜', '▐', '▙', '▟'),
            Line::Custom { tl, h, tr, v, br, bl } => (tl, h, tr, v, br, bl),
        };

        let cell = |symbol: char| Cell {
            symbol,
            fg: self.fg,
            bg: self.bg,
        };

        buf.set(area.x, area.y, cell(border.0));
        buf.set(area.x + area.width - 1, area.y, cell(border.2));
        buf.set(area.x, area.y + area.height - 1, cell(border.4));
        buf.set(area.x + area.width - 1, area.y + area.height - 1, cell(border.5));

        for i in 1..area.width - 1 {
            buf.set(area.x + i, area.y, cell(border.1));
            buf.set(area.x + i, area.y + area.height - 1, cell(border.1));
        }

        for i in 1..area.height - 1 {
            buf.set(area.x, area.y + i, cell(border.3));
            buf.set(area.x + area.width - 1, area.y + i, cell(border.3));
            for j in 1..area.width - 1 {
                buf.set(area.x + j, area.y + i, cell(' '));
            }
        }
    }
}
