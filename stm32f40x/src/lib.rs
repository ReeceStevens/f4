# ! [ doc = "Peripheral access API for STM32F411 microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api" ] # ! [ deny ( missing_docs ) ] # ! [ deny ( warnings ) ] # ! [ allow ( non_camel_case_types ) ] # ! [ no_std ]extern crate cortex_m ;
#[cfg(feature = "rt")]
extern crate cortex_m_rt ;
extern crate bare_metal ;
extern crate vcell ;
use core::ops::Deref;
use core::marker::PhantomData;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn PVD();
    fn TAMP_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn ADC();
    fn EXTI9_5();
    fn TIM1_BRK_TIM9();
    fn TIM1_UP_TIM10();
    fn TIM1_TRG_COM_TIM11();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn OTG_FS_WKUP();
    fn SDIO();
    fn TIM5();
    fn SPI3();
    fn OTG_FS();
    fn I2C3_EV();
    fn I2C3_ER();
    fn FPU();
    fn SPI4();
}
#[doc(hidden)]pub union Vector { _handler : unsafe extern "C" fn ( ) , _reserved : u32 }#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 85] = [
    Vector { _reserved: 0 },
    Vector { _handler: PVD },
    Vector { _handler: TAMP_STAMP },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: EXTI9_5 },
    Vector { _handler: TIM1_BRK_TIM9 },
    Vector { _handler: TIM1_UP_TIM10 },
    Vector { _handler: TIM1_TRG_COM_TIM11 },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: EXTI15_10 },
    Vector { _handler: RTC_ALARM },
    Vector { _handler: OTG_FS_WKUP },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SDIO },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: OTG_FS },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FPU },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SPI4 },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ Name : ident , $ handler : path , state : $ State : ty = $ initial_state : expr ) => { # [ allow ( unsafe_code ) ] # [ deny ( private_no_mangle_fns ) ] # [ no_mangle ] pub unsafe extern "C" fn $ Name ( ) { static mut STATE : $ State = $ initial_state ; let _ = $ crate :: Interrupt :: $ Name ; let f : fn ( & mut $ State ) = $ handler ; f ( & mut STATE ) } } ; ( $ Name : ident , $ handler : path ) => { # [ allow ( unsafe_code ) ] # [ deny ( private_no_mangle_fns ) ] # [ no_mangle ] pub unsafe extern "C" fn $ Name ( ) { let _ = $ crate :: Interrupt :: $ Name ; let f : fn ( ) = $ handler ; f ( ) } } ; }
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "1 - PVD through EXTI line detection interrupt"]
    PVD,
    #[doc = "2 - Tamper and TimeStamp interrupts through the EXTI line"]
    TAMP_STAMP,
    #[doc = "3 - RTC Wakeup interrupt through the EXTI line"]
    RTC_WKUP,
    #[doc = "4 - FLASH global interrupt"]
    FLASH,
    #[doc = "5 - RCC global interrupt"]
    RCC,
    #[doc = "6 - EXTI Line0 interrupt"]
    EXTI0,
    #[doc = "7 - EXTI Line1 interrupt"]
    EXTI1,
    #[doc = "8 - EXTI Line2 interrupt"]
    EXTI2,
    #[doc = "9 - EXTI Line3 interrupt"]
    EXTI3,
    #[doc = "10 - EXTI Line4 interrupt"]
    EXTI4,
    #[doc = "18 - ADC1 global interrupt"]
    ADC,
    #[doc = "23 - EXTI Line[9:5] interrupts"]
    EXTI9_5,
    #[doc = "24 - TIM1 Break interrupt and TIM9 global interrupt"]
    TIM1_BRK_TIM9,
    #[doc = "25 - TIM1 Update interrupt and TIM10 global interrupt"]
    TIM1_UP_TIM10,
    #[doc = "26 - TIM1 Trigger and Commutation interrupts and TIM11 global interrupt"]
    TIM1_TRG_COM_TIM11,
    #[doc = "27 - TIM1 Capture Compare interrupt"]
    TIM1_CC,
    #[doc = "28 - TIM2 global interrupt"]
    TIM2,
    #[doc = "29 - TIM3 global interrupt"]
    TIM3,
    #[doc = "30 - TIM4 global interrupt"]
    TIM4,
    #[doc = "31 - I2C1 event interrupt"]
    I2C1_EV,
    #[doc = "32 - I2C1 error interrupt"]
    I2C1_ER,
    #[doc = "33 - I2C2 event interrupt"]
    I2C2_EV,
    #[doc = "34 - I2C2 error interrupt"]
    I2C2_ER,
    #[doc = "35 - SPI1 global interrupt"]
    SPI1,
    #[doc = "36 - SPI2 global interrupt"]
    SPI2,
    #[doc = "40 - EXTI Line[15:10] interrupts"]
    EXTI15_10,
    #[doc = "41 - RTC Alarms (A and B) through EXTI line interrupt"]
    RTC_ALARM,
    #[doc = "42 - USB On-The-Go FS Wakeup through EXTI line interrupt"]
    OTG_FS_WKUP,
    #[doc = "49 - SDIO global interrupt"]
    SDIO,
    #[doc = "50 - TIM5 global interrupt"]
    TIM5,
    #[doc = "51 - SPI3 global interrupt"]
    SPI3,
    #[doc = "67 - USB On The Go FS global interrupt"]
    OTG_FS,
    #[doc = "72 - I2C3 event interrupt"]
    I2C3_EV,
    #[doc = "73 - I2C3 error interrupt"]
    I2C3_ER,
    #[doc = "81 - FPU interrupt"]
    FPU,
    #[doc = "84 - SPI4 global interrupt"]
    SPI4,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PVD => 1,
            Interrupt::TAMP_STAMP => 2,
            Interrupt::RTC_WKUP => 3,
            Interrupt::FLASH => 4,
            Interrupt::RCC => 5,
            Interrupt::EXTI0 => 6,
            Interrupt::EXTI1 => 7,
            Interrupt::EXTI2 => 8,
            Interrupt::EXTI3 => 9,
            Interrupt::EXTI4 => 10,
            Interrupt::ADC => 18,
            Interrupt::EXTI9_5 => 23,
            Interrupt::TIM1_BRK_TIM9 => 24,
            Interrupt::TIM1_UP_TIM10 => 25,
            Interrupt::TIM1_TRG_COM_TIM11 => 26,
            Interrupt::TIM1_CC => 27,
            Interrupt::TIM2 => 28,
            Interrupt::TIM3 => 29,
            Interrupt::TIM4 => 30,
            Interrupt::I2C1_EV => 31,
            Interrupt::I2C1_ER => 32,
            Interrupt::I2C2_EV => 33,
            Interrupt::I2C2_ER => 34,
            Interrupt::SPI1 => 35,
            Interrupt::SPI2 => 36,
            Interrupt::EXTI15_10 => 40,
            Interrupt::RTC_ALARM => 41,
            Interrupt::OTG_FS_WKUP => 42,
            Interrupt::SDIO => 49,
            Interrupt::TIM5 => 50,
            Interrupt::SPI3 => 51,
            Interrupt::OTG_FS => 67,
            Interrupt::I2C3_EV => 72,
            Interrupt::I2C3_ER => 73,
            Interrupt::FPU => 81,
            Interrupt::SPI4 => 84,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "ADC common registers"]
