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
    #[doc = "10 - MCAN0"]
    MCAN0 = 10,
    #[doc = "11 - MCAN1"]
    MCAN1 = 11,
    #[doc = "12 - MCAN2"]
    MCAN2 = 12,
    #[doc = "13 - MCAN3"]
    MCAN3 = 13,
    #[doc = "14 - MCAN4"]
    MCAN4 = 14,
    #[doc = "15 - MCAN5"]
    MCAN5 = 15,
    #[doc = "16 - MCAN6"]
    MCAN6 = 16,
    #[doc = "17 - MCAN7"]
    MCAN7 = 17,
    #[doc = "18 - PTPC"]
    PTPC = 18,
    #[doc = "27 - UART0"]
    UART0 = 27,
    #[doc = "28 - UART1"]
    UART1 = 28,
    #[doc = "29 - UART2"]
    UART2 = 29,
    #[doc = "30 - UART3"]
    UART3 = 30,
    #[doc = "31 - UART4"]
    UART4 = 31,
    #[doc = "32 - UART5"]
    UART5 = 32,
    #[doc = "33 - UART6"]
    UART6 = 33,
    #[doc = "34 - UART7"]
    UART7 = 34,
    #[doc = "35 - I2C0"]
    I2C0 = 35,
    #[doc = "36 - I2C1"]
    I2C1 = 36,
    #[doc = "37 - I2C2"]
    I2C2 = 37,
    #[doc = "38 - I2C3"]
    I2C3 = 38,
    #[doc = "39 - SPI0"]
    SPI0 = 39,
    #[doc = "40 - SPI1"]
    SPI1 = 40,
    #[doc = "41 - SPI2"]
    SPI2 = 41,
    #[doc = "42 - SPI3"]
    SPI3 = 42,
    #[doc = "43 - GPTMR0"]
    GPTMR0 = 43,
    #[doc = "44 - GPTMR1"]
    GPTMR1 = 44,
    #[doc = "45 - GPTMR2"]
    GPTMR2 = 45,
    #[doc = "46 - GPTMR3"]
    GPTMR3 = 46,
    #[doc = "47 - GPTMR4"]
    GPTMR4 = 47,
    #[doc = "48 - GPTMR5"]
    GPTMR5 = 48,
    #[doc = "49 - GPTMR6"]
    GPTMR6 = 49,
    #[doc = "50 - GPTMR7"]
    GPTMR7 = 50,
    #[doc = "51 - EWDG0"]
    EWDG0 = 51,
    #[doc = "52 - EWDG1"]
    EWDG1 = 52,
    #[doc = "53 - MBX0A"]
    MBX0A = 53,
    #[doc = "54 - MBX0B"]
    MBX0B = 54,
    #[doc = "55 - MBX1A"]
    MBX1A = 55,
    #[doc = "56 - MBX1B"]
    MBX1B = 56,
    #[doc = "57 - RNG"]
    RNG = 57,
    #[doc = "58 - HDMA"]
    HDMA = 58,
    #[doc = "59 - ADC0"]
    ADC0 = 59,
    #[doc = "60 - ADC1"]
    ADC1 = 60,
    #[doc = "61 - SDM"]
    SDM = 61,
    #[doc = "62 - OPAMP"]
    OPAMP = 62,
    #[doc = "63 - I2S0"]
    I2S0 = 63,
    #[doc = "64 - I2S1"]
    I2S1 = 64,
    #[doc = "65 - I2S2"]
    I2S2 = 65,
    #[doc = "66 - I2S3"]
    I2S3 = 66,
    #[doc = "67 - DAO"]
    DAO = 67,
    #[doc = "68 - PDM"]
    PDM = 68,
    #[doc = "69 - SMIX_DMA"]
    SMIX_DMA = 69,
    #[doc = "70 - SMIX_ASRC"]
    SMIX_ASRC = 70,
    #[doc = "71 - CAM0"]
    CAM0 = 71,
    #[doc = "72 - CAM1"]
    CAM1 = 72,
    #[doc = "73 - LCDC"]
    LCDC = 73,
    #[doc = "74 - LCDC1"]
    LCDC1 = 74,
    #[doc = "75 - PDMA"]
    PDMA = 75,
    #[doc = "76 - JPEG"]
    JPEG = 76,
    #[doc = "77 - GWCK0_FUNC"]
    GWCK0_FUNC = 77,
    #[doc = "78 - GWCK0_ERR"]
    GWCK0_ERR = 78,
    #[doc = "79 - GWCK1_FUNC"]
    GWCK1_FUNC = 79,
    #[doc = "80 - GWCK1_ERR"]
    GWCK1_ERR = 80,
    #[doc = "81 - MIPI_DSI0"]
    MIPI_DSI0 = 81,
    #[doc = "82 - MIPI_DSI1"]
    MIPI_DSI1 = 82,
    #[doc = "83 - MIPI_CSI0"]
    MIPI_CSI0 = 83,
    #[doc = "84 - MIPI_CSI0_AP"]
    MIPI_CSI0_AP = 84,
    #[doc = "85 - MIPI_CSI0_DIAG"]
    MIPI_CSI0_DIAG = 85,
    #[doc = "86 - MIPI_CSI1_AP"]
    MIPI_CSI1_AP = 86,
    #[doc = "87 - MIPI_CSI1_DIAG"]
    MIPI_CSI1_DIAG = 87,
    #[doc = "88 - MIPI_CSI1"]
    MIPI_CSI1 = 88,
    #[doc = "89 - LCB0"]
    LCB0 = 89,
    #[doc = "90 - LCB1"]
    LCB1 = 90,
    #[doc = "91 - GPU"]
    GPU = 91,
    #[doc = "92 - ENET0"]
    ENET0 = 92,
    #[doc = "93 - NTMR0"]
    NTMR0 = 93,
    #[doc = "94 - USB0"]
    USB0 = 94,
    #[doc = "95 - SDXC0"]
    SDXC0 = 95,
    #[doc = "96 - SDXC1"]
    SDXC1 = 96,
    #[doc = "97 - SDP"]
    SDP = 97,
    #[doc = "98 - XPI0"]
    XPI0 = 98,
    #[doc = "99 - XDMA"]
    XDMA = 99,
    #[doc = "100 - DDR"]
    DDR = 100,
    #[doc = "101 - FFA"]
    FFA = 101,
    #[doc = "102 - PSEC"]
    PSEC = 102,
    #[doc = "103 - TSNS"]
    TSNS = 103,
    #[doc = "104 - VAD"]
    VAD = 104,
    #[doc = "105 - PGPIO"]
    PGPIO = 105,
    #[doc = "106 - PWDG"]
    PWDG = 106,
    #[doc = "107 - PTMR"]
    PTMR = 107,
    #[doc = "108 - PUART"]
    PUART = 108,
    #[doc = "109 - FUSE"]
    FUSE = 109,
    #[doc = "110 - SECMON"]
    SECMON = 110,
    #[doc = "111 - RTC"]
    RTC = 111,
    #[doc = "112 - BGPIO"]
    BGPIO = 112,
    #[doc = "113 - BVIO"]
    BVIO = 113,
    #[doc = "114 - BROWNOUT"]
    BROWNOUT = 114,
    #[doc = "115 - SYSCTL"]
    SYSCTL = 115,
    #[doc = "116 - DEBUG0"]
    DEBUG0 = 116,
    #[doc = "117 - DEBUG1"]
    DEBUG1 = 117,
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
        fn MCAN0();
        fn MCAN1();
        fn MCAN2();
        fn MCAN3();
        fn MCAN4();
        fn MCAN5();
        fn MCAN6();
        fn MCAN7();
        fn PTPC();
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
        fn GPTMR0();
        fn GPTMR1();
        fn GPTMR2();
        fn GPTMR3();
        fn GPTMR4();
        fn GPTMR5();
        fn GPTMR6();
        fn GPTMR7();
        fn EWDG0();
        fn EWDG1();
        fn MBX0A();
        fn MBX0B();
        fn MBX1A();
        fn MBX1B();
        fn RNG();
        fn HDMA();
        fn ADC0();
        fn ADC1();
        fn SDM();
        fn OPAMP();
        fn I2S0();
        fn I2S1();
        fn I2S2();
        fn I2S3();
        fn DAO();
        fn PDM();
        fn SMIX_DMA();
        fn SMIX_ASRC();
        fn CAM0();
        fn CAM1();
        fn LCDC();
        fn LCDC1();
        fn PDMA();
        fn JPEG();
        fn GWCK0_FUNC();
        fn GWCK0_ERR();
        fn GWCK1_FUNC();
        fn GWCK1_ERR();
        fn MIPI_DSI0();
        fn MIPI_DSI1();
        fn MIPI_CSI0();
        fn MIPI_CSI0_AP();
        fn MIPI_CSI0_DIAG();
        fn MIPI_CSI1_AP();
        fn MIPI_CSI1_DIAG();
        fn MIPI_CSI1();
        fn LCB0();
        fn LCB1();
        fn GPU();
        fn ENET0();
        fn NTMR0();
        fn USB0();
        fn SDXC0();
        fn SDXC1();
        fn SDP();
        fn XPI0();
        fn XDMA();
        fn DDR();
        fn FFA();
        fn PSEC();
        fn TSNS();
        fn VAD();
        fn PGPIO();
        fn PWDG();
        fn PTMR();
        fn PUART();
        fn FUSE();
        fn SECMON();
        fn RTC();
        fn BGPIO();
        fn BVIO();
        fn BROWNOUT();
        fn SYSCTL();
        fn DEBUG0();
        fn DEBUG1();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __VECTORED_INTERRUPTS: [Vector; 118] = [
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
        Vector { _handler: MCAN0 },
        Vector { _handler: MCAN1 },
        Vector { _handler: MCAN2 },
        Vector { _handler: MCAN3 },
        Vector { _handler: MCAN4 },
        Vector { _handler: MCAN5 },
        Vector { _handler: MCAN6 },
        Vector { _handler: MCAN7 },
        Vector { _handler: PTPC },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
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
        Vector { _handler: GPTMR0 },
        Vector { _handler: GPTMR1 },
        Vector { _handler: GPTMR2 },
        Vector { _handler: GPTMR3 },
        Vector { _handler: GPTMR4 },
        Vector { _handler: GPTMR5 },
        Vector { _handler: GPTMR6 },
        Vector { _handler: GPTMR7 },
        Vector { _handler: EWDG0 },
        Vector { _handler: EWDG1 },
        Vector { _handler: MBX0A },
        Vector { _handler: MBX0B },
        Vector { _handler: MBX1A },
        Vector { _handler: MBX1B },
        Vector { _handler: RNG },
        Vector { _handler: HDMA },
        Vector { _handler: ADC0 },
        Vector { _handler: ADC1 },
        Vector { _handler: SDM },
        Vector { _handler: OPAMP },
        Vector { _handler: I2S0 },
        Vector { _handler: I2S1 },
        Vector { _handler: I2S2 },
        Vector { _handler: I2S3 },
        Vector { _handler: DAO },
        Vector { _handler: PDM },
        Vector { _handler: SMIX_DMA },
        Vector {
            _handler: SMIX_ASRC,
        },
        Vector { _handler: CAM0 },
        Vector { _handler: CAM1 },
        Vector { _handler: LCDC },
        Vector { _handler: LCDC1 },
        Vector { _handler: PDMA },
        Vector { _handler: JPEG },
        Vector {
            _handler: GWCK0_FUNC,
        },
        Vector {
            _handler: GWCK0_ERR,
        },
        Vector {
            _handler: GWCK1_FUNC,
        },
        Vector {
            _handler: GWCK1_ERR,
        },
        Vector {
            _handler: MIPI_DSI0,
        },
        Vector {
            _handler: MIPI_DSI1,
        },
        Vector {
            _handler: MIPI_CSI0,
        },
        Vector {
            _handler: MIPI_CSI0_AP,
        },
        Vector {
            _handler: MIPI_CSI0_DIAG,
        },
        Vector {
            _handler: MIPI_CSI1_AP,
        },
        Vector {
            _handler: MIPI_CSI1_DIAG,
        },
        Vector {
            _handler: MIPI_CSI1,
        },
        Vector { _handler: LCB0 },
        Vector { _handler: LCB1 },
        Vector { _handler: GPU },
        Vector { _handler: ENET0 },
        Vector { _handler: NTMR0 },
        Vector { _handler: USB0 },
        Vector { _handler: SDXC0 },
        Vector { _handler: SDXC1 },
        Vector { _handler: SDP },
        Vector { _handler: XPI0 },
        Vector { _handler: XDMA },
        Vector { _handler: DDR },
        Vector { _handler: FFA },
        Vector { _handler: PSEC },
        Vector { _handler: TSNS },
        Vector { _handler: VAD },
        Vector { _handler: PGPIO },
        Vector { _handler: PWDG },
        Vector { _handler: PTMR },
        Vector { _handler: PUART },
        Vector { _handler: FUSE },
        Vector { _handler: SECMON },
        Vector { _handler: RTC },
        Vector { _handler: BGPIO },
        Vector { _handler: BVIO },
        Vector { _handler: BROWNOUT },
        Vector { _handler: SYSCTL },
        Vector { _handler: DEBUG0 },
        Vector { _handler: DEBUG1 },
    ];
}
pub const FGPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x000c_0000usize as _) };
pub const PLIC: plic::Plic = unsafe { plic::Plic::from_ptr(0xe400_0000usize as _) };
pub const MCHTMR: mchtmr::Mchtmr = unsafe { mchtmr::Mchtmr::from_ptr(0xe600_0000usize as _) };
pub const PLICSW: plicsw::Plicsw = unsafe { plicsw::Plicsw::from_ptr(0xe640_0000usize as _) };
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
pub const GPTMR0: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf008_0000usize as _) };
pub const GPTMR1: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf008_4000usize as _) };
pub const GPTMR2: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf008_8000usize as _) };
pub const GPTMR3: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf008_c000usize as _) };
pub const GPTMR4: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf009_0000usize as _) };
pub const GPTMR5: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf009_4000usize as _) };
pub const GPTMR6: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf009_8000usize as _) };
pub const GPTMR7: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf009_c000usize as _) };
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
pub const GPIOM: gpiom::Gpiom = unsafe { gpiom::Gpiom::from_ptr(0xf00d_8000usize as _) };
pub const ADC0: adc16::Adc = unsafe { adc16::Adc::from_ptr(0xf00e_0000usize as _) };
pub const I2S0: i2s::I2s = unsafe { i2s::I2s::from_ptr(0xf020_0000usize as _) };
pub const I2S1: i2s::I2s = unsafe { i2s::I2s::from_ptr(0xf020_4000usize as _) };
pub const I2S2: i2s::I2s = unsafe { i2s::I2s::from_ptr(0xf020_8000usize as _) };
pub const I2S3: i2s::I2s = unsafe { i2s::I2s::from_ptr(0xf020_c000usize as _) };
pub const DAO: dao::Dao = unsafe { dao::Dao::from_ptr(0xf021_0000usize as _) };
pub const PDM: pdm::Pdm = unsafe { pdm::Pdm::from_ptr(0xf021_4000usize as _) };
pub const SMIX: smix::Smix = unsafe { smix::Smix::from_ptr(0xf021_8000usize as _) };
pub const MCAN0: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf028_0000usize as _) };
pub const MCAN1: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf028_4000usize as _) };
pub const MCAN2: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf028_8000usize as _) };
pub const MCAN3: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf028_c000usize as _) };
pub const MCAN4: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf029_0000usize as _) };
pub const MCAN5: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf029_4000usize as _) };
pub const MCAN6: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf029_8000usize as _) };
pub const MCAN7: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf029_c000usize as _) };
pub const PTPC: ptpc::Ptpc = unsafe { ptpc::Ptpc::from_ptr(0xf02f_c000usize as _) };
pub const LCDC0: lcdc::Lcdc = unsafe { lcdc::Lcdc::from_ptr(0xf100_0000usize as _) };
pub const LCDC1: lcdc::Lcdc = unsafe { lcdc::Lcdc::from_ptr(0xf100_4000usize as _) };
pub const CAM0: cam::Cam = unsafe { cam::Cam::from_ptr(0xf100_8000usize as _) };
pub const CAM1: cam::Cam = unsafe { cam::Cam::from_ptr(0xf100_c000usize as _) };
pub const PDMA: pdma::Pdma = unsafe { pdma::Pdma::from_ptr(0xf101_0000usize as _) };
pub const JPEG: jpeg::Jpeg = unsafe { jpeg::Jpeg::from_ptr(0xf101_4000usize as _) };
pub const GWC0: gwc::Gwc = unsafe { gwc::Gwc::from_ptr(0xf101_8000usize as _) };
pub const GWC1: gwc::Gwc = unsafe { gwc::Gwc::from_ptr(0xf101_c000usize as _) };
pub const MIPI_DSI0: mipidsi::MipiDsi =
    unsafe { mipidsi::MipiDsi::from_ptr(0xf102_0000usize as _) };
