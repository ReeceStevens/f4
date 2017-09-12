use stm32f40x::{GPIOA, C_ADC, ADC1, RCC};
use cortex_m::interrupt;

pub enum GPIO_Mode {
    IN,
    OUT,
    AN,
    AF
}

pub enum GPIO_Speed {
    LOW_2MHZ,
    MED_25MHZ,
    HIGH_50MHZ,
    MAX_100MHZ
}

pub enum GPIO_Output {
    PP,
    OD
}

pub enum GPIO_PuPd {
    NOPULL,
    UP,
    DOWN
}

macro_rules! setup_pin {
    ($pin_num:ident, $physical_pin:expr, $GPIOx:expr, $rcc_enable:ident,
     $moder:ident, $pupdr:ident, $ospeedr:ident, $otyper:ident, $bsx:ident, $brx:ident) => {
        pub struct $pin_num;

        impl $pin_num {
            pub fn init(&self, pin_mode: GPIO_Mode, pin_pupd: GPIO_PuPd) {
                unsafe {
                    let rcc = &(*RCC.get());
                    rcc.ahb1enr.modify(|_, w| w.$rcc_enable().set_bit());
                    let gpiox = &(*$GPIOx.get());
                    match pin_mode {
                        GPIO_Mode::IN => {
                            gpiox.moder.write(|w| w.$moder().bits(0x00));
                        },
                        GPIO_Mode::OUT => {
                            gpiox.moder.write(|w| w.$moder().bits(0x01));
                        },
                        GPIO_Mode::AN => {
                            gpiox.moder.write(|w| w.$moder().bits(0x02));
                        },
                        GPIO_Mode::AF => {
                            gpiox.moder.write(|w| w.$moder().bits(0x03));
                        }
                    };
                    match pin_pupd {
                        GPIO_PuPd::NOPULL => {
                            gpiox.pupdr.write(|w| w.$pupdr().bits(0x00));
                        },
                        GPIO_PuPd::UP => {
                            gpiox.pupdr.write(|w| w.$pupdr().bits(0x01));
                        },
                        GPIO_PuPd::DOWN => {
                            gpiox.pupdr.write(|w| w.$pupdr().bits(0x02));
                        }
                    };
                    gpiox.ospeedr.write(|w| w.$ospeedr().bits(0x02));
                    gpiox.otyper.write(|w| w.$otyper().bit(false));
                }
            }

            pub fn read(&self) -> bool {
                unsafe {
                    let gpiox = &(*$GPIOx.get());
                    gpiox.idr.read().bits() & (0x01 << $physical_pin) != 0
                }
            }

            pub fn set_high(&self) {
                unsafe {
                    let gpiox = &(*$GPIOx.get());
                    gpiox.bsrr.write(|w| w.$bsx().bit(true));
                }
            }

            pub fn set_low(&self) {
                unsafe {
                    let gpiox = &(*$GPIOx.get());
                    gpiox.bsrr.write(|w| w.$brx().bit(true));
                }
            }
        }
    }
}

// Declare all pins used here!
setup_pin!(PA1, 1, GPIOA, gpioaen, moder1, pupdr1, ospeedr1, ot1, bs1, br1);
setup_pin!(PA2, 2, GPIOA, gpioaen, moder2, pupdr2, ospeedr2, ot2, bs2, br2);

enum ADC_Mode {
    ADC_Mode_Independent,
    ADC_DualMode_RegSimult_InjecSimult,
    ADC_DualMode_RegSimult_AlterTrig,
    ADC_DualMode_InjecSimult,
    ADC_DualMode_RegSimult,
    ADC_DualMode_Interl,
    ADC_DualMode_AlterTrig,
    ADC_TripleMode_RegSimult_InjecSimult,
    ADC_TripleMode_RegSimult_AlterTrig,
    ADC_TripleMode_InjecSimult,
    ADC_TripleMode_RegSimult,
    ADC_TripleMode_Interl,
    ADC_TripleMode_AlterTrig,
}

