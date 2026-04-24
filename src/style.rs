use broccolor::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Modifier(pub u16);

impl Modifier {
    pub const NONE: Modifier = Modifier(0);
    pub const BOLD: Modifier = Modifier(1 << 0);
    pub const ITALIC: Modifier = Modifier(1 << 1);
    pub const UNDERLINE: Modifier = Modifier(1 << 2);
    pub const STRIKE: Modifier = Modifier(1 << 3);

    pub fn insert(&mut self, other: Modifier) {
        self.0 |= other.0;
    }

    pub fn remove(&mut self, other: Modifier) {
        self.0 &= !other.0;
    }

    pub fn contains(self, other: Modifier) -> bool {
        (self.0 & other.0) == other.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub modifier: Modifier,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: None,
            bg: None,
            modifier: Modifier::NONE,
        }
    }
}

impl Style {
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifier.insert(modifier);
        self
    }
}
