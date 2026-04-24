use crate::style::Style;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub symbol: char,
    pub style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: ' ',
            style: Style::default(),
        }
    }
}

#[derive(Clone)]
pub struct Buffer {
    pub width: u16,
    pub height: u16,
    pub content: Vec<Cell>,
}

impl Buffer {
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            content: vec![Cell::default(); size],
        }
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        if x < self.width && y < self.height {
            let index = (y as usize) * (self.width as usize) + (x as usize);
            self.content[index] = cell;
        }
    }

    pub fn reset(&mut self) {
        for cell in &mut self.content {
            *cell = Cell::default();
        }
    }
}
