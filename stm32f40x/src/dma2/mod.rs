#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - low interrupt status register"]
    pub lisr: LISR,
    #[doc = "0x04 - high interrupt status register"]
    pub hisr: HISR,
    #[doc = "0x08 - low interrupt flag clear register"]
    pub lifcr: LIFCR,
    #[doc = "0x0c - high interrupt flag clear register"]
    pub hifcr: HIFCR,
    #[doc = "0x10 - stream x configuration register"]
    pub s0cr: S0CR,
    #[doc = "0x14 - stream x number of data register"]
    pub s0ndtr: S0NDTR,
    #[doc = "0x18 - stream x peripheral address register"]
    pub s0par: S0PAR,
    #[doc = "0x1c - stream x memory 0 address register"]
    pub s0m0ar: S0M0AR,
    #[doc = "0x20 - stream x memory 1 address register"]
    pub s0m1ar: S0M1AR,
    #[doc = "0x24 - stream x FIFO control register"]
    pub s0fcr: S0FCR,
    #[doc = "0x28 - stream x configuration register"]
    pub s1cr: S1CR,
    #[doc = "0x2c - stream x number of data register"]
    pub s1ndtr: S1NDTR,
    #[doc = "0x30 - stream x peripheral address register"]
    pub s1par: S1PAR,
    #[doc = "0x34 - stream x memory 0 address register"]
    pub s1m0ar: S1M0AR,
    #[doc = "0x38 - stream x memory 1 address register"]
    pub s1m1ar: S1M1AR,
    #[doc = "0x3c - stream x FIFO control register"]
    pub s1fcr: S1FCR,
    #[doc = "0x40 - stream x configuration register"]
    pub s2cr: S2CR,
    #[doc = "0x44 - stream x number of data register"]
    pub s2ndtr: S2NDTR,
    #[doc = "0x48 - stream x peripheral address register"]
    pub s2par: S2PAR,
    #[doc = "0x4c - stream x memory 0 address register"]
    pub s2m0ar: S2M0AR,
    #[doc = "0x50 - stream x memory 1 address register"]
    pub s2m1ar: S2M1AR,
    #[doc = "0x54 - stream x FIFO control register"]
    pub s2fcr: S2FCR,
    #[doc = "0x58 - stream x configuration register"]
    pub s3cr: S3CR,
    #[doc = "0x5c - stream x number of data register"]
    pub s3ndtr: S3NDTR,
    #[doc = "0x60 - stream x peripheral address register"]
    pub s3par: S3PAR,
    #[doc = "0x64 - stream x memory 0 address register"]
    pub s3m0ar: S3M0AR,
    #[doc = "0x68 - stream x memory 1 address register"]
    pub s3m1ar: S3M1AR,
    #[doc = "0x6c - stream x FIFO control register"]
    pub s3fcr: S3FCR,
    #[doc = "0x70 - stream x configuration register"]
    pub s4cr: S4CR,
    #[doc = "0x74 - stream x number of data register"]
    pub s4ndtr: S4NDTR,
    #[doc = "0x78 - stream x peripheral address register"]
    pub s4par: S4PAR,
    #[doc = "0x7c - stream x memory 0 address register"]
    pub s4m0ar: S4M0AR,
    #[doc = "0x80 - stream x memory 1 address register"]
    pub s4m1ar: S4M1AR,
    #[doc = "0x84 - stream x FIFO control register"]
    pub s4fcr: S4FCR,
    #[doc = "0x88 - stream x configuration register"]
    pub s5cr: S5CR,
    #[doc = "0x8c - stream x number of data register"]
    pub s5ndtr: S5NDTR,
    #[doc = "0x90 - stream x peripheral address register"]
    pub s5par: S5PAR,
    #[doc = "0x94 - stream x memory 0 address register"]
    pub s5m0ar: S5M0AR,
    #[doc = "0x98 - stream x memory 1 address register"]
    pub s5m1ar: S5M1AR,
    #[doc = "0x9c - stream x FIFO control register"]
    pub s5fcr: S5FCR,
    #[doc = "0xa0 - stream x configuration register"]
    pub s6cr: S6CR,
    #[doc = "0xa4 - stream x number of data register"]
    pub s6ndtr: S6NDTR,
    #[doc = "0xa8 - stream x peripheral address register"]
    pub s6par: S6PAR,
    #[doc = "0xac - stream x memory 0 address register"]
    pub s6m0ar: S6M0AR,
    #[doc = "0xb0 - stream x memory 1 address register"]
    pub s6m1ar: S6M1AR,
    #[doc = "0xb4 - stream x FIFO control register"]
    pub s6fcr: S6FCR,
    #[doc = "0xb8 - stream x configuration register"]
    pub s7cr: S7CR,
    #[doc = "0xbc - stream x number of data register"]
    pub s7ndtr: S7NDTR,
    #[doc = "0xc0 - stream x peripheral address register"]
    pub s7par: S7PAR,
    #[doc = "0xc4 - stream x memory 0 address register"]
    pub s7m0ar: S7M0AR,
    #[doc = "0xc8 - stream x memory 1 address register"]
    pub s7m1ar: S7M1AR,
    #[doc = "0xcc - stream x FIFO control register"]
    pub s7fcr: S7FCR,
}
#[doc = "low interrupt status register"]
pub struct LISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "low interrupt status register"]
pub mod lisr;
#[doc = "high interrupt status register"]
pub struct HISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "high interrupt status register"]
pub mod hisr;
#[doc = "low interrupt flag clear register"]
pub struct LIFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "low interrupt flag clear register"]
pub mod lifcr;
#[doc = "high interrupt flag clear register"]
pub struct HIFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "high interrupt flag clear register"]
pub mod hifcr;
#[doc = "stream x configuration register"]
pub struct S0CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s0cr;
#[doc = "stream x number of data register"]
pub struct S0NDTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s0ndtr;
#[doc = "stream x peripheral address register"]
pub struct S0PAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s0par;
#[doc = "stream x memory 0 address register"]
pub struct S0M0AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s0m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S0M1AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s0m1ar;
#[doc = "stream x FIFO control register"]
pub struct S0FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s0fcr;
#[doc = "stream x configuration register"]
pub struct S1CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s1cr;
#[doc = "stream x number of data register"]
pub struct S1NDTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s1ndtr;
#[doc = "stream x peripheral address register"]
pub struct S1PAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s1par;
#[doc = "stream x memory 0 address register"]
pub struct S1M0AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s1m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S1M1AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s1m1ar;
#[doc = "stream x FIFO control register"]
pub struct S1FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s1fcr;
#[doc = "stream x configuration register"]
pub struct S2CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s2cr;
#[doc = "stream x number of data register"]
pub struct S2NDTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s2ndtr;
#[doc = "stream x peripheral address register"]
pub struct S2PAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s2par;
#[doc = "stream x memory 0 address register"]
pub struct S2M0AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s2m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S2M1AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s2m1ar;
#[doc = "stream x FIFO control register"]
pub struct S2FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s2fcr;
#[doc = "stream x configuration register"]
pub struct S3CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s3cr;
#[doc = "stream x number of data register"]
pub struct S3NDTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s3ndtr;
#[doc = "stream x peripheral address register"]
pub struct S3PAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s3par;
#[doc = "stream x memory 0 address register"]
pub struct S3M0AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s3m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S3M1AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s3m1ar;
#[doc = "stream x FIFO control register"]
pub struct S3FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s3fcr;
#[doc = "stream x configuration register"]
pub struct S4CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s4cr;
#[doc = "stream x number of data register"]
pub struct S4NDTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s4ndtr;
#[doc = "stream x peripheral address register"]
pub struct S4PAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s4par;
#[doc = "stream x memory 0 address register"]
pub struct S4M0AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s4m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S4M1AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s4m1ar;
#[doc = "stream x FIFO control register"]
pub struct S4FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s4fcr;
#[doc = "stream x configuration register"]
pub struct S5CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s5cr;
#[doc = "stream x number of data register"]
pub struct S5NDTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s5ndtr;
#[doc = "stream x peripheral address register"]
pub struct S5PAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s5par;
#[doc = "stream x memory 0 address register"]
pub struct S5M0AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s5m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S5M1AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s5m1ar;
#[doc = "stream x FIFO control register"]
pub struct S5FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s5fcr;
#[doc = "stream x configuration register"]
pub struct S6CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s6cr;
#[doc = "stream x number of data register"]
pub struct S6NDTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s6ndtr;
#[doc = "stream x peripheral address register"]
pub struct S6PAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s6par;
#[doc = "stream x memory 0 address register"]
pub struct S6M0AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s6m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S6M1AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s6m1ar;
#[doc = "stream x FIFO control register"]
pub struct S6FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s6fcr;
#[doc = "stream x configuration register"]
pub struct S7CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x configuration register"]
pub mod s7cr;
#[doc = "stream x number of data register"]
pub struct S7NDTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x number of data register"]
pub mod s7ndtr;
#[doc = "stream x peripheral address register"]
pub struct S7PAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x peripheral address register"]
pub mod s7par;
#[doc = "stream x memory 0 address register"]
pub struct S7M0AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 0 address register"]
pub mod s7m0ar;
#[doc = "stream x memory 1 address register"]
pub struct S7M1AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x memory 1 address register"]
pub mod s7m1ar;
#[doc = "stream x FIFO control register"]
pub struct S7FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "stream x FIFO control register"]
pub mod s7fcr;
