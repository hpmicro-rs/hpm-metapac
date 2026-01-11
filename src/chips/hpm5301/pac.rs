#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    #[doc = "13 - UART0"]
    UART0 = 13,
    #[doc = "14 - UART1"]
    UART1 = 14,
    #[doc = "15 - UART2"]
    UART2 = 15,
    #[doc = "16 - UART3"]
    UART3 = 16,
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
    #[doc = "44 - TRGMUX0"]
    TRGMUX0 = 44,
    #[doc = "49 - TRGMUX1"]
    TRGMUX1 = 49,
    #[doc = "51 - USB0"]
    USB0 = 51,
    #[doc = "52 - XPI0"]
    XPI0 = 52,
    #[doc = "54 - PSEC"]
    PSEC = 54,
    #[doc = "55 - SECMON"]
    SECMON = 55,
    #[doc = "57 - FUSE"]
    FUSE = 57,
    #[doc = "58 - ADC0"]
    ADC0 = 58,
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
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn CORE_LOCAL();
        fn GPIO0_A();
        fn GPIO0_B();
        fn GPIO0_X();
        fn GPIO0_Y();
        fn GPTMR0();
        fn GPTMR1();
        fn UART0();
        fn UART1();
        fn UART2();
        fn UART3();
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
        fn TRGMUX0();
        fn TRGMUX1();
        fn USB0();
        fn XPI0();
        fn PSEC();
        fn SECMON();
        fn FUSE();
        fn ADC0();
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
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 73] = [
        Vector {
            _handler: CORE_LOCAL,
        },
        Vector { _handler: GPIO0_A },
        Vector { _handler: GPIO0_B },
        Vector { _handler: GPIO0_X },
        Vector { _handler: GPIO0_Y },
        Vector { _handler: GPTMR0 },
        Vector { _handler: GPTMR1 },
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
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
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
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TRGMUX0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: TRGMUX1 },
        Vector { _reserved: 0 },
        Vector { _handler: USB0 },
        Vector { _handler: XPI0 },
        Vector { _reserved: 0 },
        Vector { _handler: PSEC },
        Vector { _handler: SECMON },
        Vector { _reserved: 0 },
        Vector { _handler: FUSE },
        Vector { _handler: ADC0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
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
pub const UART0: uart::Uart = unsafe { uart::Uart::from_ptr(0xf004_0000usize as _) };
pub const UART1: uart::Uart = unsafe { uart::Uart::from_ptr(0xf004_4000usize as _) };
pub const UART2: uart::Uart = unsafe { uart::Uart::from_ptr(0xf004_8000usize as _) };
pub const UART3: uart::Uart = unsafe { uart::Uart::from_ptr(0xf004_c000usize as _) };
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
pub const EWDG0: ewdg::Ewdg = unsafe { ewdg::Ewdg::from_ptr(0xf00b_0000usize as _) };
pub const EWDG1: ewdg::Ewdg = unsafe { ewdg::Ewdg::from_ptr(0xf00b_4000usize as _) };
pub const DMAMUX: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0xf00c_4000usize as _) };
pub const HDMA: dma::Dma = unsafe { dma::Dma::from_ptr(0xf00c_8000usize as _) };
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf00d_0000usize as _) };
pub const GPIOM: gpiom::Gpiom = unsafe { gpiom::Gpiom::from_ptr(0xf00d_8000usize as _) };
pub const PTPC: ptpc::Ptpc = unsafe { ptpc::Ptpc::from_ptr(0xf02f_c000usize as _) };
pub const XPI0: xpi::Xpi = unsafe { xpi::Xpi::from_ptr(0xf300_0000usize as _) };
pub const USB0: usb::Usb = unsafe { usb::Usb::from_ptr(0xf300_c000usize as _) };
pub const OTP: otp::Otp = unsafe { otp::Otp::from_ptr(0xf305_0000usize as _) };
pub const ADC0: adc16::Adc = unsafe { adc16::Adc::from_ptr(0xf308_0000usize as _) };
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
pub const PWDG: ewdg::Ewdg = unsafe { ewdg::Ewdg::from_ptr(0xf412_8000usize as _) };
pub const PDGO: pdgo::Pdgo = unsafe { pdgo::Pdgo::from_ptr(0xf413_4000usize as _) };
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
            3 => Ok(Self::GPIO0_X),
            4 => Ok(Self::GPIO0_Y),
            5 => Ok(Self::GPTMR0),
            6 => Ok(Self::GPTMR1),
            13 => Ok(Self::UART0),
            14 => Ok(Self::UART1),
            15 => Ok(Self::UART2),
            16 => Ok(Self::UART3),
            21 => Ok(Self::I2C0),
            22 => Ok(Self::I2C1),
            23 => Ok(Self::I2C2),
            24 => Ok(Self::I2C3),
            25 => Ok(Self::SPI0),
            26 => Ok(Self::SPI1),
            27 => Ok(Self::SPI2),
            28 => Ok(Self::SPI3),
            29 => Ok(Self::TSNS),
            30 => Ok(Self::MBX0A),
            31 => Ok(Self::MBX0B),
            32 => Ok(Self::EWDG0),
            33 => Ok(Self::EWDG1),
            34 => Ok(Self::HDMA),
            44 => Ok(Self::TRGMUX0),
            49 => Ok(Self::TRGMUX1),
            51 => Ok(Self::USB0),
            52 => Ok(Self::XPI0),
            54 => Ok(Self::PSEC),
            55 => Ok(Self::SECMON),
            57 => Ok(Self::FUSE),
            58 => Ok(Self::ADC0),
            62 => Ok(Self::ACMP_0),
            63 => Ok(Self::ACMP_1),
            64 => Ok(Self::SYSCTL),
            65 => Ok(Self::PGPIO),
            66 => Ok(Self::PTMR),
            67 => Ok(Self::PUART),
            68 => Ok(Self::PEWDG),
            69 => Ok(Self::BROWNOUT),
            70 => Ok(Self::PAD_WAKEUP),
            71 => Ok(Self::DEBUG0),
            72 => Ok(Self::DEBUG1),

            _ => Err(riscv_pac::result::Error::InvalidVariant(value)),
        }
    }
}
unsafe impl riscv_pac::ExternalInterruptNumber for Interrupt {}
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
#[path = "../../peripherals/ewdg_v53.rs"]
pub mod ewdg;
#[path = "../../peripherals/gpio_v53.rs"]
pub mod gpio;
#[path = "../../peripherals/gpiom_v53.rs"]
pub mod gpiom;
#[path = "../../peripherals/i2c_v53.rs"]
pub mod i2c;
#[path = "../../peripherals/ioc_common.rs"]
pub mod ioc;
#[path = "../../peripherals/mbx_common.rs"]
pub mod mbx;
#[path = "../../peripherals/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../../peripherals/otp_common.rs"]
pub mod otp;
#[path = "../../peripherals/pcfg_v53.rs"]
pub mod pcfg;
#[path = "../../peripherals/pdgo_v53.rs"]
pub mod pdgo;
#[path = "../../peripherals/plic_common.rs"]
pub mod plic;
#[path = "../../peripherals/plicsw_common.rs"]
pub mod plicsw;
#[path = "../../peripherals/pllctl_v2.rs"]
pub mod pllctl;
#[path = "../../peripherals/ppor_v53.rs"]
pub mod ppor;
#[path = "../../peripherals/ptpc_common.rs"]
pub mod ptpc;
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
#[path = "../../peripherals/xpi_dummy.rs"]
pub mod xpi;
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
    pub const IOC_PA00_FUNC_CTL_GPIO_A_00: u8 = 0;
    pub const IOC_PA00_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PA00_FUNC_CTL_SYSCTL_CLK_OBS_0: u8 = 24;
    pub const IOC_PA00_FUNC_CTL_TRGM0_P_00: u8 = 18;
    pub const IOC_PA00_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PA01_FUNC_CTL_ACMP_COMP_0: u8 = 19;
    pub const IOC_PA01_FUNC_CTL_GPIO_A_01: u8 = 0;
    pub const IOC_PA01_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PA01_FUNC_CTL_SYSCTL_CLK_OBS_1: u8 = 24;
    pub const IOC_PA01_FUNC_CTL_TRGM0_P_01: u8 = 18;
    pub const IOC_PA01_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PA02_FUNC_CTL_ACMP_COMP_0: u8 = 16;
    pub const IOC_PA02_FUNC_CTL_ACMP_COMP_1: u8 = 19;
    pub const IOC_PA02_FUNC_CTL_GPIO_A_02: u8 = 0;
    pub const IOC_PA02_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PA02_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PA02_FUNC_CTL_SYSCTL_CLK_OBS_2: u8 = 24;
    pub const IOC_PA02_FUNC_CTL_TRGM0_P_02: u8 = 18;
    pub const IOC_PA02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PA02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PA03_FUNC_CTL_ACMP_COMP_1: u8 = 16;
    pub const IOC_PA03_FUNC_CTL_GPIO_A_03: u8 = 0;
    pub const IOC_PA03_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PA03_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PA03_FUNC_CTL_SPI3_CS_3: u8 = 5;
    pub const IOC_PA03_FUNC_CTL_SYSCTL_CLK_OBS_3: u8 = 24;
    pub const IOC_PA03_FUNC_CTL_TRGM0_P_03: u8 = 18;
    pub const IOC_PA03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PA04_FUNC_CTL_GPIO_A_04: u8 = 0;
    pub const IOC_PA04_FUNC_CTL_JTAG_TDO: u8 = 24;
    pub const IOC_PA04_FUNC_CTL_SPI0_CS_0: u8 = 5;
    pub const IOC_PA04_FUNC_CTL_TRGM0_P_04: u8 = 18;
    pub const IOC_PA04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PA05_FUNC_CTL_GPIO_A_05: u8 = 0;
    pub const IOC_PA05_FUNC_CTL_GPTMR1_COMP_2: u8 = 1;
    pub const IOC_PA05_FUNC_CTL_JTAG_TDI: u8 = 24;
    pub const IOC_PA05_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PA05_FUNC_CTL_TRGM0_P_05: u8 = 18;
    pub const IOC_PA05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PA05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PA06_FUNC_CTL_GPIO_A_06: u8 = 0;
    pub const IOC_PA06_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PA06_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PA06_FUNC_CTL_JTAG_TCK: u8 = 24;
    pub const IOC_PA06_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PA06_FUNC_CTL_TRGM0_P_06: u8 = 18;
    pub const IOC_PA06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PA07_FUNC_CTL_GPIO_A_07: u8 = 0;
    pub const IOC_PA07_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PA07_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PA07_FUNC_CTL_JTAG_TMS: u8 = 24;
    pub const IOC_PA07_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PA07_FUNC_CTL_TRGM0_P_07: u8 = 18;
    pub const IOC_PA07_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PA08_FUNC_CTL_GPIO_A_08: u8 = 0;
    pub const IOC_PA08_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PA08_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PA08_FUNC_CTL_JTAG_TRST: u8 = 24;
    pub const IOC_PA08_FUNC_CTL_SPI3_CS_2: u8 = 5;
    pub const IOC_PA08_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PA09_FUNC_CTL_GPIO_A_09: u8 = 0;
    pub const IOC_PA09_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PA09_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PA09_FUNC_CTL_SOC_REF0: u8 = 24;
    pub const IOC_PA09_FUNC_CTL_SPI3_CS_1: u8 = 5;
    pub const IOC_PA09_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PA10_FUNC_CTL_ACMP_COMP_0: u8 = 18;
    pub const IOC_PA10_FUNC_CTL_GPIO_A_10: u8 = 0;
    pub const IOC_PA10_FUNC_CTL_GPTMR0_COMP_2: u8 = 1;
    pub const IOC_PA10_FUNC_CTL_SPI3_CS_0: u8 = 5;
    pub const IOC_PA10_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PA10_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PA11_FUNC_CTL_ACMP_COMP_1: u8 = 18;
    pub const IOC_PA11_FUNC_CTL_EWDG0_RST: u8 = 24;
    pub const IOC_PA11_FUNC_CTL_GPIO_A_11: u8 = 0;
    pub const IOC_PA11_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PA11_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PA12_FUNC_CTL_GPIO_A_12: u8 = 0;
    pub const IOC_PA12_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PA12_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PA12_FUNC_CTL_UART3_CTS: u8 = 3;
    pub const IOC_PA13_FUNC_CTL_GPIO_A_13: u8 = 0;
    pub const IOC_PA13_FUNC_CTL_GPTMR1_COMP_3: u8 = 1;
    pub const IOC_PA13_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PA13_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PA13_FUNC_CTL_UART3_DE: u8 = 2;
    pub const IOC_PA13_FUNC_CTL_UART3_RTS: u8 = 3;
    pub const IOC_PA14_FUNC_CTL_ACMP_COMP_0: u8 = 18;
    pub const IOC_PA14_FUNC_CTL_EWDG1_RST: u8 = 24;
    pub const IOC_PA14_FUNC_CTL_GPIO_A_14: u8 = 0;
    pub const IOC_PA14_FUNC_CTL_SPI3_DAT2: u8 = 5;
    pub const IOC_PA14_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PA15_FUNC_CTL_ACMP_COMP_1: u8 = 18;
    pub const IOC_PA15_FUNC_CTL_GPIO_A_15: u8 = 0;
    pub const IOC_PA15_FUNC_CTL_GPTMR0_COMP_3: u8 = 1;
    pub const IOC_PA15_FUNC_CTL_SOC_REF0: u8 = 24;
    pub const IOC_PA15_FUNC_CTL_SPI3_DAT3: u8 = 5;
    pub const IOC_PA15_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PA16_FUNC_CTL_GPIO_A_16: u8 = 0;
    pub const IOC_PA16_FUNC_CTL_TRGM0_P_04: u8 = 18;
    pub const IOC_PA17_FUNC_CTL_GPIO_A_17: u8 = 0;
    pub const IOC_PA17_FUNC_CTL_TRGM0_P_05: u8 = 18;
    pub const IOC_PA18_FUNC_CTL_GPIO_A_18: u8 = 0;
    pub const IOC_PA18_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PA18_FUNC_CTL_TRGM0_P_06: u8 = 18;
    pub const IOC_PA19_FUNC_CTL_GPIO_A_19: u8 = 0;
    pub const IOC_PA19_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PA19_FUNC_CTL_SPI1_CS_3: u8 = 5;
    pub const IOC_PA19_FUNC_CTL_TRGM0_P_07: u8 = 18;
    pub const IOC_PA20_FUNC_CTL_GPIO_A_20: u8 = 0;
    pub const IOC_PA20_FUNC_CTL_SPI2_CS_0: u8 = 5;
    pub const IOC_PA20_FUNC_CTL_TRGM0_P_00: u8 = 18;
    pub const IOC_PA21_FUNC_CTL_GPIO_A_21: u8 = 0;
    pub const IOC_PA21_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PA21_FUNC_CTL_TRGM0_P_01: u8 = 18;
    pub const IOC_PA22_FUNC_CTL_GPIO_A_22: u8 = 0;
    pub const IOC_PA22_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PA22_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PA22_FUNC_CTL_TRGM0_P_02: u8 = 18;
    pub const IOC_PA23_FUNC_CTL_GPIO_A_23: u8 = 0;
    pub const IOC_PA23_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PA23_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PA23_FUNC_CTL_TRGM0_P_03: u8 = 18;
    pub const IOC_PA24_FUNC_CTL_GPIO_A_24: u8 = 0;
    pub const IOC_PA24_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PA24_FUNC_CTL_SPI1_CS_2: u8 = 5;
    pub const IOC_PA24_FUNC_CTL_TRGM0_P_00: u8 = 18;
    pub const IOC_PA24_FUNC_CTL_XPI0_CA_CS1: u8 = 14;
    pub const IOC_PA25_FUNC_CTL_GPIO_A_25: u8 = 0;
    pub const IOC_PA25_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PA25_FUNC_CTL_SPI1_CS_1: u8 = 5;
    pub const IOC_PA25_FUNC_CTL_TRGM0_P_01: u8 = 18;
    pub const IOC_PA25_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PA26_FUNC_CTL_GPIO_A_26: u8 = 0;
    pub const IOC_PA26_FUNC_CTL_SPI1_CS_0: u8 = 5;
    pub const IOC_PA26_FUNC_CTL_SYSCTL_CLK_OBS_0: u8 = 24;
    pub const IOC_PA26_FUNC_CTL_TRGM0_P_02: u8 = 18;
    pub const IOC_PA26_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PA27_FUNC_CTL_GPIO_A_27: u8 = 0;
    pub const IOC_PA27_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PA27_FUNC_CTL_SYSCTL_CLK_OBS_1: u8 = 24;
    pub const IOC_PA27_FUNC_CTL_TRGM0_P_03: u8 = 18;
    pub const IOC_PA27_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PA28_FUNC_CTL_GPIO_A_28: u8 = 0;
    pub const IOC_PA28_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PA28_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PA28_FUNC_CTL_SYSCTL_CLK_OBS_2: u8 = 24;
    pub const IOC_PA28_FUNC_CTL_TRGM0_P_04: u8 = 18;
    pub const IOC_PA28_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PA29_FUNC_CTL_GPIO_A_29: u8 = 0;
    pub const IOC_PA29_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PA29_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PA29_FUNC_CTL_SYSCTL_CLK_OBS_3: u8 = 24;
    pub const IOC_PA29_FUNC_CTL_TRGM0_P_05: u8 = 18;
    pub const IOC_PA29_FUNC_CTL_USB0_OC: u8 = 25;
    pub const IOC_PA29_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
    pub const IOC_PA30_FUNC_CTL_GPIO_A_30: u8 = 0;
    pub const IOC_PA30_FUNC_CTL_SOC_REF0: u8 = 24;
    pub const IOC_PA30_FUNC_CTL_SPI1_DAT2: u8 = 5;
    pub const IOC_PA30_FUNC_CTL_TRGM0_P_06: u8 = 18;
    pub const IOC_PA30_FUNC_CTL_USB0_PWR: u8 = 25;
    pub const IOC_PA30_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PA31_FUNC_CTL_GPIO_A_31: u8 = 0;
    pub const IOC_PA31_FUNC_CTL_SPI1_DAT3: u8 = 5;
    pub const IOC_PA31_FUNC_CTL_TRGM0_P_07: u8 = 18;
    pub const IOC_PA31_FUNC_CTL_USB0_ID: u8 = 25;
    pub const IOC_PA31_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PB00_FUNC_CTL_ACMP_COMP_0: u8 = 19;
    pub const IOC_PB00_FUNC_CTL_GPIO_B_00: u8 = 0;
    pub const IOC_PB00_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PB00_FUNC_CTL_TRGM0_P_04: u8 = 18;
    pub const IOC_PB00_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PB01_FUNC_CTL_ACMP_COMP_1: u8 = 19;
    pub const IOC_PB01_FUNC_CTL_GPIO_B_01: u8 = 0;
    pub const IOC_PB01_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PB01_FUNC_CTL_TRGM0_P_05: u8 = 18;
    pub const IOC_PB01_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PB02_FUNC_CTL_ACMP_COMP_1: u8 = 17;
    pub const IOC_PB02_FUNC_CTL_GPIO_B_02: u8 = 0;
    pub const IOC_PB02_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PB02_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PB02_FUNC_CTL_TRGM0_P_06: u8 = 18;
    pub const IOC_PB02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PB02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PB03_FUNC_CTL_ACMP_COMP_0: u8 = 17;
    pub const IOC_PB03_FUNC_CTL_GPIO_B_03: u8 = 0;
    pub const IOC_PB03_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PB03_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PB03_FUNC_CTL_SPI2_CS_3: u8 = 5;
    pub const IOC_PB03_FUNC_CTL_TRGM0_P_07: u8 = 18;
    pub const IOC_PB03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PB04_FUNC_CTL_GPIO_B_04: u8 = 0;
    pub const IOC_PB04_FUNC_CTL_SPI3_CS_0: u8 = 5;
    pub const IOC_PB04_FUNC_CTL_TRGM0_P_00: u8 = 18;
    pub const IOC_PB04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PB05_FUNC_CTL_GPIO_B_05: u8 = 0;
    pub const IOC_PB05_FUNC_CTL_GPTMR1_COMP_2: u8 = 1;
    pub const IOC_PB05_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PB05_FUNC_CTL_TRGM0_P_01: u8 = 18;
    pub const IOC_PB05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PB05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PB06_FUNC_CTL_GPIO_B_06: u8 = 0;
    pub const IOC_PB06_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PB06_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PB06_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PB06_FUNC_CTL_TRGM0_P_02: u8 = 18;
    pub const IOC_PB06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PB07_FUNC_CTL_GPIO_B_07: u8 = 0;
    pub const IOC_PB07_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PB07_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PB07_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PB07_FUNC_CTL_TRGM0_P_03: u8 = 18;
    pub const IOC_PB07_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PB08_FUNC_CTL_ACMP_COMP_0: u8 = 16;
    pub const IOC_PB08_FUNC_CTL_GPIO_B_08: u8 = 0;
    pub const IOC_PB08_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PB08_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PB08_FUNC_CTL_SPI2_CS_2: u8 = 5;
    pub const IOC_PB08_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PB08_FUNC_CTL_USB0_ID: u8 = 25;
    pub const IOC_PB09_FUNC_CTL_ACMP_COMP_1: u8 = 16;
    pub const IOC_PB09_FUNC_CTL_GPIO_B_09: u8 = 0;
    pub const IOC_PB09_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PB09_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PB09_FUNC_CTL_SPI2_CS_1: u8 = 5;
    pub const IOC_PB09_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PB09_FUNC_CTL_USB0_OC: u8 = 25;
    pub const IOC_PB10_FUNC_CTL_ACMP_COMP_0: u8 = 16;
    pub const IOC_PB10_FUNC_CTL_GPIO_B_10: u8 = 0;
    pub const IOC_PB10_FUNC_CTL_GPTMR0_COMP_2: u8 = 1;
    pub const IOC_PB10_FUNC_CTL_SPI2_CS_0: u8 = 5;
    pub const IOC_PB10_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PB10_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PB10_FUNC_CTL_USB0_PWR: u8 = 25;
    pub const IOC_PB11_FUNC_CTL_ACMP_COMP_1: u8 = 16;
    pub const IOC_PB11_FUNC_CTL_GPIO_B_11: u8 = 0;
    pub const IOC_PB11_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PB11_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PB12_FUNC_CTL_GPIO_B_12: u8 = 0;
    pub const IOC_PB12_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PB12_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PB12_FUNC_CTL_TRGM0_P_00: u8 = 18;
    pub const IOC_PB12_FUNC_CTL_UART3_CTS: u8 = 3;
    pub const IOC_PB13_FUNC_CTL_GPIO_B_13: u8 = 0;
    pub const IOC_PB13_FUNC_CTL_GPTMR1_COMP_3: u8 = 1;
    pub const IOC_PB13_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PB13_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PB13_FUNC_CTL_TRGM0_P_01: u8 = 18;
    pub const IOC_PB13_FUNC_CTL_UART3_DE: u8 = 2;
    pub const IOC_PB13_FUNC_CTL_UART3_RTS: u8 = 3;
    pub const IOC_PB14_FUNC_CTL_GPIO_B_14: u8 = 0;
    pub const IOC_PB14_FUNC_CTL_SPI2_DAT2: u8 = 5;
    pub const IOC_PB14_FUNC_CTL_TRGM0_P_02: u8 = 18;
    pub const IOC_PB14_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PB15_FUNC_CTL_GPIO_B_15: u8 = 0;
    pub const IOC_PB15_FUNC_CTL_GPTMR0_COMP_3: u8 = 1;
    pub const IOC_PB15_FUNC_CTL_SPI2_DAT3: u8 = 5;
    pub const IOC_PB15_FUNC_CTL_TRGM0_P_03: u8 = 18;
    pub const IOC_PB15_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PX00_FUNC_CTL_GPIO_X_00: u8 = 0;
    pub const IOC_PX00_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
    pub const IOC_PX01_FUNC_CTL_GPIO_X_01: u8 = 0;
    pub const IOC_PX01_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PX02_FUNC_CTL_GPIO_X_02: u8 = 0;
    pub const IOC_PX02_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PX02_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PX03_FUNC_CTL_GPIO_X_03: u8 = 0;
    pub const IOC_PX03_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PX03_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PX04_FUNC_CTL_GPIO_X_04: u8 = 0;
    pub const IOC_PX04_FUNC_CTL_SPI1_CS_0: u8 = 5;
    pub const IOC_PX04_FUNC_CTL_XPI0_CA_CS1: u8 = 14;
    pub const IOC_PX05_FUNC_CTL_GPIO_X_05: u8 = 0;
    pub const IOC_PX05_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PX05_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PX06_FUNC_CTL_GPIO_X_06: u8 = 0;
    pub const IOC_PX06_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PX06_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PX06_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PX07_FUNC_CTL_GPIO_X_07: u8 = 0;
    pub const IOC_PX07_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PX07_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PX07_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PY00_FUNC_CTL_GPIO_Y_00: u8 = 0;
    pub const IOC_PY00_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PY00_FUNC_CTL_USB0_ID: u8 = 25;
    pub const IOC_PY01_FUNC_CTL_EWDG0_RST: u8 = 24;
    pub const IOC_PY01_FUNC_CTL_GPIO_Y_01: u8 = 0;
    pub const IOC_PY01_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PY01_FUNC_CTL_USB0_OC: u8 = 25;
    pub const IOC_PY02_FUNC_CTL_ACMP_COMP_0: u8 = 18;
    pub const IOC_PY02_FUNC_CTL_EWDG1_RST: u8 = 24;
    pub const IOC_PY02_FUNC_CTL_GPIO_Y_02: u8 = 0;
    pub const IOC_PY02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PY02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PY02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PY02_FUNC_CTL_USB0_PWR: u8 = 25;
    pub const IOC_PY03_FUNC_CTL_ACMP_COMP_1: u8 = 18;
    pub const IOC_PY03_FUNC_CTL_GPIO_Y_03: u8 = 0;
    pub const IOC_PY03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PY03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PY04_FUNC_CTL_GPIO_Y_04: u8 = 0;
    pub const IOC_PY04_FUNC_CTL_SPI2_CS_0: u8 = 5;
    pub const IOC_PY04_FUNC_CTL_TRGM0_P_04: u8 = 18;
    pub const IOC_PY04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PY05_FUNC_CTL_EWDG0_RST: u8 = 24;
    pub const IOC_PY05_FUNC_CTL_GPIO_Y_05: u8 = 0;
    pub const IOC_PY05_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PY05_FUNC_CTL_TRGM0_P_05: u8 = 18;
    pub const IOC_PY05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PY05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PY06_FUNC_CTL_EWDG1_RST: u8 = 24;
    pub const IOC_PY06_FUNC_CTL_GPIO_Y_06: u8 = 0;
    pub const IOC_PY06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PY06_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PY06_FUNC_CTL_TRGM0_P_06: u8 = 18;
    pub const IOC_PY06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PY07_FUNC_CTL_GPIO_Y_07: u8 = 0;
    pub const IOC_PY07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PY07_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PY07_FUNC_CTL_TRGM0_P_07: u8 = 18;
    pub const IOC_PY07_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const PIOC_PY00_FUNC_CTL_PGPIO_Y_00: u8 = 0;
    pub const PIOC_PY00_FUNC_CTL_PTMR_COMP_0: u8 = 2;
    pub const PIOC_PY00_FUNC_CTL_PUART_TXD: u8 = 1;
    pub const PIOC_PY00_FUNC_CTL_SOC_GPIO_Y_00: u8 = 3;
    pub const PIOC_PY01_FUNC_CTL_PGPIO_Y_01: u8 = 0;
    pub const PIOC_PY01_FUNC_CTL_PTMR_COMP_1: u8 = 2;
    pub const PIOC_PY01_FUNC_CTL_PUART_RXD: u8 = 1;
    pub const PIOC_PY01_FUNC_CTL_SOC_GPIO_Y_01: u8 = 3;
    pub const PIOC_PY02_FUNC_CTL_PGPIO_Y_02: u8 = 0;
    pub const PIOC_PY02_FUNC_CTL_PTMR_COMP_2: u8 = 2;
    pub const PIOC_PY02_FUNC_CTL_PUART_RTS: u8 = 1;
    pub const PIOC_PY02_FUNC_CTL_SOC_GPIO_Y_02: u8 = 3;
    pub const PIOC_PY03_FUNC_CTL_PGPIO_Y_03: u8 = 0;
    pub const PIOC_PY03_FUNC_CTL_PTMR_COMP_3: u8 = 2;
    pub const PIOC_PY03_FUNC_CTL_PUART_CTS: u8 = 1;
    pub const PIOC_PY03_FUNC_CTL_SOC_GPIO_Y_03: u8 = 3;
    pub const PIOC_PY04_FUNC_CTL_PGPIO_Y_04: u8 = 0;
    pub const PIOC_PY04_FUNC_CTL_PTMR_COMP_0: u8 = 2;
    pub const PIOC_PY04_FUNC_CTL_SOC_GPIO_Y_04: u8 = 3;
    pub const PIOC_PY05_FUNC_CTL_PEWDG_RST: u8 = 1;
    pub const PIOC_PY05_FUNC_CTL_PGPIO_Y_05: u8 = 0;
    pub const PIOC_PY05_FUNC_CTL_PTMR_CAPT_0: u8 = 2;
    pub const PIOC_PY05_FUNC_CTL_SOC_GPIO_Y_05: u8 = 3;
    pub const PIOC_PY06_FUNC_CTL_PGPIO_Y_06: u8 = 0;
    pub const PIOC_PY06_FUNC_CTL_PTMR_COMP_1: u8 = 2;
    pub const PIOC_PY06_FUNC_CTL_SOC_GPIO_Y_06: u8 = 3;
    pub const PIOC_PY07_FUNC_CTL_PGPIO_Y_07: u8 = 0;
    pub const PIOC_PY07_FUNC_CTL_PTMR_CAPT_1: u8 = 2;
    pub const PIOC_PY07_FUNC_CTL_SOC_GPIO_Y_07: u8 = 3;
}
pub mod trgmmux {
    //! `TRGMMUX` definitions
}