pub struct ADC_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC_COMMON {}
impl ADC_COMMON {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc_common::RegisterBlock {
        1073816320 as *const _
    }
}
impl Deref for ADC_COMMON {
    type Target = adc_common::RegisterBlock;
    fn deref(&self) -> &adc_common::RegisterBlock {
        unsafe { &*ADC_COMMON::ptr() }
    }
}
#[doc = "ADC common registers"]
pub mod adc_common;
#[doc = "Analog-to-digital converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1073815552 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog-to-digital converter"]
pub mod adc1;
#[doc = "Cryptographic processor"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cryptographic processor"]
pub mod crc;
#[doc = "Debug support"]
pub struct DBG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBG {}
impl DBG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dbg::RegisterBlock {
        3758366720 as *const _
    }
}
impl Deref for DBG {
    type Target = dbg::RegisterBlock;
    fn deref(&self) -> &dbg::RegisterBlock {
        unsafe { &*DBG::ptr() }
    }
}
#[doc = "Debug support"]
pub mod dbg;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const exti::RegisterBlock {
        1073822720 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    fn deref(&self) -> &exti::RegisterBlock {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "FLASH"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash::RegisterBlock {
        1073888256 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "FLASH"]
pub mod flash;
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iwdg::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    fn deref(&self) -> &iwdg::RegisterBlock {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_DEVICE {}
impl OTG_FS_DEVICE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_device::RegisterBlock {
        1342179328 as *const _
    }
}
impl Deref for OTG_FS_DEVICE {
    type Target = otg_fs_device::RegisterBlock;
    fn deref(&self) -> &otg_fs_device::RegisterBlock {
        unsafe { &*OTG_FS_DEVICE::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_device;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_GLOBAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_GLOBAL {}
impl OTG_FS_GLOBAL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_global::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for OTG_FS_GLOBAL {
    type Target = otg_fs_global::RegisterBlock;
    fn deref(&self) -> &otg_fs_global::RegisterBlock {
        unsafe { &*OTG_FS_GLOBAL::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_global;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_HOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_HOST {}
impl OTG_FS_HOST {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_host::RegisterBlock {
        1342178304 as *const _
    }
}
impl Deref for OTG_FS_HOST {
    type Target = otg_fs_host::RegisterBlock;
    fn deref(&self) -> &otg_fs_host::RegisterBlock {
        unsafe { &*OTG_FS_HOST::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_host;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_PWRCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_PWRCLK {}
impl OTG_FS_PWRCLK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_pwrclk::RegisterBlock {
        1342180864 as *const _
    }
}
impl Deref for OTG_FS_PWRCLK {
    type Target = otg_fs_pwrclk::RegisterBlock;
    fn deref(&self) -> &otg_fs_pwrclk::RegisterBlock {
        unsafe { &*OTG_FS_PWRCLK::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_pwrclk;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwr::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    fn deref(&self) -> &pwr::RegisterBlock {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcc::RegisterBlock {
        1073887232 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    fn deref(&self) -> &rcc::RegisterBlock {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073752064 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "Secure digital input/output interface"]
pub struct SDIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDIO {}
impl SDIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sdio::RegisterBlock {
        1073818624 as *const _
    }
}
impl Deref for SDIO {
    type Target = sdio::RegisterBlock;
    fn deref(&self) -> &sdio::RegisterBlock {
        unsafe { &*SDIO::ptr() }
    }
}
#[doc = "Secure digital input/output interface"]
pub mod sdio;
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscfg::RegisterBlock {
        1073821696 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    fn deref(&self) -> &syscfg::RegisterBlock {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "Advanced-timers"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub mod tim1;
#[doc = "TIM8"]
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073808384 as *const _
    }
}
impl Deref for TIM8 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM8::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM10 {}
impl TIM10 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim10::RegisterBlock {
        1073824768 as *const _
    }
}
impl Deref for TIM10 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        unsafe { &*TIM10::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim10;
#[doc = "General-purpose-timers"]
pub struct TIM11 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM11 {}
impl TIM11 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim11::RegisterBlock {
        1073825792 as *const _
    }
}
impl Deref for TIM11 {
    type Target = tim11::RegisterBlock;
    fn deref(&self) -> &tim11::RegisterBlock {
        unsafe { &*TIM11::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim11;
#[doc = "General purpose timers"]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM2::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim2;
#[doc = "General purpose timers"]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim3::RegisterBlock {
        1073742848 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim3::RegisterBlock;
    fn deref(&self) -> &tim3::RegisterBlock {
        unsafe { &*TIM3::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim3;
#[doc = "TIM4"]
pub struct TIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM4 {}
impl TIM4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim3::RegisterBlock {
        1073743872 as *const _
    }
}
impl Deref for TIM4 {
    type Target = tim3::RegisterBlock;
    fn deref(&self) -> &tim3::RegisterBlock {
        unsafe { &*TIM4::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim5::RegisterBlock {
        1073744896 as *const _
    }
}
impl Deref for TIM5 {
    type Target = tim5::RegisterBlock;
    fn deref(&self) -> &tim5::RegisterBlock {
        unsafe { &*TIM5::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim5;
#[doc = "General purpose timers"]
pub struct TIM9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM9 {}
impl TIM9 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim9::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for TIM9 {
    type Target = tim9::RegisterBlock;
    fn deref(&self) -> &tim9::RegisterBlock {
        unsafe { &*TIM9::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim9;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "USART2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073759232 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "USART6"]
pub struct USART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART6 {}
impl USART6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073812480 as *const _
    }
}
impl Deref for USART6 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART6::ptr() }
    }
}
#[doc = "Window watchdog"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdg::RegisterBlock {
        1073753088 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    fn deref(&self) -> &wwdg::RegisterBlock {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "Window watchdog"]
pub mod wwdg;
#[doc = "DMA controller"]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma2::RegisterBlock {
        1073898496 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma2::RegisterBlock;
    fn deref(&self) -> &dma2::RegisterBlock {
        unsafe { &*DMA2::ptr() }
    }
}
#[doc = "DMA controller"]
pub mod dma2;
#[doc = "DMA1"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma2::RegisterBlock {
        1073897472 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma2::RegisterBlock;
    fn deref(&self) -> &dma2::RegisterBlock {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioh::RegisterBlock {
        1073880064 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioh::RegisterBlock;
    fn deref(&self) -> &gpioh::RegisterBlock {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioh;
#[doc = "GPIOE"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioh::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioh::RegisterBlock;
    fn deref(&self) -> &gpioh::RegisterBlock {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "GPIOD"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioh::RegisterBlock {
        1073875968 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpioh::RegisterBlock;
    fn deref(&self) -> &gpioh::RegisterBlock {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "GPIOC"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioh::RegisterBlock {
        1073874944 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioh::RegisterBlock;
    fn deref(&self) -> &gpioh::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1073873920 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "Inter-integrated circuit"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c3::RegisterBlock {
        1073765376 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c3::RegisterBlock;
    fn deref(&self) -> &i2c3::RegisterBlock {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub mod i2c3;
#[doc = "I2C2"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c3::RegisterBlock {
        1073764352 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c3::RegisterBlock;
    fn deref(&self) -> &i2c3::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c3::RegisterBlock {
        1073763328 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c3::RegisterBlock;
    fn deref(&self) -> &i2c3::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub struct I2S2EXT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S2EXT {}
impl I2S2EXT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s2ext::RegisterBlock {
        1073755136 as *const _
    }
}
impl Deref for I2S2EXT {
    type Target = i2s2ext::RegisterBlock;
    fn deref(&self) -> &i2s2ext::RegisterBlock {
        unsafe { &*I2S2EXT::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub mod i2s2ext;
#[doc = "I2S3ext"]
pub struct I2S3EXT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S3EXT {}
impl I2S3EXT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s2ext::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for I2S3EXT {
    type Target = i2s2ext::RegisterBlock;
    fn deref(&self) -> &i2s2ext::RegisterBlock {
        unsafe { &*I2S3EXT::ptr() }
    }
}
#[doc = "SPI1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s2ext::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for SPI1 {
    type Target = i2s2ext::RegisterBlock;
    fn deref(&self) -> &i2s2ext::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "SPI2"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s2ext::RegisterBlock {
        1073756160 as *const _
    }
}
impl Deref for SPI2 {
    type Target = i2s2ext::RegisterBlock;
    fn deref(&self) -> &i2s2ext::RegisterBlock {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "SPI3"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s2ext::RegisterBlock {
        1073757184 as *const _
    }
}
impl Deref for SPI3 {
    type Target = i2s2ext::RegisterBlock;
    fn deref(&self) -> &i2s2ext::RegisterBlock {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "SPI4"]
pub struct SPI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI4 {}
impl SPI4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s2ext::RegisterBlock {
        1073820672 as *const _
    }
}
impl Deref for SPI4 {
    type Target = i2s2ext::RegisterBlock;
    fn deref(&self) -> &i2s2ext::RegisterBlock {
        unsafe { &*SPI4::ptr() }
    }
}
#[doc = "SPI5"]
pub struct SPI5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI5 {}
impl SPI5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2s2ext::RegisterBlock {
        1073827840 as *const _
    }
}
impl Deref for SPI5 {
    type Target = i2s2ext::RegisterBlock;
    fn deref(&self) -> &i2s2ext::RegisterBlock {
        unsafe { &*SPI5::ptr() }
    }
}
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ADC_COMMON"]
    pub ADC_COMMON: ADC_COMMON,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "DBG"]
    pub DBG: DBG,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "OTG_FS_DEVICE"]
    pub OTG_FS_DEVICE: OTG_FS_DEVICE,
    #[doc = "OTG_FS_GLOBAL"]
    pub OTG_FS_GLOBAL: OTG_FS_GLOBAL,
    #[doc = "OTG_FS_HOST"]
    pub OTG_FS_HOST: OTG_FS_HOST,
    #[doc = "OTG_FS_PWRCLK"]
    pub OTG_FS_PWRCLK: OTG_FS_PWRCLK,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SDIO"]
    pub SDIO: SDIO,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM8"]
    pub TIM8: TIM8,
    #[doc = "TIM10"]
    pub TIM10: TIM10,
    #[doc = "TIM11"]
    pub TIM11: TIM11,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "TIM4"]
    pub TIM4: TIM4,
    #[doc = "TIM5"]
    pub TIM5: TIM5,
    #[doc = "TIM9"]
    pub TIM9: TIM9,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART6"]
    pub USART6: USART6,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2S2EXT"]
    pub I2S2EXT: I2S2EXT,
    #[doc = "I2S3EXT"]
    pub I2S3EXT: I2S3EXT,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "SPI4"]
    pub SPI4: SPI4,
    #[doc = "SPI5"]
    pub SPI5: SPI5,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| if unsafe { DEVICE_PERIPHERALS } {
            None
        } else {
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ADC_COMMON: ADC_COMMON { _marker: PhantomData },
            ADC1: ADC1 { _marker: PhantomData },
            CRC: CRC { _marker: PhantomData },
            DBG: DBG { _marker: PhantomData },
            EXTI: EXTI { _marker: PhantomData },
            FLASH: FLASH { _marker: PhantomData },
            IWDG: IWDG { _marker: PhantomData },
            OTG_FS_DEVICE: OTG_FS_DEVICE { _marker: PhantomData },
            OTG_FS_GLOBAL: OTG_FS_GLOBAL { _marker: PhantomData },
            OTG_FS_HOST: OTG_FS_HOST { _marker: PhantomData },
            OTG_FS_PWRCLK: OTG_FS_PWRCLK { _marker: PhantomData },
            PWR: PWR { _marker: PhantomData },
            RCC: RCC { _marker: PhantomData },
            RTC: RTC { _marker: PhantomData },
            SDIO: SDIO { _marker: PhantomData },
            SYSCFG: SYSCFG { _marker: PhantomData },
            TIM1: TIM1 { _marker: PhantomData },
            TIM8: TIM8 { _marker: PhantomData },
            TIM10: TIM10 { _marker: PhantomData },
            TIM11: TIM11 { _marker: PhantomData },
            TIM2: TIM2 { _marker: PhantomData },
            TIM3: TIM3 { _marker: PhantomData },
            TIM4: TIM4 { _marker: PhantomData },
            TIM5: TIM5 { _marker: PhantomData },
            TIM9: TIM9 { _marker: PhantomData },
            USART1: USART1 { _marker: PhantomData },
            USART2: USART2 { _marker: PhantomData },
            USART6: USART6 { _marker: PhantomData },
            WWDG: WWDG { _marker: PhantomData },
            DMA2: DMA2 { _marker: PhantomData },
            DMA1: DMA1 { _marker: PhantomData },
            GPIOH: GPIOH { _marker: PhantomData },
            GPIOE: GPIOE { _marker: PhantomData },
            GPIOD: GPIOD { _marker: PhantomData },
            GPIOC: GPIOC { _marker: PhantomData },
            GPIOB: GPIOB { _marker: PhantomData },
            GPIOA: GPIOA { _marker: PhantomData },
            I2C3: I2C3 { _marker: PhantomData },
            I2C2: I2C2 { _marker: PhantomData },
            I2C1: I2C1 { _marker: PhantomData },
            I2S2EXT: I2S2EXT { _marker: PhantomData },
            I2S3EXT: I2S3EXT { _marker: PhantomData },
            SPI1: SPI1 { _marker: PhantomData },
            SPI2: SPI2 { _marker: PhantomData },
            SPI3: SPI3 { _marker: PhantomData },
            SPI4: SPI4 { _marker: PhantomData },
            SPI5: SPI5 { _marker: PhantomData },
        }
    }
}
