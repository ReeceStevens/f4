#![no_std]
#![ feature ( const_fn ) ]
#![macro_use]
pub extern crate stm32f40x;
extern crate cortex_m;
extern crate cortex_m_semihosting;

pub mod gpio;
pub mod logger;
