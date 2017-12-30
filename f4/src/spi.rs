#![allow(dead_code)]

use stm32f40x::{SpiRegisters, SPI1, SPI2, SPI4, GPIOA, GPIOB, GPIOC, RCC};
use gpio::{GPIO_AF, GPIOConfig};
use gpio::{PA5, PA6, PA7, PB10, PB14,
           PC7, PB13, PA11, PA1};

use stm32f40x::i2s2ext;

pub struct Spi1<'a> {
    dr: &'a i2s2ext::DR,
    sr: &'a i2s2ext::SR
}

impl<'a> Spi1<'a> {
    pub fn init(spi1: &SPI1, rcc: &RCC, gpioa: &GPIOA) {
        rcc.apb2enr.modify(|_, w| w.spi1en().set_bit());
        let config = GPIOConfig::new_af(GPIO_AF::AF5_SPI1);
        PA5::init(config, rcc, gpioa);
        PA6::init(config, rcc, gpioa);
        PA7::init(config, rcc, gpioa);
        channel_config(&spi1);
    }
    pub fn get_reference(spi1: &SPI1) -> Spi1 {
        Spi1 {
            dr: &spi1.dr,
            sr: &spi1.sr
        }
    }
    pub fn enable(spi1: &SPI1) {
        spi1.cr1.modify(|_, w| w.spe().set_bit());
    }
    pub fn transfer(&self, data: u8) -> u8 {
        while self.sr.read().txe().bit_is_clear() {}; // Tx buffer should be empty before we begin
        unsafe { self.dr.write(|w| w.bits(data as u32)); }
        while self.sr.read().txe().bit_is_clear() {}; // Wait until transmit complete
        while self.sr.read().rxne().bit_is_clear() {}; // wait until receive complete
        while self.sr.read().bsy().bit_is_set() {}; // Wait until SPI is not busy
        self.dr.read().bits() as u8
    }
}

pub struct Spi2<'a> {
    dr: &'a i2s2ext::DR,
    sr: &'a i2s2ext::SR
}

impl<'a> Spi2<'a> {
    pub fn init(spi2: &SPI2, gpiob: &GPIOB, gpioc: &GPIOC, rcc: &RCC) {
        rcc.apb1enr.modify(|_, w| w.spi2en().set_bit());
        let config = GPIOConfig::new_af(GPIO_AF::AF5_SPI1);
        PC7::init(config, rcc, gpioc);
        PB14::init(config, rcc, gpiob);
        PB10::init(config, rcc, gpiob);
        channel_config(&spi2);
    }
    pub fn get_reference(spi2: &SPI2) -> Spi2 {
        Spi2 {
            dr: &spi2.dr,
            sr: &spi2.sr
        }
    }
    pub fn enable(spi2: &SPI2) {
        spi2.cr1.modify(|_, w| w.spe().set_bit());
    }
    pub fn transfer(&self, data: u8) -> u8 {
        while self.sr.read().txe().bit_is_clear() {}; // Tx buffer should be empty before we begin
        unsafe { self.dr.write(|w| w.bits(data as u32)); }
        while self.sr.read().txe().bit_is_clear() {}; // Wait until transmit complete
        while self.sr.read().rxne().bit_is_clear() {}; // wait until receive complete
        while self.sr.read().bsy().bit_is_set() {}; // Wait until SPI is not busy
        self.dr.read().bits() as u8
    }
}

pub struct Spi4<'a> {
    dr: &'a i2s2ext::DR,
    sr: &'a i2s2ext::SR
}

impl<'a> Spi4<'a> {
    pub fn init(spi4: &SPI4, gpioa: &GPIOA, gpiob: &GPIOB, rcc: &RCC) {
        rcc.apb2enr.modify(|_, w| w.spi4en().set_bit());
        let config = GPIOConfig::new_af(GPIO_AF::AF6_SPI4);
        PB13::init(config, rcc, gpiob);
        PA11::init(config, rcc, gpioa);
        let config = GPIOConfig::new_af(GPIO_AF::AF5_SPI4);
        PA1::init(config, rcc, gpioa);
        channel_config(&spi4);
    }
    pub fn get_reference(spi4: &SPI4) -> Spi4 {
        Spi4 {
            dr: &spi4.dr,
            sr: &spi4.sr
        }
    }
    pub fn enable(spi4: &SPI4) {
        spi4.cr1.modify(|_, w| w.spe().set_bit());
    }
    pub fn transfer(&self, data: u8) -> u8 {
        while self.sr.read().txe().bit_is_clear() {}; // Tx buffer should be empty before we begin
        unsafe { self.dr.write(|w| w.bits(data as u32)); }
        while self.sr.read().txe().bit_is_clear() {}; // Wait until transmit complete
        while self.sr.read().rxne().bit_is_clear() {}; // wait until receive complete
        while self.sr.read().bsy().bit_is_set() {}; // Wait until SPI is not busy
        self.dr.read().bits() as u8
    }
}

fn channel_config(spi_channel: &SpiRegisters) {
    /* CR1 Init */

    // SPI_BaudRatePrescaler -> BR (prescaled by 16)
    unsafe { spi_channel.cr1.modify(|_, w| w.br().bits(0x18)); }

    // SPI_CPOL, SPI_CPHA (CPOL low, leading edge/1Edge)
    spi_channel.cr1.modify(|_, w| w.cpol().clear_bit());
    spi_channel.cr1.modify(|_, w| w.cpha().clear_bit());

    // SPI Datasize (8 bit -> clear DFF)
    spi_channel.cr1.modify(|_, w| w.dff().clear_bit());

    // SPI_FirstBit -> (LSBFirst) (set as MSB first)
    spi_channel.cr1.modify(|_, w| w.lsbfirst().clear_bit());

    // SPI Direction (full duplex mode)
    spi_channel.cr1.modify(|_, w| w.bidimode().clear_bit());
    spi_channel.cr1.modify(|_, w| w.bidioe().clear_bit());
    spi_channel.cr1.modify(|_, w| w.rxonly().clear_bit());

    // SPI_Mode and SPI_NSS
    // (SSM, SSI, MSTR) (set as master)
    spi_channel.cr1.modify(|_, w| w.ssm().set_bit());
    spi_channel.cr1.modify(|_, w| w.ssi().set_bit());
    spi_channel.cr1.modify(|_, w| w.mstr().set_bit());

    spi_channel.i2scfgr.modify(|_, w| w.i2smod().clear_bit());
    unsafe { spi_channel.crcpr.modify(|_, w| w.crcpoly().bits(7)); }
}
