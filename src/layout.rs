#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Constraint {
    Length(u16),
    Percentage(u16),
    Min(u16),
    Max(u16),
}

pub struct Layout {
    pub direction: Direction,
    pub limits: Vec<Constraint>,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            direction: Direction::Vertical,
            limits: Vec::new(),
        }
    }
}

impl Layout {
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn limits(mut self, limits: Vec<Constraint>) -> Self {
        self.limits = limits;
        self
    }

    pub fn split(&self, area: Rect) -> Vec<Rect> {
        let mut parts = Vec::new();
        let is_horiz = self.direction == Direction::Horizontal;
        let total = if is_horiz { area.width } else { area.height };
        let mut offset = 0;

        for limit in &self.limits {
            let size = match limit {
                Constraint::Length(v) => *v,
                Constraint::Percentage(p) => (total as f32 * (*p as f32 / 100.0)) as u16,
                Constraint::Min(v) => *v,
                Constraint::Max(v) => *v,
            };

            let bound = size.min(total.saturating_sub(offset));

            if is_horiz {
                parts.push(Rect {
                    x: area.x + offset,
                    y: area.y,
                    width: bound,
                    height: area.height,
                });
            } else {
                parts.push(Rect {
                    x: area.x,
                    y: area.y + offset,
                    width: area.width,
                    height: bound,
                });
            }

            offset += bound;
        }

        parts
    }
}
