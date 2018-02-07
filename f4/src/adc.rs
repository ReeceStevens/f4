use stm32f40x::{ADC1, ADC_COMMON, RCC};

use gpio::{Analog, AnalogIn};
use gpio::gpioa::{PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7};
use gpio::gpiob::{PB0, PB1};
use gpio::gpioc::{PC0, PC1, PC2, PC3, PC4, PC5};

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


pub trait ADC_Pin<ADCX> {
    fn get_channel(&self) -> u8;
}

impl ADC_Pin<ADC1> for PA0<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 0 }
}
impl ADC_Pin<ADC1> for PA1<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 1 }
}
impl ADC_Pin<ADC1> for PA2<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 2 }
}
impl ADC_Pin<ADC1> for PA3<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 3 }
}
impl ADC_Pin<ADC1> for PA4<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 4 }
}
impl ADC_Pin<ADC1> for PA5<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 5 }
}
impl ADC_Pin<ADC1> for PA6<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 6 }
}
impl ADC_Pin<ADC1> for PA7<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 7 }
}
impl ADC_Pin<ADC1> for PB0<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 8 }
}
impl ADC_Pin<ADC1> for PB1<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 9 }
}
impl ADC_Pin<ADC1> for PC0<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 10 }
}
impl ADC_Pin<ADC1> for PC1<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 11 }
}
impl ADC_Pin<ADC1> for PC2<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 12 }
}
impl ADC_Pin<ADC1> for PC3<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 13 }
}
impl ADC_Pin<ADC1> for PC4<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 14 }
}
impl ADC_Pin<ADC1> for PC5<Analog<AnalogIn>> {
    fn get_channel(&self) -> u8 { 15 }
}

pub struct Adc<ADCX, PIN> {
    adcx: ADCX,
    pin: PIN,
    channel: u8
}

pub trait AdcConversion {
    fn read(&self) -> u16;
}

impl<PIN> Adc<ADC1, PIN> {
    pub fn Adc1(adcx: ADC1, c_adc: &ADC_COMMON, pin: PIN, sample_time: ADC_SampleTime, rcc: &RCC) -> Self
        where PIN: ADC_Pin<ADC1>
    {
        c_adc.ccr.reset();
        rcc.apb2enr.modify(|_, w| w.adc1en().set_bit());
        adcx.cr1.reset();
        adcx.cr2.write(|w| w.adon().set_bit());

        let channel = pin.get_channel();

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
        adcx.sqr3.modify(|r, w| unsafe {
            let sqr_mask = 0x1Fu32;
            w.bits((r.bits() & !sqr_mask) & channel as u32)
        });

        Adc { adcx, pin, channel }
    }
}

impl<PIN> AdcConversion for Adc<ADC1, PIN> {
    fn read(&self) -> u16 {
        let adc = &self.adcx;
        adc.cr2.modify(|_, w| w.swstart().set_bit());
        while adc.sr.read().eoc().bit_is_clear() {};
        adc.dr.read().data().bits()
    }
}
