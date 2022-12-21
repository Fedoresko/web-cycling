#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        web_sys::console::log_1(&res.into())
    }}
}