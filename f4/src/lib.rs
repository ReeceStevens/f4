#![no_std]
#![macro_use]
pub extern crate stm32f40x;
extern crate cortex_m;
pub extern crate cortex_m_semihosting;
extern crate embedded_hal as hal;
#[macro_use]
extern crate nb;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe  {
        use cortex_m::itm::write_fmt;
        use cortex_m::peripheral::ITM;
        let itm = &mut *ITM::ptr();
        write_fmt(&mut itm.stim[0], format_args!("{}", info));
        loop {}
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
