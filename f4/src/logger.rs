#[allow(non_camel_case_types)]
pub enum LogLevel {
    l_info,
    l_warn,
    l_error,
    l_fatal
}

#[macro_export]
macro_rules! logger {
    ($level:expr, $($arg:tt)*) => {
        {
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
            unsafe {
                use cortex_m::peripheral::ITM;
                let itm = &mut *ITM::ptr();
                use cortex_m::itm::{write_all, write_fmt};
                write_fmt(&mut itm.stim[0], format_args!("{}{}:{}:{}|\t\t", log_color, log_name, file!(), line!()));
                write_fmt(&mut itm.stim[0], format_args!($($arg)*));
                write_fmt(&mut itm.stim[0], format_args!("\n"));
            }
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        logger!(LogLevel::l_info, $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        logger!(LogLevel::l_warn, $($arg)*);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        logger!(LogLevel::l_error, $($arg)*);
    };
}
