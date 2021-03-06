#[derive(Copy,Clone)]
pub enum EXTITrigger {
    Rising,
    Falling,
    RisingFalling
}

// TODO: Starting with only supporting hardware interrupt mode
macro_rules! exti {
    ($exti_line:ident, $mr:ident, $tr:ident) => {
        pub mod $exti_line {
            use stm32f40x::{EXTI, SYSCFG};
            use super::EXTITrigger;

            pub fn configure(syscfg: &mut SYSCFG, exti: &mut EXTI, trigger: EXTITrigger) {
                // TODO: Find correct GPIO bus to select with `bits`
                syscfg.exticr4.modify( |_, w| unsafe { w.$exti_line().bits(0x00) });
                exti.imr.modify(|_, w| w.$mr().set_bit());
                match trigger {
                    EXTITrigger::Rising => {
                        exti.rtsr.modify(|_, w| w.$tr().set_bit());
                        exti.ftsr.modify(|_, w| w.$tr().clear_bit());
                    },
                    EXTITrigger::Falling => {
                        exti.rtsr.modify(|_, w| w.$tr().clear_bit());
                        exti.ftsr.modify(|_, w| w.$tr().set_bit());
                    },
                    EXTITrigger::RisingFalling => {
                        exti.rtsr.modify(|_, w| w.$tr().set_bit());
                        exti.ftsr.modify(|_, w| w.$tr().set_bit());
                    }
                };
            }
        }
    }
}

exti!(exti14, mr14, tr14);
exti!(exti15, mr15, tr15);
