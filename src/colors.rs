#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    // Primary Colors
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Black,

    // Bright Variants
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    BrightBlack,

    // Cool Specific Colors
    Orange,
    Pink,
    Teal,
    Violet,
    Indigo,
    Lime,
    Turquoise,
    Coral,
    Crimson,
    Mint,
    Gold,
    Silver,
    Bronze,

    // Named Gray Tones
    LightGray,
    DarkGray,
    SlateGray,
    Charcoal,

    // RGB, RGBA, Hex, Indexed and Transparent
    Transparent,
    Rgb(u8, u8, u8),       // Custom RGB
    Rgba(u8, u8, u8, u8),  // Custom RGBA
    Hex(&'static str),     // Hexadecimal Color Code
    Indexed(u8),           // ANSI Indexed (0–255)
    Gray(u8),              // Gray shades (0–23)
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

#[allow(dead_code)]
impl Color {
    pub fn to_rgba(&self) -> Option<(f32, f32, f32, f32)> {
        match *self {
            Color::Rgb(r, g, b) => Some((
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                1.0, // Default alpha to 1.0 (fully opaque)
            )),
            Color::Rgba(r, g, b, a) => Some((
                r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0, // Convert alpha to f32 as well
            )),
            Color::Hex(ref hex) => {
                if hex.len() == 7 || hex.len() == 9 {
                    let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
                    let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
                    let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
                    let a = if hex.len() == 9 {
                        u8::from_str_radix(&hex[7..9], 16).ok()?
                    } else {
                        255
                    };
                    Some((
                        r as f32 / 255.0,
                        g as f32 / 255.0,
                        b as f32 / 255.0,
                        a as f32 / 255.0, // Convert alpha to f32
                    ))
                } else {
                    None
                }
            }
            Color::Gray(g) => Some((
                g as f32 / 255.0,
                g as f32 / 255.0,
                g as f32 / 255.0,
                1.0, // Grayscale has no alpha
            )),
            Color::Indexed(n) => {
                // Try to map indexed colors to standard colors or grayscale
                match n {
                    0..=7 => match n {
                        0 => Some((0.0, 0.0, 0.0, 1.0)), // Black
                        1 => Some((1.0, 0.0, 0.0, 1.0)), // Red
                        2 => Some((0.0, 1.0, 0.0, 1.0)), // Green
                        3 => Some((1.0, 1.0, 0.0, 1.0)), // Yellow
                        4 => Some((0.0, 0.0, 1.0, 1.0)), // Blue
                        5 => Some((1.0, 0.0, 1.0, 1.0)), // Magenta
                        6 => Some((0.0, 1.0, 1.0, 1.0)), // Cyan
                        7 => Some((1.0, 1.0, 1.0, 1.0)), // White
                        _ => None
                    },
                    232..=255 => {
                        // Grayscale range
                        let intensity = (n - 232) as f32 / 23.0;
                        Some((intensity, intensity, intensity, 1.0))
                    },
                    _ => {
                        // Extended color range, default to gray if mapping is unclear
                        Some((0.5, 0.5, 0.5, 1.0))
                    }
                }
            },
            Color::Transparent => Some((0.0, 0.0, 0.0, 0.0)),
            Color::Red => Some((1.0, 0.0, 0.0, 1.0)),
            Color::Green => Some((0.0, 1.0, 0.0, 1.0)),
            Color::Blue => Some((0.0, 0.0, 1.0, 1.0)),
            Color::Yellow => Some((1.0, 1.0, 0.0, 1.0)),
            Color::Magenta => Some((1.0, 0.0, 1.0, 1.0)),
            Color::Cyan => Some((0.0, 1.0, 1.0, 1.0)),
            Color::White => Some((1.0, 1.0, 1.0, 1.0)),
            Color::Black => Some((0.0, 0.0, 0.0, 1.0)),
            Color::Orange => Some((1.0, 0.647, 0.0, 1.0)),
            Color::Pink => Some((1.0, 0.75, 0.8, 1.0)),
            Color::Lime => Some((0.0, 1.0, 0.0, 1.0)),
            Color::Indigo => Some((0.294, 0.0, 0.51, 1.0)),
            Color::Violet => Some((0.933, 0.51, 0.933, 1.0)),
            Color::Turquoise => Some((0.25, 0.88, 0.82, 1.0)),
            Color::Teal => Some((0.0, 0.5, 0.5, 1.0)),
            Color::Mint => Some((0.68, 1.0, 0.65, 1.0)),
            Color::Coral => Some((1.0, 0.5, 0.31, 1.0)),
            Color::Charcoal => Some((0.2, 0.2, 0.2, 1.0)),
            Color::BrightRed => Some((1.0, 0.4, 0.4, 1.0)),
            Color::BrightGreen => Some((0.4, 1.0, 0.4, 1.0)),
            Color::BrightYellow => Some((1.0, 1.0, 0.4, 1.0)),
            Color::BrightBlue => Some((0.4, 0.4, 1.0, 1.0)),
            Color::BrightMagenta => Some((1.0, 0.4, 1.0, 1.0)),
            Color::BrightCyan => Some((0.4, 1.0, 1.0, 1.0)),
            Color::BrightWhite => Some((1.0, 1.0, 1.0, 1.0)),
            Color::BrightBlack => Some((0.2, 0.2, 0.2, 1.0)),
            Color::Crimson => Some((0.86, 0.08, 0.24, 1.0)),
            Color::Gold => Some((1.0, 0.84, 0.0, 1.0)),
            Color::Silver => Some((0.75, 0.75, 0.75, 1.0)),
            Color::Bronze => Some((0.8, 0.5, 0.2, 1.0)),
            Color::LightGray => Some((0.8, 0.8, 0.8, 1.0)),
            Color::DarkGray => Some((0.25, 0.25, 0.25, 1.0)),
            Color::SlateGray => Some((0.44, 0.5, 0.56, 1.0)),
        }
    }

    pub fn to_rgba_u8(&self) -> Option<(u8,u8,u8,u8)> {
        if let Some((r,g,b,a)) = self.to_rgba() {
            Some(((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8, (a * 255.0) as u8))
        } else {
            None
        }
    }

    pub fn to_rgba_array(&self) -> Option<[f32;4]> {
        if let Some((r,g,b,a)) = self.to_rgba() {
            Some([r, g, b, a])
        } else {
            None
        }
    }
    /// Convert color to Hexadecimal string (if applicable).
    pub fn to_hex(&self) -> Option<String> {
        match *self {
            Color::Transparent => Some("\x1b[49m".to_string()),
            Color::Rgb(r, g, b) => Some(format!("#{:02X}{:02X}{:02X}", r, g, b)),
            Color::Rgba(r, g, b, a) => Some(format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)),
            Color::Hex(ref hex) => Some(hex.to_string()),
            Color::Indexed(n) => {
                // Try to convert indexed colors to hex
                match n {
                    0..=7 => match n {
                        0 => Some("#000000".into()), // Black
                        1 => Some("#FF0000".into()), // Red
                        2 => Some("#00FF00".into()), // Green
                        3 => Some("#FFFF00".into()), // Yellow
                        4 => Some("#0000FF".into()), // Blue
                        5 => Some("#FF00FF".into()), // Magenta
                        6 => Some("#00FFFF".into()), // Cyan
                        7 => Some("#FFFFFF".into()), // White
                        _ => None
                    },
                    232..=255 => {
                        // Grayscale range
                        let intensity = (n - 232) * 11;
                        Some(format!("#{:02X}{:02X}{:02X}", intensity, intensity, intensity))
                    },
                    _ => None
                }
            },
            Color::Gray(g) => {
                let intensity = g * 11;
                Some(format!("#{:02X}{:02X}{:02X}", intensity, intensity, intensity))
            },
            Color::Red => Some("#FF0000".into()),
            Color::Green => Some("#00FF00".into()),
            Color::Blue => Some("#0000FF".into()),
            Color::Yellow => Some("#FFFF00".into()),
            Color::Magenta => Some("#FF00FF".into()),
            Color::Cyan => Some("#00FFFF".into()),
            Color::White => Some("#FFFFFF".into()),
            Color::Black => Some("#000000".into()),
            Color::Orange => Some("#FFA500".into()),
            Color::Pink => Some("#FFC0CB".into()),
            Color::Lime => Some("#00FF00".into()),
            Color::Indigo => Some("#4B0082".into()),
            Color::Violet => Some("#EE82EE".into()),
            Color::Turquoise => Some("#40E0D0".into()),
            Color::Teal => Some("#008080".into()),
            Color::Mint => Some("#98FF98".into()),
            Color::Coral => Some("#FF7F50".into()),
            Color::Charcoal => Some("#333333".into()),
            Color::BrightRed => Some("#FF6666".into()),
            Color::BrightGreen => Some("#66FF66".into()),
            Color::BrightYellow => Some("#FFFF66".into()),
            Color::BrightBlue => Some("#6666FF".into()),
            Color::BrightMagenta => Some("#FF66FF".into()),
            Color::BrightCyan => Some("#66FFFF".into()),
            Color::BrightWhite => Some("#FFFFFF".into()),
            Color::BrightBlack => Some("#333333".into()),
            Color::Crimson => Some("#DC143C".into()),
            Color::Gold => Some("#FFD700".into()),
            Color::Silver => Some("#C0C0C0".into()),
            Color::Bronze => Some("#CD7F32".into()),
            Color::LightGray => Some("#CCCCCC".into()),
            Color::DarkGray => Some("#404040".into()),
            Color::SlateGray => Some("#708090".into()),
        }
    }

    /// Convert to grayscale intensity (0–255).
    pub fn to_grayscale(&self) -> Option<u8> {
        match *self {
            Color::Transparent => Some(0),
            Color::Rgb(r, g, b) | Color::Rgba(r, g, b, _) => {
                Some(((r as u16 + g as u16 + b as u16) / 3) as u8)
            },
            Color::Gray(g) => Some(g * 11), // Approximate conversion for 0–23 to 0–255
            Color::Indexed(n) => {
                // For standard indexed colors and grayscale range
                match n {
                    0..=7 => match n {
                        0 => Some(0),    // Black
                        1 => Some(76),   // Red
                        2 => Some(149),  // Green
                        3 => Some(225),  // Yellow
                        4 => Some(29),   // Blue
                        5 => Some(102),  // Magenta
                        6 => Some(178),  // Cyan
                        7 => Some(255),  // White
                        _ => None
                    },
                    232..=255 => {
                        // Direct mapping of grayscale range
                        Some((n - 232) * 11)
                    },
                    _ => Some(128) // Default mid-gray for extended colors
                }
            },
            Color::Hex(ref hex) => {
                if hex.len() == 7 || hex.len() == 9 {
                    let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
                    let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
                    let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
                    Some(((r as u16 + g as u16 + b as u16) / 3) as u8)
                } else {
                    None
                }
            },
            // Predefined colors
            Color::Red => Some(76),
            Color::Green => Some(149),
            Color::Blue => Some(29),
            Color::Yellow => Some(225),
            Color::Magenta => Some(102),
            Color::Cyan => Some(178),
            Color::White => Some(255),
            Color::Black => Some(0),
            Color::Orange => Some(140),
            Color::Pink => Some(180),
            Color::Lime => Some(149),
            Color::Indigo => Some(60),
            Color::Violet => Some(130),
            Color::Turquoise => Some(140),
            Color::Teal => Some(100),
            Color::Mint => Some(160),
            Color::Coral => Some(120),
            Color::Charcoal => Some(50),
            Color::BrightRed => Some(100),
            Color::BrightGreen => Some(170),
            Color::BrightYellow => Some(240),
            Color::BrightBlue => Some(60),
            Color::BrightMagenta => Some(130),
            Color::BrightCyan => Some(200),
            Color::BrightWhite => Some(255),
            Color::BrightBlack => Some(30),
            Color::Crimson => Some(80),
            Color::Gold => Some(200),
            Color::Silver => Some(190),
            Color::Bronze => Some(110),
            Color::LightGray => Some(200),
            Color::DarkGray => Some(70),
            Color::SlateGray => Some(120),
        }
    }

    /// Parse Hexadecimal to Color.
    pub fn from_hex(hex: &str) -> Option<Color> {
        if hex.len() == 7 || hex.len() == 9 {
            let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
            let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
            let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
            if hex.len() == 9 {
                let a = u8::from_str_radix(&hex[7..9], 16).ok()?;
                Some(Color::Rgba(r, g, b, a))
            } else {
                Some(Color::Rgb(r, g, b))
            }
        } else {
            None
        }
    }

    /// Get a CSS-compatible color string.
    pub fn to_css(&self) -> Option<String> {
        match *self {
            Color::Rgb(r, g, b) => Some(format!("rgb({}, {}, {})", r, g, b)),
            Color::Rgba(r, g, b, a) => Some(format!("rgba({}, {}, {}, {})", r, g, b, a as f32 / 255.0)),
            _ => None,
        }
    }

    fn to_ansi_code(&self) -> String {
        match *self {
            // Standard Colors
            Color::Red => "\x1b[31m".to_string(),
            Color::Green => "\x1b[32m".to_string(),
            Color::Yellow => "\x1b[33m".to_string(),
            Color::Blue => "\x1b[34m".to_string(),
            Color::Magenta => "\x1b[35m".to_string(),
            Color::Cyan => "\x1b[36m".to_string(),
            Color::White => "\x1b[37m".to_string(),
            Color::Black => "\x1b[30m".to_string(),

            // Bright Variants
            Color::BrightRed => "\x1b[91m".to_string(),
            Color::BrightGreen => "\x1b[92m".to_string(),
            Color::BrightYellow => "\x1b[93m".to_string(),
            Color::BrightBlue => "\x1b[94m".to_string(),
            Color::BrightMagenta => "\x1b[95m".to_string(),
            Color::BrightCyan => "\x1b[96m".to_string(),
            Color::BrightWhite => "\x1b[97m".to_string(),
            Color::BrightBlack => "\x1b[90m".to_string(),

            // Cool Colors
            Color::Orange => "\x1b[38;5;208m".to_string(),
            Color::Pink => "\x1b[38;5;213m".to_string(),
            Color::Teal => "\x1b[38;5;37m".to_string(),
            Color::Violet => "\x1b[38;5;177m".to_string(),
            Color::Indigo => "\x1b[38;5;54m".to_string(),
            Color::Lime => "\x1b[38;5;154m".to_string(),
            Color::Turquoise => "\x1b[38;5;80m".to_string(),
            Color::Coral => "\x1b[38;5;203m".to_string(),
            Color::Crimson => "\x1b[38;5;161m".to_string(),
            Color::Mint => "\x1b[38;5;121m".to_string(),
            Color::Gold => "\x1b[38;5;220m".to_string(),
            Color::Silver => "\x1b[38;5;250m".to_string(),
            Color::Bronze => "\x1b[38;5;136m".to_string(),

            // Gray Tones
            Color::LightGray => "\x1b[38;5;250m".to_string(),
            Color::DarkGray => "\x1b[38;5;238m".to_string(),
            Color::SlateGray => "\x1b[38;5;241m".to_string(),
            Color::Charcoal => "\x1b[38;5;232m".to_string(),

            //Transparent
            Color::Transparent => "\x1b[39m".to_string(),

            // RGB and Indexed
            Color::Rgb(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
            Color::Indexed(n) => format!("\x1b[38;5;{}m", n),
            Color::Gray(g) => {
                let gray_index = 232 + (g.min(23)); // Gray scale ranges from 232 to 255
                format!("\x1b[38;5;{}m", gray_index)
            }

            // RGBA
            Color::Rgba(r, g, b, _) => format!("\x1b[38;2;{};{};{}m", r, g, b),

            Color::Hex(_) => {
                if let Some((r, g, b, _)) = self.to_rgba() {
                    format!("\x1b[38;2;{};{};{}m", r, g, b)
                } else {
                    "\x1b[39m".to_string() // Default color if parsing fails
                }
            }
        }
    }

    fn to_background_ansi_code(&self) -> String {
        match *self {
            // Standard Colors
            Color::Red => "\x1b[41m".to_string(),
            Color::Green => "\x1b[42m".to_string(),
            Color::Yellow => "\x1b[43m".to_string(),
            Color::Blue => "\x1b[44m".to_string(),
            Color::Magenta => "\x1b[45m".to_string(),
            Color::Cyan => "\x1b[46m".to_string(),
            Color::White => "\x1b[47m".to_string(),
            Color::Black => "\x1b[40m".to_string(),

            // Bright Variants
            Color::BrightRed => "\x1b[101m".to_string(),
            Color::BrightGreen => "\x1b[102m".to_string(),
            Color::BrightYellow => "\x1b[103m".to_string(),
            Color::BrightBlue => "\x1b[104m".to_string(),
            Color::BrightMagenta => "\x1b[105m".to_string(),
            Color::BrightCyan => "\x1b[106m".to_string(),
            Color::BrightWhite => "\x1b[107m".to_string(),
            Color::BrightBlack => "\x1b[100m".to_string(),

            // Cool Colors
            Color::Orange => "\x1b[48;5;208m".to_string(),
            Color::Pink => "\x1b[48;5;213m".to_string(),
            Color::Teal => "\x1b[48;5;37m".to_string(),
            Color::Violet => "\x1b[48;5;177m".to_string(),
            Color::Indigo => "\x1b[48;5;54m".to_string(),
            Color::Lime => "\x1b[48;5;154m".to_string(),
            Color::Turquoise => "\x1b[48;5;80m".to_string(),
            Color::Coral => "\x1b[48;5;203m".to_string(),
            Color::Crimson => "\x1b[48;5;161m".to_string(),
            Color::Mint => "\x1b[48;5;121m".to_string(),
            Color::Gold => "\x1b[48;5;220m".to_string(),
            Color::Silver => "\x1b[48;5;250m".to_string(),
            Color::Bronze => "\x1b[48;5;136m".to_string(),

            // Gray Tones
            Color::LightGray => "\x1b[48;5;250m".to_string(),
            Color::DarkGray => "\x1b[48;5;238m".to_string(),
            Color::SlateGray => "\x1b[48;5;241m".to_string(),
            Color::Charcoal => "\x1b[48;5;232m".to_string(),

            //Transparent
            Color::Transparent => "\x1b[49m".to_string(),

            // RGB and Indexed
            Color::Rgb(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
            Color::Indexed(n) => format!("\x1b[48;5;{}m", n),
            Color::Gray(g) => {
                let gray_index = 232 + (g.min(23)); // Gray scale ranges from 232 to 255
                format!("\x1b[48;5;{}m", gray_index)
            }

            // RGBA
            Color::Rgba(r, g, b, _) => format!("\x1b[48;2;{};{};{}m", r, g, b),

            // Hexadecimal Color
            Color::Hex(ref hex) => {
                if hex.len() == 6 {
                    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
                    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
                    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
                    format!("\x1b[48;2;{};{};{}m", r, g, b)
                } else {
                    "\x1b[49m".to_string() // Reset background color if hex is invalid
                }
            }
        }
    }

    fn reset() -> &'static str {
        "\x1b[0m"
    }

    fn bold() -> &'static str {
        "\x1b[1m"
    }

    fn italic() -> &'static str {
        "\x1b[3m"
    }

    fn underline() -> &'static str {
        "\x1b[4m"
    }

    fn strikethrough() -> &'static str {
        "\x1b[9m"
    }

    fn reset_style() -> &'static str {
        "\x1b[22m"
    }
}

#[allow(dead_code)]
pub trait ColoredText {
    fn colorize(&self, color: Color) -> String;
    fn background(&self, color: Color) -> String;
    fn bold(&self) -> String;
    fn italic(&self) -> String;
    fn underline(&self) -> String;
    fn strikethrough(&self) -> String;
}

impl ColoredText for String {
    fn colorize(&self, color: Color) -> String {
        format!("{}{}{}", color.to_ansi_code(), self, Color::reset())
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

impl ColoredText for &str {
    fn colorize(&self, color: Color) -> String {
        format!("{}{}{}", color.to_ansi_code(), self, Color::reset())
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