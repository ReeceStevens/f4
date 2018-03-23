#![allow(dead_code)]
#![allow(non_camel_case_types)]

use core::marker::PhantomData;
use stm32f40x::RCC;

pub struct Output<MODE>(PhantomData<MODE>);

pub struct PushPull;
pub struct OpenDrain;

pub struct Input<MODE>(PhantomData<MODE>);

pub struct Float;
pub struct Up;
pub struct Down;

pub struct Analog<MODE>(PhantomData<MODE>);

pub struct AnalogIn;
pub struct AnalogOut;

pub struct AF1;
pub struct AF2;
pub struct AF3;
pub struct AF4;
pub struct AF5;
pub struct AF6;
pub struct AF7;
pub struct AF8;
pub struct AF9;

#[derive(Copy,Clone)]
pub enum Mode {
    IN,
    OUT,
    AF,
    AN
}

#[derive(Copy,Clone)]
pub enum PuPd {
    Float,
    Up,
    Down
}

pub trait Splittable {
    type Parts;

    fn split(self, rcc: &RCC) -> Self::Parts;
}

macro_rules! gpio {
    ($GPIO_BUS:ident, $gpio_name: ident, $gpio_mod:ident, $gpio_en:ident, $gpio_rst:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $StartMode:ty, $AFR:ident, $moder:ident, $pupdr:ident, $ospeedr:ident, $otyper:ident, $afr:ident, $afr_num:ident),)+
    ]) => {
        pub mod $gpio_name {
            use super::*;
            use stm32f40x::{$GPIO_BUS, $gpio_mod};
            pub struct Parts {
                pub moder: MODER,
                pub pupdr: PUPDR,
                pub otyper: OTYPER,
                pub afrl: AFRL,
                pub afrh: AFRH,
                $(
                    pub $pxi: $PXi<$StartMode>,
                )+
            }

            impl Splittable for $GPIO_BUS {
                type Parts = Parts;

                fn split(self, rcc: &RCC) -> Parts {
                    rcc.ahb1enr.modify(|_, w| w.$gpio_en().set_bit());
                    rcc.ahb1rstr.modify(|_, w| w.$gpio_rst().set_bit());
                    rcc.ahb1rstr.modify(|_, w| w.$gpio_rst().clear_bit());
                    Parts {
                        moder: MODER(),
                        pupdr: PUPDR(),
                        otyper: OTYPER(),
                        afrl: AFRL(),
                        afrh: AFRH(),
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            // References to GPIO Registers
            pub struct MODER();
            impl MODER {
                fn moder(&self) -> &$gpio_mod::MODER {
                    unsafe { &(*$GPIO_BUS::ptr()).moder }
                }
            }

            pub struct PUPDR();
            impl PUPDR {
                fn pupdr(&self) -> &$gpio_mod::PUPDR {
                    unsafe { &(*$GPIO_BUS::ptr()).pupdr }
                }
            }

            pub struct OTYPER();
            impl OTYPER {
                fn otyper(&self) -> &$gpio_mod::OTYPER {
                    unsafe { &(*$GPIO_BUS::ptr()).otyper }
                }
            }

            pub struct AFRL();
            impl AFRL {
                fn afrl(&self) -> &$gpio_mod::AFRL {
                    unsafe { &(*$GPIO_BUS::ptr()).afrl }
                }
            }

            pub struct AFRH();
            impl AFRH {
                fn afrh(&self) -> &$gpio_mod::AFRH {
                    unsafe { &(*$GPIO_BUS::ptr()).afrh }
                }
            }

            // Generic GPIO pin for this bus
            pub struct $PXx<MODE> {
                pub i: u8,
                _mode: PhantomData<MODE>
            }

            // These unsafe functions are a single atomic write operation.
            impl<MODE> $PXx<Output<MODE>> {
                pub fn set_high(&self) {
                    unsafe { &(*$GPIO_BUS::ptr()).bsrr.write(|w| w.bits(1 << self.i)); }
                }

                pub fn set_low(&self) {
                    unsafe { &(*$GPIO_BUS::ptr()).bsrr.write(|w| w.bits(1 << (self.i + 16))); }
                }
            }

            impl<MODE> $PXx<Input<MODE>> {
                pub fn read(&self) -> bool {
                    unsafe { &(*$GPIO_BUS::ptr()).idr.read().bits() & (1 << self.i) != 0 }
                }
            }

            // Specific GPIO pin configurations
            $(
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>
                }

                impl<MODE> $PXi<Output<MODE>> {
                    pub fn set_high(&self) {
                        unsafe { &(*$GPIO_BUS::ptr()).bsrr.write(|w| w.bits(1 << $i)); }
                    }

                    pub fn set_low(&self) {
                        unsafe { &(*$GPIO_BUS::ptr()).bsrr.write(|w| w.bits(1 << ($i + 16))); }
                    }

                    pub fn into_generic_pin(self) -> $PXx<Output<MODE>> {
                        $PXx { i: $i, _mode: PhantomData }
                    }
                }

                impl<MODE> $PXi<Input<MODE>> {
                    pub fn read(&self) -> bool {
                        unsafe { &(*$GPIO_BUS::ptr()).idr.read().bits() & (1 << $i) != 0 }
                    }
                }

                impl<MODE> $PXi<MODE> {
                    pub fn into_af1(self, moder: &mut MODER, afr: &mut $AFR) -> $PXi<AF1> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::AF as u8) });
                        afr.$afr().modify(|_, w| unsafe { w.$afr_num().bits(1) });
                        $PXi::<AF1> { _mode: PhantomData }
                    }
                    pub fn into_af2(self, moder: &mut MODER, afr: &mut $AFR) -> $PXi<AF2> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::AF as u8) });
                        afr.$afr().modify(|_, w| unsafe { w.$afr_num().bits(2) });
                        $PXi::<AF2> { _mode: PhantomData }
                    }
                    pub fn into_af3(self, moder: &mut MODER, afr: &mut $AFR) -> $PXi<AF3> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::AF as u8) });
                        afr.$afr().modify(|_, w| unsafe { w.$afr_num().bits(3) });
                        $PXi::<AF3> { _mode: PhantomData }
                    }
                    pub fn into_af4(self, moder: &mut MODER, afr: &mut $AFR) -> $PXi<AF4> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::AF as u8) });
                        afr.$afr().modify(|_, w| unsafe { w.$afr_num().bits(4) });
                        $PXi::<AF4> { _mode: PhantomData }
                    }
                    pub fn into_af5(self, moder: &mut MODER, afr: &mut $AFR) -> $PXi<AF5> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::AF as u8) });
                        afr.$afr().modify(|_, w| unsafe { w.$afr_num().bits(5) });
                        $PXi::<AF5> { _mode: PhantomData }
                    }
                    pub fn into_af6(self, moder: &mut MODER, afr: &mut $AFR) -> $PXi<AF6> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::AF as u8) });
                        afr.$afr().modify(|_, w| unsafe { w.$afr_num().bits(6) });
                        $PXi::<AF6> { _mode: PhantomData }
                    }
                    pub fn into_af7(self, moder: &mut MODER, afr: &mut $AFR) -> $PXi<AF7> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::AF as u8) });
                        afr.$afr().modify(|_, w| unsafe { w.$afr_num().bits(7) });
                        $PXi::<AF7> { _mode: PhantomData }
                    }
                    pub fn into_af8(self, moder: &mut MODER, afr: &mut $AFR) -> $PXi<AF8> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::AF as u8) });
                        afr.$afr().modify(|_, w| unsafe { w.$afr_num().bits(8) });
                        $PXi::<AF8> { _mode: PhantomData }
                    }
                    pub fn into_af9(self, moder: &mut MODER, afr: &mut $AFR) -> $PXi<AF9> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::AF as u8) });
                        afr.$afr().modify(|_, w| unsafe { w.$afr_num().bits(9) });
                        $PXi::<AF9> { _mode: PhantomData }
                    }

                    pub fn into_pushpull_output(self, moder: &mut MODER, otyper: &mut OTYPER) -> $PXi<Output<PushPull>> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::OUT as u8) });
                        otyper.otyper().modify(|_, w| w.$otyper().bit(false));
                        $PXi::<Output<PushPull>> { _mode: PhantomData }
                    }

                    pub fn into_pulldown_input(self, moder: &mut MODER, pupdr: &mut PUPDR) -> $PXi<Input<Down>> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::IN as u8) });
                        pupdr.pupdr().modify(|_, w| unsafe { w.$pupdr().bits(PuPd::Down as u8) });
                        $PXi::<Input<Down>> { _mode: PhantomData }
                    }
                    pub fn into_pullup_input(self, moder: &mut MODER, pupdr: &mut PUPDR) -> $PXi<Input<Up>> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::IN as u8) });
                        pupdr.pupdr().modify(|_, w| unsafe { w.$pupdr().bits(PuPd::Up as u8) });
                        $PXi::<Input<Up>> { _mode: PhantomData }
                    }
                    pub fn into_floating_input(self, moder: &mut MODER, pupdr: &mut PUPDR) -> $PXi<Input<Float>> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::IN as u8) });
                        pupdr.pupdr().modify(|_, w| unsafe { w.$pupdr().bits(PuPd::Float as u8) });
                        $PXi::<Input<Float>> { _mode: PhantomData }
                    }

                    pub fn into_adc(self, moder: &mut MODER, pupdr: &mut PUPDR) -> $PXi<Analog<AnalogIn>> {
                        moder.moder().modify(|_, w| unsafe { w.$moder().bits(Mode::AN as u8) });
                        pupdr.pupdr().modify(|_, w| unsafe { w.$pupdr().bits(PuPd::Float as u8) });
                        $PXi::<Analog<AnalogIn>> { _mode: PhantomData }
                    }

                }

            )+

        }
    }
}

