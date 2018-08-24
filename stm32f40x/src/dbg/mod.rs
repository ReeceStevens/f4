#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IDCODE"]
    pub dbgmcu_idcode: DBGMCU_IDCODE,
    #[doc = "0x04 - Control Register"]
    pub dbgmcu_cr: DBGMCU_CR,
    #[doc = "0x08 - Debug MCU APB1 Freeze registe"]
    pub dbgmcu_apb1_fz: DBGMCU_APB1_FZ,
    #[doc = "0x0c - Debug MCU APB2 Freeze registe"]
    pub dbgmcu_apb2_fz: DBGMCU_APB2_FZ,
}
#[doc = "IDCODE"]
pub struct DBGMCU_IDCODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IDCODE"]
pub mod dbgmcu_idcode;
#[doc = "Control Register"]
pub struct DBGMCU_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod dbgmcu_cr;
#[doc = "Debug MCU APB1 Freeze registe"]
pub struct DBGMCU_APB1_FZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug MCU APB1 Freeze registe"]
pub mod dbgmcu_apb1_fz;
#[doc = "Debug MCU APB2 Freeze registe"]
pub struct DBGMCU_APB2_FZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug MCU APB2 Freeze registe"]
pub mod dbgmcu_apb2_fz;
