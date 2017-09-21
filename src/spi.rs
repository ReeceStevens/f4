// use stm32f40x::{SPI1, SPI2, SPI3};
use stm32f40x::{SpiRegisters, SPI1, SPI2, SPI3, SPI4, SPI5, GPIOA, GPIOB, GPIOC, RCC};
use gpio::{GPIO_AF};
use gpio::{PA5, PA6, PA7, PB10, PB14, PC7};

pub enum SPI {
    SPI1,
    SPI2,
    SPI3
}

pub struct Spi1;
impl Spi1 {
    fn config(&self, spi1: &SPI1, gpioa: &GPIOA, rcc: &RCC) {
        rcc.apb2enr.write(|w| w.spi1en().set_bit());
        PA5.af_init(GPIO_AF::AF5_SPI1, rcc, gpioa);
        PA6.af_init(GPIO_AF::AF5_SPI1, rcc, gpioa);
        PA7.af_init(GPIO_AF::AF5_SPI1, rcc, gpioa);
        channel_config(&spi1);
    }
}

pub struct Spi2;
impl Spi2 {
    fn config(&self, spi2: &SPI2, gpiob: &GPIOB, gpioc: &GPIOC, rcc: &RCC) {
        rcc.apb1enr.write(|w| w.spi2en().set_bit());
        PC7.af_init(GPIO_AF::AF5_SPI1, rcc, gpioc);
        PB14.af_init(GPIO_AF::AF5_SPI1, rcc, gpiob);
        PB10.af_init(GPIO_AF::AF5_SPI1, rcc, gpiob);
        channel_config(&spi2);
    }
}

pub struct Spi4;
impl Spi4 {
    fn config(&self, spi4: &SPI4, gpioa: &GPIOA, gpiob: &GPIOB, rcc: &RCC) {
        rcc.apb2enr.write(|w| w.spi4en().set_bit());
        // PC7.af_init(GPIO_AF::AF5_SPI1, rcc, gpioc);
        // PB14.af_init(GPIO_AF::AF5_SPI1, rcc, gpiob);
        // PB10.af_init(GPIO_AF::AF5_SPI1, rcc, gpiob);
        channel_config(&spi4);
    }
}

// TODO: figure out why SPI4 is definitely defined
// but not found in the stm32f40x generated library.
// pub struct Spi3;
// impl Spi3 {
//     fn config(&self, spi3: &SPI3, gpioa: &GPIOA, rcc: &RCC) {
//         rcc.apb2enr.write(|w| w.spi1en().set_bit());
//         PA5.af_init(GPIO_AF::AF5_SPI1, rcc, gpioa);
//         PA6.af_init(GPIO_AF::AF5_SPI1, rcc, gpioa);
//         PA7.af_init(GPIO_AF::AF5_SPI1, rcc, gpioa);
//         channel_config(&spi1);
//     }
// }

/*NOTE: Safe if called from init fcn */
// pub unsafe fn config(spix: SPI, spi_peripheral: SpiRegisters) {
//     match spix {
//         SPI::SPI1 => {
//             channel_config(&(*SPI1.get()))
//         },
//         SPI::SPI2 => {
//             channel_config(&(*SPI2.get()))
//         },
//         SPI::SPI3 => {
//             channel_config(&(*SPI3.get()))
//         }
//     }
// }

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

pub fn enable(spi_channel: &SpiRegisters) {
    spi_channel.cr1.write(|w| w.spe().set_bit());
}


// macro_rules! setup_spi {
//     ($spi:ident, $spi_refname:ident) => {
//         pub struct $spi_refname;
//         impl $spi_refname {
//             pub fn configure(&self, spi_bus: &SpiRegisters) {
//                 /* CR1 Init */
//                 // SPI Direction (full duplex mode)
//                 spi_bus.cr1.write(|w| w.bidimode().clear_bit());
//                 spi_bus.cr1.write(|w| w.bidioe().clear_bit());
//                 spi_bus.cr1.write(|w| w.rxonly().clear_bit());

//                 // SPI_Mode and SPI_NSS
//                 // (SSM, SSI, MSTR) (set as master)
//                 spi_bus.cr1.write(|w| w.ssm().set_bit());
//                 spi_bus.cr1.write(|w| w.ssi().set_bit());
//                 spi_bus.cr1.write(|w| w.mstr().set_bit());

//                 // SPI_FirstBit -> (LSBFirst) (set as MSB first)
//                 spi_bus.cr1.write(|w| w.lsbfirst().clear_bit());

//                 // SPI Datasize (8 bit -> clear DFF)
//                 spi_bus.cr1.write(|w| w.dff().clear_bit());

//                 // SPI_BaudRatePrescaler -> BR (prescaled by 16)
//                 unsafe { spi_bus.cr1.write(|w| w.br().bits(0x18)); }

//                 // SPI_CPOL, SPI_CPHA (CPOL low, leading edge/1Edge)
//                 spi_bus.cr1.write(|w| w.cpol().clear_bit());
//                 spi_bus.cr1.write(|w| w.cpha().clear_bit());

//                 spi_bus.i2scfgr.write(|w| w.i2smod().clear_bit());
//                 unsafe { spi_bus.crcpr.write(|w| w.crcpoly().bits(7)); }
//             }

//             pub fn enable(&self, spi_bus: &$spi) {
//                 spi_bus.cr1.write(|w| w.spe().set_bit());
//             }
//         }
//     }
// }

// setup_spi!(SPI1, Spi1);
