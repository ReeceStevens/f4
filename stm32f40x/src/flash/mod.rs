#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash access control register"]
    pub acr: ACR,
    #[doc = "0x04 - Flash key register"]
    pub keyr: KEYR,
    #[doc = "0x08 - Flash option key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x0c - Status register"]
    pub sr: SR,
    #[doc = "0x10 - Control register"]
    pub cr: CR,
    #[doc = "0x14 - Flash option control register"]
    pub optcr: OPTCR,
}
#[doc = "Flash access control register"]
pub struct ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash access control register"]
pub mod acr;
#[doc = "Flash key register"]
pub struct KEYR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "Flash option key register"]
pub struct OPTKEYR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash option key register"]
pub mod optkeyr;
#[doc = "Status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod sr;
#[doc = "Control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod cr;
#[doc = "Flash option control register"]
pub struct OPTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash option control register"]
pub mod optcr;
