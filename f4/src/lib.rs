#![no_std]
#![feature(const_fn)]
#![cfg_attr(target_arch="arm", feature(core_intrinsics))]
#![feature(lang_items, libc)]
#![macro_use]
pub extern crate stm32f40x;
#[cfg(target_arch = "arm")]
extern crate cortex_m;
pub extern crate cortex_m_semihosting;
extern crate embedded_hal as hal;
extern crate nb;

#[cfg(target_arch = "arm")]
use core::intrinsics;

mod lang_items {
    #[lang = "panic_fmt"]
    #[no_mangle]
    extern "C" fn panic_fmt(msg: ::core::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
        unsafe  {
            use super::intrinsics;
            use cortex_m::itm::write_fmt;
            use cortex_m::peripheral::ITM;
            let itm = &mut *ITM::ptr();
            write_fmt(&mut itm.stim[0], format_args!("{}:{}:{}", file, line, col));
            write_fmt(&mut itm.stim[0], msg);
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
pub mod exti;
