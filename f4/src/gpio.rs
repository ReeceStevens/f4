#![allow(dead_code)]
#![allow(non_camel_case_types)]
/// Defines the GPIO pins and ADC structures.
///
/// Example: Toggle a GPIO pin
///
/// ```rs
/// use f4::stm32f40x::{GPIOA, RCC};
/// use f4::gpio::{PA1, GPIOConfig, GPIOMode, GPIO_PuPd};
///
/// let config = GPIOConfig::new(GPIOMode::OUT, GPIO_PuPd::UP);
/// PA1::init(&config, &RCC, &GPIOA);
/// let pin = PA1::get_reference(&GPIOA);
///
/// pin.set_high();
/// pin.set_low();
/// ```
///
/// Example: Read a GPIO pin
///
/// ```rs
/// use f4::stm32f40x::{GPIOA, RCC};
/// use f4::gpio::{PA1, GPIOConfig, GPIOMode, GPIO_PuPd};
///
/// let config = GPIOConfig::new(GPIO_Mode::IN, GPIO_PuPd::NOPULL);
/// PA1::init(&config, &RCC, &GPIOA);
/// let pin = PA1::get_reference(&GPIOA);
///
/// let status: bool = pin.read();
/// ```

use stm32f40x::{GPIOA, GPIOB, GPIOC, GPIOD,
                ADC_COMMON, ADC1, RCC};
use stm32f40x;

#[derive(Copy,Clone)]
pub enum GPIO_Mode {
    IN,
    OUT,
    AN,
    AF
}

#[derive(Copy,Clone)]
pub enum GPIO_Speed {
    LOW_2MHZ,
    MED_25MHZ,
    HIGH_50MHZ,
    MAX_100MHZ
}

#[derive(Copy,Clone)]
pub enum GPIO_OutputType {
    PP,
    OD
}

#[derive(Copy,Clone)]
pub enum GPIO_PuPd {
    NOPULL,
    UP,
    DOWN
}

#[derive(Copy,Clone)]
pub enum GPIO_AF {
    NONE,
    AF1_TIM1,
    AF1_TIM2,
    AF2_TIM3,
    AF2_TIM4,
    AF2_TIM5,
    AF3_TIM9,
    AF3_TIM10,
    AF3_TIM11,
    AF4_I2C1,
    AF4_I2C2,
    AF4_I2C3,
    AF5_SPI1,
    AF5_SPI2,
    AF5_SPI3,
    AF5_SPI4,
    AF6_SPI2,
    AF6_SPI3,
    AF6_SPI4,
    AF6_SPI5,
    AF7_SPI3,
    AF7_USART1,
    AF7_USART2,
    AF8_USART6,
    AF9_TIM14,
    AF9_I2C2,
    AF9_I2C3
}

impl GPIO_AF {
    fn af_to_val(&self) -> u8 {
        use self::GPIO_AF::*;
        match *self {
            NONE                                         => 0x00,
            AF1_TIM1  | AF1_TIM2                         => 0x01,
            AF2_TIM3  | AF2_TIM4   | AF2_TIM5            => 0x02,
            AF3_TIM9  | AF3_TIM10  | AF3_TIM11           => 0x03,
            AF4_I2C1  | AF4_I2C2   | AF4_I2C3            => 0x04,
            AF5_SPI1  | AF5_SPI2   | AF5_SPI3 | AF5_SPI4 => 0x05,
            AF6_SPI2  | AF6_SPI3   | AF6_SPI4 | AF6_SPI5 => 0x06,
            AF7_SPI3  | AF7_USART1 | AF7_USART2          => 0x07,
            AF8_USART6                                   => 0x08,
            AF9_TIM14 | AF9_I2C2   | AF9_I2C3            => 0x09,
        }
    }
}

#[derive(Copy,Clone)]
pub struct GPIOConfig {
    mode: GPIO_Mode,
    speed: GPIO_Speed,
    otype: GPIO_OutputType,
    pupd: GPIO_PuPd,
    af: GPIO_AF
}

