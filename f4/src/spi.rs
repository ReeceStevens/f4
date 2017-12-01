#![allow(dead_code)]

use stm32f40x::{SpiRegisters, SPI1, SPI2, SPI4, GPIOA, GPIOB, GPIOC, RCC};
use gpio::{GPIO_AF, GPIOConfig};
use gpio::{PA5, PA6, PA7, PB10, PB14,
           PC7, PB13, PA11, PA1};

pub enum SPI {
    SPI1,
    SPI2,
    SPI3
}

pub struct Spi1;
impl Spi1 {
    fn config(spi1: &SPI1, gpioa: &GPIOA, rcc: &RCC) {
        rcc.apb2enr.write(|w| w.spi1en().set_bit());
        let config = GPIOConfig::new_af(GPIO_AF::AF5_SPI1);
        PA5.init(&config, rcc, gpioa);
        PA6.init(&config, rcc, gpioa);
        PA7.init(&config, rcc, gpioa);
        channel_config(&spi1);
    }
    fn enable(spi1: &SPI1) {
        spi1.cr1.write(|w| w.spe().set_bit());
    }
}

pub struct Spi2;
impl Spi2 {
    fn config(&self, spi2: &SPI2, gpiob: &GPIOB, gpioc: &GPIOC, rcc: &RCC) {
        rcc.apb1enr.write(|w| w.spi2en().set_bit());
        let config = GPIOConfig::new_af(GPIO_AF::AF5_SPI1);
        PC7.init(&config, rcc, gpioc);
        PB14.init(&config, rcc, gpiob);
        PB10.init(&config, rcc, gpiob);
        channel_config(&spi2);
    }
    fn enable(&self, spi2: &SPI2) {
        spi2.cr1.write(|w| w.spe().set_bit());
    }
}

pub struct Spi4;
impl Spi4 {
    fn config(&self, spi4: &SPI4, gpioa: &GPIOA, gpiob: &GPIOB, rcc: &RCC) {
        rcc.apb2enr.write(|w| w.spi4en().set_bit());
        let config = GPIOConfig::new_af(GPIO_AF::AF6_SPI4);
        PB13.init(&config, rcc, gpiob);
        PA11.init(&config, rcc, gpioa);
        let config = GPIOConfig::new_af(GPIO_AF::AF5_SPI4);
        PA1.init(&config, rcc, gpioa);
        channel_config(&spi4);
    }
    fn enable(&self, spi4: &SPI4) {
        spi4.cr1.write(|w| w.spe().set_bit());
    }
}

fn channel_config(spi_channel: &SpiRegisters) {
    /* CR1 Init */
    // SPI Direction (full duplex mode)
    spi_channel.cr1.write(|w| w.bidimode().clear_bit());
    spi_channel.cr1.write(|w| w.bidioe().clear_bit());
    spi_channel.cr1.write(|w| w.rxonly().clear_bit());

    // SPI_Mode and SPI_NSS
    // (SSM, SSI, MSTR) (set as master)
    spi_channel.cr1.write(|w| w.ssm().set_bit());
    spi_channel.cr1.write(|w| w.ssi().set_bit());
    spi_channel.cr1.write(|w| w.mstr().set_bit());

    // SPI_FirstBit -> (LSBFirst) (set as MSB first)
    spi_channel.cr1.write(|w| w.lsbfirst().clear_bit());

    // SPI Datasize (8 bit -> clear DFF)
    spi_channel.cr1.write(|w| w.dff().clear_bit());

    // SPI_BaudRatePrescaler -> BR (prescaled by 16)
    unsafe { spi_channel.cr1.write(|w| w.br().bits(0x18)); }

    // SPI_CPOL, SPI_CPHA (CPOL low, leading edge/1Edge)
    spi_channel.cr1.write(|w| w.cpol().clear_bit());
    spi_channel.cr1.write(|w| w.cpha().clear_bit());

    spi_channel.i2scfgr.write(|w| w.i2smod().clear_bit());
    unsafe { spi_channel.crcpr.write(|w| w.crcpoly().bits(7)); }
}

pub fn transfer(data: u8, spix: &SpiRegisters) -> u8 {
    unsafe { spix.dr.write(|w| w.bits(data as u32)); }
    while spix.sr.read().txe().bit_is_clear() {}; // Wait until transmit complete
    while spix.sr.read().rxne().bit_is_clear() {}; // wait until receive complete
    while spix.sr.read().bsy().bit_is_set() {}; // Wait until SPI is not busy
    spix.dr.read().bits() as u8
}
