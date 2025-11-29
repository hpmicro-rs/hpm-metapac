#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    #[doc = "7 - GPIO1_A"]
    GPIO1_A = 7,
    #[doc = "8 - GPIO1_B"]
    GPIO1_B = 8,
    #[doc = "9 - GPIO1_C"]
    GPIO1_C = 9,
    #[doc = "10 - GPIO1_D"]
    GPIO1_D = 10,
    #[doc = "11 - GPIO1_X"]
    GPIO1_X = 11,
    #[doc = "12 - GPIO1_Y"]
    GPIO1_Y = 12,
    #[doc = "13 - GPTMR0"]
    GPTMR0 = 13,
    #[doc = "14 - GPTMR1"]
    GPTMR1 = 14,
    #[doc = "15 - GPTMR2"]
    GPTMR2 = 15,
    #[doc = "16 - GPTMR3"]
    GPTMR3 = 16,
    #[doc = "21 - UART0"]
    UART0 = 21,
    #[doc = "22 - UART1"]
    UART1 = 22,
    #[doc = "23 - UART2"]
    UART2 = 23,
    #[doc = "24 - UART3"]
    UART3 = 24,
    #[doc = "25 - UART4"]
    UART4 = 25,
    #[doc = "26 - UART5"]
    UART5 = 26,
    #[doc = "27 - UART6"]
    UART6 = 27,
    #[doc = "28 - UART7"]
    UART7 = 28,
    #[doc = "29 - I2C0"]
    I2C0 = 29,
    #[doc = "30 - I2C1"]
    I2C1 = 30,
    #[doc = "31 - I2C2"]
    I2C2 = 31,
    #[doc = "32 - I2C3"]
    I2C3 = 32,
    #[doc = "33 - SPI0"]
    SPI0 = 33,
    #[doc = "34 - SPI1"]
    SPI1 = 34,
    #[doc = "35 - SPI2"]
    SPI2 = 35,
    #[doc = "36 - SPI3"]
    SPI3 = 36,
    #[doc = "37 - TSNS"]
    TSNS = 37,
    #[doc = "38 - MBX0A"]
    MBX0A = 38,
    #[doc = "39 - MBX0B"]
    MBX0B = 39,
    #[doc = "40 - MBX1A"]
    MBX1A = 40,
    #[doc = "41 - MBX1B"]
    MBX1B = 41,
    #[doc = "42 - EWDG0"]
    EWDG0 = 42,
    #[doc = "43 - EWDG1"]
    EWDG1 = 43,
    #[doc = "44 - EWDG2"]
    EWDG2 = 44,
    #[doc = "45 - EWDG3"]
    EWDG3 = 45,
    #[doc = "46 - HDMA"]
    HDMA = 46,
    #[doc = "47 - LOBS"]
    LOBS = 47,
    #[doc = "48 - ADC0"]
    ADC0 = 48,
    #[doc = "49 - ADC1"]
    ADC1 = 49,
    #[doc = "50 - ADC2"]
    ADC2 = 50,
    #[doc = "51 - ADC3"]
    ADC3 = 51,
    #[doc = "52 - DAC0"]
    DAC0 = 52,
    #[doc = "53 - DAC1"]
    DAC1 = 53,
    #[doc = "54 - ACMP0_0"]
    ACMP0_0 = 54,
    #[doc = "55 - ACMP0_1"]
    ACMP0_1 = 55,
    #[doc = "56 - ACMP1_0"]
    ACMP1_0 = 56,
    #[doc = "57 - ACMP1_1"]
    ACMP1_1 = 57,
    #[doc = "58 - ACMP2_0"]
    ACMP2_0 = 58,
    #[doc = "59 - ACMP2_1"]
    ACMP2_1 = 59,
    #[doc = "60 - ACMP3_0"]
    ACMP3_0 = 60,
    #[doc = "61 - ACMP3_1"]
    ACMP3_1 = 61,
    #[doc = "62 - I2S0"]
    I2S0 = 62,
    #[doc = "63 - I2S1"]
    I2S1 = 63,
    #[doc = "64 - DAO"]
    DAO = 64,
    #[doc = "65 - PDM"]
    PDM = 65,
    #[doc = "66 - MCAN0"]
    MCAN0 = 66,
    #[doc = "67 - MCAN1"]
    MCAN1 = 67,
    #[doc = "68 - MCAN2"]
    MCAN2 = 68,
    #[doc = "69 - MCAN3"]
    MCAN3 = 69,
    #[doc = "70 - PTPC"]
    PTPC = 70,
    #[doc = "71 - QEI0"]
    QEI0 = 71,
    #[doc = "72 - QEI1"]
    QEI1 = 72,
    #[doc = "73 - PWM0"]
    PWM0 = 73,
    #[doc = "74 - PWM1"]
    PWM1 = 74,
    #[doc = "75 - PWM2"]
    PWM2 = 75,
    #[doc = "76 - PWM3"]
    PWM3 = 76,
    #[doc = "77 - RDC0"]
    RDC0 = 77,
    #[doc = "78 - SDM0"]
    SDM0 = 78,
    #[doc = "79 - SEI0_0"]
    SEI0_0 = 79,
    #[doc = "80 - SEI0_1"]
    SEI0_1 = 80,
    #[doc = "81 - MTG0"]
    MTG0 = 81,
    #[doc = "82 - VSC0"]
    VSC0 = 82,
    #[doc = "83 - CLC0_0"]
    CLC0_0 = 83,
    #[doc = "84 - CLC0_1"]
    CLC0_1 = 84,
    #[doc = "85 - TRGMUX0"]
    TRGMUX0 = 85,
    #[doc = "86 - TRGMUX1"]
    TRGMUX1 = 86,
    #[doc = "87 - ENET0"]
    ENET0 = 87,
    #[doc = "88 - NTMR0"]
    NTMR0 = 88,
    #[doc = "89 - USB0"]
    USB0 = 89,
    #[doc = "90 - XPI0"]
    XPI0 = 90,
    #[doc = "91 - FEMC"]
    FEMC = 91,
    #[doc = "92 - PPI"]
    PPI = 92,
    #[doc = "93 - XDMA"]
    XDMA = 93,
    #[doc = "94 - FFA"]
    FFA = 94,
    #[doc = "95 - SDP"]
    SDP = 95,
    #[doc = "96 - RNG"]
    RNG = 96,
    #[doc = "97 - PSEC"]
    PSEC = 97,
    #[doc = "98 - PGPIO"]
    PGPIO = 98,
    #[doc = "99 - PEWDG"]
    PEWDG = 99,
    #[doc = "100 - PTMR"]
    PTMR = 100,
    #[doc = "101 - PUART"]
    PUART = 101,
    #[doc = "102 - FUSE"]
    FUSE = 102,
    #[doc = "103 - SECMON"]
    SECMON = 103,
    #[doc = "104 - PAD_WAKEUP"]
    PAD_WAKEUP = 104,
    #[doc = "105 - BROWNOUT"]
    BROWNOUT = 105,
    #[doc = "106 - SYSCTL"]
    SYSCTL = 106,
    #[doc = "107 - CPU0"]
    CPU0 = 107,
    #[doc = "108 - CPU1"]
    CPU1 = 108,
    #[doc = "109 - DEBUG0"]
    DEBUG0 = 109,
    #[doc = "110 - DEBUG1"]
    DEBUG1 = 110,
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn CORE_LOCAL();
        fn GPIO0_A();
        fn GPIO0_B();
        fn GPIO0_C();
        fn GPIO0_D();
        fn GPIO0_X();
        fn GPIO0_Y();
        fn GPIO1_A();
        fn GPIO1_B();
        fn GPIO1_C();
        fn GPIO1_D();
        fn GPIO1_X();
        fn GPIO1_Y();
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
        fn DAC0();
        fn DAC1();
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
        fn MCAN0();
        fn MCAN1();
        fn MCAN2();
        fn MCAN3();
        fn PTPC();
        fn QEI0();
        fn QEI1();
        fn PWM0();
        fn PWM1();
        fn PWM2();
        fn PWM3();
        fn RDC0();
        fn SDM0();
        fn SEI0_0();
        fn SEI0_1();
        fn MTG0();
        fn VSC0();
        fn CLC0_0();
        fn CLC0_1();
        fn TRGMUX0();
        fn TRGMUX1();
        fn ENET0();
        fn NTMR0();
        fn USB0();
        fn XPI0();
        fn FEMC();
        fn PPI();
        fn XDMA();
        fn FFA();
        fn SDP();
        fn RNG();
        fn PSEC();
        fn PGPIO();
        fn PEWDG();
        fn PTMR();
        fn PUART();
        fn FUSE();
        fn SECMON();
        fn PAD_WAKEUP();
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
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 111] = [
        Vector {
            _handler: CORE_LOCAL,
        },
        Vector { _handler: GPIO0_A },
        Vector { _handler: GPIO0_B },
        Vector { _handler: GPIO0_C },
        Vector { _handler: GPIO0_D },
        Vector { _handler: GPIO0_X },
        Vector { _handler: GPIO0_Y },
        Vector { _handler: GPIO1_A },
        Vector { _handler: GPIO1_B },
        Vector { _handler: GPIO1_C },
        Vector { _handler: GPIO1_D },
        Vector { _handler: GPIO1_X },
        Vector { _handler: GPIO1_Y },
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
        Vector { _handler: DAC0 },
        Vector { _handler: DAC1 },
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
        Vector { _handler: MCAN0 },
        Vector { _handler: MCAN1 },
        Vector { _handler: MCAN2 },
        Vector { _handler: MCAN3 },
        Vector { _handler: PTPC },
        Vector { _handler: QEI0 },
        Vector { _handler: QEI1 },
        Vector { _handler: PWM0 },
        Vector { _handler: PWM1 },
        Vector { _handler: PWM2 },
        Vector { _handler: PWM3 },
        Vector { _handler: RDC0 },
        Vector { _handler: SDM0 },
        Vector { _handler: SEI0_0 },
        Vector { _handler: SEI0_1 },
        Vector { _handler: MTG0 },
        Vector { _handler: VSC0 },
        Vector { _handler: CLC0_0 },
        Vector { _handler: CLC0_1 },
        Vector { _handler: TRGMUX0 },
        Vector { _handler: TRGMUX1 },
        Vector { _handler: ENET0 },
        Vector { _handler: NTMR0 },
        Vector { _handler: USB0 },
        Vector { _handler: XPI0 },
        Vector { _handler: FEMC },
        Vector { _handler: PPI },
        Vector { _handler: XDMA },
        Vector { _handler: FFA },
        Vector { _handler: SDP },
        Vector { _handler: RNG },
        Vector { _handler: PSEC },
        Vector { _handler: PGPIO },
        Vector { _handler: PEWDG },
        Vector { _handler: PTMR },
        Vector { _handler: PUART },
        Vector { _handler: FUSE },
        Vector { _handler: SECMON },
        Vector {
            _handler: PAD_WAKEUP,
        },
        Vector { _handler: BROWNOUT },
        Vector { _handler: SYSCTL },
        Vector { _handler: CPU0 },
        Vector { _handler: CPU1 },
        Vector { _handler: DEBUG0 },
        Vector { _handler: DEBUG1 },
    ];
}
pub const PLIC: plic::Plic = unsafe { plic::Plic::from_ptr(0xe400_0000usize as _) };
pub const MCHTMR: mchtmr::Mchtmr = unsafe { mchtmr::Mchtmr::from_ptr(0xe600_0000usize as _) };
pub const PLICSW: plicsw::Plicsw = unsafe { plicsw::Plicsw::from_ptr(0xe640_0000usize as _) };
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod common {
    use core::marker::PhantomData;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RW;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct R;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        #[allow(clippy::missing_safety_doc)]
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
            Self {
                ptr: ptr as _,
                phantom: PhantomData,
            }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe { (self.ptr as *mut T).read_volatile() }
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write_value(&self, val: T) {
            unsafe { (self.ptr as *mut T).write_volatile(val) }
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write(&self, f: impl FnOnce(&mut T)) {
            let mut val = Default::default();
            f(&mut val);
            self.write_value(val);
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify(&self, f: impl FnOnce(&mut T)) {
            let mut val = self.read();
            f(&mut val);
            self.write_value(val);
        }
    }
}
#[cfg(feature = "rt")]
unsafe impl riscv_rt::InterruptNumber for Interrupt {
    const MAX_INTERRUPT_NUMBER: usize = 1024;
    #[inline(always)]
    fn number(self) -> usize {
        self as usize
    }
    #[inline(always)]
    fn from_number(value: usize) -> Result<Self, riscv_rt::result::Error> {
        match value {
            0 => Ok(Self::CORE_LOCAL),
            1 => Ok(Self::GPIO0_A),
            2 => Ok(Self::GPIO0_B),
            3 => Ok(Self::GPIO0_C),
            4 => Ok(Self::GPIO0_D),
            5 => Ok(Self::GPIO0_X),
            6 => Ok(Self::GPIO0_Y),
            7 => Ok(Self::GPIO1_A),
            8 => Ok(Self::GPIO1_B),
            9 => Ok(Self::GPIO1_C),
            10 => Ok(Self::GPIO1_D),
            11 => Ok(Self::GPIO1_X),
            12 => Ok(Self::GPIO1_Y),
            13 => Ok(Self::GPTMR0),
            14 => Ok(Self::GPTMR1),
            15 => Ok(Self::GPTMR2),
            16 => Ok(Self::GPTMR3),
            21 => Ok(Self::UART0),
            22 => Ok(Self::UART1),
            23 => Ok(Self::UART2),
            24 => Ok(Self::UART3),
            25 => Ok(Self::UART4),
            26 => Ok(Self::UART5),
            27 => Ok(Self::UART6),
            28 => Ok(Self::UART7),
            29 => Ok(Self::I2C0),
            30 => Ok(Self::I2C1),
            31 => Ok(Self::I2C2),
            32 => Ok(Self::I2C3),
            33 => Ok(Self::SPI0),
            34 => Ok(Self::SPI1),
            35 => Ok(Self::SPI2),
            36 => Ok(Self::SPI3),
            37 => Ok(Self::TSNS),
            38 => Ok(Self::MBX0A),
            39 => Ok(Self::MBX0B),
            40 => Ok(Self::MBX1A),
            41 => Ok(Self::MBX1B),
            42 => Ok(Self::EWDG0),
            43 => Ok(Self::EWDG1),
            44 => Ok(Self::EWDG2),
            45 => Ok(Self::EWDG3),
            46 => Ok(Self::HDMA),
            47 => Ok(Self::LOBS),
            48 => Ok(Self::ADC0),
            49 => Ok(Self::ADC1),
            50 => Ok(Self::ADC2),
            51 => Ok(Self::ADC3),
            52 => Ok(Self::DAC0),
            53 => Ok(Self::DAC1),
            54 => Ok(Self::ACMP0_0),
            55 => Ok(Self::ACMP0_1),
            56 => Ok(Self::ACMP1_0),
            57 => Ok(Self::ACMP1_1),
            58 => Ok(Self::ACMP2_0),
            59 => Ok(Self::ACMP2_1),
            60 => Ok(Self::ACMP3_0),
            61 => Ok(Self::ACMP3_1),
            62 => Ok(Self::I2S0),
            63 => Ok(Self::I2S1),
            64 => Ok(Self::DAO),
            65 => Ok(Self::PDM),
            66 => Ok(Self::MCAN0),
            67 => Ok(Self::MCAN1),
            68 => Ok(Self::MCAN2),
            69 => Ok(Self::MCAN3),
            70 => Ok(Self::PTPC),
            71 => Ok(Self::QEI0),
            72 => Ok(Self::QEI1),
            73 => Ok(Self::PWM0),
            74 => Ok(Self::PWM1),
            75 => Ok(Self::PWM2),
            76 => Ok(Self::PWM3),
            77 => Ok(Self::RDC0),
            78 => Ok(Self::SDM0),
            79 => Ok(Self::SEI0_0),
            80 => Ok(Self::SEI0_1),
            81 => Ok(Self::MTG0),
            82 => Ok(Self::VSC0),
            83 => Ok(Self::CLC0_0),
            84 => Ok(Self::CLC0_1),
            85 => Ok(Self::TRGMUX0),
            86 => Ok(Self::TRGMUX1),
            87 => Ok(Self::ENET0),
            88 => Ok(Self::NTMR0),
            89 => Ok(Self::USB0),
            90 => Ok(Self::XPI0),
            91 => Ok(Self::FEMC),
            92 => Ok(Self::PPI),
            93 => Ok(Self::XDMA),
            94 => Ok(Self::FFA),
            95 => Ok(Self::SDP),
            96 => Ok(Self::RNG),
            97 => Ok(Self::PSEC),
            98 => Ok(Self::PGPIO),
            99 => Ok(Self::PEWDG),
            100 => Ok(Self::PTMR),
            101 => Ok(Self::PUART),
            102 => Ok(Self::FUSE),
            103 => Ok(Self::SECMON),
            104 => Ok(Self::PAD_WAKEUP),
            105 => Ok(Self::BROWNOUT),
            106 => Ok(Self::SYSCTL),
            107 => Ok(Self::CPU0),
            108 => Ok(Self::CPU1),
            109 => Ok(Self::DEBUG0),
            110 => Ok(Self::DEBUG1),

            _ => Err(riscv_rt::result::Error::InvalidVariant(value)),
        }
    }
}
unsafe impl riscv_rt::ExternalInterruptNumber for Interrupt {}
#[path = "../../peripherals/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../../peripherals/plic_common.rs"]
pub mod plic;
#[path = "../../peripherals/plicsw_common.rs"]
pub mod plicsw;
pub const FLASH_BASE: usize = 2147483648;
pub const FLASH_SIZE: usize = 268435456;
pub mod resources {
    //! `SYSCTL.RESOURCE` definitions
    pub const CPU0: usize = 0;
    pub const CPX0: usize = 1;
    pub const POW_CPU0: usize = 21;
    pub const RST_SOC: usize = 22;
    pub const RST_CPU0: usize = 23;
    pub const CLK_SRC_XTAL: usize = 32;
    pub const CLK_SRC_PLL0: usize = 33;
    pub const CLK_SRC_CLK0_PLL0: usize = 34;
    pub const CLK_SRC_CLK1_PLL0: usize = 35;
    pub const CLK_SRC_CLK2_PLL0: usize = 36;
    pub const CLK_SRC_PLL1: usize = 37;
    pub const CLK_SRC_CLK0_PLL1: usize = 38;
    pub const CLK_SRC_CLK1_PLL1: usize = 39;
    pub const CLK_SRC_PLL2: usize = 40;
    pub const CLK_SRC_CLK0_PLL2: usize = 41;
    pub const CLK_SRC_CLK1_PLL2: usize = 42;
    pub const CLK_SRC_PLL0_REF: usize = 43;
    pub const CLK_SRC_PLL1_REF: usize = 44;
    pub const CLK_SRC_PLL2_REF: usize = 45;
    pub const CLK_TOP_CPU0: usize = 64;
    pub const CLK_TOP_MCT0: usize = 65;
    pub const CLK_TOP_FEMC: usize = 66;
    pub const CLK_TOP_XPI0: usize = 67;
    pub const CLK_TOP_XPI1: usize = 68;
    pub const CLK_TOP_TMR0: usize = 69;
    pub const CLK_TOP_TMR1: usize = 70;
    pub const CLK_TOP_TMR2: usize = 71;
    pub const CLK_TOP_TMR3: usize = 72;
    pub const CLK_TOP_URT0: usize = 73;
    pub const CLK_TOP_URT1: usize = 74;
    pub const CLK_TOP_URT2: usize = 75;
    pub const CLK_TOP_URT3: usize = 76;
    pub const CLK_TOP_URT4: usize = 77;
    pub const CLK_TOP_URT5: usize = 78;
    pub const CLK_TOP_URT6: usize = 79;
    pub const CLK_TOP_URT7: usize = 80;
    pub const CLK_TOP_I2C0: usize = 81;
    pub const CLK_TOP_I2C1: usize = 82;
    pub const CLK_TOP_I2C2: usize = 83;
    pub const CLK_TOP_I2C3: usize = 84;
    pub const CLK_TOP_SPI0: usize = 85;
    pub const CLK_TOP_SPI1: usize = 86;
    pub const CLK_TOP_SPI2: usize = 87;
    pub const CLK_TOP_SPI3: usize = 88;
    pub const CLK_TOP_CAN0: usize = 89;
    pub const CLK_TOP_CAN1: usize = 90;
    pub const CLK_TOP_PTPC: usize = 91;
    pub const CLK_TOP_ANA0: usize = 92;
    pub const CLK_TOP_ANA1: usize = 93;
    pub const CLK_TOP_ANA2: usize = 94;
    pub const CLK_TOP_ANA3: usize = 95;
    pub const CLK_TOP_AUD0: usize = 96;
    pub const CLK_TOP_AUD1: usize = 97;
    pub const CLK_TOP_ETH0: usize = 98;
    pub const CLK_TOP_PTP0: usize = 99;
    pub const CLK_TOP_REF0: usize = 100;
    pub const CLK_TOP_REF1: usize = 101;
    pub const CLK_TOP_NTM0: usize = 102;
    pub const CLK_TOP_SDC0: usize = 103;
    pub const CLK_TOP_ADC0: usize = 128;
    pub const CLK_TOP_ADC1: usize = 129;
    pub const CLK_TOP_ADC2: usize = 130;
    pub const CLK_TOP_DAC0: usize = 131;
    pub const CLK_TOP_I2S0: usize = 132;
    pub const CLK_TOP_I2S1: usize = 133;
    pub const AHBP: usize = 256;
    pub const AXIS: usize = 257;
    pub const AXIC: usize = 258;
    pub const FEMC: usize = 259;
    pub const ROM0: usize = 260;
    pub const LMM0: usize = 261;
    pub const RAM0: usize = 262;
    pub const MCT0: usize = 263;
    pub const XPI0: usize = 264;
    pub const XPI1: usize = 265;
    pub const SDP0: usize = 266;
    pub const RNG0: usize = 267;
    pub const KMAN: usize = 268;
    pub const DMA0: usize = 269;
    pub const DMA1: usize = 270;
    pub const FFA0: usize = 271;
    pub const GPIO: usize = 272;
    pub const MBX0: usize = 273;
    pub const WDG0: usize = 274;
    pub const WDG1: usize = 275;
    pub const TSNS: usize = 276;
    pub const TMR0: usize = 277;
    pub const TMR1: usize = 278;
    pub const TMR2: usize = 279;
    pub const TMR3: usize = 280;
    pub const URT0: usize = 281;
    pub const URT1: usize = 282;
    pub const URT2: usize = 283;
    pub const URT3: usize = 284;
    pub const URT4: usize = 285;
    pub const URT5: usize = 286;
    pub const URT6: usize = 287;
    pub const URT7: usize = 288;
    pub const I2C0: usize = 289;
    pub const I2C1: usize = 290;
    pub const I2C2: usize = 291;
    pub const I2C3: usize = 292;
    pub const SPI0: usize = 293;
    pub const SPI1: usize = 294;
    pub const SPI2: usize = 295;
    pub const SPI3: usize = 296;
    pub const CAN0: usize = 297;
    pub const CAN1: usize = 298;
    pub const PTPC: usize = 299;
    pub const ADC0: usize = 300;
    pub const ADC1: usize = 301;
    pub const ADC2: usize = 302;
    pub const DAC0: usize = 303;
    pub const ACMP: usize = 304;
    pub const I2S0: usize = 305;
    pub const I2S1: usize = 306;
    pub const PDM0: usize = 307;
    pub const DAO: usize = 308;
    pub const SYNT: usize = 309;
    pub const MOT0: usize = 310;
    pub const MOT1: usize = 311;
    pub const ETH0: usize = 312;
    pub const NTM0: usize = 313;
    pub const SDC0: usize = 314;
    pub const USB0: usize = 315;
    pub const REF0: usize = 316;
    pub const REF1: usize = 317;
}
pub mod clocks {
    //! `SYSCTL.CLOCK` definitions
    pub const MCT0: usize = 0;
    pub const FEMC: usize = 1;
    pub const XPI0: usize = 2;
    pub const XPI1: usize = 3;
    pub const TMR0: usize = 4;
    pub const TMR1: usize = 5;
    pub const TMR2: usize = 6;
    pub const TMR3: usize = 7;
    pub const URT0: usize = 8;
    pub const URT1: usize = 9;
    pub const URT2: usize = 10;
    pub const URT3: usize = 11;
    pub const URT4: usize = 12;
    pub const URT5: usize = 13;
    pub const URT6: usize = 14;
    pub const URT7: usize = 15;
    pub const I2C0: usize = 16;
    pub const I2C1: usize = 17;
    pub const I2C2: usize = 18;
    pub const I2C3: usize = 19;
    pub const SPI0: usize = 20;
    pub const SPI1: usize = 21;
    pub const SPI2: usize = 22;
    pub const SPI3: usize = 23;
    pub const CAN0: usize = 24;
    pub const CAN1: usize = 25;
    pub const PTPC: usize = 26;
    pub const ANA0: usize = 27;
    pub const ANA1: usize = 28;
    pub const ANA2: usize = 29;
    pub const ANA3: usize = 30;
    pub const AUD0: usize = 31;
    pub const AUD1: usize = 32;
    pub const ETH0: usize = 33;
    pub const PTP0: usize = 34;
    pub const REF0: usize = 35;
    pub const REF1: usize = 36;
    pub const NTM0: usize = 37;
    pub const SDC0: usize = 38;
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
}
pub mod iomux {
    //! `FUNC_CTL` function mux definitions
    pub const IOC_PA00_FUNC_CTL_ETH0_EVTI_0: u8 = 25;
    pub const IOC_PA00_FUNC_CTL_GPIO_A_00: u8 = 0;
    pub const IOC_PA00_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PA00_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PA00_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PA00_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PA00_FUNC_CTL_RDC0_PWM_N: u8 = 20;
    pub const IOC_PA00_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PA00_FUNC_CTL_SYSCTL_CLK_OBS_0: u8 = 24;
    pub const IOC_PA00_FUNC_CTL_TRGM_P_00: u8 = 17;
    pub const IOC_PA00_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PA01_FUNC_CTL_ETH0_EVTO_0: u8 = 25;
    pub const IOC_PA01_FUNC_CTL_GPIO_A_01: u8 = 0;
    pub const IOC_PA01_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PA01_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PA01_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PA01_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PA01_FUNC_CTL_RDC0_PWM_P: u8 = 20;
    pub const IOC_PA01_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PA01_FUNC_CTL_SYSCTL_CLK_OBS_1: u8 = 24;
    pub const IOC_PA01_FUNC_CTL_TRGM_P_01: u8 = 17;
    pub const IOC_PA01_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PA02_FUNC_CTL_ETH0_EVTI_1: u8 = 25;
    pub const IOC_PA02_FUNC_CTL_GPIO_A_02: u8 = 0;
    pub const IOC_PA02_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PA02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PA02_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PA02_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PA02_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PA02_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PA02_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PA02_FUNC_CTL_SYSCTL_CLK_OBS_2: u8 = 24;
    pub const IOC_PA02_FUNC_CTL_TRGM_P_02: u8 = 17;
    pub const IOC_PA02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PA02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PA03_FUNC_CTL_ETH0_EVTO_1: u8 = 25;
    pub const IOC_PA03_FUNC_CTL_GPIO_A_03: u8 = 0;
    pub const IOC_PA03_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PA03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PA03_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PA03_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PA03_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PA03_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PA03_FUNC_CTL_SPI0_CS_3: u8 = 5;
    pub const IOC_PA03_FUNC_CTL_SYSCTL_CLK_OBS_3: u8 = 24;
    pub const IOC_PA03_FUNC_CTL_TRGM_P_03: u8 = 17;
    pub const IOC_PA03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PA04_FUNC_CTL_GPIO_A_04: u8 = 0;
    pub const IOC_PA04_FUNC_CTL_JTAG_TDO: u8 = 24;
    pub const IOC_PA04_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PA04_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PA04_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PA04_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PA04_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PA04_FUNC_CTL_TRGM_P_04: u8 = 17;
    pub const IOC_PA04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PA05_FUNC_CTL_ETH0_RXCK: u8 = 18;
    pub const IOC_PA05_FUNC_CTL_GPIO_A_05: u8 = 0;
    pub const IOC_PA05_FUNC_CTL_GPTMR1_COMP_2: u8 = 1;
    pub const IOC_PA05_FUNC_CTL_JTAG_TDI: u8 = 24;
    pub const IOC_PA05_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PA05_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PA05_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PA05_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PA05_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PA05_FUNC_CTL_SPI1_CS_0: u8 = 5;
    pub const IOC_PA05_FUNC_CTL_TRGM_P_05: u8 = 17;
    pub const IOC_PA05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PA05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PA06_FUNC_CTL_ETH0_RXDV: u8 = 18;
    pub const IOC_PA06_FUNC_CTL_FEMC_SCLK_0: u8 = 12;
    pub const IOC_PA06_FUNC_CTL_GPIO_A_06: u8 = 0;
    pub const IOC_PA06_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PA06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PA06_FUNC_CTL_JTAG_TCK: u8 = 24;
    pub const IOC_PA06_FUNC_CTL_PPI0_DQ_29: u8 = 13;
    pub const IOC_PA06_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PA06_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PA06_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PA06_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PA06_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PA06_FUNC_CTL_TRGM_P_06: u8 = 17;
    pub const IOC_PA06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PA07_FUNC_CTL_ETH0_RXER: u8 = 18;
    pub const IOC_PA07_FUNC_CTL_FEMC_SCS_0: u8 = 12;
    pub const IOC_PA07_FUNC_CTL_GPIO_A_07: u8 = 0;
    pub const IOC_PA07_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PA07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PA07_FUNC_CTL_JTAG_TMS: u8 = 24;
    pub const IOC_PA07_FUNC_CTL_PPI0_DQ_30: u8 = 13;
    pub const IOC_PA07_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PA07_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PA07_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PA07_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PA07_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PA07_FUNC_CTL_TRGM_P_07: u8 = 17;
    pub const IOC_PA07_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PA08_FUNC_CTL_ETH0_RXD_0: u8 = 18;
    pub const IOC_PA08_FUNC_CTL_FEMC_CS_1: u8 = 12;
    pub const IOC_PA08_FUNC_CTL_GPIO_A_08: u8 = 0;
    pub const IOC_PA08_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PA08_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PA08_FUNC_CTL_JTAG_TRST: u8 = 24;
    pub const IOC_PA08_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PA08_FUNC_CTL_PPI0_CS_1: u8 = 13;
    pub const IOC_PA08_FUNC_CTL_PWM1_P_0: u8 = 16;
    pub const IOC_PA08_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PA08_FUNC_CTL_RDC0_PWM_N: u8 = 20;
    pub const IOC_PA08_FUNC_CTL_SDM0_CLK_0: u8 = 23;
    pub const IOC_PA08_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PA08_FUNC_CTL_SPI0_CS_2: u8 = 5;
    pub const IOC_PA08_FUNC_CTL_TRGM_P_08: u8 = 17;
    pub const IOC_PA08_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PA09_FUNC_CTL_ETH0_RXD_1: u8 = 18;
    pub const IOC_PA09_FUNC_CTL_FEMC_WE: u8 = 12;
    pub const IOC_PA09_FUNC_CTL_GPIO_A_09: u8 = 0;
    pub const IOC_PA09_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PA09_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PA09_FUNC_CTL_I2S1_MCLK: u8 = 8;
    pub const IOC_PA09_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PA09_FUNC_CTL_PPI0_CTR_2: u8 = 13;
    pub const IOC_PA09_FUNC_CTL_PWM1_P_1: u8 = 16;
    pub const IOC_PA09_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PA09_FUNC_CTL_RDC0_PWM_P: u8 = 20;
    pub const IOC_PA09_FUNC_CTL_SDM0_DAT_0: u8 = 23;
    pub const IOC_PA09_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PA09_FUNC_CTL_SPI0_CS_1: u8 = 5;
    pub const IOC_PA09_FUNC_CTL_TRGM_P_09: u8 = 17;
    pub const IOC_PA09_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PA09_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PA10_FUNC_CTL_CPU0_NMI: u8 = 24;
    pub const IOC_PA10_FUNC_CTL_ETH0_RXD_2: u8 = 18;
    pub const IOC_PA10_FUNC_CTL_FEMC_CAS: u8 = 12;
    pub const IOC_PA10_FUNC_CTL_GPIO_A_10: u8 = 0;
    pub const IOC_PA10_FUNC_CTL_GPTMR0_COMP_2: u8 = 1;
    pub const IOC_PA10_FUNC_CTL_I2S1_BCLK: u8 = 8;
    pub const IOC_PA10_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PA10_FUNC_CTL_PPI0_DM_2: u8 = 13;
    pub const IOC_PA10_FUNC_CTL_PWM1_P_2: u8 = 16;
    pub const IOC_PA10_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PA10_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PA10_FUNC_CTL_SDM0_CLK_1: u8 = 23;
    pub const IOC_PA10_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PA10_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PA10_FUNC_CTL_TRGM_P_10: u8 = 17;
    pub const IOC_PA10_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PA10_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PA10_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PA11_FUNC_CTL_CPU1_NMI: u8 = 24;
    pub const IOC_PA11_FUNC_CTL_ETH0_RXD_3: u8 = 18;
    pub const IOC_PA11_FUNC_CTL_FEMC_RAS: u8 = 12;
    pub const IOC_PA11_FUNC_CTL_GPIO_A_11: u8 = 0;
    pub const IOC_PA11_FUNC_CTL_I2S1_FCLK: u8 = 8;
    pub const IOC_PA11_FUNC_CTL_PPI0_DM_3: u8 = 13;
    pub const IOC_PA11_FUNC_CTL_PWM1_P_3: u8 = 16;
    pub const IOC_PA11_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PA11_FUNC_CTL_SDM0_DAT_1: u8 = 23;
    pub const IOC_PA11_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PA11_FUNC_CTL_SPI0_CS_0: u8 = 5;
    pub const IOC_PA11_FUNC_CTL_TRGM_P_11: u8 = 17;
    pub const IOC_PA11_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PA11_FUNC_CTL_XPI0_CA_CS1: u8 = 14;
    pub const IOC_PA12_FUNC_CTL_ETH0_TXER: u8 = 18;
    pub const IOC_PA12_FUNC_CTL_FEMC_CS_0: u8 = 12;
    pub const IOC_PA12_FUNC_CTL_GPIO_A_12: u8 = 0;
    pub const IOC_PA12_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PA12_FUNC_CTL_I2S1_RXD_0: u8 = 8;
    pub const IOC_PA12_FUNC_CTL_PPI0_CS_0: u8 = 13;
    pub const IOC_PA12_FUNC_CTL_PWM1_P_4: u8 = 16;
    pub const IOC_PA12_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PA12_FUNC_CTL_SDM0_CLK_2: u8 = 23;
    pub const IOC_PA12_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PA12_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PA12_FUNC_CTL_TRGM_P_12: u8 = 17;
    pub const IOC_PA12_FUNC_CTL_UART3_CTS: u8 = 3;
    pub const IOC_PA13_FUNC_CTL_FEMC_BA0: u8 = 12;
    pub const IOC_PA13_FUNC_CTL_GPIO_A_13: u8 = 0;
    pub const IOC_PA13_FUNC_CTL_GPTMR1_COMP_3: u8 = 1;
    pub const IOC_PA13_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PA13_FUNC_CTL_I2S1_RXD_1: u8 = 8;
    pub const IOC_PA13_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PA13_FUNC_CTL_PPI0_CTR_3: u8 = 13;
    pub const IOC_PA13_FUNC_CTL_PWM1_P_5: u8 = 16;
    pub const IOC_PA13_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PA13_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PA13_FUNC_CTL_SDM0_DAT_2: u8 = 23;
    pub const IOC_PA13_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PA13_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PA13_FUNC_CTL_TRGM_P_13: u8 = 17;
    pub const IOC_PA13_FUNC_CTL_UART3_DE: u8 = 2;
    pub const IOC_PA13_FUNC_CTL_UART3_RTS: u8 = 3;
    pub const IOC_PA14_FUNC_CTL_ETH0_EVTI_0: u8 = 25;
    pub const IOC_PA14_FUNC_CTL_FEMC_BA1: u8 = 12;
    pub const IOC_PA14_FUNC_CTL_GPIO_A_14: u8 = 0;
    pub const IOC_PA14_FUNC_CTL_I2S1_RXD_2: u8 = 8;
    pub const IOC_PA14_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PA14_FUNC_CTL_PPI0_CS_2: u8 = 13;
    pub const IOC_PA14_FUNC_CTL_PWM1_P_6: u8 = 16;
    pub const IOC_PA14_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PA14_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PA14_FUNC_CTL_SDM0_CLK_3: u8 = 23;
    pub const IOC_PA14_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PA14_FUNC_CTL_SPI0_DAT2: u8 = 5;
    pub const IOC_PA14_FUNC_CTL_TRGM_P_14: u8 = 17;
    pub const IOC_PA14_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PA15_FUNC_CTL_ETH0_EVTO_0: u8 = 25;
    pub const IOC_PA15_FUNC_CTL_FEMC_A_10: u8 = 12;
    pub const IOC_PA15_FUNC_CTL_GPIO_A_15: u8 = 0;
    pub const IOC_PA15_FUNC_CTL_GPTMR0_COMP_3: u8 = 1;
    pub const IOC_PA15_FUNC_CTL_I2S1_RXD_3: u8 = 8;
    pub const IOC_PA15_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PA15_FUNC_CTL_PPI0_DQ_26: u8 = 13;
    pub const IOC_PA15_FUNC_CTL_PWM1_P_7: u8 = 16;
    pub const IOC_PA15_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PA15_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PA15_FUNC_CTL_SDM0_DAT_3: u8 = 23;
    pub const IOC_PA15_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PA15_FUNC_CTL_SPI0_DAT3: u8 = 5;
    pub const IOC_PA15_FUNC_CTL_TRGM_P_15: u8 = 17;
    pub const IOC_PA15_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PA16_FUNC_CTL_GPIO_A_16: u8 = 0;
    pub const IOC_PA16_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PA16_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PA16_FUNC_CTL_PWM2_P_0: u8 = 16;
    pub const IOC_PA16_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PA16_FUNC_CTL_RDC0_PWM_N: u8 = 20;
    pub const IOC_PA16_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PA16_FUNC_CTL_TRGM_P_16: u8 = 17;
    pub const IOC_PA16_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PA17_FUNC_CTL_GPIO_A_17: u8 = 0;
    pub const IOC_PA17_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PA17_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PA17_FUNC_CTL_PWM2_P_1: u8 = 16;
    pub const IOC_PA17_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PA17_FUNC_CTL_RDC0_PWM_P: u8 = 20;
    pub const IOC_PA17_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PA17_FUNC_CTL_TRGM_P_17: u8 = 17;
    pub const IOC_PA17_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PA18_FUNC_CTL_ETH0_MDC: u8 = 18;
    pub const IOC_PA18_FUNC_CTL_GPIO_A_18: u8 = 0;
    pub const IOC_PA18_FUNC_CTL_GPTMR3_COMP_1: u8 = 1;
    pub const IOC_PA18_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PA18_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PA18_FUNC_CTL_PWM2_P_2: u8 = 16;
    pub const IOC_PA18_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PA18_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PA18_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PA18_FUNC_CTL_TRGM_P_18: u8 = 17;
    pub const IOC_PA18_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PA18_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PA19_FUNC_CTL_ETH0_MDIO: u8 = 18;
    pub const IOC_PA19_FUNC_CTL_GPIO_A_19: u8 = 0;
    pub const IOC_PA19_FUNC_CTL_GPTMR3_CAPT_1: u8 = 1;
    pub const IOC_PA19_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PA19_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PA19_FUNC_CTL_PWM2_P_3: u8 = 16;
    pub const IOC_PA19_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PA19_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PA19_FUNC_CTL_SPI3_CS_3: u8 = 5;
    pub const IOC_PA19_FUNC_CTL_TRGM_P_19: u8 = 17;
    pub const IOC_PA19_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PA20_FUNC_CTL_ETH0_RXCK: u8 = 18;
    pub const IOC_PA20_FUNC_CTL_GPIO_A_20: u8 = 0;
    pub const IOC_PA20_FUNC_CTL_I2S0_MCLK: u8 = 8;
    pub const IOC_PA20_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PA20_FUNC_CTL_PWM2_P_4: u8 = 16;
    pub const IOC_PA20_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PA20_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PA20_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PA20_FUNC_CTL_TRGM_P_20: u8 = 17;
    pub const IOC_PA20_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PA21_FUNC_CTL_ETH0_RXDV: u8 = 18;
    pub const IOC_PA21_FUNC_CTL_GPIO_A_21: u8 = 0;
    pub const IOC_PA21_FUNC_CTL_GPTMR3_COMP_2: u8 = 1;
    pub const IOC_PA21_FUNC_CTL_I2S0_BCLK: u8 = 8;
    pub const IOC_PA21_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PA21_FUNC_CTL_PWM2_P_5: u8 = 16;
    pub const IOC_PA21_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PA21_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PA21_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PA21_FUNC_CTL_SPI2_CS_0: u8 = 5;
    pub const IOC_PA21_FUNC_CTL_TRGM_P_21: u8 = 17;
    pub const IOC_PA21_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PA21_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PA22_FUNC_CTL_ETH0_RXD_0: u8 = 18;
    pub const IOC_PA22_FUNC_CTL_GPIO_A_22: u8 = 0;
    pub const IOC_PA22_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PA22_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PA22_FUNC_CTL_I2S0_FCLK: u8 = 8;
    pub const IOC_PA22_FUNC_CTL_PWM2_P_6: u8 = 16;
    pub const IOC_PA22_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PA22_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PA22_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PA22_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PA22_FUNC_CTL_TRGM_P_22: u8 = 17;
    pub const IOC_PA22_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PA23_FUNC_CTL_ETH0_RXD_1: u8 = 18;
    pub const IOC_PA23_FUNC_CTL_GPIO_A_23: u8 = 0;
    pub const IOC_PA23_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PA23_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PA23_FUNC_CTL_I2S0_RXD_0: u8 = 8;
    pub const IOC_PA23_FUNC_CTL_PWM2_P_7: u8 = 16;
    pub const IOC_PA23_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PA23_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PA23_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PA23_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PA23_FUNC_CTL_TRGM_P_23: u8 = 17;
    pub const IOC_PA23_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PA23_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PA24_FUNC_CTL_ETH0_RXD_2: u8 = 18;
    pub const IOC_PA24_FUNC_CTL_GPIO_A_24: u8 = 0;
    pub const IOC_PA24_FUNC_CTL_GPTMR2_COMP_1: u8 = 1;
    pub const IOC_PA24_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PA24_FUNC_CTL_I2S0_RXD_1: u8 = 8;
    pub const IOC_PA24_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PA24_FUNC_CTL_PWM3_P_0: u8 = 16;
    pub const IOC_PA24_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PA24_FUNC_CTL_RDC0_PWM_N: u8 = 20;
    pub const IOC_PA24_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PA24_FUNC_CTL_SPI3_CS_2: u8 = 5;
    pub const IOC_PA24_FUNC_CTL_TRGM_P_24: u8 = 17;
    pub const IOC_PA24_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PA24_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PA25_FUNC_CTL_ETH0_RXD_3: u8 = 18;
    pub const IOC_PA25_FUNC_CTL_GPIO_A_25: u8 = 0;
    pub const IOC_PA25_FUNC_CTL_GPTMR2_CAPT_1: u8 = 1;
    pub const IOC_PA25_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PA25_FUNC_CTL_I2S0_RXD_2: u8 = 8;
    pub const IOC_PA25_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PA25_FUNC_CTL_PWM3_P_1: u8 = 16;
    pub const IOC_PA25_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PA25_FUNC_CTL_RDC0_PWM_P: u8 = 20;
    pub const IOC_PA25_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PA25_FUNC_CTL_SPI3_CS_1: u8 = 5;
    pub const IOC_PA25_FUNC_CTL_TRGM_P_25: u8 = 17;
    pub const IOC_PA25_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PA25_FUNC_CTL_XPI0_CA_CS1: u8 = 14;
    pub const IOC_PA26_FUNC_CTL_ETH0_TXCK: u8 = 18;
    pub const IOC_PA26_FUNC_CTL_GPIO_A_26: u8 = 0;
    pub const IOC_PA26_FUNC_CTL_GPTMR2_COMP_2: u8 = 1;
    pub const IOC_PA26_FUNC_CTL_I2S0_RXD_3: u8 = 8;
    pub const IOC_PA26_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PA26_FUNC_CTL_PWM3_P_2: u8 = 16;
    pub const IOC_PA26_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PA26_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PA26_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PA26_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PA26_FUNC_CTL_TRGM_P_26: u8 = 17;
    pub const IOC_PA26_FUNC_CTL_UART6_DE: u8 = 2;
    pub const IOC_PA26_FUNC_CTL_UART6_RTS: u8 = 3;
    pub const IOC_PA26_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PA27_FUNC_CTL_ETH0_TXD_0: u8 = 18;
    pub const IOC_PA27_FUNC_CTL_GPIO_A_27: u8 = 0;
    pub const IOC_PA27_FUNC_CTL_I2S0_MCLK: u8 = 8;
    pub const IOC_PA27_FUNC_CTL_PWM3_P_3: u8 = 16;
    pub const IOC_PA27_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PA27_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PA27_FUNC_CTL_SPI3_CS_0: u8 = 5;
    pub const IOC_PA27_FUNC_CTL_TRGM_P_27: u8 = 17;
    pub const IOC_PA27_FUNC_CTL_UART6_CTS: u8 = 3;
    pub const IOC_PA27_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PA28_FUNC_CTL_ETH0_TXD_1: u8 = 18;
    pub const IOC_PA28_FUNC_CTL_GPIO_A_28: u8 = 0;
    pub const IOC_PA28_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PA28_FUNC_CTL_I2S0_TXD_0: u8 = 8;
    pub const IOC_PA28_FUNC_CTL_PWM3_P_4: u8 = 16;
    pub const IOC_PA28_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PA28_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PA28_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PA28_FUNC_CTL_TRGM_P_28: u8 = 17;
    pub const IOC_PA28_FUNC_CTL_UART7_CTS: u8 = 3;
    pub const IOC_PA28_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PA29_FUNC_CTL_ETH0_TXD_2: u8 = 18;
    pub const IOC_PA29_FUNC_CTL_GPIO_A_29: u8 = 0;
    pub const IOC_PA29_FUNC_CTL_GPTMR3_COMP_3: u8 = 1;
    pub const IOC_PA29_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PA29_FUNC_CTL_I2S0_TXD_1: u8 = 8;
    pub const IOC_PA29_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PA29_FUNC_CTL_PWM3_P_5: u8 = 16;
    pub const IOC_PA29_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PA29_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PA29_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PA29_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PA29_FUNC_CTL_TRGM_P_29: u8 = 17;
    pub const IOC_PA29_FUNC_CTL_UART7_DE: u8 = 2;
    pub const IOC_PA29_FUNC_CTL_UART7_RTS: u8 = 3;
    pub const IOC_PA29_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
    pub const IOC_PA30_FUNC_CTL_ETH0_TXD_3: u8 = 18;
    pub const IOC_PA30_FUNC_CTL_GPIO_A_30: u8 = 0;
    pub const IOC_PA30_FUNC_CTL_I2S0_TXD_2: u8 = 8;
    pub const IOC_PA30_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PA30_FUNC_CTL_PWM3_P_6: u8 = 16;
    pub const IOC_PA30_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PA30_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PA30_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PA30_FUNC_CTL_SPI3_DAT2: u8 = 5;
    pub const IOC_PA30_FUNC_CTL_TRGM_P_30: u8 = 17;
    pub const IOC_PA30_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PA30_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PA31_FUNC_CTL_GPIO_A_31: u8 = 0;
    pub const IOC_PA31_FUNC_CTL_GPTMR2_COMP_3: u8 = 1;
    pub const IOC_PA31_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PA31_FUNC_CTL_PWM3_P_7: u8 = 16;
    pub const IOC_PA31_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PA31_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PA31_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PA31_FUNC_CTL_SPI3_DAT3: u8 = 5;
    pub const IOC_PA31_FUNC_CTL_TRGM_P_31: u8 = 17;
    pub const IOC_PA31_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PB00_FUNC_CTL_ETH0_TXCK: u8 = 18;
    pub const IOC_PB00_FUNC_CTL_FEMC_A_00: u8 = 12;
    pub const IOC_PB00_FUNC_CTL_GPIO_B_00: u8 = 0;
    pub const IOC_PB00_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PB00_FUNC_CTL_I2S0_TXD_3: u8 = 8;
    pub const IOC_PB00_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PB00_FUNC_CTL_PDM0_CLK: u8 = 9;
    pub const IOC_PB00_FUNC_CTL_PPI0_DQ_16: u8 = 13;
    pub const IOC_PB00_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PB00_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PB00_FUNC_CTL_RDC0_PWM_N: u8 = 20;
    pub const IOC_PB00_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PB00_FUNC_CTL_TRGM_P_31: u8 = 17;
    pub const IOC_PB00_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PB00_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PB00_FUNC_CTL_XPI_SLV_ADQ_3: u8 = 30;
    pub const IOC_PB01_FUNC_CTL_ETH0_TXEN: u8 = 18;
    pub const IOC_PB01_FUNC_CTL_FEMC_A_01: u8 = 12;
    pub const IOC_PB01_FUNC_CTL_GPIO_B_01: u8 = 0;
    pub const IOC_PB01_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PB01_FUNC_CTL_I2S1_MCLK: u8 = 8;
    pub const IOC_PB01_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PB01_FUNC_CTL_PDM0_D_0: u8 = 9;
    pub const IOC_PB01_FUNC_CTL_PPI0_DQ_17: u8 = 13;
    pub const IOC_PB01_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PB01_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PB01_FUNC_CTL_RDC0_PWM_P: u8 = 20;
    pub const IOC_PB01_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PB01_FUNC_CTL_TRGM_P_30: u8 = 17;
    pub const IOC_PB01_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PB01_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PB01_FUNC_CTL_XPI_SLV_ADQ_2: u8 = 30;
    pub const IOC_PB02_FUNC_CTL_ETH0_TXD_0: u8 = 18;
    pub const IOC_PB02_FUNC_CTL_FEMC_A_02: u8 = 12;
    pub const IOC_PB02_FUNC_CTL_GPIO_B_02: u8 = 0;
    pub const IOC_PB02_FUNC_CTL_GPTMR2_COMP_1: u8 = 1;
    pub const IOC_PB02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PB02_FUNC_CTL_I2S1_TXD_0: u8 = 8;
    pub const IOC_PB02_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PB02_FUNC_CTL_PDM0_D_1: u8 = 9;
    pub const IOC_PB02_FUNC_CTL_PPI0_DQ_18: u8 = 13;
    pub const IOC_PB02_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PB02_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PB02_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PB02_FUNC_CTL_SDM0_CLK_0: u8 = 23;
    pub const IOC_PB02_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PB02_FUNC_CTL_TRGM_P_29: u8 = 17;
    pub const IOC_PB02_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PB02_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PB02_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PB02_FUNC_CTL_XPI_SLV_ADQ_1: u8 = 30;
    pub const IOC_PB03_FUNC_CTL_ETH0_TXD_1: u8 = 18;
    pub const IOC_PB03_FUNC_CTL_FEMC_A_03: u8 = 12;
    pub const IOC_PB03_FUNC_CTL_GPIO_B_03: u8 = 0;
    pub const IOC_PB03_FUNC_CTL_GPTMR2_CAPT_1: u8 = 1;
    pub const IOC_PB03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PB03_FUNC_CTL_I2S1_TXD_1: u8 = 8;
    pub const IOC_PB03_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PB03_FUNC_CTL_PDM0_D_2: u8 = 9;
    pub const IOC_PB03_FUNC_CTL_PPI0_DQ_19: u8 = 13;
    pub const IOC_PB03_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PB03_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PB03_FUNC_CTL_SDM0_DAT_0: u8 = 23;
    pub const IOC_PB03_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PB03_FUNC_CTL_SPI0_CS_3: u8 = 5;
    pub const IOC_PB03_FUNC_CTL_TRGM_P_28: u8 = 17;
    pub const IOC_PB03_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PB03_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
    pub const IOC_PB03_FUNC_CTL_XPI_SLV_ADQ_0: u8 = 30;
    pub const IOC_PB04_FUNC_CTL_CPU0_NMI: u8 = 24;
    pub const IOC_PB04_FUNC_CTL_ETH0_TXD_2: u8 = 18;
    pub const IOC_PB04_FUNC_CTL_FEMC_A_04: u8 = 12;
    pub const IOC_PB04_FUNC_CTL_GPIO_B_04: u8 = 0;
    pub const IOC_PB04_FUNC_CTL_I2S1_TXD_2: u8 = 8;
    pub const IOC_PB04_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PB04_FUNC_CTL_PDM0_D_3: u8 = 9;
    pub const IOC_PB04_FUNC_CTL_PPI0_DQ_20: u8 = 13;
    pub const IOC_PB04_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PB04_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PB04_FUNC_CTL_SDM0_CLK_1: u8 = 23;
    pub const IOC_PB04_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PB04_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PB04_FUNC_CTL_TRGM_P_27: u8 = 17;
    pub const IOC_PB04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PB04_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PB04_FUNC_CTL_XPI_SLV_CLK: u8 = 30;
    pub const IOC_PB05_FUNC_CTL_CPU1_NMI: u8 = 24;
    pub const IOC_PB05_FUNC_CTL_ETH0_TXD_3: u8 = 18;
    pub const IOC_PB05_FUNC_CTL_FEMC_A_05: u8 = 12;
    pub const IOC_PB05_FUNC_CTL_GPIO_B_05: u8 = 0;
    pub const IOC_PB05_FUNC_CTL_GPTMR2_COMP_2: u8 = 1;
    pub const IOC_PB05_FUNC_CTL_I2S1_TXD_3: u8 = 8;
    pub const IOC_PB05_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PB05_FUNC_CTL_PDM0_CLK: u8 = 9;
    pub const IOC_PB05_FUNC_CTL_PPI0_DQ_21: u8 = 13;
    pub const IOC_PB05_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PB05_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PB05_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PB05_FUNC_CTL_SDM0_DAT_1: u8 = 23;
    pub const IOC_PB05_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PB05_FUNC_CTL_SPI1_CS_0: u8 = 5;
    pub const IOC_PB05_FUNC_CTL_TRGM_P_26: u8 = 17;
    pub const IOC_PB05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PB05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PB05_FUNC_CTL_XPI_SLV_CSN: u8 = 30;
    pub const IOC_PB06_FUNC_CTL_ETH0_TXEN: u8 = 18;
    pub const IOC_PB06_FUNC_CTL_GPIO_B_06: u8 = 0;
    pub const IOC_PB06_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PB06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PB06_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PB06_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PB06_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PB06_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PB06_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PB06_FUNC_CTL_TRGM_P_25: u8 = 17;
    pub const IOC_PB06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PB07_FUNC_CTL_GPIO_B_07: u8 = 0;
    pub const IOC_PB07_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PB07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PB07_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PB07_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PB07_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PB07_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PB07_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PB07_FUNC_CTL_TRGM_P_24: u8 = 17;
    pub const IOC_PB07_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PB08_FUNC_CTL_GPIO_B_08: u8 = 0;
    pub const IOC_PB08_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PB08_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PB08_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PB08_FUNC_CTL_PWM1_P_0: u8 = 16;
    pub const IOC_PB08_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PB08_FUNC_CTL_RDC0_PWM_N: u8 = 20;
    pub const IOC_PB08_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PB08_FUNC_CTL_SPI0_CS_2: u8 = 5;
    pub const IOC_PB08_FUNC_CTL_TRGM_P_23: u8 = 17;
    pub const IOC_PB08_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PB09_FUNC_CTL_GPIO_B_09: u8 = 0;
    pub const IOC_PB09_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PB09_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PB09_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PB09_FUNC_CTL_PWM1_P_1: u8 = 16;
    pub const IOC_PB09_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PB09_FUNC_CTL_RDC0_PWM_P: u8 = 20;
    pub const IOC_PB09_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PB09_FUNC_CTL_SPI0_CS_1: u8 = 5;
    pub const IOC_PB09_FUNC_CTL_TRGM_P_22: u8 = 17;
    pub const IOC_PB09_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PB10_FUNC_CTL_GPIO_B_10: u8 = 0;
    pub const IOC_PB10_FUNC_CTL_GPTMR0_COMP_2: u8 = 1;
    pub const IOC_PB10_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PB10_FUNC_CTL_PWM1_P_2: u8 = 16;
    pub const IOC_PB10_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PB10_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PB10_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PB10_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PB10_FUNC_CTL_TRGM_P_21: u8 = 17;
    pub const IOC_PB10_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PB10_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PB11_FUNC_CTL_GPIO_B_11: u8 = 0;
    pub const IOC_PB11_FUNC_CTL_PWM1_P_3: u8 = 16;
    pub const IOC_PB11_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PB11_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PB11_FUNC_CTL_SPI0_CS_0: u8 = 5;
    pub const IOC_PB11_FUNC_CTL_TRGM_P_20: u8 = 17;
    pub const IOC_PB11_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PB12_FUNC_CTL_DAO_LN: u8 = 9;
    pub const IOC_PB12_FUNC_CTL_ETH0_MDC: u8 = 18;
    pub const IOC_PB12_FUNC_CTL_EWDG0_RST: u8 = 24;
    pub const IOC_PB12_FUNC_CTL_FEMC_A_06: u8 = 12;
    pub const IOC_PB12_FUNC_CTL_GPIO_B_12: u8 = 0;
    pub const IOC_PB12_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PB12_FUNC_CTL_PPI0_DQ_22: u8 = 13;
    pub const IOC_PB12_FUNC_CTL_PWM1_P_4: u8 = 16;
    pub const IOC_PB12_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PB12_FUNC_CTL_SDM0_CLK_2: u8 = 23;
    pub const IOC_PB12_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PB12_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PB12_FUNC_CTL_TRGM_P_19: u8 = 17;
    pub const IOC_PB12_FUNC_CTL_UART3_CTS: u8 = 3;
    pub const IOC_PB12_FUNC_CTL_XPI0_CB_DQS: u8 = 14;
    pub const IOC_PB12_FUNC_CTL_XPI_SLV_ERR: u8 = 30;
    pub const IOC_PB13_FUNC_CTL_DAO_LP: u8 = 9;
    pub const IOC_PB13_FUNC_CTL_ETH0_MDIO: u8 = 18;
    pub const IOC_PB13_FUNC_CTL_EWDG1_RST: u8 = 24;
    pub const IOC_PB13_FUNC_CTL_FEMC_A_07: u8 = 12;
    pub const IOC_PB13_FUNC_CTL_GPIO_B_13: u8 = 0;
    pub const IOC_PB13_FUNC_CTL_GPTMR2_COMP_3: u8 = 1;
    pub const IOC_PB13_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PB13_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PB13_FUNC_CTL_PPI0_DQ_23: u8 = 13;
    pub const IOC_PB13_FUNC_CTL_PWM1_P_5: u8 = 16;
    pub const IOC_PB13_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PB13_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PB13_FUNC_CTL_SDM0_DAT_2: u8 = 23;
    pub const IOC_PB13_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PB13_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PB13_FUNC_CTL_TRGM_P_18: u8 = 17;
    pub const IOC_PB13_FUNC_CTL_UART3_DE: u8 = 2;
    pub const IOC_PB13_FUNC_CTL_UART3_RTS: u8 = 3;
    pub const IOC_PB13_FUNC_CTL_XPI0_CB_CS0: u8 = 14;
    pub const IOC_PB13_FUNC_CTL_XPI_SLV_RDY: u8 = 30;
    pub const IOC_PB14_FUNC_CTL_DAO_RN: u8 = 9;
    pub const IOC_PB14_FUNC_CTL_ETH0_EVTO_2: u8 = 25;
    pub const IOC_PB14_FUNC_CTL_FEMC_A_08: u8 = 12;
    pub const IOC_PB14_FUNC_CTL_GPIO_B_14: u8 = 0;
    pub const IOC_PB14_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PB14_FUNC_CTL_PPI0_DQ_24: u8 = 13;
    pub const IOC_PB14_FUNC_CTL_PWM1_P_6: u8 = 16;
    pub const IOC_PB14_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PB14_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PB14_FUNC_CTL_SDM0_CLK_3: u8 = 23;
    pub const IOC_PB14_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PB14_FUNC_CTL_SOC_REF0: u8 = 24;
    pub const IOC_PB14_FUNC_CTL_SPI0_DAT2: u8 = 5;
    pub const IOC_PB14_FUNC_CTL_TRGM_P_17: u8 = 17;
    pub const IOC_PB14_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PB14_FUNC_CTL_XPI0_CB_CS1: u8 = 14;
    pub const IOC_PB14_FUNC_CTL_XPI_SLV_DQS: u8 = 30;
    pub const IOC_PB15_FUNC_CTL_DAO_RP: u8 = 9;
    pub const IOC_PB15_FUNC_CTL_ETH0_EVTO_3: u8 = 25;
    pub const IOC_PB15_FUNC_CTL_FEMC_A_09: u8 = 12;
    pub const IOC_PB15_FUNC_CTL_GPIO_B_15: u8 = 0;
    pub const IOC_PB15_FUNC_CTL_GPTMR0_COMP_3: u8 = 1;
    pub const IOC_PB15_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PB15_FUNC_CTL_PPI0_DQ_25: u8 = 13;
    pub const IOC_PB15_FUNC_CTL_PWM1_P_7: u8 = 16;
    pub const IOC_PB15_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PB15_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PB15_FUNC_CTL_SDM0_DAT_3: u8 = 23;
    pub const IOC_PB15_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PB15_FUNC_CTL_SOC_REF1: u8 = 24;
    pub const IOC_PB15_FUNC_CTL_SPI0_DAT3: u8 = 5;
    pub const IOC_PB15_FUNC_CTL_TRGM_P_16: u8 = 17;
    pub const IOC_PB15_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PB15_FUNC_CTL_XPI0_CB_SCLK: u8 = 14;
    pub const IOC_PB16_FUNC_CTL_ETH0_EVTI_1: u8 = 25;
    pub const IOC_PB16_FUNC_CTL_FEMC_A_11: u8 = 12;
    pub const IOC_PB16_FUNC_CTL_GPIO_B_16: u8 = 0;
    pub const IOC_PB16_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PB16_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PB16_FUNC_CTL_PPI0_DQ_27: u8 = 13;
    pub const IOC_PB16_FUNC_CTL_PWM2_P_0: u8 = 16;
    pub const IOC_PB16_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PB16_FUNC_CTL_RDC0_PWM_N: u8 = 20;
    pub const IOC_PB16_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PB16_FUNC_CTL_SYSCTL_CLK_OBS_0: u8 = 24;
    pub const IOC_PB16_FUNC_CTL_TRGM_P_15: u8 = 17;
    pub const IOC_PB16_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PB16_FUNC_CTL_XPI0_CB_D_0: u8 = 14;
    pub const IOC_PB17_FUNC_CTL_ETH0_EVTO_1: u8 = 25;
    pub const IOC_PB17_FUNC_CTL_FEMC_A_12: u8 = 12;
    pub const IOC_PB17_FUNC_CTL_GPIO_B_17: u8 = 0;
    pub const IOC_PB17_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PB17_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PB17_FUNC_CTL_PPI0_DQ_28: u8 = 13;
    pub const IOC_PB17_FUNC_CTL_PWM2_P_1: u8 = 16;
    pub const IOC_PB17_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PB17_FUNC_CTL_RDC0_PWM_P: u8 = 20;
    pub const IOC_PB17_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PB17_FUNC_CTL_SYSCTL_CLK_OBS_1: u8 = 24;
    pub const IOC_PB17_FUNC_CTL_TRGM_P_14: u8 = 17;
    pub const IOC_PB17_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PB17_FUNC_CTL_XPI0_CB_D_1: u8 = 14;
    pub const IOC_PB18_FUNC_CTL_ETH0_EVTO_2: u8 = 25;
    pub const IOC_PB18_FUNC_CTL_FEMC_CKE: u8 = 12;
    pub const IOC_PB18_FUNC_CTL_GPIO_B_18: u8 = 0;
    pub const IOC_PB18_FUNC_CTL_GPTMR3_COMP_1: u8 = 1;
    pub const IOC_PB18_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PB18_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PB18_FUNC_CTL_PPI0_DQ_31: u8 = 13;
    pub const IOC_PB18_FUNC_CTL_PWM2_P_2: u8 = 16;
    pub const IOC_PB18_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PB18_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PB18_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PB18_FUNC_CTL_SYSCTL_CLK_OBS_2: u8 = 24;
    pub const IOC_PB18_FUNC_CTL_TRGM_P_13: u8 = 17;
    pub const IOC_PB18_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PB18_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PB18_FUNC_CTL_XPI0_CB_D_2: u8 = 14;
    pub const IOC_PB19_FUNC_CTL_ETH0_EVTO_3: u8 = 25;
    pub const IOC_PB19_FUNC_CTL_FEMC_CLK_0: u8 = 12;
    pub const IOC_PB19_FUNC_CTL_GPIO_B_19: u8 = 0;
    pub const IOC_PB19_FUNC_CTL_GPTMR3_CAPT_1: u8 = 1;
    pub const IOC_PB19_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PB19_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PB19_FUNC_CTL_PPI0_CLK: u8 = 13;
    pub const IOC_PB19_FUNC_CTL_PWM2_P_3: u8 = 16;
    pub const IOC_PB19_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PB19_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PB19_FUNC_CTL_SPI2_CS_3: u8 = 5;
    pub const IOC_PB19_FUNC_CTL_SYSCTL_CLK_OBS_3: u8 = 24;
    pub const IOC_PB19_FUNC_CTL_TRGM_P_12: u8 = 17;
    pub const IOC_PB19_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PB19_FUNC_CTL_XPI0_CB_D_3: u8 = 14;
    pub const IOC_PB20_FUNC_CTL_FEMC_DQS: u8 = 12;
    pub const IOC_PB20_FUNC_CTL_GPIO_B_20: u8 = 0;
    pub const IOC_PB20_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PB20_FUNC_CTL_PPI0_CTR_1: u8 = 13;
    pub const IOC_PB20_FUNC_CTL_PWM2_P_4: u8 = 16;
    pub const IOC_PB20_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PB20_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PB20_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PB20_FUNC_CTL_TRGM_P_11: u8 = 17;
    pub const IOC_PB20_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PB21_FUNC_CTL_FEMC_SCLK_1: u8 = 12;
    pub const IOC_PB21_FUNC_CTL_GPIO_B_21: u8 = 0;
    pub const IOC_PB21_FUNC_CTL_GPTMR3_COMP_2: u8 = 1;
    pub const IOC_PB21_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PB21_FUNC_CTL_PPI0_CTR_4: u8 = 13;
    pub const IOC_PB21_FUNC_CTL_PWM2_P_5: u8 = 16;
    pub const IOC_PB21_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PB21_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PB21_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PB21_FUNC_CTL_SPI3_CS_0: u8 = 5;
    pub const IOC_PB21_FUNC_CTL_TRGM_P_10: u8 = 17;
    pub const IOC_PB21_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PB21_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PB22_FUNC_CTL_FEMC_SCS_1: u8 = 12;
    pub const IOC_PB22_FUNC_CTL_GPIO_B_22: u8 = 0;
    pub const IOC_PB22_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PB22_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PB22_FUNC_CTL_PPI0_CS_3: u8 = 13;
    pub const IOC_PB22_FUNC_CTL_PWM2_P_6: u8 = 16;
    pub const IOC_PB22_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PB22_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PB22_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PB22_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PB22_FUNC_CTL_TRGM_P_09: u8 = 17;
    pub const IOC_PB22_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PB23_FUNC_CTL_GPIO_B_23: u8 = 0;
    pub const IOC_PB23_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PB23_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PB23_FUNC_CTL_PWM2_P_7: u8 = 16;
    pub const IOC_PB23_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PB23_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PB23_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PB23_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PB23_FUNC_CTL_TRGM_P_08: u8 = 17;
    pub const IOC_PB23_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PB24_FUNC_CTL_GPIO_B_24: u8 = 0;
    pub const IOC_PB24_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PB24_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PB24_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PB24_FUNC_CTL_PWM3_P_0: u8 = 16;
    pub const IOC_PB24_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PB24_FUNC_CTL_RDC0_PWM_N: u8 = 20;
    pub const IOC_PB24_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PB24_FUNC_CTL_SPI2_CS_2: u8 = 5;
    pub const IOC_PB24_FUNC_CTL_TRGM_P_07: u8 = 17;
    pub const IOC_PB24_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PB25_FUNC_CTL_GPIO_B_25: u8 = 0;
    pub const IOC_PB25_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PB25_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PB25_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PB25_FUNC_CTL_PPI0_CTR_7: u8 = 13;
    pub const IOC_PB25_FUNC_CTL_PWM3_P_1: u8 = 16;
    pub const IOC_PB25_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PB25_FUNC_CTL_RDC0_PWM_P: u8 = 20;
    pub const IOC_PB25_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PB25_FUNC_CTL_SPI2_CS_1: u8 = 5;
    pub const IOC_PB25_FUNC_CTL_TRGM_P_06: u8 = 17;
    pub const IOC_PB25_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PB26_FUNC_CTL_GPIO_B_26: u8 = 0;
    pub const IOC_PB26_FUNC_CTL_GPTMR1_COMP_2: u8 = 1;
    pub const IOC_PB26_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PB26_FUNC_CTL_PPI0_CTR_6: u8 = 13;
    pub const IOC_PB26_FUNC_CTL_PWM3_P_2: u8 = 16;
    pub const IOC_PB26_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PB26_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PB26_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PB26_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PB26_FUNC_CTL_TRGM_P_05: u8 = 17;
    pub const IOC_PB26_FUNC_CTL_UART6_DE: u8 = 2;
    pub const IOC_PB26_FUNC_CTL_UART6_RTS: u8 = 3;
    pub const IOC_PB27_FUNC_CTL_GPIO_B_27: u8 = 0;
    pub const IOC_PB27_FUNC_CTL_PPI0_CTR_5: u8 = 13;
    pub const IOC_PB27_FUNC_CTL_PWM3_P_3: u8 = 16;
    pub const IOC_PB27_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PB27_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PB27_FUNC_CTL_SPI2_CS_0: u8 = 5;
    pub const IOC_PB27_FUNC_CTL_TRGM_P_04: u8 = 17;
    pub const IOC_PB27_FUNC_CTL_UART6_CTS: u8 = 3;
    pub const IOC_PB28_FUNC_CTL_FEMC_DQS: u8 = 12;
    pub const IOC_PB28_FUNC_CTL_GPIO_B_28: u8 = 0;
    pub const IOC_PB28_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PB28_FUNC_CTL_PWM3_P_4: u8 = 16;
    pub const IOC_PB28_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PB28_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PB28_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PB28_FUNC_CTL_TRGM_P_03: u8 = 17;
    pub const IOC_PB28_FUNC_CTL_UART7_CTS: u8 = 3;
    pub const IOC_PB29_FUNC_CTL_GPIO_B_29: u8 = 0;
    pub const IOC_PB29_FUNC_CTL_GPTMR3_COMP_3: u8 = 1;
    pub const IOC_PB29_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PB29_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PB29_FUNC_CTL_PWM3_P_5: u8 = 16;
    pub const IOC_PB29_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PB29_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PB29_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PB29_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PB29_FUNC_CTL_TRGM_P_02: u8 = 17;
    pub const IOC_PB29_FUNC_CTL_UART7_DE: u8 = 2;
    pub const IOC_PB29_FUNC_CTL_UART7_RTS: u8 = 3;
    pub const IOC_PB30_FUNC_CTL_GPIO_B_30: u8 = 0;
    pub const IOC_PB30_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PB30_FUNC_CTL_PWM3_P_6: u8 = 16;
    pub const IOC_PB30_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PB30_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PB30_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PB30_FUNC_CTL_SPI2_DAT2: u8 = 5;
    pub const IOC_PB30_FUNC_CTL_TRGM_P_01: u8 = 17;
    pub const IOC_PB30_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PB31_FUNC_CTL_GPIO_B_31: u8 = 0;
    pub const IOC_PB31_FUNC_CTL_GPTMR1_COMP_3: u8 = 1;
    pub const IOC_PB31_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PB31_FUNC_CTL_PWM3_P_7: u8 = 16;
    pub const IOC_PB31_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PB31_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PB31_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PB31_FUNC_CTL_SPI2_DAT3: u8 = 5;
    pub const IOC_PB31_FUNC_CTL_TRGM_P_00: u8 = 17;
    pub const IOC_PB31_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PC00_FUNC_CTL_GPIO_C_00: u8 = 0;
    pub const IOC_PC00_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PC00_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PC00_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PC00_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PC00_FUNC_CTL_RDC0_PWM_N: u8 = 20;
    pub const IOC_PC00_FUNC_CTL_SDM0_CLK_0: u8 = 23;
    pub const IOC_PC00_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PC00_FUNC_CTL_TRGM_P_00: u8 = 17;
    pub const IOC_PC00_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PC01_FUNC_CTL_GPIO_C_01: u8 = 0;
    pub const IOC_PC01_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PC01_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PC01_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PC01_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PC01_FUNC_CTL_RDC0_PWM_P: u8 = 20;
    pub const IOC_PC01_FUNC_CTL_SDM0_DAT_0: u8 = 23;
    pub const IOC_PC01_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PC01_FUNC_CTL_TRGM_P_01: u8 = 17;
    pub const IOC_PC01_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PC02_FUNC_CTL_GPIO_C_02: u8 = 0;
    pub const IOC_PC02_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PC02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PC02_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PC02_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PC02_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PC02_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PC02_FUNC_CTL_SDM0_CLK_1: u8 = 23;
    pub const IOC_PC02_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PC02_FUNC_CTL_TRGM_P_02: u8 = 17;
    pub const IOC_PC02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PC02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PC03_FUNC_CTL_FEMC_DM_1: u8 = 12;
    pub const IOC_PC03_FUNC_CTL_GPIO_C_03: u8 = 0;
    pub const IOC_PC03_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PC03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PC03_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PC03_FUNC_CTL_PPI0_DM_1: u8 = 13;
    pub const IOC_PC03_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PC03_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PC03_FUNC_CTL_SDM0_DAT_1: u8 = 23;
    pub const IOC_PC03_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PC03_FUNC_CTL_SPI1_CS_3: u8 = 5;
    pub const IOC_PC03_FUNC_CTL_TRGM_P_03: u8 = 17;
    pub const IOC_PC03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PC04_FUNC_CTL_FEMC_DQ_08: u8 = 12;
    pub const IOC_PC04_FUNC_CTL_GPIO_C_04: u8 = 0;
    pub const IOC_PC04_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PC04_FUNC_CTL_PPI0_DQ_08: u8 = 13;
    pub const IOC_PC04_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PC04_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PC04_FUNC_CTL_SDM0_CLK_2: u8 = 23;
    pub const IOC_PC04_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PC04_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PC04_FUNC_CTL_TRGM_P_04: u8 = 17;
    pub const IOC_PC04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PC05_FUNC_CTL_FEMC_DQ_09: u8 = 12;
    pub const IOC_PC05_FUNC_CTL_GPIO_C_05: u8 = 0;
    pub const IOC_PC05_FUNC_CTL_GPTMR1_COMP_2: u8 = 1;
    pub const IOC_PC05_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PC05_FUNC_CTL_PPI0_DQ_09: u8 = 13;
    pub const IOC_PC05_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PC05_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PC05_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PC05_FUNC_CTL_SDM0_DAT_2: u8 = 23;
    pub const IOC_PC05_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PC05_FUNC_CTL_SPI0_CS_0: u8 = 5;
    pub const IOC_PC05_FUNC_CTL_TRGM_P_05: u8 = 17;
    pub const IOC_PC05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PC05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PC06_FUNC_CTL_FEMC_DQ_10: u8 = 12;
    pub const IOC_PC06_FUNC_CTL_GPIO_C_06: u8 = 0;
    pub const IOC_PC06_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PC06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PC06_FUNC_CTL_PPI0_DQ_10: u8 = 13;
    pub const IOC_PC06_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PC06_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PC06_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PC06_FUNC_CTL_SDM0_CLK_3: u8 = 23;
    pub const IOC_PC06_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PC06_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PC06_FUNC_CTL_TRGM_P_06: u8 = 17;
    pub const IOC_PC06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PC07_FUNC_CTL_FEMC_DQ_11: u8 = 12;
    pub const IOC_PC07_FUNC_CTL_GPIO_C_07: u8 = 0;
    pub const IOC_PC07_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PC07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PC07_FUNC_CTL_PPI0_DQ_11: u8 = 13;
    pub const IOC_PC07_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PC07_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PC07_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PC07_FUNC_CTL_SDM0_DAT_3: u8 = 23;
    pub const IOC_PC07_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PC07_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PC07_FUNC_CTL_TRGM_P_07: u8 = 17;
    pub const IOC_PC07_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PC08_FUNC_CTL_FEMC_DQ_12: u8 = 12;
    pub const IOC_PC08_FUNC_CTL_GPIO_C_08: u8 = 0;
    pub const IOC_PC08_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PC08_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PC08_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PC08_FUNC_CTL_PPI0_DQ_12: u8 = 13;
    pub const IOC_PC08_FUNC_CTL_PWM1_P_0: u8 = 16;
    pub const IOC_PC08_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PC08_FUNC_CTL_RDC0_PWM_N: u8 = 20;
    pub const IOC_PC08_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PC08_FUNC_CTL_SPI1_CS_2: u8 = 5;
    pub const IOC_PC08_FUNC_CTL_TRGM_P_08: u8 = 17;
    pub const IOC_PC08_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PC09_FUNC_CTL_FEMC_DQ_13: u8 = 12;
    pub const IOC_PC09_FUNC_CTL_GPIO_C_09: u8 = 0;
    pub const IOC_PC09_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PC09_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PC09_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PC09_FUNC_CTL_PPI0_DQ_13: u8 = 13;
    pub const IOC_PC09_FUNC_CTL_PWM1_P_1: u8 = 16;
    pub const IOC_PC09_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PC09_FUNC_CTL_RDC0_PWM_P: u8 = 20;
    pub const IOC_PC09_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PC09_FUNC_CTL_SPI1_CS_1: u8 = 5;
    pub const IOC_PC09_FUNC_CTL_TRGM_P_09: u8 = 17;
    pub const IOC_PC09_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PC10_FUNC_CTL_FEMC_DQ_14: u8 = 12;
    pub const IOC_PC10_FUNC_CTL_GPIO_C_10: u8 = 0;
    pub const IOC_PC10_FUNC_CTL_GPTMR0_COMP_2: u8 = 1;
    pub const IOC_PC10_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PC10_FUNC_CTL_PPI0_DQ_14: u8 = 13;
    pub const IOC_PC10_FUNC_CTL_PWM1_P_2: u8 = 16;
    pub const IOC_PC10_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PC10_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PC10_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PC10_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PC10_FUNC_CTL_TRGM_P_10: u8 = 17;
    pub const IOC_PC10_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PC10_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PC11_FUNC_CTL_FEMC_DQ_15: u8 = 12;
    pub const IOC_PC11_FUNC_CTL_GPIO_C_11: u8 = 0;
    pub const IOC_PC11_FUNC_CTL_PPI0_DQ_15: u8 = 13;
    pub const IOC_PC11_FUNC_CTL_PWM1_P_3: u8 = 16;
    pub const IOC_PC11_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PC11_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PC11_FUNC_CTL_SPI1_CS_0: u8 = 5;
    pub const IOC_PC11_FUNC_CTL_TRGM_P_11: u8 = 17;
    pub const IOC_PC11_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PC12_FUNC_CTL_FEMC_DQ_00: u8 = 12;
    pub const IOC_PC12_FUNC_CTL_GPIO_C_12: u8 = 0;
    pub const IOC_PC12_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PC12_FUNC_CTL_PPI0_DQ_00: u8 = 13;
    pub const IOC_PC12_FUNC_CTL_PWM1_P_4: u8 = 16;
    pub const IOC_PC12_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PC12_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PC12_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PC12_FUNC_CTL_TRGM_P_12: u8 = 17;
    pub const IOC_PC12_FUNC_CTL_UART6_CTS: u8 = 3;
    pub const IOC_PC13_FUNC_CTL_FEMC_DQ_01: u8 = 12;
    pub const IOC_PC13_FUNC_CTL_GPIO_C_13: u8 = 0;
    pub const IOC_PC13_FUNC_CTL_GPTMR1_COMP_3: u8 = 1;
    pub const IOC_PC13_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PC13_FUNC_CTL_I2S0_MCLK: u8 = 8;
    pub const IOC_PC13_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PC13_FUNC_CTL_PPI0_DQ_01: u8 = 13;
    pub const IOC_PC13_FUNC_CTL_PWM1_P_5: u8 = 16;
    pub const IOC_PC13_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PC13_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PC13_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PC13_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PC13_FUNC_CTL_TRGM_P_13: u8 = 17;
    pub const IOC_PC13_FUNC_CTL_UART6_DE: u8 = 2;
    pub const IOC_PC13_FUNC_CTL_UART6_RTS: u8 = 3;
    pub const IOC_PC13_FUNC_CTL_USB0_ID: u8 = 24;
    pub const IOC_PC14_FUNC_CTL_ETH0_MDC: u8 = 18;
    pub const IOC_PC14_FUNC_CTL_FEMC_DQ_02: u8 = 12;
    pub const IOC_PC14_FUNC_CTL_GPIO_C_14: u8 = 0;
    pub const IOC_PC14_FUNC_CTL_I2S0_BCLK: u8 = 8;
    pub const IOC_PC14_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PC14_FUNC_CTL_PPI0_DQ_02: u8 = 13;
    pub const IOC_PC14_FUNC_CTL_PWM1_P_6: u8 = 16;
    pub const IOC_PC14_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PC14_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PC14_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PC14_FUNC_CTL_SPI1_DAT2: u8 = 5;
    pub const IOC_PC14_FUNC_CTL_TRGM_P_14: u8 = 17;
    pub const IOC_PC14_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PC14_FUNC_CTL_USB0_OC: u8 = 24;
    pub const IOC_PC15_FUNC_CTL_ETH0_MDIO: u8 = 18;
    pub const IOC_PC15_FUNC_CTL_FEMC_DQ_03: u8 = 12;
    pub const IOC_PC15_FUNC_CTL_GPIO_C_15: u8 = 0;
    pub const IOC_PC15_FUNC_CTL_GPTMR0_COMP_3: u8 = 1;
    pub const IOC_PC15_FUNC_CTL_I2S0_FCLK: u8 = 8;
    pub const IOC_PC15_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PC15_FUNC_CTL_PPI0_DQ_03: u8 = 13;
    pub const IOC_PC15_FUNC_CTL_PWM1_P_7: u8 = 16;
    pub const IOC_PC15_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PC15_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PC15_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PC15_FUNC_CTL_SPI1_DAT3: u8 = 5;
    pub const IOC_PC15_FUNC_CTL_TRGM_P_15: u8 = 17;
    pub const IOC_PC15_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PC15_FUNC_CTL_USB0_PWR: u8 = 24;
    pub const IOC_PC16_FUNC_CTL_CPU0_NMI: u8 = 24;
    pub const IOC_PC16_FUNC_CTL_DAO_LN: u8 = 9;
    pub const IOC_PC16_FUNC_CTL_ETH0_CRS: u8 = 18;
    pub const IOC_PC16_FUNC_CTL_FEMC_DQ_04: u8 = 12;
    pub const IOC_PC16_FUNC_CTL_GPIO_C_16: u8 = 0;
    pub const IOC_PC16_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PC16_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PC16_FUNC_CTL_PPI0_DQ_04: u8 = 13;
    pub const IOC_PC16_FUNC_CTL_PWM2_P_0: u8 = 16;
    pub const IOC_PC16_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PC16_FUNC_CTL_TRGM_P_16: u8 = 17;
    pub const IOC_PC16_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PC17_FUNC_CTL_CPU1_NMI: u8 = 24;
    pub const IOC_PC17_FUNC_CTL_DAO_LP: u8 = 9;
    pub const IOC_PC17_FUNC_CTL_ETH0_COL: u8 = 18;
    pub const IOC_PC17_FUNC_CTL_FEMC_DQ_05: u8 = 12;
    pub const IOC_PC17_FUNC_CTL_GPIO_C_17: u8 = 0;
    pub const IOC_PC17_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PC17_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PC17_FUNC_CTL_PPI0_DQ_05: u8 = 13;
    pub const IOC_PC17_FUNC_CTL_PWM2_P_1: u8 = 16;
    pub const IOC_PC17_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PC17_FUNC_CTL_TRGM_P_17: u8 = 17;
    pub const IOC_PC17_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PC18_FUNC_CTL_DAO_RN: u8 = 9;
    pub const IOC_PC18_FUNC_CTL_EWDG2_RST: u8 = 24;
    pub const IOC_PC18_FUNC_CTL_FEMC_DQ_06: u8 = 12;
    pub const IOC_PC18_FUNC_CTL_GPIO_C_18: u8 = 0;
    pub const IOC_PC18_FUNC_CTL_GPTMR3_COMP_1: u8 = 1;
    pub const IOC_PC18_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PC18_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PC18_FUNC_CTL_PPI0_DQ_06: u8 = 13;
    pub const IOC_PC18_FUNC_CTL_PWM2_P_2: u8 = 16;
    pub const IOC_PC18_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PC18_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PC18_FUNC_CTL_TRGM_P_18: u8 = 17;
    pub const IOC_PC18_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PC18_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PC19_FUNC_CTL_DAO_RP: u8 = 9;
    pub const IOC_PC19_FUNC_CTL_EWDG3_RST: u8 = 24;
    pub const IOC_PC19_FUNC_CTL_FEMC_DQ_07: u8 = 12;
    pub const IOC_PC19_FUNC_CTL_GPIO_C_19: u8 = 0;
    pub const IOC_PC19_FUNC_CTL_GPTMR3_CAPT_1: u8 = 1;
    pub const IOC_PC19_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PC19_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PC19_FUNC_CTL_PPI0_DQ_07: u8 = 13;
    pub const IOC_PC19_FUNC_CTL_PWM2_P_3: u8 = 16;
    pub const IOC_PC19_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PC19_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PC19_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PC19_FUNC_CTL_SPI3_CS_3: u8 = 5;
    pub const IOC_PC19_FUNC_CTL_TRGM_P_19: u8 = 17;
    pub const IOC_PC19_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PC20_FUNC_CTL_FEMC_DM_0: u8 = 12;
    pub const IOC_PC20_FUNC_CTL_GPIO_C_20: u8 = 0;
    pub const IOC_PC20_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PC20_FUNC_CTL_PPI0_DM_0: u8 = 13;
    pub const IOC_PC20_FUNC_CTL_PWM2_P_4: u8 = 16;
    pub const IOC_PC20_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PC20_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PC20_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PC20_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PC20_FUNC_CTL_TRGM_P_20: u8 = 17;
    pub const IOC_PC20_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PC21_FUNC_CTL_FEMC_SRDY: u8 = 12;
    pub const IOC_PC21_FUNC_CTL_GPIO_C_21: u8 = 0;
    pub const IOC_PC21_FUNC_CTL_GPTMR3_COMP_2: u8 = 1;
    pub const IOC_PC21_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PC21_FUNC_CTL_PPI0_CTR_0: u8 = 13;
    pub const IOC_PC21_FUNC_CTL_PWM2_P_5: u8 = 16;
    pub const IOC_PC21_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PC21_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PC21_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PC21_FUNC_CTL_SPI2_CS_0: u8 = 5;
    pub const IOC_PC21_FUNC_CTL_TRGM_P_21: u8 = 17;
    pub const IOC_PC21_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PC21_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PC22_FUNC_CTL_CPU0_NMI: u8 = 24;
    pub const IOC_PC22_FUNC_CTL_GPIO_C_22: u8 = 0;
    pub const IOC_PC22_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PC22_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PC22_FUNC_CTL_PWM2_P_6: u8 = 16;
    pub const IOC_PC22_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PC22_FUNC_CTL_TRGM_P_22: u8 = 17;
    pub const IOC_PC22_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PC23_FUNC_CTL_CPU1_NMI: u8 = 24;
    pub const IOC_PC23_FUNC_CTL_GPIO_C_23: u8 = 0;
    pub const IOC_PC23_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PC23_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PC23_FUNC_CTL_PWM2_P_7: u8 = 16;
    pub const IOC_PC23_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PC23_FUNC_CTL_TRGM_P_23: u8 = 17;
    pub const IOC_PC23_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PC24_FUNC_CTL_ETH0_EVTI_0: u8 = 25;
    pub const IOC_PC24_FUNC_CTL_GPIO_C_24: u8 = 0;
    pub const IOC_PC24_FUNC_CTL_GPTMR2_COMP_1: u8 = 1;
    pub const IOC_PC24_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PC24_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PC24_FUNC_CTL_PDM0_CLK: u8 = 9;
    pub const IOC_PC24_FUNC_CTL_PWM3_P_0: u8 = 16;
    pub const IOC_PC24_FUNC_CTL_SPI3_CS_2: u8 = 5;
    pub const IOC_PC24_FUNC_CTL_TRGM_P_24: u8 = 17;
    pub const IOC_PC24_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PC25_FUNC_CTL_ETH0_EVTO_0: u8 = 25;
    pub const IOC_PC25_FUNC_CTL_GPIO_C_25: u8 = 0;
    pub const IOC_PC25_FUNC_CTL_GPTMR2_CAPT_1: u8 = 1;
    pub const IOC_PC25_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PC25_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PC25_FUNC_CTL_PDM0_D_0: u8 = 9;
    pub const IOC_PC25_FUNC_CTL_PWM3_P_1: u8 = 16;
    pub const IOC_PC25_FUNC_CTL_SPI3_CS_1: u8 = 5;
    pub const IOC_PC25_FUNC_CTL_TRGM_P_25: u8 = 17;
    pub const IOC_PC25_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PC25_FUNC_CTL_USB0_ID: u8 = 24;
    pub const IOC_PC26_FUNC_CTL_ETH0_EVTI_1: u8 = 25;
    pub const IOC_PC26_FUNC_CTL_GPIO_C_26: u8 = 0;
    pub const IOC_PC26_FUNC_CTL_GPTMR2_COMP_2: u8 = 1;
    pub const IOC_PC26_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PC26_FUNC_CTL_PDM0_D_1: u8 = 9;
    pub const IOC_PC26_FUNC_CTL_PWM3_P_2: u8 = 16;
    pub const IOC_PC26_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PC26_FUNC_CTL_TRGM_P_26: u8 = 17;
    pub const IOC_PC26_FUNC_CTL_UART3_DE: u8 = 2;
    pub const IOC_PC26_FUNC_CTL_UART3_RTS: u8 = 3;
    pub const IOC_PC26_FUNC_CTL_USB0_OC: u8 = 24;
    pub const IOC_PC27_FUNC_CTL_ETH0_EVTO_1: u8 = 25;
    pub const IOC_PC27_FUNC_CTL_GPIO_C_27: u8 = 0;
    pub const IOC_PC27_FUNC_CTL_PDM0_D_2: u8 = 9;
    pub const IOC_PC27_FUNC_CTL_PWM3_P_3: u8 = 16;
    pub const IOC_PC27_FUNC_CTL_SPI3_CS_0: u8 = 5;
    pub const IOC_PC27_FUNC_CTL_TRGM_P_27: u8 = 17;
    pub const IOC_PC27_FUNC_CTL_UART3_CTS: u8 = 3;
    pub const IOC_PC27_FUNC_CTL_USB0_PWR: u8 = 24;
    pub const IOC_PC28_FUNC_CTL_ETH0_EVTO_2: u8 = 25;
    pub const IOC_PC28_FUNC_CTL_GPIO_C_28: u8 = 0;
    pub const IOC_PC28_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PC28_FUNC_CTL_PDM0_D_3: u8 = 9;
    pub const IOC_PC28_FUNC_CTL_PWM3_P_4: u8 = 16;
    pub const IOC_PC28_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PC28_FUNC_CTL_TRGM_P_28: u8 = 17;
    pub const IOC_PC28_FUNC_CTL_UART7_CTS: u8 = 3;
    pub const IOC_PC29_FUNC_CTL_ETH0_EVTO_3: u8 = 25;
    pub const IOC_PC29_FUNC_CTL_GPIO_C_29: u8 = 0;
    pub const IOC_PC29_FUNC_CTL_GPTMR3_COMP_3: u8 = 1;
    pub const IOC_PC29_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PC29_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PC29_FUNC_CTL_PDM0_CLK: u8 = 9;
    pub const IOC_PC29_FUNC_CTL_PWM3_P_5: u8 = 16;
    pub const IOC_PC29_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PC29_FUNC_CTL_TRGM_P_29: u8 = 17;
    pub const IOC_PC29_FUNC_CTL_UART7_DE: u8 = 2;
    pub const IOC_PC29_FUNC_CTL_UART7_RTS: u8 = 3;
    pub const IOC_PC30_FUNC_CTL_GPIO_C_30: u8 = 0;
    pub const IOC_PC30_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PC30_FUNC_CTL_PWM3_P_6: u8 = 16;
    pub const IOC_PC30_FUNC_CTL_SPI3_DAT2: u8 = 5;
    pub const IOC_PC30_FUNC_CTL_TRGM_P_30: u8 = 17;
    pub const IOC_PC30_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PC31_FUNC_CTL_GPIO_C_31: u8 = 0;
    pub const IOC_PC31_FUNC_CTL_GPTMR2_COMP_3: u8 = 1;
    pub const IOC_PC31_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PC31_FUNC_CTL_PWM3_P_7: u8 = 16;
    pub const IOC_PC31_FUNC_CTL_SPI3_DAT3: u8 = 5;
    pub const IOC_PC31_FUNC_CTL_TRGM_P_31: u8 = 17;
    pub const IOC_PC31_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PD00_FUNC_CTL_ETH0_RXCK: u8 = 18;
    pub const IOC_PD00_FUNC_CTL_GPIO_D_00: u8 = 0;
    pub const IOC_PD00_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PD00_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PD00_FUNC_CTL_PWM3_P_0: u8 = 16;
    pub const IOC_PD00_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PD00_FUNC_CTL_SDM0_CLK_0: u8 = 23;
    pub const IOC_PD00_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PD00_FUNC_CTL_TRGM_P_00: u8 = 17;
    pub const IOC_PD00_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PD01_FUNC_CTL_ETH0_RXDV: u8 = 18;
    pub const IOC_PD01_FUNC_CTL_GPIO_D_01: u8 = 0;
    pub const IOC_PD01_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PD01_FUNC_CTL_I2S0_MCLK: u8 = 8;
    pub const IOC_PD01_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PD01_FUNC_CTL_PWM3_P_1: u8 = 16;
    pub const IOC_PD01_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PD01_FUNC_CTL_SDM0_DAT_0: u8 = 23;
    pub const IOC_PD01_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PD01_FUNC_CTL_TRGM_P_01: u8 = 17;
    pub const IOC_PD01_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PD02_FUNC_CTL_ETH0_RXER: u8 = 18;
    pub const IOC_PD02_FUNC_CTL_GPIO_D_02: u8 = 0;
    pub const IOC_PD02_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PD02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PD02_FUNC_CTL_I2S0_BCLK: u8 = 8;
    pub const IOC_PD02_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PD02_FUNC_CTL_PWM3_P_2: u8 = 16;
    pub const IOC_PD02_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PD02_FUNC_CTL_SDM0_CLK_1: u8 = 23;
    pub const IOC_PD02_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PD02_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PD02_FUNC_CTL_TRGM_P_02: u8 = 17;
    pub const IOC_PD02_FUNC_CTL_UART7_DE: u8 = 2;
    pub const IOC_PD02_FUNC_CTL_UART7_RTS: u8 = 3;
    pub const IOC_PD03_FUNC_CTL_ETH0_RXD_0: u8 = 18;
    pub const IOC_PD03_FUNC_CTL_GPIO_D_03: u8 = 0;
    pub const IOC_PD03_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PD03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PD03_FUNC_CTL_I2S0_FCLK: u8 = 8;
    pub const IOC_PD03_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PD03_FUNC_CTL_PWM3_P_3: u8 = 16;
    pub const IOC_PD03_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PD03_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PD03_FUNC_CTL_SDM0_DAT_1: u8 = 23;
    pub const IOC_PD03_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PD03_FUNC_CTL_SPI2_DAT2: u8 = 5;
    pub const IOC_PD03_FUNC_CTL_TRGM_P_03: u8 = 17;
    pub const IOC_PD03_FUNC_CTL_UART7_CTS: u8 = 3;
    pub const IOC_PD04_FUNC_CTL_ETH0_RXD_1: u8 = 18;
    pub const IOC_PD04_FUNC_CTL_GPIO_D_04: u8 = 0;
    pub const IOC_PD04_FUNC_CTL_I2S0_RXD_0: u8 = 8;
    pub const IOC_PD04_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PD04_FUNC_CTL_PWM3_P_4: u8 = 16;
    pub const IOC_PD04_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PD04_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PD04_FUNC_CTL_SDM0_CLK_2: u8 = 23;
    pub const IOC_PD04_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PD04_FUNC_CTL_SPI2_DAT3: u8 = 5;
    pub const IOC_PD04_FUNC_CTL_TRGM_P_04: u8 = 17;
    pub const IOC_PD04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PD05_FUNC_CTL_ETH0_RXD_2: u8 = 18;
    pub const IOC_PD05_FUNC_CTL_GPIO_D_05: u8 = 0;
    pub const IOC_PD05_FUNC_CTL_GPTMR1_COMP_2: u8 = 1;
    pub const IOC_PD05_FUNC_CTL_I2S0_RXD_1: u8 = 8;
    pub const IOC_PD05_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PD05_FUNC_CTL_PWM3_P_5: u8 = 16;
    pub const IOC_PD05_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PD05_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PD05_FUNC_CTL_SDM0_DAT_2: u8 = 23;
    pub const IOC_PD05_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PD05_FUNC_CTL_SPI2_CS_0: u8 = 5;
    pub const IOC_PD05_FUNC_CTL_TRGM_P_05: u8 = 17;
    pub const IOC_PD05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PD05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PD06_FUNC_CTL_ETH0_RXD_3: u8 = 18;
    pub const IOC_PD06_FUNC_CTL_GPIO_D_06: u8 = 0;
    pub const IOC_PD06_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PD06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PD06_FUNC_CTL_I2S0_RXD_2: u8 = 8;
    pub const IOC_PD06_FUNC_CTL_PWM3_P_6: u8 = 16;
    pub const IOC_PD06_FUNC_CTL_SDM0_CLK_3: u8 = 23;
    pub const IOC_PD06_FUNC_CTL_SPI2_CS_2: u8 = 5;
    pub const IOC_PD06_FUNC_CTL_TRGM_P_06: u8 = 17;
    pub const IOC_PD06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PD07_FUNC_CTL_ETH0_TXER: u8 = 18;
    pub const IOC_PD07_FUNC_CTL_GPIO_D_07: u8 = 0;
    pub const IOC_PD07_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PD07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PD07_FUNC_CTL_I2S0_RXD_3: u8 = 8;
    pub const IOC_PD07_FUNC_CTL_PWM3_P_7: u8 = 16;
    pub const IOC_PD07_FUNC_CTL_SDM0_DAT_3: u8 = 23;
    pub const IOC_PD07_FUNC_CTL_SPI2_CS_1: u8 = 5;
    pub const IOC_PD07_FUNC_CTL_TRGM_P_07: u8 = 17;
    pub const IOC_PD07_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PD08_FUNC_CTL_GPIO_D_08: u8 = 0;
    pub const IOC_PD08_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PD08_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PD08_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PD08_FUNC_CTL_PWM1_P_0: u8 = 16;
    pub const IOC_PD08_FUNC_CTL_SPI2_CS_3: u8 = 5;
    pub const IOC_PD08_FUNC_CTL_TRGM_P_08: u8 = 17;
    pub const IOC_PD08_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PD09_FUNC_CTL_GPIO_D_09: u8 = 0;
    pub const IOC_PD09_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PD09_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PD09_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PD09_FUNC_CTL_PWM1_P_1: u8 = 16;
    pub const IOC_PD09_FUNC_CTL_TRGM_P_09: u8 = 17;
    pub const IOC_PD09_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PD10_FUNC_CTL_ETH0_EVTI_0: u8 = 25;
    pub const IOC_PD10_FUNC_CTL_GPIO_D_10: u8 = 0;
    pub const IOC_PD10_FUNC_CTL_GPTMR0_COMP_2: u8 = 1;
    pub const IOC_PD10_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PD10_FUNC_CTL_PWM1_P_2: u8 = 16;
    pub const IOC_PD10_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PD10_FUNC_CTL_TRGM_P_10: u8 = 17;
    pub const IOC_PD10_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PD10_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PD11_FUNC_CTL_ETH0_EVTO_0: u8 = 25;
    pub const IOC_PD11_FUNC_CTL_GPIO_D_11: u8 = 0;
    pub const IOC_PD11_FUNC_CTL_PWM1_P_3: u8 = 16;
    pub const IOC_PD11_FUNC_CTL_SPI0_CS_0: u8 = 5;
    pub const IOC_PD11_FUNC_CTL_TRGM_P_11: u8 = 17;
    pub const IOC_PD11_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PD12_FUNC_CTL_ETH0_EVTI_1: u8 = 25;
    pub const IOC_PD12_FUNC_CTL_GPIO_D_12: u8 = 0;
    pub const IOC_PD12_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PD12_FUNC_CTL_PWM1_P_4: u8 = 16;
    pub const IOC_PD12_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PD12_FUNC_CTL_TRGM_P_12: u8 = 17;
    pub const IOC_PD12_FUNC_CTL_UART3_CTS: u8 = 3;
    pub const IOC_PD13_FUNC_CTL_ETH0_EVTO_1: u8 = 25;
    pub const IOC_PD13_FUNC_CTL_GPIO_D_13: u8 = 0;
    pub const IOC_PD13_FUNC_CTL_GPTMR1_COMP_3: u8 = 1;
    pub const IOC_PD13_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PD13_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PD13_FUNC_CTL_PWM1_P_5: u8 = 16;
    pub const IOC_PD13_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PD13_FUNC_CTL_TRGM_P_13: u8 = 17;
    pub const IOC_PD13_FUNC_CTL_UART3_DE: u8 = 2;
    pub const IOC_PD13_FUNC_CTL_UART3_RTS: u8 = 3;
    pub const IOC_PD14_FUNC_CTL_ETH0_EVTO_2: u8 = 25;
    pub const IOC_PD14_FUNC_CTL_GPIO_D_14: u8 = 0;
    pub const IOC_PD14_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PD14_FUNC_CTL_PWM1_P_6: u8 = 16;
    pub const IOC_PD14_FUNC_CTL_TRGM_P_14: u8 = 17;
    pub const IOC_PD14_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PD15_FUNC_CTL_ETH0_EVTO_3: u8 = 25;
    pub const IOC_PD15_FUNC_CTL_GPIO_D_15: u8 = 0;
    pub const IOC_PD15_FUNC_CTL_GPTMR0_COMP_3: u8 = 1;
    pub const IOC_PD15_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PD15_FUNC_CTL_PWM1_P_7: u8 = 16;
    pub const IOC_PD15_FUNC_CTL_TRGM_P_15: u8 = 17;
    pub const IOC_PD15_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PD16_FUNC_CTL_ETH0_TXCK: u8 = 18;
    pub const IOC_PD16_FUNC_CTL_GPIO_D_16: u8 = 0;
    pub const IOC_PD16_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PD16_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PD16_FUNC_CTL_PWM2_P_0: u8 = 16;
    pub const IOC_PD16_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PD16_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PD16_FUNC_CTL_TRGM_P_16: u8 = 17;
    pub const IOC_PD16_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PD17_FUNC_CTL_ETH0_TXEN: u8 = 18;
    pub const IOC_PD17_FUNC_CTL_GPIO_D_17: u8 = 0;
    pub const IOC_PD17_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PD17_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PD17_FUNC_CTL_PWM2_P_1: u8 = 16;
    pub const IOC_PD17_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PD17_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PD17_FUNC_CTL_TRGM_P_17: u8 = 17;
    pub const IOC_PD17_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PD18_FUNC_CTL_ETH0_TXD_0: u8 = 18;
    pub const IOC_PD18_FUNC_CTL_GPIO_D_18: u8 = 0;
    pub const IOC_PD18_FUNC_CTL_GPTMR3_COMP_1: u8 = 1;
    pub const IOC_PD18_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PD18_FUNC_CTL_I2S0_TXD_0: u8 = 8;
    pub const IOC_PD18_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PD18_FUNC_CTL_PWM2_P_2: u8 = 16;
    pub const IOC_PD18_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PD18_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PD18_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PD18_FUNC_CTL_TRGM_P_18: u8 = 17;
    pub const IOC_PD18_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PD18_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PD19_FUNC_CTL_ETH0_TXD_1: u8 = 18;
    pub const IOC_PD19_FUNC_CTL_GPIO_D_19: u8 = 0;
    pub const IOC_PD19_FUNC_CTL_GPTMR3_CAPT_1: u8 = 1;
    pub const IOC_PD19_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PD19_FUNC_CTL_I2S0_TXD_1: u8 = 8;
    pub const IOC_PD19_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PD19_FUNC_CTL_PWM2_P_3: u8 = 16;
    pub const IOC_PD19_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PD19_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PD19_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PD19_FUNC_CTL_SPI3_DAT2: u8 = 5;
    pub const IOC_PD19_FUNC_CTL_TRGM_P_19: u8 = 17;
    pub const IOC_PD19_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PD20_FUNC_CTL_ETH0_TXD_2: u8 = 18;
    pub const IOC_PD20_FUNC_CTL_GPIO_D_20: u8 = 0;
    pub const IOC_PD20_FUNC_CTL_I2S0_TXD_2: u8 = 8;
    pub const IOC_PD20_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PD20_FUNC_CTL_PWM2_P_4: u8 = 16;
    pub const IOC_PD20_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PD20_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PD20_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PD20_FUNC_CTL_SPI3_DAT3: u8 = 5;
    pub const IOC_PD20_FUNC_CTL_TRGM_P_20: u8 = 17;
    pub const IOC_PD20_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PD21_FUNC_CTL_ETH0_TXD_3: u8 = 18;
    pub const IOC_PD21_FUNC_CTL_GPIO_D_21: u8 = 0;
    pub const IOC_PD21_FUNC_CTL_GPTMR3_COMP_2: u8 = 1;
    pub const IOC_PD21_FUNC_CTL_I2S0_TXD_3: u8 = 8;
    pub const IOC_PD21_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PD21_FUNC_CTL_PWM2_P_5: u8 = 16;
    pub const IOC_PD21_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PD21_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PD21_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PD21_FUNC_CTL_SPI3_CS_0: u8 = 5;
    pub const IOC_PD21_FUNC_CTL_TRGM_P_21: u8 = 17;
    pub const IOC_PD21_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PD21_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PD22_FUNC_CTL_GPIO_D_22: u8 = 0;
    pub const IOC_PD22_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PD22_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PD22_FUNC_CTL_PDM0_CLK: u8 = 9;
    pub const IOC_PD22_FUNC_CTL_PWM2_P_6: u8 = 16;
    pub const IOC_PD22_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PD22_FUNC_CTL_SPI3_CS_2: u8 = 5;
    pub const IOC_PD22_FUNC_CTL_TRGM_P_22: u8 = 17;
    pub const IOC_PD22_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PD23_FUNC_CTL_GPIO_D_23: u8 = 0;
    pub const IOC_PD23_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PD23_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PD23_FUNC_CTL_PDM0_D_0: u8 = 9;
    pub const IOC_PD23_FUNC_CTL_PWM2_P_7: u8 = 16;
    pub const IOC_PD23_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PD23_FUNC_CTL_SPI3_CS_1: u8 = 5;
    pub const IOC_PD23_FUNC_CTL_TRGM_P_23: u8 = 17;
    pub const IOC_PD23_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PD23_FUNC_CTL_USB0_ID: u8 = 24;
    pub const IOC_PD24_FUNC_CTL_GPIO_D_24: u8 = 0;
    pub const IOC_PD24_FUNC_CTL_GPTMR2_COMP_1: u8 = 1;
    pub const IOC_PD24_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PD24_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PD24_FUNC_CTL_PDM0_D_1: u8 = 9;
    pub const IOC_PD24_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PD24_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PD24_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PD24_FUNC_CTL_SPI3_CS_3: u8 = 5;
    pub const IOC_PD24_FUNC_CTL_TRGM_P_24: u8 = 17;
    pub const IOC_PD24_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PD24_FUNC_CTL_USB0_OC: u8 = 24;
    pub const IOC_PD25_FUNC_CTL_GPIO_D_25: u8 = 0;
    pub const IOC_PD25_FUNC_CTL_GPTMR2_CAPT_1: u8 = 1;
    pub const IOC_PD25_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PD25_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PD25_FUNC_CTL_PDM0_D_2: u8 = 9;
    pub const IOC_PD25_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PD25_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PD25_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PD25_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PD25_FUNC_CTL_TRGM_P_25: u8 = 17;
    pub const IOC_PD25_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PD25_FUNC_CTL_USB0_PWR: u8 = 24;
    pub const IOC_PD26_FUNC_CTL_GPIO_D_26: u8 = 0;
    pub const IOC_PD26_FUNC_CTL_GPTMR2_COMP_2: u8 = 1;
    pub const IOC_PD26_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PD26_FUNC_CTL_PDM0_D_3: u8 = 9;
    pub const IOC_PD26_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PD26_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PD26_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PD26_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PD26_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PD26_FUNC_CTL_TRGM_P_26: u8 = 17;
    pub const IOC_PD26_FUNC_CTL_UART6_DE: u8 = 2;
    pub const IOC_PD26_FUNC_CTL_UART6_RTS: u8 = 3;
    pub const IOC_PD27_FUNC_CTL_GPIO_D_27: u8 = 0;
    pub const IOC_PD27_FUNC_CTL_PDM0_CLK: u8 = 9;
    pub const IOC_PD27_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PD27_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PD27_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PD27_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PD27_FUNC_CTL_SPI1_CS_0: u8 = 5;
    pub const IOC_PD27_FUNC_CTL_TRGM_P_27: u8 = 17;
    pub const IOC_PD27_FUNC_CTL_UART6_CTS: u8 = 3;
    pub const IOC_PD28_FUNC_CTL_GPIO_D_28: u8 = 0;
    pub const IOC_PD28_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PD28_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PD28_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PD28_FUNC_CTL_TRGM_P_28: u8 = 17;
    pub const IOC_PD28_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PD29_FUNC_CTL_GPIO_D_29: u8 = 0;
    pub const IOC_PD29_FUNC_CTL_GPTMR3_COMP_3: u8 = 1;
    pub const IOC_PD29_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PD29_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PD29_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PD29_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PD29_FUNC_CTL_TRGM_P_29: u8 = 17;
    pub const IOC_PD29_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PD29_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PD30_FUNC_CTL_GPIO_D_30: u8 = 0;
    pub const IOC_PD30_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PD30_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PD30_FUNC_CTL_TRGM_P_30: u8 = 17;
    pub const IOC_PD30_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PD31_FUNC_CTL_GPIO_D_31: u8 = 0;
    pub const IOC_PD31_FUNC_CTL_GPTMR2_COMP_3: u8 = 1;
    pub const IOC_PD31_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PD31_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PD31_FUNC_CTL_TRGM_P_31: u8 = 17;
    pub const IOC_PD31_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PX00_FUNC_CTL_GPIO_X_00: u8 = 0;
    pub const IOC_PX00_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PX00_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PX00_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PX00_FUNC_CTL_TRGM_P_00: u8 = 17;
    pub const IOC_PX00_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PX00_FUNC_CTL_XPI0_CB_DQS: u8 = 14;
    pub const IOC_PX01_FUNC_CTL_GPIO_X_01: u8 = 0;
    pub const IOC_PX01_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PX01_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PX01_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PX01_FUNC_CTL_TRGM_P_01: u8 = 17;
    pub const IOC_PX01_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PX01_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PX02_FUNC_CTL_GPIO_X_02: u8 = 0;
    pub const IOC_PX02_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PX02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PX02_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PX02_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PX02_FUNC_CTL_TRGM_P_02: u8 = 17;
    pub const IOC_PX02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PX02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PX02_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PX03_FUNC_CTL_GPIO_X_03: u8 = 0;
    pub const IOC_PX03_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PX03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PX03_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PX03_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PX03_FUNC_CTL_SPI0_CS_3: u8 = 5;
    pub const IOC_PX03_FUNC_CTL_TRGM_P_03: u8 = 17;
    pub const IOC_PX03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PX03_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PX04_FUNC_CTL_GPIO_X_04: u8 = 0;
    pub const IOC_PX04_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PX04_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PX04_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PX04_FUNC_CTL_TRGM_P_04: u8 = 17;
    pub const IOC_PX04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PX04_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PX05_FUNC_CTL_GPIO_X_05: u8 = 0;
    pub const IOC_PX05_FUNC_CTL_GPTMR1_COMP_2: u8 = 1;
    pub const IOC_PX05_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PX05_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PX05_FUNC_CTL_SPI1_CS_0: u8 = 5;
    pub const IOC_PX05_FUNC_CTL_TRGM_P_05: u8 = 17;
    pub const IOC_PX05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PX05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PX05_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PX06_FUNC_CTL_GPIO_X_06: u8 = 0;
    pub const IOC_PX06_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PX06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PX06_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PX06_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PX06_FUNC_CTL_TRGM_P_06: u8 = 17;
    pub const IOC_PX06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PX06_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PX07_FUNC_CTL_GPIO_X_07: u8 = 0;
    pub const IOC_PX07_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PX07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PX07_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PX07_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PX07_FUNC_CTL_TRGM_P_07: u8 = 17;
    pub const IOC_PX07_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PX07_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
    pub const IOC_PY00_FUNC_CTL_DAO_LN: u8 = 9;
    pub const IOC_PY00_FUNC_CTL_GPIO_Y_00: u8 = 0;
    pub const IOC_PY00_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PY00_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PY00_FUNC_CTL_PWM2_P_0: u8 = 16;
    pub const IOC_PY00_FUNC_CTL_TRGM_P_16: u8 = 17;
    pub const IOC_PY00_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PY01_FUNC_CTL_DAO_LP: u8 = 9;
    pub const IOC_PY01_FUNC_CTL_GPIO_Y_01: u8 = 0;
    pub const IOC_PY01_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PY01_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PY01_FUNC_CTL_PWM2_P_1: u8 = 16;
    pub const IOC_PY01_FUNC_CTL_TRGM_P_17: u8 = 17;
    pub const IOC_PY01_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PY02_FUNC_CTL_DAO_RN: u8 = 9;
    pub const IOC_PY02_FUNC_CTL_GPIO_Y_02: u8 = 0;
    pub const IOC_PY02_FUNC_CTL_GPTMR3_COMP_1: u8 = 1;
    pub const IOC_PY02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PY02_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PY02_FUNC_CTL_PWM2_P_2: u8 = 16;
    pub const IOC_PY02_FUNC_CTL_TRGM_P_18: u8 = 17;
    pub const IOC_PY02_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PY02_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PY03_FUNC_CTL_DAO_RP: u8 = 9;
    pub const IOC_PY03_FUNC_CTL_GPIO_Y_03: u8 = 0;
    pub const IOC_PY03_FUNC_CTL_GPTMR3_CAPT_1: u8 = 1;
    pub const IOC_PY03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PY03_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PY03_FUNC_CTL_PWM2_P_3: u8 = 16;
    pub const IOC_PY03_FUNC_CTL_SPI3_CS_3: u8 = 5;
    pub const IOC_PY03_FUNC_CTL_TRGM_P_19: u8 = 17;
    pub const IOC_PY03_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PY04_FUNC_CTL_GPIO_Y_04: u8 = 0;
    pub const IOC_PY04_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PY04_FUNC_CTL_PWM2_P_4: u8 = 16;
    pub const IOC_PY04_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PY04_FUNC_CTL_TRGM_P_20: u8 = 17;
    pub const IOC_PY04_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PY05_FUNC_CTL_GPIO_Y_05: u8 = 0;
    pub const IOC_PY05_FUNC_CTL_GPTMR3_COMP_2: u8 = 1;
    pub const IOC_PY05_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PY05_FUNC_CTL_PWM2_P_5: u8 = 16;
    pub const IOC_PY05_FUNC_CTL_SPI2_CS_0: u8 = 5;
    pub const IOC_PY05_FUNC_CTL_TRGM_P_21: u8 = 17;
    pub const IOC_PY05_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PY05_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PY06_FUNC_CTL_GPIO_Y_06: u8 = 0;
    pub const IOC_PY06_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PY06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PY06_FUNC_CTL_PWM2_P_6: u8 = 16;
    pub const IOC_PY06_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PY06_FUNC_CTL_TRGM_P_22: u8 = 17;
    pub const IOC_PY06_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PY07_FUNC_CTL_GPIO_Y_07: u8 = 0;
    pub const IOC_PY07_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PY07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PY07_FUNC_CTL_PWM2_P_7: u8 = 16;
    pub const IOC_PY07_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PY07_FUNC_CTL_TRGM_P_23: u8 = 17;
    pub const IOC_PY07_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const PIOC_PY00_FUNC_CTL_PGPIO_Y_00: u8 = 0;
    pub const PIOC_PY00_FUNC_CTL_PTMR_COMP_0: u8 = 2;
    pub const PIOC_PY00_FUNC_CTL_PURT_TXD: u8 = 1;
    pub const PIOC_PY00_FUNC_CTL_SOC_PY_00: u8 = 3;
    pub const PIOC_PY01_FUNC_CTL_PGPIO_Y_01: u8 = 0;
    pub const PIOC_PY01_FUNC_CTL_PTMR_COMP_1: u8 = 2;
    pub const PIOC_PY01_FUNC_CTL_PURT_RXD: u8 = 1;
    pub const PIOC_PY01_FUNC_CTL_SOC_PY_01: u8 = 3;
    pub const PIOC_PY02_FUNC_CTL_PGPIO_Y_02: u8 = 0;
    pub const PIOC_PY02_FUNC_CTL_PTMR_COMP_2: u8 = 2;
    pub const PIOC_PY02_FUNC_CTL_PURT_RTS: u8 = 1;
    pub const PIOC_PY02_FUNC_CTL_SOC_PY_02: u8 = 3;
    pub const PIOC_PY03_FUNC_CTL_PGPIO_Y_03: u8 = 0;
    pub const PIOC_PY03_FUNC_CTL_PTMR_COMP_3: u8 = 2;
    pub const PIOC_PY03_FUNC_CTL_PURT_CTS: u8 = 1;
    pub const PIOC_PY03_FUNC_CTL_SOC_PY_03: u8 = 3;
    pub const PIOC_PY04_FUNC_CTL_PGPIO_Y_04: u8 = 0;
    pub const PIOC_PY04_FUNC_CTL_PTMR_COMP_0: u8 = 2;
    pub const PIOC_PY04_FUNC_CTL_SOC_PY_04: u8 = 3;
    pub const PIOC_PY05_FUNC_CTL_PGPIO_Y_05: u8 = 0;
    pub const PIOC_PY05_FUNC_CTL_PTMR_CAPT_0: u8 = 2;
    pub const PIOC_PY05_FUNC_CTL_PWDG_RSTN: u8 = 1;
    pub const PIOC_PY05_FUNC_CTL_SOC_PY_05: u8 = 3;
    pub const PIOC_PY06_FUNC_CTL_PGPIO_Y_06: u8 = 0;
    pub const PIOC_PY06_FUNC_CTL_PTMR_COMP_1: u8 = 2;
    pub const PIOC_PY06_FUNC_CTL_SOC_PY_06: u8 = 3;
    pub const PIOC_PY07_FUNC_CTL_PGPIO_Y_07: u8 = 0;
    pub const PIOC_PY07_FUNC_CTL_PTMR_CAPT_1: u8 = 2;
    pub const PIOC_PY07_FUNC_CTL_SOC_PY_07: u8 = 3;
}
pub mod trgmmux {
    //! `TRGMMUX` definitions
}