gpio!(GPIOA, gpioa, gpioa, gpioaen, gpioarst, PAx, [
    PA0: (pa0, 0, Input<Float>, AFRL, moder0, pupdr0, ospeedr0, ot0, afrl, afrl0),
    PA1: (pa1, 1, Input<Float>, AFRL, moder1, pupdr1, ospeedr1, ot1, afrl, afrl1),
    PA2: (pa2, 2, Input<Float>, AFRL, moder2, pupdr2, ospeedr2, ot2, afrl, afrl2),
    PA3: (pa3, 3, Input<Float>, AFRL, moder3, pupdr3, ospeedr3, ot3, afrl, afrl3),
    PA4: (pa4, 4, Input<Float>, AFRL, moder4, pupdr4, ospeedr4, ot4, afrl, afrl4),
    PA5: (pa5, 5, Input<Float>, AFRL, moder5, pupdr5, ospeedr5, ot5, afrl, afrl5),
    PA6: (pa6, 6, Input<Float>, AFRL, moder6, pupdr6, ospeedr6, ot6, afrl, afrl6),
    PA7: (pa7, 7, Input<Float>, AFRL, moder7, pupdr7, ospeedr7, ot7, afrl, afrl7),
    PA8: (pa8, 8, Input<Float>, AFRH, moder8, pupdr8, ospeedr8, ot8, afrh, afrh8),
    PA9: (pa9, 9, Input<Float>, AFRH, moder9, pupdr9, ospeedr9, ot9, afrh, afrh9), // USB OTG
    PA10: (pa10, 10, Input<Float>, AFRH, moder10, pupdr10, ospeedr10, ot10, afrh, afrh10), // USB OTG
    PA11: (pa11, 11, Input<Float>, AFRH, moder11, pupdr11, ospeedr11, ot11, afrh, afrh11), // USB OTG
    PA12: (pa12, 12, Input<Float>, AFRH, moder12, pupdr12, ospeedr12, ot12, afrh, afrh12), // USB OTG
    PA13: (pa13, 13, Input<Float>, AFRH, moder13, pupdr13, ospeedr13, ot13, afrh, afrh13), //SWD
    PA14: (pa14, 14, Input<Float>, AFRH, moder14, pupdr14, ospeedr14, ot14, afrh, afrh14), //SWD
    PA15: (pa15, 15, Input<Float>, AFRH, moder15, pupdr15, ospeedr15, ot15, afrh, afrh15),
]);

