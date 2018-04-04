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
            use stm32f40x::EXTI;
            use super::EXTITrigger;

            pub fn configure(exti: &mut EXTI, trigger: EXTITrigger) {
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
