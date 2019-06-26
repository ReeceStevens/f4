use core::marker::PhantomData;
use nb;
use stm32f40x::{ADC1, ADC_COMMON, RCC};
use hal::adc::{Channel, OneShot};

use gpio::{Analog, AnalogIn};
use gpio::gpioa::{PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7};
use gpio::gpiob::{PB0, PB1};
use gpio::gpioc::{PC0, PC1, PC2, PC3, PC4, PC5};

#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub enum ADC_SampleTime {
    cycles_3 = 0x00,
    cycles_15 = 0x01,
    cycles_28 = 0x02,
    cycles_56 = 0x03,
    cycles_84 = 0x04,
    cycles_112 = 0x05,
    cycles_144 = 0x06,
    cycles_480 = 0x07
}

impl Channel<ADC1> for PA0<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 0 }
}
impl Channel<ADC1> for PA1<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 1 }
}
impl Channel<ADC1> for PA2<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 2 }
}
impl Channel<ADC1> for PA3<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 3 }
}
impl Channel<ADC1> for PA4<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 4 }
}
impl Channel<ADC1> for PA5<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 5 }
}
impl Channel<ADC1> for PA6<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 6 }
}
impl Channel<ADC1> for PA7<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 7 }
}
impl Channel<ADC1> for PB0<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 8 }
}
impl Channel<ADC1> for PB1<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 9 }
}
impl Channel<ADC1> for PC0<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 10 }
}
impl Channel<ADC1> for PC1<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 11 }
}
impl Channel<ADC1> for PC2<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 12 }
}
impl Channel<ADC1> for PC3<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 13 }
}
impl Channel<ADC1> for PC4<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 14 }
}
impl Channel<ADC1> for PC5<Analog<AnalogIn>> {
    type ID = u8;
    fn channel() -> u8 { 15 }
}

pub struct Adc<ADCX, PIN> {
    pub adcx: ADCX,
    pub channel: u8,
    _pin: PhantomData<PIN>,
}

pub trait AdcConversion {
    fn read(&self) -> u16;
}

impl<PIN> Adc<ADC1, PIN> {
    pub fn adc1(adcx: ADC1, c_adc: &ADC_COMMON, sample_time: ADC_SampleTime, rcc: &RCC) -> Self
        where PIN: Channel<ADC1, ID=u8>
    {
        c_adc.ccr.reset();
        rcc.apb2enr.modify(|_, w| w.adc1en().set_bit());
        adcx.cr1.reset();
        adcx.cr2.modify(|_, w| w.adon().set_bit());

        let channel = PIN::channel();

        if channel > 9 {
            adcx.smpr1.modify(|r, w| unsafe {
                let smpr_mask = 0x07u32 << (3*(channel - 10));
                let smpr_bits = r.bits() & !smpr_mask;
                w.bits(smpr_bits | ((sample_time as u32) << (3*(channel - 10))))
            });
        } else {
            adcx.smpr2.modify(|r, w| unsafe {
                let smpr_mask = 0x07u32 << (3*channel);
                let smpr_bits = r.bits() & !smpr_mask;
                w.bits(smpr_bits | ((sample_time as u32) << (3*channel)))
            });
        }

        // TODO: this config assumes that ADC Rank == 1
        // This assumption is fine only when 1 ADC is in use.
        adcx.sqr3.modify(|r, w| unsafe {
            let rank = 1;
            let sqr_mask = 0x1Fu32 << (rank - 1);
            let sqr_bits = r.bits() & !sqr_mask;
            w.bits(sqr_bits | ((channel as u32) << (5 * (rank - 1))))
        });

        Adc { adcx, channel, _pin: PhantomData }
    }
}

impl<PIN> OneShot<ADC1, u16, PIN> for Adc<ADC1, PIN>
where
    PIN: Channel<ADC1, ID=u8>,
{
    type Error = ();

    fn read(&mut self, _pin: &mut PIN) -> nb::Result<u16, Self::Error> {
        let adc = &self.adcx;
        adc.cr2.modify(|_, w| w.swstart().set_bit());
        while adc.sr.read().eoc().bit_is_clear() {};
        Ok(adc.dr.read().data().bits())
    }
}
