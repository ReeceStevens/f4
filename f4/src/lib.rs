#![cfg_attr(not(test), no_std)]
#![macro_use]
pub extern crate stm32f40x;
extern crate vcell;
extern crate cortex_m;
pub extern crate cortex_m_semihosting;
extern crate embedded_hal as hal;
#[macro_use]
extern crate nb;

#[macro_use]
pub mod logger;

pub mod rcc;
pub mod gpio;
pub mod spi;
pub mod timer;
pub mod usart;
pub mod adc;
pub mod exti;
pub mod i2c;