enum ADC_Prescaler {
    ADC_Prescaler_Div2,
    ADC_Prescaler_Div4,
    ADC_Prescaler_Div6,
    ADC_Prescaler_Div8
}

enum ADC_DMAMode {
    ADC_DMAAccessMode_Disabled,
    ADC_DMAAccessMode_1,
    ADC_DMAAccessMode_2,
    ADC_DMAAccessMode_3
}

enum ADC_TwoSampleDelay {
    ADC_TwoSamplingDelay_5Cycles,
    ADC_TwoSamplingDelay_6Cycles,
    ADC_TwoSamplingDelay_7Cycles,
    ADC_TwoSamplingDelay_8Cycles,
    ADC_TwoSamplingDelay_9Cycles,
    ADC_TwoSamplingDelay_10Cycles,
    ADC_TwoSamplingDelay_11Cycles,
    ADC_TwoSamplingDelay_12Cycles,
    ADC_TwoSamplingDelay_13Cycles,
    ADC_TwoSamplingDelay_14Cycles,
    ADC_TwoSamplingDelay_15Cycles,
    ADC_TwoSamplingDelay_16Cycles,
    ADC_TwoSamplingDelay_17Cycles,
    ADC_TwoSamplingDelay_18Cycles,
    ADC_TwoSamplingDelay_19Cycles,
    ADC_TwoSamplingDelay_20Cycles
}

///
/// TODO:
///  - Implement other ADC options supported
///    in stm32f4xx_adc.c
///
/// NOTE: All of these `unsafe` blocks are actually
/// safe, since they are atomic writes.
pub fn initialize_adcs(c_adc: &C_ADC, adc1: &ADC1) {
    let adc_mode = ADC_Mode::ADC_Mode_Independent;
    let adc_mode = match adc_mode {
        ADC_Mode::ADC_Mode_Independent => 0x00,
        _ => 0xFF // Not implemented yet.
    };
    let adc_prescale = ADC_Prescaler::ADC_Prescaler_Div2;
    let adc_prescale = match adc_prescale {
        ADC_Prescaler::ADC_Prescaler_Div2 => 0x00,
        ADC_Prescaler::ADC_Prescaler_Div4 => 0x01,
        ADC_Prescaler::ADC_Prescaler_Div6 => 0x02,
        ADC_Prescaler::ADC_Prescaler_Div8 => 0x03
    };
    let adc_dma = ADC_DMAMode::ADC_DMAAccessMode_Disabled;
    let adc_dma = match adc_dma {
        ADC_DMAMode::ADC_DMAAccessMode_Disabled => 0x00,
        _ => 0xFF // Not implemented yet.
    };
    let adc_twosample = ADC_TwoSampleDelay::ADC_TwoSamplingDelay_5Cycles;
    let adc_twosample = match adc_twosample {
        ADC_TwoSampleDelay::ADC_TwoSamplingDelay_5Cycles => 0x00,
        _ => 0xFF // Not implemented yet.
    };
    unsafe {
        c_adc.ccr.write(|w| w.mult().bits(adc_mode));
        c_adc.ccr.write(|w| w.adcpre().bits(adc_prescale));
        c_adc.ccr.write(|w| w.dma().bits(adc_dma));
        c_adc.ccr.write(|w| w.delay().bits(adc_twosample));
    }

    // ADC1 Initialization
    unsafe {
        let rcc = &(*RCC.get());
        rcc.apb2enr.write(|w| w.adc1en().set_bit());
        adc1.cr1.write(|w| w.res().bits(0x00)); 
    }
    adc1.cr1.write(|w| w.scan().clear_bit());
    // Clear all relevant bits in cr2
    let cr2_clear_mask = 0xC0FFF7FD;
    unsafe { adc1.cr2.modify(|r, w| w.bits(r.bits() & cr2_clear_mask)); }
    let sqr1_clear_mask = 0xFF0FFFFF; 
    unsafe { adc1.sqr1.modify(|r, w| w.bits(r.bits() & sqr1_clear_mask)); }

    // Turn ADC 1 On
    adc1.cr2.write(|w| w.adon().set_bit());
}
