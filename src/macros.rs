#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn warn(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

#[macro_export]
macro_rules! xformat_args {
    () => {};

    ($text:expr => $color:expr) => {
        format!("{}", $crate::xformat_args!(@colorize $text, $color))
    };

    ($text:expr) => {
        format!("{}", $text)
    };

    ($text:expr => $color:expr ; Debug) => {
        format!("{}", $crate::xformat_args!(@colorize format!("{:?}", $text), $color))
    };

    ($text:expr => $color:expr ; $format:expr) => {
        format!("{}", $crate::xformat_args!(@colorize $text, $color))
    };

    ($text:expr ; Debug) => {
        format!("{:?}", $text)
    };

    ($text:expr ; $format:expr) => {
        format!("{}", $text)
    };

    ($text:expr => $color:expr ; Debug, $($rest:tt)*) => {
        format!("{}{}", $crate::xformat_args!(@colorize format!("{:?}", $text), $color), $crate::xformat_args!($($rest)*))
    };

    ($text:expr => $color:expr ; $format:expr, $($rest:tt)*) => {
        format!("{}{}", $crate::xformat_args!(@colorize $text, $color), $crate::xformat_args!($($rest)*))
    };

    ($text:expr ; Debug, $($rest:tt)*) => {
        format!("{}{}", format!("{:?}", $text), $crate::xformat_args!($($rest)*))
    };

    ($text:expr ; $format:expr, $($rest:tt)*) => {
        format!("{}{}", format!("{}", $text), $crate::xformat_args!($($rest)*))
    };

    ($text:expr => $color:expr, $($rest:tt)*) => {
        format!("{}{}", $crate::xformat_args!(@colorize $text, $color), $crate::xformat_args!($($rest)*))
    };

    ($text:expr, $($rest:tt)*) => {
        format!("{}{}", format!("{}", $text), $crate::xformat_args!($($rest)*))
    };

    (@colorize $text:expr, $color:expr) => {{
        #[cfg(not(target_arch = "wasm32"))]
        {
            use broccli::colors::ColoredText;

            format!("{}", $text).colorize($color)
        }
        #[cfg(target_arch = "wasm32")]
        {
            $text
        }
    }};
}

#[macro_export]
macro_rules! xprint {
    ($($args:tt)*) => {{
        let text = $crate::xformat_args!($($args)*);

        print!("{}", text);

        #[cfg(target_arch = "wasm32")]
        $crate::macros::log(text);
    }};
}

#[macro_export]
macro_rules! xprintln {
    ($($args:tt)*) => {{
        let text = $crate::xformat_args!($($args)*);

        println!("{}", text);

        #[cfg(target_arch = "wasm32")]
        $crate::macros::log(&text);
    }};
}

#[macro_export]
macro_rules! xeprint {
    ($($args:tt)*) => {{
        let text = $crate::xformat_args!($($args)*);

        use broccli::colors::ColoredText;

        eprint!("{}{}", "error: ".colorize(axo_core::colors::Color::BrightRed), text);

        #[cfg(target_arch = "wasm32")]
        $crate::macros::error(&text);
    }};
}

#[macro_export]
macro_rules! xeprintln {
    ($($args:tt)*) => {{
        let text = $crate::xformat_args!($($args)*);

        use broccli::colors::ColoredText;

        eprintln!("{}{}", "error: ".colorize(axo_core::colors::Color::BrightRed), text);

        #[cfg(target_arch = "wasm32")]
        $crate::macros::error(&text);
    }};
}

#[macro_export]
macro_rules! xdprintln {
    ($($args:tt)*) => {{
        let text = $crate::xformat_args!($($args)*);

        use broccli::colors::ColoredText;

        println!("[{}:{}:{}] {}", file!(), line!(), column!(), text);

        #[cfg(target_arch = "wasm32")]
        $crate::macros::log(&text);
    }};
}