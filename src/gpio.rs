use stm32f40x::{GPIOA, GPIOB, GPIOC, ADC_COMMON, ADC1, RCC};

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

pub enum GPIO_AF {
    // 0x01
    AF1_TIM1,
    AF1_TIM2,
    // 0x02
    AF2_TIM3,
    AF2_TIM4,
    AF2_TIM5,
    //0x03
    AF3_TIM9,
    AF3_TIM10,
    AF3_TIM11,
    // 0x04
    AF4_I2C1,
    AF4_I2C2,
    AF4_I2C3,
    // 0x05
    AF5_SPI1,
    AF5_SPI2,
    AF5_SPI3,
    AF5_SPI4,
    // 0x06
    AF6_SPI2,
    AF6_SPI3,
    AF6_SPI4,
    AF6_SPI5,
    // 0x07
    AF7_SPI3,
    AF7_USART1,
    AF7_USART2,
    // 0x08
    AF8_USART6,
    // 0x09
    AF9_TIM14,
    AF9_I2C2,
    AF9_I2C3
}

impl GPIO_AF {
    fn af_to_val(&self) -> u8 {
        match *self {
            GPIO_AF::AF1_TIM1 => 0x01,
            GPIO_AF::AF1_TIM2 => 0x01,
            GPIO_AF::AF2_TIM3 => 0x02,
            GPIO_AF::AF2_TIM4 => 0x02,
            GPIO_AF::AF2_TIM5 => 0x02,
            GPIO_AF::AF3_TIM9 => 0x03,
            GPIO_AF::AF3_TIM10 => 0x03,
            GPIO_AF::AF3_TIM11 => 0x03,
            GPIO_AF::AF4_I2C1 => 0x04,
            GPIO_AF::AF4_I2C2 => 0x04,
            GPIO_AF::AF4_I2C3 => 0x04,
            GPIO_AF::AF5_SPI1 => 0x05,
            GPIO_AF::AF5_SPI2 => 0x05,
            GPIO_AF::AF5_SPI3 => 0x05,
            GPIO_AF::AF5_SPI4 => 0x05,
            GPIO_AF::AF6_SPI2 => 0x06,
            GPIO_AF::AF6_SPI3 => 0x06,
            GPIO_AF::AF6_SPI4 => 0x06,
            GPIO_AF::AF6_SPI5 => 0x06,
            GPIO_AF::AF7_SPI3 => 0x07,
            GPIO_AF::AF7_USART1 => 0x07,
            GPIO_AF::AF7_USART2 => 0x07,
            GPIO_AF::AF8_USART6 => 0x08,
            GPIO_AF::AF9_TIM14 => 0x09,
            GPIO_AF::AF9_I2C2 => 0x09,
            GPIO_AF::AF9_I2C3 => 0x09
        }
    }
}

macro_rules! setup_pin {
    ($pin_num:ident, $physical_pin:expr, $GPIOx:ty, $rcc_enable:ident,
     $moder:ident, $pupdr:ident, $ospeedr:ident, $otyper:ident, $bsx:ident, $brx:ident, $afr:ident, $afr_num:ident) => {
        pub struct $pin_num;

        impl $pin_num {
        // let clear_register_value = !((0xF as u32) << (($physical_pin as u32) & (0x07 as u32)) * 4);
            pub fn af_init(&self, gpio_af: GPIO_AF, rcc: &RCC, gpiox: &$GPIOx) {
                self.init(GPIO_Mode::AF, GPIO_PuPd::NOPULL, rcc, gpiox);
                let af_val = gpio_af.af_to_val();
                unsafe { gpiox.$afr.write(|w| w.$afr_num().bits(af_val as u8)); }
            }
            pub fn init(&self, pin_mode: GPIO_Mode, pin_pupd: GPIO_PuPd, rcc: &RCC, gpiox: &$GPIOx) {
                rcc.ahb1enr.modify(|_, w| w.$rcc_enable().set_bit());
                // NOTE: Not actually unsafe. Writes are atomic.
                unsafe {
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
                }
                gpiox.otyper.write(|w| w.$otyper().bit(false));
            }

            pub fn read(&self, gpiox: &$GPIOx) -> bool {
                gpiox.idr.read().bits() & (0x01 << $physical_pin) != 0
            }

            pub fn set_high(&self, gpiox: &$GPIOx) {
                gpiox.bsrr.write(|w| w.$bsx().bit(true));
            }

            pub fn set_low(&self, gpiox: &$GPIOx) {
                gpiox.bsrr.write(|w| w.$brx().bit(true));
            }
        }
    }
}

// Declare all pins used here!
setup_pin!(PA1, 1, GPIOA, gpioaen, moder1, pupdr1, ospeedr1, ot1, bs1, br1, afrl, afrl1);
setup_pin!(PA2, 2, GPIOA, gpioaen, moder2, pupdr2, ospeedr2, ot2, bs2, br2, afrl, afrl2);
setup_pin!(PA5, 5, GPIOA, gpioaen, moder5, pupdr5, ospeedr5, ot5, bs5, br5, afrl, afrl5);
setup_pin!(PA6, 6, GPIOA, gpioaen, moder6, pupdr6, ospeedr6, ot6, bs6, br6, afrl, afrl6);
setup_pin!(PA7, 7, GPIOA, gpioaen, moder7, pupdr7, ospeedr7, ot7, bs7, br7, afrl, afrl7);
setup_pin!(PB10, 10, GPIOB, gpioben, moder10, pupdr10, ospeedr10, ot10, bs10, br10, afrh, afrh10);
setup_pin!(PB14, 14, GPIOB, gpioben, moder14, pupdr14, ospeedr14, ot14, bs14, br14, afrh, afrh14);
setup_pin!(PC7, 7, GPIOC, gpiocen, moder7, pupdr7, ospeedr7, ot7, bs7, br7, afrl, afrl7);

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
pub fn initialize_adcs(c_adc: &ADC_COMMON, adc1: &ADC1) {
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
        // TODO: Figure out why mult() is missing
        // c_adc.ccr.write(|w| w.mult().bits(adc_mode));
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
