#[macro_export]
macro_rules! info {
    ($str:literal) => {
        crate::logging::log::info($str)
    };
    ($($arg:tt)*) => {
        crate::logging::log::info(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($str:literal) => {
        crate::logging::log::warn($str)
    };
    ($($arg:tt)*) => {
        crate::logging::log::warn(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($str:literal) => {
        crate::logging::log::error($str)
    };
    ($($arg:tt)*) => {
        crate::logging::log::error(&format!($($arg)*))
    };
}
