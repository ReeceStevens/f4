#![allow(dead_code)]

use stm32f40x::{SPI1, SPI2, SPI4, GPIOA, GPIOB, GPIOC, RCC};
use gpio::AF5;
use gpio::gpioa::{PA5, PA6, PA7, PA11, PA1};
use gpio::gpiob::{PB14, PB15, PB13};
use gpio::gpioc::{PC7};

use stm32f40x::i2s2ext;


pub trait Sclk<SPI> {}
pub trait Miso<SPI> {}
pub trait Mosi<SPI> {}

impl Sclk<SPI1> for PA5<AF5> {}
impl Miso<SPI1> for PA6<AF5> {}
impl Mosi<SPI1> for PA7<AF5> {}

impl Sclk<SPI2> for PC7<AF5> {}
impl Miso<SPI2> for PB14<AF5> {}
impl Mosi<SPI2> for PB15<AF5> {}

impl Sclk<SPI2> for PB13<AF5> {}
impl Miso<SPI2> for PA11<AF5> {}
impl Mosi<SPI2> for PA1<AF5> {}

pub struct Spi<SPIX, PINS> {
    spi: SPIX,
    pins: PINS
}

pub trait DuplexTransfer {
    fn send(&self, data: u8);
    fn read(&self) -> u8;
}

macro_rules! spi {
    ($SPIx:ident, $spix:ident, $apb:ident, $spixen:ident) => {
        impl<SCLK, MISO, MOSI> Spi<$SPIx, (SCLK, MISO, MOSI)> {
            pub fn $spix(spi: $SPIx, pins: (SCLK, MISO, MOSI), rcc: &RCC) -> Self
                where SCLK: Sclk<$SPIx>, MISO: Miso<$SPIx>, MOSI: Mosi<$SPIx>
            {
                rcc.$apb.modify(|_, w| w.$spixen().set_bit());
                let spix = Spi { spi, pins };
                spix.channel_config();
                spix.spi.cr1.modify(|_, w| w.spe().set_bit());
                spix
            }

            fn channel_config(&self) {
                let spi_channel = &self.spi;
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
        }

        impl<PINS> DuplexTransfer for Spi<$SPIx, PINS> {
            fn send(&self, data: u8) {
                let spi = &self.spi;
                while spi.sr.read().txe().bit_is_clear() {}; // Tx buffer should be empty before we begin
                unsafe { spi.dr.write(|w| w.bits(data as u32)); }
                while spi.sr.read().txe().bit_is_clear() {}; // Wait until transmit complete
            }

            fn read(&self) -> u8 {
                let spi = &self.spi;
                while spi.sr.read().rxne().bit_is_clear() {}; // wait until receive complete
                while spi.sr.read().bsy().bit_is_set() {}; // Wait until SPI is not busy
                spi.dr.read().bits() as u8
            }
        }
    }
}

spi!(SPI1, spi1, apb2enr, spi1en);
spi!(SPI2, spi2, apb1enr, spi2en);
spi!(SPI4, spi4, apb2enr, spi4en);
