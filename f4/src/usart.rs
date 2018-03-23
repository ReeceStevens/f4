use stm32f40x::{RCC, USART1, USART2, USART6};
use gpio::{AF7, AF8};
use gpio::gpioa::{PA9, PA15, PA10, PA2, PA3, PA11, PA12};
use gpio::gpiob::{PB3, PB7, PB6};
use rcc::get_pclk2;
use logger::*;

use core::fmt;

pub trait TxPin<USART> {}
pub trait RxPin<USART> {}

impl TxPin<USART1> for PA9<AF7> {}
impl TxPin<USART1> for PA15<AF7> {}
impl TxPin<USART1> for PB6<AF7> {}
impl RxPin<USART1> for PA10<AF7> {}
impl RxPin<USART1> for PB3<AF7> {}
impl RxPin<USART1> for PB7<AF7> {}

impl TxPin<USART2> for PA2<AF7> {}
impl RxPin<USART2> for PA3<AF7> {}

impl TxPin<USART6> for PA11<AF8> {}
impl RxPin<USART6> for PA12<AF8> {}

pub struct Usart<USART, PINS> {
    usart: USART,
    pins: PINS
}

pub trait Writer {
    fn send(&self, data: u8);
    fn print(&self, message: &[u8]) {
        for byte_char in message {
            self.send(*byte_char);
        }
    }
    fn println(&self, message: &[u8]) {
        self.print(message);
        self.send(b'\n');
    }
}


// TODO: Only set up for transmit right now
macro_rules! usart {
    ($USARTX:ident, $usartx:ident) => {
        impl<TX, RX> Usart<$USARTX, (TX, RX)> {
            pub fn $usartx(usart: $USARTX, baudrate: u32, pins: (TX, RX), rcc: &RCC) -> Self {
                rcc.apb2enr.modify(|_, w| w.usart1en().set_bit());
                rcc.apb2rstr.modify(|_, w| w.usart1rst().set_bit());
                rcc.apb2rstr.modify(|_, w| w.usart1rst().clear_bit());
                usart.cr1.modify(|_, w| w.m().clear_bit());
                usart.cr1.modify(|_, w| w.ps().clear_bit());
                usart.cr2.modify(|_, w| unsafe { w.stop().bits(0x00) });
                // Setting baud rate
                let over8 = if usart.cr1.read().over8().bit() {1} else {0};
                let apbclock = get_pclk2(rcc);
                let usart_div = (25 * apbclock) / (2*(2-over8)*baudrate);
                let brr_mantissa = usart_div / 100;
                let brr_fraction = match over8 {
                    1 => ((((usart_div - brr_mantissa) * 8) + 50) / 100) & 0x07,
                    _ => ((((usart_div - brr_mantissa) * 16) + 50) / 100) & 0x0F
                };

                usart.brr.modify(|_, w| unsafe { w.div_mantissa().bits(brr_mantissa as u16) });
                usart.brr.modify(|_, w| unsafe { w.div_fraction().bits(brr_fraction as u8) });

                usart.cr1.modify(|_, w| w.te().set_bit());
                usart.cr1.modify(|_, w| w.ue().set_bit());

                Usart { usart, pins }
            }
        }

        impl<PINS> Writer for Usart<$USARTX, PINS> {
            fn send(&self, data: u8) {
                while self.usart.sr.read().txe().bit_is_clear() {};
                self.usart.dr.write(|w| unsafe { w.dr().bits(data as u16) });
                while self.usart.sr.read().tc().bit_is_clear() {};
            }
        }

        impl<PINS> fmt::Write for Usart<$USARTX, PINS> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.print(s.as_bytes());
                Ok(())
            }
        }
    }
}

usart!(USART1, usart1);
usart!(USART2, usart2);
