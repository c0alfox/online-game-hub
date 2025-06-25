#[macro_export]
macro_rules! log {
    ($level:literal, $($arg:tt)*) => {
        println!(
            "{} [{}] - {}",
            chrono::Local::now().format("%F %T%.9f"),
            $level,
            format!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        log!("INFO", $($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        log!("WARN", $($arg)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        log!("ERROR", $($arg)*)
    };
}