gpio!(GPIOB, gpiob, gpiob, gpioben, gpiobrst, PBx, [
    PB0: (pb0, 0, Input<Float>, AFRL, moder0, pupdr0, ospeedr0, ot0, afrl, afrl0),
    PB1: (pb1, 1, Input<Float>, AFRL, moder1, pupdr1, ospeedr1, ot1, afrl, afrl1),
    PB2: (pb2, 2, Input<Float>, AFRL, moder2, pupdr2, ospeedr2, ot2, afrl, afrl2),
    PB3: (pb3, 3, Input<Float>, AFRL, moder3, pupdr3, ospeedr3, ot3, afrl, afrl3), // SWD
    PB4: (pb4, 4, Input<Float>, AFRL, moder4, pupdr4, ospeedr4, ot4, afrl, afrl4),
    PB5: (pb5, 5, Input<Float>, AFRL, moder5, pupdr5, ospeedr5, ot5, afrl, afrl5),
    PB6: (pb6, 6, Input<Float>, AFRL, moder6, pupdr6, ospeedr6, ot6, afrl, afrl6),
    PB7: (pb7, 7, Input<Float>, AFRL, moder7, pupdr7, ospeedr7, ot7, afrl, afrl7),
    PB8: (pb8, 8, Input<Float>, AFRH, moder8, pupdr8, ospeedr8, ot8, afrh, afrh8),
    PB9: (pb9, 9, Input<Float>, AFRH, moder9, pupdr9, ospeedr9, ot9, afrh, afrh9),
    PB10: (pb10, 10, Input<Float>, AFRH, moder10, pupdr10, ospeedr10, ot10, afrh, afrh10),
    PB11: (pb11, 11, Input<Float>, AFRH, moder11, pupdr11, ospeedr11, ot11, afrh, afrh11),
    PB12: (pb12, 12, Input<Float>, AFRH, moder12, pupdr12, ospeedr12, ot12, afrh, afrh12),
    PB13: (pb13, 13, Input<Float>, AFRH, moder13, pupdr13, ospeedr13, ot13, afrh, afrh13),
    PB14: (pb14, 14, Input<Float>, AFRH, moder14, pupdr14, ospeedr14, ot14, afrh, afrh14),
    PB15: (pb15, 15, Input<Float>, AFRH, moder15, pupdr15, ospeedr15, ot15, afrh, afrh15),
]);

