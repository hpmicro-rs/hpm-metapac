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
    #[doc = "11 - GPTMR0"]
    GPTMR0 = 11,
    #[doc = "12 - GPTMR1"]
    GPTMR1 = 12,
    #[doc = "13 - GPTMR2"]
    GPTMR2 = 13,
    #[doc = "14 - GPTMR3"]
    GPTMR3 = 14,
    #[doc = "15 - OWR0"]
    OWR0 = 15,
    #[doc = "16 - OWR1"]
    OWR1 = 16,
    #[doc = "17 - EUI0"]
    EUI0 = 17,
    #[doc = "18 - EUI1"]
    EUI1 = 18,
    #[doc = "19 - UART0"]
    UART0 = 19,
    #[doc = "20 - UART1"]
    UART1 = 20,
    #[doc = "21 - UART2"]
    UART2 = 21,
    #[doc = "22 - UART3"]
    UART3 = 22,
    #[doc = "23 - UART4"]
    UART4 = 23,
    #[doc = "24 - UART5"]
    UART5 = 24,
    #[doc = "25 - UART6"]
    UART6 = 25,
    #[doc = "26 - UART7"]
    UART7 = 26,
    #[doc = "27 - I2C0"]
    I2C0 = 27,
    #[doc = "28 - I2C1"]
    I2C1 = 28,
    #[doc = "29 - I2C2"]
    I2C2 = 29,
    #[doc = "30 - I2C3"]
    I2C3 = 30,
    #[doc = "31 - SPI0"]
    SPI0 = 31,
    #[doc = "32 - SPI1"]
    SPI1 = 32,
    #[doc = "33 - SPI2"]
    SPI2 = 33,
    #[doc = "34 - SPI3"]
    SPI3 = 34,
    #[doc = "35 - TSNS"]
    TSNS = 35,
    #[doc = "36 - MBX0A"]
    MBX0A = 36,
    #[doc = "37 - MBX0B"]
    MBX0B = 37,
    #[doc = "38 - EWDG0"]
    EWDG0 = 38,
    #[doc = "39 - EWDG1"]
    EWDG1 = 39,
    #[doc = "40 - HDMA"]
    HDMA = 40,
    #[doc = "41 - LOBS"]
    LOBS = 41,
    #[doc = "42 - ADC0"]
    ADC0 = 42,
    #[doc = "43 - ADC1"]
    ADC1 = 43,
    #[doc = "44 - ACMP0_0"]
    ACMP0_0 = 44,
    #[doc = "45 - ACMP0_1"]
    ACMP0_1 = 45,
    #[doc = "46 - MCAN0"]
    MCAN0 = 46,
    #[doc = "47 - MCAN1"]
    MCAN1 = 47,
    #[doc = "48 - MCAN2"]
    MCAN2 = 48,
    #[doc = "49 - MCAN3"]
    MCAN3 = 49,
    #[doc = "50 - PTPC"]
    PTPC = 50,
    #[doc = "51 - QEI0"]
    QEI0 = 51,
    #[doc = "52 - QEI1"]
    QEI1 = 52,
    #[doc = "53 - PWM0"]
    PWM0 = 53,
    #[doc = "54 - PWM1"]
    PWM1 = 54,
    #[doc = "55 - SDM0"]
    SDM0 = 55,
    #[doc = "56 - TRGM_0"]
    TRGM_0 = 56,
    #[doc = "57 - TRGM_1"]
    TRGM_1 = 57,
    #[doc = "58 - ENET0"]
    ENET0 = 58,
    #[doc = "59 - NTMR0"]
    NTMR0 = 59,
    #[doc = "60 - USB0"]
    USB0 = 60,
    #[doc = "61 - ESC"]
    ESC = 61,
    #[doc = "62 - ESC_SYNC0"]
    ESC_SYNC0 = 62,
    #[doc = "63 - ESC_SYNC1"]
    ESC_SYNC1 = 63,
    #[doc = "64 - ESC_RESET"]
    ESC_RESET = 64,
    #[doc = "65 - XPI0"]
    XPI0 = 65,
    #[doc = "66 - PPI"]
    PPI = 66,
    #[doc = "67 - XDMA"]
    XDMA = 67,
    #[doc = "68 - PGPIO"]
    PGPIO = 68,
    #[doc = "69 - PEWDG"]
    PEWDG = 69,
    #[doc = "70 - PTMR"]
    PTMR = 70,
    #[doc = "71 - PUART"]
    PUART = 71,
    #[doc = "72 - FUSE"]
    FUSE = 72,
    #[doc = "73 - DGO_PAD_WAKEUP"]
    DGO_PAD_WAKEUP = 73,
    #[doc = "74 - DGO_CNT_WAKEUP"]
    DGO_CNT_WAKEUP = 74,
    #[doc = "75 - BROWNOUT"]
    BROWNOUT = 75,
    #[doc = "76 - SYSCTL"]
    SYSCTL = 76,
    #[doc = "77 - CPU0"]
    CPU0 = 77,
    #[doc = "78 - DEBUG0"]
    DEBUG0 = 78,
    #[doc = "79 - DEBUG1"]
    DEBUG1 = 79,
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
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
        fn GPTMR0();
        fn GPTMR1();
        fn GPTMR2();
        fn GPTMR3();
        fn OWR0();
        fn OWR1();
        fn EUI0();
        fn EUI1();
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
        fn LOBS();
        fn ADC0();
        fn ADC1();
        fn ACMP0_0();
        fn ACMP0_1();
        fn MCAN0();
        fn MCAN1();
        fn MCAN2();
        fn MCAN3();
        fn PTPC();
        fn QEI0();
        fn QEI1();
        fn PWM0();
        fn PWM1();
        fn SDM0();
        fn TRGM_0();
        fn TRGM_1();
        fn ENET0();
        fn NTMR0();
        fn USB0();
        fn ESC();
        fn ESC_SYNC0();
        fn ESC_SYNC1();
        fn ESC_RESET();
        fn XPI0();
        fn PPI();
        fn XDMA();
        fn PGPIO();
        fn PEWDG();
        fn PTMR();
        fn PUART();
        fn FUSE();
        fn DGO_PAD_WAKEUP();
        fn DGO_CNT_WAKEUP();
        fn BROWNOUT();
        fn SYSCTL();
        fn CPU0();
        fn DEBUG0();
        fn DEBUG1();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 80] = [
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
        Vector { _handler: GPTMR0 },
        Vector { _handler: GPTMR1 },
        Vector { _handler: GPTMR2 },
        Vector { _handler: GPTMR3 },
        Vector { _handler: OWR0 },
        Vector { _handler: OWR1 },
        Vector { _handler: EUI0 },
        Vector { _handler: EUI1 },
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
        Vector { _handler: LOBS },
        Vector { _handler: ADC0 },
        Vector { _handler: ADC1 },
        Vector { _handler: ACMP0_0 },
        Vector { _handler: ACMP0_1 },
        Vector { _handler: MCAN0 },
        Vector { _handler: MCAN1 },
        Vector { _handler: MCAN2 },
        Vector { _handler: MCAN3 },
        Vector { _handler: PTPC },
        Vector { _handler: QEI0 },
        Vector { _handler: QEI1 },
        Vector { _handler: PWM0 },
        Vector { _handler: PWM1 },
        Vector { _handler: SDM0 },
        Vector { _handler: TRGM_0 },
        Vector { _handler: TRGM_1 },
        Vector { _handler: ENET0 },
        Vector { _handler: NTMR0 },
        Vector { _handler: USB0 },
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
        Vector { _handler: PPI },
        Vector { _handler: XDMA },
        Vector { _handler: PGPIO },
        Vector { _handler: PEWDG },
        Vector { _handler: PTMR },
        Vector { _handler: PUART },
        Vector { _handler: FUSE },
        Vector {
            _handler: DGO_PAD_WAKEUP,
        },
        Vector {
            _handler: DGO_CNT_WAKEUP,
        },
        Vector { _handler: BROWNOUT },
        Vector { _handler: SYSCTL },
        Vector { _handler: CPU0 },
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
unsafe impl riscv_pac::InterruptNumber for Interrupt {
    const MAX_INTERRUPT_NUMBER: usize = 1024;
    #[inline(always)]
    fn number(self) -> usize {
        self as usize
    }
    #[inline(always)]
    fn from_number(value: usize) -> Result<Self, riscv_pac::result::Error> {
        match value {
            0 => Ok(Self::CORE_LOCAL),
            1 => Ok(Self::GPIO0_A),
            2 => Ok(Self::GPIO0_B),
            3 => Ok(Self::GPIO0_C),
            4 => Ok(Self::GPIO0_D),
            5 => Ok(Self::GPIO0_E),
            6 => Ok(Self::GPIO0_F),
            7 => Ok(Self::GPIO0_V),
            8 => Ok(Self::GPIO0_W),
            9 => Ok(Self::GPIO0_X),
            10 => Ok(Self::GPIO0_Y),
            11 => Ok(Self::GPTMR0),
            12 => Ok(Self::GPTMR1),
            13 => Ok(Self::GPTMR2),
            14 => Ok(Self::GPTMR3),
            15 => Ok(Self::OWR0),
            16 => Ok(Self::OWR1),
            17 => Ok(Self::EUI0),
            18 => Ok(Self::EUI1),
            19 => Ok(Self::UART0),
            20 => Ok(Self::UART1),
            21 => Ok(Self::UART2),
            22 => Ok(Self::UART3),
            23 => Ok(Self::UART4),
            24 => Ok(Self::UART5),
            25 => Ok(Self::UART6),
            26 => Ok(Self::UART7),
            27 => Ok(Self::I2C0),
            28 => Ok(Self::I2C1),
            29 => Ok(Self::I2C2),
            30 => Ok(Self::I2C3),
            31 => Ok(Self::SPI0),
            32 => Ok(Self::SPI1),
            33 => Ok(Self::SPI2),
            34 => Ok(Self::SPI3),
            35 => Ok(Self::TSNS),
            36 => Ok(Self::MBX0A),
            37 => Ok(Self::MBX0B),
            38 => Ok(Self::EWDG0),
            39 => Ok(Self::EWDG1),
            40 => Ok(Self::HDMA),
            41 => Ok(Self::LOBS),
            42 => Ok(Self::ADC0),
            43 => Ok(Self::ADC1),
            44 => Ok(Self::ACMP0_0),
            45 => Ok(Self::ACMP0_1),
            46 => Ok(Self::MCAN0),
            47 => Ok(Self::MCAN1),
            48 => Ok(Self::MCAN2),
            49 => Ok(Self::MCAN3),
            50 => Ok(Self::PTPC),
            51 => Ok(Self::QEI0),
            52 => Ok(Self::QEI1),
            53 => Ok(Self::PWM0),
            54 => Ok(Self::PWM1),
            55 => Ok(Self::SDM0),
            56 => Ok(Self::TRGM_0),
            57 => Ok(Self::TRGM_1),
            58 => Ok(Self::ENET0),
            59 => Ok(Self::NTMR0),
            60 => Ok(Self::USB0),
            61 => Ok(Self::ESC),
            62 => Ok(Self::ESC_SYNC0),
            63 => Ok(Self::ESC_SYNC1),
            64 => Ok(Self::ESC_RESET),
            65 => Ok(Self::XPI0),
            66 => Ok(Self::PPI),
            67 => Ok(Self::XDMA),
            68 => Ok(Self::PGPIO),
            69 => Ok(Self::PEWDG),
            70 => Ok(Self::PTMR),
            71 => Ok(Self::PUART),
            72 => Ok(Self::FUSE),
            73 => Ok(Self::DGO_PAD_WAKEUP),
            74 => Ok(Self::DGO_CNT_WAKEUP),
            75 => Ok(Self::BROWNOUT),
            76 => Ok(Self::SYSCTL),
            77 => Ok(Self::CPU0),
            78 => Ok(Self::DEBUG0),
            79 => Ok(Self::DEBUG1),

            _ => Err(riscv_pac::result::Error::InvalidVariant(value)),
        }
    }
}
unsafe impl riscv_pac::ExternalInterruptNumber for Interrupt {}
#[path = "../../peripherals/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../../peripherals/plic_common.rs"]
pub mod plic;
#[path = "../../peripherals/plicsw_common.rs"]
pub mod plicsw;
pub const FLASH_BASE: usize = 2147483648;
pub const FLASH_SIZE: usize = 1048576;
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
    pub const CLK_SRC_CLK2_PLL1: usize = 40;
    pub const CLK_SRC_CLK3_PLL1: usize = 41;
    pub const CLK_SRC_PLL0_REF: usize = 42;
    pub const CLK_SRC_PLL1_REF: usize = 43;
    pub const CLK_TOP_CPU0: usize = 64;
    pub const CLK_TOP_MCT0: usize = 65;
    pub const CLK_TOP_CAN0: usize = 66;
    pub const CLK_TOP_CAN1: usize = 67;
    pub const CLK_TOP_CAN2: usize = 68;
    pub const CLK_TOP_CAN3: usize = 69;
    pub const CLK_TOP_TMR0: usize = 74;
    pub const CLK_TOP_TMR1: usize = 75;
    pub const CLK_TOP_TMR2: usize = 76;
    pub const CLK_TOP_TMR3: usize = 77;
    pub const CLK_TOP_I2C0: usize = 78;
    pub const CLK_TOP_I2C1: usize = 79;
    pub const CLK_TOP_I2C2: usize = 80;
    pub const CLK_TOP_I2C3: usize = 81;
    pub const CLK_TOP_SPI0: usize = 82;
    pub const CLK_TOP_SPI1: usize = 83;
    pub const CLK_TOP_SPI2: usize = 84;
    pub const CLK_TOP_SPI3: usize = 85;
    pub const CLK_TOP_URT0: usize = 86;
    pub const CLK_TOP_URT1: usize = 87;
    pub const CLK_TOP_URT2: usize = 88;
    pub const CLK_TOP_URT3: usize = 89;
    pub const CLK_TOP_URT4: usize = 90;
    pub const CLK_TOP_URT5: usize = 91;
    pub const CLK_TOP_URT6: usize = 92;
    pub const CLK_TOP_URT7: usize = 93;
    pub const CLK_TOP_XPI0: usize = 94;
    pub const CLK_TOP_ANA0: usize = 95;
    pub const CLK_TOP_ANA1: usize = 96;
    pub const CLK_TOP_ANA2: usize = 97;
    pub const CLK_TOP_ANA3: usize = 98;
    pub const CLK_TOP_REF0: usize = 99;
    pub const CLK_TOP_REF1: usize = 100;
    pub const CLK_TOP_ADC0: usize = 101;
    pub const CLK_TOP_ADC1: usize = 102;
    pub const CLK_TOP_DAC0: usize = 103;
    pub const CLK_TOP_DAC1: usize = 104;
    pub const AHB0: usize = 256;
    pub const LMM0: usize = 257;
    pub const MCT0: usize = 258;
    pub const ROM0: usize = 259;
    pub const CAN0: usize = 260;
    pub const CAN1: usize = 261;
    pub const CAN2: usize = 262;
    pub const CAN3: usize = 263;
    pub const PTPC: usize = 264;
    pub const TMR0: usize = 269;
    pub const TMR1: usize = 270;
    pub const TMR2: usize = 271;
    pub const TMR3: usize = 272;
    pub const I2C0: usize = 273;
    pub const I2C1: usize = 274;
    pub const I2C2: usize = 275;
    pub const I2C3: usize = 276;
    pub const SPI0: usize = 277;
    pub const SPI1: usize = 278;
    pub const SPI2: usize = 279;
    pub const SPI3: usize = 280;
    pub const URT0: usize = 281;
    pub const URT1: usize = 282;
    pub const URT2: usize = 283;
    pub const URT3: usize = 284;
    pub const URT4: usize = 285;
    pub const URT5: usize = 286;
    pub const URT6: usize = 287;
    pub const URT7: usize = 288;
    pub const WDG0: usize = 289;
    pub const WDG1: usize = 290;
    pub const MBX0: usize = 291;
    pub const TSNS: usize = 292;
    pub const CRC0: usize = 293;
    pub const ADC0: usize = 294;
    pub const ADC1: usize = 295;
    pub const DAC0: usize = 296;
    pub const DAC1: usize = 297;
    pub const ACMP: usize = 298;
    pub const OPA0: usize = 299;
    pub const OPA1: usize = 300;
    pub const MOT0: usize = 301;
    pub const RNG0: usize = 302;
    pub const SDP0: usize = 303;
    pub const KMAN: usize = 304;
    pub const GPIO: usize = 305;
    pub const HDMA: usize = 306;
    pub const XPI0: usize = 307;
    pub const USB0: usize = 308;
    pub const REF0: usize = 309;
    pub const REF1: usize = 310;
}
pub mod clocks {
    //! `SYSCTL.CLOCK` definitions
    pub const MCT0: usize = 0;
    pub const CAN0: usize = 1;
    pub const CAN1: usize = 2;
    pub const CAN2: usize = 3;
    pub const CAN3: usize = 4;
    pub const TMR0: usize = 9;
    pub const TMR1: usize = 10;
    pub const TMR2: usize = 11;
    pub const TMR3: usize = 12;
    pub const I2C0: usize = 13;
    pub const I2C1: usize = 14;
    pub const I2C2: usize = 15;
    pub const I2C3: usize = 16;
    pub const SPI0: usize = 17;
    pub const SPI1: usize = 18;
    pub const SPI2: usize = 19;
    pub const SPI3: usize = 20;
    pub const URT0: usize = 21;
    pub const URT1: usize = 22;
    pub const URT2: usize = 23;
    pub const URT3: usize = 24;
    pub const URT4: usize = 25;
    pub const URT5: usize = 26;
    pub const URT6: usize = 27;
    pub const URT7: usize = 28;
    pub const XPI0: usize = 29;
    pub const ANA0: usize = 30;
    pub const ANA1: usize = 31;
    pub const ANA2: usize = 32;
    pub const ANA3: usize = 33;
    pub const REF0: usize = 34;
    pub const REF1: usize = 35;
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
    pub const PF16: usize = 176;
    pub const PF17: usize = 177;
    pub const PF18: usize = 178;
    pub const PF19: usize = 179;
    pub const PF20: usize = 180;
    pub const PF21: usize = 181;
    pub const PF22: usize = 182;
    pub const PF23: usize = 183;
    pub const PF24: usize = 184;
    pub const PF25: usize = 185;
    pub const PF26: usize = 186;
    pub const PF27: usize = 187;
    pub const PF28: usize = 188;
    pub const PF29: usize = 189;
    pub const PF30: usize = 190;
    pub const PF31: usize = 191;
    pub const PV00: usize = 352;
    pub const PV01: usize = 353;
    pub const PV02: usize = 354;
    pub const PV03: usize = 355;
    pub const PV04: usize = 356;
    pub const PV05: usize = 357;
    pub const PV06: usize = 358;
    pub const PV07: usize = 359;
    pub const PV08: usize = 360;
    pub const PV09: usize = 361;
    pub const PV10: usize = 362;
    pub const PV11: usize = 363;
    pub const PV12: usize = 364;
    pub const PV15: usize = 367;
    pub const PW00: usize = 384;
    pub const PW01: usize = 385;
    pub const PW02: usize = 386;
    pub const PW03: usize = 387;
    pub const PW04: usize = 388;
    pub const PW05: usize = 389;
    pub const PW06: usize = 390;
    pub const PW07: usize = 391;
    pub const PW08: usize = 392;
    pub const PW09: usize = 393;
    pub const PW10: usize = 394;
    pub const PW11: usize = 395;
    pub const PW12: usize = 396;
    pub const PW15: usize = 399;
    pub const PW16: usize = 400;
    pub const PW17: usize = 401;
    pub const PW20: usize = 404;
    pub const PW21: usize = 405;
    pub const PX00: usize = 416;
    pub const PX01: usize = 417;
    pub const PX02: usize = 418;
    pub const PX03: usize = 419;
    pub const PX04: usize = 420;
    pub const PX05: usize = 421;
    pub const PX06: usize = 422;
    pub const PX07: usize = 423;
}
pub mod iomux {
    //! `FUNC_CTL` function mux definitions
    pub const IOC_PA00_FUNC_CTL_GPIO_A_00: u8 = 0;
    pub const IOC_PA00_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PA00_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PA00_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PA00_FUNC_CTL_SYSCTL_CLK_OBS_0: u8 = 24;
    pub const IOC_PA00_FUNC_CTL_TRGM_P_00: u8 = 17;
    pub const IOC_PA00_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PA01_FUNC_CTL_GPIO_A_01: u8 = 0;
    pub const IOC_PA01_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PA01_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PA01_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PA01_FUNC_CTL_SYSCTL_CLK_OBS_2: u8 = 24;
    pub const IOC_PA01_FUNC_CTL_TRGM_P_01: u8 = 17;
    pub const IOC_PA01_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PA02_FUNC_CTL_GPIO_A_02: u8 = 0;
    pub const IOC_PA02_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PA02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PA02_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PA02_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PA02_FUNC_CTL_SYSCTL_CLK_OBS_1: u8 = 24;
    pub const IOC_PA02_FUNC_CTL_TRGM_P_02: u8 = 17;
    pub const IOC_PA02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PA02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PA03_FUNC_CTL_GPIO_A_03: u8 = 0;
    pub const IOC_PA03_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PA03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PA03_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PA03_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PA03_FUNC_CTL_SYSCTL_CLK_OBS_3: u8 = 24;
    pub const IOC_PA03_FUNC_CTL_TRGM_P_03: u8 = 17;
    pub const IOC_PA03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PA04_FUNC_CTL_GPIO_A_04: u8 = 0;
    pub const IOC_PA04_FUNC_CTL_JTAG_TDO: u8 = 24;
    pub const IOC_PA04_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PA04_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PA04_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PA04_FUNC_CTL_TRGM_P_04: u8 = 17;
    pub const IOC_PA04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PA05_FUNC_CTL_GPIO_A_05: u8 = 0;
    pub const IOC_PA05_FUNC_CTL_GPTMR1_COMP_2: u8 = 1;
    pub const IOC_PA05_FUNC_CTL_JTAG_TDI: u8 = 24;
    pub const IOC_PA05_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PA05_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PA05_FUNC_CTL_SPI1_CS_0: u8 = 5;
    pub const IOC_PA05_FUNC_CTL_TRGM_P_05: u8 = 17;
    pub const IOC_PA05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PA05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PA06_FUNC_CTL_GPIO_A_06: u8 = 0;
    pub const IOC_PA06_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PA06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PA06_FUNC_CTL_JTAG_TCK: u8 = 24;
    pub const IOC_PA06_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PA06_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PA06_FUNC_CTL_TRGM_P_06: u8 = 17;
    pub const IOC_PA06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PA07_FUNC_CTL_GPIO_A_07: u8 = 0;
    pub const IOC_PA07_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PA07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PA07_FUNC_CTL_JTAG_TMS: u8 = 24;
    pub const IOC_PA07_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PA07_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PA07_FUNC_CTL_TRGM_P_07: u8 = 17;
    pub const IOC_PA07_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PA08_FUNC_CTL_GPIO_A_08: u8 = 0;
    pub const IOC_PA08_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PA08_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PA08_FUNC_CTL_JTAG_TRST: u8 = 24;
    pub const IOC_PA08_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PA08_FUNC_CTL_PWM1_P_0: u8 = 16;
    pub const IOC_PA08_FUNC_CTL_TRGM_P_08: u8 = 17;
    pub const IOC_PA08_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PA09_FUNC_CTL_ESC0_REFCK: u8 = 11;
    pub const IOC_PA09_FUNC_CTL_GPIO_A_09: u8 = 0;
    pub const IOC_PA09_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PA09_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PA09_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PA09_FUNC_CTL_PWM1_P_1: u8 = 16;
    pub const IOC_PA09_FUNC_CTL_TRGM_P_09: u8 = 17;
    pub const IOC_PA09_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PA16_FUNC_CTL_ESC0_P0_RXDV: u8 = 11;
    pub const IOC_PA16_FUNC_CTL_EUI0_CK: u8 = 26;
    pub const IOC_PA16_FUNC_CTL_GPIO_A_16: u8 = 0;
    pub const IOC_PA16_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PA16_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PA16_FUNC_CTL_SDM0_DAT_3: u8 = 23;
    pub const IOC_PA16_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PA17_FUNC_CTL_ESC0_P0_RXD_0: u8 = 11;
    pub const IOC_PA17_FUNC_CTL_EUI0_SH: u8 = 26;
    pub const IOC_PA17_FUNC_CTL_GPIO_A_17: u8 = 0;
    pub const IOC_PA17_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PA17_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PA17_FUNC_CTL_SDM0_CLK_3: u8 = 23;
    pub const IOC_PA17_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PA18_FUNC_CTL_ESC0_P0_RXD_1: u8 = 11;
    pub const IOC_PA18_FUNC_CTL_EUI0_DI: u8 = 26;
    pub const IOC_PA18_FUNC_CTL_GPIO_A_18: u8 = 0;
    pub const IOC_PA18_FUNC_CTL_GPTMR3_COMP_1: u8 = 1;
    pub const IOC_PA18_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PA18_FUNC_CTL_SDM0_DAT_2: u8 = 23;
    pub const IOC_PA18_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PA18_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PA19_FUNC_CTL_ESC0_P0_RXD_2: u8 = 11;
    pub const IOC_PA19_FUNC_CTL_EUI0_DO: u8 = 26;
    pub const IOC_PA19_FUNC_CTL_GPIO_A_19: u8 = 0;
    pub const IOC_PA19_FUNC_CTL_GPTMR3_CAPT_1: u8 = 1;
    pub const IOC_PA19_FUNC_CTL_SDM0_CLK_2: u8 = 23;
    pub const IOC_PA19_FUNC_CTL_SPI3_CS_3: u8 = 5;
    pub const IOC_PA19_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PA20_FUNC_CTL_ESC0_P0_RXD_3: u8 = 11;
    pub const IOC_PA20_FUNC_CTL_GPIO_A_20: u8 = 0;
    pub const IOC_PA20_FUNC_CTL_SDM0_DAT_1: u8 = 23;
    pub const IOC_PA20_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PA20_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PA21_FUNC_CTL_ESC0_P0_RXCK: u8 = 11;
    pub const IOC_PA21_FUNC_CTL_GPIO_A_21: u8 = 0;
    pub const IOC_PA21_FUNC_CTL_GPTMR3_COMP_2: u8 = 1;
    pub const IOC_PA21_FUNC_CTL_OWR0_DAT: u8 = 26;
    pub const IOC_PA21_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PA21_FUNC_CTL_SDM0_CLK_1: u8 = 23;
    pub const IOC_PA21_FUNC_CTL_SPI2_CS_0: u8 = 5;
    pub const IOC_PA21_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PA21_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PA22_FUNC_CTL_ETH0_TXER: u8 = 18;
    pub const IOC_PA22_FUNC_CTL_GPIO_A_22: u8 = 0;
    pub const IOC_PA22_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PA22_FUNC_CTL_OWR1_DAT: u8 = 26;
    pub const IOC_PA22_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PA22_FUNC_CTL_SDM0_DAT_0: u8 = 23;
    pub const IOC_PA22_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PA22_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PA23_FUNC_CTL_ESC0_P0_RXER: u8 = 11;
    pub const IOC_PA23_FUNC_CTL_ETH0_RXER: u8 = 18;
    pub const IOC_PA23_FUNC_CTL_GPIO_A_23: u8 = 0;
    pub const IOC_PA23_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PA23_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PA23_FUNC_CTL_SDM0_CLK_0: u8 = 23;
    pub const IOC_PA23_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PA23_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PA24_FUNC_CTL_ESC0_P0_TXCK: u8 = 11;
    pub const IOC_PA24_FUNC_CTL_GPIO_A_24: u8 = 0;
    pub const IOC_PA24_FUNC_CTL_GPTMR2_COMP_1: u8 = 1;
    pub const IOC_PA24_FUNC_CTL_SPI3_CS_2: u8 = 5;
    pub const IOC_PA24_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PA25_FUNC_CTL_ESC0_CTR_0: u8 = 18;
    pub const IOC_PA25_FUNC_CTL_ESC0_P0_TXD_0: u8 = 11;
    pub const IOC_PA25_FUNC_CTL_GPIO_A_25: u8 = 0;
    pub const IOC_PA25_FUNC_CTL_GPTMR2_CAPT_1: u8 = 1;
    pub const IOC_PA25_FUNC_CTL_SPI3_CS_1: u8 = 5;
    pub const IOC_PA25_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PA26_FUNC_CTL_ESC0_P0_TXD_1: u8 = 11;
    pub const IOC_PA26_FUNC_CTL_EWDG0_RST: u8 = 24;
    pub const IOC_PA26_FUNC_CTL_GPIO_A_26: u8 = 0;
    pub const IOC_PA26_FUNC_CTL_GPTMR2_COMP_2: u8 = 1;
    pub const IOC_PA26_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PA26_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PA26_FUNC_CTL_UART6_DE: u8 = 2;
    pub const IOC_PA26_FUNC_CTL_UART6_RTS: u8 = 3;
    pub const IOC_PA27_FUNC_CTL_ESC0_P0_TXD_2: u8 = 11;
    pub const IOC_PA27_FUNC_CTL_GPIO_A_27: u8 = 0;
    pub const IOC_PA27_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PA27_FUNC_CTL_SPI3_CS_0: u8 = 5;
    pub const IOC_PA27_FUNC_CTL_UART6_CTS: u8 = 3;
    pub const IOC_PA28_FUNC_CTL_ESC0_CTR_1: u8 = 18;
    pub const IOC_PA28_FUNC_CTL_ESC0_P0_TXD_3: u8 = 11;
    pub const IOC_PA28_FUNC_CTL_GPIO_A_28: u8 = 0;
    pub const IOC_PA28_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PA28_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PA28_FUNC_CTL_UART7_CTS: u8 = 3;
    pub const IOC_PA29_FUNC_CTL_ESC0_P0_TXEN: u8 = 11;
    pub const IOC_PA29_FUNC_CTL_GPIO_A_29: u8 = 0;
    pub const IOC_PA29_FUNC_CTL_GPTMR3_COMP_3: u8 = 1;
    pub const IOC_PA29_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PA29_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PA29_FUNC_CTL_UART7_DE: u8 = 2;
    pub const IOC_PA29_FUNC_CTL_UART7_RTS: u8 = 3;
    pub const IOC_PA30_FUNC_CTL_ESC0_MDIO: u8 = 11;
    pub const IOC_PA30_FUNC_CTL_ETH0_MDIO: u8 = 18;
    pub const IOC_PA30_FUNC_CTL_GPIO_A_30: u8 = 0;
    pub const IOC_PA30_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PA30_FUNC_CTL_SPI3_DAT2: u8 = 5;
    pub const IOC_PA30_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PA31_FUNC_CTL_ESC0_MDC: u8 = 11;
    pub const IOC_PA31_FUNC_CTL_ETH0_MDC: u8 = 18;
    pub const IOC_PA31_FUNC_CTL_GPIO_A_31: u8 = 0;
    pub const IOC_PA31_FUNC_CTL_GPTMR2_COMP_3: u8 = 1;
    pub const IOC_PA31_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PA31_FUNC_CTL_SPI3_DAT3: u8 = 5;
    pub const IOC_PA31_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PB00_FUNC_CTL_ESC0_P0_RXDV: u8 = 11;
    pub const IOC_PB00_FUNC_CTL_ETH0_RXDV: u8 = 18;
    pub const IOC_PB00_FUNC_CTL_GPIO_B_00: u8 = 0;
    pub const IOC_PB00_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PB00_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PB00_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PB00_FUNC_CTL_TRGM_P_00: u8 = 17;
    pub const IOC_PB01_FUNC_CTL_ESC0_P0_RXD_0: u8 = 11;
    pub const IOC_PB01_FUNC_CTL_ETH0_RXD_0: u8 = 18;
    pub const IOC_PB01_FUNC_CTL_GPIO_B_01: u8 = 0;
    pub const IOC_PB01_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PB01_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PB01_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PB01_FUNC_CTL_TRGM_P_01: u8 = 17;
    pub const IOC_PB02_FUNC_CTL_ESC0_P0_RXD_1: u8 = 11;
    pub const IOC_PB02_FUNC_CTL_ETH0_RXD_1: u8 = 18;
    pub const IOC_PB02_FUNC_CTL_GPIO_B_02: u8 = 0;
    pub const IOC_PB02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PB02_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PB02_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PB02_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PB02_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PB02_FUNC_CTL_TRGM_P_02: u8 = 17;
    pub const IOC_PB03_FUNC_CTL_ESC0_P0_RXD_2: u8 = 11;
    pub const IOC_PB03_FUNC_CTL_ETH0_RXD_2: u8 = 18;
    pub const IOC_PB03_FUNC_CTL_GPIO_B_03: u8 = 0;
    pub const IOC_PB03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PB03_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PB03_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PB03_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PB03_FUNC_CTL_TRGM_P_03: u8 = 17;
    pub const IOC_PB04_FUNC_CTL_ESC0_P0_RXD_3: u8 = 11;
    pub const IOC_PB04_FUNC_CTL_ETH0_RXD_3: u8 = 18;
    pub const IOC_PB04_FUNC_CTL_GPIO_B_04: u8 = 0;
    pub const IOC_PB04_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PB04_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PB04_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PB04_FUNC_CTL_TRGM_P_04: u8 = 17;
    pub const IOC_PB05_FUNC_CTL_ESC0_P0_RXCK: u8 = 11;
    pub const IOC_PB05_FUNC_CTL_ETH0_RXCK: u8 = 18;
    pub const IOC_PB05_FUNC_CTL_GPIO_B_05: u8 = 0;
    pub const IOC_PB05_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PB05_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PB05_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PB05_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PB05_FUNC_CTL_TRGM_P_05: u8 = 17;
    pub const IOC_PB06_FUNC_CTL_ESC0_P0_TXCK: u8 = 11;
    pub const IOC_PB06_FUNC_CTL_ETH0_TXCK: u8 = 18;
    pub const IOC_PB06_FUNC_CTL_GPIO_B_06: u8 = 0;
    pub const IOC_PB06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PB06_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PB06_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PB06_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PB06_FUNC_CTL_TRGM_P_06: u8 = 17;
    pub const IOC_PB07_FUNC_CTL_ESC0_P0_TXD_0: u8 = 11;
    pub const IOC_PB07_FUNC_CTL_ETH0_TXD_0: u8 = 18;
    pub const IOC_PB07_FUNC_CTL_GPIO_B_07: u8 = 0;
    pub const IOC_PB07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PB07_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PB07_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PB07_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PB07_FUNC_CTL_TRGM_P_07: u8 = 17;
    pub const IOC_PB08_FUNC_CTL_ESC0_P0_TXD_1: u8 = 11;
    pub const IOC_PB08_FUNC_CTL_ETH0_TXD_1: u8 = 18;
    pub const IOC_PB08_FUNC_CTL_GPIO_B_08: u8 = 0;
    pub const IOC_PB08_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PB08_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PB08_FUNC_CTL_PWM1_P_0: u8 = 16;
    pub const IOC_PB08_FUNC_CTL_TRGM_P_08: u8 = 17;
    pub const IOC_PB09_FUNC_CTL_ESC0_P0_TXD_2: u8 = 11;
    pub const IOC_PB09_FUNC_CTL_ETH0_TXD_2: u8 = 18;
    pub const IOC_PB09_FUNC_CTL_GPIO_B_09: u8 = 0;
    pub const IOC_PB09_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PB09_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PB09_FUNC_CTL_PWM1_P_1: u8 = 16;
    pub const IOC_PB09_FUNC_CTL_TRGM_P_09: u8 = 17;
    pub const IOC_PB10_FUNC_CTL_ESC0_P0_TXD_3: u8 = 11;
    pub const IOC_PB10_FUNC_CTL_ETH0_TXD_3: u8 = 18;
    pub const IOC_PB10_FUNC_CTL_GPIO_B_10: u8 = 0;
    pub const IOC_PB10_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PB10_FUNC_CTL_PWM1_P_2: u8 = 16;
    pub const IOC_PB10_FUNC_CTL_TRGM_P_10: u8 = 17;
    pub const IOC_PB11_FUNC_CTL_ESC0_P0_TXEN: u8 = 11;
    pub const IOC_PB11_FUNC_CTL_ETH0_TXEN: u8 = 18;
    pub const IOC_PB11_FUNC_CTL_GPIO_B_11: u8 = 0;
    pub const IOC_PB11_FUNC_CTL_PWM1_P_3: u8 = 16;
    pub const IOC_PB11_FUNC_CTL_TRGM_P_11: u8 = 17;
    pub const IOC_PB24_FUNC_CTL_ESC0_CTR_4: u8 = 11;
    pub const IOC_PB24_FUNC_CTL_ESC0_EVTI_0: u8 = 27;
    pub const IOC_PB24_FUNC_CTL_ETH0_EVTI_0: u8 = 25;
    pub const IOC_PB24_FUNC_CTL_GPIO_B_24: u8 = 0;
    pub const IOC_PB24_FUNC_CTL_SDM0_CLK_3: u8 = 23;
    pub const IOC_PB24_FUNC_CTL_XPI0_CA_CS1: u8 = 14;
    pub const IOC_PB25_FUNC_CTL_ESC0_CTR_5: u8 = 11;
    pub const IOC_PB25_FUNC_CTL_ESC0_EVTO_0: u8 = 27;
    pub const IOC_PB25_FUNC_CTL_ETH0_EVTO_0: u8 = 25;
    pub const IOC_PB25_FUNC_CTL_GPIO_B_25: u8 = 0;
    pub const IOC_PB25_FUNC_CTL_SDM0_DAT_3: u8 = 23;
    pub const IOC_PB25_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PB26_FUNC_CTL_ESC0_CTR_6: u8 = 11;
    pub const IOC_PB26_FUNC_CTL_ESC0_EVTI_1: u8 = 27;
    pub const IOC_PB26_FUNC_CTL_ETH0_EVTI_1: u8 = 25;
    pub const IOC_PB26_FUNC_CTL_EUI1_CK: u8 = 26;
    pub const IOC_PB26_FUNC_CTL_GPIO_B_26: u8 = 0;
    pub const IOC_PB26_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PB26_FUNC_CTL_SDM0_CLK_2: u8 = 23;
    pub const IOC_PB26_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
    pub const IOC_PB27_FUNC_CTL_ESC0_CTR_7: u8 = 11;
    pub const IOC_PB27_FUNC_CTL_EUI1_SH: u8 = 26;
    pub const IOC_PB27_FUNC_CTL_GPIO_B_27: u8 = 0;
    pub const IOC_PB27_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PB27_FUNC_CTL_SDM0_DAT_2: u8 = 23;
    pub const IOC_PB27_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PB28_FUNC_CTL_ESC0_CTR_8: u8 = 11;
    pub const IOC_PB28_FUNC_CTL_EUI1_DI: u8 = 26;
    pub const IOC_PB28_FUNC_CTL_GPIO_B_28: u8 = 0;
    pub const IOC_PB28_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PB28_FUNC_CTL_SDM0_CLK_1: u8 = 23;
    pub const IOC_PB28_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PB29_FUNC_CTL_ESC0_EVTO_1: u8 = 27;
    pub const IOC_PB29_FUNC_CTL_ETH0_EVTO_1: u8 = 25;
    pub const IOC_PB29_FUNC_CTL_EUI1_DO: u8 = 26;
    pub const IOC_PB29_FUNC_CTL_GPIO_B_29: u8 = 0;
    pub const IOC_PB29_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PB29_FUNC_CTL_SDM0_DAT_1: u8 = 23;
    pub const IOC_PB29_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PB30_FUNC_CTL_ESC0_CTR_7: u8 = 11;
    pub const IOC_PB30_FUNC_CTL_GPIO_B_30: u8 = 0;
    pub const IOC_PB30_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PB30_FUNC_CTL_SDM0_CLK_0: u8 = 23;
    pub const IOC_PB30_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PB31_FUNC_CTL_ESC0_CTR_8: u8 = 11;
    pub const IOC_PB31_FUNC_CTL_GPIO_B_31: u8 = 0;
    pub const IOC_PB31_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PB31_FUNC_CTL_SDM0_DAT_0: u8 = 23;
    pub const IOC_PB31_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PC00_FUNC_CTL_ESC0_GPI_15: u8 = 18;
    pub const IOC_PC00_FUNC_CTL_ESC0_GPI_63: u8 = 10;
    pub const IOC_PC00_FUNC_CTL_ESC0_GPO_00: u8 = 11;
    pub const IOC_PC00_FUNC_CTL_GPIO_C_00: u8 = 0;
    pub const IOC_PC00_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PC00_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PC00_FUNC_CTL_OWR0_DAT: u8 = 26;
    pub const IOC_PC00_FUNC_CTL_PPI0_DQ_24: u8 = 13;
    pub const IOC_PC00_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PC00_FUNC_CTL_TRGM_P_00: u8 = 17;
    pub const IOC_PC00_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PC00_FUNC_CTL_XPI0_CB_CS0: u8 = 14;
    pub const IOC_PC01_FUNC_CTL_ESC0_GPI_14: u8 = 18;
    pub const IOC_PC01_FUNC_CTL_ESC0_GPI_62: u8 = 10;
    pub const IOC_PC01_FUNC_CTL_ESC0_GPO_01: u8 = 11;
    pub const IOC_PC01_FUNC_CTL_GPIO_C_01: u8 = 0;
    pub const IOC_PC01_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PC01_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PC01_FUNC_CTL_OWR1_DAT: u8 = 26;
    pub const IOC_PC01_FUNC_CTL_PPI0_DQ_25: u8 = 13;
    pub const IOC_PC01_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PC01_FUNC_CTL_TRGM_P_01: u8 = 17;
    pub const IOC_PC01_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PC01_FUNC_CTL_XPI0_CB_D_1: u8 = 14;
    pub const IOC_PC02_FUNC_CTL_ESC0_GPI_13: u8 = 18;
    pub const IOC_PC02_FUNC_CTL_ESC0_GPI_61: u8 = 10;
    pub const IOC_PC02_FUNC_CTL_ESC0_GPO_02: u8 = 11;
    pub const IOC_PC02_FUNC_CTL_GPIO_C_02: u8 = 0;
    pub const IOC_PC02_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PC02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PC02_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PC02_FUNC_CTL_PPI0_DQ_22: u8 = 13;
    pub const IOC_PC02_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PC02_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PC02_FUNC_CTL_TRGM_P_02: u8 = 17;
    pub const IOC_PC02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PC02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PC02_FUNC_CTL_XPI0_CB_D_2: u8 = 14;
    pub const IOC_PC03_FUNC_CTL_ESC0_GPI_12: u8 = 18;
    pub const IOC_PC03_FUNC_CTL_ESC0_GPI_60: u8 = 10;
    pub const IOC_PC03_FUNC_CTL_ESC0_GPO_03: u8 = 11;
    pub const IOC_PC03_FUNC_CTL_GPIO_C_03: u8 = 0;
    pub const IOC_PC03_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PC03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PC03_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PC03_FUNC_CTL_PPI0_DQ_26: u8 = 13;
    pub const IOC_PC03_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PC03_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PC03_FUNC_CTL_SPI1_CS_3: u8 = 5;
    pub const IOC_PC03_FUNC_CTL_TRGM_P_03: u8 = 17;
    pub const IOC_PC03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PC03_FUNC_CTL_XPI0_CB_D_3: u8 = 14;
    pub const IOC_PC04_FUNC_CTL_ESC0_GPI_11: u8 = 18;
    pub const IOC_PC04_FUNC_CTL_ESC0_GPI_59: u8 = 10;
    pub const IOC_PC04_FUNC_CTL_ESC0_GPO_04: u8 = 11;
    pub const IOC_PC04_FUNC_CTL_GPIO_C_04: u8 = 0;
    pub const IOC_PC04_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PC04_FUNC_CTL_PPI0_DQ_27: u8 = 13;
    pub const IOC_PC04_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PC04_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PC04_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PC04_FUNC_CTL_TRGM_P_04: u8 = 17;
    pub const IOC_PC04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PC04_FUNC_CTL_XPI0_CB_SCLK: u8 = 14;
    pub const IOC_PC05_FUNC_CTL_ESC0_GPI_10: u8 = 18;
    pub const IOC_PC05_FUNC_CTL_ESC0_GPI_58: u8 = 10;
    pub const IOC_PC05_FUNC_CTL_ESC0_GPO_05: u8 = 11;
    pub const IOC_PC05_FUNC_CTL_GPIO_C_05: u8 = 0;
    pub const IOC_PC05_FUNC_CTL_GPTMR1_COMP_2: u8 = 1;
    pub const IOC_PC05_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PC05_FUNC_CTL_PPI0_DQ_28: u8 = 13;
    pub const IOC_PC05_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PC05_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PC05_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PC05_FUNC_CTL_SPI0_CS_0: u8 = 5;
    pub const IOC_PC05_FUNC_CTL_TRGM_P_05: u8 = 17;
    pub const IOC_PC05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PC05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PC05_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PC06_FUNC_CTL_ESC0_GPI_09: u8 = 18;
    pub const IOC_PC06_FUNC_CTL_ESC0_GPI_57: u8 = 10;
    pub const IOC_PC06_FUNC_CTL_ESC0_GPO_06: u8 = 11;
    pub const IOC_PC06_FUNC_CTL_GPIO_C_06: u8 = 0;
    pub const IOC_PC06_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PC06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PC06_FUNC_CTL_PPI0_DQ_29: u8 = 13;
    pub const IOC_PC06_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PC06_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PC06_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PC06_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PC06_FUNC_CTL_TRGM_P_06: u8 = 17;
    pub const IOC_PC06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PC06_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PC07_FUNC_CTL_ESC0_GPI_08: u8 = 18;
    pub const IOC_PC07_FUNC_CTL_ESC0_GPI_56: u8 = 10;
    pub const IOC_PC07_FUNC_CTL_ESC0_GPO_07: u8 = 11;
    pub const IOC_PC07_FUNC_CTL_GPIO_C_07: u8 = 0;
    pub const IOC_PC07_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PC07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PC07_FUNC_CTL_PPI0_DQ_30: u8 = 13;
    pub const IOC_PC07_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PC07_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PC07_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PC07_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PC07_FUNC_CTL_TRGM_P_07: u8 = 17;
    pub const IOC_PC07_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PC07_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PC08_FUNC_CTL_ESC0_GPI_07: u8 = 18;
    pub const IOC_PC08_FUNC_CTL_ESC0_GPI_55: u8 = 10;
    pub const IOC_PC08_FUNC_CTL_ESC0_GPO_08: u8 = 11;
    pub const IOC_PC08_FUNC_CTL_GPIO_C_08: u8 = 0;
    pub const IOC_PC08_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PC08_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PC08_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PC08_FUNC_CTL_PPI0_DQ_31: u8 = 13;
    pub const IOC_PC08_FUNC_CTL_PWM1_P_0: u8 = 16;
    pub const IOC_PC08_FUNC_CTL_SPI1_CS_2: u8 = 5;
    pub const IOC_PC08_FUNC_CTL_TRGM_P_08: u8 = 17;
    pub const IOC_PC08_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PC08_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PC09_FUNC_CTL_ESC0_CTR_0: u8 = 27;
    pub const IOC_PC09_FUNC_CTL_ESC0_GPI_06: u8 = 18;
    pub const IOC_PC09_FUNC_CTL_ESC0_GPI_54: u8 = 10;
    pub const IOC_PC09_FUNC_CTL_ESC0_GPO_09: u8 = 11;
    pub const IOC_PC09_FUNC_CTL_GPIO_C_09: u8 = 0;
    pub const IOC_PC09_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PC09_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PC09_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PC09_FUNC_CTL_PPI0_DQ_23: u8 = 13;
    pub const IOC_PC09_FUNC_CTL_PWM1_P_1: u8 = 16;
    pub const IOC_PC09_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PC09_FUNC_CTL_SPI1_CS_1: u8 = 5;
    pub const IOC_PC09_FUNC_CTL_TRGM_P_09: u8 = 17;
    pub const IOC_PC09_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PC09_FUNC_CTL_XPI0_CB_CS1: u8 = 14;
    pub const IOC_PC10_FUNC_CTL_ESC0_CTR_1: u8 = 27;
    pub const IOC_PC10_FUNC_CTL_ESC0_GPI_05: u8 = 18;
    pub const IOC_PC10_FUNC_CTL_ESC0_GPI_53: u8 = 10;
    pub const IOC_PC10_FUNC_CTL_ESC0_GPO_10: u8 = 11;
    pub const IOC_PC10_FUNC_CTL_GPIO_C_10: u8 = 0;
    pub const IOC_PC10_FUNC_CTL_GPTMR0_COMP_2: u8 = 1;
    pub const IOC_PC10_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PC10_FUNC_CTL_PPI0_DQ_20: u8 = 13;
    pub const IOC_PC10_FUNC_CTL_PWM1_P_2: u8 = 16;
    pub const IOC_PC10_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PC10_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PC10_FUNC_CTL_TRGM_P_10: u8 = 17;
    pub const IOC_PC10_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PC10_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PC10_FUNC_CTL_XPI0_CB_D_0: u8 = 14;
    pub const IOC_PC11_FUNC_CTL_ESC0_CTR_2: u8 = 27;
    pub const IOC_PC11_FUNC_CTL_ESC0_GPI_04: u8 = 18;
    pub const IOC_PC11_FUNC_CTL_ESC0_GPI_52: u8 = 10;
    pub const IOC_PC11_FUNC_CTL_ESC0_GPO_11: u8 = 11;
    pub const IOC_PC11_FUNC_CTL_GPIO_C_11: u8 = 0;
    pub const IOC_PC11_FUNC_CTL_PPI0_DQ_21: u8 = 13;
    pub const IOC_PC11_FUNC_CTL_PWM1_P_3: u8 = 16;
    pub const IOC_PC11_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PC11_FUNC_CTL_SPI1_CS_0: u8 = 5;
    pub const IOC_PC11_FUNC_CTL_TRGM_P_11: u8 = 17;
    pub const IOC_PC11_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PC11_FUNC_CTL_XPI0_CB_DQS: u8 = 14;
    pub const IOC_PC12_FUNC_CTL_ESC0_CTR_3: u8 = 27;
    pub const IOC_PC12_FUNC_CTL_ESC0_GPI_03: u8 = 18;
    pub const IOC_PC12_FUNC_CTL_ESC0_GPI_51: u8 = 10;
    pub const IOC_PC12_FUNC_CTL_ESC0_GPO_12: u8 = 11;
    pub const IOC_PC12_FUNC_CTL_GPIO_C_12: u8 = 0;
    pub const IOC_PC12_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PC12_FUNC_CTL_PPI0_DM_2: u8 = 13;
    pub const IOC_PC12_FUNC_CTL_PWM1_P_4: u8 = 16;
    pub const IOC_PC12_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PC12_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PC12_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PC12_FUNC_CTL_TRGM_P_12: u8 = 17;
    pub const IOC_PC12_FUNC_CTL_UART3_CTS: u8 = 3;
    pub const IOC_PC12_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
    pub const IOC_PC13_FUNC_CTL_ESC0_CTR_4: u8 = 27;
    pub const IOC_PC13_FUNC_CTL_ESC0_GPI_02: u8 = 18;
    pub const IOC_PC13_FUNC_CTL_ESC0_GPI_50: u8 = 10;
    pub const IOC_PC13_FUNC_CTL_ESC0_GPO_13: u8 = 11;
    pub const IOC_PC13_FUNC_CTL_GPIO_C_13: u8 = 0;
    pub const IOC_PC13_FUNC_CTL_GPTMR1_COMP_3: u8 = 1;
    pub const IOC_PC13_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PC13_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PC13_FUNC_CTL_PPI0_DQ_18: u8 = 13;
    pub const IOC_PC13_FUNC_CTL_PWM1_P_5: u8 = 16;
    pub const IOC_PC13_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PC13_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PC13_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PC13_FUNC_CTL_TRGM_P_13: u8 = 17;
    pub const IOC_PC13_FUNC_CTL_UART3_DE: u8 = 2;
    pub const IOC_PC13_FUNC_CTL_UART3_RTS: u8 = 3;
    pub const IOC_PC13_FUNC_CTL_XPI0_CA_CS1: u8 = 14;
    pub const IOC_PC14_FUNC_CTL_ESC0_CTR_5: u8 = 27;
    pub const IOC_PC14_FUNC_CTL_ESC0_GPI_01: u8 = 18;
    pub const IOC_PC14_FUNC_CTL_ESC0_GPI_49: u8 = 10;
    pub const IOC_PC14_FUNC_CTL_ESC0_GPO_14: u8 = 11;
    pub const IOC_PC14_FUNC_CTL_GPIO_C_14: u8 = 0;
    pub const IOC_PC14_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PC14_FUNC_CTL_PPI0_DQ_19: u8 = 13;
    pub const IOC_PC14_FUNC_CTL_PWM1_P_6: u8 = 16;
    pub const IOC_PC14_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PC14_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PC14_FUNC_CTL_SPI1_DAT2: u8 = 5;
    pub const IOC_PC14_FUNC_CTL_TRGM_P_14: u8 = 17;
    pub const IOC_PC14_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PC15_FUNC_CTL_ESC0_CTR_6: u8 = 27;
    pub const IOC_PC15_FUNC_CTL_ESC0_GPI_00: u8 = 18;
    pub const IOC_PC15_FUNC_CTL_ESC0_GPI_48: u8 = 10;
    pub const IOC_PC15_FUNC_CTL_ESC0_GPO_15: u8 = 11;
    pub const IOC_PC15_FUNC_CTL_GPIO_C_15: u8 = 0;
    pub const IOC_PC15_FUNC_CTL_GPTMR0_COMP_3: u8 = 1;
    pub const IOC_PC15_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PC15_FUNC_CTL_PPI0_DM_3: u8 = 13;
    pub const IOC_PC15_FUNC_CTL_PWM1_P_7: u8 = 16;
    pub const IOC_PC15_FUNC_CTL_SPI1_DAT3: u8 = 5;
    pub const IOC_PC15_FUNC_CTL_TRGM_P_15: u8 = 17;
    pub const IOC_PC15_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PC15_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PC16_FUNC_CTL_ESC0_CTR_7: u8 = 27;
    pub const IOC_PC16_FUNC_CTL_ESC0_GPI_47: u8 = 10;
    pub const IOC_PC16_FUNC_CTL_ESC0_GPO_16: u8 = 11;
    pub const IOC_PC16_FUNC_CTL_GPIO_C_16: u8 = 0;
    pub const IOC_PC16_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PC16_FUNC_CTL_PPI0_DQ_16: u8 = 13;
    pub const IOC_PC16_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PC16_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PC17_FUNC_CTL_ESC0_CTR_8: u8 = 27;
    pub const IOC_PC17_FUNC_CTL_ESC0_GPI_46: u8 = 10;
    pub const IOC_PC17_FUNC_CTL_ESC0_GPO_17: u8 = 11;
    pub const IOC_PC17_FUNC_CTL_GPIO_C_17: u8 = 0;
    pub const IOC_PC17_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PC17_FUNC_CTL_PPI0_DQ_17: u8 = 13;
    pub const IOC_PC17_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PC18_FUNC_CTL_ESC0_CTR_0: u8 = 18;
    pub const IOC_PC18_FUNC_CTL_ESC0_GPI_45: u8 = 10;
    pub const IOC_PC18_FUNC_CTL_ESC0_GPO_18: u8 = 11;
    pub const IOC_PC18_FUNC_CTL_GPIO_C_18: u8 = 0;
    pub const IOC_PC18_FUNC_CTL_GPTMR3_COMP_1: u8 = 1;
    pub const IOC_PC18_FUNC_CTL_PPI0_DQ_04: u8 = 13;
    pub const IOC_PC18_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PC18_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PC19_FUNC_CTL_ESC0_CTR_1: u8 = 18;
    pub const IOC_PC19_FUNC_CTL_ESC0_GPI_44: u8 = 10;
    pub const IOC_PC19_FUNC_CTL_ESC0_GPO_19: u8 = 11;
    pub const IOC_PC19_FUNC_CTL_GPIO_C_19: u8 = 0;
    pub const IOC_PC19_FUNC_CTL_GPTMR3_CAPT_1: u8 = 1;
    pub const IOC_PC19_FUNC_CTL_PPI0_DQ_05: u8 = 13;
    pub const IOC_PC19_FUNC_CTL_SPI2_CS_3: u8 = 5;
    pub const IOC_PC19_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PC20_FUNC_CTL_ESC0_CTR_2: u8 = 18;
    pub const IOC_PC20_FUNC_CTL_ESC0_GPI_43: u8 = 10;
    pub const IOC_PC20_FUNC_CTL_ESC0_GPO_20: u8 = 11;
    pub const IOC_PC20_FUNC_CTL_GPIO_C_20: u8 = 0;
    pub const IOC_PC20_FUNC_CTL_PPI0_DQ_06: u8 = 13;
    pub const IOC_PC20_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PC20_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PC21_FUNC_CTL_ESC0_CTR_3: u8 = 18;
    pub const IOC_PC21_FUNC_CTL_ESC0_GPI_42: u8 = 10;
    pub const IOC_PC21_FUNC_CTL_ESC0_GPO_21: u8 = 11;
    pub const IOC_PC21_FUNC_CTL_GPIO_C_21: u8 = 0;
    pub const IOC_PC21_FUNC_CTL_GPTMR3_COMP_2: u8 = 1;
    pub const IOC_PC21_FUNC_CTL_PPI0_DQ_07: u8 = 13;
    pub const IOC_PC21_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PC21_FUNC_CTL_SPI3_CS_0: u8 = 5;
    pub const IOC_PC21_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PC21_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PC22_FUNC_CTL_ESC0_GPI_41: u8 = 10;
    pub const IOC_PC22_FUNC_CTL_ESC0_GPO_22: u8 = 11;
    pub const IOC_PC22_FUNC_CTL_GPIO_C_22: u8 = 0;
    pub const IOC_PC22_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PC22_FUNC_CTL_PPI0_CTR_2: u8 = 13;
    pub const IOC_PC22_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PC22_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PC22_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PC23_FUNC_CTL_ESC0_GPI_40: u8 = 10;
    pub const IOC_PC23_FUNC_CTL_ESC0_GPO_23: u8 = 11;
    pub const IOC_PC23_FUNC_CTL_GPIO_C_23: u8 = 0;
    pub const IOC_PC23_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PC23_FUNC_CTL_PPI0_DQ_08: u8 = 13;
    pub const IOC_PC23_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PC23_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PC23_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PC24_FUNC_CTL_ESC0_GPI_39: u8 = 10;
    pub const IOC_PC24_FUNC_CTL_ESC0_GPO_24: u8 = 11;
    pub const IOC_PC24_FUNC_CTL_GPIO_C_24: u8 = 0;
    pub const IOC_PC24_FUNC_CTL_GPTMR2_COMP_1: u8 = 1;
    pub const IOC_PC24_FUNC_CTL_PPI0_DQ_09: u8 = 13;
    pub const IOC_PC24_FUNC_CTL_SPI2_CS_2: u8 = 5;
    pub const IOC_PC24_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PC25_FUNC_CTL_ESC0_GPI_38: u8 = 10;
    pub const IOC_PC25_FUNC_CTL_ESC0_GPO_25: u8 = 11;
    pub const IOC_PC25_FUNC_CTL_GPIO_C_25: u8 = 0;
    pub const IOC_PC25_FUNC_CTL_GPTMR2_CAPT_1: u8 = 1;
    pub const IOC_PC25_FUNC_CTL_PPI0_DQ_11: u8 = 13;
    pub const IOC_PC25_FUNC_CTL_SPI2_CS_1: u8 = 5;
    pub const IOC_PC25_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PC26_FUNC_CTL_ESC0_GPI_37: u8 = 10;
    pub const IOC_PC26_FUNC_CTL_ESC0_GPO_26: u8 = 11;
    pub const IOC_PC26_FUNC_CTL_GPIO_C_26: u8 = 0;
    pub const IOC_PC26_FUNC_CTL_GPTMR2_COMP_2: u8 = 1;
    pub const IOC_PC26_FUNC_CTL_PPI0_DQ_12: u8 = 13;
    pub const IOC_PC26_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PC26_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PC26_FUNC_CTL_UART6_DE: u8 = 2;
    pub const IOC_PC26_FUNC_CTL_UART6_RTS: u8 = 3;
    pub const IOC_PC27_FUNC_CTL_ESC0_CTR_4: u8 = 18;
    pub const IOC_PC27_FUNC_CTL_ESC0_GPI_36: u8 = 10;
    pub const IOC_PC27_FUNC_CTL_ESC0_GPO_27: u8 = 11;
    pub const IOC_PC27_FUNC_CTL_GPIO_C_27: u8 = 0;
    pub const IOC_PC27_FUNC_CTL_PPI0_CTR_0: u8 = 13;
    pub const IOC_PC27_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PC27_FUNC_CTL_SPI2_CS_0: u8 = 5;
    pub const IOC_PC27_FUNC_CTL_UART6_CTS: u8 = 3;
    pub const IOC_PC28_FUNC_CTL_ESC0_CTR_5: u8 = 18;
    pub const IOC_PC28_FUNC_CTL_ESC0_GPI_35: u8 = 10;
    pub const IOC_PC28_FUNC_CTL_ESC0_GPO_28: u8 = 11;
    pub const IOC_PC28_FUNC_CTL_GPIO_C_28: u8 = 0;
    pub const IOC_PC28_FUNC_CTL_PPI0_DM_0: u8 = 13;
    pub const IOC_PC28_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PC28_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PC28_FUNC_CTL_UART7_CTS: u8 = 3;
    pub const IOC_PC29_FUNC_CTL_ESC0_CTR_6: u8 = 18;
    pub const IOC_PC29_FUNC_CTL_ESC0_GPI_34: u8 = 10;
    pub const IOC_PC29_FUNC_CTL_ESC0_GPO_29: u8 = 11;
    pub const IOC_PC29_FUNC_CTL_GPIO_C_29: u8 = 0;
    pub const IOC_PC29_FUNC_CTL_GPTMR3_COMP_3: u8 = 1;
    pub const IOC_PC29_FUNC_CTL_PPI0_CLK: u8 = 13;
    pub const IOC_PC29_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PC29_FUNC_CTL_SOC_REF0: u8 = 24;
    pub const IOC_PC29_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PC29_FUNC_CTL_UART7_DE: u8 = 2;
    pub const IOC_PC29_FUNC_CTL_UART7_RTS: u8 = 3;
    pub const IOC_PC30_FUNC_CTL_ESC0_CTR_7: u8 = 18;
    pub const IOC_PC30_FUNC_CTL_ESC0_GPI_33: u8 = 10;
    pub const IOC_PC30_FUNC_CTL_ESC0_GPO_30: u8 = 11;
    pub const IOC_PC30_FUNC_CTL_GPIO_C_30: u8 = 0;
    pub const IOC_PC30_FUNC_CTL_PPI0_CS_0: u8 = 13;
    pub const IOC_PC30_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PC30_FUNC_CTL_SOC_REF1: u8 = 24;
    pub const IOC_PC30_FUNC_CTL_SPI2_DAT2: u8 = 5;
    pub const IOC_PC30_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PC31_FUNC_CTL_ESC0_CTR_8: u8 = 18;
    pub const IOC_PC31_FUNC_CTL_ESC0_GPI_32: u8 = 10;
    pub const IOC_PC31_FUNC_CTL_ESC0_GPO_31: u8 = 11;
    pub const IOC_PC31_FUNC_CTL_GPIO_C_31: u8 = 0;
    pub const IOC_PC31_FUNC_CTL_GPTMR2_COMP_3: u8 = 1;
    pub const IOC_PC31_FUNC_CTL_PPI0_CS_1: u8 = 13;
    pub const IOC_PC31_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PC31_FUNC_CTL_SPI2_DAT3: u8 = 5;
    pub const IOC_PC31_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PD00_FUNC_CTL_ESC0_CTR_0: u8 = 27;
    pub const IOC_PD00_FUNC_CTL_ESC0_GPI_31: u8 = 10;
    pub const IOC_PD00_FUNC_CTL_ESC0_GPO_32: u8 = 11;
    pub const IOC_PD00_FUNC_CTL_ESC0_REFCK: u8 = 18;
    pub const IOC_PD00_FUNC_CTL_GPIO_D_00: u8 = 0;
    pub const IOC_PD00_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PD00_FUNC_CTL_PPI0_DQ_02: u8 = 13;
    pub const IOC_PD00_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PD00_FUNC_CTL_TRGM_P_00: u8 = 17;
    pub const IOC_PD01_FUNC_CTL_ESC0_CTR_1: u8 = 27;
    pub const IOC_PD01_FUNC_CTL_ESC0_GPI_30: u8 = 10;
    pub const IOC_PD01_FUNC_CTL_ESC0_GPO_33: u8 = 11;
    pub const IOC_PD01_FUNC_CTL_ESC0_REFCK: u8 = 18;
    pub const IOC_PD01_FUNC_CTL_GPIO_D_01: u8 = 0;
    pub const IOC_PD01_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PD01_FUNC_CTL_PPI0_DQ_03: u8 = 13;
    pub const IOC_PD01_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PD01_FUNC_CTL_TRGM_P_01: u8 = 17;
    pub const IOC_PD02_FUNC_CTL_ESC0_CTR_2: u8 = 27;
    pub const IOC_PD02_FUNC_CTL_ESC0_GPI_29: u8 = 10;
    pub const IOC_PD02_FUNC_CTL_ESC0_GPO_34: u8 = 11;
    pub const IOC_PD02_FUNC_CTL_ESC0_SCL: u8 = 18;
    pub const IOC_PD02_FUNC_CTL_EWDG1_RST: u8 = 24;
    pub const IOC_PD02_FUNC_CTL_GPIO_D_02: u8 = 0;
    pub const IOC_PD02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PD02_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PD02_FUNC_CTL_PPI0_DQ_00: u8 = 13;
    pub const IOC_PD02_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PD02_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PD02_FUNC_CTL_TRGM_P_02: u8 = 17;
    pub const IOC_PD03_FUNC_CTL_ESC0_CTR_3: u8 = 27;
    pub const IOC_PD03_FUNC_CTL_ESC0_GPI_28: u8 = 10;
    pub const IOC_PD03_FUNC_CTL_ESC0_GPO_35: u8 = 11;
    pub const IOC_PD03_FUNC_CTL_ESC0_SDA: u8 = 18;
    pub const IOC_PD03_FUNC_CTL_GPIO_D_03: u8 = 0;
    pub const IOC_PD03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PD03_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PD03_FUNC_CTL_PPI0_DQ_01: u8 = 13;
    pub const IOC_PD03_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PD03_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PD03_FUNC_CTL_TRGM_P_03: u8 = 17;
    pub const IOC_PD04_FUNC_CTL_ESC0_CTR_4: u8 = 27;
    pub const IOC_PD04_FUNC_CTL_ESC0_GPI_27: u8 = 10;
    pub const IOC_PD04_FUNC_CTL_ESC0_GPO_36: u8 = 11;
    pub const IOC_PD04_FUNC_CTL_GPIO_D_04: u8 = 0;
    pub const IOC_PD04_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PD04_FUNC_CTL_PPI0_DQ_10: u8 = 13;
    pub const IOC_PD04_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PD04_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PD04_FUNC_CTL_TRGM_P_04: u8 = 17;
    pub const IOC_PD05_FUNC_CTL_ESC0_CTR_5: u8 = 27;
    pub const IOC_PD05_FUNC_CTL_ESC0_GPI_26: u8 = 10;
    pub const IOC_PD05_FUNC_CTL_ESC0_GPO_37: u8 = 11;
    pub const IOC_PD05_FUNC_CTL_GPIO_D_05: u8 = 0;
    pub const IOC_PD05_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PD05_FUNC_CTL_PPI0_DQ_13: u8 = 13;
    pub const IOC_PD05_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PD05_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PD05_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PD05_FUNC_CTL_TRGM_P_05: u8 = 17;
    pub const IOC_PD06_FUNC_CTL_ESC0_CTR_6: u8 = 27;
    pub const IOC_PD06_FUNC_CTL_ESC0_GPI_25: u8 = 10;
    pub const IOC_PD06_FUNC_CTL_ESC0_GPO_38: u8 = 11;
    pub const IOC_PD06_FUNC_CTL_GPIO_D_06: u8 = 0;
    pub const IOC_PD06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PD06_FUNC_CTL_PPI0_DQ_14: u8 = 13;
    pub const IOC_PD06_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PD06_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PD06_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PD06_FUNC_CTL_TRGM_P_06: u8 = 17;
    pub const IOC_PD07_FUNC_CTL_ESC0_CTR_7: u8 = 27;
    pub const IOC_PD07_FUNC_CTL_ESC0_GPI_24: u8 = 10;
    pub const IOC_PD07_FUNC_CTL_ESC0_GPO_39: u8 = 11;
    pub const IOC_PD07_FUNC_CTL_GPIO_D_07: u8 = 0;
    pub const IOC_PD07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PD07_FUNC_CTL_PPI0_CTR_3: u8 = 13;
    pub const IOC_PD07_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PD07_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PD07_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PD07_FUNC_CTL_TRGM_P_07: u8 = 17;
    pub const IOC_PD08_FUNC_CTL_ESC0_CTR_8: u8 = 27;
    pub const IOC_PD08_FUNC_CTL_ESC0_GPI_23: u8 = 10;
    pub const IOC_PD08_FUNC_CTL_ESC0_GPO_40: u8 = 11;
    pub const IOC_PD08_FUNC_CTL_ESC0_SCL: u8 = 18;
    pub const IOC_PD08_FUNC_CTL_GPIO_D_08: u8 = 0;
    pub const IOC_PD08_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PD08_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PD08_FUNC_CTL_PPI0_CS_3: u8 = 13;
    pub const IOC_PD08_FUNC_CTL_PWM1_P_0: u8 = 16;
    pub const IOC_PD08_FUNC_CTL_TRGM_P_08: u8 = 17;
    pub const IOC_PD09_FUNC_CTL_ESC0_GPI_22: u8 = 10;
    pub const IOC_PD09_FUNC_CTL_ESC0_GPO_41: u8 = 11;
    pub const IOC_PD09_FUNC_CTL_ESC0_SDA: u8 = 18;
    pub const IOC_PD09_FUNC_CTL_GPIO_D_09: u8 = 0;
    pub const IOC_PD09_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PD09_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PD09_FUNC_CTL_PWM1_P_1: u8 = 16;
    pub const IOC_PD09_FUNC_CTL_TRGM_P_09: u8 = 17;
    pub const IOC_PD10_FUNC_CTL_ESC0_GPI_21: u8 = 10;
    pub const IOC_PD10_FUNC_CTL_ESC0_GPO_42: u8 = 11;
    pub const IOC_PD10_FUNC_CTL_GPIO_D_10: u8 = 0;
    pub const IOC_PD10_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PD10_FUNC_CTL_PPI0_CTR_1: u8 = 13;
    pub const IOC_PD10_FUNC_CTL_PWM1_P_2: u8 = 16;
    pub const IOC_PD10_FUNC_CTL_TRGM_P_10: u8 = 17;
    pub const IOC_PD11_FUNC_CTL_ESC0_GPI_20: u8 = 10;
    pub const IOC_PD11_FUNC_CTL_ESC0_GPO_43: u8 = 11;
    pub const IOC_PD11_FUNC_CTL_GPIO_D_11: u8 = 0;
    pub const IOC_PD11_FUNC_CTL_PPI0_CS_2: u8 = 13;
    pub const IOC_PD11_FUNC_CTL_PWM1_P_3: u8 = 16;
    pub const IOC_PD11_FUNC_CTL_TRGM_P_11: u8 = 17;
    pub const IOC_PD12_FUNC_CTL_ESC0_GPI_19: u8 = 10;
    pub const IOC_PD12_FUNC_CTL_ESC0_GPO_44: u8 = 11;
    pub const IOC_PD12_FUNC_CTL_GPIO_D_12: u8 = 0;
    pub const IOC_PD12_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PD12_FUNC_CTL_PPI0_DQ_15: u8 = 13;
    pub const IOC_PD12_FUNC_CTL_PWM1_P_4: u8 = 16;
    pub const IOC_PD12_FUNC_CTL_TRGM_P_12: u8 = 17;
    pub const IOC_PD13_FUNC_CTL_ESC0_GPI_18: u8 = 10;
    pub const IOC_PD13_FUNC_CTL_ESC0_GPO_45: u8 = 11;
    pub const IOC_PD13_FUNC_CTL_GPIO_D_13: u8 = 0;
    pub const IOC_PD13_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PD13_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PD13_FUNC_CTL_PPI0_DM_1: u8 = 13;
    pub const IOC_PD13_FUNC_CTL_PWM1_P_5: u8 = 16;
    pub const IOC_PD13_FUNC_CTL_TRGM_P_13: u8 = 17;
    pub const IOC_PD14_FUNC_CTL_ESC0_GPI_17: u8 = 10;
    pub const IOC_PD14_FUNC_CTL_ESC0_GPO_46: u8 = 11;
    pub const IOC_PD14_FUNC_CTL_ESC0_REFCK: u8 = 18;
    pub const IOC_PD14_FUNC_CTL_GPIO_D_14: u8 = 0;
    pub const IOC_PD14_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PD14_FUNC_CTL_PPI0_DQ_08: u8 = 13;
    pub const IOC_PD14_FUNC_CTL_PWM1_P_6: u8 = 16;
    pub const IOC_PD14_FUNC_CTL_SYSCTL_CLK_OBS_3: u8 = 24;
    pub const IOC_PD14_FUNC_CTL_TRGM_P_14: u8 = 17;
    pub const IOC_PD15_FUNC_CTL_ESC0_GPI_16: u8 = 10;
    pub const IOC_PD15_FUNC_CTL_ESC0_GPO_47: u8 = 11;
    pub const IOC_PD15_FUNC_CTL_ESC0_REFCK: u8 = 18;
    pub const IOC_PD15_FUNC_CTL_GPIO_D_15: u8 = 0;
    pub const IOC_PD15_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PD15_FUNC_CTL_PPI0_DM_1: u8 = 13;
    pub const IOC_PD15_FUNC_CTL_PWM1_P_7: u8 = 16;
    pub const IOC_PD15_FUNC_CTL_SYSCTL_CLK_OBS_2: u8 = 24;
    pub const IOC_PD15_FUNC_CTL_TRGM_P_15: u8 = 17;
    pub const IOC_PD16_FUNC_CTL_ESC0_GPI_15: u8 = 10;
    pub const IOC_PD16_FUNC_CTL_ESC0_GPO_00: u8 = 18;
    pub const IOC_PD16_FUNC_CTL_ESC0_GPO_48: u8 = 11;
    pub const IOC_PD16_FUNC_CTL_ESC0_P2_RXDV: u8 = 27;
    pub const IOC_PD16_FUNC_CTL_GPIO_D_16: u8 = 0;
    pub const IOC_PD16_FUNC_CTL_PPI0_DQ_10: u8 = 13;
    pub const IOC_PD16_FUNC_CTL_SYSCTL_CLK_OBS_0: u8 = 24;
    pub const IOC_PD17_FUNC_CTL_ESC0_GPI_14: u8 = 10;
    pub const IOC_PD17_FUNC_CTL_ESC0_GPO_01: u8 = 18;
    pub const IOC_PD17_FUNC_CTL_ESC0_GPO_49: u8 = 11;
    pub const IOC_PD17_FUNC_CTL_ESC0_P2_RXD_0: u8 = 27;
    pub const IOC_PD17_FUNC_CTL_GPIO_D_17: u8 = 0;
    pub const IOC_PD17_FUNC_CTL_PPI0_DQ_09: u8 = 13;
    pub const IOC_PD17_FUNC_CTL_SYSCTL_CLK_OBS_1: u8 = 24;
    pub const IOC_PD18_FUNC_CTL_ESC0_GPI_13: u8 = 10;
    pub const IOC_PD18_FUNC_CTL_ESC0_GPO_02: u8 = 18;
    pub const IOC_PD18_FUNC_CTL_ESC0_GPO_50: u8 = 11;
    pub const IOC_PD18_FUNC_CTL_ESC0_P2_RXD_1: u8 = 27;
    pub const IOC_PD18_FUNC_CTL_GPIO_D_18: u8 = 0;
    pub const IOC_PD18_FUNC_CTL_PPI0_DQ_12: u8 = 13;
    pub const IOC_PD19_FUNC_CTL_ESC0_GPI_12: u8 = 10;
    pub const IOC_PD19_FUNC_CTL_ESC0_GPO_03: u8 = 18;
    pub const IOC_PD19_FUNC_CTL_ESC0_GPO_51: u8 = 11;
    pub const IOC_PD19_FUNC_CTL_ESC0_P2_RXD_2: u8 = 27;
    pub const IOC_PD19_FUNC_CTL_GPIO_D_19: u8 = 0;
    pub const IOC_PD19_FUNC_CTL_PPI0_DQ_11: u8 = 13;
    pub const IOC_PD20_FUNC_CTL_ESC0_GPI_11: u8 = 10;
    pub const IOC_PD20_FUNC_CTL_ESC0_GPO_04: u8 = 18;
    pub const IOC_PD20_FUNC_CTL_ESC0_GPO_52: u8 = 11;
    pub const IOC_PD20_FUNC_CTL_ESC0_P2_RXD_3: u8 = 27;
    pub const IOC_PD20_FUNC_CTL_GPIO_D_20: u8 = 0;
    pub const IOC_PD20_FUNC_CTL_PPI0_DQ_14: u8 = 13;
    pub const IOC_PD21_FUNC_CTL_ESC0_GPI_10: u8 = 10;
    pub const IOC_PD21_FUNC_CTL_ESC0_GPO_05: u8 = 18;
    pub const IOC_PD21_FUNC_CTL_ESC0_GPO_53: u8 = 11;
    pub const IOC_PD21_FUNC_CTL_ESC0_P2_RXCK: u8 = 27;
    pub const IOC_PD21_FUNC_CTL_GPIO_D_21: u8 = 0;
    pub const IOC_PD21_FUNC_CTL_PPI0_DQ_13: u8 = 13;
    pub const IOC_PD21_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PD22_FUNC_CTL_ESC0_GPI_09: u8 = 10;
    pub const IOC_PD22_FUNC_CTL_ESC0_GPO_06: u8 = 18;
    pub const IOC_PD22_FUNC_CTL_ESC0_GPO_54: u8 = 11;
    pub const IOC_PD22_FUNC_CTL_ESC0_P2_RXER: u8 = 27;
    pub const IOC_PD22_FUNC_CTL_GPIO_D_22: u8 = 0;
    pub const IOC_PD22_FUNC_CTL_PPI0_DQ_15: u8 = 13;
    pub const IOC_PD22_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PD23_FUNC_CTL_ESC0_GPI_08: u8 = 10;
    pub const IOC_PD23_FUNC_CTL_ESC0_GPO_07: u8 = 18;
    pub const IOC_PD23_FUNC_CTL_ESC0_GPO_55: u8 = 11;
    pub const IOC_PD23_FUNC_CTL_ESC0_P2_TXCK: u8 = 27;
    pub const IOC_PD23_FUNC_CTL_GPIO_D_23: u8 = 0;
    pub const IOC_PD23_FUNC_CTL_PPI0_DM_0: u8 = 13;
    pub const IOC_PD23_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PD24_FUNC_CTL_ESC0_GPI_07: u8 = 10;
    pub const IOC_PD24_FUNC_CTL_ESC0_GPO_08: u8 = 18;
    pub const IOC_PD24_FUNC_CTL_ESC0_GPO_56: u8 = 11;
    pub const IOC_PD24_FUNC_CTL_ESC0_P2_TXD_0: u8 = 27;
    pub const IOC_PD24_FUNC_CTL_GPIO_D_24: u8 = 0;
    pub const IOC_PD24_FUNC_CTL_PPI0_DQ_06: u8 = 13;
    pub const IOC_PD24_FUNC_CTL_SDM0_DAT_3: u8 = 23;
    pub const IOC_PD25_FUNC_CTL_ESC0_GPI_06: u8 = 10;
    pub const IOC_PD25_FUNC_CTL_ESC0_GPO_09: u8 = 18;
    pub const IOC_PD25_FUNC_CTL_ESC0_GPO_57: u8 = 11;
    pub const IOC_PD25_FUNC_CTL_ESC0_P2_TXD_1: u8 = 27;
    pub const IOC_PD25_FUNC_CTL_GPIO_D_25: u8 = 0;
    pub const IOC_PD25_FUNC_CTL_OWR0_DAT: u8 = 26;
    pub const IOC_PD25_FUNC_CTL_PPI0_DQ_07: u8 = 13;
    pub const IOC_PD25_FUNC_CTL_SDM0_CLK_3: u8 = 23;
    pub const IOC_PD26_FUNC_CTL_ESC0_GPI_05: u8 = 10;
    pub const IOC_PD26_FUNC_CTL_ESC0_GPO_10: u8 = 18;
    pub const IOC_PD26_FUNC_CTL_ESC0_GPO_58: u8 = 11;
    pub const IOC_PD26_FUNC_CTL_ESC0_P2_TXD_2: u8 = 27;
    pub const IOC_PD26_FUNC_CTL_GPIO_D_26: u8 = 0;
    pub const IOC_PD26_FUNC_CTL_OWR1_DAT: u8 = 26;
    pub const IOC_PD26_FUNC_CTL_PPI0_DQ_05: u8 = 13;
    pub const IOC_PD26_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PD26_FUNC_CTL_SDM0_CLK_2: u8 = 23;
    pub const IOC_PD27_FUNC_CTL_ESC0_GPI_04: u8 = 10;
    pub const IOC_PD27_FUNC_CTL_ESC0_GPO_11: u8 = 18;
    pub const IOC_PD27_FUNC_CTL_ESC0_GPO_59: u8 = 11;
    pub const IOC_PD27_FUNC_CTL_ESC0_P2_TXD_3: u8 = 27;
    pub const IOC_PD27_FUNC_CTL_GPIO_D_27: u8 = 0;
    pub const IOC_PD27_FUNC_CTL_PPI0_DQ_04: u8 = 13;
    pub const IOC_PD27_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PD27_FUNC_CTL_SDM0_DAT_2: u8 = 23;
    pub const IOC_PD28_FUNC_CTL_ESC0_GPI_03: u8 = 10;
    pub const IOC_PD28_FUNC_CTL_ESC0_GPO_12: u8 = 18;
    pub const IOC_PD28_FUNC_CTL_ESC0_GPO_60: u8 = 11;
    pub const IOC_PD28_FUNC_CTL_ESC0_P2_TXEN: u8 = 27;
    pub const IOC_PD28_FUNC_CTL_GPIO_D_28: u8 = 0;
    pub const IOC_PD28_FUNC_CTL_PPI0_DQ_03: u8 = 13;
    pub const IOC_PD28_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PD28_FUNC_CTL_SDM0_CLK_1: u8 = 23;
    pub const IOC_PD29_FUNC_CTL_ESC0_GPI_02: u8 = 10;
    pub const IOC_PD29_FUNC_CTL_ESC0_GPO_13: u8 = 18;
    pub const IOC_PD29_FUNC_CTL_ESC0_GPO_61: u8 = 11;
    pub const IOC_PD29_FUNC_CTL_ESC0_REFCK: u8 = 27;
    pub const IOC_PD29_FUNC_CTL_GPIO_D_29: u8 = 0;
    pub const IOC_PD29_FUNC_CTL_PPI0_DQ_02: u8 = 13;
    pub const IOC_PD29_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PD29_FUNC_CTL_SDM0_DAT_1: u8 = 23;
    pub const IOC_PD30_FUNC_CTL_ESC0_GPI_01: u8 = 10;
    pub const IOC_PD30_FUNC_CTL_ESC0_GPO_14: u8 = 18;
    pub const IOC_PD30_FUNC_CTL_ESC0_GPO_62: u8 = 11;
    pub const IOC_PD30_FUNC_CTL_ESC0_MDIO: u8 = 27;
    pub const IOC_PD30_FUNC_CTL_GPIO_D_30: u8 = 0;
    pub const IOC_PD30_FUNC_CTL_PPI0_DQ_01: u8 = 13;
    pub const IOC_PD30_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PD30_FUNC_CTL_SDM0_CLK_0: u8 = 23;
    pub const IOC_PD31_FUNC_CTL_ESC0_GPI_00: u8 = 10;
    pub const IOC_PD31_FUNC_CTL_ESC0_GPO_15: u8 = 18;
    pub const IOC_PD31_FUNC_CTL_ESC0_GPO_63: u8 = 11;
    pub const IOC_PD31_FUNC_CTL_ESC0_MDC: u8 = 27;
    pub const IOC_PD31_FUNC_CTL_GPIO_D_31: u8 = 0;
    pub const IOC_PD31_FUNC_CTL_PPI0_DQ_00: u8 = 13;
    pub const IOC_PD31_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PD31_FUNC_CTL_SDM0_DAT_0: u8 = 23;
    pub const IOC_PE00_FUNC_CTL_ESC0_CTR_4: u8 = 11;
    pub const IOC_PE00_FUNC_CTL_ESC0_EVTO_1: u8 = 27;
    pub const IOC_PE00_FUNC_CTL_ETH0_EVTO_1: u8 = 25;
    pub const IOC_PE00_FUNC_CTL_GPIO_E_00: u8 = 0;
    pub const IOC_PE00_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PE00_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PE00_FUNC_CTL_PPI0_CTR_4: u8 = 13;
    pub const IOC_PE00_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PE00_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PE00_FUNC_CTL_TRGM_P_00: u8 = 17;
    pub const IOC_PE00_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PE00_FUNC_CTL_USB0_ID: u8 = 24;
    pub const IOC_PE01_FUNC_CTL_ESC0_CTR_5: u8 = 11;
    pub const IOC_PE01_FUNC_CTL_GPIO_E_01: u8 = 0;
    pub const IOC_PE01_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PE01_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PE01_FUNC_CTL_PPI0_CTR_5: u8 = 13;
    pub const IOC_PE01_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PE01_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PE01_FUNC_CTL_TRGM_P_01: u8 = 17;
    pub const IOC_PE01_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PE01_FUNC_CTL_USB0_PWR: u8 = 24;
    pub const IOC_PE02_FUNC_CTL_ESC0_CTR_6: u8 = 11;
    pub const IOC_PE02_FUNC_CTL_GPIO_E_02: u8 = 0;
    pub const IOC_PE02_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PE02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PE02_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PE02_FUNC_CTL_PPI0_CTR_6: u8 = 13;
    pub const IOC_PE02_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PE02_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PE02_FUNC_CTL_TRGM_P_02: u8 = 17;
    pub const IOC_PE02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PE02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PE03_FUNC_CTL_ESC0_CTR_1: u8 = 11;
    pub const IOC_PE03_FUNC_CTL_GPIO_E_03: u8 = 0;
    pub const IOC_PE03_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PE03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PE03_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PE03_FUNC_CTL_PPI0_CTR_7: u8 = 13;
    pub const IOC_PE03_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PE03_FUNC_CTL_TRGM_P_03: u8 = 17;
    pub const IOC_PE03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PE03_FUNC_CTL_USB0_OC: u8 = 24;
    pub const IOC_PE04_FUNC_CTL_ESC0_CTR_2: u8 = 11;
    pub const IOC_PE04_FUNC_CTL_ESC0_EVTI_1: u8 = 27;
    pub const IOC_PE04_FUNC_CTL_ETH0_COL: u8 = 18;
    pub const IOC_PE04_FUNC_CTL_ETH0_EVTI_1: u8 = 25;
    pub const IOC_PE04_FUNC_CTL_GPIO_E_04: u8 = 0;
    pub const IOC_PE04_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PE04_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PE04_FUNC_CTL_TRGM_P_04: u8 = 17;
    pub const IOC_PE04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PE05_FUNC_CTL_ESC0_CTR_3: u8 = 11;
    pub const IOC_PE05_FUNC_CTL_ETH0_CRS: u8 = 18;
    pub const IOC_PE05_FUNC_CTL_GPIO_E_05: u8 = 0;
    pub const IOC_PE05_FUNC_CTL_GPTMR1_COMP_2: u8 = 1;
    pub const IOC_PE05_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PE05_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PE05_FUNC_CTL_TRGM_P_05: u8 = 17;
    pub const IOC_PE05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PE05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PF00_FUNC_CTL_ESC0_MDC: u8 = 11;
    pub const IOC_PF00_FUNC_CTL_ETH0_MDC: u8 = 18;
    pub const IOC_PF00_FUNC_CTL_GPIO_F_00: u8 = 0;
    pub const IOC_PF00_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PF00_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PF00_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PF00_FUNC_CTL_TRGM_P_00: u8 = 17;
    pub const IOC_PF01_FUNC_CTL_ESC0_MDIO: u8 = 11;
    pub const IOC_PF01_FUNC_CTL_ETH0_MDIO: u8 = 18;
    pub const IOC_PF01_FUNC_CTL_GPIO_F_01: u8 = 0;
    pub const IOC_PF01_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PF01_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PF01_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PF01_FUNC_CTL_TRGM_P_01: u8 = 17;
    pub const IOC_PF02_FUNC_CTL_ESC0_P1_TXCK: u8 = 11;
    pub const IOC_PF02_FUNC_CTL_ETH0_TXCK: u8 = 18;
    pub const IOC_PF02_FUNC_CTL_EUI0_CK: u8 = 26;
    pub const IOC_PF02_FUNC_CTL_GPIO_F_02: u8 = 0;
    pub const IOC_PF02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PF02_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PF02_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PF02_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PF02_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PF02_FUNC_CTL_TRGM_P_02: u8 = 17;
    pub const IOC_PF03_FUNC_CTL_ESC0_P1_TXD_0: u8 = 11;
    pub const IOC_PF03_FUNC_CTL_ETH0_TXD_0: u8 = 18;
    pub const IOC_PF03_FUNC_CTL_EUI0_SH: u8 = 26;
    pub const IOC_PF03_FUNC_CTL_GPIO_F_03: u8 = 0;
    pub const IOC_PF03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PF03_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PF03_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PF03_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PF03_FUNC_CTL_TRGM_P_03: u8 = 17;
    pub const IOC_PF04_FUNC_CTL_ESC0_P1_TXD_1: u8 = 11;
    pub const IOC_PF04_FUNC_CTL_ETH0_TXD_1: u8 = 18;
    pub const IOC_PF04_FUNC_CTL_EUI0_DI: u8 = 26;
    pub const IOC_PF04_FUNC_CTL_GPIO_F_04: u8 = 0;
    pub const IOC_PF04_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PF04_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PF04_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PF04_FUNC_CTL_TRGM_P_04: u8 = 17;
    pub const IOC_PF05_FUNC_CTL_ESC0_P1_TXD_2: u8 = 11;
    pub const IOC_PF05_FUNC_CTL_ETH0_TXD_2: u8 = 18;
    pub const IOC_PF05_FUNC_CTL_EUI0_DO: u8 = 26;
    pub const IOC_PF05_FUNC_CTL_GPIO_F_05: u8 = 0;
    pub const IOC_PF05_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PF05_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PF05_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PF05_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PF05_FUNC_CTL_TRGM_P_05: u8 = 17;
    pub const IOC_PF06_FUNC_CTL_ESC0_P1_TXD_3: u8 = 11;
    pub const IOC_PF06_FUNC_CTL_ETH0_TXD_3: u8 = 18;
    pub const IOC_PF06_FUNC_CTL_GPIO_F_06: u8 = 0;
    pub const IOC_PF06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PF06_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PF06_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PF06_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PF06_FUNC_CTL_TRGM_P_06: u8 = 17;
    pub const IOC_PF07_FUNC_CTL_ESC0_P1_TXEN: u8 = 11;
    pub const IOC_PF07_FUNC_CTL_ETH0_TXEN: u8 = 18;
    pub const IOC_PF07_FUNC_CTL_GPIO_F_07: u8 = 0;
    pub const IOC_PF07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PF07_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PF07_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PF07_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PF07_FUNC_CTL_TRGM_P_07: u8 = 17;
    pub const IOC_PF08_FUNC_CTL_ESC0_P1_RXDV: u8 = 11;
    pub const IOC_PF08_FUNC_CTL_ETH0_RXDV: u8 = 18;
    pub const IOC_PF08_FUNC_CTL_EUI1_CK: u8 = 26;
    pub const IOC_PF08_FUNC_CTL_GPIO_F_08: u8 = 0;
    pub const IOC_PF08_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PF08_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PF08_FUNC_CTL_PWM1_P_0: u8 = 16;
    pub const IOC_PF08_FUNC_CTL_TRGM_P_08: u8 = 17;
    pub const IOC_PF09_FUNC_CTL_ESC0_P1_RXD_0: u8 = 11;
    pub const IOC_PF09_FUNC_CTL_ETH0_RXD_0: u8 = 18;
    pub const IOC_PF09_FUNC_CTL_EUI1_SH: u8 = 26;
    pub const IOC_PF09_FUNC_CTL_GPIO_F_09: u8 = 0;
    pub const IOC_PF09_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PF09_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PF09_FUNC_CTL_PWM1_P_1: u8 = 16;
    pub const IOC_PF09_FUNC_CTL_TRGM_P_09: u8 = 17;
    pub const IOC_PF10_FUNC_CTL_ESC0_P1_RXD_1: u8 = 11;
    pub const IOC_PF10_FUNC_CTL_ETH0_RXD_1: u8 = 18;
    pub const IOC_PF10_FUNC_CTL_EUI1_DI: u8 = 26;
    pub const IOC_PF10_FUNC_CTL_GPIO_F_10: u8 = 0;
    pub const IOC_PF10_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PF10_FUNC_CTL_PWM1_P_2: u8 = 16;
    pub const IOC_PF10_FUNC_CTL_TRGM_P_10: u8 = 17;
    pub const IOC_PF11_FUNC_CTL_ESC0_P1_RXD_2: u8 = 11;
    pub const IOC_PF11_FUNC_CTL_ETH0_RXD_2: u8 = 18;
    pub const IOC_PF11_FUNC_CTL_EUI1_DO: u8 = 26;
    pub const IOC_PF11_FUNC_CTL_GPIO_F_11: u8 = 0;
    pub const IOC_PF11_FUNC_CTL_PWM1_P_3: u8 = 16;
    pub const IOC_PF11_FUNC_CTL_TRGM_P_11: u8 = 17;
    pub const IOC_PF12_FUNC_CTL_ESC0_P1_RXD_3: u8 = 11;
    pub const IOC_PF12_FUNC_CTL_ETH0_RXD_3: u8 = 18;
    pub const IOC_PF12_FUNC_CTL_GPIO_F_12: u8 = 0;
    pub const IOC_PF12_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PF12_FUNC_CTL_OWR0_DAT: u8 = 26;
    pub const IOC_PF12_FUNC_CTL_PWM1_P_4: u8 = 16;
    pub const IOC_PF12_FUNC_CTL_TRGM_P_12: u8 = 17;
    pub const IOC_PF13_FUNC_CTL_ESC0_P1_RXCK: u8 = 11;
    pub const IOC_PF13_FUNC_CTL_ETH0_RXCK: u8 = 18;
    pub const IOC_PF13_FUNC_CTL_GPIO_F_13: u8 = 0;
    pub const IOC_PF13_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PF13_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PF13_FUNC_CTL_OWR1_DAT: u8 = 26;
    pub const IOC_PF13_FUNC_CTL_PWM1_P_5: u8 = 16;
    pub const IOC_PF13_FUNC_CTL_TRGM_P_13: u8 = 17;
    pub const IOC_PF14_FUNC_CTL_ESC0_P1_RXER: u8 = 11;
    pub const IOC_PF14_FUNC_CTL_ETH0_RXER: u8 = 18;
    pub const IOC_PF14_FUNC_CTL_GPIO_F_14: u8 = 0;
    pub const IOC_PF14_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PF14_FUNC_CTL_PWM1_P_6: u8 = 16;
    pub const IOC_PF14_FUNC_CTL_TRGM_P_14: u8 = 17;
    pub const IOC_PF15_FUNC_CTL_ETH0_TXER: u8 = 18;
    pub const IOC_PF15_FUNC_CTL_GPIO_F_15: u8 = 0;
    pub const IOC_PF15_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PF15_FUNC_CTL_PWM1_P_7: u8 = 16;
    pub const IOC_PF15_FUNC_CTL_TRGM_P_15: u8 = 17;
    pub const IOC_PF16_FUNC_CTL_GPIO_F_16: u8 = 0;
    pub const IOC_PF16_FUNC_CTL_SDM0_DAT_0: u8 = 23;
    pub const IOC_PF17_FUNC_CTL_GPIO_F_17: u8 = 0;
    pub const IOC_PF17_FUNC_CTL_SDM0_CLK_0: u8 = 23;
    pub const IOC_PF18_FUNC_CTL_ESC0_EVTO_0: u8 = 27;
    pub const IOC_PF18_FUNC_CTL_ETH0_EVTO_0: u8 = 25;
    pub const IOC_PF18_FUNC_CTL_GPIO_F_18: u8 = 0;
    pub const IOC_PF18_FUNC_CTL_SDM0_DAT_1: u8 = 23;
    pub const IOC_PF18_FUNC_CTL_USB0_PWR: u8 = 24;
    pub const IOC_PF19_FUNC_CTL_GPIO_F_19: u8 = 0;
    pub const IOC_PF19_FUNC_CTL_SDM0_CLK_1: u8 = 23;
    pub const IOC_PF19_FUNC_CTL_USB0_PWR: u8 = 24;
    pub const IOC_PF20_FUNC_CTL_ESC0_EVTO_1: u8 = 27;
    pub const IOC_PF20_FUNC_CTL_ETH0_EVTO_1: u8 = 25;
    pub const IOC_PF20_FUNC_CTL_GPIO_F_20: u8 = 0;
    pub const IOC_PF20_FUNC_CTL_SDM0_DAT_3: u8 = 23;
    pub const IOC_PF20_FUNC_CTL_USB0_OC: u8 = 24;
    pub const IOC_PF21_FUNC_CTL_ESC0_EVTI_0: u8 = 27;
    pub const IOC_PF21_FUNC_CTL_ETH0_EVTI_0: u8 = 25;
    pub const IOC_PF21_FUNC_CTL_GPIO_F_21: u8 = 0;
    pub const IOC_PF21_FUNC_CTL_SDM0_DAT_2: u8 = 23;
    pub const IOC_PF21_FUNC_CTL_USB0_ID: u8 = 24;
    pub const IOC_PF22_FUNC_CTL_ESC0_EVTI_1: u8 = 27;
    pub const IOC_PF22_FUNC_CTL_ETH0_EVTI_1: u8 = 25;
    pub const IOC_PF22_FUNC_CTL_GPIO_F_22: u8 = 0;
    pub const IOC_PF22_FUNC_CTL_SDM0_CLK_2: u8 = 23;
    pub const IOC_PF22_FUNC_CTL_USB0_ID: u8 = 24;
    pub const IOC_PF23_FUNC_CTL_GPIO_F_23: u8 = 0;
    pub const IOC_PF23_FUNC_CTL_SDM0_CLK_3: u8 = 23;
    pub const IOC_PF23_FUNC_CTL_USB0_OC: u8 = 24;
    pub const IOC_PF24_FUNC_CTL_GPIO_F_24: u8 = 0;
    pub const IOC_PF24_FUNC_CTL_SDM0_DAT_0: u8 = 23;
    pub const IOC_PF25_FUNC_CTL_GPIO_F_25: u8 = 0;
    pub const IOC_PF25_FUNC_CTL_SDM0_CLK_0: u8 = 23;
    pub const IOC_PF26_FUNC_CTL_GPIO_F_26: u8 = 0;
    pub const IOC_PF26_FUNC_CTL_SDM0_DAT_1: u8 = 23;
    pub const IOC_PF27_FUNC_CTL_GPIO_F_27: u8 = 0;
    pub const IOC_PF27_FUNC_CTL_SDM0_CLK_1: u8 = 23;
    pub const IOC_PF28_FUNC_CTL_GPIO_F_28: u8 = 0;
    pub const IOC_PF28_FUNC_CTL_SDM0_DAT_3: u8 = 23;
    pub const IOC_PF29_FUNC_CTL_GPIO_F_29: u8 = 0;
    pub const IOC_PF29_FUNC_CTL_SDM0_DAT_2: u8 = 23;
    pub const IOC_PF30_FUNC_CTL_GPIO_F_30: u8 = 0;
    pub const IOC_PF30_FUNC_CTL_SDM0_CLK_2: u8 = 23;
    pub const IOC_PF31_FUNC_CTL_GPIO_F_31: u8 = 0;
    pub const IOC_PF31_FUNC_CTL_SDM0_CLK_3: u8 = 23;
    pub const IOC_PV00_FUNC_CTL_ESC0_P0_RXDV: u8 = 11;
    pub const IOC_PV00_FUNC_CTL_GPIO_V_00: u8 = 0;
    pub const IOC_PV00_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PV00_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PV00_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PV01_FUNC_CTL_ESC0_P0_RXD_0: u8 = 11;
    pub const IOC_PV01_FUNC_CTL_GPIO_V_01: u8 = 0;
    pub const IOC_PV01_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PV01_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PV01_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PV02_FUNC_CTL_ESC0_P0_RXD_1: u8 = 11;
    pub const IOC_PV02_FUNC_CTL_GPIO_V_02: u8 = 0;
    pub const IOC_PV02_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PV02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PV02_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PV02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PV02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PV03_FUNC_CTL_ESC0_P0_RXD_2: u8 = 11;
    pub const IOC_PV03_FUNC_CTL_GPIO_V_03: u8 = 0;
    pub const IOC_PV03_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PV03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PV03_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PV03_FUNC_CTL_SPI1_CS_3: u8 = 5;
    pub const IOC_PV03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PV04_FUNC_CTL_ESC0_P0_RXD_3: u8 = 11;
    pub const IOC_PV04_FUNC_CTL_GPIO_V_04: u8 = 0;
    pub const IOC_PV04_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PV04_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PV04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PV05_FUNC_CTL_ESC0_P0_RXCK: u8 = 11;
    pub const IOC_PV05_FUNC_CTL_GPIO_V_05: u8 = 0;
    pub const IOC_PV05_FUNC_CTL_GPTMR1_COMP_2: u8 = 1;
    pub const IOC_PV05_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PV05_FUNC_CTL_SPI0_CS_0: u8 = 5;
    pub const IOC_PV05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PV05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PV06_FUNC_CTL_ESC0_P0_TXCK: u8 = 11;
    pub const IOC_PV06_FUNC_CTL_GPIO_V_06: u8 = 0;
    pub const IOC_PV06_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PV06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PV06_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PV06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PV07_FUNC_CTL_ESC0_P0_TXD_0: u8 = 11;
    pub const IOC_PV07_FUNC_CTL_GPIO_V_07: u8 = 0;
    pub const IOC_PV07_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PV07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PV07_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PV07_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PV08_FUNC_CTL_ESC0_P0_TXD_1: u8 = 11;
    pub const IOC_PV08_FUNC_CTL_GPIO_V_08: u8 = 0;
    pub const IOC_PV08_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PV08_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PV08_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PV08_FUNC_CTL_SPI1_CS_2: u8 = 5;
    pub const IOC_PV08_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PV09_FUNC_CTL_ESC0_P0_TXD_2: u8 = 11;
    pub const IOC_PV09_FUNC_CTL_GPIO_V_09: u8 = 0;
    pub const IOC_PV09_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PV09_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PV09_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PV09_FUNC_CTL_SPI1_CS_1: u8 = 5;
    pub const IOC_PV09_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PV10_FUNC_CTL_ESC0_P0_TXD_3: u8 = 11;
    pub const IOC_PV10_FUNC_CTL_GPIO_V_10: u8 = 0;
    pub const IOC_PV10_FUNC_CTL_GPTMR0_COMP_2: u8 = 1;
    pub const IOC_PV10_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PV10_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PV10_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PV10_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PV11_FUNC_CTL_ESC0_P0_TXEN: u8 = 11;
    pub const IOC_PV11_FUNC_CTL_GPIO_V_11: u8 = 0;
    pub const IOC_PV11_FUNC_CTL_SPI1_CS_0: u8 = 5;
    pub const IOC_PV11_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PV12_FUNC_CTL_ESC0_CTR_0: u8 = 11;
    pub const IOC_PV12_FUNC_CTL_GPIO_V_12: u8 = 0;
    pub const IOC_PV12_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PV15_FUNC_CTL_ESC0_P0_RXER: u8 = 11;
    pub const IOC_PV15_FUNC_CTL_GPIO_V_15: u8 = 0;
    pub const IOC_PV15_FUNC_CTL_GPTMR0_COMP_3: u8 = 1;
    pub const IOC_PV15_FUNC_CTL_SPI1_DAT3: u8 = 5;
    pub const IOC_PW00_FUNC_CTL_ESC0_P1_RXDV: u8 = 11;
    pub const IOC_PW00_FUNC_CTL_ETH0_RXDV: u8 = 18;
    pub const IOC_PW00_FUNC_CTL_GPIO_W_00: u8 = 0;
    pub const IOC_PW00_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PW01_FUNC_CTL_ESC0_P1_RXD_0: u8 = 11;
    pub const IOC_PW01_FUNC_CTL_ETH0_RXD_0: u8 = 18;
    pub const IOC_PW01_FUNC_CTL_GPIO_W_01: u8 = 0;
    pub const IOC_PW01_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PW02_FUNC_CTL_ESC0_P1_RXD_1: u8 = 11;
    pub const IOC_PW02_FUNC_CTL_ETH0_RXD_1: u8 = 18;
    pub const IOC_PW02_FUNC_CTL_GPIO_W_02: u8 = 0;
    pub const IOC_PW02_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PW03_FUNC_CTL_ESC0_P1_RXD_2: u8 = 11;
    pub const IOC_PW03_FUNC_CTL_ETH0_RXD_2: u8 = 18;
    pub const IOC_PW03_FUNC_CTL_GPIO_W_03: u8 = 0;
    pub const IOC_PW03_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PW04_FUNC_CTL_ESC0_P1_RXD_3: u8 = 11;
    pub const IOC_PW04_FUNC_CTL_ETH0_RXD_3: u8 = 18;
    pub const IOC_PW04_FUNC_CTL_GPIO_W_04: u8 = 0;
    pub const IOC_PW04_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PW05_FUNC_CTL_ESC0_P1_RXCK: u8 = 11;
    pub const IOC_PW05_FUNC_CTL_ETH0_RXCK: u8 = 18;
    pub const IOC_PW05_FUNC_CTL_GPIO_W_05: u8 = 0;
    pub const IOC_PW05_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PW06_FUNC_CTL_ESC0_P1_TXCK: u8 = 11;
    pub const IOC_PW06_FUNC_CTL_ETH0_TXCK: u8 = 18;
    pub const IOC_PW06_FUNC_CTL_GPIO_W_06: u8 = 0;
    pub const IOC_PW07_FUNC_CTL_ESC0_P1_TXD_0: u8 = 11;
    pub const IOC_PW07_FUNC_CTL_ETH0_TXD_0: u8 = 18;
    pub const IOC_PW07_FUNC_CTL_GPIO_W_07: u8 = 0;
    pub const IOC_PW08_FUNC_CTL_ESC0_P1_TXD_1: u8 = 11;
    pub const IOC_PW08_FUNC_CTL_ETH0_TXD_1: u8 = 18;
    pub const IOC_PW08_FUNC_CTL_GPIO_W_08: u8 = 0;
    pub const IOC_PW08_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PW09_FUNC_CTL_ESC0_P1_TXD_2: u8 = 11;
    pub const IOC_PW09_FUNC_CTL_ETH0_TXD_2: u8 = 18;
    pub const IOC_PW09_FUNC_CTL_GPIO_W_09: u8 = 0;
    pub const IOC_PW09_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PW10_FUNC_CTL_ESC0_P1_TXD_3: u8 = 11;
    pub const IOC_PW10_FUNC_CTL_ETH0_TXD_3: u8 = 18;
    pub const IOC_PW10_FUNC_CTL_GPIO_W_10: u8 = 0;
    pub const IOC_PW10_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PW11_FUNC_CTL_ESC0_P1_TXEN: u8 = 11;
    pub const IOC_PW11_FUNC_CTL_ETH0_TXEN: u8 = 18;
    pub const IOC_PW11_FUNC_CTL_GPIO_W_11: u8 = 0;
    pub const IOC_PW12_FUNC_CTL_ESC0_CTR_1: u8 = 11;
    pub const IOC_PW12_FUNC_CTL_ETH0_RXER: u8 = 18;
    pub const IOC_PW12_FUNC_CTL_GPIO_W_12: u8 = 0;
    pub const IOC_PW15_FUNC_CTL_ESC0_P1_RXER: u8 = 11;
    pub const IOC_PW15_FUNC_CTL_ETH0_CRS: u8 = 18;
    pub const IOC_PW15_FUNC_CTL_GPIO_W_15: u8 = 0;
    pub const IOC_PW16_FUNC_CTL_ESC0_CTR_6: u8 = 11;
    pub const IOC_PW16_FUNC_CTL_ETH0_MDIO: u8 = 18;
    pub const IOC_PW16_FUNC_CTL_GPIO_W_16: u8 = 0;
    pub const IOC_PW17_FUNC_CTL_ESC0_CTR_7: u8 = 11;
    pub const IOC_PW17_FUNC_CTL_ETH0_MDC: u8 = 18;
    pub const IOC_PW17_FUNC_CTL_GPIO_W_17: u8 = 0;
    pub const IOC_PW20_FUNC_CTL_ESC0_REFCK: u8 = 11;
    pub const IOC_PW20_FUNC_CTL_ETH0_TXER: u8 = 18;
    pub const IOC_PW20_FUNC_CTL_GPIO_W_20: u8 = 0;
    pub const IOC_PW20_FUNC_CTL_SOC_REF0: u8 = 24;
    pub const IOC_PW21_FUNC_CTL_ESC0_REFCK: u8 = 11;
    pub const IOC_PW21_FUNC_CTL_ETH0_COL: u8 = 18;
    pub const IOC_PW21_FUNC_CTL_GPIO_W_21: u8 = 0;
    pub const IOC_PW21_FUNC_CTL_SOC_REF1: u8 = 24;
    pub const IOC_PX00_FUNC_CTL_GPIO_X_00: u8 = 0;
    pub const IOC_PX00_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PX00_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PX00_FUNC_CTL_XPI0_CB_DQS: u8 = 14;
    pub const IOC_PX01_FUNC_CTL_GPIO_X_01: u8 = 0;
    pub const IOC_PX01_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PX01_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PX01_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PX02_FUNC_CTL_GPIO_X_02: u8 = 0;
    pub const IOC_PX02_FUNC_CTL_GPTMR3_COMP_1: u8 = 1;
    pub const IOC_PX02_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PX02_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PX02_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PX03_FUNC_CTL_GPIO_X_03: u8 = 0;
    pub const IOC_PX03_FUNC_CTL_GPTMR3_CAPT_1: u8 = 1;
    pub const IOC_PX03_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PX03_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PX04_FUNC_CTL_GPIO_X_04: u8 = 0;
    pub const IOC_PX04_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PX04_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PX04_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PX05_FUNC_CTL_GPIO_X_05: u8 = 0;
    pub const IOC_PX05_FUNC_CTL_GPTMR3_COMP_2: u8 = 1;
    pub const IOC_PX05_FUNC_CTL_SPI3_CS_0: u8 = 5;
    pub const IOC_PX05_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PX05_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PX05_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PX06_FUNC_CTL_GPIO_X_06: u8 = 0;
    pub const IOC_PX06_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PX06_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PX06_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PX06_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PX07_FUNC_CTL_GPIO_X_07: u8 = 0;
    pub const IOC_PX07_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PX07_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PX07_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PX07_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
}
pub mod trgmmux {
    //! `TRGMMUX` definitions
}
