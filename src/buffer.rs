use broccolor::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub symbol: char,
    pub fg: Color,
    pub bg: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: ' ',
            fg: Color::White,
            bg: Color::Transparent,
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
