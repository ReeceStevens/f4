#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS power and clock gating control register"]
    pub fs_pcgcctl: FS_PCGCCTL,
}
#[doc = "OTG_FS power and clock gating control register"]
pub struct FS_PCGCCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS power and clock gating control register"]
pub mod fs_pcgcctl;
