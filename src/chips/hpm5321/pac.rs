#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - CORE_LOCAL"]
    CORE_LOCAL = 0,
    #[doc = "1 - GPIO0_A"]
    GPIO0_A = 1,
    #[doc = "2 - GPIO0_B"]
    GPIO0_B = 2,
    #[doc = "3 - GPIO0_X"]
    GPIO0_X = 3,
    #[doc = "4 - GPIO0_Y"]
    GPIO0_Y = 4,
    #[doc = "5 - GPTMR0"]
    GPTMR0 = 5,
    #[doc = "6 - GPTMR1"]
    GPTMR1 = 6,
    #[doc = "7 - GPTMR2"]
    GPTMR2 = 7,
    #[doc = "8 - GPTMR3"]
    GPTMR3 = 8,
    #[doc = "13 - UART0"]
    UART0 = 13,
    #[doc = "14 - UART1"]
    UART1 = 14,
    #[doc = "15 - UART2"]
    UART2 = 15,
    #[doc = "16 - UART3"]
    UART3 = 16,
    #[doc = "17 - UART4"]
    UART4 = 17,
    #[doc = "18 - UART5"]
    UART5 = 18,
    #[doc = "19 - UART6"]
    UART6 = 19,
    #[doc = "20 - UART7"]
    UART7 = 20,
    #[doc = "21 - I2C0"]
    I2C0 = 21,
    #[doc = "22 - I2C1"]
    I2C1 = 22,
    #[doc = "23 - I2C2"]
    I2C2 = 23,
    #[doc = "24 - I2C3"]
    I2C3 = 24,
    #[doc = "25 - SPI0"]
    SPI0 = 25,
    #[doc = "26 - SPI1"]
    SPI1 = 26,
    #[doc = "27 - SPI2"]
    SPI2 = 27,
    #[doc = "28 - SPI3"]
    SPI3 = 28,
    #[doc = "29 - TSNS"]
    TSNS = 29,
    #[doc = "30 - MBX0A"]
    MBX0A = 30,
    #[doc = "31 - MBX0B"]
    MBX0B = 31,
    #[doc = "32 - EWDG0"]
    EWDG0 = 32,
    #[doc = "33 - EWDG1"]
    EWDG1 = 33,
    #[doc = "34 - HDMA"]
    HDMA = 34,
    #[doc = "35 - MCAN0"]
    MCAN0 = 35,
    #[doc = "36 - MCAN1"]
    MCAN1 = 36,
    #[doc = "37 - MCAN2"]
    MCAN2 = 37,
    #[doc = "38 - MCAN3"]
    MCAN3 = 38,
    #[doc = "39 - PTPC"]
    PTPC = 39,
    #[doc = "40 - PWM0"]
    PWM0 = 40,
    #[doc = "41 - QEI0"]
    QEI0 = 41,
    #[doc = "42 - SEI0"]
    SEI0 = 42,
    #[doc = "43 - MMC0"]
    MMC0 = 43,
    #[doc = "44 - TRGMUX0"]
    TRGMUX0 = 44,
    #[doc = "45 - PWM1"]
    PWM1 = 45,
    #[doc = "46 - QEI1"]
    QEI1 = 46,
    #[doc = "47 - SEI1"]
    SEI1 = 47,
    #[doc = "48 - MMC1"]
    MMC1 = 48,
    #[doc = "49 - TRGMUX1"]
    TRGMUX1 = 49,
    #[doc = "50 - RDC"]
    RDC = 50,
    #[doc = "51 - USB0"]
    USB0 = 51,
    #[doc = "52 - XPI0"]
    XPI0 = 52,
    #[doc = "53 - SDP"]
    SDP = 53,
    #[doc = "54 - PSEC"]
    PSEC = 54,
    #[doc = "55 - SECMON"]
    SECMON = 55,
    #[doc = "56 - RNG"]
    RNG = 56,
    #[doc = "57 - FUSE"]
    FUSE = 57,
    #[doc = "58 - ADC0"]
    ADC0 = 58,
    #[doc = "59 - ADC1"]
    ADC1 = 59,
    #[doc = "60 - DAC0"]
    DAC0 = 60,
    #[doc = "61 - DAC1"]
    DAC1 = 61,
    #[doc = "62 - ACMP_0"]
    ACMP_0 = 62,
    #[doc = "63 - ACMP_1"]
    ACMP_1 = 63,
    #[doc = "64 - SYSCTL"]
    SYSCTL = 64,
    #[doc = "65 - PGPIO"]
    PGPIO = 65,
    #[doc = "66 - PTMR"]
    PTMR = 66,
    #[doc = "67 - PUART"]
    PUART = 67,
    #[doc = "68 - PEWDG"]
    PEWDG = 68,
    #[doc = "69 - BROWNOUT"]
    BROWNOUT = 69,
    #[doc = "70 - PAD_WAKEUP"]
    PAD_WAKEUP = 70,
    #[doc = "71 - DEBUG0"]
    DEBUG0 = 71,
    #[doc = "72 - DEBUG1"]
    DEBUG1 = 72,
}
unsafe impl crate::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    extern "C" {
        fn CORE_LOCAL();
        fn GPIO0_A();
        fn GPIO0_B();
        fn GPIO0_X();
        fn GPIO0_Y();
        fn GPTMR0();
        fn GPTMR1();
        fn GPTMR2();
        fn GPTMR3();
        fn UART0();
        fn UART1();
        fn UART2();
        fn UART3();
        fn UART4();
        fn UART5();
        fn UART6();
        fn UART7();
        fn I2C0();
        fn I2C1();
        fn I2C2();
        fn I2C3();
        fn SPI0();
        fn SPI1();
        fn SPI2();
        fn SPI3();
        fn TSNS();
        fn MBX0A();
        fn MBX0B();
        fn EWDG0();
        fn EWDG1();
        fn HDMA();
        fn MCAN0();
        fn MCAN1();
        fn MCAN2();
        fn MCAN3();
        fn PTPC();
        fn PWM0();
        fn QEI0();
        fn SEI0();
        fn MMC0();
        fn TRGMUX0();
        fn PWM1();
        fn QEI1();
        fn SEI1();
        fn MMC1();
        fn TRGMUX1();
        fn RDC();
        fn USB0();
        fn XPI0();
        fn SDP();
        fn PSEC();
        fn SECMON();
        fn RNG();
        fn FUSE();
        fn ADC0();
        fn ADC1();
        fn DAC0();
        fn DAC1();
        fn ACMP_0();
        fn ACMP_1();
        fn SYSCTL();
        fn PGPIO();
        fn PTMR();
        fn PUART();
        fn PEWDG();
        fn BROWNOUT();
        fn PAD_WAKEUP();
        fn DEBUG0();
        fn DEBUG1();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __VECTORED_INTERRUPTS: [Vector; 73] = [
        Vector {
            _handler: CORE_LOCAL,
        },
        Vector { _handler: GPIO0_A },
        Vector { _handler: GPIO0_B },
        Vector { _handler: GPIO0_X },
        Vector { _handler: GPIO0_Y },
        Vector { _handler: GPTMR0 },
        Vector { _handler: GPTMR1 },
        Vector { _handler: GPTMR2 },
        Vector { _handler: GPTMR3 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: UART0 },
        Vector { _handler: UART1 },
        Vector { _handler: UART2 },
        Vector { _handler: UART3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: UART6 },
        Vector { _handler: UART7 },
        Vector { _handler: I2C0 },
        Vector { _handler: I2C1 },
        Vector { _handler: I2C2 },
        Vector { _handler: I2C3 },
        Vector { _handler: SPI0 },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: SPI3 },
        Vector { _handler: TSNS },
        Vector { _handler: MBX0A },
        Vector { _handler: MBX0B },
        Vector { _handler: EWDG0 },
        Vector { _handler: EWDG1 },
        Vector { _handler: HDMA },
        Vector { _handler: MCAN0 },
        Vector { _handler: MCAN1 },
        Vector { _handler: MCAN2 },
        Vector { _handler: MCAN3 },
        Vector { _handler: PTPC },
        Vector { _handler: PWM0 },
        Vector { _handler: QEI0 },
        Vector { _handler: SEI0 },
        Vector { _handler: MMC0 },
        Vector { _handler: TRGMUX0 },
        Vector { _handler: PWM1 },
        Vector { _handler: QEI1 },
        Vector { _handler: SEI1 },
        Vector { _handler: MMC1 },
        Vector { _handler: TRGMUX1 },
        Vector { _handler: RDC },
        Vector { _handler: USB0 },
        Vector { _handler: XPI0 },
        Vector { _handler: SDP },
        Vector { _handler: PSEC },
        Vector { _handler: SECMON },
        Vector { _handler: RNG },
        Vector { _handler: FUSE },
        Vector { _handler: ADC0 },
        Vector { _handler: ADC1 },
        Vector { _handler: DAC0 },
        Vector { _handler: DAC1 },
        Vector { _handler: ACMP_0 },
        Vector { _handler: ACMP_1 },
        Vector { _handler: SYSCTL },
        Vector { _handler: PGPIO },
        Vector { _handler: PTMR },
        Vector { _handler: PUART },
        Vector { _handler: PEWDG },
        Vector { _handler: BROWNOUT },
        Vector {
            _handler: PAD_WAKEUP,
        },
        Vector { _handler: DEBUG0 },
        Vector { _handler: DEBUG1 },
    ];
}
pub const FGPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x000c_0000usize as _) };
pub const PLIC: plic::Plic = unsafe { plic::Plic::from_ptr(0xe400_0000usize as _) };
pub const MCHTMR: mchtmr::Mchtmr = unsafe { mchtmr::Mchtmr::from_ptr(0xe600_0000usize as _) };
pub const PLICSW: plicsw::Plicsw = unsafe { plicsw::Plicsw::from_ptr(0xe640_0000usize as _) };
pub const GPTMR0: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf000_0000usize as _) };
pub const GPTMR1: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf000_4000usize as _) };
pub const GPTMR2: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf000_8000usize as _) };
pub const GPTMR3: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf000_c000usize as _) };
pub const UART0: uart::Uart = unsafe { uart::Uart::from_ptr(0xf004_0000usize as _) };
pub const UART1: uart::Uart = unsafe { uart::Uart::from_ptr(0xf004_4000usize as _) };
pub const UART2: uart::Uart = unsafe { uart::Uart::from_ptr(0xf004_8000usize as _) };
pub const UART3: uart::Uart = unsafe { uart::Uart::from_ptr(0xf004_c000usize as _) };
pub const UART4: uart::Uart = unsafe { uart::Uart::from_ptr(0xf005_0000usize as _) };
pub const UART5: uart::Uart = unsafe { uart::Uart::from_ptr(0xf005_4000usize as _) };
pub const UART6: uart::Uart = unsafe { uart::Uart::from_ptr(0xf005_8000usize as _) };
pub const UART7: uart::Uart = unsafe { uart::Uart::from_ptr(0xf005_c000usize as _) };
pub const I2C0: i2c::I2c = unsafe { i2c::I2c::from_ptr(0xf006_0000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0xf006_4000usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0xf006_8000usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0xf006_c000usize as _) };
pub const SPI0: spi::Spi = unsafe { spi::Spi::from_ptr(0xf007_0000usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0xf007_4000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0xf007_8000usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0xf007_c000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0xf008_0000usize as _) };
pub const TSNS: tsns::Tsns = unsafe { tsns::Tsns::from_ptr(0xf009_0000usize as _) };
pub const MBX0A: mbx::Mbx = unsafe { mbx::Mbx::from_ptr(0xf00a_0000usize as _) };
pub const MBX0B: mbx::Mbx = unsafe { mbx::Mbx::from_ptr(0xf00a_4000usize as _) };
pub const WDG0: wdg::Wdg = unsafe { wdg::Wdg::from_ptr(0xf00b_0000usize as _) };
pub const WDG1: wdg::Wdg = unsafe { wdg::Wdg::from_ptr(0xf00b_4000usize as _) };
pub const DMAMUX: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0xf00c_4000usize as _) };
pub const HDMA: dma::Dma = unsafe { dma::Dma::from_ptr(0xf00c_8000usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0xf00c_8000usize as _) };
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf00d_0000usize as _) };
pub const GPIOM: gpiom::Gpiom = unsafe { gpiom::Gpiom::from_ptr(0xf00d_8000usize as _) };
pub const MCAN0: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf028_0000usize as _) };
pub const MCAN1: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf028_4000usize as _) };
pub const MCAN2: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf028_8000usize as _) };
pub const MCAN3: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf028_c000usize as _) };
pub const PTPC: ptpc::Ptpc = unsafe { ptpc::Ptpc::from_ptr(0xf02f_c000usize as _) };
pub const PLB: plb::Plb = unsafe { plb::Plb::from_ptr(0xf032_4000usize as _) };
pub const XPI0: xpi::Xpi = unsafe { xpi::Xpi::from_ptr(0xf300_0000usize as _) };
pub const USB0: usb::Usb = unsafe { usb::Usb::from_ptr(0xf300_c000usize as _) };
pub const SDP: sdp::Sdp = unsafe { sdp::Sdp::from_ptr(0xf304_0000usize as _) };
pub const SEC: sec::Sec = unsafe { sec::Sec::from_ptr(0xf304_4000usize as _) };
pub const PMON: pmon::Pmon = unsafe { pmon::Pmon::from_ptr(0xf304_8000usize as _) };
pub const OTP: otp::Otp = unsafe { otp::Otp::from_ptr(0xf305_0000usize as _) };
pub const KEYM: keym::Keym = unsafe { keym::Keym::from_ptr(0xf305_4000usize as _) };
pub const ADC0: adc16::Adc = unsafe { adc16::Adc::from_ptr(0xf308_0000usize as _) };
pub const ADC1: adc16::Adc = unsafe { adc16::Adc::from_ptr(0xf308_4000usize as _) };
pub const ACMP: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0xf30b_0000usize as _) };
pub const SYSCTL: sysctl::Sysctl = unsafe { sysctl::Sysctl::from_ptr(0xf400_0000usize as _) };
pub const IOC: ioc::Ioc = unsafe { ioc::Ioc::from_ptr(0xf404_0000usize as _) };
pub const PLLCTL: pllctl::Pllctlv2 = unsafe { pllctl::Pllctlv2::from_ptr(0xf40c_0000usize as _) };
pub const PPOR: ppor::Ppor = unsafe { ppor::Ppor::from_ptr(0xf410_0000usize as _) };
pub const PCFG: pcfg::Pcfg = unsafe { pcfg::Pcfg::from_ptr(0xf410_4000usize as _) };
pub const PIOC: ioc::Ioc = unsafe { ioc::Ioc::from_ptr(0xf411_8000usize as _) };
pub const PGPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf411_c000usize as _) };
pub const PTMR: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf412_0000usize as _) };
pub const PUART: uart::Uart = unsafe { uart::Uart::from_ptr(0xf412_4000usize as _) };
pub const PWDG: wdg::Wdg = unsafe { wdg::Wdg::from_ptr(0xf412_8000usize as _) };
pub const PDGO: pdgo::Pdgo = unsafe { pdgo::Pdgo::from_ptr(0xf413_4000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/acmp_common.rs"]
pub mod acmp;
#[path = "../../peripherals/adc16_v53.rs"]
pub mod adc16;
#[path = "../../peripherals/crc_common.rs"]
pub mod crc;
#[path = "../../peripherals/dma_v53.rs"]
pub mod dma;
#[path = "../../peripherals/dmamux_common.rs"]
pub mod dmamux;
#[path = "../../peripherals/gpio_v53.rs"]
pub mod gpio;
#[path = "../../peripherals/gpiom_v53.rs"]
pub mod gpiom;
#[path = "../../peripherals/i2c_v53.rs"]
pub mod i2c;
#[path = "../../peripherals/ioc_common.rs"]
pub mod ioc;
#[path = "../../peripherals/keym_common.rs"]
pub mod keym;
#[path = "../../peripherals/mbx_common.rs"]
pub mod mbx;
#[path = "../../peripherals/mcan_v53.rs"]
pub mod mcan;
#[path = "../../peripherals/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../../peripherals/otp_common.rs"]
pub mod otp;
#[path = "../../peripherals/pcfg_v53.rs"]
pub mod pcfg;
#[path = "../../peripherals/pdgo_v53.rs"]
pub mod pdgo;
#[path = "../../peripherals/plb_v53.rs"]
pub mod plb;
#[path = "../../peripherals/plic_common.rs"]
pub mod plic;
#[path = "../../peripherals/plicsw_common.rs"]
pub mod plicsw;
#[path = "../../peripherals/pllctl_v2.rs"]
pub mod pllctl;
#[path = "../../peripherals/pmon_common.rs"]
pub mod pmon;
#[path = "../../peripherals/ppor_v53.rs"]
pub mod ppor;
#[path = "../../peripherals/ptpc_common.rs"]
pub mod ptpc;
#[path = "../../peripherals/rng_common.rs"]
pub mod rng;
#[path = "../../peripherals/sdp_v53.rs"]
pub mod sdp;
#[path = "../../peripherals/sec_common.rs"]
pub mod sec;
#[path = "../../peripherals/spi_v53.rs"]
pub mod spi;
#[path = "../../peripherals/sysctl_v53.rs"]
pub mod sysctl;
#[path = "../../peripherals/tmr_common.rs"]
pub mod tmr;
#[path = "../../peripherals/tsns_common.rs"]
pub mod tsns;
#[path = "../../peripherals/uart_v53.rs"]
pub mod uart;
#[path = "../../peripherals/usb_v53.rs"]
pub mod usb;
#[path = "../../peripherals/wdg_v53.rs"]
pub mod wdg;
#[path = "../../peripherals/xpi_dummy.rs"]
pub mod xpi;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 2147483648;
pub const FLASH_SIZE: usize = 1048576;
pub mod resources {
    pub const REF0: usize = 309;
    pub const POW_CPU0: usize = 21;
    pub const RST_SOC: usize = 22;
    pub const TMR2: usize = 271;
    pub const CPX0: usize = 1;
    pub const MCT0: usize = 258;
    pub const URT3: usize = 284;
    pub const CLK_TOP_URT7: usize = 93;
    pub const CLK_TOP_REF0: usize = 99;
    pub const ADC1: usize = 295;
    pub const TMR0: usize = 269;
    pub const RNG0: usize = 302;
    pub const I2C0: usize = 273;
    pub const HDMA: usize = 306;
    pub const CLK_TOP_URT0: usize = 86;
    pub const ROM0: usize = 259;
    pub const CRC0: usize = 293;
    pub const CLK_TOP_CAN2: usize = 68;
    pub const CAN3: usize = 263;
    pub const CLK_TOP_I2C3: usize = 81;
    pub const CLK_TOP_I2C1: usize = 79;
    pub const CLK_TOP_XPI0: usize = 94;
    pub const CLK_TOP_URT3: usize = 89;
    pub const CLK_SRC_CLK1_PLL1: usize = 39;
    pub const CLK_TOP_DAC1: usize = 104;
    pub const CLK_TOP_URT2: usize = 88;
    pub const ACMP: usize = 298;
    pub const OPA1: usize = 300;
    pub const KMAN: usize = 304;
    pub const CLK_SRC_XTAL: usize = 32;
    pub const SPI0: usize = 277;
    pub const CLK_TOP_URT6: usize = 92;
    pub const WDG1: usize = 290;
    pub const CLK_TOP_SPI2: usize = 84;
    pub const I2C1: usize = 274;
    pub const URT2: usize = 283;
    pub const CLK_TOP_TMR0: usize = 74;
    pub const CLK_SRC_CLK2_PLL1: usize = 40;
    pub const CAN1: usize = 261;
    pub const CLK_TOP_CAN1: usize = 67;
    pub const CLK_TOP_SPI1: usize = 83;
    pub const CLK_TOP_SPI3: usize = 85;
    pub const AHB0: usize = 256;
    pub const SPI1: usize = 278;
    pub const SPI3: usize = 280;
    pub const URT5: usize = 286;
    pub const CLK_TOP_I2C2: usize = 80;
    pub const LMM0: usize = 257;
    pub const CLK_SRC_CLK0_PLL1: usize = 38;
    pub const CLK_TOP_SPI0: usize = 82;
    pub const CPU0: usize = 0;
    pub const TMR3: usize = 272;
    pub const SPI2: usize = 279;
    pub const WDG0: usize = 289;
    pub const OPA0: usize = 299;
    pub const USB0: usize = 308;
    pub const DAC0: usize = 296;
    pub const CLK_TOP_REF1: usize = 100;
    pub const URT1: usize = 282;
    pub const URT4: usize = 285;
    pub const URT7: usize = 288;
    pub const CLK_SRC_CLK3_PLL1: usize = 41;
    pub const CLK_TOP_MCT0: usize = 65;
    pub const TSNS: usize = 292;
    pub const CAN0: usize = 260;
    pub const CLK_TOP_TMR3: usize = 77;
    pub const CLK_TOP_I2C0: usize = 78;
    pub const CLK_TOP_URT5: usize = 91;
    pub const CLK_TOP_DAC0: usize = 103;
    pub const I2C2: usize = 275;
    pub const XPI0: usize = 307;
    pub const CLK_TOP_ANA1: usize = 96;
    pub const DAC1: usize = 297;
    pub const REF1: usize = 310;
    pub const CLK_TOP_CAN0: usize = 66;
    pub const SDP0: usize = 303;
    pub const I2C3: usize = 276;
    pub const CLK_TOP_TMR2: usize = 76;
    pub const CLK_TOP_CPU0: usize = 64;
    pub const CLK_SRC_CLK2_PLL0: usize = 36;
    pub const CLK_TOP_URT4: usize = 90;
    pub const CLK_TOP_ANA2: usize = 97;
    pub const CLK_TOP_TMR1: usize = 75;
    pub const URT0: usize = 281;
    pub const URT6: usize = 287;
    pub const GPIO: usize = 305;
    pub const RST_CPU0: usize = 23;
    pub const CLK_SRC_PLL0: usize = 33;
    pub const CLK_SRC_PLL1: usize = 37;
    pub const CLK_TOP_URT1: usize = 87;
    pub const TMR1: usize = 270;
    pub const CLK_TOP_ADC0: usize = 101;
    pub const ADC0: usize = 294;
    pub const PTPC: usize = 264;
    pub const CLK_SRC_CLK0_PLL0: usize = 34;
    pub const CLK_TOP_CAN3: usize = 69;
    pub const CLK_SRC_PLL1_REF: usize = 43;
    pub const CAN2: usize = 262;
    pub const MBX0: usize = 291;
    pub const CLK_SRC_PLL0_REF: usize = 42;
    pub const CLK_TOP_ANA0: usize = 95;
    pub const CLK_TOP_ADC1: usize = 102;
    pub const MOT0: usize = 301;
    pub const CLK_TOP_ANA3: usize = 98;
    pub const CLK_SRC_CLK1_PLL0: usize = 35;
}
pub mod clocks {
    pub const SPI0: usize = 17;
    pub const SPI2: usize = 19;
    pub const CAN0: usize = 1;
    pub const MCT0: usize = 0;
    pub const URT3: usize = 24;
    pub const I2C2: usize = 15;
    pub const XPI0: usize = 29;
    pub const TMR2: usize = 11;
    pub const ANA1: usize = 31;
    pub const ANA3: usize = 33;
    pub const REF0: usize = 34;
    pub const REF1: usize = 35;
    pub const SPI3: usize = 20;
    pub const SPI1: usize = 18;
    pub const URT1: usize = 22;
    pub const URT2: usize = 23;
    pub const ANA2: usize = 32;
    pub const CAN1: usize = 2;
    pub const URT5: usize = 26;
    pub const TMR1: usize = 10;
    pub const URT7: usize = 28;
    pub const URT0: usize = 21;
    pub const I2C1: usize = 14;
    pub const I2C3: usize = 16;
    pub const CAN2: usize = 3;
    pub const ANA0: usize = 30;
    pub const TMR3: usize = 12;
    pub const URT4: usize = 25;
    pub const I2C0: usize = 13;
    pub const URT6: usize = 27;
    pub const TMR0: usize = 9;
    pub const CAN3: usize = 4;
}
pub mod pins {
    pub const PA00: usize = 0;
    pub const PA01: usize = 1;
    pub const PA02: usize = 2;
    pub const PA03: usize = 3;
    pub const PA04: usize = 4;
    pub const PA05: usize = 5;
    pub const PA06: usize = 6;
    pub const PA07: usize = 7;
    pub const PA08: usize = 8;
    pub const PA09: usize = 9;
    pub const PA10: usize = 10;
    pub const PA11: usize = 11;
    pub const PA12: usize = 12;
    pub const PA13: usize = 13;
    pub const PA14: usize = 14;
    pub const PA15: usize = 15;
    pub const PA16: usize = 16;
    pub const PA17: usize = 17;
    pub const PA18: usize = 18;
    pub const PA19: usize = 19;
    pub const PA20: usize = 20;
    pub const PA21: usize = 21;
    pub const PA22: usize = 22;
    pub const PA23: usize = 23;
    pub const PA24: usize = 24;
    pub const PA25: usize = 25;
    pub const PA26: usize = 26;
    pub const PA27: usize = 27;
    pub const PA28: usize = 28;
    pub const PA29: usize = 29;
    pub const PA30: usize = 30;
    pub const PA31: usize = 31;
    pub const PB00: usize = 32;
    pub const PB01: usize = 33;
    pub const PB02: usize = 34;
    pub const PB03: usize = 35;
    pub const PB04: usize = 36;
    pub const PB05: usize = 37;
    pub const PB06: usize = 38;
    pub const PB07: usize = 39;
    pub const PB08: usize = 40;
    pub const PB09: usize = 41;
    pub const PB10: usize = 42;
    pub const PB11: usize = 43;
    pub const PB12: usize = 44;
    pub const PB13: usize = 45;
    pub const PB14: usize = 46;
    pub const PB15: usize = 47;
    pub const PX00: usize = 416;
    pub const PX01: usize = 416;
    pub const PX03: usize = 417;
    pub const PX02: usize = 417;
    pub const PX05: usize = 418;
    pub const PX04: usize = 418;
    pub const PX06: usize = 419;
    pub const PX07: usize = 419;
    pub const PY00: usize = 448;
    pub const PY01: usize = 449;
    pub const PY02: usize = 450;
    pub const PY03: usize = 451;
    pub const PY04: usize = 452;
    pub const PY05: usize = 453;
    pub const PY06: usize = 454;
    pub const PY07: usize = 455;
}
