#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - time register"]
    pub tr: TR,
    #[doc = "0x04 - date register"]
    pub dr: DR,
    #[doc = "0x08 - control register"]
    pub cr: CR,
    #[doc = "0x0c - initialization and status register"]
    pub isr: ISR,
    #[doc = "0x10 - prescaler register"]
    pub prer: PRER,
    #[doc = "0x14 - wakeup timer register"]
    pub wutr: WUTR,
    #[doc = "0x18 - calibration register"]
    pub calibr: CALIBR,
    #[doc = "0x1c - alarm A register"]
    pub alrmar: ALRMAR,
    #[doc = "0x20 - alarm B register"]
    pub alrmbr: ALRMBR,
    #[doc = "0x24 - write protection register"]
    pub wpr: WPR,
    #[doc = "0x28 - sub second register"]
    pub ssr: SSR,
    #[doc = "0x2c - shift control register"]
    pub shiftr: SHIFTR,
    #[doc = "0x30 - time stamp time register"]
    pub tstr: TSTR,
    #[doc = "0x34 - time stamp date register"]
    pub tsdr: TSDR,
    #[doc = "0x38 - timestamp sub second register"]
    pub tsssr: TSSSR,
    #[doc = "0x3c - calibration register"]
    pub calr: CALR,
    #[doc = "0x40 - tamper and alternate function configuration register"]
    pub tafcr: TAFCR,
    #[doc = "0x44 - alarm A sub second register"]
    pub alrmassr: ALRMASSR,
    #[doc = "0x48 - alarm B sub second register"]
    pub alrmbssr: ALRMBSSR,
    _reserved0: [u8; 4usize],
    #[doc = "0x50 - backup register"]
    pub bkp0r: BKP0R,
    #[doc = "0x54 - backup register"]
    pub bkp1r: BKP1R,
    #[doc = "0x58 - backup register"]
    pub bkp2r: BKP2R,
    #[doc = "0x5c - backup register"]
    pub bkp3r: BKP3R,
    #[doc = "0x60 - backup register"]
    pub bkp4r: BKP4R,
    #[doc = "0x64 - backup register"]
    pub bkp5r: BKP5R,
    #[doc = "0x68 - backup register"]
    pub bkp6r: BKP6R,
    #[doc = "0x6c - backup register"]
    pub bkp7r: BKP7R,
    #[doc = "0x70 - backup register"]
    pub bkp8r: BKP8R,
    #[doc = "0x74 - backup register"]
    pub bkp9r: BKP9R,
    #[doc = "0x78 - backup register"]
    pub bkp10r: BKP10R,
    #[doc = "0x7c - backup register"]
    pub bkp11r: BKP11R,
    #[doc = "0x80 - backup register"]
    pub bkp12r: BKP12R,
    #[doc = "0x84 - backup register"]
    pub bkp13r: BKP13R,
    #[doc = "0x88 - backup register"]
    pub bkp14r: BKP14R,
    #[doc = "0x8c - backup register"]
    pub bkp15r: BKP15R,
    #[doc = "0x90 - backup register"]
    pub bkp16r: BKP16R,
    #[doc = "0x94 - backup register"]
    pub bkp17r: BKP17R,
    #[doc = "0x98 - backup register"]
    pub bkp18r: BKP18R,
    #[doc = "0x9c - backup register"]
    pub bkp19r: BKP19R,
}
#[doc = "time register"]
pub struct TR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "time register"]
pub mod tr;
#[doc = "date register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "date register"]
pub mod dr;
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "initialization and status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization and status register"]
pub mod isr;
#[doc = "prescaler register"]
pub struct PRER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "prescaler register"]
pub mod prer;
#[doc = "wakeup timer register"]
pub struct WUTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "wakeup timer register"]
pub mod wutr;
#[doc = "calibration register"]
pub struct CALIBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "calibration register"]
pub mod calibr;
#[doc = "alarm A register"]
pub struct ALRMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "alarm A register"]
pub mod alrmar;
#[doc = "alarm B register"]
pub struct ALRMBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "alarm B register"]
pub mod alrmbr;
#[doc = "write protection register"]
pub struct WPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "write protection register"]
pub mod wpr;
#[doc = "sub second register"]
pub struct SSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "sub second register"]
pub mod ssr;
#[doc = "shift control register"]
pub struct SHIFTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "shift control register"]
pub mod shiftr;
#[doc = "time stamp time register"]
pub struct TSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "time stamp time register"]
pub mod tstr;
#[doc = "time stamp date register"]
pub struct TSDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "time stamp date register"]
pub mod tsdr;
#[doc = "timestamp sub second register"]
pub struct TSSSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "timestamp sub second register"]
pub mod tsssr;
#[doc = "calibration register"]
pub struct CALR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "calibration register"]
pub mod calr;
#[doc = "tamper and alternate function configuration register"]
pub struct TAFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "tamper and alternate function configuration register"]
pub mod tafcr;
#[doc = "alarm A sub second register"]
pub struct ALRMASSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "alarm A sub second register"]
pub mod alrmassr;
#[doc = "alarm B sub second register"]
pub struct ALRMBSSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "alarm B sub second register"]
pub mod alrmbssr;
#[doc = "backup register"]
pub struct BKP0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp0r;
#[doc = "backup register"]
pub struct BKP1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp1r;
#[doc = "backup register"]
pub struct BKP2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp2r;
#[doc = "backup register"]
pub struct BKP3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp3r;
#[doc = "backup register"]
pub struct BKP4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp4r;
#[doc = "backup register"]
pub struct BKP5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp5r;
#[doc = "backup register"]
pub struct BKP6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp6r;
#[doc = "backup register"]
pub struct BKP7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp7r;
#[doc = "backup register"]
pub struct BKP8R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp8r;
#[doc = "backup register"]
pub struct BKP9R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp9r;
#[doc = "backup register"]
pub struct BKP10R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp10r;
#[doc = "backup register"]
pub struct BKP11R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp11r;
#[doc = "backup register"]
pub struct BKP12R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp12r;
#[doc = "backup register"]
pub struct BKP13R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp13r;
#[doc = "backup register"]
pub struct BKP14R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp14r;
#[doc = "backup register"]
pub struct BKP15R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp15r;
#[doc = "backup register"]
pub struct BKP16R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp16r;
#[doc = "backup register"]
pub struct BKP17R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp17r;
#[doc = "backup register"]
pub struct BKP18R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp18r;
#[doc = "backup register"]
pub struct BKP19R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp19r;
