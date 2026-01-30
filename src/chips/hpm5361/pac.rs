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
    #[doc = "42 - SEI0_0"]
    SEI0_0 = 42,
    #[doc = "43 - MMC0"]
    MMC0 = 43,
    #[doc = "44 - TRGMUX0"]
    TRGMUX0 = 44,
    #[doc = "45 - PWM1"]
    PWM1 = 45,
    #[doc = "46 - QEI1"]
    QEI1 = 46,
    #[doc = "47 - SEI0_1"]
    SEI0_1 = 47,
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
        fn SEI0_0();
        fn MMC0();
        fn TRGMUX0();
        fn PWM1();
        fn QEI1();
        fn SEI0_1();
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
        Vector { _handler: SEI0_0 },
        Vector { _handler: MMC0 },
        Vector { _handler: TRGMUX0 },
        Vector { _handler: PWM1 },
        Vector { _handler: QEI1 },
        Vector { _handler: SEI0_1 },
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
pub const EWDG0: ewdg::Ewdg = unsafe { ewdg::Ewdg::from_ptr(0xf00b_0000usize as _) };
pub const EWDG1: ewdg::Ewdg = unsafe { ewdg::Ewdg::from_ptr(0xf00b_4000usize as _) };
pub const DMAMUX: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0xf00c_4000usize as _) };
pub const HDMA: dma::Dma = unsafe { dma::Dma::from_ptr(0xf00c_8000usize as _) };
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf00d_0000usize as _) };
pub const GPIOM: gpiom::Gpiom = unsafe { gpiom::Gpiom::from_ptr(0xf00d_8000usize as _) };
pub const MCAN0: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf028_0000usize as _) };
pub const MCAN1: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf028_4000usize as _) };
pub const MCAN2: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf028_8000usize as _) };
pub const MCAN3: mcan::Mcan = unsafe { mcan::Mcan::from_ptr(0xf028_c000usize as _) };
pub const PTPC: ptpc::Ptpc = unsafe { ptpc::Ptpc::from_ptr(0xf02f_c000usize as _) };
pub const QEI0: qei::Qei = unsafe { qei::Qei::from_ptr(0xf030_0000usize as _) };
pub const QEI1: qei::Qei = unsafe { qei::Qei::from_ptr(0xf030_4000usize as _) };
pub const QEO0: qeo::Qeo = unsafe { qeo::Qeo::from_ptr(0xf030_8000usize as _) };
pub const QEO1: qeo::Qeo = unsafe { qeo::Qeo::from_ptr(0xf030_c000usize as _) };
pub const MMC0: mmc::Mmc = unsafe { mmc::Mmc::from_ptr(0xf031_0000usize as _) };
pub const MMC1: mmc::Mmc = unsafe { mmc::Mmc::from_ptr(0xf031_4000usize as _) };
pub const PWM0: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0xf031_8000usize as _) };
pub const PWM1: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0xf031_c000usize as _) };
pub const RDC0: rdc::Rdc = unsafe { rdc::Rdc::from_ptr(0xf032_0000usize as _) };
pub const PLB: plb::Plb = unsafe { plb::Plb::from_ptr(0xf032_4000usize as _) };
pub const SYNT: synt::Synt = unsafe { synt::Synt::from_ptr(0xf032_8000usize as _) };
pub const SEI: sei::Sei = unsafe { sei::Sei::from_ptr(0xf032_c000usize as _) };
pub const TRGM0: trgm::Trgm = unsafe { trgm::Trgm::from_ptr(0xf033_c000usize as _) };
pub const XPI0: xpi::Xpi = unsafe { xpi::Xpi::from_ptr(0xf300_0000usize as _) };
pub const USB0: usb::Usb = unsafe { usb::Usb::from_ptr(0xf300_c000usize as _) };
pub const SDP: sdp::Sdp = unsafe { sdp::Sdp::from_ptr(0xf304_0000usize as _) };
pub const SEC: sec::Sec = unsafe { sec::Sec::from_ptr(0xf304_4000usize as _) };
pub const PMON: pmon::Pmon = unsafe { pmon::Pmon::from_ptr(0xf304_8000usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0xf304_c000usize as _) };
pub const OTP: otp::Otp = unsafe { otp::Otp::from_ptr(0xf305_0000usize as _) };
pub const KEYM: keym::Keym = unsafe { keym::Keym::from_ptr(0xf305_4000usize as _) };
pub const ADC0: adc16::Adc = unsafe { adc16::Adc::from_ptr(0xf308_0000usize as _) };
pub const ADC1: adc16::Adc = unsafe { adc16::Adc::from_ptr(0xf308_4000usize as _) };
pub const DAC0: dac::Dac = unsafe { dac::Dac::from_ptr(0xf309_0000usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0xf309_4000usize as _) };
pub const OPAMP0: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0xf30a_0000usize as _) };
pub const OPAMP1: opamp::Opamp = unsafe { opamp::Opamp::from_ptr(0xf30a_4000usize as _) };
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
            7 => Ok(Self::GPTMR2),
            8 => Ok(Self::GPTMR3),
            13 => Ok(Self::UART0),
            14 => Ok(Self::UART1),
            15 => Ok(Self::UART2),
            16 => Ok(Self::UART3),
            17 => Ok(Self::UART4),
            18 => Ok(Self::UART5),
            19 => Ok(Self::UART6),
            20 => Ok(Self::UART7),
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
            35 => Ok(Self::MCAN0),
            36 => Ok(Self::MCAN1),
            37 => Ok(Self::MCAN2),
            38 => Ok(Self::MCAN3),
            39 => Ok(Self::PTPC),
            40 => Ok(Self::PWM0),
            41 => Ok(Self::QEI0),
            42 => Ok(Self::SEI0_0),
            43 => Ok(Self::MMC0),
            44 => Ok(Self::TRGMUX0),
            45 => Ok(Self::PWM1),
            46 => Ok(Self::QEI1),
            47 => Ok(Self::SEI0_1),
            48 => Ok(Self::MMC1),
            49 => Ok(Self::TRGMUX1),
            50 => Ok(Self::RDC),
            51 => Ok(Self::USB0),
            52 => Ok(Self::XPI0),
            53 => Ok(Self::SDP),
            54 => Ok(Self::PSEC),
            55 => Ok(Self::SECMON),
            56 => Ok(Self::RNG),
            57 => Ok(Self::FUSE),
            58 => Ok(Self::ADC0),
            59 => Ok(Self::ADC1),
            60 => Ok(Self::DAC0),
            61 => Ok(Self::DAC1),
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
#[path = "../../peripherals/dac_v53.rs"]
pub mod dac;
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
#[path = "../../peripherals/keym_common.rs"]
pub mod keym;
#[path = "../../peripherals/mbx_common.rs"]
pub mod mbx;
#[path = "../../peripherals/mcan_v53.rs"]
pub mod mcan;
#[path = "../../peripherals/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../../peripherals/mmc_v53.rs"]
pub mod mmc;
#[path = "../../peripherals/opamp_v53.rs"]
pub mod opamp;
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
#[path = "../../peripherals/pwm_v53.rs"]
pub mod pwm;
#[path = "../../peripherals/qei_v53.rs"]
pub mod qei;
#[path = "../../peripherals/qeo_v53.rs"]
pub mod qeo;
#[path = "../../peripherals/rdc_v53.rs"]
pub mod rdc;
#[path = "../../peripherals/rng_common.rs"]
pub mod rng;
#[path = "../../peripherals/sdp_v53.rs"]
pub mod sdp;
#[path = "../../peripherals/sec_common.rs"]
pub mod sec;
#[path = "../../peripherals/sei_v53.rs"]
pub mod sei;
#[path = "../../peripherals/spi_v53.rs"]
pub mod spi;
#[path = "../../peripherals/synt_v53.rs"]
pub mod synt;
#[path = "../../peripherals/sysctl_v53.rs"]
pub mod sysctl;
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
    pub const IOC_PA00_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PA00_FUNC_CTL_PWM0_FAULT_0: u8 = 16;
    pub const IOC_PA00_FUNC_CTL_PWM1_FAULT_0: u8 = 19;
    pub const IOC_PA00_FUNC_CTL_PWM1_P_0: u8 = 17;
    pub const IOC_PA00_FUNC_CTL_SYSCTL_CLK_OBS_0: u8 = 24;
    pub const IOC_PA00_FUNC_CTL_TRGM0_P_00: u8 = 18;
    pub const IOC_PA00_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PA01_FUNC_CTL_ACMP_COMP_0: u8 = 19;
    pub const IOC_PA01_FUNC_CTL_GPIO_A_01: u8 = 0;
    pub const IOC_PA01_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PA01_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PA01_FUNC_CTL_PWM0_FAULT_1: u8 = 16;
    pub const IOC_PA01_FUNC_CTL_PWM1_P_1: u8 = 17;
    pub const IOC_PA01_FUNC_CTL_SYSCTL_CLK_OBS_1: u8 = 24;
    pub const IOC_PA01_FUNC_CTL_TRGM0_P_01: u8 = 18;
    pub const IOC_PA01_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PA02_FUNC_CTL_ACMP_COMP_0: u8 = 16;
    pub const IOC_PA02_FUNC_CTL_ACMP_COMP_1: u8 = 19;
    pub const IOC_PA02_FUNC_CTL_GPIO_A_02: u8 = 0;
    pub const IOC_PA02_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PA02_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PA02_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PA02_FUNC_CTL_PWM1_P_2: u8 = 17;
    pub const IOC_PA02_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PA02_FUNC_CTL_SYSCTL_CLK_OBS_2: u8 = 24;
    pub const IOC_PA02_FUNC_CTL_TRGM0_P_02: u8 = 18;
    pub const IOC_PA02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PA02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PA03_FUNC_CTL_ACMP_COMP_1: u8 = 16;
    pub const IOC_PA03_FUNC_CTL_GPIO_A_03: u8 = 0;
    pub const IOC_PA03_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PA03_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PA03_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PA03_FUNC_CTL_PWM1_FAULT_1: u8 = 19;
    pub const IOC_PA03_FUNC_CTL_PWM1_P_3: u8 = 17;
    pub const IOC_PA03_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PA03_FUNC_CTL_SPI3_CS_3: u8 = 5;
    pub const IOC_PA03_FUNC_CTL_SYSCTL_CLK_OBS_3: u8 = 24;
    pub const IOC_PA03_FUNC_CTL_TRGM0_P_03: u8 = 18;
    pub const IOC_PA03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PA04_FUNC_CTL_GPIO_A_04: u8 = 0;
    pub const IOC_PA04_FUNC_CTL_JTAG_TDO: u8 = 24;
    pub const IOC_PA04_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PA04_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PA04_FUNC_CTL_PWM1_P_4: u8 = 17;
    pub const IOC_PA04_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PA04_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PA04_FUNC_CTL_RDC0_EXC_P: u8 = 19;
    pub const IOC_PA04_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PA04_FUNC_CTL_SPI0_CS_0: u8 = 5;
    pub const IOC_PA04_FUNC_CTL_TRGM0_P_04: u8 = 18;
    pub const IOC_PA04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PA05_FUNC_CTL_GPIO_A_05: u8 = 0;
    pub const IOC_PA05_FUNC_CTL_GPTMR1_COMP_2: u8 = 1;
    pub const IOC_PA05_FUNC_CTL_JTAG_TDI: u8 = 24;
    pub const IOC_PA05_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PA05_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PA05_FUNC_CTL_PWM1_P_5: u8 = 17;
    pub const IOC_PA05_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PA05_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PA05_FUNC_CTL_RDC0_EXC_N: u8 = 19;
    pub const IOC_PA05_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PA05_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PA05_FUNC_CTL_TRGM0_P_05: u8 = 18;
    pub const IOC_PA05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PA05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PA06_FUNC_CTL_GPIO_A_06: u8 = 0;
    pub const IOC_PA06_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PA06_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PA06_FUNC_CTL_JTAG_TCK: u8 = 24;
    pub const IOC_PA06_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PA06_FUNC_CTL_PWM1_P_6: u8 = 17;
    pub const IOC_PA06_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PA06_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PA06_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PA06_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PA06_FUNC_CTL_TRGM0_P_06: u8 = 18;
    pub const IOC_PA06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PA07_FUNC_CTL_GPIO_A_07: u8 = 0;
    pub const IOC_PA07_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PA07_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PA07_FUNC_CTL_JTAG_TMS: u8 = 24;
    pub const IOC_PA07_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PA07_FUNC_CTL_PWM1_P_7: u8 = 17;
    pub const IOC_PA07_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PA07_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PA07_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PA07_FUNC_CTL_TRGM0_P_07: u8 = 18;
    pub const IOC_PA07_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PA08_FUNC_CTL_GPIO_A_08: u8 = 0;
    pub const IOC_PA08_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PA08_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PA08_FUNC_CTL_JTAG_TRST: u8 = 24;
    pub const IOC_PA08_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PA08_FUNC_CTL_PWM0_FAULT_0: u8 = 18;
    pub const IOC_PA08_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PA08_FUNC_CTL_SPI3_CS_2: u8 = 5;
    pub const IOC_PA08_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PA09_FUNC_CTL_GPIO_A_09: u8 = 0;
    pub const IOC_PA09_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PA09_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PA09_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PA09_FUNC_CTL_PWM0_FAULT_1: u8 = 18;
    pub const IOC_PA09_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PA09_FUNC_CTL_SOC_REF0: u8 = 24;
    pub const IOC_PA09_FUNC_CTL_SPI3_CS_1: u8 = 5;
    pub const IOC_PA09_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PA10_FUNC_CTL_ACMP_COMP_0: u8 = 18;
    pub const IOC_PA10_FUNC_CTL_GPIO_A_10: u8 = 0;
    pub const IOC_PA10_FUNC_CTL_GPTMR0_COMP_2: u8 = 1;
    pub const IOC_PA10_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PA10_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PA10_FUNC_CTL_PWM1_FAULT_0: u8 = 17;
    pub const IOC_PA10_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PA10_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PA10_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PA10_FUNC_CTL_SPI3_CS_0: u8 = 5;
    pub const IOC_PA10_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PA10_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PA11_FUNC_CTL_ACMP_COMP_1: u8 = 18;
    pub const IOC_PA11_FUNC_CTL_EWDG0_RST: u8 = 24;
    pub const IOC_PA11_FUNC_CTL_GPIO_A_11: u8 = 0;
    pub const IOC_PA11_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PA11_FUNC_CTL_PWM1_FAULT_1: u8 = 17;
    pub const IOC_PA11_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PA11_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PA11_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PA11_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PA11_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PA12_FUNC_CTL_GPIO_A_12: u8 = 0;
    pub const IOC_PA12_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PA12_FUNC_CTL_PWM0_FAULT_0: u8 = 18;
    pub const IOC_PA12_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PA12_FUNC_CTL_PWM1_FAULT_0: u8 = 17;
    pub const IOC_PA12_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PA12_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PA12_FUNC_CTL_RDC0_EXC_P: u8 = 19;
    pub const IOC_PA12_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PA12_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PA12_FUNC_CTL_UART3_CTS: u8 = 3;
    pub const IOC_PA13_FUNC_CTL_GPIO_A_13: u8 = 0;
    pub const IOC_PA13_FUNC_CTL_GPTMR1_COMP_3: u8 = 1;
    pub const IOC_PA13_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PA13_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PA13_FUNC_CTL_PWM0_FAULT_1: u8 = 18;
    pub const IOC_PA13_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PA13_FUNC_CTL_PWM1_FAULT_1: u8 = 17;
    pub const IOC_PA13_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PA13_FUNC_CTL_RDC0_EXC_N: u8 = 19;
    pub const IOC_PA13_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PA13_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PA13_FUNC_CTL_UART3_DE: u8 = 2;
    pub const IOC_PA13_FUNC_CTL_UART3_RTS: u8 = 3;
    pub const IOC_PA14_FUNC_CTL_ACMP_COMP_0: u8 = 18;
    pub const IOC_PA14_FUNC_CTL_EWDG1_RST: u8 = 24;
    pub const IOC_PA14_FUNC_CTL_GPIO_A_14: u8 = 0;
    pub const IOC_PA14_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PA14_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PA14_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PA14_FUNC_CTL_SPI3_DAT2: u8 = 5;
    pub const IOC_PA14_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PA15_FUNC_CTL_ACMP_COMP_1: u8 = 18;
    pub const IOC_PA15_FUNC_CTL_GPIO_A_15: u8 = 0;
    pub const IOC_PA15_FUNC_CTL_GPTMR0_COMP_3: u8 = 1;
    pub const IOC_PA15_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PA15_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PA15_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PA15_FUNC_CTL_SOC_REF0: u8 = 24;
    pub const IOC_PA15_FUNC_CTL_SPI3_DAT3: u8 = 5;
    pub const IOC_PA15_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PA16_FUNC_CTL_GPIO_A_16: u8 = 0;
    pub const IOC_PA16_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PA16_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PA16_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PA16_FUNC_CTL_PWM1_P_0: u8 = 17;
    pub const IOC_PA16_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PA16_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PA16_FUNC_CTL_TRGM0_P_04: u8 = 18;
    pub const IOC_PA16_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PA17_FUNC_CTL_GPIO_A_17: u8 = 0;
    pub const IOC_PA17_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PA17_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PA17_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PA17_FUNC_CTL_PWM1_P_1: u8 = 17;
    pub const IOC_PA17_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PA17_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PA17_FUNC_CTL_TRGM0_P_05: u8 = 18;
    pub const IOC_PA17_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PA18_FUNC_CTL_GPIO_A_18: u8 = 0;
    pub const IOC_PA18_FUNC_CTL_GPTMR3_COMP_1: u8 = 1;
    pub const IOC_PA18_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PA18_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PA18_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PA18_FUNC_CTL_PWM1_P_2: u8 = 17;
    pub const IOC_PA18_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PA18_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PA18_FUNC_CTL_TRGM0_P_06: u8 = 18;
    pub const IOC_PA18_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PA18_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PA19_FUNC_CTL_GPIO_A_19: u8 = 0;
    pub const IOC_PA19_FUNC_CTL_GPTMR3_CAPT_1: u8 = 1;
    pub const IOC_PA19_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PA19_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PA19_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PA19_FUNC_CTL_PWM1_P_3: u8 = 17;
    pub const IOC_PA19_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PA19_FUNC_CTL_SPI1_CS_3: u8 = 5;
    pub const IOC_PA19_FUNC_CTL_TRGM0_P_07: u8 = 18;
    pub const IOC_PA19_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PA20_FUNC_CTL_GPIO_A_20: u8 = 0;
    pub const IOC_PA20_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PA20_FUNC_CTL_PWM0_FAULT_0: u8 = 16;
    pub const IOC_PA20_FUNC_CTL_PWM1_P_4: u8 = 17;
    pub const IOC_PA20_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PA20_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PA20_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PA20_FUNC_CTL_SPI2_CS_0: u8 = 5;
    pub const IOC_PA20_FUNC_CTL_TRGM0_P_00: u8 = 18;
    pub const IOC_PA20_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PA21_FUNC_CTL_GPIO_A_21: u8 = 0;
    pub const IOC_PA21_FUNC_CTL_GPTMR3_COMP_2: u8 = 1;
    pub const IOC_PA21_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PA21_FUNC_CTL_PWM0_FAULT_1: u8 = 16;
    pub const IOC_PA21_FUNC_CTL_PWM1_P_5: u8 = 17;
    pub const IOC_PA21_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PA21_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PA21_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PA21_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PA21_FUNC_CTL_TRGM0_P_01: u8 = 18;
    pub const IOC_PA21_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PA21_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PA22_FUNC_CTL_GPIO_A_22: u8 = 0;
    pub const IOC_PA22_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PA22_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PA22_FUNC_CTL_PWM1_FAULT_0: u8 = 19;
    pub const IOC_PA22_FUNC_CTL_PWM1_P_6: u8 = 17;
    pub const IOC_PA22_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PA22_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PA22_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PA22_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PA22_FUNC_CTL_TRGM0_P_02: u8 = 18;
    pub const IOC_PA22_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PA23_FUNC_CTL_GPIO_A_23: u8 = 0;
    pub const IOC_PA23_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PA23_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PA23_FUNC_CTL_PWM1_FAULT_1: u8 = 19;
    pub const IOC_PA23_FUNC_CTL_PWM1_P_7: u8 = 17;
    pub const IOC_PA23_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PA23_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PA23_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PA23_FUNC_CTL_TRGM0_P_03: u8 = 18;
    pub const IOC_PA23_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PA24_FUNC_CTL_GPIO_A_24: u8 = 0;
    pub const IOC_PA24_FUNC_CTL_GPTMR2_COMP_1: u8 = 1;
    pub const IOC_PA24_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PA24_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PA24_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PA24_FUNC_CTL_PWM1_P_0: u8 = 17;
    pub const IOC_PA24_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PA24_FUNC_CTL_SPI1_CS_2: u8 = 5;
    pub const IOC_PA24_FUNC_CTL_TRGM0_P_00: u8 = 18;
    pub const IOC_PA24_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PA24_FUNC_CTL_XPI0_CA_CS1: u8 = 14;
    pub const IOC_PA25_FUNC_CTL_GPIO_A_25: u8 = 0;
    pub const IOC_PA25_FUNC_CTL_GPTMR2_CAPT_1: u8 = 1;
    pub const IOC_PA25_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PA25_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PA25_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PA25_FUNC_CTL_PWM1_P_1: u8 = 17;
    pub const IOC_PA25_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PA25_FUNC_CTL_SPI1_CS_1: u8 = 5;
    pub const IOC_PA25_FUNC_CTL_TRGM0_P_01: u8 = 18;
    pub const IOC_PA25_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PA25_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PA26_FUNC_CTL_GPIO_A_26: u8 = 0;
    pub const IOC_PA26_FUNC_CTL_GPTMR2_COMP_2: u8 = 1;
    pub const IOC_PA26_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PA26_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PA26_FUNC_CTL_PWM1_P_2: u8 = 17;
    pub const IOC_PA26_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PA26_FUNC_CTL_QEO0_A: u8 = 21;
    pub const IOC_PA26_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PA26_FUNC_CTL_SPI1_CS_0: u8 = 5;
    pub const IOC_PA26_FUNC_CTL_SYSCTL_CLK_OBS_0: u8 = 24;
    pub const IOC_PA26_FUNC_CTL_TRGM0_P_02: u8 = 18;
    pub const IOC_PA26_FUNC_CTL_UART6_DE: u8 = 2;
    pub const IOC_PA26_FUNC_CTL_UART6_RTS: u8 = 3;
    pub const IOC_PA26_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PA27_FUNC_CTL_GPIO_A_27: u8 = 0;
    pub const IOC_PA27_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PA27_FUNC_CTL_PWM1_P_3: u8 = 17;
    pub const IOC_PA27_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PA27_FUNC_CTL_QEO0_B: u8 = 21;
    pub const IOC_PA27_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PA27_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PA27_FUNC_CTL_SYSCTL_CLK_OBS_1: u8 = 24;
    pub const IOC_PA27_FUNC_CTL_TRGM0_P_03: u8 = 18;
    pub const IOC_PA27_FUNC_CTL_UART6_CTS: u8 = 3;
    pub const IOC_PA27_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PA28_FUNC_CTL_GPIO_A_28: u8 = 0;
    pub const IOC_PA28_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PA28_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PA28_FUNC_CTL_PWM1_P_4: u8 = 17;
    pub const IOC_PA28_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PA28_FUNC_CTL_QEO0_Z: u8 = 21;
    pub const IOC_PA28_FUNC_CTL_RDC0_EXC_P: u8 = 19;
    pub const IOC_PA28_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PA28_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PA28_FUNC_CTL_SYSCTL_CLK_OBS_2: u8 = 24;
    pub const IOC_PA28_FUNC_CTL_TRGM0_P_04: u8 = 18;
    pub const IOC_PA28_FUNC_CTL_UART7_CTS: u8 = 3;
    pub const IOC_PA28_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PA29_FUNC_CTL_GPIO_A_29: u8 = 0;
    pub const IOC_PA29_FUNC_CTL_GPTMR3_COMP_3: u8 = 1;
    pub const IOC_PA29_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PA29_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PA29_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PA29_FUNC_CTL_PWM1_P_5: u8 = 17;
    pub const IOC_PA29_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PA29_FUNC_CTL_RDC0_EXC_N: u8 = 19;
    pub const IOC_PA29_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PA29_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PA29_FUNC_CTL_SYSCTL_CLK_OBS_3: u8 = 24;
    pub const IOC_PA29_FUNC_CTL_TRGM0_P_05: u8 = 18;
    pub const IOC_PA29_FUNC_CTL_UART7_DE: u8 = 2;
    pub const IOC_PA29_FUNC_CTL_UART7_RTS: u8 = 3;
    pub const IOC_PA29_FUNC_CTL_USB0_OC: u8 = 25;
    pub const IOC_PA29_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
    pub const IOC_PA30_FUNC_CTL_GPIO_A_30: u8 = 0;
    pub const IOC_PA30_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PA30_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PA30_FUNC_CTL_PWM1_P_6: u8 = 17;
    pub const IOC_PA30_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PA30_FUNC_CTL_SOC_REF0: u8 = 24;
    pub const IOC_PA30_FUNC_CTL_SPI1_DAT2: u8 = 5;
    pub const IOC_PA30_FUNC_CTL_TRGM0_P_06: u8 = 18;
    pub const IOC_PA30_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PA30_FUNC_CTL_USB0_PWR: u8 = 25;
    pub const IOC_PA30_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PA31_FUNC_CTL_GPIO_A_31: u8 = 0;
    pub const IOC_PA31_FUNC_CTL_GPTMR2_COMP_3: u8 = 1;
    pub const IOC_PA31_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PA31_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PA31_FUNC_CTL_PWM1_P_7: u8 = 17;
    pub const IOC_PA31_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PA31_FUNC_CTL_SPI1_DAT3: u8 = 5;
    pub const IOC_PA31_FUNC_CTL_TRGM0_P_07: u8 = 18;
    pub const IOC_PA31_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PA31_FUNC_CTL_USB0_ID: u8 = 25;
    pub const IOC_PA31_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PB00_FUNC_CTL_ACMP_COMP_0: u8 = 19;
    pub const IOC_PB00_FUNC_CTL_GPIO_B_00: u8 = 0;
    pub const IOC_PB00_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PB00_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PB00_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PB00_FUNC_CTL_PWM1_FAULT_0: u8 = 17;
    pub const IOC_PB00_FUNC_CTL_TRGM0_P_04: u8 = 18;
    pub const IOC_PB00_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PB01_FUNC_CTL_ACMP_COMP_1: u8 = 19;
    pub const IOC_PB01_FUNC_CTL_GPIO_B_01: u8 = 0;
    pub const IOC_PB01_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PB01_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PB01_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PB01_FUNC_CTL_PWM1_FAULT_1: u8 = 17;
    pub const IOC_PB01_FUNC_CTL_TRGM0_P_05: u8 = 18;
    pub const IOC_PB01_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PB02_FUNC_CTL_ACMP_COMP_1: u8 = 17;
    pub const IOC_PB02_FUNC_CTL_GPIO_B_02: u8 = 0;
    pub const IOC_PB02_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PB02_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PB02_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PB02_FUNC_CTL_PWM0_FAULT_0: u8 = 19;
    pub const IOC_PB02_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PB02_FUNC_CTL_TRGM0_P_06: u8 = 18;
    pub const IOC_PB02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PB02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PB03_FUNC_CTL_ACMP_COMP_0: u8 = 17;
    pub const IOC_PB03_FUNC_CTL_GPIO_B_03: u8 = 0;
    pub const IOC_PB03_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PB03_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PB03_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PB03_FUNC_CTL_PWM0_FAULT_1: u8 = 19;
    pub const IOC_PB03_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PB03_FUNC_CTL_SPI2_CS_3: u8 = 5;
    pub const IOC_PB03_FUNC_CTL_TRGM0_P_07: u8 = 18;
    pub const IOC_PB03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PB04_FUNC_CTL_GPIO_B_04: u8 = 0;
    pub const IOC_PB04_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PB04_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PB04_FUNC_CTL_PWM1_P_0: u8 = 17;
    pub const IOC_PB04_FUNC_CTL_QEI1_A: u8 = 20;
    pub const IOC_PB04_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PB04_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PB04_FUNC_CTL_SPI3_CS_0: u8 = 5;
    pub const IOC_PB04_FUNC_CTL_TRGM0_P_00: u8 = 18;
    pub const IOC_PB04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PB05_FUNC_CTL_GPIO_B_05: u8 = 0;
    pub const IOC_PB05_FUNC_CTL_GPTMR1_COMP_2: u8 = 1;
    pub const IOC_PB05_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PB05_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PB05_FUNC_CTL_PWM1_P_1: u8 = 17;
    pub const IOC_PB05_FUNC_CTL_QEI1_B: u8 = 20;
    pub const IOC_PB05_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PB05_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PB05_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PB05_FUNC_CTL_TRGM0_P_01: u8 = 18;
    pub const IOC_PB05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PB05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PB06_FUNC_CTL_GPIO_B_06: u8 = 0;
    pub const IOC_PB06_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PB06_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PB06_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PB06_FUNC_CTL_PWM1_P_2: u8 = 17;
    pub const IOC_PB06_FUNC_CTL_QEI1_Z: u8 = 20;
    pub const IOC_PB06_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PB06_FUNC_CTL_RDC0_EXC_P: u8 = 19;
    pub const IOC_PB06_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PB06_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PB06_FUNC_CTL_TRGM0_P_02: u8 = 18;
    pub const IOC_PB06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PB07_FUNC_CTL_GPIO_B_07: u8 = 0;
    pub const IOC_PB07_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PB07_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PB07_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PB07_FUNC_CTL_PWM1_P_3: u8 = 17;
    pub const IOC_PB07_FUNC_CTL_QEI1_H0: u8 = 20;
    pub const IOC_PB07_FUNC_CTL_RDC0_EXC_N: u8 = 19;
    pub const IOC_PB07_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PB07_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PB07_FUNC_CTL_TRGM0_P_03: u8 = 18;
    pub const IOC_PB07_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PB08_FUNC_CTL_ACMP_COMP_0: u8 = 16;
    pub const IOC_PB08_FUNC_CTL_GPIO_B_08: u8 = 0;
    pub const IOC_PB08_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PB08_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PB08_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PB08_FUNC_CTL_PWM1_P_4: u8 = 17;
    pub const IOC_PB08_FUNC_CTL_QEI1_H1: u8 = 20;
    pub const IOC_PB08_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PB08_FUNC_CTL_SEI1_DE: u8 = 22;
    pub const IOC_PB08_FUNC_CTL_SPI2_CS_2: u8 = 5;
    pub const IOC_PB08_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PB08_FUNC_CTL_USB0_ID: u8 = 25;
    pub const IOC_PB09_FUNC_CTL_ACMP_COMP_1: u8 = 16;
    pub const IOC_PB09_FUNC_CTL_GPIO_B_09: u8 = 0;
    pub const IOC_PB09_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PB09_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PB09_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PB09_FUNC_CTL_PWM1_P_5: u8 = 17;
    pub const IOC_PB09_FUNC_CTL_QEI1_F: u8 = 20;
    pub const IOC_PB09_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PB09_FUNC_CTL_SEI1_CK: u8 = 22;
    pub const IOC_PB09_FUNC_CTL_SPI2_CS_1: u8 = 5;
    pub const IOC_PB09_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PB09_FUNC_CTL_USB0_OC: u8 = 25;
    pub const IOC_PB10_FUNC_CTL_ACMP_COMP_0: u8 = 16;
    pub const IOC_PB10_FUNC_CTL_GPIO_B_10: u8 = 0;
    pub const IOC_PB10_FUNC_CTL_GPTMR0_COMP_2: u8 = 1;
    pub const IOC_PB10_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PB10_FUNC_CTL_PWM1_P_6: u8 = 17;
    pub const IOC_PB10_FUNC_CTL_QEI0_H1: u8 = 20;
    pub const IOC_PB10_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PB10_FUNC_CTL_SEI1_TX: u8 = 22;
    pub const IOC_PB10_FUNC_CTL_SPI2_CS_0: u8 = 5;
    pub const IOC_PB10_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PB10_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PB10_FUNC_CTL_USB0_PWR: u8 = 25;
    pub const IOC_PB11_FUNC_CTL_ACMP_COMP_1: u8 = 16;
    pub const IOC_PB11_FUNC_CTL_GPIO_B_11: u8 = 0;
    pub const IOC_PB11_FUNC_CTL_PWM1_P_7: u8 = 17;
    pub const IOC_PB11_FUNC_CTL_QEI0_F: u8 = 20;
    pub const IOC_PB11_FUNC_CTL_SEI1_RX: u8 = 22;
    pub const IOC_PB11_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PB11_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PB12_FUNC_CTL_GPIO_B_12: u8 = 0;
    pub const IOC_PB12_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PB12_FUNC_CTL_PWM1_FAULT_0: u8 = 16;
    pub const IOC_PB12_FUNC_CTL_PWM1_P_0: u8 = 17;
    pub const IOC_PB12_FUNC_CTL_QEI0_A: u8 = 20;
    pub const IOC_PB12_FUNC_CTL_QEO1_A: u8 = 21;
    pub const IOC_PB12_FUNC_CTL_SEI0_DE: u8 = 22;
    pub const IOC_PB12_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PB12_FUNC_CTL_TRGM0_P_00: u8 = 18;
    pub const IOC_PB12_FUNC_CTL_UART3_CTS: u8 = 3;
    pub const IOC_PB13_FUNC_CTL_GPIO_B_13: u8 = 0;
    pub const IOC_PB13_FUNC_CTL_GPTMR1_COMP_3: u8 = 1;
    pub const IOC_PB13_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PB13_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PB13_FUNC_CTL_PWM1_FAULT_1: u8 = 16;
    pub const IOC_PB13_FUNC_CTL_PWM1_P_1: u8 = 17;
    pub const IOC_PB13_FUNC_CTL_QEI0_B: u8 = 20;
    pub const IOC_PB13_FUNC_CTL_QEO1_B: u8 = 21;
    pub const IOC_PB13_FUNC_CTL_SEI0_CK: u8 = 22;
    pub const IOC_PB13_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PB13_FUNC_CTL_TRGM0_P_01: u8 = 18;
    pub const IOC_PB13_FUNC_CTL_UART3_DE: u8 = 2;
    pub const IOC_PB13_FUNC_CTL_UART3_RTS: u8 = 3;
    pub const IOC_PB14_FUNC_CTL_GPIO_B_14: u8 = 0;
    pub const IOC_PB14_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PB14_FUNC_CTL_PWM0_FAULT_0: u8 = 16;
    pub const IOC_PB14_FUNC_CTL_PWM1_P_2: u8 = 17;
    pub const IOC_PB14_FUNC_CTL_QEI0_Z: u8 = 20;
    pub const IOC_PB14_FUNC_CTL_QEO1_Z: u8 = 21;
    pub const IOC_PB14_FUNC_CTL_RDC0_EXC_P: u8 = 19;
    pub const IOC_PB14_FUNC_CTL_SEI0_TX: u8 = 22;
    pub const IOC_PB14_FUNC_CTL_SPI2_DAT2: u8 = 5;
    pub const IOC_PB14_FUNC_CTL_TRGM0_P_02: u8 = 18;
    pub const IOC_PB14_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PB15_FUNC_CTL_GPIO_B_15: u8 = 0;
    pub const IOC_PB15_FUNC_CTL_GPTMR0_COMP_3: u8 = 1;
    pub const IOC_PB15_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PB15_FUNC_CTL_PWM0_FAULT_1: u8 = 16;
    pub const IOC_PB15_FUNC_CTL_PWM1_P_3: u8 = 17;
    pub const IOC_PB15_FUNC_CTL_QEI0_H0: u8 = 20;
    pub const IOC_PB15_FUNC_CTL_RDC0_EXC_N: u8 = 19;
    pub const IOC_PB15_FUNC_CTL_SEI0_RX: u8 = 22;
    pub const IOC_PB15_FUNC_CTL_SPI2_DAT3: u8 = 5;
    pub const IOC_PB15_FUNC_CTL_TRGM0_P_03: u8 = 18;
    pub const IOC_PB15_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PX00_FUNC_CTL_GPIO_X_00: u8 = 0;
    pub const IOC_PX00_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PX00_FUNC_CTL_MCAN0_TXD: u8 = 7;
    pub const IOC_PX00_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PX00_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
    pub const IOC_PX01_FUNC_CTL_GPIO_X_01: u8 = 0;
    pub const IOC_PX01_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PX01_FUNC_CTL_MCAN0_RXD: u8 = 7;
    pub const IOC_PX01_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PX01_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PX02_FUNC_CTL_GPIO_X_02: u8 = 0;
    pub const IOC_PX02_FUNC_CTL_GPTMR2_COMP_1: u8 = 1;
    pub const IOC_PX02_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PX02_FUNC_CTL_MCAN0_STBY: u8 = 7;
    pub const IOC_PX02_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PX02_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PX02_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PX03_FUNC_CTL_GPIO_X_03: u8 = 0;
    pub const IOC_PX03_FUNC_CTL_GPTMR2_CAPT_1: u8 = 1;
    pub const IOC_PX03_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PX03_FUNC_CTL_MCAN1_STBY: u8 = 7;
    pub const IOC_PX03_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PX03_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PX04_FUNC_CTL_GPIO_X_04: u8 = 0;
    pub const IOC_PX04_FUNC_CTL_MCAN1_RXD: u8 = 7;
    pub const IOC_PX04_FUNC_CTL_SPI1_CS_0: u8 = 5;
    pub const IOC_PX04_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PX04_FUNC_CTL_XPI0_CA_CS1: u8 = 14;
    pub const IOC_PX05_FUNC_CTL_GPIO_X_05: u8 = 0;
    pub const IOC_PX05_FUNC_CTL_GPTMR2_COMP_2: u8 = 1;
    pub const IOC_PX05_FUNC_CTL_MCAN1_TXD: u8 = 7;
    pub const IOC_PX05_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PX05_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PX05_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PX05_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PX06_FUNC_CTL_GPIO_X_06: u8 = 0;
    pub const IOC_PX06_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PX06_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PX06_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PX06_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PX06_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PX07_FUNC_CTL_GPIO_X_07: u8 = 0;
    pub const IOC_PX07_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PX07_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PX07_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PX07_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PX07_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PY00_FUNC_CTL_GPIO_Y_00: u8 = 0;
    pub const IOC_PY00_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PY00_FUNC_CTL_MCAN2_TXD: u8 = 7;
    pub const IOC_PY00_FUNC_CTL_PWM0_FAULT_0: u8 = 18;
    pub const IOC_PY00_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PY00_FUNC_CTL_PWM1_P_4: u8 = 17;
    pub const IOC_PY00_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PY00_FUNC_CTL_USB0_ID: u8 = 25;
    pub const IOC_PY01_FUNC_CTL_EWDG0_RST: u8 = 24;
    pub const IOC_PY01_FUNC_CTL_GPIO_Y_01: u8 = 0;
    pub const IOC_PY01_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PY01_FUNC_CTL_MCAN2_RXD: u8 = 7;
    pub const IOC_PY01_FUNC_CTL_PWM0_FAULT_1: u8 = 18;
    pub const IOC_PY01_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PY01_FUNC_CTL_PWM1_P_5: u8 = 17;
    pub const IOC_PY01_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PY01_FUNC_CTL_USB0_OC: u8 = 25;
    pub const IOC_PY02_FUNC_CTL_ACMP_COMP_0: u8 = 18;
    pub const IOC_PY02_FUNC_CTL_EWDG1_RST: u8 = 24;
    pub const IOC_PY02_FUNC_CTL_GPIO_Y_02: u8 = 0;
    pub const IOC_PY02_FUNC_CTL_GPTMR3_COMP_1: u8 = 1;
    pub const IOC_PY02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PY02_FUNC_CTL_MCAN2_STBY: u8 = 7;
    pub const IOC_PY02_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PY02_FUNC_CTL_PWM1_FAULT_0: u8 = 19;
    pub const IOC_PY02_FUNC_CTL_PWM1_P_6: u8 = 17;
    pub const IOC_PY02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PY02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PY02_FUNC_CTL_USB0_PWR: u8 = 25;
    pub const IOC_PY03_FUNC_CTL_ACMP_COMP_1: u8 = 18;
    pub const IOC_PY03_FUNC_CTL_GPIO_Y_03: u8 = 0;
    pub const IOC_PY03_FUNC_CTL_GPTMR3_CAPT_1: u8 = 1;
    pub const IOC_PY03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PY03_FUNC_CTL_MCAN3_STBY: u8 = 7;
    pub const IOC_PY03_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PY03_FUNC_CTL_PWM1_FAULT_1: u8 = 19;
    pub const IOC_PY03_FUNC_CTL_PWM1_P_7: u8 = 17;
    pub const IOC_PY03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PY04_FUNC_CTL_GPIO_Y_04: u8 = 0;
    pub const IOC_PY04_FUNC_CTL_MCAN3_RXD: u8 = 7;
    pub const IOC_PY04_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PY04_FUNC_CTL_SPI2_CS_0: u8 = 5;
    pub const IOC_PY04_FUNC_CTL_TRGM0_P_04: u8 = 18;
    pub const IOC_PY04_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PY05_FUNC_CTL_EWDG0_RST: u8 = 24;
    pub const IOC_PY05_FUNC_CTL_GPIO_Y_05: u8 = 0;
    pub const IOC_PY05_FUNC_CTL_GPTMR3_COMP_2: u8 = 1;
    pub const IOC_PY05_FUNC_CTL_MCAN3_TXD: u8 = 7;
    pub const IOC_PY05_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PY05_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PY05_FUNC_CTL_TRGM0_P_05: u8 = 18;
    pub const IOC_PY05_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PY05_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PY06_FUNC_CTL_EWDG1_RST: u8 = 24;
    pub const IOC_PY06_FUNC_CTL_GPIO_Y_06: u8 = 0;
    pub const IOC_PY06_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PY06_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PY06_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PY06_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PY06_FUNC_CTL_TRGM0_P_06: u8 = 18;
    pub const IOC_PY06_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PY07_FUNC_CTL_GPIO_Y_07: u8 = 0;
    pub const IOC_PY07_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PY07_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PY07_FUNC_CTL_PWM0_P_7: u8 = 16;
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
    pub const TRGM0_DMA_SRC_PWM0_CMP0: usize = 0;
    pub const TRGM0_DMA_SRC_PWM0_CMP1: usize = 1;
    pub const TRGM0_DMA_SRC_PWM0_CMP2: usize = 2;
    pub const TRGM0_DMA_SRC_PWM0_CMP3: usize = 3;
    pub const TRGM0_DMA_SRC_PWM0_CMP4: usize = 4;
    pub const TRGM0_DMA_SRC_PWM0_CMP5: usize = 5;
    pub const TRGM0_DMA_SRC_PWM0_CMP6: usize = 6;
    pub const TRGM0_DMA_SRC_PWM0_CMP7: usize = 7;
    pub const TRGM0_DMA_SRC_PWM0_CMP8: usize = 8;
    pub const TRGM0_DMA_SRC_PWM0_CMP9: usize = 9;
    pub const TRGM0_DMA_SRC_PWM0_CMP10: usize = 10;
    pub const TRGM0_DMA_SRC_PWM0_CMP11: usize = 11;
    pub const TRGM0_DMA_SRC_PWM0_CMP12: usize = 12;
    pub const TRGM0_DMA_SRC_PWM0_CMP13: usize = 13;
    pub const TRGM0_DMA_SRC_PWM0_CMP14: usize = 14;
    pub const TRGM0_DMA_SRC_PWM0_CMP15: usize = 15;
    pub const TRGM0_DMA_SRC_PWM0_CMP16: usize = 16;
    pub const TRGM0_DMA_SRC_PWM0_CMP17: usize = 17;
    pub const TRGM0_DMA_SRC_PWM0_CMP18: usize = 18;
    pub const TRGM0_DMA_SRC_PWM0_CMP19: usize = 19;
    pub const TRGM0_DMA_SRC_PWM0_CMP20: usize = 20;
    pub const TRGM0_DMA_SRC_PWM0_CMP21: usize = 21;
    pub const TRGM0_DMA_SRC_PWM0_CMP22: usize = 22;
    pub const TRGM0_DMA_SRC_PWM0_CMP23: usize = 23;
    pub const TRGM0_DMA_SRC_PWM0_RLD: usize = 24;
    pub const TRGM0_DMA_SRC_PWM0_HALFRLD: usize = 25;
    pub const TRGM0_DMA_SRC_PWM0_XRLD: usize = 26;
    pub const TRGM0_DMA_SRC_PWM1_CMP0: usize = 27;
    pub const TRGM0_DMA_SRC_PWM1_CMP1: usize = 28;
    pub const TRGM0_DMA_SRC_PWM1_CMP2: usize = 29;
    pub const TRGM0_DMA_SRC_PWM1_CMP3: usize = 30;
    pub const TRGM0_DMA_SRC_PWM1_CMP4: usize = 31;
    pub const TRGM0_DMA_SRC_PWM1_CMP5: usize = 32;
    pub const TRGM0_DMA_SRC_PWM1_CMP6: usize = 33;
    pub const TRGM0_DMA_SRC_PWM1_CMP7: usize = 34;
    pub const TRGM0_DMA_SRC_PWM1_CMP8: usize = 35;
    pub const TRGM0_DMA_SRC_PWM1_CMP9: usize = 36;
    pub const TRGM0_DMA_SRC_PWM1_CMP10: usize = 37;
    pub const TRGM0_DMA_SRC_PWM1_CMP11: usize = 38;
    pub const TRGM0_DMA_SRC_PWM1_CMP12: usize = 39;
    pub const TRGM0_DMA_SRC_PWM1_CMP13: usize = 40;
    pub const TRGM0_DMA_SRC_PWM1_CMP14: usize = 41;
    pub const TRGM0_DMA_SRC_PWM1_CMP15: usize = 42;
    pub const TRGM0_DMA_SRC_PWM1_CMP16: usize = 43;
    pub const TRGM0_DMA_SRC_PWM1_CMP17: usize = 44;
    pub const TRGM0_DMA_SRC_PWM1_CMP18: usize = 45;
    pub const TRGM0_DMA_SRC_PWM1_CMP19: usize = 46;
    pub const TRGM0_DMA_SRC_PWM1_CMP20: usize = 47;
    pub const TRGM0_DMA_SRC_PWM1_CMP21: usize = 48;
    pub const TRGM0_DMA_SRC_PWM1_CMP22: usize = 49;
    pub const TRGM0_DMA_SRC_PWM1_CMP23: usize = 50;
    pub const TRGM0_DMA_SRC_PWM1_RLD: usize = 51;
    pub const TRGM0_DMA_SRC_PWM1_HALFRLD: usize = 52;
    pub const TRGM0_DMA_SRC_PWM1_XRLD: usize = 53;
    pub const TRGM0_DMA_SRC_QEI0: usize = 54;
    pub const TRGM0_DMA_SRC_QEI1: usize = 55;
    pub const TRGM0_DMA_SRC_MMC0: usize = 56;
    pub const TRGM0_DMA_SRC_MMC1: usize = 57;
    pub const TRGM0_DMA_SRC_SEI0: usize = 58;
    pub const TRGM0_DMA_SRC_SEI1: usize = 59;
    pub const TRGM0_DMA_SRC_TRGM0: usize = 60;
    pub const TRGM0_DMA_SRC_TRGM1: usize = 61;
    pub const TRGM0_FILTER_SRC_PWM0_IN0: usize = 0;
    pub const TRGM0_FILTER_SRC_PWM0_IN1: usize = 1;
    pub const TRGM0_FILTER_SRC_PWM0_IN2: usize = 2;
    pub const TRGM0_FILTER_SRC_PWM0_IN3: usize = 3;
    pub const TRGM0_FILTER_SRC_PWM0_IN4: usize = 4;
    pub const TRGM0_FILTER_SRC_PWM0_IN5: usize = 5;
    pub const TRGM0_FILTER_SRC_PWM0_IN6: usize = 6;
    pub const TRGM0_FILTER_SRC_PWM0_IN7: usize = 7;
    pub const TRGM0_FILTER_SRC_PWM1_IN0: usize = 8;
    pub const TRGM0_FILTER_SRC_PWM1_IN1: usize = 9;
    pub const TRGM0_FILTER_SRC_PWM1_IN2: usize = 10;
    pub const TRGM0_FILTER_SRC_PWM1_IN3: usize = 11;
    pub const TRGM0_FILTER_SRC_PWM1_IN4: usize = 12;
    pub const TRGM0_FILTER_SRC_PWM1_IN5: usize = 13;
    pub const TRGM0_FILTER_SRC_PWM1_IN6: usize = 14;
    pub const TRGM0_FILTER_SRC_PWM1_IN7: usize = 15;
    pub const TRGM0_FILTER_SRC_TRGM_IN0: usize = 16;
    pub const TRGM0_FILTER_SRC_TRGM_P00: usize = 16;
    pub const TRGM0_FILTER_SRC_TRGM_IN1: usize = 17;
    pub const TRGM0_FILTER_SRC_TRGM_P01: usize = 17;
    pub const TRGM0_FILTER_SRC_TRGM_P02: usize = 18;
    pub const TRGM0_FILTER_SRC_TRGM_IN2: usize = 18;
    pub const TRGM0_FILTER_SRC_TRGM_IN3: usize = 19;
    pub const TRGM0_FILTER_SRC_TRGM_P03: usize = 19;
    pub const TRGM0_FILTER_SRC_TRGM_IN4: usize = 20;
    pub const TRGM0_FILTER_SRC_TRGM_P04: usize = 20;
    pub const TRGM0_FILTER_SRC_TRGM_P05: usize = 21;
    pub const TRGM0_FILTER_SRC_TRGM_IN5: usize = 21;
    pub const TRGM0_FILTER_SRC_TRGM_IN6: usize = 22;
    pub const TRGM0_FILTER_SRC_TRGM_P06: usize = 22;
    pub const TRGM0_FILTER_SRC_TRGM_IN7: usize = 23;
    pub const TRGM0_FILTER_SRC_TRGM_P07: usize = 23;
    pub const TRGM0_FILTER_SRC_PWM0_FAULT0: usize = 24;
    pub const TRGM0_FILTER_SRC_PWM0_FAULT1: usize = 25;
    pub const TRGM0_FILTER_SRC_PWM1_FAULT0: usize = 26;
    pub const TRGM0_FILTER_SRC_PWM1_FAULT1: usize = 27;
    pub const TRGM0_INPUT_SRC_VSS: usize = 0;
    pub const TRGM0_INPUT_SRC_VDD: usize = 1;
    pub const TRGM0_INPUT_SRC_DEBUG_FLAG: usize = 2;
    pub const TRGM0_INPUT_SRC_USB0_SOF: usize = 3;
    pub const TRGM0_INPUT_SRC_PTPC_CMP0: usize = 4;
    pub const TRGM0_INPUT_SRC_PTPC_CMP1: usize = 5;
    pub const TRGM0_INPUT_SRC_ACMP0_OUT: usize = 6;
    pub const TRGM0_INPUT_SRC_ACMP1_OUT: usize = 7;
    pub const TRGM0_INPUT_SRC_GPTMR0_OUT2: usize = 8;
    pub const TRGM0_INPUT_SRC_GPTMR0_OUT3: usize = 9;
    pub const TRGM0_INPUT_SRC_GPTMR1_OUT2: usize = 10;
    pub const TRGM0_INPUT_SRC_GPTMR1_OUT3: usize = 11;
    pub const TRGM0_INPUT_SRC_GPTMR2_OUT2: usize = 12;
    pub const TRGM0_INPUT_SRC_GPTMR2_OUT3: usize = 13;
    pub const TRGM0_INPUT_SRC_GPTMR3_OUT2: usize = 14;
    pub const TRGM0_INPUT_SRC_GPTMR3_OUT3: usize = 15;
    pub const TRGM0_INPUT_SRC_TRGM0_P0: usize = 16;
    pub const TRGM0_INPUT_SRC_TRGM0_P1: usize = 17;
    pub const TRGM0_INPUT_SRC_TRGM0_P2: usize = 18;
    pub const TRGM0_INPUT_SRC_TRGM0_P3: usize = 19;
    pub const TRGM0_INPUT_SRC_TRGM0_P4: usize = 20;
    pub const TRGM0_INPUT_SRC_TRGM0_P5: usize = 21;
    pub const TRGM0_INPUT_SRC_TRGM0_P6: usize = 22;
    pub const TRGM0_INPUT_SRC_TRGM0_P7: usize = 23;
    pub const TRGM0_INPUT_SRC_SYNT0_CH0: usize = 24;
    pub const TRGM0_INPUT_SRC_SYNT0_CH1: usize = 25;
    pub const TRGM0_INPUT_SRC_SYNT0_CH2: usize = 26;
    pub const TRGM0_INPUT_SRC_SYNT0_CH3: usize = 27;
    pub const TRGM0_INPUT_SRC_MMC0_TRGO_0: usize = 28;
    pub const TRGM0_INPUT_SRC_MMC0_TRGO_1: usize = 29;
    pub const TRGM0_INPUT_SRC_MMC1_TRGO_0: usize = 30;
    pub const TRGM0_INPUT_SRC_MMC1_TRGO_1: usize = 31;
    pub const TRGM0_INPUT_SRC_QEO0_TRGO_0: usize = 32;
    pub const TRGM0_INPUT_SRC_QEO0_TRGO_1: usize = 33;
    pub const TRGM0_INPUT_SRC_QEO0_TRGO_2: usize = 34;
    pub const TRGM0_INPUT_SRC_QEO0_TRGO_3: usize = 35;
    pub const TRGM0_INPUT_SRC_QEO0_TRGO_4: usize = 36;
    pub const TRGM0_INPUT_SRC_QEO0_TRGO_5: usize = 37;
    pub const TRGM0_INPUT_SRC_QEO0_TRGO_6: usize = 38;
    pub const TRGM0_INPUT_SRC_QEO0_TRGO_7: usize = 39;
    pub const TRGM0_INPUT_SRC_QEO1_TRGO_0: usize = 40;
    pub const TRGM0_INPUT_SRC_QEO1_TRGO_1: usize = 41;
    pub const TRGM0_INPUT_SRC_QEO1_TRGO_2: usize = 42;
    pub const TRGM0_INPUT_SRC_QEO1_TRGO_3: usize = 43;
    pub const TRGM0_INPUT_SRC_QEO1_TRGO_4: usize = 44;
    pub const TRGM0_INPUT_SRC_QEO1_TRGO_5: usize = 45;
    pub const TRGM0_INPUT_SRC_QEO1_TRGO_6: usize = 46;
    pub const TRGM0_INPUT_SRC_QEO1_TRGO_7: usize = 47;
    pub const TRGM0_INPUT_SRC_PWM0_CH8REF: usize = 48;
    pub const TRGM0_INPUT_SRC_PWM0_CH9REF: usize = 49;
    pub const TRGM0_INPUT_SRC_PWM0_CH10REF: usize = 50;
    pub const TRGM0_INPUT_SRC_PWM0_CH11REF: usize = 51;
    pub const TRGM0_INPUT_SRC_PWM0_CH12REF: usize = 52;
    pub const TRGM0_INPUT_SRC_PWM0_CH13REF: usize = 53;
    pub const TRGM0_INPUT_SRC_PWM0_CH14REF: usize = 54;
    pub const TRGM0_INPUT_SRC_PWM0_CH15REF: usize = 55;
    pub const TRGM0_INPUT_SRC_PWM1_CH8REF: usize = 56;
    pub const TRGM0_INPUT_SRC_PWM1_CH9REF: usize = 57;
    pub const TRGM0_INPUT_SRC_PWM1_CH10REF: usize = 58;
    pub const TRGM0_INPUT_SRC_PWM1_CH11REF: usize = 59;
    pub const TRGM0_INPUT_SRC_PWM1_CH12REF: usize = 60;
    pub const TRGM0_INPUT_SRC_PWM1_CH13REF: usize = 61;
    pub const TRGM0_INPUT_SRC_PWM1_CH14REF: usize = 62;
    pub const TRGM0_INPUT_SRC_PWM1_CH15REF: usize = 63;
    pub const TRGM0_INPUT_SRC_PLB_OUT00: usize = 64;
    pub const TRGM0_INPUT_SRC_PLB_OUT01: usize = 65;
    pub const TRGM0_INPUT_SRC_PLB_OUT02: usize = 66;
    pub const TRGM0_INPUT_SRC_PLB_OUT03: usize = 67;
    pub const TRGM0_INPUT_SRC_PLB_OUT04: usize = 68;
    pub const TRGM0_INPUT_SRC_PLB_OUT05: usize = 69;
    pub const TRGM0_INPUT_SRC_PLB_OUT06: usize = 70;
    pub const TRGM0_INPUT_SRC_PLB_OUT07: usize = 71;
    pub const TRGM0_INPUT_SRC_PLB_OUT08: usize = 72;
    pub const TRGM0_INPUT_SRC_PLB_OUT09: usize = 73;
    pub const TRGM0_INPUT_SRC_PLB_OUT10: usize = 74;
    pub const TRGM0_INPUT_SRC_PLB_OUT11: usize = 75;
    pub const TRGM0_INPUT_SRC_PLB_OUT12: usize = 76;
    pub const TRGM0_INPUT_SRC_PLB_OUT13: usize = 77;
    pub const TRGM0_INPUT_SRC_PLB_OUT14: usize = 78;
    pub const TRGM0_INPUT_SRC_PLB_OUT15: usize = 79;
    pub const TRGM0_INPUT_SRC_PLB_OUT16: usize = 80;
    pub const TRGM0_INPUT_SRC_PLB_OUT17: usize = 81;
    pub const TRGM0_INPUT_SRC_PLB_OUT18: usize = 82;
    pub const TRGM0_INPUT_SRC_PLB_OUT19: usize = 83;
    pub const TRGM0_INPUT_SRC_PLB_OUT20: usize = 84;
    pub const TRGM0_INPUT_SRC_PLB_OUT21: usize = 85;
    pub const TRGM0_INPUT_SRC_PLB_OUT22: usize = 86;
    pub const TRGM0_INPUT_SRC_PLB_OUT23: usize = 87;
    pub const TRGM0_INPUT_SRC_PLB_OUT24: usize = 88;
    pub const TRGM0_INPUT_SRC_PLB_OUT25: usize = 89;
    pub const TRGM0_INPUT_SRC_PLB_OUT26: usize = 90;
    pub const TRGM0_INPUT_SRC_PLB_OUT27: usize = 91;
    pub const TRGM0_INPUT_SRC_PLB_OUT28: usize = 92;
    pub const TRGM0_INPUT_SRC_PLB_OUT29: usize = 93;
    pub const TRGM0_INPUT_SRC_PLB_OUT30: usize = 94;
    pub const TRGM0_INPUT_SRC_PLB_OUT31: usize = 95;
    pub const TRGM0_INPUT_SRC_RDC_TRGO_0: usize = 96;
    pub const TRGM0_INPUT_SRC_RDC_TRGO_1: usize = 97;
    pub const TRGM0_INPUT_SRC_QEI1_TRGO: usize = 98;
    pub const TRGM0_INPUT_SRC_QEI0_TRGO: usize = 99;
    pub const TRGM0_INPUT_SRC_SEI_TRGO_0: usize = 100;
    pub const TRGM0_INPUT_SRC_SEI_TRGO_1: usize = 101;
    pub const TRGM0_INPUT_SRC_SEI_TRGO_2: usize = 102;
    pub const TRGM0_INPUT_SRC_SEI_TRGO_3: usize = 103;
    pub const TRGM0_INPUT_SRC_SEI_TRGO_4: usize = 104;
    pub const TRGM0_INPUT_SRC_SEI_TRGO_5: usize = 105;
    pub const TRGM0_INPUT_SRC_SEI_TRGO_6: usize = 106;
    pub const TRGM0_INPUT_SRC_SEI_TRGO_7: usize = 107;
    pub const TRGM0_INPUT_SRC_PWM0_FAULT0: usize = 108;
    pub const TRGM0_INPUT_SRC_PWM0_FAULT1: usize = 109;
    pub const TRGM0_INPUT_SRC_PWM1_FAULT0: usize = 110;
    pub const TRGM0_INPUT_SRC_PWM1_FAULT1: usize = 111;
    pub const TRGM0_INPUT_SRC_PWM0_CAPIN0: usize = 112;
    pub const TRGM0_INPUT_SRC_PWM0_CAPIN1: usize = 113;
    pub const TRGM0_INPUT_SRC_PWM0_CAPIN2: usize = 114;
    pub const TRGM0_INPUT_SRC_PWM0_CAPIN3: usize = 115;
    pub const TRGM0_INPUT_SRC_PWM0_CAPIN4: usize = 116;
    pub const TRGM0_INPUT_SRC_PWM0_CAPIN5: usize = 117;
    pub const TRGM0_INPUT_SRC_PWM0_CAPIN6: usize = 118;
    pub const TRGM0_INPUT_SRC_PWM0_CAPIN7: usize = 119;
    pub const TRGM0_INPUT_SRC_PWM1_CAPIN0: usize = 120;
    pub const TRGM0_INPUT_SRC_PWM1_CAPIN1: usize = 121;
    pub const TRGM0_INPUT_SRC_PWM1_CAPIN2: usize = 122;
    pub const TRGM0_INPUT_SRC_PWM1_CAPIN3: usize = 123;
    pub const TRGM0_INPUT_SRC_PWM1_CAPIN4: usize = 124;
    pub const TRGM0_INPUT_SRC_PWM1_CAPIN5: usize = 125;
    pub const TRGM0_INPUT_SRC_PWM1_CAPIN6: usize = 126;
    pub const TRGM0_INPUT_SRC_PWM1_CAPIN7: usize = 127;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP0_0: usize = 0;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP0_1: usize = 1;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP0_2: usize = 2;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP0_3: usize = 3;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP0_4: usize = 4;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP0_5: usize = 5;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP0_6: usize = 6;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP0_7: usize = 7;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP1_0: usize = 8;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP1_1: usize = 9;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP1_2: usize = 10;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP1_3: usize = 11;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP1_4: usize = 12;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP1_5: usize = 13;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP1_6: usize = 14;
    pub const TRGM0_OUTPUT_SRC_MOT2OPAMP1_7: usize = 15;
    pub const TRGM0_OUTPUT_SRC_GPTMR0_IN2: usize = 16;
    pub const TRGM0_OUTPUT_SRC_GPTMR0_IN3: usize = 17;
    pub const TRGM0_OUTPUT_SRC_GPTMR0_SYNCI: usize = 18;
    pub const TRGM0_OUTPUT_SRC_GPTMR1_IN2: usize = 19;
    pub const TRGM0_OUTPUT_SRC_GPTMR1_IN3: usize = 20;
    pub const TRGM0_OUTPUT_SRC_GPTMR1_SYNCI: usize = 21;
    pub const TRGM0_OUTPUT_SRC_GPTMR2_IN2: usize = 22;
    pub const TRGM0_OUTPUT_SRC_GPTMR2_IN3: usize = 23;
    pub const TRGM0_OUTPUT_SRC_GPTMR2_SYNCI: usize = 24;
    pub const TRGM0_OUTPUT_SRC_GPTMR3_IN2: usize = 25;
    pub const TRGM0_OUTPUT_SRC_GPTMR3_IN3: usize = 26;
    pub const TRGM0_OUTPUT_SRC_GPTMR3_SYNCI: usize = 27;
    pub const TRGM0_OUTPUT_SRC_ACMP0_WIN: usize = 28;
    pub const TRGM0_OUTPUT_SRC_ACMP1_WIN: usize = 29;
    pub const TRGM0_OUTPUT_SRC_DAC0_BUFTRG: usize = 30;
    pub const TRGM0_OUTPUT_SRC_DAC1_BUFTRG: usize = 31;
    pub const TRGM0_OUTPUT_SRC_ADC0_STRGI: usize = 32;
    pub const TRGM0_OUTPUT_SRC_ADC1_STRGI: usize = 33;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI0A: usize = 34;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI0B: usize = 35;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI0C: usize = 36;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI1A: usize = 37;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI1B: usize = 38;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI1C: usize = 39;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI2A: usize = 40;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI2B: usize = 41;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI2C: usize = 42;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI3A: usize = 43;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI3B: usize = 44;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI3C: usize = 45;
    pub const TRGM0_OUTPUT_SRC_MCAN_PTPC0_CAP: usize = 46;
    pub const TRGM0_OUTPUT_SRC_MCAN_PTPC1_CAP: usize = 47;
    pub const TRGM0_OUTPUT_SRC_QEO0_TRIG_IN0: usize = 48;
    pub const TRGM0_OUTPUT_SRC_QEO0_TRIG_IN1: usize = 49;
    pub const TRGM0_OUTPUT_SRC_QEO1_TRIG_IN0: usize = 50;
    pub const TRGM0_OUTPUT_SRC_QEO1_TRIG_IN1: usize = 51;
    pub const TRGM0_OUTPUT_SRC_SEI_TRIG_IN0: usize = 52;
    pub const TRGM0_OUTPUT_SRC_SEI_TRIG_IN1: usize = 53;
    pub const TRGM0_OUTPUT_SRC_SEI_TRIG_IN2: usize = 54;
    pub const TRGM0_OUTPUT_SRC_SEI_TRIG_IN3: usize = 55;
    pub const TRGM0_OUTPUT_SRC_SEI_TRIG_IN4: usize = 56;
    pub const TRGM0_OUTPUT_SRC_SEI_TRIG_IN5: usize = 57;
    pub const TRGM0_OUTPUT_SRC_SEI_TRIG_IN6: usize = 58;
    pub const TRGM0_OUTPUT_SRC_SEI_TRIG_IN7: usize = 59;
    pub const TRGM0_OUTPUT_SRC_MMC0_TRIG_IN0: usize = 60;
    pub const TRGM0_OUTPUT_SRC_MMC0_TRIG_IN1: usize = 61;
    pub const TRGM0_OUTPUT_SRC_MMC1_TRIG_IN0: usize = 62;
    pub const TRGM0_OUTPUT_SRC_MMC1_TRIG_IN1: usize = 63;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_00: usize = 64;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_01: usize = 65;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_02: usize = 66;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_03: usize = 67;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_04: usize = 68;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_05: usize = 69;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_06: usize = 70;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_07: usize = 71;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_08: usize = 72;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_09: usize = 73;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_10: usize = 74;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_11: usize = 75;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_12: usize = 76;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_13: usize = 77;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_14: usize = 78;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_15: usize = 79;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_16: usize = 80;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_17: usize = 81;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_18: usize = 82;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_19: usize = 83;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_20: usize = 84;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_21: usize = 85;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_22: usize = 86;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_23: usize = 87;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_24: usize = 88;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_25: usize = 89;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_26: usize = 90;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_27: usize = 91;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_28: usize = 92;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_29: usize = 93;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_30: usize = 94;
    pub const TRGM0_OUTPUT_SRC_PLB_IN_31: usize = 95;
    pub const TRGM0_OUTPUT_SRC_MOT_GPIO0: usize = 96;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P0: usize = 96;
    pub const TRGM0_OUTPUT_SRC_MOT_GPIO1: usize = 97;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P1: usize = 97;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P2: usize = 98;
    pub const TRGM0_OUTPUT_SRC_MOT_GPIO2: usize = 98;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P3: usize = 99;
    pub const TRGM0_OUTPUT_SRC_MOT_GPIO3: usize = 99;
    pub const TRGM0_OUTPUT_SRC_MOT_GPIO4: usize = 100;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P4: usize = 100;
    pub const TRGM0_OUTPUT_SRC_MOT_GPIO5: usize = 101;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P5: usize = 101;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P6: usize = 102;
    pub const TRGM0_OUTPUT_SRC_MOT_GPIO6: usize = 102;
    pub const TRGM0_OUTPUT_SRC_MOT_GPIO7: usize = 103;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P7: usize = 103;
    pub const TRGM0_OUTPUT_SRC_PWM_IN8: usize = 104;
    pub const TRGM0_OUTPUT_SRC_PWM_IN9: usize = 105;
    pub const TRGM0_OUTPUT_SRC_PWM_IN10: usize = 106;
    pub const TRGM0_OUTPUT_SRC_PWM_IN11: usize = 107;
    pub const TRGM0_OUTPUT_SRC_PWM_IN12: usize = 108;
    pub const TRGM0_OUTPUT_SRC_PWM_IN13: usize = 109;
    pub const TRGM0_OUTPUT_SRC_PWM_IN14: usize = 110;
    pub const TRGM0_OUTPUT_SRC_PWM_IN15: usize = 111;
    pub const TRGM0_OUTPUT_SRC_PWM0_FRCI: usize = 112;
    pub const TRGM0_OUTPUT_SRC_PWM0_FRCSYNCI: usize = 113;
    pub const TRGM0_OUTPUT_SRC_PWM0_SYNCI: usize = 114;
    pub const TRGM0_OUTPUT_SRC_PWM0_SHRLDSYNCI: usize = 115;
    pub const TRGM0_OUTPUT_SRC_PWM0_FAULTI0: usize = 116;
    pub const TRGM0_OUTPUT_SRC_PWM0_FAULTI1: usize = 117;
    pub const TRGM0_OUTPUT_SRC_PWM1_FRCI: usize = 118;
    pub const TRGM0_OUTPUT_SRC_PWM1_FRCSYNCI: usize = 119;
    pub const TRGM0_OUTPUT_SRC_PWM1_SYNCI: usize = 120;
    pub const TRGM0_OUTPUT_SRC_PWM1_SHRLDSYNCI: usize = 121;
    pub const TRGM0_OUTPUT_SRC_PWM1_FAULTI0: usize = 122;
    pub const TRGM0_OUTPUT_SRC_PWM1_FAULTI1: usize = 123;
    pub const TRGM0_OUTPUT_SRC_RDC_TRIG_IN0: usize = 124;
    pub const TRGM0_OUTPUT_SRC_RDC_TRIG_IN1: usize = 125;
    pub const TRGM0_OUTPUT_SRC_SYNCTIMER_TRIG: usize = 126;
    pub const TRGM0_OUTPUT_SRC_QEI0_TRIG_IN: usize = 127;
    pub const TRGM0_OUTPUT_SRC_QEI1_TRIG_IN: usize = 128;
    pub const TRGM0_OUTPUT_SRC_QEI0_PAUSE: usize = 129;
    pub const TRGM0_OUTPUT_SRC_QEI1_PAUSE: usize = 130;
    pub const TRGM0_OUTPUT_SRC_UART_TRIG0: usize = 131;
    pub const TRGM0_OUTPUT_SRC_UART_TRIG1: usize = 132;
    pub const TRGM0_OUTPUT_SRC_TRGM_IRQ0: usize = 133;
    pub const TRGM0_OUTPUT_SRC_TRGM_IRQ1: usize = 134;
    pub const TRGM0_OUTPUT_SRC_TRGM_DMA0: usize = 135;
    pub const TRGM0_OUTPUT_SRC_TRGM_DMA1: usize = 136;
}
