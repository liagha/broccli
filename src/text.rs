use crate::style::Style;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct Span<'a> {
    pub content: Cow<'a, str>,
    pub style: Style,
}

impl<'a> Span<'a> {
    pub fn new<T: Into<Cow<'a, str>>>(content: T) -> Self {
        Self {
            content: content.into(),
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line<'a> {
    pub spans: Vec<Span<'a>>,
}

impl<'a> Line<'a> {
    pub fn new(spans: Vec<Span<'a>>) -> Self {
        Self { spans }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text<'a> {
    pub lines: Vec<Line<'a>>,
}

impl<'a> Text<'a> {
    pub fn new(lines: Vec<Line<'a>>) -> Self {
        Self { lines }
    }
}
