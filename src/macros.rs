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

#[cfg(feature = "xprint")]
#[macro_export]
macro_rules! xprintln {
    ($($args:tt)*) => {
        println!("{}", xformat_args!($($args)*));
    };
}

#[cfg(feature = "xprint")]
#[macro_export]
macro_rules! xprint {
    ($($args:tt)*) => {
        print!("{}", xformat_args!($($args)*));
    };
}

#[cfg(feature = "xprint")]
#[macro_export]
macro_rules! xprintb {
    ($($args:tt)*) => {
        println!("{}", xformat_block!($($args)*, 0));
    };
}

#[cfg(feature = "xprint")]
#[macro_export]
macro_rules! xeprint {
    ($($args:tt)*) => {{
        let text = $crate::xformat_args!($($args)*);

        use broccli::colors::ColoredText;

        eprint!("{}{}", "error: ".colorize($crate::colors::Color::BrightRed), text);

        #[cfg(target_arch = "wasm32")]
        $crate::macros::error(&text);
    }};
}

#[cfg(feature = "xprint")]
#[macro_export]
macro_rules! xeprintln {
    ($($args:tt)*) => {{
        let text = $crate::xformat_args!($($args)*);

        use broccli::colors::ColoredText;

        eprintln!("{}{}", "error: ".colorize($crate::colors::Color::BrightRed), text);

        #[cfg(target_arch = "wasm32")]
        $crate::macros::error(&text);
    }};
}

#[cfg(feature = "xprint")]
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