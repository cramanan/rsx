/// Log a message to the JavaScript console if on wasm32. Otherwise logs it to stdout.
///
/// Note: this does not work properly for server-side WASM since it will mistakenly try to log to
/// the JS console.
#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => {
            web_sys::console::log_1(&::std::format!($($arg)*).into());
    };
}

/// Log a warning to the JavaScript console if on wasm32. Otherwise logs it to stderr.
///
/// Note: this does not work properly for server-side WASM since it will mistakenly try to log to
/// the JS console.
#[macro_export]
macro_rules! console_warn {
    ($($arg:tt)*) => {
        web_sys::console::warn_1(&::std::format!($($arg)*).into());
    };
}

/// Prints an error message to the JavaScript console if on wasm32. Otherwise logs it to stderr.
///
/// Note: this does not work properly for server-side WASM since it will mistakenly try to log to
/// the JS console.
#[macro_export]
macro_rules! console_error {
    ($($arg:tt)*) => {
        web_sys::console::error_1(&::std::format!($($arg)*).into());
    };
}

/// Debug the value of a variable to the JavaScript console if on wasm32. Otherwise logs it to
/// stdout.
///
/// Note: this does not work properly for server-side WASM since it will mistakenly try to log to
/// the JS console.
#[macro_export]
macro_rules! console_dbg {
    () => {
        web_sys::console::log_1(
            &::std::format!("[{}:{}]", ::std::file!(), ::std::line!(),).into(),
        );
        ::std::dbg!($arg);
    };

    ($arg:expr $(,)?) => {
        let arg = $arg;
        $crate::rt::web_sys::console::log_1(
            &::std::format!(
                "[{}:{}] {} = {:#?}",
                ::std::file!(),
                ::std::line!(),
                ::std::stringify!($arg),
                arg
            )
            .into(),
        );
        arg
    };

    ($($arg:expr),+ $(,)?) => {
        $($crate::console_dbg!($arg);)+
    }
}
