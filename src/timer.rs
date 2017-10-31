#![allow(dead_code)]
use core;
use core::result::Result::Err;
use stm32f40x::{tim1, RCC};

pub struct TimerError;
pub type TimerResult<T> = core::result::Result<T, TimerError>;

const ABP1_CLOCK_SPEED: u32 = 42; // 42 MHz


// TODO: Only support timers 2-4 atm
pub enum TimerChannel {
    TIM2,
    TIM3,
    TIM4
}

struct Timer<'a>(&'a tim1::RegisterBlock);

impl<'a> Timer<'a> {
    pub fn init(&self, tc: &TimerChannel, rcc: &RCC, frequency: u32) {
        let timx = self.0;
        let ticks_per_timer = ABP1_CLOCK_SPEED / frequency;
        let prescaler_value = (ticks_per_timer - 1) >> 16;
        let autoreload_value = ticks_per_timer / (prescaler_value - 1);
        match *tc {
            TimerChannel::TIM2 => {
                rcc.apb1enr.write(|w| w.tim2en().set_bit());
            },
            TimerChannel::TIM3 => {
                rcc.apb1enr.write(|w| w.tim3en().set_bit());
            },
            TimerChannel::TIM4 => {
                rcc.apb1enr.write(|w| w.tim4en().set_bit());
            }
        };
        unsafe {
            timx.psc.write(|w| w.psc().bits(prescaler_value as u16));
            timx.arr.write(|w| w.arr().bits(autoreload_value as u16));
        }
        timx.dier.write(|w| w.uie().set_bit());
        timx.cr1.write(|w| w.opm().clear_bit()); // Set to continuous mode (is this necessary?)
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