gpio!(GPIOC, gpioc, gpioh, gpiocen, gpiocrst, PCx, [
    PC0: (pc0, 0, Input<Float>, AFRL, moder0, pupdr0, ospeedr0, ot0, afrl, afrl0), // USB OTG
    PC1: (pc1, 1, Input<Float>, AFRL, moder1, pupdr1, ospeedr1, ot1, afrl, afrl1),
    PC2: (pc2, 2, Input<Float>, AFRL, moder2, pupdr2, ospeedr2, ot2, afrl, afrl2),
    PC3: (pc3, 3, Input<Float>, AFRL, moder3, pupdr3, ospeedr3, ot3, afrl, afrl3),
    PC4: (pc4, 4, Input<Float>, AFRL, moder4, pupdr4, ospeedr4, ot4, afrl, afrl4),
    PC5: (pc5, 5, Input<Float>, AFRL, moder5, pupdr5, ospeedr5, ot5, afrl, afrl5),
    PC6: (pc6, 6, Input<Float>, AFRL, moder6, pupdr6, ospeedr6, ot6, afrl, afrl6),
    PC7: (pc7, 7, Input<Float>, AFRL, moder7, pupdr7, ospeedr7, ot7, afrl, afrl7),
    PC8: (pc8, 8, Input<Float>, AFRH, moder8, pupdr8, ospeedr8, ot8, afrh, afrh8),
    PC9: (pc9, 9, Input<Float>, AFRH, moder9, pupdr9, ospeedr9, ot9, afrh, afrh9),
    PC10: (pc10, 10, Input<Float>, AFRH, moder10, pupdr10, ospeedr10, ot10, afrh, afrh10),
    PC11: (pc11, 11, Input<Float>, AFRH, moder11, pupdr11, ospeedr11, ot11, afrh, afrh11),
    PC12: (pc12, 12, Input<Float>, AFRH, moder12, pupdr12, ospeedr12, ot12, afrh, afrh12),
    PC13: (pc13, 13, Input<Float>, AFRH, moder13, pupdr13, ospeedr13, ot13, afrh, afrh13),
    PC14: (pc14, 14, Input<Float>, AFRH, moder14, pupdr14, ospeedr14, ot14, afrh, afrh14),
    PC15: (pc15, 15, Input<Float>, AFRH, moder15, pupdr15, ospeedr15, ot15, afrh, afrh15),
]);

