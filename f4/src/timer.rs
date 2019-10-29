#![allow(dead_code)]
use core;
use core::result::Result::Err;
use stm32f40x::{TIM2, TIM3, TIM4, TIM5, TIM9, TIM10, TIM11, RCC};

use rcc::{get_pclk1, get_pclk2};

pub struct TimerError;
pub type TimerResult<T> = core::result::Result<T, TimerError>;

const ABP1_CLOCK_SPEED: u32 = 21_000_000; // 21 MHz


// // TODO: Only support timers 2-4 atm
// pub enum TimerChannel {
//     TIM2,
//     TIM3,
//     TIM4
// }

macro_rules! setup_timer {
    ($timer_name:ident, $timer_type:ident, $arr:ident, $clkspeed:ident, $apb:ident, $rcc_enable:ident) => {
        pub struct $timer_name<'a>(pub &'a $timer_type);

        impl<'a> $timer_name<'a> {
            pub fn init(&self, rcc: &RCC, frequency: u32) {
                let timx = self.0;
                let no_apb_prescale = rcc.cfgr.read().ppre1().bits() < 4;
                // A key hidden quote from the reference manual:
                //     The timer clock frequencies for STM32F411xC/E are automatically set by hardware. There
                //     are two cases:
                //     1. If the APB prescaler is 1, the timer clock frequencies are set to the same frequency as
                //     that of the APB domain to which the timers are connected.
                //     2. Otherwise, they are set to twice (×2) the frequency of the APB domain to which the
                //     timers are connected.
                let clock_speed: u32 = if no_apb_prescale {
                    $clkspeed(rcc)
                } else {
                    $clkspeed(rcc) * 2
                };
                let ticks_per_timer: u32 = clock_speed / frequency;
                let prescaler_value = (ticks_per_timer - 1) >> 16;
                let autoreload_value = ticks_per_timer / (prescaler_value + 1);
                rcc.$apb.modify(|_, w| w.$rcc_enable().set_bit());
                unsafe {
                    timx.psc.modify(|_, w| w.psc().bits(prescaler_value as u16));
                    timx.arr.modify(|_, w| w.$arr().bits(autoreload_value as u16));
                    timx.arr.modify(|_, w| w.arr_h().bits(0x0000 as u16));
                }
                timx.dier.modify(|_, w| w.uie().set_bit());
                timx.cr1.modify(|r, w| unsafe { // TIM10 and 11 don't have generated opm fields for some reason
                    w.bits(r.bits() & !0x03)
                });
            }

            /// Clears the update event flag
            ///
            /// Returns `Err` if no update event has occurred
            pub fn clear_update_flag(&self) -> TimerResult<()> {
                let timx = self.0;
                if timx.sr.read().uif().bit_is_clear() {
                    Err(TimerError {})
                } else {
                    timx.sr.modify(|_, w| w.uif().clear_bit());
                    Ok(())
                }
            }

            /// Resumes the timer count
            pub fn resume(&self) {
                self.0.cr1.modify(|_, w| w.cen().set_bit());
            }

            /// Pauses the timer
            pub fn pause(&self) {
                self.0.cr1.modify(|_, w| w.cen().clear_bit());
            }
        }

    }
}

setup_timer!(Timer2, TIM2, arr_l, get_pclk1, apb1enr, tim2en);
setup_timer!(Timer3, TIM3, arr_l, get_pclk1, apb1enr, tim3en);
setup_timer!(Timer4, TIM4, arr_l, get_pclk1, apb1enr, tim4en);
setup_timer!(Timer5, TIM5, arr_l, get_pclk1, apb1enr, tim5en);

// setup_timer!(Timer9, TIM9, arr, get_pclk2, apb2enr, tim9en);
// setup_timer!(Timer10, TIM10, arr, get_pclk2, apb2enr, tim10en);
// setup_timer!(Timer11, TIM11, arr, get_pclk2, apb2enr, tim11en);
