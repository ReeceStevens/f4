use stm32f40x::{RCC, GPIOA, GPIOB, USART1};
use gpio::{GPIOConfig, GPIO_AF};
use gpio::{PA9, PA15, PB6};
use rcc::get_pclk2;
use logger::*;


#[derive(Clone,Copy)]
pub enum USART_Channel {
    USART1,
    USART2,
    USART6
}

pub enum USART_Mode {
    TX,
    RX
}

pub trait USART {
    fn configure(&self, rcc: &RCC, gpiob: &GPIOB, mode: USART_Mode, baudrate: u32);
    fn putcharx(&self, byte_char: u8);
    // fn getcharx() -> u8; TODO: Not yet implemented
    fn print(&self, message: &[u8]) {
        for byte_char in message {
            self.putcharx(*byte_char);
        }
        self.putcharx(b'\n');
    }
}

pub struct USART_1<'a>(pub &'a USART1);

impl<'a> USART for USART_1<'a> {
    fn configure(&self, rcc: &RCC, gpiob: &GPIOB, mode: USART_Mode, baudrate: u32) {
        let usart = &self.0;

        let config = GPIOConfig::new_af(GPIO_AF::AF7_USART1);
        // logger!(LogLevel::l_info, "GPIO AF val: {}", config.af.af_to_val());
        PB6::init(config, rcc, gpiob);
        // PA9::init(config, rcc, gpioa);
        // PA15::init(config, rcc, gpioa);

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
        // let brr = apbclock / baudrate;
        // usart.brr.write(|w| unsafe { w.bits(brr) });
        // logger!(LogLevel::l_info, "Baud rate: {}", baudrate);
        // logger!(LogLevel::l_info, "brr val: {}", usart.brr.read().bits());
        // logger!(LogLevel::l_info, "usart_div val: {}", usart_div);

        usart.cr1.modify(|_, w| w.te().set_bit());
        usart.cr1.modify(|_, w| w.ue().set_bit());

    }
    fn putcharx(&self, byte_char: u8) {
        let usart = self.0;
        while usart.sr.read().txe().bit_is_clear() {};
        usart.dr.write(|w| unsafe { w.dr().bits(byte_char as u16) });
        while usart.sr.read().tc().bit_is_clear() {};
    }
}
