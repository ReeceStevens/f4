#![no_std]
#![feature(const_fn)]
#![macro_use]
pub extern crate stm32f40x;
extern crate cortex_m;
pub extern crate cortex_m_semihosting;

#[macro_use]
pub mod logger;

pub mod rcc;
pub mod gpio;
pub mod spi;
pub mod timer;
pub mod usart;
