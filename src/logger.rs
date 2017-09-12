use core::fmt::Write;
use core::fmt;
use cortex_m_semihosting::hio;

pub enum LogLevel {
    l_info,
    l_warn,
    l_error,
    l_fatal
}

#[macro_export]
macro_rules! logger {
    ($level:expr, $($arg:tt)*) => {
        let mut stdout = hio::hstdout().unwrap();
        let log_color = match $level {
            LogLevel::l_info => "\x1b[00;36m",
            LogLevel::l_warn => "\x1b[00;33m",
            LogLevel::l_error => "\x1b[00;31m",
            LogLevel::l_fatal => "\x1b[37;41m"
        };
        let log_name = match $level {
            LogLevel::l_info => "INFO",
            LogLevel::l_warn => "WARN",
            LogLevel::l_error => "ERROR",
            LogLevel::l_fatal => "FATAL",
        };
        stdout.write_fmt(format_args!("{}{} |\t\t", log_color, log_name));
        stdout.write_fmt(format_args!($($arg)*));
        stdout.write_fmt(format_args!("\n"));
    };
}