pub const MIPI_DSI1: mipidsi::MipiDsi =
    unsafe { mipidsi::MipiDsi::from_ptr(0xf102_4000usize as _) };
pub const MIPI_CSI0: mipicsi::MipiCsi =
    unsafe { mipicsi::MipiCsi::from_ptr(0xf102_8000usize as _) };
pub const MIPI_CSI1: mipicsi::MipiCsi =
    unsafe { mipicsi::MipiCsi::from_ptr(0xf102_c000usize as _) };
pub const LVB: lvb::Lvb = unsafe { lvb::Lvb::from_ptr(0xf103_0000usize as _) };
pub const PIXELMUX: pixelmux::PixelMux =
    unsafe { pixelmux::PixelMux::from_ptr(0xf103_4000usize as _) };
pub const LCB: lcb::Lcb = unsafe { lcb::Lcb::from_ptr(0xf103_8000usize as _) };
pub const GPU: gpu::Gpu = unsafe { gpu::Gpu::from_ptr(0xf108_0000usize as _) };
pub const ENET0: enet::Enet = unsafe { enet::Enet::from_ptr(0xf110_0000usize as _) };
pub const NTMR0: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf111_0000usize as _) };
pub const USB: usb::Usb = unsafe { usb::Usb::from_ptr(0xf112_0000usize as _) };
pub const SDXC0: sdxc::Sdxc = unsafe { sdxc::Sdxc::from_ptr(0xf113_0000usize as _) };
pub const SDXC1: sdxc::Sdxc = unsafe { sdxc::Sdxc::from_ptr(0xf113_4000usize as _) };
pub const XPI0: xpi::Xpi = unsafe { xpi::Xpi::from_ptr(0xf300_0000usize as _) };
pub const XDMA: dma::Dma = unsafe { dma::Dma::from_ptr(0xf300_8000usize as _) };
pub const DDRCTL: ddrctl::Ddrctl = unsafe { ddrctl::Ddrctl::from_ptr(0xf301_0000usize as _) };
pub const FFA: ffa::Ffa = unsafe { ffa::Ffa::from_ptr(0xf301_8000usize as _) };
pub const OTP: otp::Otp = unsafe { otp::Otp::from_ptr(0xf305_0000usize as _) };
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
pub const VAD: vad::Vad = unsafe { vad::Vad::from_ptr(0xf412_c000usize as _) };
pub const MIPI_DSI_PHY0: mipidsiphy::MipiDsiPhy =
    unsafe { mipidsiphy::MipiDsiPhy::from_ptr(0xf414_0000usize as _) };
