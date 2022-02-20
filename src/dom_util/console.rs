/// console log macro
///
/// ```
/// macro_rules! console_log {
///    () => {
///        web_sys::console::log_1();
///    };
///    ($($t:tt)*) => {
///        web_sys::console::log_1(&format!($($t)*).into());
///    };
///}
/// ```
#[macro_export]
macro_rules! console_log {
    () => {
        web_sys::console::log_1();
    };
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    };
}
