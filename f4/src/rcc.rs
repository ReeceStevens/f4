use stm32f40x::{RCC, PWR, FLASH};

const ABP1_CLOCK_SPEED: u32 = 21_000_000; // 21 MHz
const HSI_VALUE: u32 = 16_000_000;
// const HSE_VALUE: u32 = 25_000_000;
const HSE_VALUE: u32 = 8_000_000; // On the discovery board, HSE == 8Mhz
const PLLM: u32 = 0x0000003F;
const PLLN: u32 = 0x00003FC0;
const PLLP: u32 = 0x0000C000;
const PLLQ: u32 = 0x00F00000;
const APBAHBPrescTable: [u32; 16] = [0, 0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 6, 7, 8, 9];

// #define PLL_M      25
// #define PLL_N      336

// /* SYSCLK = PLL_VCO / PLL_P */
// #define PLL_P      2

// /* USB OTG FS, SDIO and RNG Clock =  PLL_VCO / PLLQ */
// #define PLL_Q      7

#[derive(Copy,Clone)]
pub enum SysClkSource {
    HSI = 0b00,
    HSE = 0b01,
    PLL = 0b10
}

pub fn get_pll_freq(rcc: &RCC) -> u32 {
    let pll_src = rcc.pllcfgr.read().pllsrc().bit();
    let pllm = rcc.pllcfgr.read().bits() & PLLM;
    let plln = (rcc.pllcfgr.read().bits() & PLLN) >> 6;
    let pllp = (((rcc.pllcfgr.read().bits() & PLLP) >> 16) + 1) * 2;

    let pllvco = if pll_src == true {
        (HSE_VALUE / pllm) * plln
    } else {
        (HSI_VALUE / pllm) * plln
    };
    return pllvco / pllp;

}

pub fn get_sysclk_freq(rcc: &RCC) -> u32 {
    if rcc.cfgr.read().sws1().bit() {
        get_pll_freq(rcc)
    } else if rcc.cfgr.read().sws0().bit() {
        HSE_VALUE
    } else {
        HSI_VALUE
    }
}

pub fn get_hclk_freq(rcc: &RCC) -> u32 {
    let hpre = rcc.cfgr.read().hpre().bits() as usize;
    let prescaler = APBAHBPrescTable[hpre];
    get_sysclk_freq(rcc) >> prescaler
}

pub fn get_pclk1(rcc: &RCC) -> u32 {
    let ppre1 = rcc.cfgr.read().ppre1().bits() as usize;
    let prescaler = APBAHBPrescTable[ppre1];
    get_hclk_freq(rcc) >> prescaler
}

pub fn get_pclk2(rcc: &RCC) -> u32 {
    let ppre2 = rcc.cfgr.read().ppre2().bits() as usize;
    let prescaler = APBAHBPrescTable[ppre2];
    get_hclk_freq(rcc) >> prescaler
}

#[derive(Clone,Copy,Debug)]
pub struct Clocks {
    sysclk: u32,
    hclk: u32,
    pclk1: u32,
    pclk2: u32
}

pub fn configure_system_clocks(src: SysClkSource, rcc: &RCC, pwr: &PWR, flash: &FLASH) -> Clocks {
    rcc.cr.write(|w| w.hseon().set_bit());
    rcc.apb1enr.write(|w| w.pwren().set_bit());
    pwr.cr.write(|w| unsafe {w.vos().bits(0b11)});
    rcc.cfgr.reset();
    // STM32 Drivers Say:
    // HCLK = SYSCLK / 1
    // PCLK1 = HCLK / 4
    // PCLK2 = HCLK / 2
    rcc.cfgr.modify(|_, w| unsafe {w.hpre().bits(0x00)});
    rcc.cfgr.modify(|_, w| unsafe {w.ppre1().bits(0x6)});
    rcc.cfgr.modify(|_, w| unsafe {w.ppre2().bits(0x5)});
    // Suggested PLL setting:
    let pllm = 25;
    let plln = 336;
    let pllp = 2;
    let pllp_bits = (pllp / 2) - 1;
    let pllq = 7;
    let pllcfgr_register = (pllm & PLLM) | 
                           ((plln << 6) & PLLN) |
                           ((pllp_bits << 16) & PLLP) |
                           ((pllq << 24) & PLLQ);
    rcc.pllcfgr.write(|w| unsafe{w.bits(pllcfgr_register)});
    // and use HSE as the source
    rcc.pllcfgr.modify(|_, w| w.pllsrc().set_bit());

    rcc.cr.modify(|_, w| w.pllon().set_bit());

    flash.acr.modify(|_, w| w.icen().set_bit());
    flash.acr.modify(|_, w| w.dcen().set_bit());
    flash.acr.modify(|_, w| unsafe {w.latency().bits(5)});

    match src { 
        SysClkSource::HSI => {
            while !(rcc.cfgr.read().sws0().bit_is_clear() && rcc.cfgr.read().sws1().bit_is_clear()) {}
        },
        SysClkSource::HSE => {
            rcc.cfgr.modify(|_, w| w.sw0().set_bit());
            while !(rcc.cfgr.read().sws0().bit_is_set()) {}
        },
        SysClkSource::PLL => {
            rcc.cfgr.modify(|_, w| w.sw1().set_bit());
            while !(rcc.cfgr.read().sws1().bit_is_set()) {}
        }
    };
    
    Clocks {
        sysclk: get_sysclk_freq(rcc),
        hclk: get_hclk_freq(rcc),
        pclk1: get_pclk1(rcc),
        pclk2:  get_pclk2(rcc)
    }

}
