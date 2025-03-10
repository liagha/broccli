use crate::Color;
use crate::conversion::ColorConversion;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub struct ColoredText<T> {
    pub content: T,
    pub color: Color,
}

impl<T: Display> Display for ColoredText<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}{}{}", self.color.to_ansi_code(), self.content, Color::reset())
    }
}

impl<T: Debug> Debug for ColoredText<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}{:?}{}", self.color.to_ansi_code(), self.content, Color::reset())
    }
}

impl<T: Display + Debug> TextStyle for T {
    fn colorize(&self, color: Color) -> String {
        format!("{}{}{}", color.to_ansi_code(), self, Color::reset())
    }

    fn to_colored<U: Display + Debug>(self, color: Color) -> ColoredText<U>
    where
        Self: Into<U>,
    {
        ColoredText {
            content: self.into(),
            color,
        }
    }
    fn background(&self, color: Color) -> String {
        format!("{}{}{}", color.to_background_ansi_code(), self, Color::reset())
    }

    fn bold(&self) -> String {
        format!("{}{}{}", Color::bold(), self, Color::reset())
    }

    fn italic(&self) -> String {
        format!("{}{}{}", Color::italic(), self, Color::reset())
    }

    fn underline(&self) -> String {
        format!("{}{}{}", Color::underline(), self, Color::reset())
    }

    fn strikethrough(&self) -> String {
        format!("{}{}{}", Color::strikethrough(), self, Color::reset())
    }
}

#[allow(dead_code)]
pub trait TextStyle {
    fn colorize(&self, color: Color) -> String;
    fn to_colored<T: Display + Debug>(self, color: Color) -> ColoredText<T>
    where
        Self: Sized + Into<T>;    fn background(&self, color: Color) -> String;
    fn bold(&self) -> String;
    fn italic(&self) -> String;
    fn underline(&self) -> String;
    fn strikethrough(&self) -> String;
}