pub const MIPI_DSI_PHY1: mipidsiphy::MipiDsiPhy =
    unsafe { mipidsiphy::MipiDsiPhy::from_ptr(0xf414_4000usize as _) };
pub const MIPI_CSI_PHY0: mipicsiphy::MipiCsiPhy =
    unsafe { mipicsiphy::MipiCsiPhy::from_ptr(0xf414_8000usize as _) };
pub const MIPI_CSI_PHY1: mipicsiphy::MipiCsiPhy =
    unsafe { mipicsiphy::MipiCsiPhy::from_ptr(0xf414_c000usize as _) };
pub const DDRPHY: ddrphy::Ddrphy = unsafe { ddrphy::Ddrphy::from_ptr(0xf415_0000usize as _) };
pub const TSNS: tsns::Tsns = unsafe { tsns::Tsns::from_ptr(0xf415_4000usize as _) };
pub const BPOR: bpor::Bpor = unsafe { bpor::Bpor::from_ptr(0xf420_4000usize as _) };
pub const BCFG: bcfg::Bcfg = unsafe { bcfg::Bcfg::from_ptr(0xf420_8000usize as _) };
pub const BIOC: ioc::Ioc = unsafe { ioc::Ioc::from_ptr(0xf421_0000usize as _) };
pub const BGPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf421_4000usize as _) };
pub const RTCSHW: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0xf421_c000usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0xf424_4000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc16_v68.rs"]
pub mod adc16;
#[path = "../../peripherals/bcfg_v68.rs"]
pub mod bcfg;
#[path = "../../peripherals/bpor_v68.rs"]
pub mod bpor;
#[path = "../../peripherals/cam_v68.rs"]
pub mod cam;
#[path = "../../peripherals/crc_common.rs"]
pub mod crc;
#[path = "../../peripherals/dao_v68.rs"]
pub mod dao;
#[path = "../../peripherals/ddrctl_v68.rs"]
pub mod ddrctl;
#[path = "../../peripherals/ddrphy_v68.rs"]
pub mod ddrphy;
#[path = "../../peripherals/dma_v53.rs"]
pub mod dma;
#[path = "../../peripherals/dmamux_common.rs"]
pub mod dmamux;
#[path = "../../peripherals/enet_v68.rs"]
pub mod enet;
#[path = "../../peripherals/ffa_common.rs"]
pub mod ffa;
#[path = "../../peripherals/gpio_common.rs"]
pub mod gpio;
#[path = "../../peripherals/gpiom_v68.rs"]
pub mod gpiom;
#[path = "../../peripherals/gpu_v68.rs"]
pub mod gpu;
#[path = "../../peripherals/gwc_v68.rs"]
pub mod gwc;
#[path = "../../peripherals/i2c_v53.rs"]
pub mod i2c;
#[path = "../../peripherals/i2s_common.rs"]
pub mod i2s;
#[path = "../../peripherals/ioc_common.rs"]
pub mod ioc;
#[path = "../../peripherals/jpeg_common.rs"]
pub mod jpeg;
#[path = "../../peripherals/lcb_v68.rs"]
pub mod lcb;
#[path = "../../peripherals/lcdc_v68.rs"]
pub mod lcdc;
#[path = "../../peripherals/lvb_v68.rs"]
pub mod lvb;
#[path = "../../peripherals/mbx_common.rs"]
pub mod mbx;
#[path = "../../peripherals/mcan_v68.rs"]
pub mod mcan;
#[path = "../../peripherals/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../../peripherals/mipicsi_v68.rs"]
pub mod mipicsi;
#[path = "../../peripherals/mipicsiphy_v68.rs"]
pub mod mipicsiphy;
#[path = "../../peripherals/mipidsi_v68.rs"]
pub mod mipidsi;
#[path = "../../peripherals/mipidsiphy_v68.rs"]
pub mod mipidsiphy;
#[path = "../../peripherals/otp_common.rs"]
pub mod otp;
#[path = "../../peripherals/pcfg_v68.rs"]
pub mod pcfg;
#[path = "../../peripherals/pdm_common.rs"]
pub mod pdm;
#[path = "../../peripherals/pdma_v68.rs"]
pub mod pdma;
#[path = "../../peripherals/pixelmux_v68.rs"]
pub mod pixelmux;
#[path = "../../peripherals/plic_common.rs"]
pub mod plic;
#[path = "../../peripherals/plicsw_common.rs"]
pub mod plicsw;
#[path = "../../peripherals/pllctl_v2.rs"]
pub mod pllctl;
#[path = "../../peripherals/ppor_v68.rs"]
pub mod ppor;
#[path = "../../peripherals/ptpc_common.rs"]
pub mod ptpc;
#[path = "../../peripherals/rtc_common.rs"]
pub mod rtc;
#[path = "../../peripherals/sdxc_v68.rs"]
pub mod sdxc;
#[path = "../../peripherals/smix_v68.rs"]
pub mod smix;
#[path = "../../peripherals/spi_v53.rs"]
pub mod spi;
#[path = "../../peripherals/sysctl_v68.rs"]
pub mod sysctl;
#[path = "../../peripherals/tmr_common.rs"]
pub mod tmr;
#[path = "../../peripherals/tsns_common.rs"]
pub mod tsns;
#[path = "../../peripherals/uart_v68.rs"]
pub mod uart;
#[path = "../../peripherals/usb_v53.rs"]
pub mod usb;
#[path = "../../peripherals/vad_common.rs"]
pub mod vad;
#[path = "../../peripherals/wdg_v68.rs"]
pub mod wdg;
#[path = "../../peripherals/xpi_dummy.rs"]
pub mod xpi;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 2147483648;
pub const FLASH_SIZE: usize = 1048576;
pub mod resources {
    pub const URT4: usize = 296;
    pub const CPX0: usize = 1;
    pub const CLK_TOP_AXIC: usize = 69;
    pub const CLK_TOP_SPI1: usize = 93;
    pub const CLK_TOP_URT1: usize = 97;
    pub const PTPC: usize = 273;
    pub const XPI0: usize = 328;
    pub const CLK_TOP_URT6: usize = 102;
    pub const OAMP: usize = 275;
    pub const WDG0: usize = 300;
    pub const LIN5: usize = 281;
    pub const CLK_TOP_CAN2: usize = 74;
    pub const CLK_TOP_AUD0: usize = 116;
    pub const TMR7: usize = 311;
    pub const CLK_TOP_URT0: usize = 96;
    pub const DAO0: usize = 317;
    pub const CLK_SRC_PLL3_REF: usize = 48;
    pub const ETH0: usize = 331;
    pub const NTM0: usize = 335;
    pub const CAN3: usize = 268;
    pub const SMIX: usize = 318;
    pub const GWC1: usize = 345;
    pub const PDM0: usize = 316;
    pub const KMAN: usize = 321;
    pub const CLK_TOP_I2S1: usize = 136;
    pub const URT7: usize = 299;
    pub const USB0: usize = 332;
    pub const CAN4: usize = 269;
    pub const SPI1: usize = 289;
    pub const CLK_TOP_CAN7: usize = 79;
    pub const URT3: usize = 295;
    pub const RST_SOC: usize = 25;
    pub const CLK_TOP_CAM0: usize = 127;
    pub const CLK_TOP_AXIV: usize = 70;
    pub const CLK_TOP_TMR1: usize = 105;
    pub const CLK_TOP_ANA0: usize = 114;
    pub const CLK_TOP_URT7: usize = 103;
    pub const CLK_TOP_AXIS: usize = 68;
    pub const CLK_TOP_LIN5: usize = 85;
    pub const RST_CPU0: usize = 28;
    pub const LIN4: usize = 280;
    pub const CLK_TOP_ANA1: usize = 115;
    pub const AXIC: usize = 257;
    pub const AXIG: usize = 259;
    pub const CLK_SRC_CLK1_PLL2: usize = 40;
    pub const CLK_TOP_CAN5: usize = 77;
    pub const CLK_TOP_SDC1: usize = 123;
    pub const CLK_TOP_ADC1: usize = 134;
    pub const SPI3: usize = 291;
    pub const CLK_TOP_CAN1: usize = 73;
    pub const POW_GPU: usize = 23;
    pub const CLK_TOP_SPI3: usize = 95;
    pub const CPU0: usize = 0;
    pub const CLK_TOP_LIN1: usize = 81;
    pub const AXIS: usize = 256;
    pub const RST_VIS: usize = 27;
    pub const AXIV: usize = 258;
    pub const LMM0: usize = 260;
    pub const POW_VIS: usize = 21;
    pub const CLK_TOP_REF1: usize = 126;
    pub const ROM0: usize = 262;
    pub const URT5: usize = 297;
    pub const CLK_TOP_LIN3: usize = 83;
    pub const TMR2: usize = 306;
    pub const TMR4: usize = 308;
    pub const TMR6: usize = 310;
    pub const ADC0: usize = 323;
    pub const CLK_TOP_I2C1: usize = 89;
    pub const WDG1: usize = 301;
    pub const LIN7: usize = 283;
    pub const CLK_SRC_PLL0_REF: usize = 45;
    pub const CLK_TOP_I2S3: usize = 138;
    pub const SPI2: usize = 290;
    pub const SDP0: usize = 320;
    pub const ADC1: usize = 324;
    pub const XRAM: usize = 264;
    pub const SDC0: usize = 333;
    pub const SDC1: usize = 334;
    pub const URT6: usize = 298;
    pub const I2S0: usize = 312;
    pub const CLK_TOP_AXID: usize = 71;
    pub const LCD1: usize = 343;
    pub const CLK_TOP_XPI0: usize = 112;
    pub const CLK_TOP_SDC0: usize = 122;
    pub const LCD0: usize = 342;
    pub const CSI0: usize = 346;
    pub const CAN6: usize = 271;
    pub const CLK_TOP_URT5: usize = 101;
    pub const JPEG: usize = 341;
    pub const RST_CON: usize = 26;
    pub const CLK_TOP_NTM0: usize = 124;
    pub const SDM0: usize = 325;
    pub const CLK_TOP_CAN4: usize = 76;
    pub const MBX1: usize = 303;
    pub const CLK_TOP_LIN7: usize = 87;
    pub const CLK_SRC_PLL3: usize = 41;
    pub const CLK_TOP_AXIF: usize = 67;
    pub const CAN1: usize = 266;
    pub const CAN7: usize = 272;
    pub const URT1: usize = 293;
    pub const CLK_TOP_I2C0: usize = 88;
    pub const DDR0: usize = 263;
    pub const RNG0: usize = 319;
    pub const LCB0: usize = 351;
    pub const CLK_TOP_PTP0: usize = 121;
    pub const CLK_TOP_URT2: usize = 98;
    pub const GPIO: usize = 322;
    pub const RST_GPU: usize = 29;
    pub const CLK_SRC_PLL4: usize = 43;
    pub const CLK_TOP_I2S2: usize = 137;
    pub const LIN6: usize = 282;
    pub const XDMA: usize = 327;
    pub const MBX0: usize = 302;
    pub const CLK_TOP_AUD3: usize = 119;
    pub const CLK_TOP_CSI0: usize = 131;
    pub const LIN0: usize = 276;
    pub const CLK_TOP_LIN6: usize = 86;
    pub const FFA0: usize = 329;
    pub const CSI1: usize = 347;
    pub const DSI0: usize = 348;
    pub const TMR3: usize = 307;
    pub const GPU0: usize = 352;
    pub const CLK_SRC_PLL4_REF: usize = 49;
    pub const CLK_TOP_LIN4: usize = 84;
    pub const CRC0: usize = 274;
    pub const REF0: usize = 336;
    pub const CLK_SRC_PLL2_REF: usize = 47;
    pub const CLK_TOP_TMR7: usize = 111;
    pub const CLK_SRC_PLL0: usize = 33;
    pub const CLK_TOP_SPI2: usize = 94;
    pub const CLK_TOP_TMR6: usize = 110;
    pub const CLK_TOP_REF0: usize = 125;
    pub const CLK_SRC_CLK1_PLL1: usize = 37;
    pub const CLK_TOP_URT4: usize = 100;
    pub const I2S3: usize = 315;
    pub const CLK_SRC_CLK0_PLL4: usize = 44;
    pub const I2S1: usize = 313;
    pub const CAM1: usize = 339;
    pub const CLK_TOP_MCT0: usize = 65;
    pub const CLK_SRC_XTAL: usize = 32;
    pub const CLK_TOP_CAN0: usize = 72;
    pub const CLK_TOP_TMR2: usize = 106;
    pub const CLK_TOP_SPI0: usize = 92;
    pub const I2C0: usize = 284;
    pub const CLK_TOP_TMR3: usize = 107;
    pub const CLK_TOP_CAN3: usize = 75;
    pub const CLK_TOP_ADC0: usize = 133;
    pub const CLK_TOP_I2C2: usize = 90;
    pub const LVB0: usize = 350;
    pub const URT2: usize = 294;
    pub const TSNS: usize = 330;
    pub const CLK_TOP_CPU0: usize = 64;
    pub const CLK_TOP_CAM1: usize = 128;
    pub const CLK_TOP_TMR5: usize = 109;
    pub const CAM0: usize = 338;
    pub const CLK_TOP_URT3: usize = 99;
    pub const DSI1: usize = 349;
    pub const CLK_SRC_CLK0_PLL2: usize = 39;
    pub const CLK_SRC_CLK0_PLL3: usize = 42;
    pub const PDMA: usize = 340;
    pub const CLK_TOP_I2S0: usize = 135;
    pub const CLK_TOP_CSI1: usize = 132;
    pub const LIN2: usize = 278;
    pub const I2C1: usize = 285;
    pub const CLK_TOP_LCD0: usize = 129;
    pub const CLK_TOP_LCD1: usize = 130;
    pub const REF1: usize = 337;
    pub const CLK_TOP_TMR4: usize = 108;
    pub const CLK_TOP_GPU0: usize = 66;
    pub const CLK_SRC_PLL2: usize = 38;
    pub const CLK_TOP_CAN6: usize = 78;
    pub const CLK_TOP_I2C3: usize = 91;
    pub const I2C3: usize = 287;
    pub const I2S2: usize = 314;
    pub const HDMA: usize = 326;
    pub const CLK_TOP_LIN2: usize = 82;
    pub const CLK_TOP_AUD2: usize = 118;
    pub const TMR1: usize = 305;
    pub const CLK_TOP_ETH0: usize = 120;
    pub const LIN3: usize = 279;
    pub const TMR5: usize = 309;
    pub const CLK_SRC_PLL1: usize = 35;
    pub const URT0: usize = 292;
    pub const CLK_TOP_XRAM: usize = 113;
    pub const SPI0: usize = 288;
    pub const CAN5: usize = 270;
    pub const TMR0: usize = 304;
    pub const CLK_SRC_CLK0_PLL0: usize = 34;
    pub const CLK_TOP_LIN0: usize = 80;
    pub const CLK_TOP_AUD1: usize = 117;
    pub const CLK_TOP_TMR0: usize = 104;
    pub const MCT0: usize = 261;
    pub const I2C2: usize = 286;
    pub const POW_CPU0: usize = 22;
    pub const GWC0: usize = 344;
    pub const CLK_SRC_CLK0_PLL1: usize = 36;
    pub const LIN1: usize = 277;
    pub const CAN2: usize = 267;
    pub const CLK_SRC_PLL1_REF: usize = 46;
    pub const CAN0: usize = 265;
}
pub mod clocks {
    pub const CSI0: usize = 67;
    pub const AXIS: usize = 4;
    pub const SPI2: usize = 30;
    pub const ANA0: usize = 50;
    pub const LIN3: usize = 19;
    pub const URT1: usize = 33;
    pub const ANA1: usize = 51;
    pub const TMR7: usize = 47;
    pub const ETH0: usize = 56;
    pub const CAN4: usize = 12;
    pub const LIN1: usize = 17;
    pub const CAM1: usize = 64;
    pub const URT6: usize = 38;
    pub const CAN0: usize = 8;
    pub const SPI0: usize = 28;
    pub const XRAM: usize = 49;
    pub const LIN4: usize = 20;
    pub const XPI0: usize = 48;
    pub const REF0: usize = 61;
    pub const LCD1: usize = 66;
    pub const AXID: usize = 7;
    pub const LIN5: usize = 21;
    pub const SDC1: usize = 59;
    pub const URT2: usize = 34;
    pub const LCD0: usize = 65;
    pub const I2C3: usize = 27;
    pub const URT5: usize = 37;
    pub const AXIC: usize = 5;
    pub const CAN6: usize = 14;
    pub const URT4: usize = 36;
    pub const PTP0: usize = 57;
    pub const CAN1: usize = 9;
    pub const AXIF: usize = 3;
    pub const LIN7: usize = 23;
    pub const AUD0: usize = 52;
    pub const NTM0: usize = 60;
    pub const I2C2: usize = 26;
    pub const TMR6: usize = 46;
    pub const GPU0: usize = 2;
    pub const CPU0: usize = 0;
    pub const I2C1: usize = 25;
    pub const SPI3: usize = 31;
    pub const CAN2: usize = 10;
    pub const SPI1: usize = 29;
    pub const LIN2: usize = 18;
    pub const URT3: usize = 35;
    pub const CAN7: usize = 15;
    pub const TMR1: usize = 41;
    pub const REF1: usize = 62;
    pub const TMR3: usize = 43;
    pub const TMR0: usize = 40;
    pub const SDC0: usize = 58;
    pub const URT7: usize = 39;
    pub const MCT0: usize = 1;
    pub const CAN3: usize = 11;
    pub const AUD2: usize = 54;
    pub const LIN6: usize = 22;
    pub const AXIV: usize = 6;
    pub const TMR5: usize = 45;
    pub const CSI1: usize = 68;
    pub const AUD1: usize = 53;
    pub const LIN0: usize = 16;
    pub const TMR4: usize = 44;
    pub const I2C0: usize = 24;
    pub const TMR2: usize = 42;
    pub const AUD3: usize = 55;
    pub const CAM0: usize = 63;
    pub const CAN5: usize = 13;
    pub const URT0: usize = 32;
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
    pub const PF11: usize = 171;
    pub const PF12: usize = 172;
    pub const PF13: usize = 173;
    pub const PF14: usize = 174;
    pub const PF15: usize = 175;
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
    pub const PX12: usize = 428;
    pub const PX13: usize = 429;
    pub const PX14: usize = 430;
    pub const PX15: usize = 431;
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
    pub const PY12: usize = 460;
    pub const PY13: usize = 461;
    pub const PY14: usize = 462;
    pub const PY15: usize = 463;
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
    pub const PZ12: usize = 492;
    pub const PZ13: usize = 493;
    pub const PZ14: usize = 494;
    pub const PZ15: usize = 495;
}
