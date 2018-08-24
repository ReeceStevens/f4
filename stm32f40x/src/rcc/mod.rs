#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - clock control register"]
    pub cr: CR,
    #[doc = "0x04 - PLL configuration register"]
    pub pllcfgr: PLLCFGR,
    #[doc = "0x08 - clock configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x0c - clock interrupt register"]
    pub cir: CIR,
    #[doc = "0x10 - AHB1 peripheral reset register"]
    pub ahb1rstr: AHB1RSTR,
    #[doc = "0x14 - AHB2 peripheral reset register"]
    pub ahb2rstr: AHB2RSTR,
    _reserved0: [u8; 8usize],
    #[doc = "0x20 - APB1 peripheral reset register"]
    pub apb1rstr: APB1RSTR,
    #[doc = "0x24 - APB2 peripheral reset register"]
    pub apb2rstr: APB2RSTR,
    _reserved1: [u8; 8usize],
    #[doc = "0x30 - AHB1 peripheral clock register"]
    pub ahb1enr: AHB1ENR,
    #[doc = "0x34 - AHB2 peripheral clock enable register"]
    pub ahb2enr: AHB2ENR,
    _reserved2: [u8; 8usize],
    #[doc = "0x40 - APB1 peripheral clock enable register"]
    pub apb1enr: APB1ENR,
    #[doc = "0x44 - APB2 peripheral clock enable register"]
    pub apb2enr: APB2ENR,
    _reserved3: [u8; 8usize],
    #[doc = "0x50 - AHB1 peripheral clock enable in low power mode register"]
    pub ahb1lpenr: AHB1LPENR,
    #[doc = "0x54 - AHB2 peripheral clock enable in low power mode register"]
    pub ahb2lpenr: AHB2LPENR,
    _reserved4: [u8; 8usize],
    #[doc = "0x60 - APB1 peripheral clock enable in low power mode register"]
    pub apb1lpenr: APB1LPENR,
    #[doc = "0x64 - APB2 peripheral clock enabled in low power mode register"]
    pub apb2lpenr: APB2LPENR,
    _reserved5: [u8; 8usize],
    #[doc = "0x70 - Backup domain control register"]
    pub bdcr: BDCR,
    #[doc = "0x74 - clock control & status register"]
    pub csr: CSR,
    _reserved6: [u8; 8usize],
    #[doc = "0x80 - spread spectrum clock generation register"]
    pub sscgr: SSCGR,
    #[doc = "0x84 - PLLI2S configuration register"]
    pub plli2scfgr: PLLI2SCFGR,
}
#[doc = "clock control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "clock control register"]
pub mod cr;
#[doc = "PLL configuration register"]
pub struct PLLCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLL configuration register"]
pub mod pllcfgr;
#[doc = "clock configuration register"]
pub struct CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "clock configuration register"]
pub mod cfgr;
#[doc = "clock interrupt register"]
pub struct CIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "clock interrupt register"]
pub mod cir;
#[doc = "AHB1 peripheral reset register"]
pub struct AHB1RSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB1 peripheral reset register"]
pub mod ahb1rstr;
#[doc = "AHB2 peripheral reset register"]
pub struct AHB2RSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "APB1 peripheral reset register"]
pub struct APB1RSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 peripheral reset register"]
pub mod apb1rstr;
#[doc = "APB2 peripheral reset register"]
pub struct APB2RSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "AHB1 peripheral clock register"]
pub struct AHB1ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB1 peripheral clock register"]
pub mod ahb1enr;
#[doc = "AHB2 peripheral clock enable register"]
pub struct AHB2ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB2 peripheral clock enable register"]
pub mod ahb2enr;
#[doc = "APB1 peripheral clock enable register"]
pub struct APB1ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1enr;
#[doc = "APB2 peripheral clock enable register"]
pub struct APB2ENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "AHB1 peripheral clock enable in low power mode register"]
pub struct AHB1LPENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB1 peripheral clock enable in low power mode register"]
pub mod ahb1lpenr;
#[doc = "AHB2 peripheral clock enable in low power mode register"]
pub struct AHB2LPENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB2 peripheral clock enable in low power mode register"]
pub mod ahb2lpenr;
#[doc = "APB1 peripheral clock enable in low power mode register"]
pub struct APB1LPENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB1 peripheral clock enable in low power mode register"]
pub mod apb1lpenr;
#[doc = "APB2 peripheral clock enabled in low power mode register"]
pub struct APB2LPENR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB2 peripheral clock enabled in low power mode register"]
pub mod apb2lpenr;
#[doc = "Backup domain control register"]
pub struct BDCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Backup domain control register"]
pub mod bdcr;
#[doc = "clock control & status register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "clock control & status register"]
pub mod csr;
#[doc = "spread spectrum clock generation register"]
pub struct SSCGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "spread spectrum clock generation register"]
pub mod sscgr;
#[doc = "PLLI2S configuration register"]
pub struct PLLI2SCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PLLI2S configuration register"]
pub mod plli2scfgr;