gpio!(GPIOD, gpiod, gpioh, gpioden, gpiodrst, PDx, [
    PD0: (pd0, 0, Input<Float>, AFRL, moder0, pupdr0, ospeedr0, ot0, afrl, afrl0),
    PD1: (pd1, 1, Input<Float>, AFRL, moder1, pupdr1, ospeedr1, ot1, afrl, afrl1),
    PD2: (pd2, 2, Input<Float>, AFRL, moder2, pupdr2, ospeedr2, ot2, afrl, afrl2),
    PD3: (pd3, 3, Input<Float>, AFRL, moder3, pupdr3, ospeedr3, ot3, afrl, afrl3),
    PD4: (pd4, 4, Input<Float>, AFRL, moder4, pupdr4, ospeedr4, ot4, afrl, afrl4),
    PD5: (pd5, 5, Input<Float>, AFRL, moder5, pupdr5, ospeedr5, ot5, afrl, afrl5), // USB OTG
    PD6: (pd6, 6, Input<Float>, AFRL, moder6, pupdr6, ospeedr6, ot6, afrl, afrl6),
    PD7: (pd7, 7, Input<Float>, AFRL, moder7, pupdr7, ospeedr7, ot7, afrl, afrl7),
    PD8: (pd8, 8, Input<Float>, AFRH, moder8, pupdr8, ospeedr8, ot8, afrh, afrh8),
    PD9: (pd9, 9, Input<Float>, AFRH, moder9, pupdr9, ospeedr9, ot9, afrh, afrh9),
    PD10: (pd10, 10, Input<Float>, AFRH, moder10, pupdr10, ospeedr10, ot10, afrh, afrh10),
    PD11: (pd11, 11, Input<Float>, AFRH, moder11, pupdr11, ospeedr11, ot11, afrh, afrh11),
    PD12: (pd12, 12, Input<Float>, AFRH, moder12, pupdr12, ospeedr12, ot12, afrh, afrh12),
    PD13: (pd13, 13, Input<Float>, AFRH, moder13, pupdr13, ospeedr13, ot13, afrh, afrh13),
    PD14: (pd14, 14, Input<Float>, AFRH, moder14, pupdr14, ospeedr14, ot14, afrh, afrh14),
    PD15: (pd15, 15, Input<Float>, AFRH, moder15, pupdr15, ospeedr15, ot15, afrh, afrh15),
]);