impl GPIOConfig {
    pub fn new(m: GPIO_Mode, p: GPIO_PuPd) -> GPIOConfig {
        GPIOConfig {
            mode: m,
            speed: GPIO_Speed::HIGH_50MHZ,
            otype: GPIO_OutputType::PP,
            pupd: p,
            af: GPIO_AF::NONE,
        }
    }

    pub fn new_af(af: GPIO_AF) -> GPIOConfig {
        let mut config = GPIOConfig::new(GPIO_Mode::AF, GPIO_PuPd::NOPULL);
        config.af = af;
        config
    }
}

macro_rules! setup_pin {
    ($pin_num:ident, $physical_pin:expr, $GPIOx:ty, $gpio_mod:ident, $rcc_enable:ident, $moder:ident, $pupdr:ident, $ospeedr:ident, $otyper:ident, $bsx:ident,
     $brx:ident, $afr:ident, $afr_num:ident) => {

        pub struct $pin_num<'a> {
            bsrr: &'a stm32f40x::$gpio_mod::BSRR,
            idr: &'a stm32f40x::$gpio_mod::IDR
        }

        impl<'a> $pin_num<'a> {
            pub fn get_reference(gpiox: &$GPIOx) -> $pin_num{
                $pin_num {
                    bsrr: &gpiox.bsrr,
                    idr: &gpiox.idr
                }
            }

            pub fn init(config: GPIOConfig, rcc: &RCC, gpiox: &$GPIOx) {
                rcc.ahb1enr.modify(|_, w| w.$rcc_enable().set_bit());
                unsafe { // NOTE: Actually safe. Writes are atomic.
                    gpiox.moder.modify(|_, w| w.$moder().bits(config.mode as u8));
                    gpiox.pupdr.modify(|_, w| w.$pupdr().bits(config.pupd as u8));
                    gpiox.ospeedr.modify(|_, w| w.$ospeedr().bits(config.speed as u8));
                    match config.af {
                        GPIO_AF::NONE => {},
                        _ => {gpiox.$afr.modify(|_, w| w.$afr_num().bits(config.af.af_to_val()))}
                    }
                }
                gpiox.otyper.modify(|_, w| w.$otyper().bit(config.otype as u8 != 0x00));
            }

            pub fn read(&self) -> bool {
                self.idr.read().bits() & (0x01 << $physical_pin) != 0
            }

            pub fn set_high(&self) {
                self.bsrr.write(|w| w.$bsx().bit(true));
            }

            pub fn set_low(&self) {
                self.bsrr.write(|w| w.$brx().bit(true));
            }
        }
    }
}

// Declare all pins used here!
setup_pin!(PA1, 1, GPIOA, gpioa, gpioaen, moder1, pupdr1, ospeedr1, ot1, bs1, br1, afrl, afrl1);
setup_pin!(PA2, 2, GPIOA, gpioa, gpioaen, moder2, pupdr2, ospeedr2, ot2, bs2, br2, afrl, afrl2);
setup_pin!(PA5, 5, GPIOA, gpioa, gpioaen, moder5, pupdr5, ospeedr5, ot5, bs5, br5, afrl, afrl5);
setup_pin!(PA6, 6, GPIOA, gpioa, gpioaen, moder6, pupdr6, ospeedr6, ot6, bs6, br6, afrl, afrl6);
setup_pin!(PA7, 7, GPIOA, gpioa, gpioaen, moder7, pupdr7, ospeedr7, ot7, bs7, br7, afrl, afrl7);
setup_pin!(PA11, 11, GPIOA, gpioa, gpioaen, moder11, pupdr11, ospeedr11, ot11, bs11, br11, afrh, afrh11);
setup_pin!(PB10, 10, GPIOB, gpiob, gpioben, moder10, pupdr10, ospeedr10, ot10, bs10, br10, afrh, afrh10);
setup_pin!(PB13, 13, GPIOB, gpiob, gpioben, moder13, pupdr13, ospeedr13, ot13, bs13, br13, afrh, afrh13);
setup_pin!(PB14, 14, GPIOB, gpiob, gpioben, moder14, pupdr14, ospeedr14, ot14, bs14, br14, afrh, afrh14);
setup_pin!(PC7, 7, GPIOC, gpioh, gpiocen, moder7, pupdr7, ospeedr7, ot7, bs7, br7, afrl, afrl7);
setup_pin!(PD13, 13, GPIOD, gpioh, gpioden, moder13, pupdr13, ospeedr13, ot13, bs13, br13, afrh, afrh13);
setup_pin!(PD15, 15, GPIOD, gpioh, gpioden, moder15, pupdr15, ospeedr15, ot15, bs15, br15, afrh, afrh15);

