#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - CORE_LOCAL"]
    CORE_LOCAL = 0,
    #[doc = "1 - GPIO0_A"]
    GPIO0_A = 1,
    #[doc = "2 - GPIO0_B"]
    GPIO0_B = 2,
    #[doc = "3 - GPIO0_C"]
    GPIO0_C = 3,
    #[doc = "4 - GPIO0_D"]
    GPIO0_D = 4,
    #[doc = "5 - GPIO0_X"]
    GPIO0_X = 5,
    #[doc = "6 - GPIO0_Y"]
    GPIO0_Y = 6,
    #[doc = "7 - GPIO0_Z"]
    GPIO0_Z = 7,
    #[doc = "8 - GPIO1_A"]
    GPIO1_A = 8,
    #[doc = "9 - GPIO1_B"]
    GPIO1_B = 9,
    #[doc = "10 - GPIO1_C"]
    GPIO1_C = 10,
    #[doc = "11 - GPIO1_D"]
    GPIO1_D = 11,
    #[doc = "12 - GPIO1_X"]
    GPIO1_X = 12,
    #[doc = "13 - GPIO1_Y"]
    GPIO1_Y = 13,
    #[doc = "14 - GPIO1_Z"]
    GPIO1_Z = 14,
    #[doc = "15 - ADC0"]
    ADC0 = 15,
    #[doc = "16 - ADC1"]
    ADC1 = 16,
    #[doc = "17 - ADC2"]
    ADC2 = 17,
    #[doc = "18 - SDFM"]
    SDFM = 18,
    #[doc = "19 - DAC0"]
    DAC0 = 19,
    #[doc = "20 - DAC1"]
    DAC1 = 20,
    #[doc = "21 - ACMP_0"]
    ACMP_0 = 21,
    #[doc = "22 - ACMP_1"]
    ACMP_1 = 22,
    #[doc = "23 - ACMP_2"]
    ACMP_2 = 23,
    #[doc = "24 - ACMP_3"]
    ACMP_3 = 24,
    #[doc = "25 - SPI0"]
    SPI0 = 25,
    #[doc = "26 - SPI1"]
    SPI1 = 26,
    #[doc = "27 - SPI2"]
    SPI2 = 27,
    #[doc = "28 - SPI3"]
    SPI3 = 28,
    #[doc = "29 - UART0"]
    UART0 = 29,
    #[doc = "30 - UART1"]
    UART1 = 30,
    #[doc = "31 - UART2"]
    UART2 = 31,
    #[doc = "32 - UART3"]
    UART3 = 32,
    #[doc = "33 - UART4"]
    UART4 = 33,
    #[doc = "34 - UART5"]
    UART5 = 34,
    #[doc = "35 - UART6"]
    UART6 = 35,
    #[doc = "36 - UART7"]
    UART7 = 36,
    #[doc = "37 - MCAN0"]
    MCAN0 = 37,
    #[doc = "38 - MCAN1"]
    MCAN1 = 38,
    #[doc = "39 - MCAN2"]
    MCAN2 = 39,
    #[doc = "40 - MCAN3"]
    MCAN3 = 40,
    #[doc = "41 - PTPC"]
    PTPC = 41,
    #[doc = "42 - WDG0"]
    WDG0 = 42,
    #[doc = "43 - WDG1"]
    WDG1 = 43,
    #[doc = "44 - TSNS"]
    TSNS = 44,
    #[doc = "45 - MBX0A"]
    MBX0A = 45,
    #[doc = "46 - MBX0B"]
    MBX0B = 46,
    #[doc = "47 - MBX1A"]
    MBX1A = 47,
    #[doc = "48 - MBX1B"]
    MBX1B = 48,
    #[doc = "49 - GPTMR0"]
    GPTMR0 = 49,
    #[doc = "50 - GPTMR1"]
    GPTMR1 = 50,
    #[doc = "51 - GPTMR2"]
    GPTMR2 = 51,
    #[doc = "52 - GPTMR3"]
    GPTMR3 = 52,
    #[doc = "53 - I2C0"]
    I2C0 = 53,
    #[doc = "54 - I2C1"]
    I2C1 = 54,
    #[doc = "55 - I2C2"]
    I2C2 = 55,
    #[doc = "56 - I2C3"]
    I2C3 = 56,
    #[doc = "57 - PWM0"]
    PWM0 = 57,
    #[doc = "58 - HALL0"]
    HALL0 = 58,
    #[doc = "59 - QEI0"]
    QEI0 = 59,
    #[doc = "60 - PWM1"]
    PWM1 = 60,
    #[doc = "61 - HALL1"]
    HALL1 = 61,
    #[doc = "62 - QEI1"]
    QEI1 = 62,
    #[doc = "63 - PWM2"]
    PWM2 = 63,
    #[doc = "64 - HALL2"]
    HALL2 = 64,
    #[doc = "65 - QEI2"]
    QEI2 = 65,
    #[doc = "66 - PWM3"]
    PWM3 = 66,
    #[doc = "67 - HALL3"]
    HALL3 = 67,
    #[doc = "68 - QEI3"]
    QEI3 = 68,
    #[doc = "69 - SDP"]
    SDP = 69,
    #[doc = "70 - XPI0"]
    XPI0 = 70,
    #[doc = "71 - XDMA"]
    XDMA = 71,
    #[doc = "72 - HDMA"]
    HDMA = 72,
    #[doc = "73 - RNG"]
    RNG = 73,
    #[doc = "74 - USB0"]
    USB0 = 74,
    #[doc = "75 - PSEC"]
    PSEC = 75,
    #[doc = "76 - PGPIO"]
    PGPIO = 76,
    #[doc = "77 - PWDG"]
    PWDG = 77,
    #[doc = "78 - PTMR"]
    PTMR = 78,
    #[doc = "79 - PUART"]
    PUART = 79,
    #[doc = "80 - FUSE"]
    FUSE = 80,
    #[doc = "81 - SECMON"]
    SECMON = 81,
    #[doc = "82 - RTC"]
    RTC = 82,
    #[doc = "83 - BUTN"]
    BUTN = 83,
    #[doc = "84 - BGPIO"]
    BGPIO = 84,
    #[doc = "85 - BVIO"]
    BVIO = 85,
    #[doc = "86 - BROWNOUT"]
    BROWNOUT = 86,
    #[doc = "87 - SYSCTL"]
    SYSCTL = 87,
    #[doc = "88 - DEBUG_0"]
    DEBUG_0 = 88,
    #[doc = "89 - DEBUG_1"]
    DEBUG_1 = 89,
    #[doc = "90 - LIN0"]
    LIN0 = 90,
    #[doc = "91 - LIN1"]
    LIN1 = 91,
    #[doc = "92 - LIN2"]
    LIN2 = 92,
    #[doc = "93 - LIN3"]
    LIN3 = 93,
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
        fn GPIO0_C();
        fn GPIO0_D();
        fn GPIO0_X();
        fn GPIO0_Y();
        fn GPIO0_Z();
        fn GPIO1_A();
        fn GPIO1_B();
        fn GPIO1_C();
        fn GPIO1_D();
        fn GPIO1_X();
        fn GPIO1_Y();
        fn GPIO1_Z();
        fn ADC0();
        fn ADC1();
        fn ADC2();
        fn SDFM();
        fn DAC0();
        fn DAC1();
        fn ACMP_0();
        fn ACMP_1();
        fn ACMP_2();
        fn ACMP_3();
        fn SPI0();
        fn SPI1();
        fn SPI2();
        fn SPI3();
        fn UART0();
        fn UART1();
        fn UART2();
        fn UART3();
        fn UART4();
        fn UART5();
        fn UART6();
        fn UART7();
        fn MCAN0();
        fn MCAN1();
        fn MCAN2();
        fn MCAN3();
        fn PTPC();
        fn WDG0();
        fn WDG1();
        fn TSNS();
        fn MBX0A();
        fn MBX0B();
        fn MBX1A();
        fn MBX1B();
        fn GPTMR0();
        fn GPTMR1();
        fn GPTMR2();
        fn GPTMR3();
        fn I2C0();
        fn I2C1();
        fn I2C2();
        fn I2C3();
        fn PWM0();
        fn HALL0();
        fn QEI0();
        fn PWM1();
        fn HALL1();
        fn QEI1();
        fn PWM2();
        fn HALL2();
        fn QEI2();
        fn PWM3();
        fn HALL3();
        fn QEI3();
        fn SDP();
        fn XPI0();
        fn XDMA();
        fn HDMA();
        fn RNG();
        fn USB0();
        fn PSEC();
        fn PGPIO();
        fn PWDG();
        fn PTMR();
        fn PUART();
        fn FUSE();
        fn SECMON();
        fn RTC();
        fn BUTN();
        fn BGPIO();
        fn BVIO();
        fn BROWNOUT();
        fn SYSCTL();
        fn DEBUG_0();
        fn DEBUG_1();
        fn LIN0();
        fn LIN1();
        fn LIN2();
        fn LIN3();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __VECTORED_INTERRUPTS: [Vector; 94] = [
        Vector {
            _handler: CORE_LOCAL,
        },
        Vector { _handler: GPIO0_A },
        Vector { _handler: GPIO0_B },
        Vector { _handler: GPIO0_C },
        Vector { _handler: GPIO0_D },
        Vector { _handler: GPIO0_X },
        Vector { _handler: GPIO0_Y },
        Vector { _handler: GPIO0_Z },
        Vector { _handler: GPIO1_A },
        Vector { _handler: GPIO1_B },
        Vector { _handler: GPIO1_C },
        Vector { _handler: GPIO1_D },
        Vector { _handler: GPIO1_X },
        Vector { _handler: GPIO1_Y },
        Vector { _handler: GPIO1_Z },
        Vector { _handler: ADC0 },
        Vector { _handler: ADC1 },
        Vector { _handler: ADC2 },
        Vector { _handler: SDFM },
        Vector { _handler: DAC0 },
        Vector { _handler: DAC1 },
        Vector { _handler: ACMP_0 },
        Vector { _handler: ACMP_1 },
        Vector { _handler: ACMP_2 },
        Vector { _handler: ACMP_3 },
        Vector { _handler: SPI0 },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: SPI3 },
        Vector { _handler: UART0 },
        Vector { _handler: UART1 },
        Vector { _handler: UART2 },
        Vector { _handler: UART3 },
        Vector { _handler: UART4 },
        Vector { _handler: UART5 },
        Vector { _handler: UART6 },
        Vector { _handler: UART7 },
        Vector { _handler: MCAN0 },
        Vector { _handler: MCAN1 },
        Vector { _handler: MCAN2 },
        Vector { _handler: MCAN3 },
        Vector { _handler: PTPC },
        Vector { _handler: WDG0 },
        Vector { _handler: WDG1 },
        Vector { _handler: TSNS },
        Vector { _handler: MBX0A },
        Vector { _handler: MBX0B },
        Vector { _handler: MBX1A },
        Vector { _handler: MBX1B },
        Vector { _handler: GPTMR0 },
        Vector { _handler: GPTMR1 },
        Vector { _handler: GPTMR2 },
        Vector { _handler: GPTMR3 },
        Vector { _handler: I2C0 },
        Vector { _handler: I2C1 },
        Vector { _handler: I2C2 },
        Vector { _handler: I2C3 },
        Vector { _handler: PWM0 },
        Vector { _handler: HALL0 },
        Vector { _handler: QEI0 },
        Vector { _handler: PWM1 },
        Vector { _handler: HALL1 },
        Vector { _handler: QEI1 },
        Vector { _handler: PWM2 },
        Vector { _handler: HALL2 },
        Vector { _handler: QEI2 },
        Vector { _handler: PWM3 },
        Vector { _handler: HALL3 },
        Vector { _handler: QEI3 },
        Vector { _handler: SDP },
        Vector { _handler: XPI0 },
        Vector { _handler: XDMA },
        Vector { _handler: HDMA },
        Vector { _handler: RNG },
        Vector { _handler: USB0 },
        Vector { _handler: PSEC },
        Vector { _handler: PGPIO },
        Vector { _handler: PWDG },
        Vector { _handler: PTMR },
        Vector { _handler: PUART },
        Vector { _handler: FUSE },
        Vector { _handler: SECMON },
        Vector { _handler: RTC },
        Vector { _handler: BUTN },
        Vector { _handler: BGPIO },
        Vector { _handler: BVIO },
        Vector { _handler: BROWNOUT },
        Vector { _handler: SYSCTL },
        Vector { _handler: DEBUG_0 },
        Vector { _handler: DEBUG_1 },
        Vector { _handler: LIN0 },
        Vector { _handler: LIN1 },
        Vector { _handler: LIN2 },
        Vector { _handler: LIN3 },
    ];
}
pub const PLIC: plic::Plic = unsafe { plic::Plic::from_ptr(0xe400_0000usize as _) };
pub const MCHTMR: mchtmr::Mchtmr = unsafe { mchtmr::Mchtmr::from_ptr(0xe600_0000usize as _) };
pub const PLICSW: plicsw::Plicsw = unsafe { plicsw::Plicsw::from_ptr(0xe640_0000usize as _) };
pub const XPI0: xpi::Xpi = unsafe { xpi::Xpi::from_ptr(0xf304_0000usize as _) };
pub const SYSCTL: sysctl::Sysctl = unsafe { sysctl::Sysctl::from_ptr(0xf400_0000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../../peripherals/plic_common.rs"]
pub mod plic;
#[path = "../../peripherals/plicsw_common.rs"]
pub mod plicsw;
#[path = "../../peripherals/sysctl_v62.rs"]
pub mod sysctl;
#[path = "../../peripherals/xpi_dummy.rs"]
pub mod xpi;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 2147483648;
pub const FLASH_SIZE: usize = 1048576;
pub mod resources {
    pub const RNG0: usize = 317;
    pub const CLK_SRC_CLK2_PLL0: usize = 36;
    pub const ADC1: usize = 275;
    pub const RST_CPU0: usize = 24;
    pub const CLK_SRC_CLK0_PLL0: usize = 34;
    pub const CLK_SRC_PLL2_REF: usize = 45;
    pub const CLK_SRC_CLK1_PLL1: usize = 39;
    pub const LMM0: usize = 259;
    pub const CPU0: usize = 0;
    pub const CLK_TOP_URT0: usize = 72;
    pub const CLK_TOP_ANA4: usize = 97;
    pub const CLK_TOP_REF1: usize = 99;
    pub const I2C1: usize = 266;
    pub const CLK_TOP_CPU0: usize = 64;
    pub const CLK_TOP_LIN2: usize = 102;
    pub const CLK_TOP_ADC0: usize = 128;
    pub const XPI0: usize = 312;
    pub const CLK_TOP_PTPC: usize = 92;
    pub const CLK_TOP_LIN0: usize = 100;
    pub const CLK_SRC_PLL2: usize = 40;
    pub const URT3: usize = 288;
    pub const CRC0: usize = 306;
    pub const WDG1: usize = 303;
    pub const CLK_TOP_CAN0: usize = 88;
    pub const LIN1: usize = 294;
    pub const CLK_TOP_LIN1: usize = 101;
    pub const CLK_TOP_XPI0: usize = 67;
    pub const ADC2: usize = 276;
    pub const CLK_TOP_I2C0: usize = 80;
    pub const CLK_TOP_I2C1: usize = 81;
    pub const MBX0: usize = 304;
    pub const CLK_TOP_I2C3: usize = 83;
    pub const LIN3: usize = 296;
    pub const ACMP: usize = 279;
    pub const URT4: usize = 289;
    pub const I2C0: usize = 265;
    pub const XDMA: usize = 314;
    pub const CLK_TOP_CAN2: usize = 90;
    pub const CLK_TOP_TMR3: usize = 71;
    pub const SPI3: usize = 283;
    pub const MOT2: usize = 309;
    pub const CLK_TOP_CAN1: usize = 89;
    pub const CPU1: usize = 8;
    pub const TMR3: usize = 272;
    pub const CAN2: usize = 300;
    pub const CPX0: usize = 1;
    pub const I2C2: usize = 267;
    pub const CLK_TOP_MCT1: usize = 66;
    pub const URT0: usize = 285;
    pub const RST_CPU1: usize = 25;
    pub const CLK_TOP_ADC1: usize = 129;
    pub const REF1: usize = 321;
    pub const CLK_TOP_CAN3: usize = 91;
    pub const CLK_SRC_CLK1_PLL2: usize = 42;
    pub const CLK_TOP_URT5: usize = 77;
    pub const TMR2: usize = 271;
    pub const MSYN: usize = 311;
    pub const CLK_SRC_XTAL: usize = 32;
    pub const CLK_TOP_TMR1: usize = 69;
    pub const CLK_TOP_URT3: usize = 75;
    pub const CLK_TOP_DAC0: usize = 131;
    pub const DAC0: usize = 277;
    pub const TSNS: usize = 318;
    pub const CLK_TOP_ANA2: usize = 95;
    pub const SDP0: usize = 316;
    pub const CLK_TOP_ADC2: usize = 130;
    pub const AXIC: usize = 258;
    pub const CLK_TOP_URT7: usize = 79;
    pub const CLK_SRC_PLL0: usize = 33;
    pub const CAN3: usize = 301;
    pub const MOT3: usize = 310;
    pub const CLK_SRC_CLK0_PLL2: usize = 41;
    pub const URT5: usize = 290;
    pub const CLK_TOP_SPI2: usize = 86;
    pub const I2C3: usize = 268;
    pub const POW_CPU0: usize = 21;
    pub const CLK_TOP_TMR2: usize = 70;
    pub const CLK_TOP_URT4: usize = 76;
    pub const URT6: usize = 291;
    pub const ROM0: usize = 263;
    pub const CLK_TOP_I2C2: usize = 82;
    pub const GPIO: usize = 273;
    pub const CLK_TOP_URT2: usize = 74;
    pub const POW_CPU1: usize = 22;
    pub const SPI2: usize = 282;
    pub const TMR0: usize = 269;
    pub const MCT1: usize = 262;
    pub const CLK_TOP_MCT0: usize = 65;
    pub const CLK_TOP_URT6: usize = 78;
    pub const CLK_TOP_URT1: usize = 73;
    pub const AHBP: usize = 256;
    pub const ADC0: usize = 274;
    pub const PTPC: usize = 297;
    pub const LIN0: usize = 293;
    pub const CLK_TOP_ANA0: usize = 93;
    pub const CLK_TOP_TMR0: usize = 68;
    pub const CLK_TOP_REF0: usize = 98;
    pub const CLK_TOP_SPI1: usize = 85;
    pub const CLK_SRC_PLL0_REF: usize = 43;
    pub const CLK_TOP_LIN3: usize = 103;
    pub const CLK_TOP_ANA3: usize = 96;
    pub const CLK_SRC_PLL1_REF: usize = 44;
    pub const DAC1: usize = 278;
    pub const SDM0: usize = 284;
    pub const USB0: usize = 319;
    pub const REF0: usize = 320;
    pub const RAM0: usize = 264;
    pub const MBX1: usize = 305;
    pub const MOT0: usize = 307;
    pub const CAN0: usize = 298;
    pub const CLK_SRC_CLK1_PLL0: usize = 35;
    pub const SPI1: usize = 281;
    pub const RST_SOC: usize = 23;
    pub const SPI0: usize = 280;
    pub const URT1: usize = 286;
    pub const URT2: usize = 287;
    pub const KMAN: usize = 315;
    pub const CLK_TOP_SPI3: usize = 87;
    pub const CPX1: usize = 9;
    pub const CAN1: usize = 299;
    pub const CLK_TOP_DAC1: usize = 132;
    pub const URT7: usize = 292;
    pub const MOT1: usize = 308;
    pub const CLK_SRC_PLL1: usize = 37;
    pub const TMR1: usize = 270;
    pub const HDMA: usize = 313;
    pub const MCT0: usize = 260;
    pub const CLK_TOP_SPI0: usize = 84;
    pub const CLK_TOP_ANA1: usize = 94;
    pub const CLK_SRC_CLK0_PLL1: usize = 38;
    pub const LMM1: usize = 261;
    pub const AXIS: usize = 257;
    pub const LIN2: usize = 295;
    pub const WDG0: usize = 302;
}
pub mod clocks {
    pub const CAN0: usize = 23;
    pub const TMR0: usize = 3;
    pub const LIN1: usize = 36;
    pub const LIN2: usize = 37;
    pub const I2C0: usize = 15;
    pub const REF1: usize = 34;
    pub const TMR2: usize = 5;
    pub const URT2: usize = 9;
    pub const CAN2: usize = 25;
    pub const URT5: usize = 12;
    pub const CAN1: usize = 24;
    pub const LIN3: usize = 38;
    pub const URT0: usize = 7;
    pub const MCT1: usize = 1;
    pub const TMR1: usize = 4;
    pub const I2C3: usize = 18;
    pub const URT1: usize = 8;
    pub const SPI2: usize = 21;
    pub const I2C2: usize = 17;
    pub const SPI1: usize = 20;
    pub const ANA1: usize = 29;
    pub const CAN3: usize = 26;
    pub const ANA2: usize = 30;
    pub const ANA3: usize = 31;
    pub const URT7: usize = 14;
    pub const URT4: usize = 11;
    pub const TMR3: usize = 6;
    pub const SPI0: usize = 19;
    pub const ANA0: usize = 28;
    pub const URT3: usize = 10;
    pub const REF0: usize = 33;
    pub const PTPC: usize = 27;
    pub const LIN0: usize = 35;
    pub const ANA4: usize = 32;
    pub const SPI3: usize = 22;
    pub const MCT0: usize = 0;
    pub const URT6: usize = 13;
    pub const XPI0: usize = 2;
    pub const I2C1: usize = 16;
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
    pub const PB16: usize = 48;
    pub const PB17: usize = 49;
    pub const PB18: usize = 50;
    pub const PB19: usize = 51;
    pub const PB20: usize = 52;
    pub const PB21: usize = 53;
    pub const PB22: usize = 54;
    pub const PB23: usize = 55;
    pub const PB24: usize = 56;
    pub const PB25: usize = 57;
    pub const PB26: usize = 58;
    pub const PB27: usize = 59;
    pub const PB28: usize = 60;
    pub const PB29: usize = 61;
    pub const PB30: usize = 62;
    pub const PB31: usize = 63;
    pub const PC00: usize = 64;
    pub const PC01: usize = 65;
    pub const PC02: usize = 66;
    pub const PC03: usize = 67;
    pub const PC04: usize = 68;
    pub const PC05: usize = 69;
    pub const PC06: usize = 70;
    pub const PC07: usize = 71;
    pub const PC08: usize = 72;
    pub const PC09: usize = 73;
    pub const PC10: usize = 74;
    pub const PC11: usize = 75;
    pub const PC12: usize = 76;
    pub const PC13: usize = 77;
    pub const PC14: usize = 78;
    pub const PC15: usize = 79;
    pub const PC16: usize = 80;
    pub const PC17: usize = 81;
    pub const PC18: usize = 82;
    pub const PC19: usize = 83;
    pub const PC20: usize = 84;
    pub const PC21: usize = 85;
    pub const PC22: usize = 86;
    pub const PC23: usize = 87;
    pub const PC24: usize = 88;
    pub const PC25: usize = 89;
    pub const PC26: usize = 90;
    pub const PC27: usize = 91;
    pub const PX00: usize = 416;
    pub const PX01: usize = 417;
    pub const PX02: usize = 418;
    pub const PX03: usize = 419;
    pub const PX04: usize = 420;
    pub const PX05: usize = 421;
    pub const PX06: usize = 422;
    pub const PX07: usize = 423;
    pub const PY00: usize = 448;
    pub const PY01: usize = 449;
    pub const PY02: usize = 450;
    pub const PY03: usize = 451;
    pub const PY04: usize = 452;
    pub const PY05: usize = 453;
    pub const PY06: usize = 454;
    pub const PY07: usize = 455;
    pub const PZ00: usize = 480;
    pub const PZ01: usize = 481;
    pub const PZ02: usize = 482;
    pub const PZ03: usize = 483;
    pub const PZ04: usize = 484;
    pub const PZ05: usize = 485;
    pub const PZ06: usize = 486;
    pub const PZ07: usize = 487;
}
