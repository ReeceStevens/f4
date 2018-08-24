#![no_std]
#![feature(const_fn, core_intrinsics)]
#![cfg_attr(target_arch="arm", feature(core_intrinsics))]
#![feature(panic_implementation, libc)]
#![macro_use]
pub extern crate stm32f40x;
extern crate cortex_m;
pub extern crate cortex_m_semihosting;
extern crate embedded_hal as hal;
extern crate nb;

#[cfg(target_arch = "arm")]
use core::intrinsics;
use core::panic::PanicInfo;

#[panic_implementation]
fn panic(info: &PanicInfo) -> ! {
    unsafe  {
        use core::intrinsics;
        use cortex_m::itm::write_fmt;
        use cortex_m::peripheral::ITM;
        let itm = &mut *ITM::ptr();
        write_fmt(&mut itm.stim[0], format_args!("{}", info));
        intrinsics::abort()
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
