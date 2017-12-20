#![allow(dead_code)]
use core;
use core::result::Result::Err;
use stm32f40x::{TIM2, RCC};

pub struct TimerError;
pub type TimerResult<T> = core::result::Result<T, TimerError>;

const ABP1_CLOCK_SPEED: u32 = 21_000_000; // 21 MHz


// TODO: Only support timers 2-4 atm
pub enum TimerChannel {
    TIM2,
    TIM3,
    TIM4
}

pub struct Timer<'a> {
    timx: &'a TIM2
}

impl<'a> Timer<'a> {
    pub fn init(&self, tc: TimerChannel, rcc: &RCC, frequency: u32) {
        let timx = self.timx;
        let ticks_per_timer: u32 = ABP1_CLOCK_SPEED / frequency;
        let prescaler_value = (ticks_per_timer - 1) >> 16;
        let autoreload_value = ticks_per_timer / (prescaler_value + 1);
        match tc {
            TimerChannel::TIM2 => {
                rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());
            },
            TimerChannel::TIM3 => {
                rcc.apb1enr.modify(|_, w| w.tim3en().set_bit());
            },
            TimerChannel::TIM4 => {
                rcc.apb1enr.modify(|_, w| w.tim4en().set_bit());
            }
        };
        unsafe {
            timx.psc.modify(|_, w| w.psc().bits(prescaler_value as u16));
            timx.arr.modify(|_, w| w.arr_l().bits(autoreload_value as u16));
            timx.arr.modify(|_, w| w.arr_h().bits(0x0000 as u16));
        }
        timx.dier.modify(|_, w| w.uie().set_bit());
        timx.cr1.modify(|_, w| w.opm().clear_bit());
    }

    pub fn new(timx: &TIM2) -> Timer {
        Timer { timx: &timx }
    }

    /// Clears the update event flag
    ///
    /// Returns `Err` if no update event has occurred
    pub fn clear_update_flag(&self) -> TimerResult<()> {

        if self.timx.sr.read().uif().bit_is_clear() {
            Err(TimerError {})
        } else {
            self.timx.sr.modify(|_, w| w.uif().clear_bit());
            Ok(())
        }
    }

    /// Resumes the timer count
    pub fn resume(&self) {
        self.timx.cr1.modify(|_, w| w.cen().set_bit());
    }

    /// Pauses the timer
    pub fn pause(&self) {
        self.timx.cr1.modify(|_, w| w.cen().clear_bit());
    }
}