#[derive(Copy,Clone)]
enum ADC_Mode {
    ADC_Mode_Independent                    = 0x00,
    // TODO: all other modes not supported
    ADC_DualMode_RegSimult_InjecSimult      = 0x01,
    ADC_DualMode_RegSimult_AlterTrig        = 0x02,
    ADC_DualMode_InjecSimult                = 0x05,
    ADC_DualMode_RegSimult                  = 0x06,
    ADC_DualMode_Interl                     = 0x07,
    ADC_DualMode_AlterTrig                  = 0x09,
    ADC_TripleMode_RegSimult_InjecSimult    = 0x11,
    ADC_TripleMode_RegSimult_AlterTrig      = 0x12,
    ADC_TripleMode_InjecSimult              = 0x15,
    ADC_TripleMode_RegSimult                = 0x16,
    ADC_TripleMode_Interl                   = 0x17,
    ADC_TripleMode_AlterTrig                = 0x19,
}

#[derive(Copy,Clone)]
enum ADC_Prescaler {
    ADC_Prescaler_Div2,
    ADC_Prescaler_Div4,
    ADC_Prescaler_Div6,
    ADC_Prescaler_Div8
}

#[derive(Copy,Clone)]
enum ADC_DMAMode {
    ADC_DMAAccessMode_Disabled,
    ADC_DMAAccessMode_1,
    ADC_DMAAccessMode_2,
    ADC_DMAAccessMode_3
}

#[derive(Copy,Clone)]
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

#[derive(Copy,Clone)]
struct ADCConfig {
    adc_mode: ADC_Mode,
    adc_prescaler: ADC_Prescaler,
    adc_dma: ADC_DMAMode,
    adc_twosample: ADC_TwoSampleDelay,
}

impl ADCConfig {
    fn new() -> ADCConfig {
        ADCConfig {
            adc_mode: ADC_Mode::ADC_Mode_Independent,
            adc_prescaler: ADC_Prescaler::ADC_Prescaler_Div2,
            adc_dma: ADC_DMAMode::ADC_DMAAccessMode_Disabled,
            adc_twosample: ADC_TwoSampleDelay::ADC_TwoSamplingDelay_5Cycles
        }
    }
}

///
/// TODO:
///  - Implement other ADC options supported
///    in stm32f4xx_adc.c
///
/// NOTE: All of these `unsafe` blocks are actually
/// safe, since they are atomic writes.
pub fn initialize_adcs(c_adc: &ADC_COMMON, adc1: &ADC1) {
    let ref adc_config = ADCConfig::new();
    unsafe {
        // TODO: Figure out why mult() is missing
        // c_adc.ccr.write(|w| w.mult().bits(adc_mode));
        c_adc.ccr.modify(|r, w| w.bits((r.bits() & 0x00FF) & (adc_config.adc_mode as u32)));
        c_adc.ccr.write(|w| w.adcpre().bits(adc_config.adc_prescaler as u8));
        c_adc.ccr.write(|w| w.dma().bits(adc_config.adc_dma as u8));
        c_adc.ccr.write(|w| w.delay().bits(adc_config.adc_twosample as u8));
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
