#![allow(dead_code)]
#![allow(unused_variables)]
use stm32f40x::{RCC, I2C1, I2C2, I2C3};
use nb::Error::{Other, WouldBlock};
use hal::blocking::i2c::{Read, Write};

use rcc::Clocks;
use logger::*;

use gpio::{AF4};
use gpio::gpioa::{PA8};
use gpio::gpiob::{PB6, PB7, PB8, PB9, PB10, PB11};
use gpio::gpioc::PC9;

pub trait Sda<I2C> {}
pub trait Scl<I2C> {}

impl Scl<I2C1> for PB6<AF4> {}
impl Sda<I2C1> for PB7<AF4> {}
impl Scl<I2C1> for PB8<AF4> {}
impl Sda<I2C1> for PB9<AF4> {}

impl Scl<I2C2> for PB10<AF4> {}
impl Sda<I2C2> for PB11<AF4> {}

impl Scl<I2C3> for PA8<AF4> {}
impl Sda<I2C3> for PC9<AF4> {}

#[derive(Debug)]
pub enum Error {
    BusError,
    AckFailure,
    ArbitrationLoss,
    OverUnderrunError
}

pub enum Mode {
    Standard,
    Fast { duty: Duty },
}

pub enum Duty {
    Ratio2to1,
    Ratio16to9,
}

pub struct I2c<I2CX, SCL, SDA> {
    i2c: I2CX,
    scl: SCL,
    sda: SDA,
    freq: u32,
    mode: Mode
}

macro_rules! wait_for_flag {
    ($i2c:expr, $flag:ident) => {
        {
            let sr1 = $i2c.sr1.read();

            if sr1.berr().bit_is_set() {
                Err(Other(Error::BusError))
            } else if sr1.arlo().bit_is_set() {
                Err(Other(Error::ArbitrationLoss))
            } else if sr1.af().bit_is_set() {
                Err(Other(Error::AckFailure))
            } else if sr1.ovr().bit_is_set() {
                Err(Other(Error::OverUnderrunError))
            } else if sr1.$flag().bit_is_set() {
                Ok(())
            } else {
                Err(WouldBlock)
            }
        }
    }
}

