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
    #[doc = "5 - GPIO0_E"]
    GPIO0_E = 5,
    #[doc = "6 - GPIO0_F"]
    GPIO0_F = 6,
    #[doc = "7 - GPIO0_V"]
    GPIO0_V = 7,
    #[doc = "8 - GPIO0_W"]
    GPIO0_W = 8,
    #[doc = "9 - GPIO0_X"]
    GPIO0_X = 9,
    #[doc = "10 - GPIO0_Y"]
    GPIO0_Y = 10,
    #[doc = "11 - GPIO0_Z"]
    GPIO0_Z = 11,
    #[doc = "12 - GPIO1_A"]
    GPIO1_A = 12,
    #[doc = "13 - GPIO1_B"]
    GPIO1_B = 13,
    #[doc = "14 - GPIO1_C"]
    GPIO1_C = 14,
    #[doc = "15 - GPIO1_D"]
    GPIO1_D = 15,
    #[doc = "16 - GPIO1_E"]
    GPIO1_E = 16,
    #[doc = "17 - GPIO1_F"]
    GPIO1_F = 17,
    #[doc = "18 - GPIO1_V"]
    GPIO1_V = 18,
    #[doc = "19 - GPIO1_W"]
    GPIO1_W = 19,
    #[doc = "20 - GPIO1_X"]
    GPIO1_X = 20,
    #[doc = "21 - GPIO1_Y"]
    GPIO1_Y = 21,
    #[doc = "22 - GPIO1_Z"]
    GPIO1_Z = 22,
    #[doc = "23 - GPTMR0"]
    GPTMR0 = 23,
    #[doc = "24 - GPTMR1"]
    GPTMR1 = 24,
    #[doc = "25 - GPTMR2"]
    GPTMR2 = 25,
    #[doc = "26 - GPTMR3"]
    GPTMR3 = 26,
    #[doc = "27 - GPTMR4"]
    GPTMR4 = 27,
    #[doc = "28 - GPTMR5"]
    GPTMR5 = 28,
    #[doc = "29 - GPTMR6"]
    GPTMR6 = 29,
    #[doc = "30 - GPTMR7"]
    GPTMR7 = 30,
    #[doc = "31 - UART0"]
    UART0 = 31,
    #[doc = "32 - UART1"]
    UART1 = 32,
    #[doc = "33 - UART2"]
    UART2 = 33,
    #[doc = "34 - UART3"]
    UART3 = 34,
    #[doc = "35 - UART4"]
    UART4 = 35,
    #[doc = "36 - UART5"]
    UART5 = 36,
    #[doc = "37 - UART6"]
    UART6 = 37,
    #[doc = "38 - UART7"]
    UART7 = 38,
    #[doc = "39 - I2C0"]
    I2C0 = 39,
    #[doc = "40 - I2C1"]
    I2C1 = 40,
    #[doc = "41 - I2C2"]
    I2C2 = 41,
    #[doc = "42 - I2C3"]
    I2C3 = 42,
    #[doc = "43 - SPI0"]
    SPI0 = 43,
    #[doc = "44 - SPI1"]
    SPI1 = 44,
    #[doc = "45 - SPI2"]
    SPI2 = 45,
    #[doc = "46 - SPI3"]
    SPI3 = 46,
    #[doc = "47 - TSNS"]
    TSNS = 47,
    #[doc = "48 - MBX0A"]
    MBX0A = 48,
    #[doc = "49 - MBX0B"]
    MBX0B = 49,
    #[doc = "50 - MBX1A"]
    MBX1A = 50,
    #[doc = "51 - MBX1B"]
    MBX1B = 51,
    #[doc = "52 - EWDG0"]
    EWDG0 = 52,
    #[doc = "53 - EWDG1"]
    EWDG1 = 53,
    #[doc = "54 - EWDG2"]
    EWDG2 = 54,
    #[doc = "55 - EWDG3"]
    EWDG3 = 55,
    #[doc = "56 - HDMA"]
    HDMA = 56,
    #[doc = "57 - LOBS"]
    LOBS = 57,
    #[doc = "58 - ADC0"]
    ADC0 = 58,
    #[doc = "59 - ADC1"]
    ADC1 = 59,
    #[doc = "60 - ADC2"]
    ADC2 = 60,
    #[doc = "61 - ADC3"]
    ADC3 = 61,
    #[doc = "62 - ACMP0_0"]
    ACMP0_0 = 62,
    #[doc = "63 - ACMP0_1"]
    ACMP0_1 = 63,
    #[doc = "64 - ACMP1_0"]
    ACMP1_0 = 64,
    #[doc = "65 - ACMP1_1"]
    ACMP1_1 = 65,
    #[doc = "66 - ACMP2_0"]
    ACMP2_0 = 66,
    #[doc = "67 - ACMP2_1"]
    ACMP2_1 = 67,
    #[doc = "68 - ACMP3_0"]
    ACMP3_0 = 68,
    #[doc = "69 - ACMP3_1"]
    ACMP3_1 = 69,
    #[doc = "70 - I2S0"]
    I2S0 = 70,
    #[doc = "71 - I2S1"]
    I2S1 = 71,
    #[doc = "72 - DAO"]
    DAO = 72,
    #[doc = "73 - PDM"]
    PDM = 73,
    #[doc = "74 - UART8"]
    UART8 = 74,
    #[doc = "75 - UART9"]
    UART9 = 75,
    #[doc = "76 - UART10"]
    UART10 = 76,
    #[doc = "77 - UART11"]
    UART11 = 77,
    #[doc = "78 - UART12"]
    UART12 = 78,
    #[doc = "79 - UART13"]
    UART13 = 79,
    #[doc = "80 - UART14"]
    UART14 = 80,
    #[doc = "81 - UART15"]
    UART15 = 81,
    #[doc = "82 - I2C4"]
    I2C4 = 82,
    #[doc = "83 - I2C5"]
    I2C5 = 83,
    #[doc = "84 - I2C6"]
    I2C6 = 84,
    #[doc = "85 - I2C7"]
    I2C7 = 85,
    #[doc = "86 - SPI4"]
    SPI4 = 86,
    #[doc = "87 - SPI5"]
    SPI5 = 87,
    #[doc = "88 - SPI6"]
    SPI6 = 88,
    #[doc = "89 - SPI7"]
    SPI7 = 89,
    #[doc = "90 - MCAN0"]
    MCAN0 = 90,
    #[doc = "91 - MCAN1"]
    MCAN1 = 91,
    #[doc = "92 - MCAN2"]
    MCAN2 = 92,
    #[doc = "93 - MCAN3"]
    MCAN3 = 93,
    #[doc = "94 - MCAN4"]
    MCAN4 = 94,
    #[doc = "95 - MCAN5"]
    MCAN5 = 95,
    #[doc = "96 - MCAN6"]
    MCAN6 = 96,
    #[doc = "97 - MCAN7"]
    MCAN7 = 97,
    #[doc = "98 - PTPC"]
    PTPC = 98,
    #[doc = "99 - QEI0"]
    QEI0 = 99,
    #[doc = "100 - QEI1"]
    QEI1 = 100,
    #[doc = "101 - QEI2"]
    QEI2 = 101,
    #[doc = "102 - QEI3"]
    QEI3 = 102,
    #[doc = "103 - PWM0"]
    PWM0 = 103,
    #[doc = "104 - PWM1"]
    PWM1 = 104,
    #[doc = "105 - PWM2"]
    PWM2 = 105,
    #[doc = "106 - PWM3"]
    PWM3 = 106,
    #[doc = "107 - RDC0"]
    RDC0 = 107,
    #[doc = "108 - RDC1"]
    RDC1 = 108,
    #[doc = "109 - SDM0"]
    SDM0 = 109,
    #[doc = "110 - SDM1"]
    SDM1 = 110,
    #[doc = "111 - SEI0"]
    SEI0 = 111,
    #[doc = "112 - SEI1"]
    SEI1 = 112,
    #[doc = "113 - SEI2"]
    SEI2 = 113,
    #[doc = "114 - SEI3"]
    SEI3 = 114,
    #[doc = "115 - MTG0"]
    MTG0 = 115,
    #[doc = "116 - MTG1"]
    MTG1 = 116,
    #[doc = "117 - VSC0"]
    VSC0 = 117,
    #[doc = "118 - VSC1"]
    VSC1 = 118,
    #[doc = "119 - CLC0_0"]
    CLC0_0 = 119,
    #[doc = "120 - CLC0_1"]
    CLC0_1 = 120,
    #[doc = "121 - CLC1_0"]
    CLC1_0 = 121,
    #[doc = "122 - CLC1_1"]
    CLC1_1 = 122,
    #[doc = "123 - TRGMUX0"]
    TRGMUX0 = 123,
    #[doc = "124 - TRGMUX1"]
    TRGMUX1 = 124,
    #[doc = "125 - ENET0"]
    ENET0 = 125,
    #[doc = "126 - NTMR0"]
    NTMR0 = 126,
    #[doc = "127 - USB0"]
    USB0 = 127,
    #[doc = "128 - TSW0"]
    TSW0 = 128,
    #[doc = "129 - TSW1"]
    TSW1 = 129,
    #[doc = "130 - TSW2"]
    TSW2 = 130,
    #[doc = "131 - TSW3"]
    TSW3 = 131,
    #[doc = "132 - TSW_PTP_EVT"]
    TSW_PTP_EVT = 132,
    #[doc = "133 - ESC"]
    ESC = 133,
    #[doc = "134 - ESC_SYNC0"]
    ESC_SYNC0 = 134,
    #[doc = "135 - ESC_SYNC1"]
    ESC_SYNC1 = 135,
    #[doc = "136 - ESC_RESET"]
    ESC_RESET = 136,
    #[doc = "137 - XPI0"]
    XPI0 = 137,
    #[doc = "138 - FEMC"]
    FEMC = 138,
    #[doc = "139 - PPI"]
    PPI = 139,
    #[doc = "140 - XDMA"]
    XDMA = 140,
    #[doc = "141 - FFA"]
    FFA = 141,
    #[doc = "142 - SDP"]
    SDP = 142,
    #[doc = "143 - RNG"]
    RNG = 143,
    #[doc = "144 - PKA"]
    PKA = 144,
    #[doc = "145 - PSEC"]
    PSEC = 145,
    #[doc = "146 - PGPIO"]
    PGPIO = 146,
    #[doc = "147 - PEWDG"]
    PEWDG = 147,
    #[doc = "148 - PTMR"]
    PTMR = 148,
    #[doc = "149 - PUART"]
    PUART = 149,
    #[doc = "150 - FUSE"]
    FUSE = 150,
    #[doc = "151 - SECMON"]
    SECMON = 151,
    #[doc = "152 - RTC"]
    RTC = 152,
    #[doc = "153 - PAD_WAKEUP"]
    PAD_WAKEUP = 153,
    #[doc = "154 - BGPIO"]
    BGPIO = 154,
    #[doc = "155 - BVIO"]
    BVIO = 155,
    #[doc = "156 - BROWNOUT"]
    BROWNOUT = 156,
    #[doc = "157 - SYSCTL"]
    SYSCTL = 157,
    #[doc = "158 - CPU0"]
    CPU0 = 158,
    #[doc = "159 - CPU1"]
    CPU1 = 159,
    #[doc = "160 - DEBUG0"]
    DEBUG0 = 160,
    #[doc = "161 - DEBUG1"]
    DEBUG1 = 161,
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
        fn GPIO0_E();
        fn GPIO0_F();
        fn GPIO0_V();
        fn GPIO0_W();
        fn GPIO0_X();
        fn GPIO0_Y();
        fn GPIO0_Z();
        fn GPIO1_A();
        fn GPIO1_B();
        fn GPIO1_C();
        fn GPIO1_D();
        fn GPIO1_E();
        fn GPIO1_F();
        fn GPIO1_V();
        fn GPIO1_W();
        fn GPIO1_X();
        fn GPIO1_Y();
        fn GPIO1_Z();
        fn GPTMR0();
        fn GPTMR1();
        fn GPTMR2();
        fn GPTMR3();
        fn GPTMR4();
        fn GPTMR5();
        fn GPTMR6();
        fn GPTMR7();
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
        fn MBX1A();
        fn MBX1B();
        fn EWDG0();
        fn EWDG1();
        fn EWDG2();
        fn EWDG3();
        fn HDMA();
        fn LOBS();
        fn ADC0();
        fn ADC1();
        fn ADC2();
        fn ADC3();
        fn ACMP0_0();
        fn ACMP0_1();
        fn ACMP1_0();
        fn ACMP1_1();
        fn ACMP2_0();
        fn ACMP2_1();
        fn ACMP3_0();
        fn ACMP3_1();
        fn I2S0();
        fn I2S1();
        fn DAO();
        fn PDM();
        fn UART8();
        fn UART9();
        fn UART10();
        fn UART11();
        fn UART12();
        fn UART13();
        fn UART14();
        fn UART15();
        fn I2C4();
        fn I2C5();
        fn I2C6();
        fn I2C7();
        fn SPI4();
        fn SPI5();
        fn SPI6();
        fn SPI7();
        fn MCAN0();
        fn MCAN1();
        fn MCAN2();
        fn MCAN3();
        fn MCAN4();
        fn MCAN5();
        fn MCAN6();
        fn MCAN7();
        fn PTPC();
        fn QEI0();
        fn QEI1();
        fn QEI2();
        fn QEI3();
        fn PWM0();
        fn PWM1();
        fn PWM2();
        fn PWM3();
        fn RDC0();
        fn RDC1();
        fn SDM0();
        fn SDM1();
        fn SEI0();
        fn SEI1();
        fn SEI2();
        fn SEI3();
        fn MTG0();
        fn MTG1();
        fn VSC0();
        fn VSC1();
        fn CLC0_0();
        fn CLC0_1();
        fn CLC1_0();
        fn CLC1_1();
        fn TRGMUX0();
        fn TRGMUX1();
        fn ENET0();
        fn NTMR0();
        fn USB0();
        fn TSW0();
        fn TSW1();
        fn TSW2();
        fn TSW3();
        fn TSW_PTP_EVT();
        fn ESC();
        fn ESC_SYNC0();
        fn ESC_SYNC1();
        fn ESC_RESET();
        fn XPI0();
        fn FEMC();
        fn PPI();
        fn XDMA();
        fn FFA();
        fn SDP();
        fn RNG();
        fn PKA();
        fn PSEC();
        fn PGPIO();
        fn PEWDG();
        fn PTMR();
        fn PUART();
        fn FUSE();
        fn SECMON();
        fn RTC();
        fn PAD_WAKEUP();
        fn BGPIO();
        fn BVIO();
        fn BROWNOUT();
        fn SYSCTL();
        fn CPU0();
        fn CPU1();
        fn DEBUG0();
        fn DEBUG1();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __VECTORED_INTERRUPTS: [Vector; 162] = [
        Vector {
            _handler: CORE_LOCAL,
        },
        Vector { _handler: GPIO0_A },
        Vector { _handler: GPIO0_B },
        Vector { _handler: GPIO0_C },
        Vector { _handler: GPIO0_D },
        Vector { _handler: GPIO0_E },
        Vector { _handler: GPIO0_F },
        Vector { _handler: GPIO0_V },
        Vector { _handler: GPIO0_W },
        Vector { _handler: GPIO0_X },
        Vector { _handler: GPIO0_Y },
        Vector { _handler: GPIO0_Z },
        Vector { _handler: GPIO1_A },
        Vector { _handler: GPIO1_B },
        Vector { _handler: GPIO1_C },
        Vector { _handler: GPIO1_D },
        Vector { _handler: GPIO1_E },
        Vector { _handler: GPIO1_F },
        Vector { _handler: GPIO1_V },
        Vector { _handler: GPIO1_W },
        Vector { _handler: GPIO1_X },
        Vector { _handler: GPIO1_Y },
        Vector { _handler: GPIO1_Z },
        Vector { _handler: GPTMR0 },
        Vector { _handler: GPTMR1 },
        Vector { _handler: GPTMR2 },
        Vector { _handler: GPTMR3 },
        Vector { _handler: GPTMR4 },
        Vector { _handler: GPTMR5 },
        Vector { _handler: GPTMR6 },
        Vector { _handler: GPTMR7 },
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
        Vector { _handler: MBX1A },
        Vector { _handler: MBX1B },
        Vector { _handler: EWDG0 },
        Vector { _handler: EWDG1 },
        Vector { _handler: EWDG2 },
        Vector { _handler: EWDG3 },
        Vector { _handler: HDMA },
        Vector { _handler: LOBS },
        Vector { _handler: ADC0 },
        Vector { _handler: ADC1 },
        Vector { _handler: ADC2 },
        Vector { _handler: ADC3 },
        Vector { _handler: ACMP0_0 },
        Vector { _handler: ACMP0_1 },
        Vector { _handler: ACMP1_0 },
        Vector { _handler: ACMP1_1 },
        Vector { _handler: ACMP2_0 },
        Vector { _handler: ACMP2_1 },
        Vector { _handler: ACMP3_0 },
        Vector { _handler: ACMP3_1 },
        Vector { _handler: I2S0 },
        Vector { _handler: I2S1 },
        Vector { _handler: DAO },
        Vector { _handler: PDM },
        Vector { _handler: UART8 },
        Vector { _handler: UART9 },
        Vector { _handler: UART10 },
        Vector { _handler: UART11 },
        Vector { _handler: UART12 },
        Vector { _handler: UART13 },
        Vector { _handler: UART14 },
        Vector { _handler: UART15 },
        Vector { _handler: I2C4 },
        Vector { _handler: I2C5 },
        Vector { _handler: I2C6 },
        Vector { _handler: I2C7 },
        Vector { _handler: SPI4 },
        Vector { _handler: SPI5 },
        Vector { _handler: SPI6 },
        Vector { _handler: SPI7 },
        Vector { _handler: MCAN0 },
        Vector { _handler: MCAN1 },
        Vector { _handler: MCAN2 },
        Vector { _handler: MCAN3 },
        Vector { _handler: MCAN4 },
        Vector { _handler: MCAN5 },
        Vector { _handler: MCAN6 },
        Vector { _handler: MCAN7 },
        Vector { _handler: PTPC },
        Vector { _handler: QEI0 },
        Vector { _handler: QEI1 },
        Vector { _handler: QEI2 },
        Vector { _handler: QEI3 },
        Vector { _handler: PWM0 },
        Vector { _handler: PWM1 },
        Vector { _handler: PWM2 },
        Vector { _handler: PWM3 },
        Vector { _handler: RDC0 },
        Vector { _handler: RDC1 },
        Vector { _handler: SDM0 },
        Vector { _handler: SDM1 },
        Vector { _handler: SEI0 },
        Vector { _handler: SEI1 },
        Vector { _handler: SEI2 },
        Vector { _handler: SEI3 },
        Vector { _handler: MTG0 },
        Vector { _handler: MTG1 },
        Vector { _handler: VSC0 },
        Vector { _handler: VSC1 },
        Vector { _handler: CLC0_0 },
        Vector { _handler: CLC0_1 },
        Vector { _handler: CLC1_0 },
        Vector { _handler: CLC1_1 },
        Vector { _handler: TRGMUX0 },
        Vector { _handler: TRGMUX1 },
        Vector { _handler: ENET0 },
        Vector { _handler: NTMR0 },
        Vector { _handler: USB0 },
        Vector { _handler: TSW0 },
        Vector { _handler: TSW1 },
        Vector { _handler: TSW2 },
        Vector { _handler: TSW3 },
        Vector {
            _handler: TSW_PTP_EVT,
        },
        Vector { _handler: ESC },
        Vector {
            _handler: ESC_SYNC0,
        },
        Vector {
            _handler: ESC_SYNC1,
        },
        Vector {
            _handler: ESC_RESET,
        },
        Vector { _handler: XPI0 },
        Vector { _handler: FEMC },
        Vector { _handler: PPI },
        Vector { _handler: XDMA },
        Vector { _handler: FFA },
        Vector { _handler: SDP },
        Vector { _handler: RNG },
        Vector { _handler: PKA },
        Vector { _handler: PSEC },
        Vector { _handler: PGPIO },
        Vector { _handler: PEWDG },
        Vector { _handler: PTMR },
        Vector { _handler: PUART },
        Vector { _handler: FUSE },
        Vector { _handler: SECMON },
        Vector { _handler: RTC },
        Vector {
            _handler: PAD_WAKEUP,
        },
        Vector { _handler: BGPIO },
        Vector { _handler: BVIO },
        Vector { _handler: BROWNOUT },
        Vector { _handler: SYSCTL },
        Vector { _handler: CPU0 },
        Vector { _handler: CPU1 },
        Vector { _handler: DEBUG0 },
        Vector { _handler: DEBUG1 },
    ];
}
pub const FGPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x0030_0000usize as _) };
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
pub const TSNS: tsns::Tsns = unsafe { tsns::Tsns::from_ptr(0xf009_0000usize as _) };
pub const MBX0A: mbx::Mbx = unsafe { mbx::Mbx::from_ptr(0xf00a_0000usize as _) };
pub const MBX0B: mbx::Mbx = unsafe { mbx::Mbx::from_ptr(0xf00a_4000usize as _) };
pub const MBX1A: mbx::Mbx = unsafe { mbx::Mbx::from_ptr(0xf00a_8000usize as _) };
pub const MBX1B: mbx::Mbx = unsafe { mbx::Mbx::from_ptr(0xf00a_c000usize as _) };
pub const WDG0: wdg::Wdg = unsafe { wdg::Wdg::from_ptr(0xf00b_0000usize as _) };
pub const WDG1: wdg::Wdg = unsafe { wdg::Wdg::from_ptr(0xf00b_4000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0xf00c_0000usize as _) };
pub const DMAMUX: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0xf00c_4000usize as _) };
pub const HDMA: dma::Dma = unsafe { dma::Dma::from_ptr(0xf00c_8000usize as _) };
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf00d_0000usize as _) };
pub const GPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf00d_4000usize as _) };
pub const GPIOM: gpiom::Gpiom = unsafe { gpiom::Gpiom::from_ptr(0xf00d_8000usize as _) };
pub const LOBS: lobs::Lobs = unsafe { lobs::Lobs::from_ptr(0xf00d_c000usize as _) };
pub const ADC0: adc16::Adc = unsafe { adc16::Adc::from_ptr(0xf010_0000usize as _) };
pub const ADC1: adc16::Adc = unsafe { adc16::Adc::from_ptr(0xf010_4000usize as _) };
pub const ADC2: adc16::Adc = unsafe { adc16::Adc::from_ptr(0xf010_8000usize as _) };
pub const ACMP0: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0xf013_0000usize as _) };
pub const ACMP1: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0xf013_4000usize as _) };
pub const ACMP2: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0xf013_8000usize as _) };
pub const ACMP3: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0xf013_c000usize as _) };
pub const I2S0: i2s::I2s = unsafe { i2s::I2s::from_ptr(0xf014_0000usize as _) };
pub const I2S1: i2s::I2s = unsafe { i2s::I2s::from_ptr(0xf014_4000usize as _) };
pub const PDM: pdm::Pdm = unsafe { pdm::Pdm::from_ptr(0xf015_4000usize as _) };
pub const DAO: dao::Dao = unsafe { dao::Dao::from_ptr(0xf021_0000usize as _) };
pub const MCAN0: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf030_0000usize as _) };
pub const MCAN1: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf030_4000usize as _) };
pub const MCAN2: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf030_8000usize as _) };
pub const MCAN3: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf030_c000usize as _) };
pub const SYNT: synt::Synt = unsafe { synt::Synt::from_ptr(0xf032_8000usize as _) };
pub const PTPC: ptpc::Ptpc = unsafe { ptpc::Ptpc::from_ptr(0xf037_c000usize as _) };
pub const QEI0: qei::Qei = unsafe { qei::Qei::from_ptr(0xf040_0000usize as _) };
pub const QEI1: qei::Qei = unsafe { qei::Qei::from_ptr(0xf040_4000usize as _) };
pub const QEO0: qeo::Qeo = unsafe { qeo::Qeo::from_ptr(0xf041_0000usize as _) };
pub const QEO1: qeo::Qeo = unsafe { qeo::Qeo::from_ptr(0xf041_4000usize as _) };
pub const PWM0: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0xf042_0000usize as _) };
pub const PWM1: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0xf042_4000usize as _) };
pub const PWM2: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0xf042_8000usize as _) };
pub const PWM3: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0xf042_c000usize as _) };
pub const RDC0: rdc::Rdc = unsafe { rdc::Rdc::from_ptr(0xf044_0000usize as _) };
pub const SDM0: sdm::Sdm = unsafe { sdm::Sdm::from_ptr(0xf045_0000usize as _) };
pub const PLB: plb::Plb = unsafe { plb::Plb::from_ptr(0xf046_0000usize as _) };
pub const SEI: sei::Sei = unsafe { sei::Sei::from_ptr(0xf047_0000usize as _) };
pub const TRGM0: trgm::Trgm = unsafe { trgm::Trgm::from_ptr(0xf047_c000usize as _) };
pub const MTG0: mtg::Mtg = unsafe { mtg::Mtg::from_ptr(0xf049_0000usize as _) };
pub const VSC0: vsc::Vsc = unsafe { vsc::Vsc::from_ptr(0xf04a_0000usize as _) };
pub const CLC0: clc::Clc = unsafe { clc::Clc::from_ptr(0xf04b_0000usize as _) };
pub const USB0: usb::Usb = unsafe { usb::Usb::from_ptr(0xf112_0000usize as _) };
pub const ENET0: enet::Enet = unsafe { enet::Enet::from_ptr(0xf140_0000usize as _) };
pub const ENET1: enet::Enet = unsafe { enet::Enet::from_ptr(0xf140_4000usize as _) };
pub const ESC: esc::Esc = unsafe { esc::Esc::from_ptr(0xf140_8000usize as _) };
pub const NTMR0: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf141_0000usize as _) };
pub const NTMR1: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf141_4000usize as _) };
pub const XPI0: xpi::Xpi = unsafe { xpi::Xpi::from_ptr(0xf300_0000usize as _) };
pub const XPI1: xpi::Xpi = unsafe { xpi::Xpi::from_ptr(0xf300_4000usize as _) };
pub const FEMC: femc::Femc = unsafe { femc::Femc::from_ptr(0xf300_c000usize as _) };
pub const PPI: ppi::Ppi = unsafe { ppi::Ppi::from_ptr(0xf301_0000usize as _) };
pub const FFA: ffa::Ffa = unsafe { ffa::Ffa::from_ptr(0xf301_8000usize as _) };
pub const XDMA: dma::Dma = unsafe { dma::Dma::from_ptr(0xf310_0000usize as _) };
pub const SDP: sdp::Sdp = unsafe { sdp::Sdp::from_ptr(0xf314_0000usize as _) };
pub const PSEC: psec::Psec = unsafe { psec::Psec::from_ptr(0xf314_4000usize as _) };
pub const PMON: pmon::Pmon = unsafe { pmon::Pmon::from_ptr(0xf314_8000usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0xf314_c000usize as _) };
pub const KEYM: keym::Keym = unsafe { keym::Keym::from_ptr(0xf315_4000usize as _) };
pub const OTP: otp::Otp = unsafe { otp::Otp::from_ptr(0xf315_8000usize as _) };
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
pub const BPOR: bpor::Bpor = unsafe { bpor::Bpor::from_ptr(0xf420_4000usize as _) };
pub const BCFG: bcfg::Bcfg = unsafe { bcfg::Bcfg::from_ptr(0xf420_8000usize as _) };
pub const BIOC: ioc::Ioc = unsafe { ioc::Ioc::from_ptr(0xf421_0000usize as _) };
pub const BGPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf421_4000usize as _) };
pub const BSEC: bsec::Bsec = unsafe { bsec::Bsec::from_ptr(0xf424_0000usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0xf424_4000usize as _) };
pub const BKEY: bkey::Bkey = unsafe { bkey::Bkey::from_ptr(0xf424_8000usize as _) };
pub const BMON: bmon::Bmon = unsafe { bmon::Bmon::from_ptr(0xf424_c000usize as _) };
pub const TAMP: tamp::Tamp = unsafe { tamp::Tamp::from_ptr(0xf425_0000usize as _) };
pub const MONO: mono::Mono = unsafe { mono::Mono::from_ptr(0xf425_4000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/acmp_v6e.rs"]
pub mod acmp;
#[path = "../../peripherals/adc16_v6e.rs"]
pub mod adc16;
#[path = "../../peripherals/bcfg_v68.rs"]
pub mod bcfg;
#[path = "../../peripherals/bkey_common.rs"]
pub mod bkey;
#[path = "../../peripherals/bmon_common.rs"]
pub mod bmon;
#[path = "../../peripherals/bpor_v68.rs"]
pub mod bpor;
#[path = "../../peripherals/bsec_common.rs"]
pub mod bsec;
#[path = "../../peripherals/clc_v6e.rs"]
pub mod clc;
#[path = "../../peripherals/crc_common.rs"]
pub mod crc;
#[path = "../../peripherals/dao_v68.rs"]
pub mod dao;
#[path = "../../peripherals/dma_v6e.rs"]
pub mod dma;
#[path = "../../peripherals/dmamux_common.rs"]
pub mod dmamux;
#[path = "../../peripherals/enet_v68.rs"]
pub mod enet;
#[path = "../../peripherals/esc_v6e.rs"]
pub mod esc;
#[path = "../../peripherals/femc_v6e.rs"]
pub mod femc;
#[path = "../../peripherals/ffa_v6e.rs"]
pub mod ffa;
#[path = "../../peripherals/gpio_v53.rs"]
pub mod gpio;
#[path = "../../peripherals/gpiom_v67.rs"]
pub mod gpiom;
#[path = "../../peripherals/i2c_v53.rs"]
pub mod i2c;
#[path = "../../peripherals/i2s_common.rs"]
pub mod i2s;
#[path = "../../peripherals/ioc_common.rs"]
pub mod ioc;
#[path = "../../peripherals/keym_common.rs"]
pub mod keym;
#[path = "../../peripherals/lobs_v6e.rs"]
pub mod lobs;
#[path = "../../peripherals/mbx_common.rs"]
pub mod mbx;
#[path = "../../peripherals/mcan_v53.rs"]
pub mod mcan;
#[path = "../../peripherals/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../../peripherals/mono_common.rs"]
pub mod mono;
#[path = "../../peripherals/mtg_v6e.rs"]
pub mod mtg;
#[path = "../../peripherals/otp_common.rs"]
pub mod otp;
#[path = "../../peripherals/pcfg_v6e.rs"]
pub mod pcfg;
#[path = "../../peripherals/pdm_common.rs"]
pub mod pdm;
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
#[path = "../../peripherals/ppi_v6e.rs"]
pub mod ppi;
#[path = "../../peripherals/ppor_v6e.rs"]
pub mod ppor;
#[path = "../../peripherals/psec_common.rs"]
pub mod psec;
#[path = "../../peripherals/ptpc_common.rs"]
pub mod ptpc;
#[path = "../../peripherals/pwm_v53.rs"]
pub mod pwm;
#[path = "../../peripherals/qei_v6e.rs"]
pub mod qei;
#[path = "../../peripherals/qeo_v6e.rs"]
pub mod qeo;
#[path = "../../peripherals/rdc_v6e.rs"]
pub mod rdc;
#[path = "../../peripherals/rng_common.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_common.rs"]
pub mod rtc;
#[path = "../../peripherals/sdm_v6e.rs"]
pub mod sdm;
#[path = "../../peripherals/sdp_v53.rs"]
pub mod sdp;
#[path = "../../peripherals/sei_v6e.rs"]
pub mod sei;
#[path = "../../peripherals/spi_v53.rs"]
pub mod spi;
#[path = "../../peripherals/synt_v53.rs"]
pub mod synt;
#[path = "../../peripherals/sysctl_v6e.rs"]
pub mod sysctl;
#[path = "../../peripherals/tamp_v62.rs"]
pub mod tamp;
#[path = "../../peripherals/tmr_common.rs"]
pub mod tmr;
#[path = "../../peripherals/trgm_v53.rs"]
pub mod trgm;
#[path = "../../peripherals/tsns_common.rs"]
pub mod tsns;
#[path = "../../peripherals/uart_v53.rs"]
pub mod uart;
#[path = "../../peripherals/usb_v53.rs"]
pub mod usb;
#[path = "../../peripherals/vsc_v6e.rs"]
pub mod vsc;
#[path = "../../peripherals/wdg_v68.rs"]
pub mod wdg;
#[path = "../../peripherals/xpi_dummy.rs"]
pub mod xpi;
pub const FLASH_BASE: usize = 2147483648;
pub const FLASH_SIZE: usize = 1048576;
pub mod resources {}
pub mod clocks {}
pub mod pins {}
