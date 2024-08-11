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
    #[doc = "7 - GPIO0_X"]
    GPIO0_X = 7,
    #[doc = "8 - GPIO0_Y"]
    GPIO0_Y = 8,
    #[doc = "9 - GPIO0_Z"]
    GPIO0_Z = 9,
    #[doc = "10 - GPIO1_A"]
    GPIO1_A = 10,
    #[doc = "11 - GPIO1_B"]
    GPIO1_B = 11,
    #[doc = "12 - GPIO1_C"]
    GPIO1_C = 12,
    #[doc = "13 - GPIO1_D"]
    GPIO1_D = 13,
    #[doc = "14 - GPIO1_E"]
    GPIO1_E = 14,
    #[doc = "15 - GPIO1_F"]
    GPIO1_F = 15,
    #[doc = "16 - GPIO1_X"]
    GPIO1_X = 16,
    #[doc = "17 - GPIO1_Y"]
    GPIO1_Y = 17,
    #[doc = "18 - GPIO1_Z"]
    GPIO1_Z = 18,
    #[doc = "19 - ADC0"]
    ADC0 = 19,
    #[doc = "20 - ADC1"]
    ADC1 = 20,
    #[doc = "21 - ADC2"]
    ADC2 = 21,
    #[doc = "22 - ADC3"]
    ADC3 = 22,
    #[doc = "23 - ACMP_0"]
    ACMP_0 = 23,
    #[doc = "24 - ACMP_1"]
    ACMP_1 = 24,
    #[doc = "25 - ACMP_2"]
    ACMP_2 = 25,
    #[doc = "26 - ACMP_3"]
    ACMP_3 = 26,
    #[doc = "27 - SPI0"]
    SPI0 = 27,
    #[doc = "28 - SPI1"]
    SPI1 = 28,
    #[doc = "29 - SPI2"]
    SPI2 = 29,
    #[doc = "30 - SPI3"]
    SPI3 = 30,
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
    #[doc = "39 - UART8"]
    UART8 = 39,
    #[doc = "40 - UART9"]
    UART9 = 40,
    #[doc = "41 - UART10"]
    UART10 = 41,
    #[doc = "42 - UART11"]
    UART11 = 42,
    #[doc = "43 - UART12"]
    UART12 = 43,
    #[doc = "44 - UART13"]
    UART13 = 44,
    #[doc = "45 - UART14"]
    UART14 = 45,
    #[doc = "46 - UART15"]
    UART15 = 46,
    #[doc = "47 - CAN0"]
    CAN0 = 47,
    #[doc = "48 - CAN1"]
    CAN1 = 48,
    #[doc = "49 - CAN2"]
    CAN2 = 49,
    #[doc = "50 - CAN3"]
    CAN3 = 50,
    #[doc = "51 - PTPC"]
    PTPC = 51,
    #[doc = "52 - WDG0"]
    WDG0 = 52,
    #[doc = "53 - WDG1"]
    WDG1 = 53,
    #[doc = "54 - WDG2"]
    WDG2 = 54,
    #[doc = "55 - WDG3"]
    WDG3 = 55,
    #[doc = "56 - MBX0A"]
    MBX0A = 56,
    #[doc = "57 - MBX0B"]
    MBX0B = 57,
    #[doc = "58 - MBX1A"]
    MBX1A = 58,
    #[doc = "59 - MBX1B"]
    MBX1B = 59,
    #[doc = "60 - GPTMR0"]
    GPTMR0 = 60,
    #[doc = "61 - GPTMR1"]
    GPTMR1 = 61,
    #[doc = "62 - GPTMR2"]
    GPTMR2 = 62,
    #[doc = "63 - GPTMR3"]
    GPTMR3 = 63,
    #[doc = "64 - GPTMR4"]
    GPTMR4 = 64,
    #[doc = "65 - GPTMR5"]
    GPTMR5 = 65,
    #[doc = "66 - GPTMR6"]
    GPTMR6 = 66,
    #[doc = "67 - GPTMR7"]
    GPTMR7 = 67,
    #[doc = "68 - I2C0"]
    I2C0 = 68,
    #[doc = "69 - I2C1"]
    I2C1 = 69,
    #[doc = "70 - I2C2"]
    I2C2 = 70,
    #[doc = "71 - I2C3"]
    I2C3 = 71,
    #[doc = "72 - PWM0"]
    PWM0 = 72,
    #[doc = "73 - HALL0"]
    HALL0 = 73,
    #[doc = "74 - QEI0"]
    QEI0 = 74,
    #[doc = "75 - PWM1"]
    PWM1 = 75,
    #[doc = "76 - HALL1"]
    HALL1 = 76,
    #[doc = "77 - QEI1"]
    QEI1 = 77,
    #[doc = "78 - PWM2"]
    PWM2 = 78,
    #[doc = "79 - HALL2"]
    HALL2 = 79,
    #[doc = "80 - QEI2"]
    QEI2 = 80,
    #[doc = "81 - PWM3"]
    PWM3 = 81,
    #[doc = "82 - HALL3"]
    HALL3 = 82,
    #[doc = "83 - QEI3"]
    QEI3 = 83,
    #[doc = "84 - SDP"]
    SDP = 84,
    #[doc = "85 - XPI0"]
    XPI0 = 85,
    #[doc = "86 - XPI1"]
    XPI1 = 86,
    #[doc = "87 - XDMA"]
    XDMA = 87,
    #[doc = "88 - HDMA"]
    HDMA = 88,
    #[doc = "89 - FEMC"]
    FEMC = 89,
    #[doc = "90 - RNG"]
    RNG = 90,
    #[doc = "91 - I2S0"]
    I2S0 = 91,
    #[doc = "92 - I2S1"]
    I2S1 = 92,
    #[doc = "93 - I2S2"]
    I2S2 = 93,
    #[doc = "94 - I2S3"]
    I2S3 = 94,
    #[doc = "95 - DAO"]
    DAO = 95,
    #[doc = "96 - PDM"]
    PDM = 96,
    #[doc = "97 - CAM0"]
    CAM0 = 97,
    #[doc = "98 - CAM1"]
    CAM1 = 98,
    #[doc = "99 - LCDC_D0"]
    LCDC_D0 = 99,
    #[doc = "100 - LCDC_D1"]
    LCDC_D1 = 100,
    #[doc = "101 - PDMA_D0"]
    PDMA_D0 = 101,
    #[doc = "102 - PDMA_D1"]
    PDMA_D1 = 102,
    #[doc = "103 - JPEG"]
    JPEG = 103,
    #[doc = "104 - NTMR0"]
    NTMR0 = 104,
    #[doc = "105 - NTMR1"]
    NTMR1 = 105,
    #[doc = "106 - USB0"]
    USB0 = 106,
    #[doc = "107 - USB1"]
    USB1 = 107,
    #[doc = "108 - ENET0"]
    ENET0 = 108,
    #[doc = "109 - ENET1"]
    ENET1 = 109,
    #[doc = "110 - SDXC0"]
    SDXC0 = 110,
    #[doc = "111 - SDXC1"]
    SDXC1 = 111,
    #[doc = "112 - PSEC"]
    PSEC = 112,
    #[doc = "113 - PGPIO"]
    PGPIO = 113,
    #[doc = "114 - PWDG"]
    PWDG = 114,
    #[doc = "115 - PTMR"]
    PTMR = 115,
    #[doc = "116 - PUART"]
    PUART = 116,
    #[doc = "117 - VAD"]
    VAD = 117,
    #[doc = "118 - FUSE"]
    FUSE = 118,
    #[doc = "119 - SECMON"]
    SECMON = 119,
    #[doc = "120 - RTC"]
    RTC = 120,
    #[doc = "121 - BUTN"]
    BUTN = 121,
    #[doc = "122 - BGPIO"]
    BGPIO = 122,
    #[doc = "123 - BVIO"]
    BVIO = 123,
    #[doc = "124 - BROWNOUT"]
    BROWNOUT = 124,
    #[doc = "125 - SYSCTL"]
    SYSCTL = 125,
    #[doc = "126 - DEBUG_0"]
    DEBUG_0 = 126,
    #[doc = "127 - DEBUG_1"]
    DEBUG_1 = 127,
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
        fn GPIO0_X();
        fn GPIO0_Y();
        fn GPIO0_Z();
        fn GPIO1_A();
        fn GPIO1_B();
        fn GPIO1_C();
        fn GPIO1_D();
        fn GPIO1_E();
        fn GPIO1_F();
        fn GPIO1_X();
        fn GPIO1_Y();
        fn GPIO1_Z();
        fn ADC0();
        fn ADC1();
        fn ADC2();
        fn ADC3();
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
        fn UART8();
        fn UART9();
        fn UART10();
        fn UART11();
        fn UART12();
        fn UART13();
        fn UART14();
        fn UART15();
        fn CAN0();
        fn CAN1();
        fn CAN2();
        fn CAN3();
        fn PTPC();
        fn WDG0();
        fn WDG1();
        fn WDG2();
        fn WDG3();
        fn MBX0A();
        fn MBX0B();
        fn MBX1A();
        fn MBX1B();
        fn GPTMR0();
        fn GPTMR1();
        fn GPTMR2();
        fn GPTMR3();
        fn GPTMR4();
        fn GPTMR5();
        fn GPTMR6();
        fn GPTMR7();
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
        fn XPI1();
        fn XDMA();
        fn HDMA();
        fn FEMC();
        fn RNG();
        fn I2S0();
        fn I2S1();
        fn I2S2();
        fn I2S3();
        fn DAO();
        fn PDM();
        fn CAM0();
        fn CAM1();
        fn LCDC_D0();
        fn LCDC_D1();
        fn PDMA_D0();
        fn PDMA_D1();
        fn JPEG();
        fn NTMR0();
        fn NTMR1();
        fn USB0();
        fn USB1();
        fn ENET0();
        fn ENET1();
        fn SDXC0();
        fn SDXC1();
        fn PSEC();
        fn PGPIO();
        fn PWDG();
        fn PTMR();
        fn PUART();
        fn VAD();
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
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __VECTORED_INTERRUPTS: [Vector; 128] = [
        Vector {
            _handler: CORE_LOCAL,
        },
        Vector { _handler: GPIO0_A },
        Vector { _handler: GPIO0_B },
        Vector { _handler: GPIO0_C },
        Vector { _handler: GPIO0_D },
        Vector { _handler: GPIO0_E },
        Vector { _handler: GPIO0_F },
        Vector { _handler: GPIO0_X },
        Vector { _handler: GPIO0_Y },
        Vector { _handler: GPIO0_Z },
        Vector { _handler: GPIO1_A },
        Vector { _handler: GPIO1_B },
        Vector { _handler: GPIO1_C },
        Vector { _handler: GPIO1_D },
        Vector { _handler: GPIO1_E },
        Vector { _handler: GPIO1_F },
        Vector { _handler: GPIO1_X },
        Vector { _handler: GPIO1_Y },
        Vector { _handler: GPIO1_Z },
        Vector { _handler: ADC0 },
        Vector { _handler: ADC1 },
        Vector { _handler: ADC2 },
        Vector { _handler: ADC3 },
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
        Vector { _handler: UART8 },
        Vector { _handler: UART9 },
        Vector { _handler: UART10 },
        Vector { _handler: UART11 },
        Vector { _handler: UART12 },
        Vector { _handler: UART13 },
        Vector { _handler: UART14 },
        Vector { _handler: UART15 },
        Vector { _handler: CAN0 },
        Vector { _handler: CAN1 },
        Vector { _handler: CAN2 },
        Vector { _handler: CAN3 },
        Vector { _handler: PTPC },
        Vector { _handler: WDG0 },
        Vector { _handler: WDG1 },
        Vector { _handler: WDG2 },
        Vector { _handler: WDG3 },
        Vector { _handler: MBX0A },
        Vector { _handler: MBX0B },
        Vector { _handler: MBX1A },
        Vector { _handler: MBX1B },
        Vector { _handler: GPTMR0 },
        Vector { _handler: GPTMR1 },
        Vector { _handler: GPTMR2 },
        Vector { _handler: GPTMR3 },
        Vector { _handler: GPTMR4 },
        Vector { _handler: GPTMR5 },
        Vector { _handler: GPTMR6 },
        Vector { _handler: GPTMR7 },
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
        Vector { _handler: XPI1 },
        Vector { _handler: XDMA },
        Vector { _handler: HDMA },
        Vector { _handler: FEMC },
        Vector { _handler: RNG },
        Vector { _handler: I2S0 },
        Vector { _handler: I2S1 },
        Vector { _handler: I2S2 },
        Vector { _handler: I2S3 },
        Vector { _handler: DAO },
        Vector { _handler: PDM },
        Vector { _handler: CAM0 },
        Vector { _handler: CAM1 },
        Vector { _handler: LCDC_D0 },
        Vector { _handler: LCDC_D1 },
        Vector { _handler: PDMA_D0 },
        Vector { _handler: PDMA_D1 },
        Vector { _handler: JPEG },
        Vector { _handler: NTMR0 },
        Vector { _handler: NTMR1 },
        Vector { _handler: USB0 },
        Vector { _handler: USB1 },
        Vector { _handler: ENET0 },
        Vector { _handler: ENET1 },
        Vector { _handler: SDXC0 },
        Vector { _handler: SDXC1 },
        Vector { _handler: PSEC },
        Vector { _handler: PGPIO },
        Vector { _handler: PWDG },
        Vector { _handler: PTMR },
        Vector { _handler: PUART },
        Vector { _handler: VAD },
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
    ];
}
pub const FGPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x000c_0000usize as _) };
pub const PLIC: plic::Plic = unsafe { plic::Plic::from_ptr(0xe400_0000usize as _) };
pub const MCHTMR: mchtmr::Mchtmr = unsafe { mchtmr::Mchtmr::from_ptr(0xe600_0000usize as _) };
pub const PLICSW: plicsw::Plicsw = unsafe { plicsw::Plicsw::from_ptr(0xe640_0000usize as _) };
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf000_0000usize as _) };
pub const GPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf000_4000usize as _) };
pub const GPIOM: gpiom::Gpiom = unsafe { gpiom::Gpiom::from_ptr(0xf000_8000usize as _) };
pub const FEMC: femc::Femc = unsafe { femc::Femc::from_ptr(0xf000_8000usize as _) };
pub const ADC0: adc12::Adc = unsafe { adc12::Adc::from_ptr(0xf001_0000usize as _) };
pub const ADC1: adc12::Adc = unsafe { adc12::Adc::from_ptr(0xf001_4000usize as _) };
pub const ADC2: adc12::Adc = unsafe { adc12::Adc::from_ptr(0xf001_8000usize as _) };
pub const ADC3: adc16::Adc = unsafe { adc16::Adc::from_ptr(0xf001_c000usize as _) };
pub const ACMP: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0xf002_0000usize as _) };
pub const SPI0: spi::Spi = unsafe { spi::Spi::from_ptr(0xf003_0000usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0xf003_4000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0xf003_8000usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0xf003_c000usize as _) };
pub const UART0: uart::Uart = unsafe { uart::Uart::from_ptr(0xf004_0000usize as _) };
pub const UART1: uart::Uart = unsafe { uart::Uart::from_ptr(0xf004_4000usize as _) };
pub const UART2: uart::Uart = unsafe { uart::Uart::from_ptr(0xf004_8000usize as _) };
pub const UART3: uart::Uart = unsafe { uart::Uart::from_ptr(0xf004_c000usize as _) };
pub const UART4: uart::Uart = unsafe { uart::Uart::from_ptr(0xf005_0000usize as _) };
pub const UART5: uart::Uart = unsafe { uart::Uart::from_ptr(0xf005_4000usize as _) };
pub const UART6: uart::Uart = unsafe { uart::Uart::from_ptr(0xf005_8000usize as _) };
pub const UART7: uart::Uart = unsafe { uart::Uart::from_ptr(0xf005_c000usize as _) };
pub const UART8: uart::Uart = unsafe { uart::Uart::from_ptr(0xf006_0000usize as _) };
pub const UART9: uart::Uart = unsafe { uart::Uart::from_ptr(0xf006_4000usize as _) };
pub const UART10: uart::Uart = unsafe { uart::Uart::from_ptr(0xf006_8000usize as _) };
pub const UART11: uart::Uart = unsafe { uart::Uart::from_ptr(0xf006_c000usize as _) };
pub const UART12: uart::Uart = unsafe { uart::Uart::from_ptr(0xf007_0000usize as _) };
pub const UART13: uart::Uart = unsafe { uart::Uart::from_ptr(0xf007_4000usize as _) };
pub const UART14: uart::Uart = unsafe { uart::Uart::from_ptr(0xf007_8000usize as _) };
pub const UART15: uart::Uart = unsafe { uart::Uart::from_ptr(0xf007_c000usize as _) };
pub const CAN0: can::Can = unsafe { can::Can::from_ptr(0xf008_0000usize as _) };
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0xf008_4000usize as _) };
pub const WDG0: wdg::Wdg = unsafe { wdg::Wdg::from_ptr(0xf009_0000usize as _) };
pub const WDG1: wdg::Wdg = unsafe { wdg::Wdg::from_ptr(0xf009_4000usize as _) };
pub const WDG2: wdg::Wdg = unsafe { wdg::Wdg::from_ptr(0xf009_8000usize as _) };
pub const WDG3: wdg::Wdg = unsafe { wdg::Wdg::from_ptr(0xf009_c000usize as _) };
pub const MBX0A: mbx::Mbx = unsafe { mbx::Mbx::from_ptr(0xf00a_0000usize as _) };
pub const MBX0B: mbx::Mbx = unsafe { mbx::Mbx::from_ptr(0xf00a_4000usize as _) };
pub const MBX1A: mbx::Mbx = unsafe { mbx::Mbx::from_ptr(0xf00a_8000usize as _) };
pub const MBX1B: mbx::Mbx = unsafe { mbx::Mbx::from_ptr(0xf00a_c000usize as _) };
pub const PTPC: ptpc::Ptpc = unsafe { ptpc::Ptpc::from_ptr(0xf00b_0000usize as _) };
pub const DMAMUX: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0xf00c_0000usize as _) };
pub const HDMA: dma::Dma = unsafe { dma::Dma::from_ptr(0xf00c_4000usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0xf00c_8000usize as _) };
pub const KEYM: keym::Keym = unsafe { keym::Keym::from_ptr(0xf00c_c000usize as _) };
pub const ENET0: enet::Enet = unsafe { enet::Enet::from_ptr(0xf00d_0000usize as _) };
pub const ENET1: enet::Enet = unsafe { enet::Enet::from_ptr(0xf00d_4000usize as _) };
pub const I2S0: i2s::I2s = unsafe { i2s::I2s::from_ptr(0xf010_0000usize as _) };
pub const I2S1: i2s::I2s = unsafe { i2s::I2s::from_ptr(0xf010_4000usize as _) };
pub const I2S2: i2s::I2s = unsafe { i2s::I2s::from_ptr(0xf010_8000usize as _) };
pub const I2S3: i2s::I2s = unsafe { i2s::I2s::from_ptr(0xf010_c000usize as _) };
pub const DAO: dao::Dao = unsafe { dao::Dao::from_ptr(0xf011_0000usize as _) };
pub const PDM: pdm::Pdm = unsafe { pdm::Pdm::from_ptr(0xf011_4000usize as _) };
pub const PWM0: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0xf020_0000usize as _) };
pub const HALL0: hall::Hall = unsafe { hall::Hall::from_ptr(0xf020_4000usize as _) };
pub const QEI0: qei::Qei = unsafe { qei::Qei::from_ptr(0xf020_8000usize as _) };
pub const TRGM0: trgm::Trgm = unsafe { trgm::Trgm::from_ptr(0xf020_c000usize as _) };
pub const PWM1: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0xf021_0000usize as _) };
pub const HALL1: hall::Hall = unsafe { hall::Hall::from_ptr(0xf021_4000usize as _) };
pub const QEI1: qei::Qei = unsafe { qei::Qei::from_ptr(0xf021_8000usize as _) };
pub const TRGM1: trgm::Trgm = unsafe { trgm::Trgm::from_ptr(0xf021_c000usize as _) };
pub const PWM2: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0xf022_0000usize as _) };
pub const HALL2: hall::Hall = unsafe { hall::Hall::from_ptr(0xf022_4000usize as _) };
pub const QEI2: qei::Qei = unsafe { qei::Qei::from_ptr(0xf022_8000usize as _) };
pub const TRGM2: trgm::Trgm = unsafe { trgm::Trgm::from_ptr(0xf022_c000usize as _) };
pub const PWM3: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0xf023_0000usize as _) };
pub const HALL3: hall::Hall = unsafe { hall::Hall::from_ptr(0xf023_4000usize as _) };
pub const QEI3: qei::Qei = unsafe { qei::Qei::from_ptr(0xf023_8000usize as _) };
pub const TRGM3: trgm::Trgm = unsafe { trgm::Trgm::from_ptr(0xf023_c000usize as _) };
pub const SYNT: synt::Synt = unsafe { synt::Synt::from_ptr(0xf024_0000usize as _) };
pub const LCDC: lcdc::Lcdc = unsafe { lcdc::Lcdc::from_ptr(0xf100_0000usize as _) };
pub const CAM0: cam::Cam = unsafe { cam::Cam::from_ptr(0xf100_8000usize as _) };
pub const CAM1: cam::Cam = unsafe { cam::Cam::from_ptr(0xf100_c000usize as _) };
pub const PDMA: pdma::Pdma = unsafe { pdma::Pdma::from_ptr(0xf101_0000usize as _) };
pub const JPEG: jpeg::Jpeg = unsafe { jpeg::Jpeg::from_ptr(0xf101_4000usize as _) };
pub const NTMR0: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf201_0000usize as _) };
pub const NTMR1: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf201_4000usize as _) };
pub const USB0: usb::Usb = unsafe { usb::Usb::from_ptr(0xf202_0000usize as _) };
pub const USB1: usb::Usb = unsafe { usb::Usb::from_ptr(0xf202_4000usize as _) };
pub const SDXC0: sdxc::Sdxc = unsafe { sdxc::Sdxc::from_ptr(0xf203_0000usize as _) };
pub const SDXC1: sdxc::Sdxc = unsafe { sdxc::Sdxc::from_ptr(0xf203_4000usize as _) };
pub const CONCTL: conctl::Conctl = unsafe { conctl::Conctl::from_ptr(0xf204_0000usize as _) };
pub const GPTMR0: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf300_0000usize as _) };
pub const GPTMR1: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf300_4000usize as _) };
pub const GPTMR2: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf300_8000usize as _) };
pub const GPTMR3: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf300_c000usize as _) };
pub const GPTMR4: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf301_0000usize as _) };
pub const GPTMR5: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf301_4000usize as _) };
pub const GPTMR6: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf301_8000usize as _) };
pub const GPTMR7: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf301_c000usize as _) };
pub const I2C0: i2c::I2c = unsafe { i2c::I2c::from_ptr(0xf302_0000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0xf302_4000usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0xf302_8000usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0xf302_c000usize as _) };
pub const XPI0: xpi::Xpi = unsafe { xpi::Xpi::from_ptr(0xf304_0000usize as _) };
pub const XPI1: xpi::Xpi = unsafe { xpi::Xpi::from_ptr(0xf304_4000usize as _) };
pub const XDMA: dma::Dma = unsafe { dma::Dma::from_ptr(0xf304_8000usize as _) };
pub const SDP: sdp::Sdp = unsafe { sdp::Sdp::from_ptr(0xf304_c000usize as _) };
pub const SYSCTL: sysctl::Sysctl = unsafe { sysctl::Sysctl::from_ptr(0xf400_0000usize as _) };
pub const IOC: ioc::Ioc = unsafe { ioc::Ioc::from_ptr(0xf404_0000usize as _) };
pub const OTPSHW: otp::Otp = unsafe { otp::Otp::from_ptr(0xf408_0000usize as _) };
pub const PPOR: ppor::Ppor = unsafe { ppor::Ppor::from_ptr(0xf40c_0000usize as _) };
pub const PCFG: pcfg::Pcfg = unsafe { pcfg::Pcfg::from_ptr(0xf40c_4000usize as _) };
pub const OTP: otp::Otp = unsafe { otp::Otp::from_ptr(0xf40c_8000usize as _) };
pub const PSEC: psec::Psec = unsafe { psec::Psec::from_ptr(0xf40c_c000usize as _) };
pub const PMON: pmon::Pmon = unsafe { pmon::Pmon::from_ptr(0xf40d_0000usize as _) };
pub const PIOC: ioc::Ioc = unsafe { ioc::Ioc::from_ptr(0xf40d_8000usize as _) };
pub const PGPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf40d_c000usize as _) };
pub const PTMR: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf40e_0000usize as _) };
pub const PUART: uart::Uart = unsafe { uart::Uart::from_ptr(0xf40e_4000usize as _) };
pub const PWDG: wdg::Wdg = unsafe { wdg::Wdg::from_ptr(0xf40e_8000usize as _) };
pub const VAD: vad::Vad = unsafe { vad::Vad::from_ptr(0xf40e_c000usize as _) };
pub const PLLCTL: pllctl::Pllctl = unsafe { pllctl::Pllctl::from_ptr(0xf410_0000usize as _) };
pub const BPOR: bpor::Bpor = unsafe { bpor::Bpor::from_ptr(0xf500_4000usize as _) };
pub const BCFG: bcfg::Bcfg = unsafe { bcfg::Bcfg::from_ptr(0xf500_8000usize as _) };
pub const BUTN: butn::Butn = unsafe { butn::Butn::from_ptr(0xf500_c000usize as _) };
pub const BIOC: ioc::Ioc = unsafe { ioc::Ioc::from_ptr(0xf501_0000usize as _) };
pub const BGPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf501_4000usize as _) };
pub const RTCSHW: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0xf501_c000usize as _) };
pub const BSEC: bsec::Bsec = unsafe { bsec::Bsec::from_ptr(0xf504_0000usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0xf504_4000usize as _) };
pub const BKEY: bkey::Bkey = unsafe { bkey::Bkey::from_ptr(0xf504_8000usize as _) };
pub const BMON: bmon::Bmon = unsafe { bmon::Bmon::from_ptr(0xf504_c000usize as _) };
pub const TAMP: tamp::Tamp = unsafe { tamp::Tamp::from_ptr(0xf505_0000usize as _) };
pub const MONO: mono::Mono = unsafe { mono::Mono::from_ptr(0xf505_4000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/acmp_common.rs"]
pub mod acmp;
#[path = "../../peripherals/adc12_v67.rs"]
pub mod adc12;
#[path = "../../peripherals/adc16_v67.rs"]
pub mod adc16;
#[path = "../../peripherals/bcfg_v67.rs"]
pub mod bcfg;
#[path = "../../peripherals/bkey_common.rs"]
pub mod bkey;
#[path = "../../peripherals/bmon_common.rs"]
pub mod bmon;
#[path = "../../peripherals/bpor_v67.rs"]
pub mod bpor;
#[path = "../../peripherals/bsec_common.rs"]
pub mod bsec;
#[path = "../../peripherals/butn_common.rs"]
pub mod butn;
#[path = "../../peripherals/cam_v67.rs"]
pub mod cam;
#[path = "../../peripherals/can_v67.rs"]
pub mod can;
#[path = "../../peripherals/conctl_v67.rs"]
pub mod conctl;
#[path = "../../peripherals/dao_v67.rs"]
pub mod dao;
#[path = "../../peripherals/dma_v67.rs"]
pub mod dma;
#[path = "../../peripherals/dmamux_common.rs"]
pub mod dmamux;
#[path = "../../peripherals/enet_v67.rs"]
pub mod enet;
#[path = "../../peripherals/femc_common.rs"]
pub mod femc;
#[path = "../../peripherals/gpio_common.rs"]
pub mod gpio;
#[path = "../../peripherals/gpiom_v67.rs"]
pub mod gpiom;
#[path = "../../peripherals/hall_common.rs"]
pub mod hall;
#[path = "../../peripherals/i2c_v67.rs"]
pub mod i2c;
#[path = "../../peripherals/i2s_common.rs"]
pub mod i2s;
#[path = "../../peripherals/ioc_v67.rs"]
pub mod ioc;
#[path = "../../peripherals/jpeg_common.rs"]
pub mod jpeg;
#[path = "../../peripherals/keym_common.rs"]
pub mod keym;
#[path = "../../peripherals/lcdc_v67.rs"]
pub mod lcdc;
#[path = "../../peripherals/mbx_common.rs"]
pub mod mbx;
#[path = "../../peripherals/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../../peripherals/mono_common.rs"]
pub mod mono;
#[path = "../../peripherals/otp_common.rs"]
pub mod otp;
#[path = "../../peripherals/pcfg_v67.rs"]
pub mod pcfg;
#[path = "../../peripherals/pdm_common.rs"]
pub mod pdm;
#[path = "../../peripherals/pdma_v67.rs"]
pub mod pdma;
#[path = "../../peripherals/plic_common.rs"]
pub mod plic;
#[path = "../../peripherals/plicsw_common.rs"]
pub mod plicsw;
#[path = "../../peripherals/pllctl_v67.rs"]
pub mod pllctl;
#[path = "../../peripherals/pmon_common.rs"]
pub mod pmon;
#[path = "../../peripherals/ppor_v67.rs"]
pub mod ppor;
#[path = "../../peripherals/psec_common.rs"]
pub mod psec;
#[path = "../../peripherals/ptpc_common.rs"]
pub mod ptpc;
#[path = "../../peripherals/pwm_v67.rs"]
pub mod pwm;
#[path = "../../peripherals/qei_v67.rs"]
pub mod qei;
#[path = "../../peripherals/rng_common.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_common.rs"]
pub mod rtc;
#[path = "../../peripherals/sdp_v67.rs"]
pub mod sdp;
#[path = "../../peripherals/sdxc_v67.rs"]
pub mod sdxc;
#[path = "../../peripherals/spi_v67.rs"]
pub mod spi;
#[path = "../../peripherals/synt_v67.rs"]
pub mod synt;
#[path = "../../peripherals/sysctl_v67.rs"]
pub mod sysctl;
#[path = "../../peripherals/tamp_v67.rs"]
pub mod tamp;
#[path = "../../peripherals/tmr_common.rs"]
pub mod tmr;
#[path = "../../peripherals/trgm_v67.rs"]
pub mod trgm;
#[path = "../../peripherals/uart_v67.rs"]
pub mod uart;
#[path = "../../peripherals/usb_v67.rs"]
pub mod usb;
#[path = "../../peripherals/vad_common.rs"]
pub mod vad;
#[path = "../../peripherals/wdg_v67.rs"]
pub mod wdg;
#[path = "../../peripherals/xpi_dummy.rs"]
pub mod xpi;
pub const FLASH_BASE: usize = 2147483648;
pub const FLASH_SIZE: usize = 1048576;
pub mod resources {
    //! `SYSCTL.RESOURCE` definitions
    pub const CPU0_CORE: usize = 0;
    pub const CPU0_SUBSYS: usize = 1;
    pub const CPU1_CORE: usize = 8;
    pub const CPX1_SUBSYS: usize = 9;
    pub const POW_CON: usize = 21;
    pub const POW_VIS: usize = 22;
    pub const POW_CPU0: usize = 23;
    pub const POW_CPU1: usize = 24;
    pub const RST_SOC: usize = 25;
    pub const RST_CON: usize = 26;
    pub const RST_VIS: usize = 27;
    pub const RST_CPU0: usize = 28;
    pub const RST_CPU1: usize = 29;
    pub const CLK_SRC_XTAL: usize = 32;
    pub const CLK_SRC_PLL0: usize = 33;
    pub const CLK_SRC_PLL0CLK0: usize = 34;
    pub const CLK_SRC_PLL1: usize = 35;
    pub const CLK_SRC_PLL1CLK0: usize = 36;
    pub const CLK_SRC_PLL1CLK1: usize = 37;
    pub const CLK_SRC_PLL2: usize = 38;
    pub const CLK_SRC_PLL2CLK0: usize = 39;
    pub const CLK_SRC_PLL2CLK1: usize = 40;
    pub const CLK_SRC_PLL3: usize = 41;
    pub const CLK_SRC_PLL3CLK0: usize = 42;
    pub const CLK_SRC_PLL4: usize = 43;
    pub const CLK_SRC_PLL4CLK0: usize = 44;
    pub const CLK_TOP_CPU0: usize = 64;
    pub const CLK_TOP_MCHTMR0: usize = 65;
    pub const CLK_TOP_CPU1: usize = 66;
    pub const CLK_TOP_MCHTMR1: usize = 67;
    pub const CLK_TOP_AXI: usize = 68;
    pub const CLK_TOP_CONN: usize = 69;
    pub const CLK_TOP_VIS: usize = 70;
    pub const CLK_TOP_AHB: usize = 71;
    pub const CLK_TOP_FEMC: usize = 72;
    pub const CLK_TOP_XPI0: usize = 73;
    pub const CLK_TOP_XPI1: usize = 74;
    pub const CLK_TOP_GPTMR0: usize = 75;
    pub const CLK_TOP_GPTMR1: usize = 76;
    pub const CLK_TOP_GPTMR2: usize = 77;
    pub const CLK_TOP_GPTMR3: usize = 78;
    pub const CLK_TOP_GPTMR4: usize = 79;
    pub const CLK_TOP_GPTMR5: usize = 80;
    pub const CLK_TOP_GPTMR6: usize = 81;
    pub const CLK_TOP_GPTMR7: usize = 82;
    pub const CLK_TOP_UART0: usize = 83;
    pub const CLK_TOP_UART1: usize = 84;
    pub const CLK_TOP_UART2: usize = 85;
    pub const CLK_TOP_UART3: usize = 86;
    pub const CLK_TOP_UART4: usize = 87;
    pub const CLK_TOP_UART5: usize = 88;
    pub const CLK_TOP_UART6: usize = 89;
    pub const CLK_TOP_UART7: usize = 90;
    pub const CLK_TOP_UART8: usize = 91;
    pub const CLK_TOP_UART9: usize = 92;
    pub const CLK_TOP_UART10: usize = 93;
    pub const CLK_TOP_UART11: usize = 94;
    pub const CLK_TOP_UART12: usize = 95;
    pub const CLK_TOP_UART13: usize = 96;
    pub const CLK_TOP_UART14: usize = 97;
    pub const CLK_TOP_UART15: usize = 98;
    pub const CLK_TOP_I2C0: usize = 99;
    pub const CLK_TOP_I2C1: usize = 100;
    pub const CLK_TOP_I2C2: usize = 101;
    pub const CLK_TOP_I2C3: usize = 102;
    pub const CLK_TOP_SPI0: usize = 103;
    pub const CLK_TOP_SPI1: usize = 104;
    pub const CLK_TOP_SPI2: usize = 105;
    pub const CLK_TOP_SPI3: usize = 106;
    pub const CLK_TOP_CAN0: usize = 107;
    pub const CLK_TOP_CAN1: usize = 108;
    pub const CLK_TOP_CAN2: usize = 109;
    pub const CLK_TOP_CAN3: usize = 110;
    pub const CLK_TOP_PTPC: usize = 111;
    pub const CLK_TOP_ANA0: usize = 112;
    pub const CLK_TOP_ANA1: usize = 113;
    pub const CLK_TOP_ANA2: usize = 114;
    pub const CLK_TOP_AUD0: usize = 115;
    pub const CLK_TOP_AUD1: usize = 116;
    pub const CLK_TOP_AUD2: usize = 117;
    pub const CLK_TOP_LCDC: usize = 118;
    pub const CLK_TOP_CAM0: usize = 119;
    pub const CLK_TOP_CAM1: usize = 120;
    pub const CLK_TOP_ENET0: usize = 121;
    pub const CLK_TOP_ENET1: usize = 122;
    pub const CLK_TOP_PTP0: usize = 123;
    pub const CLK_TOP_PTP1: usize = 124;
    pub const CLK_TOP_REF0: usize = 125;
    pub const CLK_TOP_REF1: usize = 126;
    pub const CLK_TOP_NTMR0: usize = 127;
    pub const CLK_TOP_NTMR1: usize = 128;
    pub const CLK_TOP_SDXC0: usize = 129;
    pub const CLK_TOP_SDXC1: usize = 130;
    pub const CLK_TOP_ADC0: usize = 192;
    pub const CLK_TOP_ADC1: usize = 193;
    pub const CLK_TOP_ADC2: usize = 194;
    pub const CLK_TOP_ADC3: usize = 195;
    pub const CLK_TOP_I2S0: usize = 196;
    pub const CLK_TOP_I2S1: usize = 197;
    pub const CLK_TOP_I2S2: usize = 198;
    pub const CLK_TOP_I2S3: usize = 199;
    pub const AHBAPB_BUS: usize = 256;
    pub const AXI_BUS: usize = 257;
    pub const CONN_BUS: usize = 258;
    pub const VIS_BUS: usize = 259;
    pub const FEMC: usize = 260;
    pub const ROM: usize = 261;
    pub const LMM0: usize = 262;
    pub const LMM1: usize = 263;
    pub const MCHTMR0: usize = 264;
    pub const MCHTMR1: usize = 265;
    pub const AXI_SRAM0: usize = 266;
    pub const AXI_SRAM1: usize = 267;
    pub const XPI0: usize = 268;
    pub const XPI1: usize = 269;
    pub const SDP: usize = 270;
    pub const RNG: usize = 271;
    pub const KEYM: usize = 272;
    pub const HDMA: usize = 273;
    pub const XDMA: usize = 274;
    pub const GPIO: usize = 275;
    pub const MBX0: usize = 276;
    pub const MBX1: usize = 277;
    pub const WDG0: usize = 278;
    pub const WDG1: usize = 279;
    pub const WDG2: usize = 280;
    pub const WDG3: usize = 281;
    pub const GPTMR0: usize = 282;
    pub const GPTMR1: usize = 283;
    pub const GPTMR2: usize = 284;
    pub const GPTMR3: usize = 285;
    pub const GPTMR4: usize = 286;
    pub const GPTMR5: usize = 287;
    pub const GPTMR6: usize = 288;
    pub const GPTMR7: usize = 289;
    pub const UART0: usize = 290;
    pub const UART1: usize = 291;
    pub const UART2: usize = 292;
    pub const UART3: usize = 293;
    pub const UART4: usize = 294;
    pub const UART5: usize = 295;
    pub const UART6: usize = 296;
    pub const UART7: usize = 297;
    pub const UART8: usize = 298;
    pub const UART9: usize = 299;
    pub const UART10: usize = 300;
    pub const UART11: usize = 301;
    pub const UART12: usize = 302;
    pub const UART13: usize = 303;
    pub const UART14: usize = 304;
    pub const UART15: usize = 305;
    pub const I2C0: usize = 306;
    pub const I2C1: usize = 307;
    pub const I2C2: usize = 308;
    pub const I2C3: usize = 309;
    pub const SPI0: usize = 310;
    pub const SPI1: usize = 311;
    pub const SPI2: usize = 312;
    pub const SPI3: usize = 313;
    pub const CAN0: usize = 314;
    pub const CAN1: usize = 315;
    pub const CAN2: usize = 316;
    pub const CAN3: usize = 317;
    pub const PTPC: usize = 318;
    pub const ADC0: usize = 319;
    pub const ADC1: usize = 320;
    pub const ADC2: usize = 321;
    pub const ADC3: usize = 322;
    pub const ACMP: usize = 323;
    pub const I2S0: usize = 324;
    pub const I2S1: usize = 325;
    pub const I2S2: usize = 326;
    pub const I2S3: usize = 327;
    pub const PDM: usize = 328;
    pub const DAO: usize = 329;
    pub const SYNT: usize = 330;
    pub const MOT0: usize = 331;
    pub const MOT1: usize = 332;
    pub const MOT2: usize = 333;
    pub const MOT3: usize = 334;
    pub const LCDC: usize = 335;
    pub const CAM0: usize = 336;
    pub const CAM1: usize = 337;
    pub const JPEG: usize = 338;
    pub const PDMA: usize = 339;
    pub const ENET0: usize = 340;
    pub const ENET1: usize = 341;
    pub const NTMR0: usize = 342;
    pub const NTMR1: usize = 343;
    pub const SDXC0: usize = 344;
    pub const SDXC1: usize = 345;
    pub const USB0: usize = 346;
    pub const USB1: usize = 347;
    pub const REF0: usize = 348;
    pub const REF1: usize = 349;
}
pub mod clocks {
    //! `SYSCTL.CLOCK` definitions
    pub const CPU0: usize = 0;
    pub const MCHTMR0: usize = 1;
    pub const CPU1: usize = 2;
    pub const MCHTMR1: usize = 3;
    pub const AXI: usize = 4;
    pub const CONN: usize = 5;
    pub const VIS: usize = 6;
    pub const AHB: usize = 7;
    pub const FEMC: usize = 8;
    pub const XPI0: usize = 9;
    pub const XPI1: usize = 10;
    pub const GPTMR0: usize = 11;
    pub const GPTMR1: usize = 12;
    pub const GPTMR2: usize = 13;
    pub const GPTMR3: usize = 14;
    pub const GPTMR4: usize = 15;
    pub const GPTMR5: usize = 16;
    pub const GPTMR6: usize = 17;
    pub const GPTMR7: usize = 18;
    pub const UART0: usize = 19;
    pub const UART1: usize = 20;
    pub const UART2: usize = 21;
    pub const UART3: usize = 22;
    pub const UART4: usize = 23;
    pub const UART5: usize = 24;
    pub const UART6: usize = 25;
    pub const UART7: usize = 26;
    pub const UART8: usize = 27;
    pub const UART9: usize = 28;
    pub const UART10: usize = 29;
    pub const UART11: usize = 30;
    pub const UART12: usize = 31;
    pub const UART13: usize = 32;
    pub const UART14: usize = 33;
    pub const UART15: usize = 34;
    pub const I2C0: usize = 35;
    pub const I2C1: usize = 36;
    pub const I2C2: usize = 37;
    pub const I2C3: usize = 38;
    pub const SPI0: usize = 39;
    pub const SPI1: usize = 40;
    pub const SPI2: usize = 41;
    pub const SPI3: usize = 42;
    pub const CAN0: usize = 43;
    pub const CAN1: usize = 44;
    pub const CAN2: usize = 45;
    pub const CAN3: usize = 46;
    pub const PTPC: usize = 47;
    pub const ANA0: usize = 48;
    pub const ANA1: usize = 49;
    pub const ANA2: usize = 50;
    pub const AUD0: usize = 51;
    pub const AUD1: usize = 52;
    pub const AUD2: usize = 53;
    pub const LCDC: usize = 54;
    pub const CAM0: usize = 55;
    pub const CAM1: usize = 56;
    pub const ENET0: usize = 57;
    pub const ENET1: usize = 58;
    pub const PTP0: usize = 59;
    pub const PTP1: usize = 60;
    pub const REF0: usize = 61;
    pub const REF1: usize = 62;
    pub const NTMR0: usize = 63;
    pub const NTMR1: usize = 64;
    pub const SDXC0: usize = 65;
    pub const SDXC1: usize = 66;
}
pub mod pins {
    //! Pin pad definitions
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
    pub const PC28: usize = 92;
    pub const PC29: usize = 93;
    pub const PC30: usize = 94;
    pub const PC31: usize = 95;
    pub const PD00: usize = 96;
    pub const PD01: usize = 97;
    pub const PD02: usize = 98;
    pub const PD03: usize = 99;
    pub const PD04: usize = 100;
    pub const PD05: usize = 101;
    pub const PD06: usize = 102;
    pub const PD07: usize = 103;
    pub const PD08: usize = 104;
    pub const PD09: usize = 105;
    pub const PD10: usize = 106;
    pub const PD11: usize = 107;
    pub const PD12: usize = 108;
    pub const PD13: usize = 109;
    pub const PD14: usize = 110;
    pub const PD15: usize = 111;
    pub const PD16: usize = 112;
    pub const PD17: usize = 113;
    pub const PD18: usize = 114;
    pub const PD19: usize = 115;
    pub const PD20: usize = 116;
    pub const PD21: usize = 117;
    pub const PD22: usize = 118;
    pub const PD23: usize = 119;
    pub const PD24: usize = 120;
    pub const PD25: usize = 121;
    pub const PD26: usize = 122;
    pub const PD27: usize = 123;
    pub const PD28: usize = 124;
    pub const PD29: usize = 125;
    pub const PD30: usize = 126;
    pub const PD31: usize = 127;
    pub const PE00: usize = 128;
    pub const PE01: usize = 129;
    pub const PE02: usize = 130;
    pub const PE03: usize = 131;
    pub const PE04: usize = 132;
    pub const PE05: usize = 133;
    pub const PE06: usize = 134;
    pub const PE07: usize = 135;
    pub const PE08: usize = 136;
    pub const PE09: usize = 137;
    pub const PE10: usize = 138;
    pub const PE11: usize = 139;
    pub const PE12: usize = 140;
    pub const PE13: usize = 141;
    pub const PE14: usize = 142;
    pub const PE15: usize = 143;
    pub const PE16: usize = 144;
    pub const PE17: usize = 145;
    pub const PE18: usize = 146;
    pub const PE19: usize = 147;
    pub const PE20: usize = 148;
    pub const PE21: usize = 149;
    pub const PE22: usize = 150;
    pub const PE23: usize = 151;
    pub const PE24: usize = 152;
    pub const PE25: usize = 153;
    pub const PE26: usize = 154;
    pub const PE27: usize = 155;
    pub const PE28: usize = 156;
    pub const PE29: usize = 157;
    pub const PE30: usize = 158;
    pub const PE31: usize = 159;
    pub const PF00: usize = 160;
    pub const PF01: usize = 161;
    pub const PF02: usize = 162;
    pub const PF03: usize = 163;
    pub const PF04: usize = 164;
    pub const PF05: usize = 165;
    pub const PF06: usize = 166;
    pub const PF07: usize = 167;
    pub const PF08: usize = 168;
    pub const PF09: usize = 169;
    pub const PF10: usize = 170;
    pub const PX00: usize = 416;
    pub const PX01: usize = 417;
    pub const PX02: usize = 418;
    pub const PX03: usize = 419;
    pub const PX04: usize = 420;
    pub const PX05: usize = 421;
    pub const PX06: usize = 422;
    pub const PX07: usize = 423;
    pub const PX08: usize = 424;
    pub const PX09: usize = 425;
    pub const PX10: usize = 426;
    pub const PX11: usize = 427;
    pub const PY00: usize = 448;
    pub const PY01: usize = 449;
    pub const PY02: usize = 450;
    pub const PY03: usize = 451;
    pub const PY04: usize = 452;
    pub const PY05: usize = 453;
    pub const PY06: usize = 454;
    pub const PY07: usize = 455;
    pub const PY08: usize = 456;
    pub const PY09: usize = 457;
    pub const PY10: usize = 458;
    pub const PY11: usize = 459;
    pub const PZ00: usize = 480;
    pub const PZ01: usize = 481;
    pub const PZ02: usize = 482;
    pub const PZ03: usize = 483;
    pub const PZ04: usize = 484;
    pub const PZ05: usize = 485;
    pub const PZ06: usize = 486;
    pub const PZ07: usize = 487;
    pub const PZ08: usize = 488;
    pub const PZ09: usize = 489;
    pub const PZ10: usize = 490;
    pub const PZ11: usize = 491;
}
pub mod iomux {
    //! `FUNC_CTL` function mux definitions
    pub const BIOC_PZ00_FUNC_CTL_BGPIO_Z_00: u8 = 0;
    pub const BIOC_PZ00_FUNC_CTL_PWR_ON: u8 = 1;
    pub const BIOC_PZ00_FUNC_CTL_SOC_PZ_00: u8 = 3;
    pub const BIOC_PZ00_FUNC_CTL_TAMP_00: u8 = 2;
    pub const BIOC_PZ01_FUNC_CTL_BGPIO_Z_01: u8 = 0;
    pub const BIOC_PZ01_FUNC_CTL_RESETN: u8 = 1;
    pub const BIOC_PZ01_FUNC_CTL_SOC_PZ_01: u8 = 3;
    pub const BIOC_PZ01_FUNC_CTL_TAMP_01: u8 = 2;
    pub const BIOC_PZ02_FUNC_CTL_BGPIO_Z_02: u8 = 0;
    pub const BIOC_PZ02_FUNC_CTL_PBUTN: u8 = 1;
    pub const BIOC_PZ02_FUNC_CTL_SOC_PZ_02: u8 = 3;
    pub const BIOC_PZ02_FUNC_CTL_TAMP_02: u8 = 2;
    pub const BIOC_PZ03_FUNC_CTL_BGPIO_Z_03: u8 = 0;
    pub const BIOC_PZ03_FUNC_CTL_SOC_PZ_03: u8 = 3;
    pub const BIOC_PZ03_FUNC_CTL_TAMP_03: u8 = 2;
    pub const BIOC_PZ03_FUNC_CTL_WBUTN: u8 = 1;
    pub const BIOC_PZ04_FUNC_CTL_BGPIO_Z_04: u8 = 0;
    pub const BIOC_PZ04_FUNC_CTL_PLED: u8 = 1;
    pub const BIOC_PZ04_FUNC_CTL_SOC_PZ_04: u8 = 3;
    pub const BIOC_PZ04_FUNC_CTL_TAMP_04: u8 = 2;
    pub const BIOC_PZ05_FUNC_CTL_BGPIO_Z_05: u8 = 0;
    pub const BIOC_PZ05_FUNC_CTL_SOC_PZ_05: u8 = 3;
    pub const BIOC_PZ05_FUNC_CTL_TAMP_05: u8 = 2;
    pub const BIOC_PZ05_FUNC_CTL_WLED: u8 = 1;
    pub const BIOC_PZ06_FUNC_CTL_BGPIO_Z_06: u8 = 0;
    pub const BIOC_PZ06_FUNC_CTL_SOC_PZ_06: u8 = 3;
    pub const BIOC_PZ06_FUNC_CTL_TAMP_06: u8 = 2;
    pub const BIOC_PZ07_FUNC_CTL_BGPIO_Z_07: u8 = 0;
    pub const BIOC_PZ07_FUNC_CTL_SOC_PZ_07: u8 = 3;
    pub const BIOC_PZ07_FUNC_CTL_TAMP_07: u8 = 2;
    pub const BIOC_PZ08_FUNC_CTL_BGPIO_Z_08: u8 = 0;
    pub const BIOC_PZ08_FUNC_CTL_SOC_PZ_08: u8 = 3;
    pub const BIOC_PZ08_FUNC_CTL_TAMP_08: u8 = 2;
    pub const BIOC_PZ09_FUNC_CTL_BGPIO_Z_09: u8 = 0;
    pub const BIOC_PZ09_FUNC_CTL_SOC_PZ_09: u8 = 3;
    pub const BIOC_PZ09_FUNC_CTL_TAMP_09: u8 = 2;
    pub const BIOC_PZ10_FUNC_CTL_BGPIO_Z_10: u8 = 0;
    pub const BIOC_PZ10_FUNC_CTL_HIBERNATE: u8 = 1;
    pub const BIOC_PZ10_FUNC_CTL_SOC_PZ_10: u8 = 3;
    pub const BIOC_PZ10_FUNC_CTL_TAMP_10: u8 = 2;
    pub const BIOC_PZ11_FUNC_CTL_BGPIO_Z_11: u8 = 0;
    pub const BIOC_PZ11_FUNC_CTL_SOC_PZ_11: u8 = 3;
    pub const BIOC_PZ11_FUNC_CTL_STANDBY: u8 = 1;
    pub const BIOC_PZ11_FUNC_CTL_TAMP_11: u8 = 2;
    pub const IOC_PA00_FUNC_CTL_CAM0_D_6: u8 = 22;
    pub const IOC_PA00_FUNC_CTL_GPIO_A_00: u8 = 0;
    pub const IOC_PA00_FUNC_CTL_I2S1_TXD_2: u8 = 8;
    pub const IOC_PA00_FUNC_CTL_I2S2_TXD_0: u8 = 9;
    pub const IOC_PA00_FUNC_CTL_SPI0_DAT3: u8 = 5;
    pub const IOC_PA00_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PA01_FUNC_CTL_CAM0_D_8: u8 = 22;
    pub const IOC_PA01_FUNC_CTL_CAN3_RXD: u8 = 7;
    pub const IOC_PA01_FUNC_CTL_GPIO_A_01: u8 = 0;
    pub const IOC_PA01_FUNC_CTL_I2S1_TXD_1: u8 = 8;
    pub const IOC_PA01_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PA01_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PA02_FUNC_CTL_CAM0_D_9: u8 = 22;
    pub const IOC_PA02_FUNC_CTL_CAN3_TXD: u8 = 7;
    pub const IOC_PA02_FUNC_CTL_GPIO_A_02: u8 = 0;
    pub const IOC_PA02_FUNC_CTL_I2S1_TXD_0: u8 = 8;
    pub const IOC_PA02_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PA02_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PA03_FUNC_CTL_CAM0_D_3: u8 = 22;
    pub const IOC_PA03_FUNC_CTL_GPIO_A_03: u8 = 0;
    pub const IOC_PA03_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PA03_FUNC_CTL_I2S1_TXD_3: u8 = 8;
    pub const IOC_PA03_FUNC_CTL_I2S2_BCLK: u8 = 9;
    pub const IOC_PA03_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PA04_FUNC_CTL_CAM0_D_7: u8 = 22;
    pub const IOC_PA04_FUNC_CTL_GPIO_A_04: u8 = 0;
    pub const IOC_PA04_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PA04_FUNC_CTL_I2S1_MCLK: u8 = 8;
    pub const IOC_PA04_FUNC_CTL_I2S2_MCLK: u8 = 9;
    pub const IOC_PA04_FUNC_CTL_SPI0_DAT2: u8 = 5;
    pub const IOC_PA04_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PA05_FUNC_CTL_CAM0_HSYNC: u8 = 22;
    pub const IOC_PA05_FUNC_CTL_DAOL_P: u8 = 10;
    pub const IOC_PA05_FUNC_CTL_GPIO_A_05: u8 = 0;
    pub const IOC_PA05_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PA05_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PA05_FUNC_CTL_I2S1_BCLK: u8 = 8;
    pub const IOC_PA05_FUNC_CTL_SPI0_CSN: u8 = 5;
    pub const IOC_PA05_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PA06_FUNC_CTL_CAM0_VSYNC: u8 = 22;
    pub const IOC_PA06_FUNC_CTL_DAOR_P: u8 = 10;
    pub const IOC_PA06_FUNC_CTL_GPIO_A_06: u8 = 0;
    pub const IOC_PA06_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PA06_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PA06_FUNC_CTL_I2S1_FCLK: u8 = 8;
    pub const IOC_PA06_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PA06_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PA07_FUNC_CTL_CAM0_D_2: u8 = 22;
    pub const IOC_PA07_FUNC_CTL_GPIO_A_07: u8 = 0;
    pub const IOC_PA07_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PA07_FUNC_CTL_I2S1_MCLK: u8 = 9;
    pub const IOC_PA07_FUNC_CTL_SOC_REF1: u8 = 24;
    pub const IOC_PA07_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PA08_FUNC_CTL_CAM0_D_4: u8 = 22;
    pub const IOC_PA08_FUNC_CTL_GPIO_A_08: u8 = 0;
    pub const IOC_PA08_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PA08_FUNC_CTL_I2S1_RXD_3: u8 = 8;
    pub const IOC_PA08_FUNC_CTL_I2S2_FCLK: u8 = 9;
    pub const IOC_PA08_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PA09_FUNC_CTL_CAM0_D_5: u8 = 22;
    pub const IOC_PA09_FUNC_CTL_GPIO_A_09: u8 = 0;
    pub const IOC_PA09_FUNC_CTL_I2S1_RXD_2: u8 = 8;
    pub const IOC_PA09_FUNC_CTL_I2S2_RXD_0: u8 = 9;
    pub const IOC_PA09_FUNC_CTL_SOC_REF0: u8 = 24;
    pub const IOC_PA09_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PA10_FUNC_CTL_CAM0_XCLK: u8 = 22;
    pub const IOC_PA10_FUNC_CTL_CAN3_STBY: u8 = 7;
    pub const IOC_PA10_FUNC_CTL_DAOL_N: u8 = 10;
    pub const IOC_PA10_FUNC_CTL_GPIO_A_10: u8 = 0;
    pub const IOC_PA10_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PA10_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PA10_FUNC_CTL_I2S1_RXD_1: u8 = 8;
    pub const IOC_PA10_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PA11_FUNC_CTL_CAM0_PIXCLK: u8 = 22;
    pub const IOC_PA11_FUNC_CTL_CAN2_STBY: u8 = 7;
    pub const IOC_PA11_FUNC_CTL_DAOR_N: u8 = 10;
    pub const IOC_PA11_FUNC_CTL_GPIO_A_11: u8 = 0;
    pub const IOC_PA11_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PA11_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PA11_FUNC_CTL_I2S1_RXD_0: u8 = 8;
    pub const IOC_PA11_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PA12_FUNC_CTL_CAM0_PIXCLK: u8 = 22;
    pub const IOC_PA12_FUNC_CTL_DIS0_B_4: u8 = 20;
    pub const IOC_PA12_FUNC_CTL_GPIO_A_12: u8 = 0;
    pub const IOC_PA12_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PA12_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PA12_FUNC_CTL_I2S1_RXD_3: u8 = 9;
    pub const IOC_PA12_FUNC_CTL_SPI1_DAT3: u8 = 5;
    pub const IOC_PA12_FUNC_CTL_UART9_CTS: u8 = 3;
    pub const IOC_PA13_FUNC_CTL_CAM0_HSYNC: u8 = 22;
    pub const IOC_PA13_FUNC_CTL_DIS0_B_6: u8 = 20;
    pub const IOC_PA13_FUNC_CTL_GPIO_A_13: u8 = 0;
    pub const IOC_PA13_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PA13_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PA13_FUNC_CTL_I2S1_RXD_2: u8 = 9;
    pub const IOC_PA13_FUNC_CTL_SPI1_DAT2: u8 = 5;
    pub const IOC_PA13_FUNC_CTL_UART9_RTS: u8 = 3;
    pub const IOC_PA14_FUNC_CTL_CAN1_STBY: u8 = 7;
    pub const IOC_PA14_FUNC_CTL_DIS0_VSYNC: u8 = 20;
    pub const IOC_PA14_FUNC_CTL_GPIO_A_14: u8 = 0;
    pub const IOC_PA14_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PA14_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PA14_FUNC_CTL_I2S1_TXD_3: u8 = 9;
    pub const IOC_PA14_FUNC_CTL_I2S2_TXD_3: u8 = 8;
    pub const IOC_PA14_FUNC_CTL_UART8_CTS: u8 = 3;
    pub const IOC_PA15_FUNC_CTL_CAN0_STBY: u8 = 7;
    pub const IOC_PA15_FUNC_CTL_DIS0_EN: u8 = 20;
    pub const IOC_PA15_FUNC_CTL_GPIO_A_15: u8 = 0;
    pub const IOC_PA15_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PA15_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PA15_FUNC_CTL_I2S1_TXD_2: u8 = 9;
    pub const IOC_PA15_FUNC_CTL_I2S2_TXD_2: u8 = 8;
    pub const IOC_PA15_FUNC_CTL_UART8_DE: u8 = 2;
    pub const IOC_PA15_FUNC_CTL_UART8_RTS: u8 = 3;
    pub const IOC_PA16_FUNC_CTL_CAM0_XCLK: u8 = 22;
    pub const IOC_PA16_FUNC_CTL_CAN0_RXD: u8 = 7;
    pub const IOC_PA16_FUNC_CTL_DIS0_B_3: u8 = 20;
    pub const IOC_PA16_FUNC_CTL_GPIO_A_16: u8 = 0;
    pub const IOC_PA16_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PA16_FUNC_CTL_I2S1_RXD_1: u8 = 9;
    pub const IOC_PA16_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PA16_FUNC_CTL_UART10_CTS: u8 = 3;
    pub const IOC_PA17_FUNC_CTL_CAM0_VSYNC: u8 = 22;
    pub const IOC_PA17_FUNC_CTL_CAN0_TXD: u8 = 7;
    pub const IOC_PA17_FUNC_CTL_DIS0_B_5: u8 = 20;
    pub const IOC_PA17_FUNC_CTL_GPIO_A_17: u8 = 0;
    pub const IOC_PA17_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PA17_FUNC_CTL_I2S1_RXD_0: u8 = 9;
    pub const IOC_PA17_FUNC_CTL_UART10_DE: u8 = 2;
    pub const IOC_PA17_FUNC_CTL_UART10_RTS: u8 = 3;
    pub const IOC_PA18_FUNC_CTL_CAM0_D_4: u8 = 22;
    pub const IOC_PA18_FUNC_CTL_DIS0_B_7: u8 = 20;
    pub const IOC_PA18_FUNC_CTL_GPIO_A_18: u8 = 0;
    pub const IOC_PA18_FUNC_CTL_I2S1_BCLK: u8 = 9;
    pub const IOC_PA18_FUNC_CTL_SPI1_CSN: u8 = 5;
    pub const IOC_PA18_FUNC_CTL_UART10_TXD: u8 = 2;
    pub const IOC_PA19_FUNC_CTL_CAN1_RXD: u8 = 7;
    pub const IOC_PA19_FUNC_CTL_DIS0_HSYNC: u8 = 20;
    pub const IOC_PA19_FUNC_CTL_GPIO_A_19: u8 = 0;
    pub const IOC_PA19_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PA19_FUNC_CTL_I2S1_TXD_1: u8 = 9;
    pub const IOC_PA19_FUNC_CTL_I2S2_TXD_1: u8 = 8;
    pub const IOC_PA19_FUNC_CTL_PWM1_P_7: u8 = 16;
    pub const IOC_PA19_FUNC_CTL_UART11_CTS: u8 = 3;
    pub const IOC_PA20_FUNC_CTL_CAN1_TXD: u8 = 7;
    pub const IOC_PA20_FUNC_CTL_DIS0_CLK: u8 = 20;
    pub const IOC_PA20_FUNC_CTL_GPIO_A_20: u8 = 0;
    pub const IOC_PA20_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PA20_FUNC_CTL_I2S1_TXD_0: u8 = 9;
    pub const IOC_PA20_FUNC_CTL_I2S2_TXD_0: u8 = 8;
    pub const IOC_PA20_FUNC_CTL_PWM1_P_6: u8 = 16;
    pub const IOC_PA20_FUNC_CTL_UART11_DE: u8 = 2;
    pub const IOC_PA20_FUNC_CTL_UART11_RTS: u8 = 3;
    pub const IOC_PA21_FUNC_CTL_DAOL_P: u8 = 10;
    pub const IOC_PA21_FUNC_CTL_DIS0_R_5: u8 = 20;
    pub const IOC_PA21_FUNC_CTL_GPIO_A_21: u8 = 0;
    pub const IOC_PA21_FUNC_CTL_I2S3_BCLK: u8 = 9;
    pub const IOC_PA21_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PA21_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PA21_FUNC_CTL_UART11_RXD: u8 = 2;
    pub const IOC_PA22_FUNC_CTL_CAM0_D_2: u8 = 22;
    pub const IOC_PA22_FUNC_CTL_DAOR_P: u8 = 10;
    pub const IOC_PA22_FUNC_CTL_DIS0_G_2: u8 = 20;
    pub const IOC_PA22_FUNC_CTL_GPIO_A_22: u8 = 0;
    pub const IOC_PA22_FUNC_CTL_I2S1_FCLK: u8 = 9;
    pub const IOC_PA22_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PA22_FUNC_CTL_UART11_TXD: u8 = 2;
    pub const IOC_PA23_FUNC_CTL_CAM0_D_3: u8 = 22;
    pub const IOC_PA23_FUNC_CTL_DIS0_G_3: u8 = 20;
    pub const IOC_PA23_FUNC_CTL_GPIO_A_23: u8 = 0;
    pub const IOC_PA23_FUNC_CTL_I2S2_MCLK: u8 = 8;
    pub const IOC_PA23_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PA23_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PA23_FUNC_CTL_UART10_RXD: u8 = 2;
    pub const IOC_PA24_FUNC_CTL_CAM0_D_6: u8 = 22;
    pub const IOC_PA24_FUNC_CTL_CAN2_RXD: u8 = 7;
    pub const IOC_PA24_FUNC_CTL_DIS0_G_5: u8 = 20;
    pub const IOC_PA24_FUNC_CTL_GPIO_A_24: u8 = 0;
    pub const IOC_PA24_FUNC_CTL_I2S2_FCLK: u8 = 8;
    pub const IOC_PA24_FUNC_CTL_PWM1_P_5: u8 = 16;
    pub const IOC_PA24_FUNC_CTL_UART8_RXD: u8 = 2;
    pub const IOC_PA25_FUNC_CTL_CAM0_D_7: u8 = 22;
    pub const IOC_PA25_FUNC_CTL_CAN2_TXD: u8 = 7;
    pub const IOC_PA25_FUNC_CTL_DIS0_G_7: u8 = 20;
    pub const IOC_PA25_FUNC_CTL_GPIO_A_25: u8 = 0;
    pub const IOC_PA25_FUNC_CTL_I2S2_BCLK: u8 = 8;
    pub const IOC_PA25_FUNC_CTL_PWM1_P_4: u8 = 16;
    pub const IOC_PA25_FUNC_CTL_UART8_TXD: u8 = 2;
    pub const IOC_PA26_FUNC_CTL_DAOL_N: u8 = 10;
    pub const IOC_PA26_FUNC_CTL_DIS0_R_4: u8 = 20;
    pub const IOC_PA26_FUNC_CTL_GPIO_A_26: u8 = 0;
    pub const IOC_PA26_FUNC_CTL_I2S3_TXD_1: u8 = 9;
    pub const IOC_PA26_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PA26_FUNC_CTL_SPI2_CSN: u8 = 5;
    pub const IOC_PA26_FUNC_CTL_UART12_RXD: u8 = 2;
    pub const IOC_PA27_FUNC_CTL_DAOR_N: u8 = 10;
    pub const IOC_PA27_FUNC_CTL_DIS0_R_6: u8 = 20;
    pub const IOC_PA27_FUNC_CTL_GPIO_A_27: u8 = 0;
    pub const IOC_PA27_FUNC_CTL_I2S3_TXD_0: u8 = 9;
    pub const IOC_PA27_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PA27_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PA27_FUNC_CTL_UART12_TXD: u8 = 2;
    pub const IOC_PA28_FUNC_CTL_CAM0_D_5: u8 = 22;
    pub const IOC_PA28_FUNC_CTL_DIS0_R_7: u8 = 20;
    pub const IOC_PA28_FUNC_CTL_GPIO_A_28: u8 = 0;
    pub const IOC_PA28_FUNC_CTL_I2S3_BCLK: u8 = 9;
    pub const IOC_PA28_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PA28_FUNC_CTL_UART13_TXD: u8 = 2;
    pub const IOC_PA29_FUNC_CTL_CAM0_D_9: u8 = 22;
    pub const IOC_PA29_FUNC_CTL_CAN3_RXD: u8 = 7;
    pub const IOC_PA29_FUNC_CTL_DIS0_G_4: u8 = 20;
    pub const IOC_PA29_FUNC_CTL_GPIO_A_29: u8 = 0;
    pub const IOC_PA29_FUNC_CTL_I2S2_RXD_1: u8 = 8;
    pub const IOC_PA29_FUNC_CTL_I2S3_RXD_1: u8 = 9;
    pub const IOC_PA29_FUNC_CTL_PWM1_P_3: u8 = 16;
    pub const IOC_PA29_FUNC_CTL_UART9_RXD: u8 = 2;
    pub const IOC_PA30_FUNC_CTL_CAM0_D_8: u8 = 22;
    pub const IOC_PA30_FUNC_CTL_CAN3_TXD: u8 = 7;
    pub const IOC_PA30_FUNC_CTL_DIS0_G_6: u8 = 20;
    pub const IOC_PA30_FUNC_CTL_GPIO_A_30: u8 = 0;
    pub const IOC_PA30_FUNC_CTL_I2S2_RXD_0: u8 = 8;
    pub const IOC_PA30_FUNC_CTL_I2S3_RXD_0: u8 = 9;
    pub const IOC_PA30_FUNC_CTL_PWM1_P_1: u8 = 16;
    pub const IOC_PA30_FUNC_CTL_UART9_TXD: u8 = 2;
    pub const IOC_PA31_FUNC_CTL_DIS0_R_3: u8 = 20;
    pub const IOC_PA31_FUNC_CTL_GPIO_A_31: u8 = 0;
    pub const IOC_PA31_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PA31_FUNC_CTL_I2S3_TXD_3: u8 = 9;
    pub const IOC_PA31_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PA31_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PA31_FUNC_CTL_UART13_CTS: u8 = 3;
    pub const IOC_PB00_FUNC_CTL_DIS0_R_2: u8 = 20;
    pub const IOC_PB00_FUNC_CTL_GPIO_B_00: u8 = 0;
    pub const IOC_PB00_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PB00_FUNC_CTL_I2S3_TXD_2: u8 = 9;
    pub const IOC_PB00_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PB00_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PB00_FUNC_CTL_UART13_DE: u8 = 2;
    pub const IOC_PB00_FUNC_CTL_UART13_RTS: u8 = 3;
    pub const IOC_PB01_FUNC_CTL_DIS0_G_1: u8 = 20;
    pub const IOC_PB01_FUNC_CTL_GPIO_B_01: u8 = 0;
    pub const IOC_PB01_FUNC_CTL_I2S2_RXD_3: u8 = 8;
    pub const IOC_PB01_FUNC_CTL_I2S3_RXD_3: u8 = 9;
    pub const IOC_PB01_FUNC_CTL_PWM1_P_2: u8 = 16;
    pub const IOC_PB01_FUNC_CTL_SYSCTL_CLK_OBS_1: u8 = 24;
    pub const IOC_PB01_FUNC_CTL_UART14_RXD: u8 = 2;
    pub const IOC_PB02_FUNC_CTL_DIS0_B_2: u8 = 20;
    pub const IOC_PB02_FUNC_CTL_GPIO_B_02: u8 = 0;
    pub const IOC_PB02_FUNC_CTL_I2S2_RXD_2: u8 = 8;
    pub const IOC_PB02_FUNC_CTL_I2S3_RXD_2: u8 = 9;
    pub const IOC_PB02_FUNC_CTL_PWM1_P_0: u8 = 16;
    pub const IOC_PB02_FUNC_CTL_SYSCTL_CLK_OBS_0: u8 = 24;
    pub const IOC_PB02_FUNC_CTL_UART14_TXD: u8 = 2;
    pub const IOC_PB03_FUNC_CTL_CAM1_D_2: u8 = 22;
    pub const IOC_PB03_FUNC_CTL_DIS0_R_0: u8 = 20;
    pub const IOC_PB03_FUNC_CTL_GPIO_B_03: u8 = 0;
    pub const IOC_PB03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PB03_FUNC_CTL_I2S3_MCLK: u8 = 9;
    pub const IOC_PB03_FUNC_CTL_TRGM0_P_11: u8 = 16;
    pub const IOC_PB03_FUNC_CTL_UART14_CTS: u8 = 3;
    pub const IOC_PB04_FUNC_CTL_CAM1_D_4: u8 = 22;
    pub const IOC_PB04_FUNC_CTL_DIS0_R_1: u8 = 20;
    pub const IOC_PB04_FUNC_CTL_GPIO_B_04: u8 = 0;
    pub const IOC_PB04_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PB04_FUNC_CTL_I2S2_FCLK: u8 = 9;
    pub const IOC_PB04_FUNC_CTL_I2S3_RXD_3: u8 = 8;
    pub const IOC_PB04_FUNC_CTL_TRGM0_P_10: u8 = 16;
    pub const IOC_PB04_FUNC_CTL_UART14_DE: u8 = 2;
    pub const IOC_PB04_FUNC_CTL_UART14_RTS: u8 = 3;
    pub const IOC_PB05_FUNC_CTL_CAM1_D_6: u8 = 22;
    pub const IOC_PB05_FUNC_CTL_DIS0_B_0: u8 = 20;
    pub const IOC_PB05_FUNC_CTL_GPIO_B_05: u8 = 0;
    pub const IOC_PB05_FUNC_CTL_I2S2_RXD_0: u8 = 9;
    pub const IOC_PB05_FUNC_CTL_I2S3_RXD_2: u8 = 8;
    pub const IOC_PB05_FUNC_CTL_TRGM0_P_08: u8 = 16;
    pub const IOC_PB05_FUNC_CTL_UART13_RXD: u8 = 2;
    pub const IOC_PB06_FUNC_CTL_CAM1_PIXCLK: u8 = 22;
    pub const IOC_PB06_FUNC_CTL_DIS0_G_0: u8 = 20;
    pub const IOC_PB06_FUNC_CTL_GPIO_B_06: u8 = 0;
    pub const IOC_PB06_FUNC_CTL_I2S3_RXD_1: u8 = 8;
    pub const IOC_PB06_FUNC_CTL_SYSCTL_CLK_OBS_2: u8 = 24;
    pub const IOC_PB06_FUNC_CTL_TRGM1_P_05: u8 = 16;
    pub const IOC_PB06_FUNC_CTL_UART15_RXD: u8 = 2;
    pub const IOC_PB07_FUNC_CTL_CAM1_HSYNC: u8 = 22;
    pub const IOC_PB07_FUNC_CTL_DIS0_B_1: u8 = 20;
    pub const IOC_PB07_FUNC_CTL_GPIO_B_07: u8 = 0;
    pub const IOC_PB07_FUNC_CTL_I2S3_RXD_0: u8 = 8;
    pub const IOC_PB07_FUNC_CTL_SYSCTL_CLK_OBS_3: u8 = 24;
    pub const IOC_PB07_FUNC_CTL_TRGM1_P_02: u8 = 16;
    pub const IOC_PB07_FUNC_CTL_UART15_TXD: u8 = 2;
    pub const IOC_PB08_FUNC_CTL_CAM1_D_3: u8 = 22;
    pub const IOC_PB08_FUNC_CTL_CAN2_RXD: u8 = 7;
    pub const IOC_PB08_FUNC_CTL_GPIO_B_08: u8 = 0;
    pub const IOC_PB08_FUNC_CTL_I2S2_BCLK: u8 = 9;
    pub const IOC_PB08_FUNC_CTL_I2S3_TXD_3: u8 = 8;
    pub const IOC_PB08_FUNC_CTL_TRGM0_P_09: u8 = 16;
    pub const IOC_PB08_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PB09_FUNC_CTL_CAM1_D_5: u8 = 22;
    pub const IOC_PB09_FUNC_CTL_CAN2_TXD: u8 = 7;
    pub const IOC_PB09_FUNC_CTL_GPIO_B_09: u8 = 0;
    pub const IOC_PB09_FUNC_CTL_I2S2_MCLK: u8 = 9;
    pub const IOC_PB09_FUNC_CTL_I2S3_MCLK: u8 = 8;
    pub const IOC_PB09_FUNC_CTL_TRGM0_P_07: u8 = 16;
    pub const IOC_PB09_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PB09_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PB10_FUNC_CTL_CAM1_XCLK: u8 = 22;
    pub const IOC_PB10_FUNC_CTL_CAN3_STBY: u8 = 7;
    pub const IOC_PB10_FUNC_CTL_GPIO_B_10: u8 = 0;
    pub const IOC_PB10_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PB10_FUNC_CTL_I2S3_BCLK: u8 = 8;
    pub const IOC_PB10_FUNC_CTL_TRGM1_P_04: u8 = 16;
    pub const IOC_PB10_FUNC_CTL_UART12_CTS: u8 = 3;
    pub const IOC_PB11_FUNC_CTL_CAM1_VSYNC: u8 = 22;
    pub const IOC_PB11_FUNC_CTL_CAN2_STBY: u8 = 7;
    pub const IOC_PB11_FUNC_CTL_GPIO_B_11: u8 = 0;
    pub const IOC_PB11_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PB11_FUNC_CTL_I2S3_FCLK: u8 = 8;
    pub const IOC_PB11_FUNC_CTL_TRGM1_P_01: u8 = 16;
    pub const IOC_PB11_FUNC_CTL_UART12_DE: u8 = 2;
    pub const IOC_PB11_FUNC_CTL_UART12_RTS: u8 = 3;
    pub const IOC_PB12_FUNC_CTL_CAM1_D_7: u8 = 22;
    pub const IOC_PB12_FUNC_CTL_CAN3_TXD: u8 = 7;
    pub const IOC_PB12_FUNC_CTL_GPIO_B_12: u8 = 0;
    pub const IOC_PB12_FUNC_CTL_I2S2_TXD_0: u8 = 9;
    pub const IOC_PB12_FUNC_CTL_I2S3_TXD_2: u8 = 8;
    pub const IOC_PB12_FUNC_CTL_TRGM0_P_06: u8 = 16;
    pub const IOC_PB12_FUNC_CTL_UART3_DE: u8 = 2;
    pub const IOC_PB12_FUNC_CTL_UART3_RTS: u8 = 3;
    pub const IOC_PB13_FUNC_CTL_CAM1_D_8: u8 = 22;
    pub const IOC_PB13_FUNC_CTL_CAN1_STBY: u8 = 7;
    pub const IOC_PB13_FUNC_CTL_GPIO_B_13: u8 = 0;
    pub const IOC_PB13_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PB13_FUNC_CTL_I2S3_TXD_1: u8 = 8;
    pub const IOC_PB13_FUNC_CTL_TRGM1_P_03: u8 = 16;
    pub const IOC_PB13_FUNC_CTL_UART15_CTS: u8 = 3;
    pub const IOC_PB14_FUNC_CTL_CAM1_D_9: u8 = 22;
    pub const IOC_PB14_FUNC_CTL_CAN0_STBY: u8 = 7;
    pub const IOC_PB14_FUNC_CTL_GPIO_B_14: u8 = 0;
    pub const IOC_PB14_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PB14_FUNC_CTL_I2S3_TXD_0: u8 = 8;
    pub const IOC_PB14_FUNC_CTL_TRGM1_P_00: u8 = 16;
    pub const IOC_PB14_FUNC_CTL_UART15_DE: u8 = 2;
    pub const IOC_PB14_FUNC_CTL_UART15_RTS: u8 = 3;
    pub const IOC_PB15_FUNC_CTL_CAN0_TXD: u8 = 7;
    pub const IOC_PB15_FUNC_CTL_DAOR_P: u8 = 10;
    pub const IOC_PB15_FUNC_CTL_GPIO_B_15: u8 = 0;
    pub const IOC_PB15_FUNC_CTL_PWM0_FAULT_0: u8 = 16;
    pub const IOC_PB15_FUNC_CTL_SOC_REF0: u8 = 24;
    pub const IOC_PB15_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PB15_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PB16_FUNC_CTL_DAOR_N: u8 = 10;
    pub const IOC_PB16_FUNC_CTL_GPIO_B_16: u8 = 0;
    pub const IOC_PB16_FUNC_CTL_PWM1_FAULT_1: u8 = 16;
    pub const IOC_PB16_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PB17_FUNC_CTL_CAN0_RXD: u8 = 7;
    pub const IOC_PB17_FUNC_CTL_DAOL_P: u8 = 10;
    pub const IOC_PB17_FUNC_CTL_GPIO_B_17: u8 = 0;
    pub const IOC_PB17_FUNC_CTL_PWM0_FAULT_1: u8 = 16;
    pub const IOC_PB17_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PB18_FUNC_CTL_CAN1_RXD: u8 = 7;
    pub const IOC_PB18_FUNC_CTL_DAOL_N: u8 = 10;
    pub const IOC_PB18_FUNC_CTL_FEMC_DQ_25: u8 = 12;
    pub const IOC_PB18_FUNC_CTL_GPIO_B_18: u8 = 0;
    pub const IOC_PB18_FUNC_CTL_PWM1_P_1: u8 = 16;
    pub const IOC_PB18_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PB19_FUNC_CTL_CAN1_TXD: u8 = 7;
    pub const IOC_PB19_FUNC_CTL_FEMC_DQ_24: u8 = 12;
    pub const IOC_PB19_FUNC_CTL_GPIO_B_19: u8 = 0;
    pub const IOC_PB19_FUNC_CTL_PWM1_P_0: u8 = 16;
    pub const IOC_PB19_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PB19_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PB20_FUNC_CTL_CAN3_RXD: u8 = 7;
    pub const IOC_PB20_FUNC_CTL_FEMC_DQ_23: u8 = 12;
    pub const IOC_PB20_FUNC_CTL_GPIO_B_20: u8 = 0;
    pub const IOC_PB20_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PB20_FUNC_CTL_SPI2_DAT3: u8 = 5;
    pub const IOC_PB20_FUNC_CTL_UART3_CTS: u8 = 3;
    pub const IOC_PB21_FUNC_CTL_FEMC_DQ_27: u8 = 12;
    pub const IOC_PB21_FUNC_CTL_GPIO_B_21: u8 = 0;
    pub const IOC_PB21_FUNC_CTL_PWM1_P_3: u8 = 16;
    pub const IOC_PB21_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PB21_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PB22_FUNC_CTL_FEMC_DQ_26: u8 = 12;
    pub const IOC_PB22_FUNC_CTL_GPIO_B_22: u8 = 0;
    pub const IOC_PB22_FUNC_CTL_PWM1_P_2: u8 = 16;
    pub const IOC_PB22_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PB22_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PB23_FUNC_CTL_FEMC_DQ_22: u8 = 12;
    pub const IOC_PB23_FUNC_CTL_GPIO_B_23: u8 = 0;
    pub const IOC_PB23_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PB23_FUNC_CTL_SPI2_DAT2: u8 = 5;
    pub const IOC_PB23_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PB24_FUNC_CTL_FEMC_DQ_29: u8 = 12;
    pub const IOC_PB24_FUNC_CTL_GPIO_B_24: u8 = 0;
    pub const IOC_PB24_FUNC_CTL_PWM1_P_5: u8 = 16;
    pub const IOC_PB24_FUNC_CTL_SPI2_CSN: u8 = 5;
    pub const IOC_PB24_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PB25_FUNC_CTL_FEMC_DQ_28: u8 = 12;
    pub const IOC_PB25_FUNC_CTL_GPIO_B_25: u8 = 0;
    pub const IOC_PB25_FUNC_CTL_PWM1_P_4: u8 = 16;
    pub const IOC_PB25_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PB25_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PB26_FUNC_CTL_FEMC_DQ_21: u8 = 12;
    pub const IOC_PB26_FUNC_CTL_GPIO_B_26: u8 = 0;
    pub const IOC_PB26_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PB26_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PB27_FUNC_CTL_FEMC_DQ_20: u8 = 12;
    pub const IOC_PB27_FUNC_CTL_GPIO_B_27: u8 = 0;
    pub const IOC_PB27_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PB27_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PB28_FUNC_CTL_FEMC_DQ_19: u8 = 12;
    pub const IOC_PB28_FUNC_CTL_GPIO_B_28: u8 = 0;
    pub const IOC_PB28_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PB28_FUNC_CTL_SPI3_DAT2: u8 = 5;
    pub const IOC_PB28_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PB29_FUNC_CTL_FEMC_DQ_31: u8 = 12;
    pub const IOC_PB29_FUNC_CTL_GPIO_B_29: u8 = 0;
    pub const IOC_PB29_FUNC_CTL_PWM1_P_7: u8 = 16;
    pub const IOC_PB29_FUNC_CTL_SPI3_CSN: u8 = 5;
    pub const IOC_PB29_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PB30_FUNC_CTL_FEMC_DQ_30: u8 = 12;
    pub const IOC_PB30_FUNC_CTL_GPIO_B_30: u8 = 0;
    pub const IOC_PB30_FUNC_CTL_PWM1_P_6: u8 = 16;
    pub const IOC_PB30_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PB30_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PB31_FUNC_CTL_FEMC_DQ_18: u8 = 12;
    pub const IOC_PB31_FUNC_CTL_GPIO_B_31: u8 = 0;
    pub const IOC_PB31_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PB31_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PC00_FUNC_CTL_FEMC_DQ_17: u8 = 12;
    pub const IOC_PC00_FUNC_CTL_GPIO_C_00: u8 = 0;
    pub const IOC_PC00_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PC00_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PC01_FUNC_CTL_FEMC_DQ_16: u8 = 12;
    pub const IOC_PC01_FUNC_CTL_GPIO_C_01: u8 = 0;
    pub const IOC_PC01_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PC01_FUNC_CTL_SPI3_DAT3: u8 = 5;
    pub const IOC_PC01_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PC02_FUNC_CTL_FEMC_DM_2: u8 = 12;
    pub const IOC_PC02_FUNC_CTL_GPIO_C_02: u8 = 0;
    pub const IOC_PC02_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PC02_FUNC_CTL_TRGM0_P_03: u8 = 16;
    pub const IOC_PC02_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PC03_FUNC_CTL_FEMC_DM_3: u8 = 12;
    pub const IOC_PC03_FUNC_CTL_GPIO_C_03: u8 = 0;
    pub const IOC_PC03_FUNC_CTL_PWM1_FAULT_0: u8 = 16;
    pub const IOC_PC03_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PC03_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PC04_FUNC_CTL_FEMC_A_02: u8 = 12;
    pub const IOC_PC04_FUNC_CTL_GPIO_C_04: u8 = 0;
    pub const IOC_PC04_FUNC_CTL_TRGM1_P_11: u8 = 16;
    pub const IOC_PC04_FUNC_CTL_UART10_RXD: u8 = 2;
    pub const IOC_PC05_FUNC_CTL_FEMC_A_03: u8 = 12;
    pub const IOC_PC05_FUNC_CTL_GPIO_C_05: u8 = 0;
    pub const IOC_PC05_FUNC_CTL_TRGM1_P_09: u8 = 16;
    pub const IOC_PC05_FUNC_CTL_UART10_TXD: u8 = 2;
    pub const IOC_PC06_FUNC_CTL_FEMC_A_04: u8 = 12;
    pub const IOC_PC06_FUNC_CTL_GPIO_C_06: u8 = 0;
    pub const IOC_PC06_FUNC_CTL_TRGM0_P_04: u8 = 16;
    pub const IOC_PC06_FUNC_CTL_UART8_RXD: u8 = 2;
    pub const IOC_PC06_FUNC_CTL_XPI1_CB_D_3: u8 = 14;
    pub const IOC_PC07_FUNC_CTL_FEMC_A_05: u8 = 12;
    pub const IOC_PC07_FUNC_CTL_GPIO_C_07: u8 = 0;
    pub const IOC_PC07_FUNC_CTL_TRGM0_P_02: u8 = 16;
    pub const IOC_PC07_FUNC_CTL_UART8_TXD: u8 = 2;
    pub const IOC_PC07_FUNC_CTL_XPI1_CB_D_2: u8 = 14;
    pub const IOC_PC08_FUNC_CTL_ETH1_EVTO_2: u8 = 19;
    pub const IOC_PC08_FUNC_CTL_FEMC_A_00: u8 = 12;
    pub const IOC_PC08_FUNC_CTL_GPIO_C_08: u8 = 0;
    pub const IOC_PC08_FUNC_CTL_TRGM1_P_10: u8 = 16;
    pub const IOC_PC08_FUNC_CTL_UART11_RXD: u8 = 2;
    pub const IOC_PC09_FUNC_CTL_ETH1_EVTO_0: u8 = 19;
    pub const IOC_PC09_FUNC_CTL_FEMC_A_01: u8 = 12;
    pub const IOC_PC09_FUNC_CTL_GPIO_C_09: u8 = 0;
    pub const IOC_PC09_FUNC_CTL_TRGM1_P_08: u8 = 16;
    pub const IOC_PC09_FUNC_CTL_UART11_TXD: u8 = 2;
    pub const IOC_PC09_FUNC_CTL_XPI1_CB_CS1: u8 = 14;
    pub const IOC_PC10_FUNC_CTL_FEMC_A_06: u8 = 12;
    pub const IOC_PC10_FUNC_CTL_GPIO_C_10: u8 = 0;
    pub const IOC_PC10_FUNC_CTL_TRGM1_P_07: u8 = 16;
    pub const IOC_PC10_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PC10_FUNC_CTL_XPI1_CB_CS0: u8 = 14;
    pub const IOC_PC11_FUNC_CTL_FEMC_A_07: u8 = 12;
    pub const IOC_PC11_FUNC_CTL_GPIO_C_11: u8 = 0;
    pub const IOC_PC11_FUNC_CTL_TRGM0_P_05: u8 = 16;
    pub const IOC_PC11_FUNC_CTL_UART9_RXD: u8 = 2;
    pub const IOC_PC11_FUNC_CTL_XPI1_CB_D_1: u8 = 14;
    pub const IOC_PC12_FUNC_CTL_FEMC_A_08: u8 = 12;
    pub const IOC_PC12_FUNC_CTL_GPIO_C_12: u8 = 0;
    pub const IOC_PC12_FUNC_CTL_TRGM0_P_01: u8 = 16;
    pub const IOC_PC12_FUNC_CTL_UART9_TXD: u8 = 2;
    pub const IOC_PC12_FUNC_CTL_XPI1_CB_D_0: u8 = 14;
    pub const IOC_PC13_FUNC_CTL_FEMC_BA0: u8 = 12;
    pub const IOC_PC13_FUNC_CTL_GPIO_C_13: u8 = 0;
    pub const IOC_PC13_FUNC_CTL_UART13_RXD: u8 = 2;
    pub const IOC_PC14_FUNC_CTL_ETH1_EVTO_1: u8 = 19;
    pub const IOC_PC14_FUNC_CTL_FEMC_BA1: u8 = 12;
    pub const IOC_PC14_FUNC_CTL_GPIO_C_14: u8 = 0;
    pub const IOC_PC14_FUNC_CTL_TRGM3_P_07: u8 = 16;
    pub const IOC_PC14_FUNC_CTL_UART13_TXD: u8 = 2;
    pub const IOC_PC14_FUNC_CTL_XPI1_CB_DQS: u8 = 14;
    pub const IOC_PC15_FUNC_CTL_FEMC_A_10: u8 = 12;
    pub const IOC_PC15_FUNC_CTL_GPIO_C_15: u8 = 0;
    pub const IOC_PC15_FUNC_CTL_TRGM1_P_06: u8 = 16;
    pub const IOC_PC15_FUNC_CTL_UART12_TXD: u8 = 2;
    pub const IOC_PC15_FUNC_CTL_XPI1_CB_SCLK: u8 = 14;
    pub const IOC_PC16_FUNC_CTL_FEMC_DQS: u8 = 12;
    pub const IOC_PC16_FUNC_CTL_GPIO_C_16: u8 = 0;
    pub const IOC_PC16_FUNC_CTL_TRGM2_P_02: u8 = 16;
    pub const IOC_PC16_FUNC_CTL_UART14_RXD: u8 = 2;
    pub const IOC_PC16_FUNC_CTL_XPI1_CA_CS0: u8 = 14;
    pub const IOC_PC17_FUNC_CTL_FEMC_A_09: u8 = 12;
    pub const IOC_PC17_FUNC_CTL_GPIO_C_17: u8 = 0;
    pub const IOC_PC17_FUNC_CTL_TRGM0_P_00: u8 = 16;
    pub const IOC_PC17_FUNC_CTL_UART14_TXD: u8 = 2;
    pub const IOC_PC17_FUNC_CTL_XPI1_CA_SCLK: u8 = 14;
    pub const IOC_PC18_FUNC_CTL_FEMC_RAS: u8 = 12;
    pub const IOC_PC18_FUNC_CTL_GPIO_C_18: u8 = 0;
    pub const IOC_PC18_FUNC_CTL_TRGM3_P_10: u8 = 16;
    pub const IOC_PC18_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PC19_FUNC_CTL_ETH1_EVTI_0: u8 = 19;
    pub const IOC_PC19_FUNC_CTL_FEMC_CS_0: u8 = 12;
    pub const IOC_PC19_FUNC_CTL_GPIO_C_19: u8 = 0;
    pub const IOC_PC19_FUNC_CTL_TRGM3_P_08: u8 = 16;
    pub const IOC_PC19_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PC19_FUNC_CTL_XPI1_CA_CS1: u8 = 14;
    pub const IOC_PC20_FUNC_CTL_FEMC_CS_1: u8 = 12;
    pub const IOC_PC20_FUNC_CTL_GPIO_C_20: u8 = 0;
    pub const IOC_PC20_FUNC_CTL_TRGM3_P_06: u8 = 16;
    pub const IOC_PC20_FUNC_CTL_UART12_RXD: u8 = 2;
    pub const IOC_PC20_FUNC_CTL_XPI1_CA_DQS: u8 = 14;
    pub const IOC_PC21_FUNC_CTL_FEMC_A_11: u8 = 12;
    pub const IOC_PC21_FUNC_CTL_GPIO_C_21: u8 = 0;
    pub const IOC_PC21_FUNC_CTL_TRGM2_P_03: u8 = 16;
    pub const IOC_PC21_FUNC_CTL_UART15_RXD: u8 = 2;
    pub const IOC_PC21_FUNC_CTL_XPI1_CA_D_2: u8 = 14;
    pub const IOC_PC22_FUNC_CTL_FEMC_A_12: u8 = 12;
    pub const IOC_PC22_FUNC_CTL_GPIO_C_22: u8 = 0;
    pub const IOC_PC22_FUNC_CTL_TRGM2_P_00: u8 = 16;
    pub const IOC_PC22_FUNC_CTL_UART15_TXD: u8 = 2;
    pub const IOC_PC22_FUNC_CTL_XPI1_CA_D_0: u8 = 14;
    pub const IOC_PC23_FUNC_CTL_ETH1_EVTI_2: u8 = 19;
    pub const IOC_PC23_FUNC_CTL_FEMC_CAS: u8 = 12;
    pub const IOC_PC23_FUNC_CTL_GPIO_C_23: u8 = 0;
    pub const IOC_PC23_FUNC_CTL_TRGM3_P_11: u8 = 16;
    pub const IOC_PC23_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PC24_FUNC_CTL_ETH1_EVTI_1: u8 = 19;
    pub const IOC_PC24_FUNC_CTL_FEMC_WE: u8 = 12;
    pub const IOC_PC24_FUNC_CTL_GPIO_C_24: u8 = 0;
    pub const IOC_PC24_FUNC_CTL_TRGM3_P_09: u8 = 16;
    pub const IOC_PC24_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PC25_FUNC_CTL_FEMC_CKE: u8 = 12;
    pub const IOC_PC25_FUNC_CTL_GPIO_C_25: u8 = 0;
    pub const IOC_PC25_FUNC_CTL_TRGM2_P_04: u8 = 16;
    pub const IOC_PC25_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PC25_FUNC_CTL_XPI1_CA_D_3: u8 = 14;
    pub const IOC_PC26_FUNC_CTL_FEMC_CLK: u8 = 12;
    pub const IOC_PC26_FUNC_CTL_GPIO_C_26: u8 = 0;
    pub const IOC_PC26_FUNC_CTL_TRGM2_P_01: u8 = 16;
    pub const IOC_PC26_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PC26_FUNC_CTL_XPI1_CA_D_1: u8 = 14;
    pub const IOC_PC27_FUNC_CTL_ETH1_EVTO_2: u8 = 19;
    pub const IOC_PC27_FUNC_CTL_FEMC_DQ_05: u8 = 12;
    pub const IOC_PC27_FUNC_CTL_GPIO_C_27: u8 = 0;
    pub const IOC_PC27_FUNC_CTL_PWM3_P_5: u8 = 16;
    pub const IOC_PC27_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PC28_FUNC_CTL_ETH1_EVTO_0: u8 = 19;
    pub const IOC_PC28_FUNC_CTL_FEMC_DQ_06: u8 = 12;
    pub const IOC_PC28_FUNC_CTL_GPIO_C_28: u8 = 0;
    pub const IOC_PC28_FUNC_CTL_PWM3_P_6: u8 = 16;
    pub const IOC_PC28_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PC29_FUNC_CTL_ETH1_EVTI_0: u8 = 19;
    pub const IOC_PC29_FUNC_CTL_FEMC_DQ_07: u8 = 12;
    pub const IOC_PC29_FUNC_CTL_GPIO_C_29: u8 = 0;
    pub const IOC_PC29_FUNC_CTL_PWM3_P_7: u8 = 16;
    pub const IOC_PC29_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PC29_FUNC_CTL_XPI1_CB_CS1: u8 = 14;
    pub const IOC_PC30_FUNC_CTL_FEMC_DM_0: u8 = 12;
    pub const IOC_PC30_FUNC_CTL_GPIO_C_30: u8 = 0;
    pub const IOC_PC30_FUNC_CTL_TRGM2_P_05: u8 = 16;
    pub const IOC_PC30_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PC30_FUNC_CTL_XPI1_CB_CS0: u8 = 14;
    pub const IOC_PC31_FUNC_CTL_FEMC_DM_1: u8 = 12;
    pub const IOC_PC31_FUNC_CTL_GPIO_C_31: u8 = 0;
    pub const IOC_PC31_FUNC_CTL_PWM2_FAULT_0: u8 = 16;
    pub const IOC_PC31_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PC31_FUNC_CTL_XPI1_CB_SCLK: u8 = 14;
    pub const IOC_PD00_FUNC_CTL_FEMC_DQ_02: u8 = 12;
    pub const IOC_PD00_FUNC_CTL_GPIO_D_00: u8 = 0;
    pub const IOC_PD00_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PD00_FUNC_CTL_PWM3_P_2: u8 = 16;
    pub const IOC_PD00_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PD01_FUNC_CTL_ETH1_EVTO_1: u8 = 19;
    pub const IOC_PD01_FUNC_CTL_FEMC_DQ_03: u8 = 12;
    pub const IOC_PD01_FUNC_CTL_GPIO_D_01: u8 = 0;
    pub const IOC_PD01_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PD01_FUNC_CTL_PWM3_P_3: u8 = 16;
    pub const IOC_PD01_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PD01_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PD02_FUNC_CTL_ETH1_EVTI_1: u8 = 19;
    pub const IOC_PD02_FUNC_CTL_FEMC_DQ_04: u8 = 12;
    pub const IOC_PD02_FUNC_CTL_GPIO_D_02: u8 = 0;
    pub const IOC_PD02_FUNC_CTL_PWM3_P_4: u8 = 16;
    pub const IOC_PD02_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PD02_FUNC_CTL_XPI1_CB_DQS: u8 = 14;
    pub const IOC_PD03_FUNC_CTL_FEMC_DQ_09: u8 = 12;
    pub const IOC_PD03_FUNC_CTL_GPIO_D_03: u8 = 0;
    pub const IOC_PD03_FUNC_CTL_PWM2_P_1: u8 = 16;
    pub const IOC_PD03_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PD03_FUNC_CTL_XPI1_CB_D_1: u8 = 14;
    pub const IOC_PD04_FUNC_CTL_FEMC_DQ_08: u8 = 12;
    pub const IOC_PD04_FUNC_CTL_GPIO_D_04: u8 = 0;
    pub const IOC_PD04_FUNC_CTL_PWM2_P_0: u8 = 16;
    pub const IOC_PD04_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PD04_FUNC_CTL_XPI1_CB_D_0: u8 = 14;
    pub const IOC_PD05_FUNC_CTL_FEMC_DQ_01: u8 = 12;
    pub const IOC_PD05_FUNC_CTL_GPIO_D_05: u8 = 0;
    pub const IOC_PD05_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PD05_FUNC_CTL_PWM3_P_1: u8 = 16;
    pub const IOC_PD05_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PD05_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PD06_FUNC_CTL_FEMC_DQ_11: u8 = 12;
    pub const IOC_PD06_FUNC_CTL_GPIO_D_06: u8 = 0;
    pub const IOC_PD06_FUNC_CTL_PWM2_P_3: u8 = 16;
    pub const IOC_PD06_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PD06_FUNC_CTL_XPI1_CB_D_3: u8 = 14;
    pub const IOC_PD07_FUNC_CTL_FEMC_DQ_10: u8 = 12;
    pub const IOC_PD07_FUNC_CTL_GPIO_D_07: u8 = 0;
    pub const IOC_PD07_FUNC_CTL_PWM2_P_2: u8 = 16;
    pub const IOC_PD07_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PD07_FUNC_CTL_XPI1_CB_D_2: u8 = 14;
    pub const IOC_PD08_FUNC_CTL_ETH1_EVTI_2: u8 = 19;
    pub const IOC_PD08_FUNC_CTL_FEMC_DQ_00: u8 = 12;
    pub const IOC_PD08_FUNC_CTL_GPIO_D_08: u8 = 0;
    pub const IOC_PD08_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PD08_FUNC_CTL_PWM3_P_0: u8 = 16;
    pub const IOC_PD08_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PD08_FUNC_CTL_XPI1_CA_CS1: u8 = 14;
    pub const IOC_PD09_FUNC_CTL_CAN2_STBY: u8 = 7;
    pub const IOC_PD09_FUNC_CTL_FEMC_DQ_13: u8 = 12;
    pub const IOC_PD09_FUNC_CTL_GPIO_D_09: u8 = 0;
    pub const IOC_PD09_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PD09_FUNC_CTL_PWM2_P_5: u8 = 16;
    pub const IOC_PD09_FUNC_CTL_UART6_CTS: u8 = 3;
    pub const IOC_PD09_FUNC_CTL_XPI1_CA_DQS: u8 = 14;
    pub const IOC_PD10_FUNC_CTL_CAN0_STBY: u8 = 7;
    pub const IOC_PD10_FUNC_CTL_FEMC_DQ_12: u8 = 12;
    pub const IOC_PD10_FUNC_CTL_GPIO_D_10: u8 = 0;
    pub const IOC_PD10_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PD10_FUNC_CTL_PWM2_P_4: u8 = 16;
    pub const IOC_PD10_FUNC_CTL_UART6_DE: u8 = 2;
    pub const IOC_PD10_FUNC_CTL_UART6_RTS: u8 = 3;
    pub const IOC_PD10_FUNC_CTL_XPI1_CA_SCLK: u8 = 14;
    pub const IOC_PD11_FUNC_CTL_CAN0_RXD: u8 = 7;
    pub const IOC_PD11_FUNC_CTL_ETH1_MDC: u8 = 19;
    pub const IOC_PD11_FUNC_CTL_GPIO_D_11: u8 = 0;
    pub const IOC_PD11_FUNC_CTL_PWM3_FAULT_1: u8 = 16;
    pub const IOC_PD11_FUNC_CTL_UART8_CTS: u8 = 3;
    pub const IOC_PD11_FUNC_CTL_XPI1_CA_D_3: u8 = 14;
    pub const IOC_PD12_FUNC_CTL_CAN3_STBY: u8 = 7;
    pub const IOC_PD12_FUNC_CTL_ETH0_MDC: u8 = 19;
    pub const IOC_PD12_FUNC_CTL_FEMC_DQ_15: u8 = 12;
    pub const IOC_PD12_FUNC_CTL_GPIO_D_12: u8 = 0;
    pub const IOC_PD12_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PD12_FUNC_CTL_PWM2_P_7: u8 = 16;
    pub const IOC_PD12_FUNC_CTL_UART7_CTS: u8 = 3;
    pub const IOC_PD12_FUNC_CTL_XPI1_CA_D_1: u8 = 14;
    pub const IOC_PD13_FUNC_CTL_CAN1_STBY: u8 = 7;
    pub const IOC_PD13_FUNC_CTL_FEMC_DQ_14: u8 = 12;
    pub const IOC_PD13_FUNC_CTL_GPIO_D_13: u8 = 0;
    pub const IOC_PD13_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PD13_FUNC_CTL_PWM2_P_6: u8 = 16;
    pub const IOC_PD13_FUNC_CTL_UART7_DE: u8 = 2;
    pub const IOC_PD13_FUNC_CTL_UART7_RTS: u8 = 3;
    pub const IOC_PD13_FUNC_CTL_XPI1_CA_CS0: u8 = 14;
    pub const IOC_PD14_FUNC_CTL_CAN0_TXD: u8 = 7;
    pub const IOC_PD14_FUNC_CTL_ETH1_MDIO: u8 = 19;
    pub const IOC_PD14_FUNC_CTL_GPIO_D_14: u8 = 0;
    pub const IOC_PD14_FUNC_CTL_PWM3_FAULT_0: u8 = 16;
    pub const IOC_PD14_FUNC_CTL_UART8_DE: u8 = 2;
    pub const IOC_PD14_FUNC_CTL_UART8_RTS: u8 = 3;
    pub const IOC_PD14_FUNC_CTL_XPI1_CA_D_2: u8 = 14;
    pub const IOC_PD15_FUNC_CTL_ETH0_MDIO: u8 = 19;
    pub const IOC_PD15_FUNC_CTL_GPIO_D_15: u8 = 0;
    pub const IOC_PD15_FUNC_CTL_PWM2_FAULT_1: u8 = 16;
    pub const IOC_PD15_FUNC_CTL_UART8_DE: u8 = 2;
    pub const IOC_PD15_FUNC_CTL_XPI1_CA_D_0: u8 = 14;
    pub const IOC_PD16_FUNC_CTL_CAN1_TXD: u8 = 7;
    pub const IOC_PD16_FUNC_CTL_ETH1_TXD_1: u8 = 18;
    pub const IOC_PD16_FUNC_CTL_GPIO_D_16: u8 = 0;
    pub const IOC_PD16_FUNC_CTL_SDC1_DATA_4: u8 = 17;
    pub const IOC_PD16_FUNC_CTL_TRGM2_P_06: u8 = 16;
    pub const IOC_PD16_FUNC_CTL_UART9_DE: u8 = 2;
    pub const IOC_PD16_FUNC_CTL_UART9_RTS: u8 = 3;
    pub const IOC_PD17_FUNC_CTL_CAN3_RXD: u8 = 7;
    pub const IOC_PD17_FUNC_CTL_ETH1_RXD_3: u8 = 18;
    pub const IOC_PD17_FUNC_CTL_GPIO_D_17: u8 = 0;
    pub const IOC_PD17_FUNC_CTL_SDC1_DATA_1: u8 = 17;
    pub const IOC_PD17_FUNC_CTL_TRGM3_P_03: u8 = 16;
    pub const IOC_PD17_FUNC_CTL_UART11_CTS: u8 = 3;
    pub const IOC_PD18_FUNC_CTL_CAN3_TXD: u8 = 7;
    pub const IOC_PD18_FUNC_CTL_ETH1_RXD_0: u8 = 18;
    pub const IOC_PD18_FUNC_CTL_GPIO_D_18: u8 = 0;
    pub const IOC_PD18_FUNC_CTL_SDC1_DATA_0: u8 = 17;
    pub const IOC_PD18_FUNC_CTL_TRGM3_P_00: u8 = 16;
    pub const IOC_PD18_FUNC_CTL_UART11_DE: u8 = 2;
    pub const IOC_PD18_FUNC_CTL_UART11_RTS: u8 = 3;
    pub const IOC_PD19_FUNC_CTL_CAN1_RXD: u8 = 7;
    pub const IOC_PD19_FUNC_CTL_ETH1_TXEN: u8 = 18;
    pub const IOC_PD19_FUNC_CTL_GPIO_D_19: u8 = 0;
    pub const IOC_PD19_FUNC_CTL_SDC1_DATA_6: u8 = 17;
    pub const IOC_PD19_FUNC_CTL_SPI0_DAT2: u8 = 5;
    pub const IOC_PD19_FUNC_CTL_TRGM2_P_09: u8 = 16;
    pub const IOC_PD19_FUNC_CTL_UART9_CTS: u8 = 3;
    pub const IOC_PD20_FUNC_CTL_CAN2_TXD: u8 = 7;
    pub const IOC_PD20_FUNC_CTL_ETH1_TXCK: u8 = 18;
    pub const IOC_PD20_FUNC_CTL_GPIO_D_20: u8 = 0;
    pub const IOC_PD20_FUNC_CTL_SDC1_DS: u8 = 17;
    pub const IOC_PD20_FUNC_CTL_TRGM2_P_07: u8 = 16;
    pub const IOC_PD20_FUNC_CTL_UART10_DE: u8 = 2;
    pub const IOC_PD20_FUNC_CTL_UART10_RTS: u8 = 3;
    pub const IOC_PD21_FUNC_CTL_ETH1_RXDV: u8 = 18;
    pub const IOC_PD21_FUNC_CTL_GPIO_D_21: u8 = 0;
    pub const IOC_PD21_FUNC_CTL_SDC1_CMD: u8 = 17;
    pub const IOC_PD21_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PD21_FUNC_CTL_TRGM3_P_04: u8 = 16;
    pub const IOC_PD21_FUNC_CTL_UART8_RXD: u8 = 2;
    pub const IOC_PD22_FUNC_CTL_ETH1_RXCK: u8 = 18;
    pub const IOC_PD22_FUNC_CTL_GPIO_D_22: u8 = 0;
    pub const IOC_PD22_FUNC_CTL_SDC1_CLK: u8 = 17;
    pub const IOC_PD22_FUNC_CTL_SPI0_CSN: u8 = 5;
    pub const IOC_PD22_FUNC_CTL_TRGM3_P_01: u8 = 16;
    pub const IOC_PD22_FUNC_CTL_UART8_TXD: u8 = 2;
    pub const IOC_PD23_FUNC_CTL_ETH1_TXD_3: u8 = 18;
    pub const IOC_PD23_FUNC_CTL_GPIO_D_23: u8 = 0;
    pub const IOC_PD23_FUNC_CTL_SDC1_RSTN: u8 = 17;
    pub const IOC_PD23_FUNC_CTL_TRGM2_P_11: u8 = 16;
    pub const IOC_PD23_FUNC_CTL_UART10_RXD: u8 = 2;
    pub const IOC_PD24_FUNC_CTL_ETH1_TXD_2: u8 = 18;
    pub const IOC_PD24_FUNC_CTL_GPIO_D_24: u8 = 0;
    pub const IOC_PD24_FUNC_CTL_SDC1_DATA_7: u8 = 17;
    pub const IOC_PD24_FUNC_CTL_SPI0_DAT3: u8 = 5;
    pub const IOC_PD24_FUNC_CTL_TRGM2_P_10: u8 = 16;
    pub const IOC_PD24_FUNC_CTL_UART10_TXD: u8 = 2;
    pub const IOC_PD25_FUNC_CTL_CAN2_RXD: u8 = 7;
    pub const IOC_PD25_FUNC_CTL_ETH1_TXD_0: u8 = 18;
    pub const IOC_PD25_FUNC_CTL_GPIO_D_25: u8 = 0;
    pub const IOC_PD25_FUNC_CTL_SDC1_DATA_5: u8 = 17;
    pub const IOC_PD25_FUNC_CTL_TRGM2_P_08: u8 = 16;
    pub const IOC_PD25_FUNC_CTL_UART10_CTS: u8 = 3;
    pub const IOC_PD26_FUNC_CTL_ETH1_RXD_2: u8 = 18;
    pub const IOC_PD26_FUNC_CTL_GPIO_D_26: u8 = 0;
    pub const IOC_PD26_FUNC_CTL_SDC1_DATA_3: u8 = 17;
    pub const IOC_PD26_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PD26_FUNC_CTL_TRGM3_P_05: u8 = 16;
    pub const IOC_PD26_FUNC_CTL_UART9_RXD: u8 = 2;
    pub const IOC_PD27_FUNC_CTL_ETH1_RXD_1: u8 = 18;
    pub const IOC_PD27_FUNC_CTL_GPIO_D_27: u8 = 0;
    pub const IOC_PD27_FUNC_CTL_SDC1_DATA_2: u8 = 17;
    pub const IOC_PD27_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PD27_FUNC_CTL_TRGM3_P_02: u8 = 16;
    pub const IOC_PD27_FUNC_CTL_UART9_TXD: u8 = 2;
    pub const IOC_PD28_FUNC_CTL_ETH0_TXD_2: u8 = 18;
    pub const IOC_PD28_FUNC_CTL_GPIO_D_28: u8 = 0;
    pub const IOC_PD28_FUNC_CTL_PWM2_P_5: u8 = 16;
    pub const IOC_PD28_FUNC_CTL_SDC1_CDN: u8 = 17;
    pub const IOC_PD28_FUNC_CTL_UART11_RXD: u8 = 2;
    pub const IOC_PD28_FUNC_CTL_XPI0_CA_CS1: u8 = 14;
    pub const IOC_PD29_FUNC_CTL_ETH0_TXD_1: u8 = 18;
    pub const IOC_PD29_FUNC_CTL_GPIO_D_29: u8 = 0;
    pub const IOC_PD29_FUNC_CTL_PWM2_P_4: u8 = 16;
    pub const IOC_PD29_FUNC_CTL_SDC1_VSEL: u8 = 17;
    pub const IOC_PD29_FUNC_CTL_UART11_TXD: u8 = 2;
    pub const IOC_PD29_FUNC_CTL_XPI0_CB_DQS: u8 = 14;
    pub const IOC_PD30_FUNC_CTL_CAN0_RXD: u8 = 7;
    pub const IOC_PD30_FUNC_CTL_ETH0_RXDV: u8 = 18;
    pub const IOC_PD30_FUNC_CTL_GPIO_D_30: u8 = 0;
    pub const IOC_PD30_FUNC_CTL_PWM2_P_1: u8 = 16;
    pub const IOC_PD30_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PD30_FUNC_CTL_UART14_RXD: u8 = 2;
    pub const IOC_PD30_FUNC_CTL_XPI0_CB_D_2: u8 = 14;
    pub const IOC_PD31_FUNC_CTL_CAN0_TXD: u8 = 7;
    pub const IOC_PD31_FUNC_CTL_ETH0_RXD_0: u8 = 18;
    pub const IOC_PD31_FUNC_CTL_GPIO_D_31: u8 = 0;
    pub const IOC_PD31_FUNC_CTL_PWM2_P_0: u8 = 16;
    pub const IOC_PD31_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PD31_FUNC_CTL_UART14_TXD: u8 = 2;
    pub const IOC_PD31_FUNC_CTL_XPI0_CB_D_0: u8 = 14;
    pub const IOC_PE00_FUNC_CTL_ETH0_TXEN: u8 = 18;
    pub const IOC_PE00_FUNC_CTL_GPIO_E_00: u8 = 0;
    pub const IOC_PE00_FUNC_CTL_PWM2_P_6: u8 = 16;
    pub const IOC_PE00_FUNC_CTL_SDC1_WP: u8 = 17;
    pub const IOC_PE00_FUNC_CTL_UART12_RXD: u8 = 2;
    pub const IOC_PE00_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PE01_FUNC_CTL_ETH0_TXCK: u8 = 18;
    pub const IOC_PE01_FUNC_CTL_GPIO_E_01: u8 = 0;
    pub const IOC_PE01_FUNC_CTL_PWM2_P_7: u8 = 16;
    pub const IOC_PE01_FUNC_CTL_SDC0_CDN: u8 = 17;
    pub const IOC_PE01_FUNC_CTL_UART12_TXD: u8 = 2;
    pub const IOC_PE01_FUNC_CTL_XPI0_CB_CS1: u8 = 14;
    pub const IOC_PE02_FUNC_CTL_ETH0_RXD_2: u8 = 18;
    pub const IOC_PE02_FUNC_CTL_GPIO_E_02: u8 = 0;
    pub const IOC_PE02_FUNC_CTL_PWM3_P_0: u8 = 16;
    pub const IOC_PE02_FUNC_CTL_SPI1_DAT2: u8 = 5;
    pub const IOC_PE02_FUNC_CTL_UART13_TXD: u8 = 2;
    pub const IOC_PE02_FUNC_CTL_XPI0_CB_CS0: u8 = 14;
    pub const IOC_PE03_FUNC_CTL_CAN1_RXD: u8 = 7;
    pub const IOC_PE03_FUNC_CTL_ETH0_RXCK: u8 = 18;
    pub const IOC_PE03_FUNC_CTL_GPIO_E_03: u8 = 0;
    pub const IOC_PE03_FUNC_CTL_PWM2_P_3: u8 = 16;
    pub const IOC_PE03_FUNC_CTL_SPI1_CSN: u8 = 5;
    pub const IOC_PE03_FUNC_CTL_UART15_RXD: u8 = 2;
    pub const IOC_PE03_FUNC_CTL_XPI0_CB_D_3: u8 = 14;
    pub const IOC_PE04_FUNC_CTL_CAN1_TXD: u8 = 7;
    pub const IOC_PE04_FUNC_CTL_ETH0_RXD_1: u8 = 18;
    pub const IOC_PE04_FUNC_CTL_GPIO_E_04: u8 = 0;
    pub const IOC_PE04_FUNC_CTL_PWM2_P_2: u8 = 16;
    pub const IOC_PE04_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PE04_FUNC_CTL_UART15_TXD: u8 = 2;
    pub const IOC_PE04_FUNC_CTL_XPI0_CB_D_1: u8 = 14;
    pub const IOC_PE05_FUNC_CTL_CAN3_RXD: u8 = 7;
    pub const IOC_PE05_FUNC_CTL_ETH0_TXD_3: u8 = 18;
    pub const IOC_PE05_FUNC_CTL_GPIO_E_05: u8 = 0;
    pub const IOC_PE05_FUNC_CTL_PWM3_P_4: u8 = 16;
    pub const IOC_PE05_FUNC_CTL_SDC0_WP: u8 = 17;
    pub const IOC_PE05_FUNC_CTL_UART13_CTS: u8 = 3;
    pub const IOC_PE06_FUNC_CTL_CAN3_TXD: u8 = 7;
    pub const IOC_PE06_FUNC_CTL_ETH0_TXD_0: u8 = 18;
    pub const IOC_PE06_FUNC_CTL_GPIO_E_06: u8 = 0;
    pub const IOC_PE06_FUNC_CTL_PWM3_P_2: u8 = 16;
    pub const IOC_PE06_FUNC_CTL_SDC0_VSEL: u8 = 17;
    pub const IOC_PE06_FUNC_CTL_UART13_DE: u8 = 2;
    pub const IOC_PE06_FUNC_CTL_UART13_RTS: u8 = 3;
    pub const IOC_PE07_FUNC_CTL_ETH0_RXD_3: u8 = 18;
    pub const IOC_PE07_FUNC_CTL_GPIO_E_07: u8 = 0;
    pub const IOC_PE07_FUNC_CTL_PWM3_P_1: u8 = 16;
    pub const IOC_PE07_FUNC_CTL_SPI1_DAT3: u8 = 5;
    pub const IOC_PE07_FUNC_CTL_UART13_RXD: u8 = 2;
    pub const IOC_PE07_FUNC_CTL_XPI0_CB_SCLK: u8 = 14;
    pub const IOC_PE08_FUNC_CTL_CAN2_RXD: u8 = 7;
    pub const IOC_PE08_FUNC_CTL_ETH0_MDC: u8 = 19;
    pub const IOC_PE08_FUNC_CTL_GPIO_E_08: u8 = 0;
    pub const IOC_PE08_FUNC_CTL_SDC0_DATA_1: u8 = 17;
    pub const IOC_PE08_FUNC_CTL_UART12_CTS: u8 = 3;
    pub const IOC_PE08_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PE09_FUNC_CTL_CAN2_TXD: u8 = 7;
    pub const IOC_PE09_FUNC_CTL_ETH0_MDIO: u8 = 19;
    pub const IOC_PE09_FUNC_CTL_GPIO_E_09: u8 = 0;
    pub const IOC_PE09_FUNC_CTL_SDC0_DATA_0: u8 = 17;
    pub const IOC_PE09_FUNC_CTL_UART12_DE: u8 = 2;
    pub const IOC_PE09_FUNC_CTL_UART12_RTS: u8 = 3;
    pub const IOC_PE09_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PE10_FUNC_CTL_CAN0_STBY: u8 = 7;
    pub const IOC_PE10_FUNC_CTL_ETH1_MDC: u8 = 19;
    pub const IOC_PE10_FUNC_CTL_GPIO_E_10: u8 = 0;
    pub const IOC_PE10_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PE10_FUNC_CTL_SDC0_CMD: u8 = 17;
    pub const IOC_PE10_FUNC_CTL_UART15_CTS: u8 = 3;
    pub const IOC_PE10_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
    pub const IOC_PE11_FUNC_CTL_CAN1_STBY: u8 = 7;
    pub const IOC_PE11_FUNC_CTL_ETH1_MDIO: u8 = 19;
    pub const IOC_PE11_FUNC_CTL_GPIO_E_11: u8 = 0;
    pub const IOC_PE11_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PE11_FUNC_CTL_SDC0_CLK: u8 = 17;
    pub const IOC_PE11_FUNC_CTL_UART15_DE: u8 = 2;
    pub const IOC_PE11_FUNC_CTL_UART15_RTS: u8 = 3;
    pub const IOC_PE11_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PE12_FUNC_CTL_CAN2_STBY: u8 = 7;
    pub const IOC_PE12_FUNC_CTL_GPIO_E_12: u8 = 0;
    pub const IOC_PE12_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PE12_FUNC_CTL_SDC0_DATA_3: u8 = 17;
    pub const IOC_PE12_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PE12_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PE13_FUNC_CTL_CAN3_STBY: u8 = 7;
    pub const IOC_PE13_FUNC_CTL_GPIO_E_13: u8 = 0;
    pub const IOC_PE13_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PE13_FUNC_CTL_SDC0_DATA_2: u8 = 17;
    pub const IOC_PE13_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PE13_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PE14_FUNC_CTL_ETH1_TXEN: u8 = 18;
    pub const IOC_PE14_FUNC_CTL_GPIO_E_14: u8 = 0;
    pub const IOC_PE14_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PE14_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PE14_FUNC_CTL_I2S0_TXD_2: u8 = 8;
    pub const IOC_PE14_FUNC_CTL_PWM3_P_5: u8 = 16;
    pub const IOC_PE14_FUNC_CTL_SDC1_WP: u8 = 17;
    pub const IOC_PE14_FUNC_CTL_UART14_CTS: u8 = 3;
    pub const IOC_PE15_FUNC_CTL_ETH0_EVTO_0: u8 = 19;
    pub const IOC_PE15_FUNC_CTL_ETH1_RXDV: u8 = 18;
    pub const IOC_PE15_FUNC_CTL_GPIO_E_15: u8 = 0;
    pub const IOC_PE15_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PE15_FUNC_CTL_I2S0_TXD_1: u8 = 8;
    pub const IOC_PE15_FUNC_CTL_SDC1_CDN: u8 = 17;
    pub const IOC_PE15_FUNC_CTL_UART14_DE: u8 = 2;
    pub const IOC_PE15_FUNC_CTL_UART14_RTS: u8 = 3;
    pub const IOC_PE16_FUNC_CTL_ETH1_REFCLK: u8 = 18;
    pub const IOC_PE16_FUNC_CTL_GPIO_E_16: u8 = 0;
    pub const IOC_PE16_FUNC_CTL_GPTMR2_COMP_1: u8 = 1;
    pub const IOC_PE16_FUNC_CTL_I2S0_MCLK: u8 = 8;
    pub const IOC_PE16_FUNC_CTL_PWM3_P_3: u8 = 16;
    pub const IOC_PE16_FUNC_CTL_SDC1_VSEL: u8 = 17;
    pub const IOC_PE16_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PE17_FUNC_CTL_ETH0_EVTO_2: u8 = 19;
    pub const IOC_PE17_FUNC_CTL_ETH1_TXD_1: u8 = 18;
    pub const IOC_PE17_FUNC_CTL_GPIO_E_17: u8 = 0;
    pub const IOC_PE17_FUNC_CTL_GPTMR3_CAPT_1: u8 = 1;
    pub const IOC_PE17_FUNC_CTL_I2S0_TXD_3: u8 = 8;
    pub const IOC_PE17_FUNC_CTL_PWM3_P_6: u8 = 16;
    pub const IOC_PE17_FUNC_CTL_SDC0_WP: u8 = 17;
    pub const IOC_PE17_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PE18_FUNC_CTL_ETH0_EVTO_1: u8 = 19;
    pub const IOC_PE18_FUNC_CTL_ETH1_RXD_1: u8 = 18;
    pub const IOC_PE18_FUNC_CTL_GPIO_E_18: u8 = 0;
    pub const IOC_PE18_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PE18_FUNC_CTL_I2S0_FCLK: u8 = 8;
    pub const IOC_PE18_FUNC_CTL_PWM3_P_7: u8 = 16;
    pub const IOC_PE18_FUNC_CTL_SDC0_CDN: u8 = 17;
    pub const IOC_PE18_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PE19_FUNC_CTL_ACMP_COMP_3: u8 = 16;
    pub const IOC_PE19_FUNC_CTL_ETH0_EVTI_2: u8 = 19;
    pub const IOC_PE19_FUNC_CTL_ETH1_TXD_0: u8 = 18;
    pub const IOC_PE19_FUNC_CTL_GPIO_E_19: u8 = 0;
    pub const IOC_PE19_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PE19_FUNC_CTL_I2S0_RXD_3: u8 = 8;
    pub const IOC_PE19_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PE20_FUNC_CTL_ACMP_COMP_0: u8 = 16;
    pub const IOC_PE20_FUNC_CTL_ETH0_EVTI_0: u8 = 19;
    pub const IOC_PE20_FUNC_CTL_ETH1_RXD_0: u8 = 18;
    pub const IOC_PE20_FUNC_CTL_GPIO_E_20: u8 = 0;
    pub const IOC_PE20_FUNC_CTL_I2S0_BCLK: u8 = 8;
    pub const IOC_PE20_FUNC_CTL_SDC0_VSEL: u8 = 17;
    pub const IOC_PE20_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PE21_FUNC_CTL_ETH0_RXDV: u8 = 18;
    pub const IOC_PE21_FUNC_CTL_GPIO_E_21: u8 = 0;
    pub const IOC_PE21_FUNC_CTL_GPTMR4_CAPT_0: u8 = 1;
    pub const IOC_PE21_FUNC_CTL_I2S0_TXD_0: u8 = 8;
    pub const IOC_PE21_FUNC_CTL_PDM0_D_3: u8 = 10;
    pub const IOC_PE21_FUNC_CTL_SDC0_DATA_1: u8 = 17;
    pub const IOC_PE21_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PE22_FUNC_CTL_ETH0_MDC: u8 = 19;
    pub const IOC_PE22_FUNC_CTL_ETH0_RXD_1: u8 = 18;
    pub const IOC_PE22_FUNC_CTL_GPIO_E_22: u8 = 0;
    pub const IOC_PE22_FUNC_CTL_GPTMR4_COMP_0: u8 = 1;
    pub const IOC_PE22_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PE22_FUNC_CTL_PDM0_D_1: u8 = 10;
    pub const IOC_PE22_FUNC_CTL_SDC0_CMD: u8 = 17;
    pub const IOC_PE22_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PE23_FUNC_CTL_ETH0_MDIO: u8 = 19;
    pub const IOC_PE23_FUNC_CTL_ETH0_RXD_0: u8 = 18;
    pub const IOC_PE23_FUNC_CTL_GPIO_E_23: u8 = 0;
    pub const IOC_PE23_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PE23_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PE23_FUNC_CTL_PDM0_CLK: u8 = 10;
    pub const IOC_PE23_FUNC_CTL_SDC0_DATA_3: u8 = 17;
    pub const IOC_PE23_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PE24_FUNC_CTL_ACMP_COMP_2: u8 = 16;
    pub const IOC_PE24_FUNC_CTL_GPIO_E_24: u8 = 0;
    pub const IOC_PE24_FUNC_CTL_GPTMR3_COMP_1: u8 = 1;
    pub const IOC_PE24_FUNC_CTL_I2S0_RXD_2: u8 = 8;
    pub const IOC_PE24_FUNC_CTL_SOC_REF1: u8 = 24;
    pub const IOC_PE24_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PE25_FUNC_CTL_ACMP_COMP_1: u8 = 16;
    pub const IOC_PE25_FUNC_CTL_ETH0_EVTI_1: u8 = 19;
    pub const IOC_PE25_FUNC_CTL_GPIO_E_25: u8 = 0;
    pub const IOC_PE25_FUNC_CTL_GPTMR4_CAPT_1: u8 = 1;
    pub const IOC_PE25_FUNC_CTL_I2S0_RXD_1: u8 = 8;
    pub const IOC_PE25_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PE26_FUNC_CTL_CAN3_TXD: u8 = 7;
    pub const IOC_PE26_FUNC_CTL_ETH0_TXEN: u8 = 18;
    pub const IOC_PE26_FUNC_CTL_GPIO_E_26: u8 = 0;
    pub const IOC_PE26_FUNC_CTL_I2S0_RXD_0: u8 = 8;
    pub const IOC_PE26_FUNC_CTL_SDC0_DATA_0: u8 = 17;
    pub const IOC_PE26_FUNC_CTL_SPI2_DAT3: u8 = 5;
    pub const IOC_PE26_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PE26_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PE27_FUNC_CTL_CAN0_RXD: u8 = 7;
    pub const IOC_PE27_FUNC_CTL_ETH0_TXD_1: u8 = 18;
    pub const IOC_PE27_FUNC_CTL_ETH1_MDC: u8 = 19;
    pub const IOC_PE27_FUNC_CTL_GPIO_E_27: u8 = 0;
    pub const IOC_PE27_FUNC_CTL_GPTMR4_COMP_1: u8 = 1;
    pub const IOC_PE27_FUNC_CTL_SDC0_CLK: u8 = 17;
    pub const IOC_PE27_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PE27_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PE28_FUNC_CTL_CAN0_TXD: u8 = 7;
    pub const IOC_PE28_FUNC_CTL_ETH0_TXD_0: u8 = 18;
    pub const IOC_PE28_FUNC_CTL_ETH1_MDIO: u8 = 19;
    pub const IOC_PE28_FUNC_CTL_GPIO_E_28: u8 = 0;
    pub const IOC_PE28_FUNC_CTL_GPTMR2_CAPT_1: u8 = 1;
    pub const IOC_PE28_FUNC_CTL_SDC0_DATA_2: u8 = 17;
    pub const IOC_PE28_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PE28_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PE29_FUNC_CTL_CAN3_RXD: u8 = 7;
    pub const IOC_PE29_FUNC_CTL_GPIO_E_29: u8 = 0;
    pub const IOC_PE29_FUNC_CTL_PDM0_D_2: u8 = 10;
    pub const IOC_PE29_FUNC_CTL_SPI2_DAT2: u8 = 5;
    pub const IOC_PE29_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PE29_FUNC_CTL_USB0_OC: u8 = 24;
    pub const IOC_PE30_FUNC_CTL_CAN1_RXD: u8 = 7;
    pub const IOC_PE30_FUNC_CTL_GPIO_E_30: u8 = 0;
    pub const IOC_PE30_FUNC_CTL_PDM0_D_0: u8 = 10;
    pub const IOC_PE30_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PE30_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PE30_FUNC_CTL_USB1_OC: u8 = 24;
    pub const IOC_PE31_FUNC_CTL_CAN1_TXD: u8 = 7;
    pub const IOC_PE31_FUNC_CTL_ETH0_REFCLK: u8 = 18;
    pub const IOC_PE31_FUNC_CTL_GPIO_E_31: u8 = 0;
    pub const IOC_PE31_FUNC_CTL_PDM0_CLK: u8 = 10;
    pub const IOC_PE31_FUNC_CTL_SPI2_CSN: u8 = 5;
    pub const IOC_PE31_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PF00_FUNC_CTL_CAN2_TXD: u8 = 7;
    pub const IOC_PF00_FUNC_CTL_ETH0_EVTI_0: u8 = 19;
    pub const IOC_PF00_FUNC_CTL_GPIO_F_00: u8 = 0;
    pub const IOC_PF00_FUNC_CTL_I2S0_TXD_3: u8 = 9;
    pub const IOC_PF00_FUNC_CTL_PDM0_D_1: u8 = 10;
    pub const IOC_PF00_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PF00_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PF01_FUNC_CTL_ETH0_EVTI_1: u8 = 19;
    pub const IOC_PF01_FUNC_CTL_GPIO_F_01: u8 = 0;
    pub const IOC_PF01_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PF01_FUNC_CTL_I2S0_RXD_3: u8 = 9;
    pub const IOC_PF01_FUNC_CTL_PDM0_D_0: u8 = 10;
    pub const IOC_PF01_FUNC_CTL_UART3_DE: u8 = 2;
    pub const IOC_PF01_FUNC_CTL_UART3_RTS: u8 = 3;
    pub const IOC_PF02_FUNC_CTL_CAN2_RXD: u8 = 7;
    pub const IOC_PF02_FUNC_CTL_ETH0_EVTI_2: u8 = 19;
    pub const IOC_PF02_FUNC_CTL_GPIO_F_02: u8 = 0;
    pub const IOC_PF02_FUNC_CTL_I2S0_RXD_2: u8 = 9;
    pub const IOC_PF02_FUNC_CTL_PDM0_D_3: u8 = 10;
    pub const IOC_PF02_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PF03_FUNC_CTL_GPIO_F_03: u8 = 0;
    pub const IOC_PF03_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PF03_FUNC_CTL_I2S0_MCLK: u8 = 9;
    pub const IOC_PF03_FUNC_CTL_PDM0_D_2: u8 = 10;
    pub const IOC_PF03_FUNC_CTL_SPI3_CSN: u8 = 5;
    pub const IOC_PF03_FUNC_CTL_UART3_CTS: u8 = 3;
    pub const IOC_PF04_FUNC_CTL_GPIO_F_04: u8 = 0;
    pub const IOC_PF04_FUNC_CTL_GPTMR5_COMP_0: u8 = 1;
    pub const IOC_PF04_FUNC_CTL_I2S0_TXD_2: u8 = 9;
    pub const IOC_PF04_FUNC_CTL_PDM0_CLK: u8 = 10;
    pub const IOC_PF04_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PF05_FUNC_CTL_ETH0_EVTO_0: u8 = 19;
    pub const IOC_PF05_FUNC_CTL_GPIO_F_05: u8 = 0;
    pub const IOC_PF05_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PF05_FUNC_CTL_I2S0_RXD_1: u8 = 9;
    pub const IOC_PF05_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PF05_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PF05_FUNC_CTL_USB1_OC: u8 = 24;
    pub const IOC_PF06_FUNC_CTL_ETH0_EVTO_1: u8 = 19;
    pub const IOC_PF06_FUNC_CTL_GPIO_F_06: u8 = 0;
    pub const IOC_PF06_FUNC_CTL_GPTMR5_CAPT_1: u8 = 1;
    pub const IOC_PF06_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PF06_FUNC_CTL_I2S0_BCLK: u8 = 9;
    pub const IOC_PF06_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PF06_FUNC_CTL_UART8_RXD: u8 = 2;
    pub const IOC_PF06_FUNC_CTL_USB1_PWR: u8 = 24;
    pub const IOC_PF07_FUNC_CTL_GPIO_F_07: u8 = 0;
    pub const IOC_PF07_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PF07_FUNC_CTL_I2S0_TXD_1: u8 = 9;
    pub const IOC_PF07_FUNC_CTL_PDM0_CLK: u8 = 10;
    pub const IOC_PF07_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PF07_FUNC_CTL_UART8_TXD: u8 = 2;
    pub const IOC_PF07_FUNC_CTL_USB1_ID: u8 = 24;
    pub const IOC_PF08_FUNC_CTL_GPIO_F_08: u8 = 0;
    pub const IOC_PF08_FUNC_CTL_GPTMR5_CAPT_0: u8 = 1;
    pub const IOC_PF08_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PF08_FUNC_CTL_I2S0_RXD_0: u8 = 9;
    pub const IOC_PF08_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PF08_FUNC_CTL_USB0_OC: u8 = 24;
    pub const IOC_PF09_FUNC_CTL_ETH0_EVTO_2: u8 = 19;
    pub const IOC_PF09_FUNC_CTL_GPIO_F_09: u8 = 0;
    pub const IOC_PF09_FUNC_CTL_GPTMR5_COMP_1: u8 = 1;
    pub const IOC_PF09_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PF09_FUNC_CTL_I2S0_FCLK: u8 = 9;
    pub const IOC_PF09_FUNC_CTL_SPI3_DAT3: u8 = 5;
    pub const IOC_PF09_FUNC_CTL_UART9_RXD: u8 = 2;
    pub const IOC_PF09_FUNC_CTL_USB0_PWR: u8 = 24;
    pub const IOC_PF10_FUNC_CTL_GPIO_F_10: u8 = 0;
    pub const IOC_PF10_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PF10_FUNC_CTL_I2S0_TXD_0: u8 = 9;
    pub const IOC_PF10_FUNC_CTL_SPI3_DAT2: u8 = 5;
    pub const IOC_PF10_FUNC_CTL_UART9_TXD: u8 = 2;
    pub const IOC_PF10_FUNC_CTL_USB0_ID: u8 = 24;
    pub const IOC_PX00_FUNC_CTL_GPIO_X_00: u8 = 0;
    pub const IOC_PX00_FUNC_CTL_SDC1_DATA_3: u8 = 17;
    pub const IOC_PX00_FUNC_CTL_XPI0_CB_D_3: u8 = 14;
    pub const IOC_PX01_FUNC_CTL_FEMC_DQS: u8 = 12;
    pub const IOC_PX01_FUNC_CTL_GPIO_X_01: u8 = 0;
    pub const IOC_PX01_FUNC_CTL_SDC1_DATA_2: u8 = 17;
    pub const IOC_PX01_FUNC_CTL_XPI0_CB_D_2: u8 = 14;
    pub const IOC_PX02_FUNC_CTL_GPIO_X_02: u8 = 0;
    pub const IOC_PX02_FUNC_CTL_XPI0_CA_DQS: u8 = 24;
    pub const IOC_PX02_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
    pub const IOC_PX03_FUNC_CTL_GPIO_X_03: u8 = 0;
    pub const IOC_PX03_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PX03_FUNC_CTL_XPI1_CA_DQS: u8 = 24;
    pub const IOC_PX04_FUNC_CTL_GPIO_X_04: u8 = 0;
    pub const IOC_PX04_FUNC_CTL_SDC1_CMD: u8 = 17;
    pub const IOC_PX04_FUNC_CTL_XPI0_CA_CS1: u8 = 14;
    pub const IOC_PX04_FUNC_CTL_XPI1_CA_DQS: u8 = 24;
    pub const IOC_PX05_FUNC_CTL_GPIO_X_05: u8 = 0;
    pub const IOC_PX05_FUNC_CTL_SDC1_CLK: u8 = 17;
    pub const IOC_PX05_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PX06_FUNC_CTL_FEMC_DQS: u8 = 12;
    pub const IOC_PX06_FUNC_CTL_GPIO_X_06: u8 = 0;
    pub const IOC_PX06_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PX07_FUNC_CTL_GPIO_X_07: u8 = 0;
    pub const IOC_PX07_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PX08_FUNC_CTL_GPIO_X_08: u8 = 0;
    pub const IOC_PX08_FUNC_CTL_SDC1_DATA_1: u8 = 17;
    pub const IOC_PX08_FUNC_CTL_XPI0_CB_D_1: u8 = 14;
    pub const IOC_PX09_FUNC_CTL_GPIO_X_09: u8 = 0;
    pub const IOC_PX09_FUNC_CTL_SDC1_DATA_0: u8 = 17;
    pub const IOC_PX09_FUNC_CTL_XPI0_CB_D_0: u8 = 14;
    pub const IOC_PX10_FUNC_CTL_GPIO_X_10: u8 = 0;
    pub const IOC_PX10_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PX11_FUNC_CTL_GPIO_X_11: u8 = 0;
    pub const IOC_PX11_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PY00_FUNC_CTL_GPIO_Y_00: u8 = 0;
    pub const IOC_PY00_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PY00_FUNC_CTL_I2S1_TXD_1: u8 = 8;
    pub const IOC_PY00_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PY00_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PY00_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PY01_FUNC_CTL_GPIO_Y_01: u8 = 0;
    pub const IOC_PY01_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PY01_FUNC_CTL_I2S1_FCLK: u8 = 8;
    pub const IOC_PY01_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PY01_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PY02_FUNC_CTL_CAN0_TXD: u8 = 7;
    pub const IOC_PY02_FUNC_CTL_GPIO_Y_02: u8 = 0;
    pub const IOC_PY02_FUNC_CTL_I2S1_MCLK: u8 = 8;
    pub const IOC_PY02_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PY02_FUNC_CTL_UART6_DE: u8 = 2;
    pub const IOC_PY02_FUNC_CTL_UART6_RTS: u8 = 3;
    pub const IOC_PY03_FUNC_CTL_CAN0_RXD: u8 = 7;
    pub const IOC_PY03_FUNC_CTL_GPIO_Y_03: u8 = 0;
    pub const IOC_PY03_FUNC_CTL_SPI3_CSN: u8 = 5;
    pub const IOC_PY03_FUNC_CTL_UART6_CTS: u8 = 3;
    pub const IOC_PY04_FUNC_CTL_ACMP_COMP_1: u8 = 16;
    pub const IOC_PY04_FUNC_CTL_CAN2_STBY: u8 = 7;
    pub const IOC_PY04_FUNC_CTL_DAOL_P: u8 = 10;
    pub const IOC_PY04_FUNC_CTL_GPIO_Y_04: u8 = 0;
    pub const IOC_PY04_FUNC_CTL_GPTMR6_COMP_1: u8 = 1;
    pub const IOC_PY04_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PY04_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PY04_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PY04_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PY05_FUNC_CTL_ACMP_COMP_0: u8 = 16;
    pub const IOC_PY05_FUNC_CTL_CAN0_STBY: u8 = 7;
    pub const IOC_PY05_FUNC_CTL_DAOL_N: u8 = 10;
    pub const IOC_PY05_FUNC_CTL_GPIO_Y_05: u8 = 0;
    pub const IOC_PY05_FUNC_CTL_GPTMR6_CAPT_1: u8 = 1;
    pub const IOC_PY05_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PY05_FUNC_CTL_SPI1_CSN: u8 = 5;
    pub const IOC_PY05_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PY06_FUNC_CTL_CAN1_TXD: u8 = 7;
    pub const IOC_PY06_FUNC_CTL_GPIO_Y_06: u8 = 0;
    pub const IOC_PY06_FUNC_CTL_I2S1_TXD_0: u8 = 8;
    pub const IOC_PY06_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PY07_FUNC_CTL_CAN1_RXD: u8 = 7;
    pub const IOC_PY07_FUNC_CTL_GPIO_Y_07: u8 = 0;
    pub const IOC_PY07_FUNC_CTL_I2S1_BCLK: u8 = 8;
    pub const IOC_PY07_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PY08_FUNC_CTL_ACMP_COMP_2: u8 = 16;
    pub const IOC_PY08_FUNC_CTL_DAOR_P: u8 = 10;
    pub const IOC_PY08_FUNC_CTL_GPIO_Y_08: u8 = 0;
    pub const IOC_PY08_FUNC_CTL_GPTMR6_COMP_0: u8 = 1;
    pub const IOC_PY08_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PY08_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PY09_FUNC_CTL_ACMP_COMP_3: u8 = 16;
    pub const IOC_PY09_FUNC_CTL_DAOR_N: u8 = 10;
    pub const IOC_PY09_FUNC_CTL_GPIO_Y_09: u8 = 0;
    pub const IOC_PY09_FUNC_CTL_GPTMR6_CAPT_0: u8 = 1;
    pub const IOC_PY09_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PY09_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PY10_FUNC_CTL_CAN2_TXD: u8 = 7;
    pub const IOC_PY10_FUNC_CTL_GPIO_Y_10: u8 = 0;
    pub const IOC_PY10_FUNC_CTL_I2S1_RXD_1: u8 = 8;
    pub const IOC_PY10_FUNC_CTL_PDM0_CLK: u8 = 10;
    pub const IOC_PY10_FUNC_CTL_UART7_DE: u8 = 2;
    pub const IOC_PY10_FUNC_CTL_UART7_RTS: u8 = 3;
    pub const IOC_PY11_FUNC_CTL_CAN2_RXD: u8 = 7;
    pub const IOC_PY11_FUNC_CTL_GPIO_Y_11: u8 = 0;
    pub const IOC_PY11_FUNC_CTL_I2S1_RXD_0: u8 = 8;
    pub const IOC_PY11_FUNC_CTL_PDM0_D_0: u8 = 10;
    pub const IOC_PY11_FUNC_CTL_UART7_CTS: u8 = 3;
    pub const IOC_PZ00_FUNC_CTL_GPIO_Z_00: u8 = 0;
    pub const IOC_PZ00_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PZ00_FUNC_CTL_UART15_TXD: u8 = 2;
    pub const IOC_PZ01_FUNC_CTL_GPIO_Z_01: u8 = 0;
    pub const IOC_PZ01_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PZ01_FUNC_CTL_UART15_RXD: u8 = 2;
    pub const IOC_PZ02_FUNC_CTL_GPIO_Z_02: u8 = 0;
    pub const IOC_PZ02_FUNC_CTL_PDM0_D_1: u8 = 10;
    pub const IOC_PZ02_FUNC_CTL_SPI0_CSN: u8 = 5;
    pub const IOC_PZ02_FUNC_CTL_UART10_RXD: u8 = 2;
    pub const IOC_PZ03_FUNC_CTL_GPIO_Z_03: u8 = 0;
    pub const IOC_PZ03_FUNC_CTL_PDM0_D_0: u8 = 10;
    pub const IOC_PZ03_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PZ03_FUNC_CTL_UART10_TXD: u8 = 2;
    pub const IOC_PZ04_FUNC_CTL_GPIO_Z_04: u8 = 0;
    pub const IOC_PZ04_FUNC_CTL_GPTMR7_CAPT_0: u8 = 1;
    pub const IOC_PZ04_FUNC_CTL_I2S0_FCLK: u8 = 8;
    pub const IOC_PZ04_FUNC_CTL_PDM0_D_3: u8 = 10;
    pub const IOC_PZ04_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PZ04_FUNC_CTL_UART11_RXD: u8 = 2;
    pub const IOC_PZ05_FUNC_CTL_GPIO_Z_05: u8 = 0;
    pub const IOC_PZ05_FUNC_CTL_PDM0_D_2: u8 = 10;
    pub const IOC_PZ05_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PZ05_FUNC_CTL_UART11_TXD: u8 = 2;
    pub const IOC_PZ06_FUNC_CTL_GPIO_Z_06: u8 = 0;
    pub const IOC_PZ06_FUNC_CTL_GPTMR7_COMP_1: u8 = 1;
    pub const IOC_PZ06_FUNC_CTL_I2S0_BCLK: u8 = 8;
    pub const IOC_PZ06_FUNC_CTL_PDM0_CLK: u8 = 10;
    pub const IOC_PZ06_FUNC_CTL_UART12_RXD: u8 = 2;
    pub const IOC_PZ07_FUNC_CTL_GPIO_Z_07: u8 = 0;
    pub const IOC_PZ07_FUNC_CTL_I2S0_TXD_0: u8 = 8;
    pub const IOC_PZ07_FUNC_CTL_PDM0_CLK: u8 = 10;
    pub const IOC_PZ07_FUNC_CTL_UART12_TXD: u8 = 2;
    pub const IOC_PZ08_FUNC_CTL_ACMP_COMP_0: u8 = 16;
    pub const IOC_PZ08_FUNC_CTL_GPIO_Z_08: u8 = 0;
    pub const IOC_PZ08_FUNC_CTL_GPTMR7_COMP_0: u8 = 1;
    pub const IOC_PZ08_FUNC_CTL_I2S0_RXD_0: u8 = 8;
    pub const IOC_PZ08_FUNC_CTL_UART13_RXD: u8 = 2;
    pub const IOC_PZ09_FUNC_CTL_ACMP_COMP_1: u8 = 16;
    pub const IOC_PZ09_FUNC_CTL_GPIO_Z_09: u8 = 0;
    pub const IOC_PZ09_FUNC_CTL_I2S0_MCLK: u8 = 8;
    pub const IOC_PZ09_FUNC_CTL_UART13_TXD: u8 = 2;
    pub const IOC_PZ10_FUNC_CTL_ACMP_COMP_3: u8 = 16;
    pub const IOC_PZ10_FUNC_CTL_CAN1_STBY: u8 = 7;
    pub const IOC_PZ10_FUNC_CTL_GPIO_Z_10: u8 = 0;
    pub const IOC_PZ10_FUNC_CTL_GPTMR7_CAPT_1: u8 = 1;
    pub const IOC_PZ10_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PZ10_FUNC_CTL_UART14_RXD: u8 = 2;
    pub const IOC_PZ11_FUNC_CTL_ACMP_COMP_2: u8 = 16;
    pub const IOC_PZ11_FUNC_CTL_CAN3_STBY: u8 = 7;
    pub const IOC_PZ11_FUNC_CTL_GPIO_Z_11: u8 = 0;
    pub const IOC_PZ11_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PZ11_FUNC_CTL_UART14_TXD: u8 = 2;
    pub const PIOC_PY00_FUNC_CTL_JTAG_TDO: u8 = 1;
    pub const PIOC_PY00_FUNC_CTL_PGPIO_Y_00: u8 = 0;
    pub const PIOC_PY00_FUNC_CTL_PTMR_COMP_0: u8 = 2;
    pub const PIOC_PY00_FUNC_CTL_SOC_PY_00: u8 = 3;
    pub const PIOC_PY01_FUNC_CTL_JTAG_TDI: u8 = 1;
    pub const PIOC_PY01_FUNC_CTL_PGPIO_Y_01: u8 = 0;
    pub const PIOC_PY01_FUNC_CTL_PTMR_COMP_1: u8 = 2;
    pub const PIOC_PY01_FUNC_CTL_SOC_PY_01: u8 = 3;
    pub const PIOC_PY02_FUNC_CTL_JTAG_TCK: u8 = 1;
    pub const PIOC_PY02_FUNC_CTL_PGPIO_Y_02: u8 = 0;
    pub const PIOC_PY02_FUNC_CTL_PTMR_COMP_2: u8 = 2;
    pub const PIOC_PY02_FUNC_CTL_SOC_PY_02: u8 = 3;
    pub const PIOC_PY03_FUNC_CTL_JTAG_TMS: u8 = 1;
    pub const PIOC_PY03_FUNC_CTL_PGPIO_Y_03: u8 = 0;
    pub const PIOC_PY03_FUNC_CTL_PTMR_COMP_3: u8 = 2;
    pub const PIOC_PY03_FUNC_CTL_SOC_PY_03: u8 = 3;
    pub const PIOC_PY04_FUNC_CTL_JTAG_TRST: u8 = 1;
    pub const PIOC_PY04_FUNC_CTL_PGPIO_Y_04: u8 = 0;
    pub const PIOC_PY04_FUNC_CTL_PTMR_COMP_0: u8 = 2;
    pub const PIOC_PY04_FUNC_CTL_SOC_PY_04: u8 = 3;
    pub const PIOC_PY05_FUNC_CTL_PGPIO_Y_05: u8 = 0;
    pub const PIOC_PY05_FUNC_CTL_PTMR_CAPT_0: u8 = 2;
    pub const PIOC_PY05_FUNC_CTL_PWDG_RST: u8 = 1;
    pub const PIOC_PY05_FUNC_CTL_SOC_PY_05: u8 = 3;
    pub const PIOC_PY06_FUNC_CTL_PGPIO_Y_06: u8 = 0;
    pub const PIOC_PY06_FUNC_CTL_PTMR_COMP_1: u8 = 2;
    pub const PIOC_PY06_FUNC_CTL_PUART_TXD: u8 = 1;
    pub const PIOC_PY06_FUNC_CTL_SOC_PY_06: u8 = 3;
    pub const PIOC_PY07_FUNC_CTL_PGPIO_Y_07: u8 = 0;
    pub const PIOC_PY07_FUNC_CTL_PTMR_CAPT_1: u8 = 2;
    pub const PIOC_PY07_FUNC_CTL_PUART_RXD: u8 = 1;
    pub const PIOC_PY07_FUNC_CTL_SOC_PY_07: u8 = 3;
    pub const PIOC_PY08_FUNC_CTL_PGPIO_Y_08: u8 = 0;
    pub const PIOC_PY08_FUNC_CTL_PTMR_COMP_2: u8 = 2;
    pub const PIOC_PY08_FUNC_CTL_PUART_RTS: u8 = 1;
    pub const PIOC_PY08_FUNC_CTL_SOC_PY_08: u8 = 3;
    pub const PIOC_PY09_FUNC_CTL_PGPIO_Y_09: u8 = 0;
    pub const PIOC_PY09_FUNC_CTL_PTMR_CAPT_2: u8 = 2;
    pub const PIOC_PY09_FUNC_CTL_PUART_CTS: u8 = 1;
    pub const PIOC_PY09_FUNC_CTL_SOC_PY_09: u8 = 3;
    pub const PIOC_PY10_FUNC_CTL_PGPIO_Y_10: u8 = 0;
    pub const PIOC_PY10_FUNC_CTL_PTMR_COMP_3: u8 = 2;
    pub const PIOC_PY10_FUNC_CTL_SOC_PY_10: u8 = 3;
    pub const PIOC_PY10_FUNC_CTL_VAD_CLK: u8 = 1;
    pub const PIOC_PY11_FUNC_CTL_PGPIO_Y_11: u8 = 0;
    pub const PIOC_PY11_FUNC_CTL_PTMR_CAPT_3: u8 = 2;
    pub const PIOC_PY11_FUNC_CTL_SOC_PY_11: u8 = 3;
    pub const PIOC_PY11_FUNC_CTL_VAD_DAT: u8 = 1;
}
pub mod trgmmux {
    //! `TRGMMUX` definitions
    pub const TRGM3_FILTER_SRC_TRGM3_IN10: usize = 18;
    pub const TRGM1_DMA_SRC_PWM1_CMP14: usize = 14;
    pub const TRGM2_DMA_SRC_PWM2_CMP21: usize = 21;
    pub const TRGM3_FILTER_SRC_TRGM3_IN2: usize = 10;
    pub const TRGM2_OUTPUT_SRC_TRGM2_P3: usize = 3;
    pub const TRGM3_INPUT_SRC_GPTMR7_OUT2: usize = 50;
    pub const TRGM1_INPUT_SRC_TRGM1_P5: usize = 7;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN8: usize = 22;
    pub const TRGM1_OUTPUT_SRC_ADCX_PTRGI1A: usize = 52;
    pub const TRGM3_OUTPUT_SRC_ADCX_PTRGI3C: usize = 54;
    pub const TRGM0_INPUT_SRC_DEBUG_FLAG: usize = 56;
    pub const TRGM2_FILTER_SRC_PWM2_IN6: usize = 6;
    pub const TRGM2_DMA_SRC_PWM2_CMP8: usize = 8;
    pub const TRGM1_INPUT_SRC_SYNT_CH3: usize = 47;
    pub const TRGM2_INPUT_SRC_DEBUG_FLAG: usize = 56;
    pub const TRGM2_DMA_SRC_PWM2_CMP1: usize = 1;
    pub const TRGM0_DMA_SRC_PWM0_HALFRLD: usize = 25;
    pub const TRGM2_OUTPUT_SRC_ADC0_STRGI: usize = 48;
    pub const TRGM2_DMA_SRC_PWM2_CMP2: usize = 2;
    pub const TRGM0_OUTPUT_SRC_HALL0_U: usize = 44;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN23: usize = 37;
    pub const TRGM2_INPUT_SRC_PTPC_CMP0: usize = 42;
    pub const TRGM1_INPUT_SRC_USB0_SOF: usize = 38;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN21: usize = 35;
    pub const TRGM0_OUTPUT_SRC_QEI0_PAUSE: usize = 42;
    pub const TRGM2_DMA_SRC_PWM2_CMP11: usize = 11;
    pub const TRGM0_INPUT_SRC_ENET1_PTP_OUT3: usize = 41;
    pub const TRGM2_INPUT_SRC_USB1_SOF: usize = 39;
    pub const TRGM1_OUTPUT_SRC_HALL1_U: usize = 44;
    pub const TRGM0_INPUT_SRC_PWM0_CH8REF: usize = 20;
    pub const TRGM3_INPUT_SRC_PWM3_CH22REF: usize = 34;
    pub const TRGM3_OUTPUT_SRC_PTPC_CAP0: usize = 62;
    pub const TRGM1_DMA_SRC_PWM1_CMP1: usize = 1;
    pub const TRGM1_INPUT_SRC_SYNT_CH0: usize = 44;
    pub const TRGM2_OUTPUT_SRC_TRGM2_P7: usize = 7;
    pub const TRGM0_INPUT_SRC_PWM0_CH18REF: usize = 30;
    pub const TRGM2_DMA_SRC_PWM2_CMP0: usize = 0;
    pub const TRGM1_DMA_SRC_PWM1_CMP16: usize = 16;
    pub const TRGM3_FILTER_SRC_PWM3_IN1: usize = 1;
    pub const TRGM0_DMA_SRC_PWM0_CMP8: usize = 8;
    pub const TRGM3_FILTER_SRC_PWM3_IN0: usize = 0;
    pub const TRGM0_INPUT_SRC_PWM0_CH23REF: usize = 35;
    pub const TRGM0_DMA_SRC_PWM0_CMP1: usize = 1;
    pub const TRGM1_INPUT_SRC_TRGM1_P0: usize = 2;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN21: usize = 35;
    pub const TRGM2_INPUT_SRC_TRGM1_OUTX1: usize = 17;
    pub const TRGM1_INPUT_SRC_SYNT_CH1: usize = 45;
    pub const TRGM3_INPUT_SRC_TRGM3_P7: usize = 9;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P9: usize = 9;
    pub const TRGM2_OUTPUT_SRC_TRGM2_P2: usize = 2;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN21: usize = 35;
    pub const TRGM2_INPUT_SRC_CMP0_OUT: usize = 52;
    pub const TRGM2_INPUT_SRC_TRGM2_P10: usize = 12;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN9: usize = 23;
    pub const TRGM0_FILTER_SRC_TRGM0_IN11: usize = 19;
    pub const TRGM0_DMA_SRC_PWM0_CMP6: usize = 6;
    pub const TRGM0_DMA_SRC_PWM0_CMP11: usize = 11;
    pub const TRGM0_DMA_SRC_PWM0_CMP14: usize = 14;
    pub const TRGM3_DMA_SRC_PWM3_CMP9: usize = 9;
    pub const TRGM3_DMA_SRC_PWM3_XRLD: usize = 26;
    pub const TRGM2_FILTER_SRC_TRGM2_IN11: usize = 19;
    pub const TRGM1_INPUT_SRC_PWM1_CH18REF: usize = 30;
    pub const TRGM2_DMA_SRC_PWM2_CMP17: usize = 17;
    pub const TRGM1_INPUT_SRC_PWM1_CH8REF: usize = 20;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN18: usize = 32;
    pub const TRGM0_INPUT_SRC_TRGM0_P10: usize = 12;
    pub const TRGM1_OUTPUT_SRC_GPTMR2_IN3: usize = 57;
    pub const TRGM1_DMA_SRC_PWM1_CMP23: usize = 23;
    pub const TRGM2_DMA_SRC_PWM2_CMP20: usize = 20;
    pub const TRGM3_DMA_SRC_PWM3_CMP2: usize = 2;
    pub const TRGM3_DMA_SRC_PWM3_HALFRLD: usize = 25;
    pub const TRGM0_INPUT_SRC_PWM0_CH15REF: usize = 27;
    pub const TRGM1_INPUT_SRC_CMP3_OUT: usize = 55;
    pub const TRGM1_OUTPUT_SRC_GPTMR3_IN3: usize = 60;
    pub const TRGM0_INPUT_SRC_PWM0_CH16REF: usize = 28;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN20: usize = 34;
    pub const TRGM1_OUTPUT_SRC_ADC3_STRGI: usize = 51;
    pub const TRGM3_OUTPUT_SRC_TRGM3_P2: usize = 2;
    pub const TRGM1_OUTPUT_SRC_ACMP1_WIN: usize = 61;
    pub const TRGM3_OUTPUT_SRC_ADC2_STRGI: usize = 50;
    pub const TRGM3_FILTER_SRC_PWM3_IN5: usize = 5;
    pub const TRGM0_INPUT_SRC_TRGM0_P7: usize = 9;
    pub const TRGM2_INPUT_SRC_USB0_SOF: usize = 38;
    pub const TRGM1_INPUT_SRC_GPTMR2_OUT3: usize = 49;
    pub const TRGM1_INPUT_SRC_TRGM1_P4: usize = 6;
    pub const TRGM2_OUTPUT_SRC_GPTMR5_IN3: usize = 60;
    pub const TRGM1_INPUT_SRC_GPTMR3_OUT3: usize = 51;
    pub const TRGM0_DMA_SRC_PWM0_CMP17: usize = 17;
    pub const TRGM1_DMA_SRC_PWM1_CMP6: usize = 6;
    pub const TRGM3_INPUT_SRC_TRGM3_P4: usize = 6;
    pub const TRGM3_OUTPUT_SRC_ADC1_STRGI: usize = 49;
    pub const TRGM3_INPUT_SRC_GPTMR7_OUT3: usize = 51;
    pub const TRGM2_INPUT_SRC_TRGM2_P7: usize = 9;
    pub const TRGM0_OUTPUT_SRC_QEI0_A: usize = 38;
    pub const TRGM0_OUTPUT_SRC_ACMP0_WIN: usize = 61;
    pub const TRGM0_OUTPUT_SRC_GPTMR1_IN2: usize = 59;
    pub const TRGM1_INPUT_SRC_PWM1_CH19REF: usize = 31;
    pub const TRGM1_OUTPUT_SRC_GPTMR2_IN2: usize = 56;
    pub const TRGM3_INPUT_SRC_PWM3_CH12REF: usize = 24;
    pub const TRGM0_INPUT_SRC_PWM0_CH14REF: usize = 26;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN15: usize = 29;
    pub const TRGM1_OUTPUT_SRC_QEI1_B: usize = 39;
    pub const TRGM1_FILTER_SRC_TRGM1_IN6: usize = 14;
    pub const TRGM3_INPUT_SRC_PWM3_CH23REF: usize = 35;
    pub const TRGM2_INPUT_SRC_PWM2_CH19REF: usize = 31;
    pub const TRGM2_INPUT_SRC_CMP2_OUT: usize = 54;
    pub const TRGM2_OUTPUT_SRC_TRGM2_P1: usize = 1;
    pub const TRGM0_OUTPUT_SRC_GPTMR0_IN3: usize = 57;
    pub const TRGM3_INPUT_SRC_TRGM3_P6: usize = 8;
    pub const TRGM0_OUTPUT_SRC_PWM0_FAULTI0: usize = 18;
    pub const TRGM2_DMA_SRC_PWM2_CMP15: usize = 15;
    pub const TRGM2_FILTER_SRC_TRGM2_IN7: usize = 15;
    pub const TRGM3_INPUT_SRC_QEI3_TRGO: usize = 36;
    pub const TRGM3_OUTPUT_SRC_QEI3_A: usize = 38;
    pub const TRGM0_FILTER_SRC_PWM0_IN5: usize = 5;
    pub const TRGM2_INPUT_SRC_PWM2_CH9REF: usize = 21;
    pub const TRGM0_INPUT_SRC_TRGM0_P1: usize = 3;
    pub const TRGM2_OUTPUT_SRC_TRGM2_OUTX0: usize = 12;
    pub const TRGM2_INPUT_SRC_GPTMR5_OUT2: usize = 50;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P8: usize = 8;
    pub const TRGM0_OUTPUT_SRC_PWM0_SYNCI: usize = 14;
    pub const TRGM1_OUTPUT_SRC_PWM1_FAULTI0: usize = 18;
    pub const TRGM0_INPUT_SRC_TRGM0_P6: usize = 8;
    pub const TRGM0_INPUT_SRC_SYNT0_CH1: usize = 45;
    pub const TRGM3_INPUT_SRC_HALL3_TRGO: usize = 37;
    pub const TRGM2_OUTPUT_SRC_GPTMR4_IN3: usize = 57;
    pub const TRGM3_OUTPUT_SRC_PWM3_FRCSYNCI: usize = 16;
    pub const TRGM3_OUTPUT_SRC_TRGM3_P9: usize = 9;
    pub const TRGM1_FILTER_SRC_PWM1_IN7: usize = 7;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN18: usize = 32;
    pub const TRGM2_OUTPUT_SRC_QEI2_Z: usize = 40;
    pub const TRGM3_OUTPUT_SRC_QEI3_PAUSE: usize = 42;
    pub const TRGM3_INPUT_SRC_TRGM0_OUTX0: usize = 18;
    pub const TRGM3_FILTER_SRC_TRGM3_IN7: usize = 15;
    pub const TRGM0_DMA_SRC_PWM0_CMP12: usize = 12;
    pub const TRGM0_DMA_SRC_QEI0: usize = 27;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN21: usize = 35;
    pub const TRGM0_INPUT_SRC_CMP3_OUT: usize = 55;
    pub const TRGM1_INPUT_SRC_PTPC_CMP1: usize = 43;
    pub const TRGM3_INPUT_SRC_PTPC_CMP1: usize = 43;
    pub const TRGM1_INPUT_SRC_PTPC_CMP0: usize = 42;
    pub const TRGM2_OUTPUT_SRC_ADCX_PTRGI2A: usize = 52;
    pub const TRGM3_OUTPUT_SRC_GPTMR7_IN2: usize = 59;
    pub const TRGM1_DMA_SRC_PWM1_CMP5: usize = 5;
    pub const TRGM2_DMA_SRC_PWM2_CMP6: usize = 6;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P10: usize = 10;
    pub const TRGM2_DMA_SRC_HALL2: usize = 28;
    pub const TRGM2_OUTPUT_SRC_PWM2_FRCSYNCI: usize = 16;
    pub const TRGM3_INPUT_SRC_SYNT_CH3: usize = 47;
    pub const TRGM0_OUTPUT_SRC_QEI0_H: usize = 41;
    pub const TRGM2_INPUT_SRC_SYNT_CH0: usize = 44;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN22: usize = 36;
    pub const TRGM1_INPUT_SRC_TRGM1_P8: usize = 10;
    pub const TRGM3_DMA_SRC_PWM3_RLD: usize = 24;
    pub const TRGM1_FILTER_SRC_TRGM1_IN9: usize = 17;
    pub const TRGM2_INPUT_SRC_SYNT_CH3: usize = 47;
    pub const TRGM3_DMA_SRC_PWM3_CMP18: usize = 18;
    pub const TRGM0_OUTPUT_SRC_ADC0_STRGI: usize = 48;
    pub const TRGM2_FILTER_SRC_PWM2_IN7: usize = 7;
    pub const TRGM2_OUTPUT_SRC_PWM2_SYNCI: usize = 14;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN13: usize = 27;
    pub const TRGM1_DMA_SRC_PWM1_CMP10: usize = 10;
    pub const TRGM1_OUTPUT_SRC_ADCX_PTRGI1B: usize = 53;
    pub const TRGM0_FILTER_SRC_TRGM0_IN4: usize = 12;
    pub const TRGM1_INPUT_SRC_TRGM1_P10: usize = 12;
    pub const TRGM2_DMA_SRC_PWM2_CMP22: usize = 22;
    pub const TRGM1_OUTPUT_SRC_QEI1_H: usize = 41;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P6: usize = 6;
    pub const TRGM0_OUTPUT_SRC_QEI0_SNAPI: usize = 43;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN10: usize = 24;
    pub const TRGM3_OUTPUT_SRC_HALL3_V: usize = 45;
    pub const TRGM2_DMA_SRC_PWM2_CMP12: usize = 12;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN9: usize = 23;
    pub const TRGM2_FILTER_SRC_PWM2_IN4: usize = 4;
    pub const TRGM0_FILTER_SRC_PWM0_IN4: usize = 4;
    pub const TRGM1_DMA_SRC_PWM1_CMP11: usize = 11;
    pub const TRGM2_INPUT_SRC_PWM2_CH10REF: usize = 22;
    pub const TRGM0_FILTER_SRC_PWM0_IN0: usize = 0;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P8: usize = 8;
    pub const TRGM3_OUTPUT_SRC_TRGM3_P3: usize = 3;
    pub const TRGM1_FILTER_SRC_PWM1_IN2: usize = 2;
    pub const TRGM2_INPUT_SRC_SYNT_CH1: usize = 45;
    pub const TRGM1_INPUT_SRC_DEBUG_FLAG: usize = 56;
    pub const TRGM1_INPUT_SRC_PWM1_CH15REF: usize = 27;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN9: usize = 23;
    pub const TRGM2_OUTPUT_SRC_ADC1_STRGI: usize = 49;
    pub const TRGM2_INPUT_SRC_TRGM2_P8: usize = 10;
    pub const TRGM3_OUTPUT_SRC_PWM3_SHRLDSYNCI: usize = 17;
    pub const TRGM3_OUTPUT_SRC_HALL3_W: usize = 46;
    pub const TRGM0_INPUT_SRC_PWM0_CH9REF: usize = 21;
    pub const TRGM1_FILTER_SRC_TRGM1_IN2: usize = 10;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P5: usize = 5;
    pub const TRGM2_OUTPUT_SRC_TRGM2_P5: usize = 5;
    pub const TRGM0_DMA_SRC_PWM0_CMP5: usize = 5;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P7: usize = 7;
    pub const TRGM2_INPUT_SRC_PWM2_CH12REF: usize = 24;
    pub const TRGM3_DMA_SRC_PWM3_CMP1: usize = 1;
    pub const TRGM3_OUTPUT_SRC_PWM3_FAULTI3: usize = 21;
    pub const TRGM1_DMA_SRC_PWM1_HALFRLD: usize = 25;
    pub const TRGM0_INPUT_SRC_HALL0_TRGO: usize = 37;
    pub const TRGM1_OUTPUT_SRC_ADC2_STRGI: usize = 50;
    pub const TRGM2_INPUT_SRC_PWM2_CH14REF: usize = 26;
    pub const TRGM2_INPUT_SRC_TRGM3_OUTX0: usize = 14;
    pub const TRGM3_INPUT_SRC_PWM3_CH18REF: usize = 30;
    pub const TRGM2_INPUT_SRC_PWM2_CH16REF: usize = 28;
    pub const TRGM3_OUTPUT_SRC_TRGM3_P10: usize = 10;
    pub const TRGM1_INPUT_SRC_TRGM1_P7: usize = 9;
    pub const TRGM0_OUTPUT_SRC_QEI0_Z: usize = 40;
    pub const TRGM2_OUTPUT_SRC_PWM2_FAULTI3: usize = 21;
    pub const TRGM1_INPUT_SRC_TRGM0_OUTX0: usize = 18;
    pub const TRGM3_OUTPUT_SRC_GPTMR6_IN3: usize = 57;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P6: usize = 6;
    pub const TRGM2_FILTER_SRC_PWM2_IN0: usize = 0;
    pub const TRGM1_INPUT_SRC_TRGM3_OUTX1: usize = 15;
    pub const TRGM2_OUTPUT_SRC_QEI2_PAUSE: usize = 42;
    pub const TRGM1_FILTER_SRC_TRGM1_IN11: usize = 19;
    pub const TRGM2_FILTER_SRC_TRGM2_IN9: usize = 17;
    pub const TRGM2_DMA_SRC_PWM2_CMP18: usize = 18;
    pub const TRGM1_DMA_SRC_HALL1: usize = 28;
    pub const TRGM2_DMA_SRC_QEI2: usize = 27;
    pub const TRGM0_OUTPUT_SRC_HALL0_SNAPI: usize = 47;
    pub const TRGM0_INPUT_SRC_GPTMR0_OUT3: usize = 49;
    pub const TRGM0_OUTPUT_SRC_GPTMR1_IN3: usize = 60;
    pub const TRGM3_INPUT_SRC_PWM3_CH21REF: usize = 33;
    pub const TRGM3_INPUT_SRC_CMP0_OUT: usize = 52;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN10: usize = 24;
    pub const TRGM2_OUTPUT_SRC_QEI2_SNAPI: usize = 43;
    pub const TRGM2_FILTER_SRC_TRGM2_IN3: usize = 11;
    pub const TRGM3_OUTPUT_SRC_QEI3_B: usize = 39;
    pub const TRGM1_DMA_SRC_PWM1_CMP22: usize = 22;
    pub const TRGM2_INPUT_SRC_GPTMR4_OUT3: usize = 49;
    pub const TRGM0_INPUT_SRC_TRGM0_P8: usize = 10;
    pub const TRGM3_INPUT_SRC_VSS: usize = 0;
    pub const TRGM2_INPUT_SRC_TRGM1_OUTX0: usize = 16;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN17: usize = 31;
    pub const TRGM3_DMA_SRC_HALL3: usize = 28;
    pub const TRGM0_OUTPUT_SRC_ADC1_STRGI: usize = 49;
    pub const TRGM0_OUTPUT_SRC_PWM0_FRCI: usize = 15;
    pub const TRGM0_OUTPUT_SRC_PWM0_FAULTI1: usize = 19;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN9: usize = 23;
    pub const TRGM0_OUTPUT_SRC_GPTMR0_IN2: usize = 56;
    pub const TRGM2_INPUT_SRC_PWM2_CH18REF: usize = 30;
    pub const TRGM1_FILTER_SRC_PWM1_IN3: usize = 3;
    pub const TRGM2_DMA_SRC_PWM2_CMP19: usize = 19;
    pub const TRGM0_INPUT_SRC_PTPC_CMP1: usize = 43;
    pub const TRGM0_INPUT_SRC_TRGM0_P11: usize = 13;
    pub const TRGM0_INPUT_SRC_PWM0_CH10REF: usize = 22;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN12: usize = 26;
    pub const TRGM1_OUTPUT_SRC_GPTMR3_IN2: usize = 59;
    pub const TRGM3_INPUT_SRC_PTPC_CMP0: usize = 42;
    pub const TRGM2_OUTPUT_SRC_TRGM2_P9: usize = 9;
    pub const TRGM2_DMA_SRC_PWM2_RLD: usize = 24;
    pub const TRGM2_DMA_SRC_PWM2_CMP5: usize = 5;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN19: usize = 33;
    pub const TRGM0_INPUT_SRC_PWM0_CH17REF: usize = 29;
    pub const TRGM2_OUTPUT_SRC_ADCX_PTRGI2C: usize = 54;
    pub const TRGM0_FILTER_SRC_PWM0_IN1: usize = 1;
    pub const TRGM0_FILTER_SRC_PWM0_IN2: usize = 2;
    pub const TRGM1_FILTER_SRC_TRGM1_IN10: usize = 18;
    pub const TRGM3_OUTPUT_SRC_GPTMR7_IN3: usize = 60;
    pub const TRGM3_DMA_SRC_PWM3_CMP3: usize = 3;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN19: usize = 33;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN8: usize = 22;
    pub const TRGM1_INPUT_SRC_PWM1_CH14REF: usize = 26;
    pub const TRGM2_OUTPUT_SRC_GPTMR5_IN2: usize = 59;
    pub const TRGM2_OUTPUT_SRC_HALL2_W: usize = 46;
    pub const TRGM2_DMA_SRC_PWM2_CMP7: usize = 7;
    pub const TRGM2_OUTPUT_SRC_ADCX_PTRGI2B: usize = 53;
    pub const TRGM1_OUTPUT_SRC_PWM1_FRCI: usize = 15;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P11: usize = 11;
    pub const TRGM3_FILTER_SRC_TRGM3_IN6: usize = 14;
    pub const TRGM3_OUTPUT_SRC_PWM3_FRCI: usize = 15;
    pub const TRGM2_INPUT_SRC_TRGM3_OUTX1: usize = 15;
    pub const TRGM0_DMA_SRC_PWM0_RLD: usize = 24;
    pub const TRGM3_FILTER_SRC_TRGM3_IN4: usize = 12;
    pub const TRGM2_OUTPUT_SRC_TRGM2_P6: usize = 6;
    pub const TRGM0_DMA_SRC_PWM0_CMP2: usize = 2;
    pub const TRGM1_FILTER_SRC_PWM1_IN4: usize = 4;
    pub const TRGM3_DMA_SRC_PWM3_CMP0: usize = 0;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN20: usize = 34;
    pub const TRGM3_DMA_SRC_QEI3: usize = 27;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN23: usize = 37;
    pub const TRGM0_FILTER_SRC_TRGM0_IN1: usize = 9;
    pub const TRGM2_OUTPUT_SRC_TRGM2_P0: usize = 0;
    pub const TRGM3_INPUT_SRC_SYNT_CH1: usize = 45;
    pub const TRGM2_OUTPUT_SRC_TRGM2_P8: usize = 8;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN22: usize = 36;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN23: usize = 37;
    pub const TRGM0_INPUT_SRC_TRGM0_P4: usize = 6;
    pub const TRGM3_OUTPUT_SRC_QEI3_Z: usize = 40;
    pub const TRGM1_DMA_SRC_PWM1_CMP13: usize = 13;
    pub const TRGM2_OUTPUT_SRC_HALL2_V: usize = 45;
    pub const TRGM1_OUTPUT_SRC_PWM1_FAULTI3: usize = 21;
    pub const TRGM1_INPUT_SRC_VSS: usize = 0;
    pub const TRGM3_INPUT_SRC_TRGM3_P5: usize = 7;
    pub const TRGM2_INPUT_SRC_TRGM2_P11: usize = 13;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN19: usize = 33;
    pub const TRGM2_INPUT_SRC_TRGM2_P3: usize = 5;
    pub const TRGM3_OUTPUT_SRC_ADCX_PTRGI3B: usize = 53;
    pub const TRGM3_DMA_SRC_PWM3_CMP4: usize = 4;
    pub const TRGM3_OUTPUT_SRC_TRGM3_P8: usize = 8;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN17: usize = 31;
    pub const TRGM2_INPUT_SRC_VSS: usize = 0;
    pub const TRGM2_INPUT_SRC_PWM2_CH8REF: usize = 20;
    pub const TRGM2_INPUT_SRC_PWM2_CH21REF: usize = 33;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN13: usize = 27;
    pub const TRGM3_OUTPUT_SRC_HALL3_U: usize = 44;
    pub const TRGM1_INPUT_SRC_GPTMR3_OUT2: usize = 50;
    pub const TRGM2_OUTPUT_SRC_PTPC_CAP0: usize = 62;
    pub const TRGM1_FILTER_SRC_TRGM1_IN3: usize = 11;
    pub const TRGM1_DMA_SRC_PWM1_CMP18: usize = 18;
    pub const TRGM1_DMA_SRC_PWM1_XRLD: usize = 26;
    pub const TRGM0_OUTPUT_SRC_PWM0_FAULTI3: usize = 21;
    pub const TRGM3_DMA_SRC_PWM3_CMP15: usize = 15;
    pub const TRGM1_INPUT_SRC_PWM1_CH20REF: usize = 32;
    pub const TRGM1_FILTER_SRC_TRGM1_IN8: usize = 16;
    pub const TRGM0_INPUT_SRC_TRGM3_OUTX1: usize = 15;
    pub const TRGM0_INPUT_SRC_TRGM3_OUTX0: usize = 14;
    pub const TRGM3_INPUT_SRC_PWM3_CH16REF: usize = 28;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN15: usize = 29;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P7: usize = 7;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN16: usize = 30;
    pub const TRGM0_INPUT_SRC_VSS: usize = 0;
    pub const TRGM1_INPUT_SRC_USB1_SOF: usize = 39;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P2: usize = 2;
    pub const TRGM2_FILTER_SRC_TRGM2_IN10: usize = 18;
    pub const TRGM3_FILTER_SRC_PWM3_IN4: usize = 4;
    pub const TRGM2_INPUT_SRC_TRGM2_P1: usize = 3;
    pub const TRGM1_INPUT_SRC_PWM1_CH13REF: usize = 25;
    pub const TRGM3_FILTER_SRC_TRGM3_IN1: usize = 9;
    pub const TRGM0_INPUT_SRC_TRGM1_OUTX0: usize = 18;
    pub const TRGM2_INPUT_SRC_PWM2_CH20REF: usize = 32;
    pub const TRGM3_INPUT_SRC_ENET0_PTP_OUT3: usize = 40;
    pub const TRGM1_DMA_SRC_PWM1_CMP3: usize = 3;
    pub const TRGM0_DMA_SRC_PWM0_CMP9: usize = 9;
    pub const TRGM1_INPUT_SRC_CMP0_OUT: usize = 52;
    pub const TRGM2_DMA_SRC_PWM2_CMP16: usize = 16;
    pub const TRGM3_OUTPUT_SRC_PWM3_SYNCI: usize = 14;
    pub const TRGM0_INPUT_SRC_QEI0_TRGO: usize = 36;
    pub const TRGM1_INPUT_SRC_PWM1_CH23REF: usize = 35;
    pub const TRGM2_INPUT_SRC_PWM2_CH22REF: usize = 34;
    pub const TRGM1_OUTPUT_SRC_PTPC_CAP0: usize = 62;
    pub const TRGM3_INPUT_SRC_CMP3_OUT: usize = 55;
    pub const TRGM2_DMA_SRC_PWM2_XRLD: usize = 26;
    pub const TRGM2_OUTPUT_SRC_QEI2_H: usize = 41;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN11: usize = 25;
    pub const TRGM2_INPUT_SRC_PWM2_CH23REF: usize = 35;
    pub const TRGM2_OUTPUT_SRC_PWM2_SHRLDSYNCI: usize = 17;
    pub const TRGM0_FILTER_SRC_TRGM0_IN7: usize = 15;
    pub const TRGM3_INPUT_SRC_SYNT_CH2: usize = 46;
    pub const TRGM0_FILTER_SRC_TRGM0_IN8: usize = 16;
    pub const TRGM0_INPUT_SRC_ENET0_PTP_OUT3: usize = 40;
    pub const TRGM0_DMA_SRC_PWM0_CMP10: usize = 10;
    pub const TRGM3_OUTPUT_SRC_PWM3_FAULTI1: usize = 19;
    pub const TRGM3_OUTPUT_SRC_TRGM3_P0: usize = 0;
    pub const TRGM0_INPUT_SRC_SYNT0_CH2: usize = 46;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN13: usize = 27;
    pub const TRGM1_DMA_SRC_PWM1_CMP12: usize = 12;
    pub const TRGM1_OUTPUT_SRC_PWM1_FAULTI2: usize = 20;
    pub const TRGM2_INPUT_SRC_PWM2_CH15REF: usize = 27;
    pub const TRGM1_INPUT_SRC_ENET1_PTP_OUT3: usize = 41;
    pub const TRGM1_OUTPUT_SRC_HALL1_V: usize = 45;
    pub const TRGM1_OUTPUT_SRC_PTPC_CAP1: usize = 63;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN14: usize = 28;
    pub const TRGM1_INPUT_SRC_TRGM1_P3: usize = 5;
    pub const TRGM3_DMA_SRC_PWM3_CMP21: usize = 21;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN22: usize = 36;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN18: usize = 32;
    pub const TRGM1_DMA_SRC_PWM1_CMP7: usize = 7;
    pub const TRGM3_INPUT_SRC_TRGM3_P8: usize = 10;
    pub const TRGM2_INPUT_SRC_PWM2_CH11REF: usize = 23;
    pub const TRGM0_OUTPUT_SRC_PTPC_CAP1: usize = 63;
    pub const TRGM0_INPUT_SRC_TRGM0_P9: usize = 11;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN14: usize = 28;
    pub const TRGM3_FILTER_SRC_TRGM3_IN9: usize = 17;
    pub const TRGM0_DMA_SRC_PWM0_CMP13: usize = 13;
    pub const TRGM3_INPUT_SRC_PWM3_CH15REF: usize = 27;
    pub const TRGM2_FILTER_SRC_TRGM2_IN1: usize = 9;
    pub const TRGM0_INPUT_SRC_USB0_SOF: usize = 38;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN16: usize = 30;
    pub const TRGM3_OUTPUT_SRC_QEI3_H: usize = 41;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN20: usize = 34;
    pub const TRGM0_FILTER_SRC_TRGM0_IN3: usize = 11;
    pub const TRGM3_OUTPUT_SRC_TRGM3_P4: usize = 4;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P4: usize = 4;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P11: usize = 11;
    pub const TRGM0_DMA_SRC_PWM0_CMP15: usize = 15;
    pub const TRGM3_INPUT_SRC_USB1_SOF: usize = 39;
    pub const TRGM0_OUTPUT_SRC_ADC3_STRGI: usize = 51;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN8: usize = 22;
    pub const TRGM2_OUTPUT_SRC_ADC2_STRGI: usize = 50;
    pub const TRGM1_INPUT_SRC_PWM1_CH22REF: usize = 34;
    pub const TRGM2_DMA_SRC_PWM2_CMP23: usize = 23;
    pub const TRGM3_INPUT_SRC_TRGM0_OUTX1: usize = 19;
    pub const TRGM3_FILTER_SRC_TRGM3_IN0: usize = 8;
    pub const TRGM0_OUTPUT_SRC_QEI0_B: usize = 39;
    pub const TRGM2_FILTER_SRC_TRGM2_IN4: usize = 12;
    pub const TRGM1_FILTER_SRC_TRGM1_IN5: usize = 13;
    pub const TRGM3_FILTER_SRC_TRGM3_IN8: usize = 16;
    pub const TRGM1_DMA_SRC_PWM1_CMP17: usize = 17;
    pub const TRGM1_DMA_SRC_PWM1_CMP4: usize = 4;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN11: usize = 25;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN16: usize = 30;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN11: usize = 25;
    pub const TRGM1_OUTPUT_SRC_QEI1_A: usize = 38;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN20: usize = 34;
    pub const TRGM0_INPUT_SRC_CMP0_OUT: usize = 52;
    pub const TRGM3_INPUT_SRC_TRGM3_P0: usize = 2;
    pub const TRGM0_INPUT_SRC_PTPC_CMP0: usize = 42;
    pub const TRGM3_INPUT_SRC_USB0_SOF: usize = 38;
    pub const TRGM1_DMA_SRC_PWM1_CMP8: usize = 8;
    pub const TRGM0_DMA_SRC_PWM0_CMP22: usize = 22;
    pub const TRGM0_DMA_SRC_PWM0_CMP19: usize = 19;
    pub const TRGM1_INPUT_SRC_ENET0_PTP_OUT3: usize = 40;
    pub const TRGM1_DMA_SRC_PWM1_CMP0: usize = 0;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI0B: usize = 53;
    pub const TRGM0_INPUT_SRC_PWM0_CH22REF: usize = 34;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN10: usize = 24;
    pub const TRGM2_FILTER_SRC_TRGM2_IN5: usize = 13;
    pub const TRGM2_INPUT_SRC_TRGM2_P2: usize = 4;
    pub const TRGM3_INPUT_SRC_TRGM1_OUTX0: usize = 16;
    pub const TRGM3_INPUT_SRC_GPTMR6_OUT3: usize = 49;
    pub const TRGM1_DMA_SRC_PWM1_CMP19: usize = 19;
    pub const TRGM2_INPUT_SRC_TRGM2_P6: usize = 8;
    pub const TRGM0_OUTPUT_SRC_PWM0_FAULTI2: usize = 20;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN17: usize = 31;
    pub const TRGM1_INPUT_SRC_PWM1_CH11REF: usize = 23;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P2: usize = 2;
    pub const TRGM3_INPUT_SRC_TRGM3_P1: usize = 3;
    pub const TRGM0_OUTPUT_SRC_PWM0_FRCSYNCI: usize = 16;
    pub const TRGM3_INPUT_SRC_CMP1_OUT: usize = 53;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN13: usize = 27;
    pub const TRGM0_DMA_SRC_PWM0_CMP18: usize = 18;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI0C: usize = 54;
    pub const TRGM2_FILTER_SRC_PWM2_IN2: usize = 2;
    pub const TRGM3_OUTPUT_SRC_GPTMR7_SYNCI: usize = 58;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN17: usize = 31;
    pub const TRGM2_INPUT_SRC_PWM2_CH13REF: usize = 25;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN11: usize = 25;
    pub const TRGM1_INPUT_SRC_TRGM1_P11: usize = 13;
    pub const TRGM1_FILTER_SRC_PWM1_IN0: usize = 0;
    pub const TRGM1_OUTPUT_SRC_ADCX_PTRGI1C: usize = 54;
    pub const TRGM1_OUTPUT_SRC_QEI1_SNAPI: usize = 43;
    pub const TRGM1_INPUT_SRC_PWM1_CH10REF: usize = 22;
    pub const TRGM0_OUTPUT_SRC_TRGM0_OUTX1: usize = 13;
    pub const TRGM3_INPUT_SRC_GPTMR6_OUT2: usize = 48;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN12: usize = 26;
    pub const TRGM3_OUTPUT_SRC_PTPC_CAP1: usize = 63;
    pub const TRGM3_OUTPUT_SRC_GPTMR6_SYNCI: usize = 55;
    pub const TRGM0_FILTER_SRC_PWM0_IN7: usize = 7;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN8: usize = 22;
    pub const TRGM3_OUTPUT_SRC_HALL3_SNAPI: usize = 47;
    pub const TRGM2_FILTER_SRC_PWM2_IN5: usize = 5;
    pub const TRGM3_INPUT_SRC_TRGM3_P10: usize = 12;
    pub const TRGM3_FILTER_SRC_TRGM3_IN11: usize = 19;
    pub const TRGM1_DMA_SRC_QEI1: usize = 27;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN18: usize = 32;
    pub const TRGM3_INPUT_SRC_TRGM3_P9: usize = 11;
    pub const TRGM0_INPUT_SRC_TRGM0_P2: usize = 4;
    pub const TRGM2_OUTPUT_SRC_PTPC_CAP1: usize = 63;
    pub const TRGM3_OUTPUT_SRC_ADC0_STRGI: usize = 48;
    pub const TRGM3_DMA_SRC_PWM3_CMP16: usize = 16;
    pub const TRGM2_OUTPUT_SRC_TRGM2_P10: usize = 10;
    pub const TRGM2_INPUT_SRC_ENET0_PTP_OUT3: usize = 40;
    pub const TRGM2_OUTPUT_SRC_QEI2_A: usize = 38;
    pub const TRGM1_OUTPUT_SRC_ADC1_STRGI: usize = 49;
    pub const TRGM2_FILTER_SRC_TRGM2_IN2: usize = 10;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI0A: usize = 52;
    pub const TRGM3_OUTPUT_SRC_TRGM3_P11: usize = 11;
    pub const TRGM0_INPUT_SRC_PWM0_CH20REF: usize = 32;
    pub const TRGM1_OUTPUT_SRC_HALL1_SNAPI: usize = 47;
    pub const TRGM3_INPUT_SRC_SYNT_CH0: usize = 44;
    pub const TRGM2_OUTPUT_SRC_TRGM2_P4: usize = 4;
    pub const TRGM3_DMA_SRC_PWM3_CMP5: usize = 5;
    pub const TRGM3_INPUT_SRC_TRGM2_OUTX1: usize = 15;
    pub const TRGM2_INPUT_SRC_TRGM0_OUTX1: usize = 19;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P4: usize = 4;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN23: usize = 37;
    pub const TRGM3_INPUT_SRC_PWM3_CH19REF: usize = 31;
    pub const TRGM3_INPUT_SRC_CMP2_OUT: usize = 54;
    pub const TRGM2_OUTPUT_SRC_PWM2_FRCI: usize = 15;
    pub const TRGM1_FILTER_SRC_PWM1_IN6: usize = 6;
    pub const TRGM0_FILTER_SRC_TRGM0_IN9: usize = 17;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN12: usize = 26;
    pub const TRGM1_FILTER_SRC_PWM1_IN1: usize = 1;
    pub const TRGM0_INPUT_SRC_PWM0_CH21REF: usize = 33;
    pub const TRGM2_INPUT_SRC_GPTMR5_OUT3: usize = 51;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P9: usize = 9;
    pub const TRGM3_OUTPUT_SRC_TRGM3_P5: usize = 5;
    pub const TRGM2_INPUT_SRC_TRGM2_P5: usize = 7;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P1: usize = 1;
    pub const TRGM3_OUTPUT_SRC_TRGM3_P6: usize = 6;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P10: usize = 10;
    pub const TRGM1_OUTPUT_SRC_PWM1_FAULTI1: usize = 19;
    pub const TRGM2_OUTPUT_SRC_TRGM2_OUTX1: usize = 13;
    pub const TRGM3_OUTPUT_SRC_PWM3_FAULTI0: usize = 18;
    pub const TRGM3_INPUT_SRC_TRGM3_P3: usize = 5;
    pub const TRGM0_FILTER_SRC_TRGM0_IN2: usize = 10;
    pub const TRGM2_FILTER_SRC_TRGM2_IN6: usize = 14;
    pub const TRGM0_OUTPUT_SRC_GPTMR0_SYNCI: usize = 55;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P0: usize = 0;
    pub const TRGM0_INPUT_SRC_PWM0_CH12REF: usize = 24;
    pub const TRGM2_OUTPUT_SRC_GPTMR4_IN2: usize = 56;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN19: usize = 33;
    pub const TRGM3_INPUT_SRC_PWM3_CH14REF: usize = 26;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P1: usize = 1;
    pub const TRGM1_INPUT_SRC_PWM1_CH17REF: usize = 29;
    pub const TRGM2_INPUT_SRC_PWM2_CH17REF: usize = 29;
    pub const TRGM2_INPUT_SRC_SYNT_CH2: usize = 46;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P3: usize = 3;
    pub const TRGM2_OUTPUT_SRC_ADC3_STRGI: usize = 51;
    pub const TRGM3_OUTPUT_SRC_QEI3_SNAPI: usize = 43;
    pub const TRGM0_DMA_SRC_PWM0_CMP16: usize = 16;
    pub const TRGM3_DMA_SRC_PWM3_CMP7: usize = 7;
    pub const TRGM3_DMA_SRC_PWM3_CMP11: usize = 11;
    pub const TRGM3_DMA_SRC_PWM3_CMP17: usize = 17;
    pub const TRGM2_INPUT_SRC_TRGM2_P4: usize = 6;
    pub const TRGM2_INPUT_SRC_PTPC_CMP1: usize = 43;
    pub const TRGM0_OUTPUT_SRC_ADC2_STRGI: usize = 50;
    pub const TRGM3_INPUT_SRC_TRGM3_P11: usize = 13;
    pub const TRGM0_INPUT_SRC_TRGM2_OUTX1: usize = 17;
    pub const TRGM2_OUTPUT_SRC_GPTMR4_SYNCI: usize = 55;
    pub const TRGM1_DMA_SRC_PWM1_CMP9: usize = 9;
    pub const TRGM3_DMA_SRC_PWM3_CMP20: usize = 20;
    pub const TRGM2_INPUT_SRC_GPTMR4_OUT2: usize = 48;
    pub const TRGM3_OUTPUT_SRC_ACMP3_WIN: usize = 61;
    pub const TRGM0_DMA_SRC_PWM0_CMP0: usize = 0;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN10: usize = 24;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN22: usize = 36;
    pub const TRGM3_DMA_SRC_PWM3_CMP22: usize = 22;
    pub const TRGM1_FILTER_SRC_TRGM1_IN0: usize = 8;
    pub const TRGM1_INPUT_SRC_CMP1_OUT: usize = 53;
    pub const TRGM3_INPUT_SRC_PWM3_CH20REF: usize = 32;
    pub const TRGM2_INPUT_SRC_TRGM2_P9: usize = 11;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN15: usize = 29;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN14: usize = 28;
    pub const TRGM3_DMA_SRC_PWM3_CMP6: usize = 6;
    pub const TRGM2_OUTPUT_SRC_TRGM2_P11: usize = 11;
    pub const TRGM3_DMA_SRC_PWM3_CMP14: usize = 14;
    pub const TRGM3_INPUT_SRC_PWM3_CH9REF: usize = 21;
    pub const TRGM2_OUTPUT_SRC_QEI2_B: usize = 39;
    pub const TRGM1_OUTPUT_SRC_PWM1_FRCSYNCI: usize = 16;
    pub const TRGM2_OUTPUT_SRC_HALL2_SNAPI: usize = 47;
    pub const TRGM1_INPUT_SRC_TRGM1_P1: usize = 3;
    pub const TRGM1_FILTER_SRC_TRGM1_IN1: usize = 9;
    pub const TRGM3_FILTER_SRC_PWM3_IN7: usize = 7;
    pub const TRGM1_INPUT_SRC_TRGM0_OUTX1: usize = 19;
    pub const TRGM2_OUTPUT_SRC_ACMP2_WIN: usize = 61;
    pub const TRGM0_INPUT_SRC_SYNT0_CH3: usize = 47;
    pub const TRGM0_DMA_SRC_PWM0_CMP3: usize = 3;
    pub const TRGM3_DMA_SRC_PWM3_CMP8: usize = 8;
    pub const TRGM1_INPUT_SRC_PWM1_CH12REF: usize = 24;
    pub const TRGM2_INPUT_SRC_TRGM0_OUTX0: usize = 18;
    pub const TRGM3_INPUT_SRC_DEBUG_FLAG: usize = 56;
    pub const TRGM0_FILTER_SRC_TRGM0_IN0: usize = 8;
    pub const TRGM1_DMA_SRC_PWM1_CMP21: usize = 21;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN14: usize = 28;
    pub const TRGM2_DMA_SRC_PWM2_CMP4: usize = 4;
    pub const TRGM1_INPUT_SRC_HALL1_TRGO: usize = 37;
    pub const TRGM0_INPUT_SRC_PWM0_CH11REF: usize = 23;
    pub const TRGM0_FILTER_SRC_PWM0_IN6: usize = 6;
    pub const TRGM0_DMA_SRC_PWM0_CMP21: usize = 21;
    pub const TRGM1_DMA_SRC_PWM1_CMP2: usize = 2;
    pub const TRGM2_INPUT_SRC_QEI2_TRGO: usize = 36;
    pub const TRGM0_INPUT_SRC_CMP2_OUT: usize = 54;
    pub const TRGM3_INPUT_SRC_PWM3_CH11REF: usize = 23;
    pub const TRGM3_INPUT_SRC_PWM3_CH13REF: usize = 25;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P3: usize = 3;
    pub const TRGM2_DMA_SRC_PWM2_CMP13: usize = 13;
    pub const TRGM1_OUTPUT_SRC_GPTMR3_SYNCI: usize = 58;
    pub const TRGM1_OUTPUT_SRC_PWM1_SYNCI: usize = 14;
    pub const TRGM0_DMA_SRC_PWM0_XRLD: usize = 26;
    pub const TRGM3_FILTER_SRC_PWM3_IN6: usize = 6;
    pub const TRGM3_INPUT_SRC_TRGM1_OUTX1: usize = 17;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P0: usize = 0;
    pub const TRGM1_INPUT_SRC_TRGM1_P6: usize = 8;
    pub const TRGM1_OUTPUT_SRC_HALL1_W: usize = 46;
    pub const TRGM2_OUTPUT_SRC_GPTMR5_SYNCI: usize = 58;
    pub const TRGM3_DMA_SRC_PWM3_CMP12: usize = 12;
    pub const TRGM2_OUTPUT_SRC_PWM2_FAULTI0: usize = 18;
    pub const TRGM0_OUTPUT_SRC_PTPC_CAP0: usize = 62;
    pub const TRGM1_INPUT_SRC_TRGM2_OUTX0: usize = 16;
    pub const TRGM2_OUTPUT_SRC_PWM2_IN12: usize = 26;
    pub const TRGM1_OUTPUT_SRC_PWM1_SHRLDSYNCI: usize = 17;
    pub const TRGM1_FILTER_SRC_TRGM1_IN7: usize = 15;
    pub const TRGM0_INPUT_SRC_PWM0_CH19REF: usize = 31;
    pub const TRGM0_INPUT_SRC_GPTMR1_OUT3: usize = 51;
    pub const TRGM3_INPUT_SRC_PWM3_CH8REF: usize = 20;
    pub const TRGM3_OUTPUT_SRC_TRGM3_OUTX0: usize = 12;
    pub const TRGM1_INPUT_SRC_PWM1_CH16REF: usize = 28;
    pub const TRGM2_INPUT_SRC_CMP1_OUT: usize = 53;
    pub const TRGM1_DMA_SRC_PWM1_CMP20: usize = 20;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P5: usize = 5;
    pub const TRGM2_DMA_SRC_PWM2_CMP10: usize = 10;
    pub const TRGM0_INPUT_SRC_VDD: usize = 1;
    pub const TRGM0_OUTPUT_SRC_TRGM0_OUTX0: usize = 12;
    pub const TRGM3_OUTPUT_SRC_TRGM3_P7: usize = 7;
    pub const TRGM1_INPUT_SRC_VDD: usize = 1;
    pub const TRGM1_INPUT_SRC_PWM1_CH9REF: usize = 21;
    pub const TRGM3_INPUT_SRC_PWM3_CH17REF: usize = 29;
    pub const TRGM1_INPUT_SRC_QEI1_TRGO: usize = 36;
    pub const TRGM2_FILTER_SRC_TRGM2_IN0: usize = 8;
    pub const TRGM0_INPUT_SRC_GPTMR1_OUT2: usize = 50;
    pub const TRGM1_OUTPUT_SRC_TRGM1_OUTX0: usize = 12;
    pub const TRGM1_INPUT_SRC_TRGM1_P2: usize = 4;
    pub const TRGM3_OUTPUT_SRC_ADCX_PTRGI3A: usize = 52;
    pub const TRGM1_INPUT_SRC_CMP2_OUT: usize = 54;
    pub const TRGM3_INPUT_SRC_TRGM3_P2: usize = 4;
    pub const TRGM2_OUTPUT_SRC_PWM2_FAULTI2: usize = 20;
    pub const TRGM0_OUTPUT_SRC_GPTMR1_SYNCI: usize = 58;
    pub const TRGM0_FILTER_SRC_TRGM0_IN6: usize = 14;
    pub const TRGM1_FILTER_SRC_PWM1_IN5: usize = 5;
    pub const TRGM0_OUTPUT_SRC_HALL0_W: usize = 46;
    pub const TRGM3_DMA_SRC_PWM3_CMP23: usize = 23;
    pub const TRGM0_INPUT_SRC_PWM0_CH13REF: usize = 25;
    pub const TRGM2_INPUT_SRC_HALL2_TRGO: usize = 37;
    pub const TRGM1_DMA_SRC_PWM1_RLD: usize = 24;
    pub const TRGM0_FILTER_SRC_TRGM0_IN5: usize = 13;
    pub const TRGM3_DMA_SRC_PWM3_CMP19: usize = 19;
    pub const TRGM3_INPUT_SRC_TRGM2_OUTX0: usize = 14;
    pub const TRGM2_FILTER_SRC_PWM2_IN3: usize = 3;
    pub const TRGM0_INPUT_SRC_TRGM0_P0: usize = 2;
    pub const TRGM1_OUTPUT_SRC_ADC0_STRGI: usize = 48;
    pub const TRGM0_INPUT_SRC_SYNT0_CH0: usize = 44;
    pub const TRGM1_DMA_SRC_PWM1_CMP15: usize = 15;
    pub const TRGM0_INPUT_SRC_CMP1_OUT: usize = 53;
    pub const TRGM3_DMA_SRC_PWM3_CMP10: usize = 10;
    pub const TRGM0_INPUT_SRC_GPTMR0_OUT2: usize = 48;
    pub const TRGM1_OUTPUT_SRC_GPTMR2_SYNCI: usize = 55;
    pub const TRGM2_OUTPUT_SRC_HALL2_U: usize = 44;
    pub const TRGM0_INPUT_SRC_TRGM0_P5: usize = 7;
    pub const TRGM0_FILTER_SRC_PWM0_IN3: usize = 3;
    pub const TRGM2_INPUT_SRC_ENET1_PTP_OUT3: usize = 41;
    pub const TRGM0_OUTPUT_SRC_PWM0_SHRLDSYNCI: usize = 17;
    pub const TRGM0_DMA_SRC_PWM0_CMP20: usize = 20;
    pub const TRGM2_OUTPUT_SRC_PWM2_FAULTI1: usize = 19;
    pub const TRGM0_OUTPUT_SRC_HALL0_V: usize = 45;
    pub const TRGM1_INPUT_SRC_SYNT_CH2: usize = 46;
    pub const TRGM3_OUTPUT_SRC_TRGM3_OUTX1: usize = 13;
    pub const TRGM3_INPUT_SRC_PWM3_CH10REF: usize = 22;
    pub const TRGM0_DMA_SRC_PWM0_CMP7: usize = 7;
    pub const TRGM0_DMA_SRC_PWM0_CMP4: usize = 4;
    pub const TRGM1_FILTER_SRC_TRGM1_IN4: usize = 12;
    pub const TRGM0_DMA_SRC_HALL0: usize = 28;
    pub const TRGM0_INPUT_SRC_TRGM0_P3: usize = 5;
    pub const TRGM3_INPUT_SRC_VDD: usize = 1;
    pub const TRGM1_OUTPUT_SRC_QEI1_PAUSE: usize = 42;
    pub const TRGM2_DMA_SRC_PWM2_HALFRLD: usize = 25;
    pub const TRGM1_INPUT_SRC_TRGM2_OUTX1: usize = 17;
    pub const TRGM1_INPUT_SRC_GPTMR2_OUT2: usize = 48;
    pub const TRGM0_FILTER_SRC_TRGM0_IN10: usize = 18;
    pub const TRGM2_FILTER_SRC_PWM2_IN1: usize = 1;
    pub const TRGM3_OUTPUT_SRC_TRGM3_P1: usize = 1;
    pub const TRGM3_OUTPUT_SRC_ADC3_STRGI: usize = 51;
    pub const TRGM2_DMA_SRC_PWM2_CMP3: usize = 3;
    pub const TRGM3_OUTPUT_SRC_PWM3_FAULTI2: usize = 20;
    pub const TRGM1_OUTPUT_SRC_TRGM1_OUTX1: usize = 13;
    pub const TRGM0_INPUT_SRC_TRGM2_OUTX0: usize = 16;
    pub const TRGM2_INPUT_SRC_CMP3_OUT: usize = 55;
    pub const TRGM3_FILTER_SRC_TRGM3_IN5: usize = 13;
    pub const TRGM2_INPUT_SRC_VDD: usize = 1;
    pub const TRGM3_FILTER_SRC_PWM3_IN2: usize = 2;
    pub const TRGM2_DMA_SRC_PWM2_CMP9: usize = 9;
    pub const TRGM0_INPUT_SRC_USB1_SOF: usize = 39;
    pub const TRGM1_INPUT_SRC_PWM1_CH21REF: usize = 33;
    pub const TRGM2_DMA_SRC_PWM2_CMP14: usize = 14;
    pub const TRGM3_DMA_SRC_PWM3_CMP13: usize = 13;
    pub const TRGM0_INPUT_SRC_TRGM1_OUTX1: usize = 19;
    pub const TRGM1_OUTPUT_SRC_QEI1_Z: usize = 40;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN15: usize = 29;
    pub const TRGM3_OUTPUT_SRC_PWM3_IN16: usize = 30;
    pub const TRGM1_INPUT_SRC_TRGM3_OUTX0: usize = 14;
    pub const TRGM3_OUTPUT_SRC_GPTMR6_IN2: usize = 56;
    pub const TRGM2_FILTER_SRC_TRGM2_IN8: usize = 16;
    pub const TRGM3_FILTER_SRC_PWM3_IN3: usize = 3;
    pub const TRGM3_INPUT_SRC_ENET1_PTP_OUT3: usize = 41;
    pub const TRGM1_INPUT_SRC_TRGM1_P9: usize = 11;
    pub const TRGM2_INPUT_SRC_TRGM2_P0: usize = 2;
    pub const TRGM3_FILTER_SRC_TRGM3_IN3: usize = 11;
    pub const TRGM0_DMA_SRC_PWM0_CMP23: usize = 23;
}