macro_rules! i2c_setup {
    ($($I2CX:ident: ($i2cx:ident, $i2cxen:ident, $i2cxrst:ident),)+) => {
    $(
        impl<SCL, SDA> I2c<$I2CX, SCL, SDA> {
            pub fn $i2cx(
                rcc: &RCC,
                i2c: $I2CX,
                scl: SCL,
                sda: SDA,
                freq: u32,
                mode: Mode,
                clocks: &Clocks,
            ) -> Self
                where SCL: Scl<$I2CX>, SDA: Sda<$I2CX>
            {
                // Enable peripheral bus
                rcc.apb1enr.modify(|r, w| w.$i2cxen().set_bit());
                rcc.apb1rstr.modify(|r, w| w.$i2cxrst().set_bit());
                rcc.apb1rstr.modify(|r, w| w.$i2cxrst().clear_bit());
                let i2c_interface =  I2c { i2c, scl, sda, freq, mode };
                i2c_interface.init(clocks.pclk1);
                i2c_interface
            }


            fn init(&self, pclk1: u32) {
                let pclk1_mhz = (pclk1 / 1_000_000_u32).min(2);

                // Disable i2c while configuring
                self.i2c.cr1.modify(|r, w| w.pe().clear_bit());

                self.i2c.cr2.modify(|r, w| unsafe {w.freq().bits(pclk1_mhz as u8)});

                match &self.mode {
                    Mode::Standard => {
                        let ccr =  (pclk1 / (2 * self.freq as u32)).max(4);
                        self.i2c.ccr.write(|w| unsafe {
                            w.ccr().bits(ccr as u16)
                             .f_s().clear_bit()
                        });
                        let trise = pclk1_mhz + 1;
                        self.i2c.trise.write(|w| unsafe {w.trise().bits(trise as u8)});
                    },
                    Mode::Fast { duty } => {
                        let trise = pclk1_mhz * 300 / 1000 + 1; // TODO: Why is this heuristic right??
                        self.i2c.trise.write(|w| unsafe {w.trise().bits(trise as u8)});

                        match duty {
                            Duty::Ratio2to1 => {
                                let ccr =  (pclk1 / (3 * self.freq as u32)).max(1);
                                self.i2c.ccr.write(|w| unsafe {
                                    w.ccr().bits(ccr as u16)
                                     .duty().clear_bit()
                                     .f_s().set_bit()
                                });
                            },
                            Duty::Ratio16to9 => {
                                let ccr =  (pclk1 / (25 * self.freq as u32)).max(1);
                                self.i2c.ccr.write(|w| unsafe {
                                    w.ccr().bits(ccr as u16)
                                     .duty().set_bit()
                                     .f_s().set_bit()
                                });
                            },
                        }
                    },
                }

                // Re-enable i2c
                self.i2c.cr1.modify(|r, w| w.pe().set_bit());
            }

            fn transmit(&self, address: u8, payload: &[u8]) -> Result<(), Error> {
                // Send start bit
                self.i2c.cr1.modify(|r, w| w.start().set_bit());
                block!(wait_for_flag!(self.i2c, sb))?;
                // Send address (there are 7 and 10 bit variants, only addressing 7 here)
                self.i2c.dr.modify(|r, w| unsafe { w.dr().bits(address << 1) });
                block!(wait_for_flag!(self.i2c, addr))?;

                self.i2c.sr2.read();

                block!(wait_for_flag!(self.i2c, tx_e))?;
                for chunk in payload {
                    self.i2c.dr.modify(|r, w| unsafe { w.dr().bits(*chunk) });
                    block!(wait_for_flag!(self.i2c, tx_e))?;
                }

                self.i2c.cr1.modify(|r, w| w.stop().set_bit());
                Ok(())
            }

            fn receive(&self, address: u8, buffer: &mut [u8]) -> Result<(), Error> {
                // Send start bit
                self.i2c.cr1.modify(|r, w| w.start().set_bit());
                block!(wait_for_flag!(self.i2c, sb))?;

                match buffer.len() {
                    1 => {

                        // Send address (there are 7 and 10 bit variants, only addressing 7 here)
                        self.i2c.dr.write(|w| unsafe { w.dr().bits(address << 1 | 0x01) });
                        block!(wait_for_flag!(self.i2c, addr))?;

                        self.i2c.cr1.modify(|r, w| w.ack().clear_bit());
                        self.i2c.sr2.read();
                        self.i2c.cr1.modify(|r, w| w.stop().set_bit());
                        block!(wait_for_flag!(self.i2c, rx_ne))?;
                        buffer[0] = self.i2c.dr.read().dr().bits();

                    },
                    2 => {
                        self.i2c.cr1.modify(|r, w| w.ack().set_bit().pos().set_bit());
                        // Send address (there are 7 and 10 bit variants, only addressing 7 here)
                        self.i2c.dr.write(|w| unsafe { w.dr().bits(address << 1 | 0x01) });
                        block!(wait_for_flag!(self.i2c, addr))?;
                        self.i2c.sr2.read();

                        self.i2c.cr1.modify(|r, w| w.ack().clear_bit().pos().set_bit());
                        self.i2c.sr2.read();
                        block!(wait_for_flag!(self.i2c, btf))?;
                        self.i2c.cr1.modify(|r, w| w.stop().set_bit());
                        buffer[0] = self.i2c.dr.read().dr().bits();
                        buffer[1] = self.i2c.dr.read().dr().bits();
                    },
                    buf_length => {
                        self.i2c.cr1.modify(|r, w| w.ack().set_bit().pos().set_bit());
                        // Send address (there are 7 and 10 bit variants, only addressing 7 here)
                        self.i2c.dr.write(|w| unsafe { w.dr().bits(address << 1 | 0x01) });
                        block!(wait_for_flag!(self.i2c, addr))?;
                        self.i2c.sr2.read();

                        let (leading_bytes, last_three_bytes) = buffer.split_at_mut(buf_length - 3);
                        for byte in leading_bytes {
                            self.i2c.cr1.modify(|r, w| w.ack().set_bit()); // TODO do we actually need this line? seems like overkill
                            block!(wait_for_flag!(self.i2c, rx_ne))?;
                            *byte = self.i2c.dr.read().dr().bits();
                        }

                        block!(wait_for_flag!(self.i2c, btf))?;
                        self.i2c.cr1.modify(|r, w| w.ack().clear_bit());
                        last_three_bytes[0] = self.i2c.dr.read().dr().bits();

                        block!(wait_for_flag!(self.i2c, btf))?;
                        self.i2c.cr1.modify(|r, w| w.stop().set_bit());
                        last_three_bytes[1] = self.i2c.dr.read().dr().bits();
                        last_three_bytes[2] = self.i2c.dr.read().dr().bits();
                    },
                }
                Ok(())
            }
        }

        impl<SCL, SDA> Write for I2c<$I2CX, SCL, SDA> {
            type Error = Error;

            fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
                self.transmit(addr, bytes)
            }
        }

        impl<SCL, SDA> Read for I2c<$I2CX, SCL, SDA> {
            type Error = Error;

            fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
                self.receive(addr, buffer)
            }
        }
    )+
    }
}

i2c_setup! {
    I2C1: (i2c1, i2c1en, i2c1rst),
    I2C2: (i2c2, i2c2en, i2c2rst),
    I2C3: (i2c3, i2c3en, i2c3rst),
}
