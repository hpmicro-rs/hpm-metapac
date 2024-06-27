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
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 2147483648;
pub const FLASH_SIZE: usize = 1048576;
pub mod resources {
    pub const CAM0: usize = 336;
    pub const MCHTMR0: usize = 264;
    pub const ENET0: usize = 340;
    pub const CLK_TOP_ENET0: usize = 121;
    pub const SPI1: usize = 311;
    pub const CLK_TOP_AUD1: usize = 116;
    pub const JPEG: usize = 338;
    pub const USB0: usize = 346;
    pub const CLK_TOP_MCHTMR0: usize = 65;
    pub const HDMA: usize = 273;
    pub const AXI_BUS: usize = 257;
    pub const XPI0: usize = 268;
    pub const CLK_TOP_GPTMR1: usize = 76;
    pub const MBX0: usize = 276;
    pub const CLK_TOP_SPI2: usize = 105;
    pub const CLK_TOP_CAM0: usize = 119;
    pub const GPTMR1: usize = 283;
    pub const I2C1: usize = 307;
    pub const CLK_TOP_UART2: usize = 85;
    pub const UART12: usize = 302;
    pub const RST_CON: usize = 26;
    pub const CLK_SRC_PLL4: usize = 43;
    pub const SPI0: usize = 310;
    pub const CAN0: usize = 314;
    pub const I2S2: usize = 326;
    pub const UART4: usize = 294;
    pub const MBX1: usize = 277;
    pub const PDM: usize = 328;
    pub const CLK_TOP_ENET1: usize = 122;
    pub const CLK_TOP_AXI: usize = 68;
    pub const CLK_TOP_UART3: usize = 86;
    pub const NTMR0: usize = 342;
    pub const UART1: usize = 291;
    pub const CAN1: usize = 315;
    pub const UART15: usize = 305;
    pub const I2S3: usize = 327;
    pub const CAM1: usize = 337;
    pub const CLK_TOP_I2C3: usize = 102;
    pub const CLK_TOP_SPI1: usize = 104;
    pub const CLK_TOP_UART7: usize = 90;
    pub const CLK_TOP_UART15: usize = 98;
    pub const CLK_TOP_GPTMR7: usize = 82;
    pub const CLK_TOP_CAN1: usize = 108;
    pub const CLK_SRC_PLL0CLK0: usize = 34;
    pub const CPU0_SUBSYS: usize = 1;
    pub const GPIO: usize = 275;
    pub const UART14: usize = 304;
    pub const CLK_TOP_CAN2: usize = 109;
    pub const REF1: usize = 349;
    pub const CLK_SRC_XTAL: usize = 32;
    pub const CLK_TOP_GPTMR2: usize = 77;
    pub const CLK_TOP_ANA1: usize = 113;
    pub const UART9: usize = 299;
    pub const I2C2: usize = 308;
    pub const CLK_TOP_PTP0: usize = 123;
    pub const CLK_TOP_I2S2: usize = 198;
    pub const ENET1: usize = 341;
    pub const GPTMR4: usize = 286;
    pub const SPI3: usize = 313;
    pub const CLK_TOP_UART5: usize = 88;
    pub const CLK_TOP_LCDC: usize = 118;
    pub const CLK_TOP_ADC3: usize = 195;
    pub const AXI_SRAM1: usize = 267;
    pub const CLK_TOP_MCHTMR1: usize = 67;
    pub const CLK_SRC_PLL2CLK0: usize = 39;
    pub const CLK_TOP_GPTMR6: usize = 81;
    pub const CONN_BUS: usize = 258;
    pub const WDG1: usize = 279;
    pub const CLK_TOP_UART9: usize = 92;
    pub const GPTMR2: usize = 284;
    pub const GPTMR5: usize = 287;
    pub const RST_CPU1: usize = 29;
    pub const CLK_TOP_UART6: usize = 89;
    pub const CLK_TOP_I2S0: usize = 196;
    pub const FEMC: usize = 260;
    pub const UART0: usize = 290;
    pub const UART2: usize = 292;
    pub const CPX1_SUBSYS: usize = 9;
    pub const WDG3: usize = 281;
    pub const I2C0: usize = 306;
    pub const SPI2: usize = 312;
    pub const ADC0: usize = 319;
    pub const CLK_TOP_GPTMR5: usize = 80;
    pub const SYNT: usize = 330;
    pub const CLK_TOP_AUD0: usize = 115;
    pub const CLK_TOP_I2C1: usize = 100;
    pub const CLK_TOP_CAM1: usize = 120;
    pub const LMM1: usize = 263;
    pub const CLK_TOP_AUD2: usize = 117;
    pub const I2S0: usize = 324;
    pub const MOT3: usize = 334;
    pub const MOT1: usize = 332;
    pub const PDMA: usize = 339;
    pub const CLK_TOP_CAN0: usize = 107;
    pub const SDXC1: usize = 345;
    pub const CLK_TOP_FEMC: usize = 72;
    pub const XPI1: usize = 269;
    pub const CLK_TOP_REF1: usize = 126;
    pub const POW_CPU1: usize = 24;
    pub const CLK_TOP_ANA0: usize = 112;
    pub const CLK_TOP_VIS: usize = 70;
    pub const CLK_SRC_PLL2CLK1: usize = 40;
    pub const POW_VIS: usize = 22;
    pub const SDXC0: usize = 344;
    pub const CLK_TOP_ADC0: usize = 192;
    pub const CLK_TOP_GPTMR0: usize = 75;
    pub const CLK_TOP_SDXC0: usize = 129;
    pub const CLK_TOP_ANA2: usize = 114;
    pub const USB1: usize = 347;
    pub const CLK_TOP_UART11: usize = 94;
    pub const LMM0: usize = 262;
    pub const ADC1: usize = 320;
    pub const CLK_TOP_XPI0: usize = 73;
    pub const RNG: usize = 271;
    pub const CLK_TOP_UART10: usize = 93;
    pub const CLK_TOP_GPTMR3: usize = 78;
    pub const CLK_TOP_CAN3: usize = 110;
    pub const CLK_SRC_PLL1CLK0: usize = 36;
    pub const XDMA: usize = 274;
    pub const POW_CPU0: usize = 23;
    pub const CLK_TOP_I2C0: usize = 99;
    pub const RST_CPU0: usize = 28;
    pub const UART5: usize = 295;
    pub const MCHTMR1: usize = 265;
    pub const POW_CON: usize = 21;
    pub const CLK_TOP_AHB: usize = 71;
    pub const CLK_TOP_SPI3: usize = 106;
    pub const ROM: usize = 261;
    pub const UART11: usize = 301;
    pub const UART8: usize = 298;
    pub const CLK_TOP_UART12: usize = 95;
    pub const CLK_TOP_PTP1: usize = 124;
    pub const AXI_SRAM0: usize = 266;
    pub const CLK_TOP_CPU0: usize = 64;
    pub const CLK_TOP_REF0: usize = 125;
    pub const CLK_TOP_SDXC1: usize = 130;
    pub const CLK_TOP_CONN: usize = 69;
    pub const CLK_SRC_PLL2: usize = 38;
    pub const CLK_TOP_NTMR1: usize = 128;
    pub const GPTMR6: usize = 288;
    pub const RST_SOC: usize = 25;
    pub const CLK_TOP_ADC1: usize = 193;
    pub const CLK_TOP_ADC2: usize = 194;
    pub const UART3: usize = 293;
    pub const NTMR1: usize = 343;
    pub const CLK_SRC_PLL4CLK0: usize = 44;
    pub const CLK_TOP_I2S3: usize = 199;
    pub const CLK_TOP_CPU1: usize = 66;
    pub const RST_VIS: usize = 27;
    pub const GPTMR0: usize = 282;
    pub const GPTMR3: usize = 285;
    pub const CLK_SRC_PLL3CLK0: usize = 42;
    pub const UART10: usize = 300;
    pub const CLK_SRC_PLL3: usize = 41;
    pub const UART7: usize = 297;
    pub const UART13: usize = 303;
    pub const CAN3: usize = 317;
    pub const CLK_TOP_I2S1: usize = 197;
    pub const PTPC: usize = 318;
    pub const SDP: usize = 270;
    pub const CLK_TOP_UART8: usize = 91;
    pub const UART6: usize = 296;
    pub const ACMP: usize = 323;
    pub const KEYM: usize = 272;
    pub const CLK_TOP_UART0: usize = 83;
    pub const CLK_TOP_SPI0: usize = 103;
    pub const VIS_BUS: usize = 259;
    pub const CLK_TOP_UART14: usize = 97;
    pub const CPU0_CORE: usize = 0;
    pub const CLK_TOP_NTMR0: usize = 127;
    pub const CLK_TOP_XPI1: usize = 74;
    pub const CLK_TOP_UART4: usize = 87;
    pub const WDG0: usize = 278;
    pub const WDG2: usize = 280;
    pub const GPTMR7: usize = 289;
    pub const I2S1: usize = 325;
    pub const LCDC: usize = 335;
    pub const I2C3: usize = 309;
    pub const CLK_SRC_PLL1: usize = 35;
    pub const CLK_TOP_GPTMR4: usize = 79;
    pub const CAN2: usize = 316;
    pub const MOT2: usize = 333;
    pub const REF0: usize = 348;
    pub const CPU1_CORE: usize = 8;
    pub const CLK_TOP_UART1: usize = 84;
    pub const AHBAPB_BUS: usize = 256;
    pub const CLK_SRC_PLL0: usize = 33;
    pub const DAO: usize = 329;
    pub const CLK_TOP_I2C2: usize = 101;
    pub const CLK_SRC_PLL1CLK1: usize = 37;
    pub const CLK_TOP_PTPC: usize = 111;
    pub const CLK_TOP_UART13: usize = 96;
    pub const ADC3: usize = 322;
    pub const MOT0: usize = 331;
    pub const ADC2: usize = 321;
}
pub mod clocks {
    pub const AXI: usize = 4;
    pub const PTP0: usize = 59;
    pub const UART9: usize = 28;
    pub const GPTMR0: usize = 11;
    pub const I2C1: usize = 36;
    pub const SPI1: usize = 40;
    pub const I2C2: usize = 37;
    pub const GPTMR1: usize = 12;
    pub const ANA0: usize = 48;
    pub const GPTMR5: usize = 16;
    pub const AHB: usize = 7;
    pub const I2C0: usize = 35;
    pub const CAN0: usize = 43;
    pub const AUD1: usize = 52;
    pub const CONN: usize = 5;
    pub const UART8: usize = 27;
    pub const UART14: usize = 33;
    pub const I2C3: usize = 38;
    pub const MCHTMR0: usize = 1;
    pub const CAN3: usize = 46;
    pub const ENET0: usize = 57;
    pub const AUD2: usize = 53;
    pub const SPI0: usize = 39;
    pub const UART1: usize = 20;
    pub const CAM1: usize = 56;
    pub const UART2: usize = 21;
    pub const REF1: usize = 62;
    pub const SDXC0: usize = 65;
    pub const CPU0: usize = 0;
    pub const CAN1: usize = 44;
    pub const CAN2: usize = 45;
    pub const PTPC: usize = 47;
    pub const ANA1: usize = 49;
    pub const UART0: usize = 19;
    pub const UART13: usize = 32;
    pub const REF0: usize = 61;
    pub const MCHTMR: usize = 3;
    pub const GPTMR4: usize = 15;
    pub const VIS: usize = 6;
    pub const NTMR1: usize = 64;
    pub const XPI0: usize = 9;
    pub const GPTMR6: usize = 17;
    pub const UART5: usize = 24;
    pub const UART7: usize = 26;
    pub const FEMC: usize = 8;
    pub const ENET1: usize = 58;
    pub const SDXC1: usize = 66;
    pub const UART6: usize = 25;
    pub const LCDC: usize = 54;
    pub const UART11: usize = 30;
    pub const SPI2: usize = 41;
    pub const XPI1: usize = 10;
    pub const ANA2: usize = 50;
    pub const SPI3: usize = 42;
    pub const UART3: usize = 22;
    pub const PTP1: usize = 60;
    pub const UART12: usize = 31;
    pub const NTMR0: usize = 63;
    pub const UART15: usize = 34;
    pub const CPU1: usize = 2;
    pub const AUD0: usize = 51;
    pub const GPTMR2: usize = 13;
    pub const GPTMR3: usize = 14;
    pub const UART10: usize = 29;
    pub const CAM0: usize = 55;
    pub const UART4: usize = 23;
    pub const GPTMR7: usize = 18;
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
