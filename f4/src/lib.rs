#![no_std]
#![feature(const_fn)]
#![cfg_attr(target_arch="arm", feature(core_intrinsics))]
#![feature(lang_items, libc)]
#![macro_use]
#![feature(compiler_builtins_lib)]
extern crate compiler_builtins;
pub extern crate stm32f40x;
#[cfg(target_arch = "arm")]
extern crate cortex_m;
pub extern crate cortex_m_semihosting;

#[cfg(target_arch = "arm")]
use core::intrinsics;

mod lang_items {
    #[lang = "panic_fmt"]
    #[no_mangle]
    extern "C" fn panic_fmt(msg: ::core::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
        unsafe  {
            use super::intrinsics;
            // use cortex_m_semihosting::hio as hio;
            use core::fmt::Write;
            // let mut stdout = hio::hstdout().unwrap();
            use cortex_m::itm::write_fmt;
            // stdout.write_fmt(format_args!("{}:{}:{}", file, line, col));
            // stdout.write_fmt(msg);
            write_fmt(format_args!("{}:{}:{}", file, line, col));
            write_fmt(msg);
            intrinsics::abort()
        }
    }
}

#[macro_use]
pub mod logger;

pub mod rcc;
pub mod gpio;
pub mod spi;
pub mod timer;
pub mod usart;
pub mod adc;

