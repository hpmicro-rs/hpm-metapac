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
    #[doc = "7 - GPIO0_Z"]
    GPIO0_Z = 7,
    #[doc = "8 - ADC0"]
    ADC0 = 8,
    #[doc = "9 - ADC1"]
    ADC1 = 9,
    #[doc = "10 - ADC2"]
    ADC2 = 10,
    #[doc = "11 - DAC0"]
    DAC0 = 11,
    #[doc = "12 - ACMP_0"]
    ACMP_0 = 12,
    #[doc = "13 - ACMP_1"]
    ACMP_1 = 13,
    #[doc = "14 - SPI0"]
    SPI0 = 14,
    #[doc = "15 - SPI1"]
    SPI1 = 15,
    #[doc = "16 - SPI2"]
    SPI2 = 16,
    #[doc = "17 - SPI3"]
    SPI3 = 17,
    #[doc = "18 - UART0"]
    UART0 = 18,
    #[doc = "19 - UART1"]
    UART1 = 19,
    #[doc = "20 - UART2"]
    UART2 = 20,
    #[doc = "21 - UART3"]
    UART3 = 21,
    #[doc = "22 - UART4"]
    UART4 = 22,
    #[doc = "23 - UART5"]
    UART5 = 23,
    #[doc = "24 - UART6"]
    UART6 = 24,
    #[doc = "25 - UART7"]
    UART7 = 25,
    #[doc = "26 - CAN0"]
    CAN0 = 26,
    #[doc = "27 - CAN1"]
    CAN1 = 27,
    #[doc = "28 - PTPC"]
    PTPC = 28,
    #[doc = "29 - WDG0"]
    WDG0 = 29,
    #[doc = "30 - WDG1"]
    WDG1 = 30,
    #[doc = "31 - TSNS"]
    TSNS = 31,
    #[doc = "32 - MBX0A"]
    MBX0A = 32,
    #[doc = "33 - MBX0B"]
    MBX0B = 33,
    #[doc = "34 - GPTMR0"]
    GPTMR0 = 34,
    #[doc = "35 - GPTMR1"]
    GPTMR1 = 35,
    #[doc = "36 - GPTMR2"]
    GPTMR2 = 36,
    #[doc = "37 - GPTMR3"]
    GPTMR3 = 37,
    #[doc = "38 - I2C0"]
    I2C0 = 38,
    #[doc = "39 - I2C1"]
    I2C1 = 39,
    #[doc = "40 - I2C2"]
    I2C2 = 40,
    #[doc = "41 - I2C3"]
    I2C3 = 41,
    #[doc = "42 - PWM0"]
    PWM0 = 42,
    #[doc = "43 - HALL0"]
    HALL0 = 43,
    #[doc = "44 - QEI0"]
    QEI0 = 44,
    #[doc = "45 - PWM1"]
    PWM1 = 45,
    #[doc = "46 - HALL1"]
    HALL1 = 46,
    #[doc = "47 - QEI1"]
    QEI1 = 47,
    #[doc = "48 - SDP"]
    SDP = 48,
    #[doc = "49 - XPI0"]
    XPI0 = 49,
    #[doc = "50 - XPI1"]
    XPI1 = 50,
    #[doc = "51 - XDMA"]
    XDMA = 51,
    #[doc = "52 - HDMA"]
    HDMA = 52,
    #[doc = "53 - FEMC"]
    FEMC = 53,
    #[doc = "54 - RNG"]
    RNG = 54,
    #[doc = "55 - I2S0"]
    I2S0 = 55,
    #[doc = "56 - I2S1"]
    I2S1 = 56,
    #[doc = "57 - DAO"]
    DAO = 57,
    #[doc = "58 - PDM"]
    PDM = 58,
    #[doc = "59 - FFA"]
    FFA = 59,
    #[doc = "60 - NTMR0"]
    NTMR0 = 60,
    #[doc = "61 - USB0"]
    USB0 = 61,
    #[doc = "62 - ENET0"]
    ENET0 = 62,
    #[doc = "63 - SDXC0"]
    SDXC0 = 63,
    #[doc = "64 - PSEC"]
    PSEC = 64,
    #[doc = "65 - PGPIO"]
    PGPIO = 65,
    #[doc = "66 - PWDG"]
    PWDG = 66,
    #[doc = "67 - PTMR"]
    PTMR = 67,
    #[doc = "68 - PUART"]
    PUART = 68,
    #[doc = "69 - FUSE"]
    FUSE = 69,
    #[doc = "70 - SECMON"]
    SECMON = 70,
    #[doc = "71 - RTC"]
    RTC = 71,
    #[doc = "72 - BUTN"]
    BUTN = 72,
    #[doc = "73 - BGPIO"]
    BGPIO = 73,
    #[doc = "74 - BVIO"]
    BVIO = 74,
    #[doc = "75 - BROWNOUT"]
    BROWNOUT = 75,
    #[doc = "76 - SYSCTL"]
    SYSCTL = 76,
    #[doc = "77 - DEBUG_0"]
    DEBUG_0 = 77,
    #[doc = "78 - DEBUG_1"]
    DEBUG_1 = 78,
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
        fn GPIO0_Z();
        fn ADC0();
        fn ADC1();
        fn ADC2();
        fn DAC0();
        fn ACMP_0();
        fn ACMP_1();
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
        fn CAN0();
        fn CAN1();
        fn PTPC();
        fn WDG0();
        fn WDG1();
        fn TSNS();
        fn MBX0A();
        fn MBX0B();
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
        fn SDP();
        fn XPI0();
        fn XPI1();
        fn XDMA();
        fn HDMA();
        fn FEMC();
        fn RNG();
        fn I2S0();
        fn I2S1();
        fn DAO();
        fn PDM();
        fn FFA();
        fn NTMR0();
        fn USB0();
        fn ENET0();
        fn SDXC0();
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
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 79] = [
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
        Vector { _handler: ADC0 },
        Vector { _handler: ADC1 },
        Vector { _handler: ADC2 },
        Vector { _handler: DAC0 },
        Vector { _handler: ACMP_0 },
        Vector { _handler: ACMP_1 },
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
        Vector { _handler: CAN0 },
        Vector { _handler: CAN1 },
        Vector { _handler: PTPC },
        Vector { _handler: WDG0 },
        Vector { _handler: WDG1 },
        Vector { _handler: TSNS },
        Vector { _handler: MBX0A },
        Vector { _handler: MBX0B },
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
        Vector { _handler: SDP },
        Vector { _handler: XPI0 },
        Vector { _handler: XPI1 },
        Vector { _handler: XDMA },
        Vector { _handler: HDMA },
        Vector { _handler: FEMC },
        Vector { _handler: RNG },
        Vector { _handler: I2S0 },
        Vector { _handler: I2S1 },
        Vector { _handler: DAO },
        Vector { _handler: PDM },
        Vector { _handler: FFA },
        Vector { _handler: NTMR0 },
        Vector { _handler: USB0 },
        Vector { _handler: ENET0 },
        Vector { _handler: SDXC0 },
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
    ];
}
pub const FGPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x000c_0000usize as _) };
pub const PLIC: plic::Plic = unsafe { plic::Plic::from_ptr(0xe400_0000usize as _) };
pub const MCHTMR: mchtmr::Mchtmr = unsafe { mchtmr::Mchtmr::from_ptr(0xe600_0000usize as _) };
pub const PLICSW: plicsw::Plicsw = unsafe { plicsw::Plicsw::from_ptr(0xe640_0000usize as _) };
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf000_0000usize as _) };
pub const GPIOM: gpiom::Gpiom = unsafe { gpiom::Gpiom::from_ptr(0xf000_8000usize as _) };
pub const ADC0: adc16::Adc = unsafe { adc16::Adc::from_ptr(0xf001_0000usize as _) };
pub const ADC1: adc16::Adc = unsafe { adc16::Adc::from_ptr(0xf001_4000usize as _) };
pub const ADC2: adc16::Adc = unsafe { adc16::Adc::from_ptr(0xf001_8000usize as _) };
pub const ACMP: acmp::Acmp = unsafe { acmp::Acmp::from_ptr(0xf002_0000usize as _) };
pub const DAC0: dac::Dac = unsafe { dac::Dac::from_ptr(0xf002_4000usize as _) };
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
pub const MBX0A: mbx::Mbx = unsafe { mbx::Mbx::from_ptr(0xf00a_0000usize as _) };
pub const MBX0B: mbx::Mbx = unsafe { mbx::Mbx::from_ptr(0xf00a_4000usize as _) };
pub const PTPC: ptpc::Ptpc = unsafe { ptpc::Ptpc::from_ptr(0xf00b_0000usize as _) };
pub const DMAMUX: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0xf00c_0000usize as _) };
pub const HDMA: dma::Dma = unsafe { dma::Dma::from_ptr(0xf00c_4000usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0xf00c_8000usize as _) };
pub const KEYM: keym::Keym = unsafe { keym::Keym::from_ptr(0xf00c_c000usize as _) };
pub const I2S0: i2s::I2s = unsafe { i2s::I2s::from_ptr(0xf010_0000usize as _) };
pub const I2S1: i2s::I2s = unsafe { i2s::I2s::from_ptr(0xf010_4000usize as _) };
pub const PDM: pdm::Pdm = unsafe { pdm::Pdm::from_ptr(0xf011_4000usize as _) };
pub const PWM0: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0xf020_0000usize as _) };
pub const HALL0: hall::Hall = unsafe { hall::Hall::from_ptr(0xf020_4000usize as _) };
pub const QEI0: qei::Qei = unsafe { qei::Qei::from_ptr(0xf020_8000usize as _) };
pub const TRGM0: trgm::Trgm = unsafe { trgm::Trgm::from_ptr(0xf020_c000usize as _) };
pub const PWM1: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0xf021_0000usize as _) };
pub const HALL1: hall::Hall = unsafe { hall::Hall::from_ptr(0xf021_4000usize as _) };
pub const QEI1: qei::Qei = unsafe { qei::Qei::from_ptr(0xf021_8000usize as _) };
pub const TRGM1: trgm::Trgm = unsafe { trgm::Trgm::from_ptr(0xf021_c000usize as _) };
pub const SYNT: synt::Synt = unsafe { synt::Synt::from_ptr(0xf024_0000usize as _) };
pub const ENET0: enet::Enet = unsafe { enet::Enet::from_ptr(0xf200_0000usize as _) };
pub const NTMR0: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf201_0000usize as _) };
pub const USB0: usb::Usb = unsafe { usb::Usb::from_ptr(0xf202_0000usize as _) };
pub const SDXC0: sdxc::Sdxc = unsafe { sdxc::Sdxc::from_ptr(0xf203_0000usize as _) };
pub const GPTMR0: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf300_0000usize as _) };
pub const GPTMR1: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf300_4000usize as _) };
pub const GPTMR2: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf300_8000usize as _) };
pub const GPTMR3: tmr::Tmr = unsafe { tmr::Tmr::from_ptr(0xf300_c000usize as _) };
pub const I2C0: i2c::I2c = unsafe { i2c::I2c::from_ptr(0xf302_0000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0xf302_4000usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0xf302_8000usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0xf302_c000usize as _) };
pub const XPI0: xpi::Xpi = unsafe { xpi::Xpi::from_ptr(0xf304_0000usize as _) };
pub const XPI1: xpi::Xpi = unsafe { xpi::Xpi::from_ptr(0xf304_4000usize as _) };
pub const XDMA: dma::Dma = unsafe { dma::Dma::from_ptr(0xf304_8000usize as _) };
pub const SDP: sdp::Sdp = unsafe { sdp::Sdp::from_ptr(0xf304_c000usize as _) };
pub const FEMC: femc::Femc = unsafe { femc::Femc::from_ptr(0xf305_0000usize as _) };
pub const FFA: ffa::Ffa = unsafe { ffa::Ffa::from_ptr(0xf305_8000usize as _) };
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
pub const PLLCTL: pllctl::Pllctlv2 = unsafe { pllctl::Pllctlv2::from_ptr(0xf410_0000usize as _) };
pub const TSNS: tsns::Tsns = unsafe { tsns::Tsns::from_ptr(0xf410_4000usize as _) };
pub const BPOR: bpor::Bpor = unsafe { bpor::Bpor::from_ptr(0xf500_4000usize as _) };
pub const BCFG: bcfg::Bcfg = unsafe { bcfg::Bcfg::from_ptr(0xf500_8000usize as _) };
pub const BUTN: butn::Butn = unsafe { butn::Butn::from_ptr(0xf500_c000usize as _) };
pub const BIOC: ioc::Ioc = unsafe { ioc::Ioc::from_ptr(0xf501_0000usize as _) };
pub const BGPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0xf501_4000usize as _) };
pub const BSEC: bsec::Bsec = unsafe { bsec::Bsec::from_ptr(0xf504_0000usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0xf504_4000usize as _) };
pub const BKEY: bkey::Bkey = unsafe { bkey::Bkey::from_ptr(0xf504_8000usize as _) };
pub const BMON: bmon::Bmon = unsafe { bmon::Bmon::from_ptr(0xf504_c000usize as _) };
pub const TAMP: tamp::Tamp = unsafe { tamp::Tamp::from_ptr(0xf505_0000usize as _) };
pub const MONO: mono::Mono = unsafe { mono::Mono::from_ptr(0xf505_4000usize as _) };
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
            5 => Ok(Self::GPIO0_X),
            6 => Ok(Self::GPIO0_Y),
            7 => Ok(Self::GPIO0_Z),
            8 => Ok(Self::ADC0),
            9 => Ok(Self::ADC1),
            10 => Ok(Self::ADC2),
            11 => Ok(Self::DAC0),
            12 => Ok(Self::ACMP_0),
            13 => Ok(Self::ACMP_1),
            14 => Ok(Self::SPI0),
            15 => Ok(Self::SPI1),
            16 => Ok(Self::SPI2),
            17 => Ok(Self::SPI3),
            18 => Ok(Self::UART0),
            19 => Ok(Self::UART1),
            20 => Ok(Self::UART2),
            21 => Ok(Self::UART3),
            22 => Ok(Self::UART4),
            23 => Ok(Self::UART5),
            24 => Ok(Self::UART6),
            25 => Ok(Self::UART7),
            26 => Ok(Self::CAN0),
            27 => Ok(Self::CAN1),
            28 => Ok(Self::PTPC),
            29 => Ok(Self::WDG0),
            30 => Ok(Self::WDG1),
            31 => Ok(Self::TSNS),
            32 => Ok(Self::MBX0A),
            33 => Ok(Self::MBX0B),
            34 => Ok(Self::GPTMR0),
            35 => Ok(Self::GPTMR1),
            36 => Ok(Self::GPTMR2),
            37 => Ok(Self::GPTMR3),
            38 => Ok(Self::I2C0),
            39 => Ok(Self::I2C1),
            40 => Ok(Self::I2C2),
            41 => Ok(Self::I2C3),
            42 => Ok(Self::PWM0),
            43 => Ok(Self::HALL0),
            44 => Ok(Self::QEI0),
            45 => Ok(Self::PWM1),
            46 => Ok(Self::HALL1),
            47 => Ok(Self::QEI1),
            48 => Ok(Self::SDP),
            49 => Ok(Self::XPI0),
            50 => Ok(Self::XPI1),
            51 => Ok(Self::XDMA),
            52 => Ok(Self::HDMA),
            53 => Ok(Self::FEMC),
            54 => Ok(Self::RNG),
            55 => Ok(Self::I2S0),
            56 => Ok(Self::I2S1),
            57 => Ok(Self::DAO),
            58 => Ok(Self::PDM),
            59 => Ok(Self::FFA),
            60 => Ok(Self::NTMR0),
            61 => Ok(Self::USB0),
            62 => Ok(Self::ENET0),
            63 => Ok(Self::SDXC0),
            64 => Ok(Self::PSEC),
            65 => Ok(Self::PGPIO),
            66 => Ok(Self::PWDG),
            67 => Ok(Self::PTMR),
            68 => Ok(Self::PUART),
            69 => Ok(Self::FUSE),
            70 => Ok(Self::SECMON),
            71 => Ok(Self::RTC),
            72 => Ok(Self::BUTN),
            73 => Ok(Self::BGPIO),
            74 => Ok(Self::BVIO),
            75 => Ok(Self::BROWNOUT),
            76 => Ok(Self::SYSCTL),
            77 => Ok(Self::DEBUG_0),
            78 => Ok(Self::DEBUG_1),

            _ => Err(riscv_pac::result::Error::InvalidVariant(value)),
        }
    }
}
unsafe impl riscv_pac::ExternalInterruptNumber for Interrupt {}
#[path = "../../peripherals/acmp_common.rs"]
pub mod acmp;
#[path = "../../peripherals/adc16_v63.rs"]
pub mod adc16;
#[path = "../../peripherals/bcfg_v62.rs"]
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
#[path = "../../peripherals/can_v67.rs"]
pub mod can;
#[path = "../../peripherals/dac_v63.rs"]
pub mod dac;
#[path = "../../peripherals/dma_v67.rs"]
pub mod dma;
#[path = "../../peripherals/dmamux_common.rs"]
pub mod dmamux;
#[path = "../../peripherals/enet_v63.rs"]
pub mod enet;
#[path = "../../peripherals/femc_common.rs"]
pub mod femc;
#[path = "../../peripherals/ffa_common.rs"]
pub mod ffa;
#[path = "../../peripherals/gpio_common.rs"]
pub mod gpio;
#[path = "../../peripherals/gpiom_v63.rs"]
pub mod gpiom;
#[path = "../../peripherals/hall_common.rs"]
pub mod hall;
#[path = "../../peripherals/i2c_v67.rs"]
pub mod i2c;
#[path = "../../peripherals/i2s_common.rs"]
pub mod i2s;
#[path = "../../peripherals/ioc_common.rs"]
pub mod ioc;
#[path = "../../peripherals/keym_common.rs"]
pub mod keym;
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
#[path = "../../peripherals/plic_common.rs"]
pub mod plic;
#[path = "../../peripherals/plicsw_common.rs"]
pub mod plicsw;
#[path = "../../peripherals/pllctl_v2.rs"]
pub mod pllctl;
#[path = "../../peripherals/pmon_common.rs"]
pub mod pmon;
#[path = "../../peripherals/ppor_v67.rs"]
pub mod ppor;
#[path = "../../peripherals/psec_common.rs"]
pub mod psec;
#[path = "../../peripherals/ptpc_common.rs"]
pub mod ptpc;
#[path = "../../peripherals/pwm_v53.rs"]
pub mod pwm;
#[path = "../../peripherals/qei_v67.rs"]
pub mod qei;
#[path = "../../peripherals/rng_common.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_common.rs"]
pub mod rtc;
#[path = "../../peripherals/sdp_v53.rs"]
pub mod sdp;
#[path = "../../peripherals/sdxc_v63.rs"]
pub mod sdxc;
#[path = "../../peripherals/spi_v67.rs"]
pub mod spi;
#[path = "../../peripherals/synt_v67.rs"]
pub mod synt;
#[path = "../../peripherals/sysctl_v63.rs"]
pub mod sysctl;
#[path = "../../peripherals/tamp_v62.rs"]
pub mod tamp;
#[path = "../../peripherals/tmr_common.rs"]
pub mod tmr;
#[path = "../../peripherals/trgm_v67.rs"]
pub mod trgm;
#[path = "../../peripherals/tsns_common.rs"]
pub mod tsns;
#[path = "../../peripherals/uart_v67.rs"]
pub mod uart;
#[path = "../../peripherals/usb_v67.rs"]
pub mod usb;
#[path = "../../peripherals/wdg_v67.rs"]
pub mod wdg;
#[path = "../../peripherals/xpi_dummy.rs"]
pub mod xpi;
pub const FLASH_BASE: usize = 2147483648;
pub const FLASH_SIZE: usize = 4194304;
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
    pub const IOC_PA00_FUNC_CTL_GPIO_A_00: u8 = 0;
    pub const IOC_PA00_FUNC_CTL_SDC0_DATA_1: u8 = 17;
    pub const IOC_PA00_FUNC_CTL_SPI3_CSN: u8 = 5;
    pub const IOC_PA00_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PA00_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PA01_FUNC_CTL_GPIO_A_01: u8 = 0;
    pub const IOC_PA01_FUNC_CTL_SDC0_DATA_0: u8 = 17;
    pub const IOC_PA01_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PA01_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PA01_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PA02_FUNC_CTL_GPIO_A_02: u8 = 0;
    pub const IOC_PA02_FUNC_CTL_SDC0_CLK: u8 = 17;
    pub const IOC_PA02_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PA02_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PA02_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
    pub const IOC_PA03_FUNC_CTL_GPIO_A_03: u8 = 0;
    pub const IOC_PA03_FUNC_CTL_SDC0_CMD: u8 = 17;
    pub const IOC_PA03_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PA03_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PA03_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PA04_FUNC_CTL_ACMP_COMP_1: u8 = 16;
    pub const IOC_PA04_FUNC_CTL_GPIO_A_04: u8 = 0;
    pub const IOC_PA04_FUNC_CTL_SDC0_DATA_3: u8 = 17;
    pub const IOC_PA04_FUNC_CTL_SPI3_DAT3: u8 = 5;
    pub const IOC_PA04_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PA04_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PA05_FUNC_CTL_ACMP_COMP_0: u8 = 16;
    pub const IOC_PA05_FUNC_CTL_GPIO_A_05: u8 = 0;
    pub const IOC_PA05_FUNC_CTL_SDC0_DATA_2: u8 = 17;
    pub const IOC_PA05_FUNC_CTL_SPI3_DAT2: u8 = 5;
    pub const IOC_PA05_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PA05_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PA06_FUNC_CTL_ETH0_TXEN: u8 = 18;
    pub const IOC_PA06_FUNC_CTL_GPIO_A_06: u8 = 0;
    pub const IOC_PA06_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PA06_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PA06_FUNC_CTL_SPI0_CSN: u8 = 5;
    pub const IOC_PA06_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PA06_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PA06_FUNC_CTL_XPI0_CA_CS1: u8 = 14;
    pub const IOC_PA07_FUNC_CTL_ETH0_TXD_1: u8 = 18;
    pub const IOC_PA07_FUNC_CTL_GPIO_A_07: u8 = 0;
    pub const IOC_PA07_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PA07_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PA07_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PA07_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PA07_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PA08_FUNC_CTL_CAN0_TXD: u8 = 7;
    pub const IOC_PA08_FUNC_CTL_ETH0_TXD_0: u8 = 18;
    pub const IOC_PA08_FUNC_CTL_GPIO_A_08: u8 = 0;
    pub const IOC_PA08_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PA08_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PA08_FUNC_CTL_SDC0_DATA_2: u8 = 17;
    pub const IOC_PA08_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PA08_FUNC_CTL_UART3_DE: u8 = 2;
    pub const IOC_PA08_FUNC_CTL_UART3_RTS: u8 = 3;
    pub const IOC_PA08_FUNC_CTL_XPI0_CB_D_0: u8 = 14;
    pub const IOC_PA09_FUNC_CTL_CAN0_RXD: u8 = 7;
    pub const IOC_PA09_FUNC_CTL_ETH0_RXD_1: u8 = 18;
    pub const IOC_PA09_FUNC_CTL_GPIO_A_09: u8 = 0;
    pub const IOC_PA09_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PA09_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PA09_FUNC_CTL_SDC0_DATA_3: u8 = 17;
    pub const IOC_PA09_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PA09_FUNC_CTL_UART3_CTS: u8 = 3;
    pub const IOC_PA09_FUNC_CTL_XPI0_CB_D_2: u8 = 14;
    pub const IOC_PA10_FUNC_CTL_CAN1_TXD: u8 = 7;
    pub const IOC_PA10_FUNC_CTL_ETH0_RXD_0: u8 = 18;
    pub const IOC_PA10_FUNC_CTL_GPIO_A_10: u8 = 0;
    pub const IOC_PA10_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PA10_FUNC_CTL_SDC0_CMD: u8 = 17;
    pub const IOC_PA10_FUNC_CTL_SPI0_CSN: u8 = 5;
    pub const IOC_PA10_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PA10_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PA10_FUNC_CTL_XPI0_CB_D_1: u8 = 14;
    pub const IOC_PA11_FUNC_CTL_CAN1_RXD: u8 = 7;
    pub const IOC_PA11_FUNC_CTL_ETH0_RXDV: u8 = 18;
    pub const IOC_PA11_FUNC_CTL_GPIO_A_11: u8 = 0;
    pub const IOC_PA11_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PA11_FUNC_CTL_SDC0_CLK: u8 = 17;
    pub const IOC_PA11_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PA11_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PA11_FUNC_CTL_XPI0_CB_SCLK: u8 = 14;
    pub const IOC_PA12_FUNC_CTL_ETH0_REFCLK: u8 = 18;
    pub const IOC_PA12_FUNC_CTL_GPIO_A_12: u8 = 0;
    pub const IOC_PA12_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PA12_FUNC_CTL_SDC0_DATA_0: u8 = 17;
    pub const IOC_PA12_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PA12_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PA12_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PA12_FUNC_CTL_XPI0_CB_D_3: u8 = 14;
    pub const IOC_PA13_FUNC_CTL_ETH0_MDIO: u8 = 19;
    pub const IOC_PA13_FUNC_CTL_ETH0_RXER: u8 = 18;
    pub const IOC_PA13_FUNC_CTL_GPIO_A_13: u8 = 0;
    pub const IOC_PA13_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PA13_FUNC_CTL_PDM0_D_3: u8 = 10;
    pub const IOC_PA13_FUNC_CTL_SDC0_DATA_1: u8 = 17;
    pub const IOC_PA13_FUNC_CTL_SOC_REF1: u8 = 24;
    pub const IOC_PA13_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PA13_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PA13_FUNC_CTL_XPI0_CB_DQS: u8 = 14;
    pub const IOC_PA14_FUNC_CTL_ETH0_MDC: u8 = 19;
    pub const IOC_PA14_FUNC_CTL_ETH0_RXD_3: u8 = 18;
    pub const IOC_PA14_FUNC_CTL_GPIO_A_14: u8 = 0;
    pub const IOC_PA14_FUNC_CTL_PDM0_D_2: u8 = 10;
    pub const IOC_PA14_FUNC_CTL_PWM1_P_5: u8 = 16;
    pub const IOC_PA14_FUNC_CTL_SDC0_CDN: u8 = 17;
    pub const IOC_PA14_FUNC_CTL_SOC_REF0: u8 = 24;
    pub const IOC_PA14_FUNC_CTL_SPI0_DAT3: u8 = 5;
    pub const IOC_PA14_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PA14_FUNC_CTL_XPI0_CB_CS1: u8 = 14;
    pub const IOC_PA15_FUNC_CTL_ETH0_MDIO: u8 = 19;
    pub const IOC_PA15_FUNC_CTL_ETH0_RXD_2: u8 = 18;
    pub const IOC_PA15_FUNC_CTL_GPIO_A_15: u8 = 0;
    pub const IOC_PA15_FUNC_CTL_PDM0_CLK: u8 = 10;
    pub const IOC_PA15_FUNC_CTL_PWM1_P_4: u8 = 16;
    pub const IOC_PA15_FUNC_CTL_SDC0_WP: u8 = 17;
    pub const IOC_PA15_FUNC_CTL_SPI0_DAT2: u8 = 5;
    pub const IOC_PA15_FUNC_CTL_SYSCTL_CLK_OBS_3: u8 = 24;
    pub const IOC_PA15_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PA15_FUNC_CTL_XPI0_CB_CS0: u8 = 14;
    pub const IOC_PA16_FUNC_CTL_ETH0_MDC: u8 = 19;
    pub const IOC_PA16_FUNC_CTL_ETH0_RXCK: u8 = 18;
    pub const IOC_PA16_FUNC_CTL_GPIO_A_16: u8 = 0;
    pub const IOC_PA16_FUNC_CTL_PDM0_D_1: u8 = 10;
    pub const IOC_PA16_FUNC_CTL_PWM1_P_3: u8 = 16;
    pub const IOC_PA16_FUNC_CTL_SDC0_VSEL: u8 = 17;
    pub const IOC_PA16_FUNC_CTL_SPI1_CSN: u8 = 5;
    pub const IOC_PA16_FUNC_CTL_SYSCTL_CLK_OBS_2: u8 = 24;
    pub const IOC_PA16_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PA17_FUNC_CTL_ETH0_RXD_1: u8 = 18;
    pub const IOC_PA17_FUNC_CTL_GPIO_A_17: u8 = 0;
    pub const IOC_PA17_FUNC_CTL_PDM0_D_0: u8 = 10;
    pub const IOC_PA17_FUNC_CTL_PWM1_P_2: u8 = 16;
    pub const IOC_PA17_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PA17_FUNC_CTL_SYSCTL_CLK_OBS_1: u8 = 24;
    pub const IOC_PA17_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PA18_FUNC_CTL_ETH0_RXD_0: u8 = 18;
    pub const IOC_PA18_FUNC_CTL_GPIO_A_18: u8 = 0;
    pub const IOC_PA18_FUNC_CTL_PDM0_CLK: u8 = 10;
    pub const IOC_PA18_FUNC_CTL_PWM1_P_1: u8 = 16;
    pub const IOC_PA18_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PA18_FUNC_CTL_SYSCTL_CLK_OBS_0: u8 = 24;
    pub const IOC_PA18_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PA19_FUNC_CTL_DAOR_P: u8 = 10;
    pub const IOC_PA19_FUNC_CTL_ETH0_RXDV: u8 = 18;
    pub const IOC_PA19_FUNC_CTL_GPIO_A_19: u8 = 0;
    pub const IOC_PA19_FUNC_CTL_GPTMR0_CAPT_0: u8 = 1;
    pub const IOC_PA19_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PA19_FUNC_CTL_PWM1_P_0: u8 = 16;
    pub const IOC_PA19_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PA19_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PA20_FUNC_CTL_DAOR_N: u8 = 10;
    pub const IOC_PA20_FUNC_CTL_ETH0_TXD_0: u8 = 18;
    pub const IOC_PA20_FUNC_CTL_GPIO_A_20: u8 = 0;
    pub const IOC_PA20_FUNC_CTL_GPTMR0_CAPT_1: u8 = 1;
    pub const IOC_PA20_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PA20_FUNC_CTL_TRGM1_P_00: u8 = 16;
    pub const IOC_PA20_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PA21_FUNC_CTL_CAN0_TXD: u8 = 7;
    pub const IOC_PA21_FUNC_CTL_DAOL_P: u8 = 10;
    pub const IOC_PA21_FUNC_CTL_ETH0_TXD_1: u8 = 18;
    pub const IOC_PA21_FUNC_CTL_GPIO_A_21: u8 = 0;
    pub const IOC_PA21_FUNC_CTL_GPTMR0_COMP_0: u8 = 1;
    pub const IOC_PA21_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PA21_FUNC_CTL_TRGM1_P_01: u8 = 16;
    pub const IOC_PA21_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PA22_FUNC_CTL_CAN0_RXD: u8 = 7;
    pub const IOC_PA22_FUNC_CTL_DAOL_N: u8 = 10;
    pub const IOC_PA22_FUNC_CTL_ETH0_REFCLK: u8 = 18;
    pub const IOC_PA22_FUNC_CTL_GPIO_A_22: u8 = 0;
    pub const IOC_PA22_FUNC_CTL_GPTMR0_COMP_1: u8 = 1;
    pub const IOC_PA22_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PA22_FUNC_CTL_TRGM1_P_02: u8 = 16;
    pub const IOC_PA22_FUNC_CTL_UART6_DE: u8 = 2;
    pub const IOC_PA22_FUNC_CTL_UART6_RTS: u8 = 3;
    pub const IOC_PA23_FUNC_CTL_CAN0_STBY: u8 = 7;
    pub const IOC_PA23_FUNC_CTL_ETH0_TXEN: u8 = 18;
    pub const IOC_PA23_FUNC_CTL_FEMC_CS_1: u8 = 12;
    pub const IOC_PA23_FUNC_CTL_GPIO_A_23: u8 = 0;
    pub const IOC_PA23_FUNC_CTL_GPTMR1_CAPT_0: u8 = 1;
    pub const IOC_PA23_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PA23_FUNC_CTL_TRGM1_P_03: u8 = 16;
    pub const IOC_PA23_FUNC_CTL_UART6_CTS: u8 = 3;
    pub const IOC_PA24_FUNC_CTL_CAN1_STBY: u8 = 7;
    pub const IOC_PA24_FUNC_CTL_ETH0_TXD_2: u8 = 18;
    pub const IOC_PA24_FUNC_CTL_FEMC_SCLK: u8 = 12;
    pub const IOC_PA24_FUNC_CTL_GPIO_A_24: u8 = 0;
    pub const IOC_PA24_FUNC_CTL_GPTMR1_CAPT_1: u8 = 1;
    pub const IOC_PA24_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PA24_FUNC_CTL_TRGM1_P_04: u8 = 16;
    pub const IOC_PA24_FUNC_CTL_UART7_DE: u8 = 2;
    pub const IOC_PA24_FUNC_CTL_UART7_RTS: u8 = 3;
    pub const IOC_PA25_FUNC_CTL_CAN1_TXD: u8 = 7;
    pub const IOC_PA25_FUNC_CTL_ETH0_MDC: u8 = 19;
    pub const IOC_PA25_FUNC_CTL_ETH0_TXD_3: u8 = 18;
    pub const IOC_PA25_FUNC_CTL_FEMC_DQ_07: u8 = 12;
    pub const IOC_PA25_FUNC_CTL_GPIO_A_25: u8 = 0;
    pub const IOC_PA25_FUNC_CTL_GPTMR1_COMP_0: u8 = 1;
    pub const IOC_PA25_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PA25_FUNC_CTL_TRGM1_P_05: u8 = 16;
    pub const IOC_PA25_FUNC_CTL_UART7_CTS: u8 = 3;
    pub const IOC_PA26_FUNC_CTL_CAN1_RXD: u8 = 7;
    pub const IOC_PA26_FUNC_CTL_ETH0_CRS: u8 = 18;
    pub const IOC_PA26_FUNC_CTL_ETH0_MDIO: u8 = 19;
    pub const IOC_PA26_FUNC_CTL_FEMC_DQ_06: u8 = 12;
    pub const IOC_PA26_FUNC_CTL_GPIO_A_26: u8 = 0;
    pub const IOC_PA26_FUNC_CTL_GPTMR1_COMP_1: u8 = 1;
    pub const IOC_PA26_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PA26_FUNC_CTL_TRGM1_P_06: u8 = 16;
    pub const IOC_PA26_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PA26_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PA27_FUNC_CTL_ETH0_COL: u8 = 18;
    pub const IOC_PA27_FUNC_CTL_FEMC_DQ_05: u8 = 12;
    pub const IOC_PA27_FUNC_CTL_GPIO_A_27: u8 = 0;
    pub const IOC_PA27_FUNC_CTL_TRGM1_P_07: u8 = 16;
    pub const IOC_PA27_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PA28_FUNC_CTL_FEMC_DQ_04: u8 = 12;
    pub const IOC_PA28_FUNC_CTL_GPIO_A_28: u8 = 0;
    pub const IOC_PA28_FUNC_CTL_SPI0_CSN: u8 = 5;
    pub const IOC_PA28_FUNC_CTL_TRGM1_P_08: u8 = 16;
    pub const IOC_PA28_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PA28_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PA29_FUNC_CTL_CAN0_TXD: u8 = 7;
    pub const IOC_PA29_FUNC_CTL_FEMC_DQ_03: u8 = 12;
    pub const IOC_PA29_FUNC_CTL_GPIO_A_29: u8 = 0;
    pub const IOC_PA29_FUNC_CTL_SPI0_MISO: u8 = 5;
    pub const IOC_PA29_FUNC_CTL_TRGM1_P_09: u8 = 16;
    pub const IOC_PA29_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PA30_FUNC_CTL_CAN0_RXD: u8 = 7;
    pub const IOC_PA30_FUNC_CTL_FEMC_DQ_02: u8 = 12;
    pub const IOC_PA30_FUNC_CTL_GPIO_A_30: u8 = 0;
    pub const IOC_PA30_FUNC_CTL_SPI0_SCLK: u8 = 5;
    pub const IOC_PA30_FUNC_CTL_TRGM1_P_10: u8 = 16;
    pub const IOC_PA30_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PA31_FUNC_CTL_FEMC_DQ_01: u8 = 12;
    pub const IOC_PA31_FUNC_CTL_GPIO_A_31: u8 = 0;
    pub const IOC_PA31_FUNC_CTL_SPI0_MOSI: u8 = 5;
    pub const IOC_PA31_FUNC_CTL_TRGM1_P_11: u8 = 16;
    pub const IOC_PA31_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PB00_FUNC_CTL_FEMC_DQ_00: u8 = 12;
    pub const IOC_PB00_FUNC_CTL_GPIO_B_00: u8 = 0;
    pub const IOC_PB00_FUNC_CTL_PWM1_P_0: u8 = 16;
    pub const IOC_PB00_FUNC_CTL_SPI0_DAT2: u8 = 5;
    pub const IOC_PB00_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PB01_FUNC_CTL_FEMC_DM_0: u8 = 12;
    pub const IOC_PB01_FUNC_CTL_GPIO_B_01: u8 = 0;
    pub const IOC_PB01_FUNC_CTL_PWM1_P_1: u8 = 16;
    pub const IOC_PB01_FUNC_CTL_SPI0_DAT3: u8 = 5;
    pub const IOC_PB01_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PB02_FUNC_CTL_FEMC_DQ_08: u8 = 12;
    pub const IOC_PB02_FUNC_CTL_GPIO_B_02: u8 = 0;
    pub const IOC_PB02_FUNC_CTL_PWM1_P_2: u8 = 16;
    pub const IOC_PB02_FUNC_CTL_SPI1_CSN: u8 = 5;
    pub const IOC_PB02_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PB03_FUNC_CTL_FEMC_DQ_09: u8 = 12;
    pub const IOC_PB03_FUNC_CTL_GPIO_B_03: u8 = 0;
    pub const IOC_PB03_FUNC_CTL_PWM1_P_3: u8 = 16;
    pub const IOC_PB03_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PB03_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PB03_FUNC_CTL_XPI1_CB_CS0: u8 = 14;
    pub const IOC_PB04_FUNC_CTL_FEMC_DQ_10: u8 = 12;
    pub const IOC_PB04_FUNC_CTL_GPIO_B_04: u8 = 0;
    pub const IOC_PB04_FUNC_CTL_PWM1_P_4: u8 = 16;
    pub const IOC_PB04_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PB04_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PB04_FUNC_CTL_XPI1_CB_CS1: u8 = 14;
    pub const IOC_PB05_FUNC_CTL_FEMC_DQ_11: u8 = 12;
    pub const IOC_PB05_FUNC_CTL_GPIO_B_05: u8 = 0;
    pub const IOC_PB05_FUNC_CTL_PWM1_P_5: u8 = 16;
    pub const IOC_PB05_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PB05_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PB05_FUNC_CTL_XPI1_CB_DQS: u8 = 14;
    pub const IOC_PB06_FUNC_CTL_FEMC_DQ_12: u8 = 12;
    pub const IOC_PB06_FUNC_CTL_GPIO_B_06: u8 = 0;
    pub const IOC_PB06_FUNC_CTL_PWM1_P_6: u8 = 16;
    pub const IOC_PB06_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PB06_FUNC_CTL_XPI1_CB_D_3: u8 = 14;
    pub const IOC_PB07_FUNC_CTL_FEMC_DQ_13: u8 = 12;
    pub const IOC_PB07_FUNC_CTL_GPIO_B_07: u8 = 0;
    pub const IOC_PB07_FUNC_CTL_PWM1_P_7: u8 = 16;
    pub const IOC_PB07_FUNC_CTL_SPI1_DAT2: u8 = 5;
    pub const IOC_PB07_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PB07_FUNC_CTL_XPI1_CB_D_1: u8 = 14;
    pub const IOC_PB08_FUNC_CTL_FEMC_DQ_14: u8 = 12;
    pub const IOC_PB08_FUNC_CTL_GPIO_B_08: u8 = 0;
    pub const IOC_PB08_FUNC_CTL_PWM1_FAULT_0: u8 = 16;
    pub const IOC_PB08_FUNC_CTL_SPI1_DAT3: u8 = 5;
    pub const IOC_PB08_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PB08_FUNC_CTL_XPI1_CB_SCLK: u8 = 14;
    pub const IOC_PB09_FUNC_CTL_FEMC_DQ_15: u8 = 12;
    pub const IOC_PB09_FUNC_CTL_GPIO_B_09: u8 = 0;
    pub const IOC_PB09_FUNC_CTL_PWM1_FAULT_1: u8 = 16;
    pub const IOC_PB09_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PB09_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PB09_FUNC_CTL_XPI1_CB_D_2: u8 = 14;
    pub const IOC_PB10_FUNC_CTL_FEMC_DM_1: u8 = 12;
    pub const IOC_PB10_FUNC_CTL_GPIO_B_10: u8 = 0;
    pub const IOC_PB10_FUNC_CTL_PWM0_FAULT_1: u8 = 16;
    pub const IOC_PB10_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PB10_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PB10_FUNC_CTL_XPI1_CB_D_0: u8 = 14;
    pub const IOC_PB11_FUNC_CTL_FEMC_WE: u8 = 12;
    pub const IOC_PB11_FUNC_CTL_GPIO_B_11: u8 = 0;
    pub const IOC_PB11_FUNC_CTL_PWM0_FAULT_0: u8 = 16;
    pub const IOC_PB11_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PB11_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PB11_FUNC_CTL_XPI1_CA_DQS: u8 = 14;
    pub const IOC_PB12_FUNC_CTL_FEMC_CAS: u8 = 12;
    pub const IOC_PB12_FUNC_CTL_GPIO_B_12: u8 = 0;
    pub const IOC_PB12_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PB12_FUNC_CTL_SPI1_CSN: u8 = 5;
    pub const IOC_PB12_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PB12_FUNC_CTL_XPI1_CA_D_0: u8 = 14;
    pub const IOC_PB13_FUNC_CTL_FEMC_RAS: u8 = 12;
    pub const IOC_PB13_FUNC_CTL_GPIO_B_13: u8 = 0;
    pub const IOC_PB13_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PB13_FUNC_CTL_SPI2_CSN: u8 = 5;
    pub const IOC_PB13_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PB13_FUNC_CTL_XPI1_CA_D_2: u8 = 14;
    pub const IOC_PB14_FUNC_CTL_CAN1_TXD: u8 = 7;
    pub const IOC_PB14_FUNC_CTL_FEMC_CS_0: u8 = 12;
    pub const IOC_PB14_FUNC_CTL_GPIO_B_14: u8 = 0;
    pub const IOC_PB14_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PB14_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PB14_FUNC_CTL_UART6_DE: u8 = 2;
    pub const IOC_PB14_FUNC_CTL_UART6_RTS: u8 = 3;
    pub const IOC_PB14_FUNC_CTL_XPI1_CA_SCLK: u8 = 14;
    pub const IOC_PB15_FUNC_CTL_CAN1_RXD: u8 = 7;
    pub const IOC_PB15_FUNC_CTL_FEMC_BA0: u8 = 12;
    pub const IOC_PB15_FUNC_CTL_GPIO_B_15: u8 = 0;
    pub const IOC_PB15_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PB15_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PB15_FUNC_CTL_UART6_CTS: u8 = 3;
    pub const IOC_PB15_FUNC_CTL_XPI1_CA_D_1: u8 = 14;
    pub const IOC_PB16_FUNC_CTL_FEMC_BA1: u8 = 12;
    pub const IOC_PB16_FUNC_CTL_GPIO_B_16: u8 = 0;
    pub const IOC_PB16_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PB16_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PB16_FUNC_CTL_UART7_DE: u8 = 2;
    pub const IOC_PB16_FUNC_CTL_UART7_RTS: u8 = 3;
    pub const IOC_PB16_FUNC_CTL_XPI1_CA_D_3: u8 = 14;
    pub const IOC_PB17_FUNC_CTL_FEMC_A_10: u8 = 12;
    pub const IOC_PB17_FUNC_CTL_GPIO_B_17: u8 = 0;
    pub const IOC_PB17_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PB17_FUNC_CTL_UART7_CTS: u8 = 3;
    pub const IOC_PB17_FUNC_CTL_XPI1_CA_CS0: u8 = 14;
    pub const IOC_PB18_FUNC_CTL_CAN1_STBY: u8 = 7;
    pub const IOC_PB18_FUNC_CTL_FEMC_A_00: u8 = 12;
    pub const IOC_PB18_FUNC_CTL_GPIO_B_18: u8 = 0;
    pub const IOC_PB18_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PB18_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PB18_FUNC_CTL_I2S0_TXD_3: u8 = 8;
    pub const IOC_PB18_FUNC_CTL_PWM0_P_6: u8 = 16;
    pub const IOC_PB18_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PB18_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PB19_FUNC_CTL_CAN0_STBY: u8 = 7;
    pub const IOC_PB19_FUNC_CTL_FEMC_A_01: u8 = 12;
    pub const IOC_PB19_FUNC_CTL_GPIO_B_19: u8 = 0;
    pub const IOC_PB19_FUNC_CTL_GPTMR2_CAPT_1: u8 = 1;
    pub const IOC_PB19_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PB19_FUNC_CTL_I2S0_TXD_2: u8 = 8;
    pub const IOC_PB19_FUNC_CTL_PWM0_P_7: u8 = 16;
    pub const IOC_PB19_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PB20_FUNC_CTL_CAN0_TXD: u8 = 7;
    pub const IOC_PB20_FUNC_CTL_FEMC_A_02: u8 = 12;
    pub const IOC_PB20_FUNC_CTL_GPIO_B_20: u8 = 0;
    pub const IOC_PB20_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PB20_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PB20_FUNC_CTL_I2S0_TXD_1: u8 = 8;
    pub const IOC_PB20_FUNC_CTL_TRGM0_P_00: u8 = 16;
    pub const IOC_PB20_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PB20_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PB21_FUNC_CTL_CAN0_RXD: u8 = 7;
    pub const IOC_PB21_FUNC_CTL_FEMC_A_03: u8 = 12;
    pub const IOC_PB21_FUNC_CTL_GPIO_B_21: u8 = 0;
    pub const IOC_PB21_FUNC_CTL_GPTMR2_COMP_1: u8 = 1;
    pub const IOC_PB21_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PB21_FUNC_CTL_I2S0_TXD_0: u8 = 8;
    pub const IOC_PB21_FUNC_CTL_TRGM0_P_01: u8 = 16;
    pub const IOC_PB21_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PB22_FUNC_CTL_CAN1_TXD: u8 = 7;
    pub const IOC_PB22_FUNC_CTL_FEMC_CLK: u8 = 12;
    pub const IOC_PB22_FUNC_CTL_GPIO_B_22: u8 = 0;
    pub const IOC_PB22_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PB22_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PB22_FUNC_CTL_I2S0_BCLK: u8 = 8;
    pub const IOC_PB22_FUNC_CTL_SOC_REF0: u8 = 24;
    pub const IOC_PB22_FUNC_CTL_TRGM0_P_02: u8 = 16;
    pub const IOC_PB22_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PB23_FUNC_CTL_CAN1_RXD: u8 = 7;
    pub const IOC_PB23_FUNC_CTL_FEMC_CKE: u8 = 12;
    pub const IOC_PB23_FUNC_CTL_GPIO_B_23: u8 = 0;
    pub const IOC_PB23_FUNC_CTL_GPTMR3_CAPT_1: u8 = 1;
    pub const IOC_PB23_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PB23_FUNC_CTL_I2S0_FCLK: u8 = 8;
    pub const IOC_PB23_FUNC_CTL_SOC_REF1: u8 = 24;
    pub const IOC_PB23_FUNC_CTL_TRGM0_P_03: u8 = 16;
    pub const IOC_PB23_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PB24_FUNC_CTL_FEMC_A_12: u8 = 12;
    pub const IOC_PB24_FUNC_CTL_GPIO_B_24: u8 = 0;
    pub const IOC_PB24_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PB24_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PB24_FUNC_CTL_I2S0_MCLK: u8 = 8;
    pub const IOC_PB24_FUNC_CTL_TRGM0_P_04: u8 = 16;
    pub const IOC_PB24_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PB25_FUNC_CTL_FEMC_A_11: u8 = 12;
    pub const IOC_PB25_FUNC_CTL_GPIO_B_25: u8 = 0;
    pub const IOC_PB25_FUNC_CTL_GPTMR3_COMP_1: u8 = 1;
    pub const IOC_PB25_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PB25_FUNC_CTL_I2S0_RXD_0: u8 = 8;
    pub const IOC_PB25_FUNC_CTL_TRGM0_P_05: u8 = 16;
    pub const IOC_PB25_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PB26_FUNC_CTL_FEMC_A_09: u8 = 12;
    pub const IOC_PB26_FUNC_CTL_GPIO_B_26: u8 = 0;
    pub const IOC_PB26_FUNC_CTL_I2S0_RXD_1: u8 = 8;
    pub const IOC_PB26_FUNC_CTL_TRGM0_P_06: u8 = 16;
    pub const IOC_PB26_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PB27_FUNC_CTL_FEMC_A_08: u8 = 12;
    pub const IOC_PB27_FUNC_CTL_GPIO_B_27: u8 = 0;
    pub const IOC_PB27_FUNC_CTL_I2S0_RXD_2: u8 = 8;
    pub const IOC_PB27_FUNC_CTL_SPI1_CSN: u8 = 5;
    pub const IOC_PB27_FUNC_CTL_TRGM0_P_07: u8 = 16;
    pub const IOC_PB27_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PB28_FUNC_CTL_FEMC_A_07: u8 = 12;
    pub const IOC_PB28_FUNC_CTL_GPIO_B_28: u8 = 0;
    pub const IOC_PB28_FUNC_CTL_I2S0_RXD_3: u8 = 8;
    pub const IOC_PB28_FUNC_CTL_SPI1_MISO: u8 = 5;
    pub const IOC_PB28_FUNC_CTL_TRGM0_P_08: u8 = 16;
    pub const IOC_PB28_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PB29_FUNC_CTL_FEMC_A_06: u8 = 12;
    pub const IOC_PB29_FUNC_CTL_GPIO_B_29: u8 = 0;
    pub const IOC_PB29_FUNC_CTL_I2S0_TXD_3: u8 = 8;
    pub const IOC_PB29_FUNC_CTL_SPI1_SCLK: u8 = 5;
    pub const IOC_PB29_FUNC_CTL_TRGM0_P_09: u8 = 16;
    pub const IOC_PB29_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PB30_FUNC_CTL_FEMC_A_05: u8 = 12;
    pub const IOC_PB30_FUNC_CTL_GPIO_B_30: u8 = 0;
    pub const IOC_PB30_FUNC_CTL_I2S0_TXD_2: u8 = 8;
    pub const IOC_PB30_FUNC_CTL_SPI1_MOSI: u8 = 5;
    pub const IOC_PB30_FUNC_CTL_TRGM0_P_10: u8 = 16;
    pub const IOC_PB30_FUNC_CTL_UART2_DE: u8 = 2;
    pub const IOC_PB30_FUNC_CTL_UART2_RTS: u8 = 3;
    pub const IOC_PB31_FUNC_CTL_FEMC_A_04: u8 = 12;
    pub const IOC_PB31_FUNC_CTL_GPIO_B_31: u8 = 0;
    pub const IOC_PB31_FUNC_CTL_I2S0_TXD_1: u8 = 8;
    pub const IOC_PB31_FUNC_CTL_SPI2_CSN: u8 = 5;
    pub const IOC_PB31_FUNC_CTL_TRGM0_P_11: u8 = 16;
    pub const IOC_PB31_FUNC_CTL_UART2_CTS: u8 = 3;
    pub const IOC_PC00_FUNC_CTL_FEMC_SRDY: u8 = 12;
    pub const IOC_PC00_FUNC_CTL_GPIO_C_00: u8 = 0;
    pub const IOC_PC00_FUNC_CTL_I2S0_TXD_0: u8 = 8;
    pub const IOC_PC00_FUNC_CTL_PWM0_P_0: u8 = 16;
    pub const IOC_PC00_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PC00_FUNC_CTL_UART3_DE: u8 = 2;
    pub const IOC_PC00_FUNC_CTL_UART3_RTS: u8 = 3;
    pub const IOC_PC00_FUNC_CTL_USB0_ID: u8 = 24;
    pub const IOC_PC01_FUNC_CTL_FEMC_DQS: u8 = 12;
    pub const IOC_PC01_FUNC_CTL_GPIO_C_01: u8 = 0;
    pub const IOC_PC01_FUNC_CTL_I2S0_BCLK: u8 = 8;
    pub const IOC_PC01_FUNC_CTL_PWM0_P_1: u8 = 16;
    pub const IOC_PC01_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PC01_FUNC_CTL_UART3_CTS: u8 = 3;
    pub const IOC_PC01_FUNC_CTL_USB0_PWR: u8 = 24;
    pub const IOC_PC02_FUNC_CTL_GPIO_C_02: u8 = 0;
    pub const IOC_PC02_FUNC_CTL_I2S0_FCLK: u8 = 8;
    pub const IOC_PC02_FUNC_CTL_PWM0_P_2: u8 = 16;
    pub const IOC_PC02_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PC02_FUNC_CTL_UART4_DE: u8 = 2;
    pub const IOC_PC02_FUNC_CTL_UART4_RTS: u8 = 3;
    pub const IOC_PC02_FUNC_CTL_USB0_OC: u8 = 24;
    pub const IOC_PC03_FUNC_CTL_GPIO_C_03: u8 = 0;
    pub const IOC_PC03_FUNC_CTL_I2S0_MCLK: u8 = 8;
    pub const IOC_PC03_FUNC_CTL_I2S1_TXD_3: u8 = 9;
    pub const IOC_PC03_FUNC_CTL_PDM0_CLK: u8 = 10;
    pub const IOC_PC03_FUNC_CTL_PWM0_P_3: u8 = 16;
    pub const IOC_PC03_FUNC_CTL_SPI2_DAT2: u8 = 5;
    pub const IOC_PC03_FUNC_CTL_UART4_CTS: u8 = 3;
    pub const IOC_PC04_FUNC_CTL_GPIO_C_04: u8 = 0;
    pub const IOC_PC04_FUNC_CTL_I2S0_RXD_0: u8 = 8;
    pub const IOC_PC04_FUNC_CTL_I2S1_TXD_2: u8 = 9;
    pub const IOC_PC04_FUNC_CTL_PDM0_D_2: u8 = 10;
    pub const IOC_PC04_FUNC_CTL_PWM0_P_4: u8 = 16;
    pub const IOC_PC04_FUNC_CTL_SPI2_DAT3: u8 = 5;
    pub const IOC_PC04_FUNC_CTL_UART5_DE: u8 = 2;
    pub const IOC_PC04_FUNC_CTL_UART5_RTS: u8 = 3;
    pub const IOC_PC05_FUNC_CTL_GPIO_C_05: u8 = 0;
    pub const IOC_PC05_FUNC_CTL_I2S0_RXD_1: u8 = 8;
    pub const IOC_PC05_FUNC_CTL_I2S1_TXD_1: u8 = 9;
    pub const IOC_PC05_FUNC_CTL_PDM0_D_3: u8 = 10;
    pub const IOC_PC05_FUNC_CTL_PWM0_P_5: u8 = 16;
    pub const IOC_PC05_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PC05_FUNC_CTL_UART5_CTS: u8 = 3;
    pub const IOC_PC05_FUNC_CTL_USB0_OC: u8 = 24;
    pub const IOC_PC06_FUNC_CTL_GPIO_C_06: u8 = 0;
    pub const IOC_PC06_FUNC_CTL_GPTMR2_CAPT_0: u8 = 1;
    pub const IOC_PC06_FUNC_CTL_I2S0_RXD_2: u8 = 8;
    pub const IOC_PC06_FUNC_CTL_I2S1_TXD_0: u8 = 9;
    pub const IOC_PC06_FUNC_CTL_PDM0_CLK: u8 = 10;
    pub const IOC_PC06_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PC06_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PC06_FUNC_CTL_USB0_ID: u8 = 24;
    pub const IOC_PC07_FUNC_CTL_ETH0_MDIO: u8 = 19;
    pub const IOC_PC07_FUNC_CTL_GPIO_C_07: u8 = 0;
    pub const IOC_PC07_FUNC_CTL_GPTMR2_CAPT_1: u8 = 1;
    pub const IOC_PC07_FUNC_CTL_I2S0_RXD_3: u8 = 8;
    pub const IOC_PC07_FUNC_CTL_I2S1_MCLK: u8 = 9;
    pub const IOC_PC07_FUNC_CTL_PDM0_D_0: u8 = 10;
    pub const IOC_PC07_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PC07_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PC07_FUNC_CTL_USB0_OC: u8 = 24;
    pub const IOC_PC08_FUNC_CTL_ETH0_MDC: u8 = 19;
    pub const IOC_PC08_FUNC_CTL_GPIO_C_08: u8 = 0;
    pub const IOC_PC08_FUNC_CTL_GPTMR2_COMP_0: u8 = 1;
    pub const IOC_PC08_FUNC_CTL_I2S1_FCLK: u8 = 9;
    pub const IOC_PC08_FUNC_CTL_PDM0_D_1: u8 = 10;
    pub const IOC_PC08_FUNC_CTL_SPI2_CSN: u8 = 5;
    pub const IOC_PC08_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PC08_FUNC_CTL_USB0_PWR: u8 = 24;
    pub const IOC_PC09_FUNC_CTL_CAN0_TXD: u8 = 7;
    pub const IOC_PC09_FUNC_CTL_DAOR_N: u8 = 10;
    pub const IOC_PC09_FUNC_CTL_GPIO_C_09: u8 = 0;
    pub const IOC_PC09_FUNC_CTL_GPTMR2_COMP_1: u8 = 1;
    pub const IOC_PC09_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PC09_FUNC_CTL_I2S1_BCLK: u8 = 9;
    pub const IOC_PC09_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PC10_FUNC_CTL_CAN0_RXD: u8 = 7;
    pub const IOC_PC10_FUNC_CTL_DAOR_P: u8 = 10;
    pub const IOC_PC10_FUNC_CTL_GPIO_C_10: u8 = 0;
    pub const IOC_PC10_FUNC_CTL_GPTMR3_CAPT_0: u8 = 1;
    pub const IOC_PC10_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PC10_FUNC_CTL_I2S1_RXD_0: u8 = 9;
    pub const IOC_PC10_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PC11_FUNC_CTL_CAN0_STBY: u8 = 7;
    pub const IOC_PC11_FUNC_CTL_DAOL_N: u8 = 10;
    pub const IOC_PC11_FUNC_CTL_GPIO_C_11: u8 = 0;
    pub const IOC_PC11_FUNC_CTL_GPTMR3_CAPT_1: u8 = 1;
    pub const IOC_PC11_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PC11_FUNC_CTL_I2S1_RXD_1: u8 = 9;
    pub const IOC_PC11_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const IOC_PC12_FUNC_CTL_CAN1_STBY: u8 = 7;
    pub const IOC_PC12_FUNC_CTL_DAOL_P: u8 = 10;
    pub const IOC_PC12_FUNC_CTL_GPIO_C_12: u8 = 0;
    pub const IOC_PC12_FUNC_CTL_GPTMR3_COMP_0: u8 = 1;
    pub const IOC_PC12_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PC12_FUNC_CTL_I2S1_RXD_2: u8 = 9;
    pub const IOC_PC12_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PC13_FUNC_CTL_CAN1_TXD: u8 = 7;
    pub const IOC_PC13_FUNC_CTL_GPIO_C_13: u8 = 0;
    pub const IOC_PC13_FUNC_CTL_GPTMR3_COMP_1: u8 = 1;
    pub const IOC_PC13_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PC13_FUNC_CTL_I2S1_RXD_3: u8 = 9;
    pub const IOC_PC13_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PC14_FUNC_CTL_ACMP_COMP_0: u8 = 16;
    pub const IOC_PC14_FUNC_CTL_CAN1_RXD: u8 = 7;
    pub const IOC_PC14_FUNC_CTL_GPIO_C_14: u8 = 0;
    pub const IOC_PC14_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PC14_FUNC_CTL_I2S1_MCLK: u8 = 9;
    pub const IOC_PC14_FUNC_CTL_UART6_DE: u8 = 2;
    pub const IOC_PC14_FUNC_CTL_UART6_RTS: u8 = 3;
    pub const IOC_PC15_FUNC_CTL_ACMP_COMP_1: u8 = 16;
    pub const IOC_PC15_FUNC_CTL_GPIO_C_15: u8 = 0;
    pub const IOC_PC15_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PC15_FUNC_CTL_UART6_CTS: u8 = 3;
    pub const IOC_PC16_FUNC_CTL_GPIO_C_16: u8 = 0;
    pub const IOC_PC16_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PC16_FUNC_CTL_I2S1_TXD_3: u8 = 9;
    pub const IOC_PC16_FUNC_CTL_UART7_DE: u8 = 2;
    pub const IOC_PC16_FUNC_CTL_UART7_RTS: u8 = 3;
    pub const IOC_PC17_FUNC_CTL_GPIO_C_17: u8 = 0;
    pub const IOC_PC17_FUNC_CTL_I2S1_TXD_2: u8 = 9;
    pub const IOC_PC17_FUNC_CTL_PDM0_D_1: u8 = 10;
    pub const IOC_PC17_FUNC_CTL_UART7_CTS: u8 = 3;
    pub const IOC_PC18_FUNC_CTL_GPIO_C_18: u8 = 0;
    pub const IOC_PC18_FUNC_CTL_I2S1_TXD_1: u8 = 9;
    pub const IOC_PC18_FUNC_CTL_PDM0_D_0: u8 = 10;
    pub const IOC_PC18_FUNC_CTL_SPI3_CSN: u8 = 5;
    pub const IOC_PC18_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PC18_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PC19_FUNC_CTL_GPIO_C_19: u8 = 0;
    pub const IOC_PC19_FUNC_CTL_I2S1_TXD_0: u8 = 9;
    pub const IOC_PC19_FUNC_CTL_PDM0_CLK: u8 = 10;
    pub const IOC_PC19_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PC19_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PC20_FUNC_CTL_ETH0_EVTO_1: u8 = 19;
    pub const IOC_PC20_FUNC_CTL_GPIO_C_20: u8 = 0;
    pub const IOC_PC20_FUNC_CTL_I2S1_MCLK: u8 = 9;
    pub const IOC_PC20_FUNC_CTL_PDM0_D_3: u8 = 10;
    pub const IOC_PC20_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PC20_FUNC_CTL_UART1_DE: u8 = 2;
    pub const IOC_PC20_FUNC_CTL_UART1_RTS: u8 = 3;
    pub const IOC_PC20_FUNC_CTL_WDG0_RST: u8 = 24;
    pub const IOC_PC21_FUNC_CTL_ETH0_EVTO_0: u8 = 19;
    pub const IOC_PC21_FUNC_CTL_GPIO_C_21: u8 = 0;
    pub const IOC_PC21_FUNC_CTL_I2S1_FCLK: u8 = 9;
    pub const IOC_PC21_FUNC_CTL_PDM0_D_2: u8 = 10;
    pub const IOC_PC21_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PC21_FUNC_CTL_UART1_CTS: u8 = 3;
    pub const IOC_PC21_FUNC_CTL_WDG1_RST: u8 = 24;
    pub const IOC_PC22_FUNC_CTL_ETH0_MDIO: u8 = 19;
    pub const IOC_PC22_FUNC_CTL_GPIO_C_22: u8 = 0;
    pub const IOC_PC22_FUNC_CTL_I2S1_BCLK: u8 = 9;
    pub const IOC_PC22_FUNC_CTL_PDM0_CLK: u8 = 10;
    pub const IOC_PC22_FUNC_CTL_SDC0_WP: u8 = 17;
    pub const IOC_PC22_FUNC_CTL_SPI2_CSN: u8 = 5;
    pub const IOC_PC22_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PC23_FUNC_CTL_ETH0_MDC: u8 = 19;
    pub const IOC_PC23_FUNC_CTL_GPIO_C_23: u8 = 0;
    pub const IOC_PC23_FUNC_CTL_I2S1_MCLK: u8 = 9;
    pub const IOC_PC23_FUNC_CTL_SDC0_VSEL: u8 = 17;
    pub const IOC_PC23_FUNC_CTL_SPI2_MOSI: u8 = 5;
    pub const IOC_PC23_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PC24_FUNC_CTL_GPIO_C_24: u8 = 0;
    pub const IOC_PC24_FUNC_CTL_I2S1_RXD_0: u8 = 9;
    pub const IOC_PC24_FUNC_CTL_SDC0_CDN: u8 = 17;
    pub const IOC_PC24_FUNC_CTL_SPI2_MISO: u8 = 5;
    pub const IOC_PC24_FUNC_CTL_UART1_TXD: u8 = 2;
    pub const IOC_PC25_FUNC_CTL_GPIO_C_25: u8 = 0;
    pub const IOC_PC25_FUNC_CTL_I2S1_RXD_1: u8 = 9;
    pub const IOC_PC25_FUNC_CTL_SDC0_WP: u8 = 17;
    pub const IOC_PC25_FUNC_CTL_SPI2_SCLK: u8 = 5;
    pub const IOC_PC25_FUNC_CTL_UART1_RXD: u8 = 2;
    pub const IOC_PC26_FUNC_CTL_ETH0_EVTI_1: u8 = 19;
    pub const IOC_PC26_FUNC_CTL_GPIO_C_26: u8 = 0;
    pub const IOC_PC26_FUNC_CTL_I2S1_RXD_2: u8 = 9;
    pub const IOC_PC26_FUNC_CTL_SDC0_VSEL: u8 = 17;
    pub const IOC_PC26_FUNC_CTL_SPI2_DAT3: u8 = 5;
    pub const IOC_PC26_FUNC_CTL_UART2_TXD: u8 = 2;
    pub const IOC_PC27_FUNC_CTL_ETH0_EVTI_0: u8 = 19;
    pub const IOC_PC27_FUNC_CTL_GPIO_C_27: u8 = 0;
    pub const IOC_PC27_FUNC_CTL_I2S1_RXD_3: u8 = 9;
    pub const IOC_PC27_FUNC_CTL_SDC0_CDN: u8 = 17;
    pub const IOC_PC27_FUNC_CTL_SPI2_DAT2: u8 = 5;
    pub const IOC_PC27_FUNC_CTL_UART2_RXD: u8 = 2;
    pub const IOC_PX00_FUNC_CTL_GPIO_X_00: u8 = 0;
    pub const IOC_PX00_FUNC_CTL_XPI0_CA_D_2: u8 = 14;
    pub const IOC_PX01_FUNC_CTL_GPIO_X_01: u8 = 0;
    pub const IOC_PX01_FUNC_CTL_XPI0_CA_D_1: u8 = 14;
    pub const IOC_PX02_FUNC_CTL_GPIO_X_02: u8 = 0;
    pub const IOC_PX02_FUNC_CTL_XPI0_CA_CS0: u8 = 14;
    pub const IOC_PX03_FUNC_CTL_GPIO_X_03: u8 = 0;
    pub const IOC_PX03_FUNC_CTL_XPI0_CA_D_0: u8 = 14;
    pub const IOC_PX04_FUNC_CTL_GPIO_X_04: u8 = 0;
    pub const IOC_PX04_FUNC_CTL_XPI0_CA_SCLK: u8 = 14;
    pub const IOC_PX05_FUNC_CTL_GPIO_X_05: u8 = 0;
    pub const IOC_PX05_FUNC_CTL_XPI0_CA_D_3: u8 = 14;
    pub const IOC_PX06_FUNC_CTL_GPIO_X_06: u8 = 0;
    pub const IOC_PX06_FUNC_CTL_XPI0_CA_DQS: u8 = 14;
    pub const IOC_PX07_FUNC_CTL_FEMC_DQS: u8 = 12;
    pub const IOC_PX07_FUNC_CTL_GPIO_X_07: u8 = 0;
    pub const IOC_PX07_FUNC_CTL_XPI1_CA_DQS: u8 = 14;
    pub const IOC_PY00_FUNC_CTL_CAN0_TXD: u8 = 7;
    pub const IOC_PY00_FUNC_CTL_GPIO_Y_00: u8 = 0;
    pub const IOC_PY00_FUNC_CTL_SPI3_CSN: u8 = 5;
    pub const IOC_PY00_FUNC_CTL_UART7_DE: u8 = 2;
    pub const IOC_PY00_FUNC_CTL_UART7_RTS: u8 = 3;
    pub const IOC_PY01_FUNC_CTL_CAN0_RXD: u8 = 7;
    pub const IOC_PY01_FUNC_CTL_GPIO_Y_01: u8 = 0;
    pub const IOC_PY01_FUNC_CTL_SPI3_MISO: u8 = 5;
    pub const IOC_PY01_FUNC_CTL_UART7_CTS: u8 = 3;
    pub const IOC_PY02_FUNC_CTL_GPIO_Y_02: u8 = 0;
    pub const IOC_PY02_FUNC_CTL_SPI3_SCLK: u8 = 5;
    pub const IOC_PY02_FUNC_CTL_UART0_DE: u8 = 2;
    pub const IOC_PY02_FUNC_CTL_UART0_RTS: u8 = 3;
    pub const IOC_PY03_FUNC_CTL_GPIO_Y_03: u8 = 0;
    pub const IOC_PY03_FUNC_CTL_SPI3_MOSI: u8 = 5;
    pub const IOC_PY03_FUNC_CTL_UART0_CTS: u8 = 3;
    pub const IOC_PY04_FUNC_CTL_CAN1_TXD: u8 = 7;
    pub const IOC_PY04_FUNC_CTL_DAOR_P: u8 = 10;
    pub const IOC_PY04_FUNC_CTL_GPIO_Y_04: u8 = 0;
    pub const IOC_PY04_FUNC_CTL_I2C0_SCL: u8 = 4;
    pub const IOC_PY04_FUNC_CTL_UART7_TXD: u8 = 2;
    pub const IOC_PY04_FUNC_CTL_WDG0_RST: u8 = 24;
    pub const IOC_PY05_FUNC_CTL_CAN1_RXD: u8 = 7;
    pub const IOC_PY05_FUNC_CTL_DAOR_N: u8 = 10;
    pub const IOC_PY05_FUNC_CTL_GPIO_Y_05: u8 = 0;
    pub const IOC_PY05_FUNC_CTL_I2C0_SDA: u8 = 4;
    pub const IOC_PY05_FUNC_CTL_UART7_RXD: u8 = 2;
    pub const IOC_PY05_FUNC_CTL_WDG1_RST: u8 = 24;
    pub const IOC_PY06_FUNC_CTL_DAOL_P: u8 = 10;
    pub const IOC_PY06_FUNC_CTL_GPIO_Y_06: u8 = 0;
    pub const IOC_PY06_FUNC_CTL_I2C1_SCL: u8 = 4;
    pub const IOC_PY06_FUNC_CTL_UART0_TXD: u8 = 2;
    pub const IOC_PY07_FUNC_CTL_DAOL_N: u8 = 10;
    pub const IOC_PY07_FUNC_CTL_GPIO_Y_07: u8 = 0;
    pub const IOC_PY07_FUNC_CTL_I2C1_SDA: u8 = 4;
    pub const IOC_PY07_FUNC_CTL_UART0_RXD: u8 = 2;
    pub const IOC_PZ00_FUNC_CTL_CAN0_TXD: u8 = 7;
    pub const IOC_PZ00_FUNC_CTL_GPIO_Z_00: u8 = 0;
    pub const IOC_PZ00_FUNC_CTL_UART3_TXD: u8 = 2;
    pub const IOC_PZ01_FUNC_CTL_CAN0_RXD: u8 = 7;
    pub const IOC_PZ01_FUNC_CTL_GPIO_Z_01: u8 = 0;
    pub const IOC_PZ01_FUNC_CTL_UART3_RXD: u8 = 2;
    pub const IOC_PZ02_FUNC_CTL_GPIO_Z_02: u8 = 0;
    pub const IOC_PZ02_FUNC_CTL_I2C2_SCL: u8 = 4;
    pub const IOC_PZ02_FUNC_CTL_UART4_TXD: u8 = 2;
    pub const IOC_PZ03_FUNC_CTL_GPIO_Z_03: u8 = 0;
    pub const IOC_PZ03_FUNC_CTL_I2C2_SDA: u8 = 4;
    pub const IOC_PZ03_FUNC_CTL_UART4_RXD: u8 = 2;
    pub const IOC_PZ04_FUNC_CTL_CAN1_TXD: u8 = 7;
    pub const IOC_PZ04_FUNC_CTL_GPIO_Z_04: u8 = 0;
    pub const IOC_PZ04_FUNC_CTL_UART5_TXD: u8 = 2;
    pub const IOC_PZ05_FUNC_CTL_CAN1_RXD: u8 = 7;
    pub const IOC_PZ05_FUNC_CTL_GPIO_Z_05: u8 = 0;
    pub const IOC_PZ05_FUNC_CTL_UART5_RXD: u8 = 2;
    pub const IOC_PZ06_FUNC_CTL_GPIO_Z_06: u8 = 0;
    pub const IOC_PZ06_FUNC_CTL_I2C3_SCL: u8 = 4;
    pub const IOC_PZ06_FUNC_CTL_UART6_TXD: u8 = 2;
    pub const IOC_PZ07_FUNC_CTL_GPIO_Z_07: u8 = 0;
    pub const IOC_PZ07_FUNC_CTL_I2C3_SDA: u8 = 4;
    pub const IOC_PZ07_FUNC_CTL_UART6_RXD: u8 = 2;
    pub const PIOC_PY00_FUNC_CTL_JTAG_TDO: u8 = 1;
    pub const PIOC_PY00_FUNC_CTL_PGPIO_Y_00: u8 = 0;
    pub const PIOC_PY00_FUNC_CTL_PTMR_COMP_0: u8 = 2;
    pub const PIOC_PY00_FUNC_CTL_SOC_PY_00: u8 = 3;
    pub const PIOC_PY01_FUNC_CTL_JTAG_TDI: u8 = 1;
    pub const PIOC_PY01_FUNC_CTL_PGPIO_Y_01: u8 = 0;
    pub const PIOC_PY01_FUNC_CTL_PTMR_CAPT_0: u8 = 2;
    pub const PIOC_PY01_FUNC_CTL_SOC_PY_01: u8 = 3;
    pub const PIOC_PY02_FUNC_CTL_JTAG_TCK: u8 = 1;
    pub const PIOC_PY02_FUNC_CTL_PGPIO_Y_02: u8 = 0;
    pub const PIOC_PY02_FUNC_CTL_PTMR_COMP_1: u8 = 2;
    pub const PIOC_PY02_FUNC_CTL_SOC_PY_02: u8 = 3;
    pub const PIOC_PY03_FUNC_CTL_JTAG_TMS: u8 = 1;
    pub const PIOC_PY03_FUNC_CTL_PGPIO_Y_03: u8 = 0;
    pub const PIOC_PY03_FUNC_CTL_PTMR_CAPT_1: u8 = 2;
    pub const PIOC_PY03_FUNC_CTL_SOC_PY_03: u8 = 3;
    pub const PIOC_PY04_FUNC_CTL_JTAG_TRST: u8 = 1;
    pub const PIOC_PY04_FUNC_CTL_PGPIO_Y_04: u8 = 0;
    pub const PIOC_PY04_FUNC_CTL_PTMR_COMP_2: u8 = 2;
    pub const PIOC_PY04_FUNC_CTL_SOC_PY_04: u8 = 3;
    pub const PIOC_PY05_FUNC_CTL_PGPIO_Y_05: u8 = 0;
    pub const PIOC_PY05_FUNC_CTL_PTMR_CAPT_2: u8 = 2;
    pub const PIOC_PY05_FUNC_CTL_PWDG_RST: u8 = 1;
    pub const PIOC_PY05_FUNC_CTL_SOC_PY_05: u8 = 3;
    pub const PIOC_PY06_FUNC_CTL_PGPIO_Y_06: u8 = 0;
    pub const PIOC_PY06_FUNC_CTL_PTMR_COMP_3: u8 = 2;
    pub const PIOC_PY06_FUNC_CTL_PUART_TXD: u8 = 1;
    pub const PIOC_PY06_FUNC_CTL_SOC_PY_06: u8 = 3;
    pub const PIOC_PY07_FUNC_CTL_PGPIO_Y_07: u8 = 0;
    pub const PIOC_PY07_FUNC_CTL_PTMR_CAPT_3: u8 = 2;
    pub const PIOC_PY07_FUNC_CTL_PUART_RXD: u8 = 1;
    pub const PIOC_PY07_FUNC_CTL_SOC_PY_07: u8 = 3;
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
    pub const TRGM0_DMA_SRC_QEI0: usize = 27;
    pub const TRGM0_DMA_SRC_HALL0: usize = 28;
    pub const TRGM0_FILTER_SRC_PWM0_IN0: usize = 0;
    pub const TRGM0_FILTER_SRC_PWM0_IN1: usize = 1;
    pub const TRGM0_FILTER_SRC_PWM0_IN2: usize = 2;
    pub const TRGM0_FILTER_SRC_PWM0_IN3: usize = 3;
    pub const TRGM0_FILTER_SRC_PWM0_IN4: usize = 4;
    pub const TRGM0_FILTER_SRC_PWM0_IN5: usize = 5;
    pub const TRGM0_FILTER_SRC_PWM0_IN6: usize = 6;
    pub const TRGM0_FILTER_SRC_PWM0_IN7: usize = 7;
    pub const TRGM0_FILTER_SRC_TRGM0_IN0: usize = 8;
    pub const TRGM0_FILTER_SRC_TRGM0_IN1: usize = 9;
    pub const TRGM0_FILTER_SRC_TRGM0_IN2: usize = 10;
    pub const TRGM0_FILTER_SRC_TRGM0_IN3: usize = 11;
    pub const TRGM0_FILTER_SRC_TRGM0_IN4: usize = 12;
    pub const TRGM0_FILTER_SRC_TRGM0_IN5: usize = 13;
    pub const TRGM0_FILTER_SRC_TRGM0_IN6: usize = 14;
    pub const TRGM0_FILTER_SRC_TRGM0_IN7: usize = 15;
    pub const TRGM0_FILTER_SRC_TRGM0_IN8: usize = 16;
    pub const TRGM0_FILTER_SRC_TRGM0_IN9: usize = 17;
    pub const TRGM0_FILTER_SRC_TRGM0_IN10: usize = 18;
    pub const TRGM0_FILTER_SRC_TRGM0_IN11: usize = 19;
    pub const TRGM0_INPUT_SRC_VSS: usize = 0;
    pub const TRGM0_INPUT_SRC_VDD: usize = 1;
    pub const TRGM0_INPUT_SRC_TRGM0_P0: usize = 2;
    pub const TRGM0_INPUT_SRC_TRGM0_P1: usize = 3;
    pub const TRGM0_INPUT_SRC_TRGM0_P2: usize = 4;
    pub const TRGM0_INPUT_SRC_TRGM0_P3: usize = 5;
    pub const TRGM0_INPUT_SRC_TRGM0_P4: usize = 6;
    pub const TRGM0_INPUT_SRC_TRGM0_P5: usize = 7;
    pub const TRGM0_INPUT_SRC_TRGM0_P6: usize = 8;
    pub const TRGM0_INPUT_SRC_TRGM0_P7: usize = 9;
    pub const TRGM0_INPUT_SRC_TRGM0_P8: usize = 10;
    pub const TRGM0_INPUT_SRC_TRGM0_P9: usize = 11;
    pub const TRGM0_INPUT_SRC_TRGM0_P10: usize = 12;
    pub const TRGM0_INPUT_SRC_TRGM0_P11: usize = 13;
    pub const TRGM0_INPUT_SRC_TRGM1_OUTX0: usize = 18;
    pub const TRGM0_INPUT_SRC_TRGM1_OUTX1: usize = 19;
    pub const TRGM0_INPUT_SRC_PWM0_CH8REF: usize = 20;
    pub const TRGM0_INPUT_SRC_PWM0_CH9REF: usize = 21;
    pub const TRGM0_INPUT_SRC_PWM0_CH10REF: usize = 22;
    pub const TRGM0_INPUT_SRC_PWM0_CH11REF: usize = 23;
    pub const TRGM0_INPUT_SRC_PWM0_CH12REF: usize = 24;
    pub const TRGM0_INPUT_SRC_PWM0_CH13REF: usize = 25;
    pub const TRGM0_INPUT_SRC_PWM0_CH14REF: usize = 26;
    pub const TRGM0_INPUT_SRC_PWM0_CH15REF: usize = 27;
    pub const TRGM0_INPUT_SRC_PWM0_CH16REF: usize = 28;
    pub const TRGM0_INPUT_SRC_PWM0_CH17REF: usize = 29;
    pub const TRGM0_INPUT_SRC_PWM0_CH18REF: usize = 30;
    pub const TRGM0_INPUT_SRC_PWM0_CH19REF: usize = 31;
    pub const TRGM0_INPUT_SRC_PWM0_CH20REF: usize = 32;
    pub const TRGM0_INPUT_SRC_PWM0_CH21REF: usize = 33;
    pub const TRGM0_INPUT_SRC_PWM0_CH22REF: usize = 34;
    pub const TRGM0_INPUT_SRC_PWM0_CH23REF: usize = 35;
    pub const TRGM0_INPUT_SRC_QEI0_TRGO: usize = 36;
    pub const TRGM0_INPUT_SRC_HALL0_TRGO: usize = 37;
    pub const TRGM0_INPUT_SRC_USB0_SOF: usize = 38;
    pub const TRGM0_INPUT_SRC_NTMR0_CH1_OUT: usize = 39;
    pub const TRGM0_INPUT_SRC_ENET0_PTP_OUT3: usize = 40;
    pub const TRGM0_INPUT_SRC_NTMR0_CH0_OUT: usize = 41;
    pub const TRGM0_INPUT_SRC_PTPC_CMP0: usize = 42;
    pub const TRGM0_INPUT_SRC_PTPC_CMP1: usize = 43;
    pub const TRGM0_INPUT_SRC_SYNT0_CH0: usize = 44;
    pub const TRGM0_INPUT_SRC_SYNT0_CH1: usize = 45;
    pub const TRGM0_INPUT_SRC_SYNT0_CH2: usize = 46;
    pub const TRGM0_INPUT_SRC_SYNT0_CH3: usize = 47;
    pub const TRGM0_INPUT_SRC_GPTMR0_OUT2: usize = 48;
    pub const TRGM0_INPUT_SRC_GPTMR0_OUT3: usize = 49;
    pub const TRGM0_INPUT_SRC_GPTMR1_OUT2: usize = 50;
    pub const TRGM0_INPUT_SRC_GPTMR1_OUT3: usize = 51;
    pub const TRGM0_INPUT_SRC_ACMP0_OUT: usize = 52;
    pub const TRGM0_INPUT_SRC_ACMP1_OUT: usize = 53;
    pub const TRGM0_INPUT_SRC_DEBUG_FLAG: usize = 56;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P0: usize = 0;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P1: usize = 1;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P2: usize = 2;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P3: usize = 3;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P4: usize = 4;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P5: usize = 5;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P6: usize = 6;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P7: usize = 7;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P8: usize = 8;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P9: usize = 9;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P10: usize = 10;
    pub const TRGM0_OUTPUT_SRC_TRGM0_P11: usize = 11;
    pub const TRGM0_OUTPUT_SRC_TRGM0_OUTX0: usize = 12;
    pub const TRGM0_OUTPUT_SRC_TRGM0_OUTX1: usize = 13;
    pub const TRGM0_OUTPUT_SRC_PWM0_SYNCI: usize = 14;
    pub const TRGM0_OUTPUT_SRC_PWM0_FRCI: usize = 15;
    pub const TRGM0_OUTPUT_SRC_PWM0_FRCSYNCI: usize = 16;
    pub const TRGM0_OUTPUT_SRC_PWM0_SHRLDSYNCI: usize = 17;
    pub const TRGM0_OUTPUT_SRC_PWM0_FAULTI0: usize = 18;
    pub const TRGM0_OUTPUT_SRC_PWM0_FAULTI1: usize = 19;
    pub const TRGM0_OUTPUT_SRC_PWM0_FAULTI2: usize = 20;
    pub const TRGM0_OUTPUT_SRC_PWM0_FAULTI3: usize = 21;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN8: usize = 22;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN9: usize = 23;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN10: usize = 24;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN11: usize = 25;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN12: usize = 26;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN13: usize = 27;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN14: usize = 28;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN15: usize = 29;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN16: usize = 30;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN17: usize = 31;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN18: usize = 32;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN19: usize = 33;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN20: usize = 34;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN21: usize = 35;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN22: usize = 36;
    pub const TRGM0_OUTPUT_SRC_PWM0_IN23: usize = 37;
    pub const TRGM0_OUTPUT_SRC_QEI0_A: usize = 38;
    pub const TRGM0_OUTPUT_SRC_QEI0_B: usize = 39;
    pub const TRGM0_OUTPUT_SRC_QEI0_Z: usize = 40;
    pub const TRGM0_OUTPUT_SRC_QEI0_H: usize = 41;
    pub const TRGM0_OUTPUT_SRC_QEI0_PAUSE: usize = 42;
    pub const TRGM0_OUTPUT_SRC_QEI0_SNAPI: usize = 43;
    pub const TRGM0_OUTPUT_SRC_HALL0_U: usize = 44;
    pub const TRGM0_OUTPUT_SRC_HALL0_V: usize = 45;
    pub const TRGM0_OUTPUT_SRC_HALL0_W: usize = 46;
    pub const TRGM0_OUTPUT_SRC_HALL0_SNAPI: usize = 47;
    pub const TRGM0_OUTPUT_SRC_ADC0_STRGI_ADCX_PTRGI2A: usize = 48;
    pub const TRGM0_OUTPUT_SRC_ADC1_STRGI_ADCX_PTRGI2B: usize = 49;
    pub const TRGM0_OUTPUT_SRC_ADC2_STRGI_ADCX_PTRGI2C: usize = 50;
    pub const TRGM0_OUTPUT_SRC_DAC_BUFF_TRIGGER: usize = 51;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI0A: usize = 52;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI0B: usize = 53;
    pub const TRGM0_OUTPUT_SRC_ADCX_PTRGI0C: usize = 54;
    pub const TRGM0_OUTPUT_SRC_GPTMR0_SYNCI: usize = 55;
    pub const TRGM0_OUTPUT_SRC_GPTMR0_IN2: usize = 56;
    pub const TRGM0_OUTPUT_SRC_GPTMR0_IN3: usize = 57;
    pub const TRGM0_OUTPUT_SRC_GPTMR1_SYNCI: usize = 58;
    pub const TRGM0_OUTPUT_SRC_GPTMR1_IN2: usize = 59;
    pub const TRGM0_OUTPUT_SRC_GPTMR1_IN3: usize = 60;
    pub const TRGM0_OUTPUT_SRC_ACMP0_WIN: usize = 61;
    pub const TRGM0_OUTPUT_SRC_PTPC_CAP0: usize = 62;
    pub const TRGM0_OUTPUT_SRC_PTPC_CAP1: usize = 63;
    pub const TRGM0_OUTPUT_SRC_DAC_STEP_TRIGGER_IN0: usize = 64;
    pub const TRGM0_OUTPUT_SRC_DAC_STEP_TRIGGER_IN1: usize = 65;
    pub const TRGM0_OUTPUT_SRC_DAC_STEP_TRIGGER_IN2: usize = 66;
    pub const TRGM0_OUTPUT_SRC_DAC_STEP_TRIGGER_IN3: usize = 67;
    pub const TRGM1_DMA_SRC_PWM1_CMP0: usize = 0;
    pub const TRGM1_DMA_SRC_PWM1_CMP1: usize = 1;
    pub const TRGM1_DMA_SRC_PWM1_CMP2: usize = 2;
    pub const TRGM1_DMA_SRC_PWM1_CMP3: usize = 3;
    pub const TRGM1_DMA_SRC_PWM1_CMP4: usize = 4;
    pub const TRGM1_DMA_SRC_PWM1_CMP5: usize = 5;
    pub const TRGM1_DMA_SRC_PWM1_CMP6: usize = 6;
    pub const TRGM1_DMA_SRC_PWM1_CMP7: usize = 7;
    pub const TRGM1_DMA_SRC_PWM1_CMP8: usize = 8;
    pub const TRGM1_DMA_SRC_PWM1_CMP9: usize = 9;
    pub const TRGM1_DMA_SRC_PWM1_CMP10: usize = 10;
    pub const TRGM1_DMA_SRC_PWM1_CMP11: usize = 11;
    pub const TRGM1_DMA_SRC_PWM1_CMP12: usize = 12;
    pub const TRGM1_DMA_SRC_PWM1_CMP13: usize = 13;
    pub const TRGM1_DMA_SRC_PWM1_CMP14: usize = 14;
    pub const TRGM1_DMA_SRC_PWM1_CMP15: usize = 15;
    pub const TRGM1_DMA_SRC_PWM1_CMP16: usize = 16;
    pub const TRGM1_DMA_SRC_PWM1_CMP17: usize = 17;
    pub const TRGM1_DMA_SRC_PWM1_CMP18: usize = 18;
    pub const TRGM1_DMA_SRC_PWM1_CMP19: usize = 19;
    pub const TRGM1_DMA_SRC_PWM1_CMP20: usize = 20;
    pub const TRGM1_DMA_SRC_PWM1_CMP21: usize = 21;
    pub const TRGM1_DMA_SRC_PWM1_CMP22: usize = 22;
    pub const TRGM1_DMA_SRC_PWM1_CMP23: usize = 23;
    pub const TRGM1_DMA_SRC_PWM1_RLD: usize = 24;
    pub const TRGM1_DMA_SRC_PWM1_HALFRLD: usize = 25;
    pub const TRGM1_DMA_SRC_PWM1_XRLD: usize = 26;
    pub const TRGM1_DMA_SRC_QEI1: usize = 27;
    pub const TRGM1_DMA_SRC_HALL1: usize = 28;
    pub const TRGM1_FILTER_SRC_PWM1_IN0: usize = 0;
    pub const TRGM1_FILTER_SRC_PWM1_IN1: usize = 1;
    pub const TRGM1_FILTER_SRC_PWM1_IN2: usize = 2;
    pub const TRGM1_FILTER_SRC_PWM1_IN3: usize = 3;
    pub const TRGM1_FILTER_SRC_PWM1_IN4: usize = 4;
    pub const TRGM1_FILTER_SRC_PWM1_IN5: usize = 5;
    pub const TRGM1_FILTER_SRC_PWM1_IN6: usize = 6;
    pub const TRGM1_FILTER_SRC_PWM1_IN7: usize = 7;
    pub const TRGM1_FILTER_SRC_TRGM1_IN0: usize = 8;
    pub const TRGM1_FILTER_SRC_TRGM1_IN1: usize = 9;
    pub const TRGM1_FILTER_SRC_TRGM1_IN2: usize = 10;
    pub const TRGM1_FILTER_SRC_TRGM1_IN3: usize = 11;
    pub const TRGM1_FILTER_SRC_TRGM1_IN4: usize = 12;
    pub const TRGM1_FILTER_SRC_TRGM1_IN5: usize = 13;
    pub const TRGM1_FILTER_SRC_TRGM1_IN6: usize = 14;
    pub const TRGM1_FILTER_SRC_TRGM1_IN7: usize = 15;
    pub const TRGM1_FILTER_SRC_TRGM1_IN8: usize = 16;
    pub const TRGM1_FILTER_SRC_TRGM1_IN9: usize = 17;
    pub const TRGM1_FILTER_SRC_TRGM1_IN10: usize = 18;
    pub const TRGM1_FILTER_SRC_TRGM1_IN11: usize = 19;
    pub const TRGM1_INPUT_SRC_VSS: usize = 0;
    pub const TRGM1_INPUT_SRC_VDD: usize = 1;
    pub const TRGM1_INPUT_SRC_TRGM1_P0: usize = 2;
    pub const TRGM1_INPUT_SRC_TRGM1_P1: usize = 3;
    pub const TRGM1_INPUT_SRC_TRGM1_P2: usize = 4;
    pub const TRGM1_INPUT_SRC_TRGM1_P3: usize = 5;
    pub const TRGM1_INPUT_SRC_TRGM1_P4: usize = 6;
    pub const TRGM1_INPUT_SRC_TRGM1_P5: usize = 7;
    pub const TRGM1_INPUT_SRC_TRGM1_P6: usize = 8;
    pub const TRGM1_INPUT_SRC_TRGM1_P7: usize = 9;
    pub const TRGM1_INPUT_SRC_TRGM1_P8: usize = 10;
    pub const TRGM1_INPUT_SRC_TRGM1_P9: usize = 11;
    pub const TRGM1_INPUT_SRC_TRGM1_P10: usize = 12;
    pub const TRGM1_INPUT_SRC_TRGM1_P11: usize = 13;
    pub const TRGM1_INPUT_SRC_TRGM0_OUTX0: usize = 18;
    pub const TRGM1_INPUT_SRC_TRGM0_OUTX1: usize = 19;
    pub const TRGM1_INPUT_SRC_PWM1_CH8REF: usize = 20;
    pub const TRGM1_INPUT_SRC_PWM1_CH9REF: usize = 21;
    pub const TRGM1_INPUT_SRC_PWM1_CH10REF: usize = 22;
    pub const TRGM1_INPUT_SRC_PWM1_CH11REF: usize = 23;
    pub const TRGM1_INPUT_SRC_PWM1_CH12REF: usize = 24;
    pub const TRGM1_INPUT_SRC_PWM1_CH13REF: usize = 25;
    pub const TRGM1_INPUT_SRC_PWM1_CH14REF: usize = 26;
    pub const TRGM1_INPUT_SRC_PWM1_CH15REF: usize = 27;
    pub const TRGM1_INPUT_SRC_PWM1_CH16REF: usize = 28;
    pub const TRGM1_INPUT_SRC_PWM1_CH17REF: usize = 29;
    pub const TRGM1_INPUT_SRC_PWM1_CH18REF: usize = 30;
    pub const TRGM1_INPUT_SRC_PWM1_CH19REF: usize = 31;
    pub const TRGM1_INPUT_SRC_PWM1_CH20REF: usize = 32;
    pub const TRGM1_INPUT_SRC_PWM1_CH21REF: usize = 33;
    pub const TRGM1_INPUT_SRC_PWM1_CH22REF: usize = 34;
    pub const TRGM1_INPUT_SRC_PWM1_CH23REF: usize = 35;
    pub const TRGM1_INPUT_SRC_QEI1_TRGO: usize = 36;
    pub const TRGM1_INPUT_SRC_HALL1_TRGO: usize = 37;
    pub const TRGM1_INPUT_SRC_USB0_SOF: usize = 38;
    pub const TRGM1_INPUT_SRC_NTMR0_CH1_OUT: usize = 39;
    pub const TRGM1_INPUT_SRC_ENET0_PTP_OUT3: usize = 40;
    pub const TRGM1_INPUT_SRC_NTMR0_CH0_OUT: usize = 41;
    pub const TRGM1_INPUT_SRC_PTPC_CMP0: usize = 42;
    pub const TRGM1_INPUT_SRC_PTPC_CMP1: usize = 43;
    pub const TRGM1_INPUT_SRC_SYNT0_CH0: usize = 44;
    pub const TRGM1_INPUT_SRC_SYNT0_CH1: usize = 45;
    pub const TRGM1_INPUT_SRC_SYNT0_CH2: usize = 46;
    pub const TRGM1_INPUT_SRC_SYNT0_CH3: usize = 47;
    pub const TRGM1_INPUT_SRC_GPTMR2_OUT2: usize = 48;
    pub const TRGM1_INPUT_SRC_GPTMR2_OUT3: usize = 49;
    pub const TRGM1_INPUT_SRC_GPTMR3_OUT2: usize = 50;
    pub const TRGM1_INPUT_SRC_GPTMR3_OUT3: usize = 51;
    pub const TRGM1_INPUT_SRC_ACMP0_OUT: usize = 52;
    pub const TRGM1_INPUT_SRC_ACMP1_OUT: usize = 53;
    pub const TRGM1_INPUT_SRC_DEBUG_FLAG: usize = 56;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P0: usize = 0;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P1: usize = 1;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P2: usize = 2;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P3: usize = 3;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P4: usize = 4;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P5: usize = 5;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P6: usize = 6;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P7: usize = 7;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P8: usize = 8;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P9: usize = 9;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P10: usize = 10;
    pub const TRGM1_OUTPUT_SRC_TRGM1_P11: usize = 11;
    pub const TRGM1_OUTPUT_SRC_TRGM1_OUTX0: usize = 12;
    pub const TRGM1_OUTPUT_SRC_TRGM1_OUTX1: usize = 13;
    pub const TRGM1_OUTPUT_SRC_PWM1_SYNCI: usize = 14;
    pub const TRGM1_OUTPUT_SRC_PWM1_FRCI: usize = 15;
    pub const TRGM1_OUTPUT_SRC_PWM1_FRCSYNCI: usize = 16;
    pub const TRGM1_OUTPUT_SRC_PWM1_SHRLDSYNCI: usize = 17;
    pub const TRGM1_OUTPUT_SRC_PWM1_FAULTI0: usize = 18;
    pub const TRGM1_OUTPUT_SRC_PWM1_FAULTI1: usize = 19;
    pub const TRGM1_OUTPUT_SRC_PWM1_FAULTI2: usize = 20;
    pub const TRGM1_OUTPUT_SRC_PWM1_FAULTI3: usize = 21;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN8: usize = 22;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN9: usize = 23;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN10: usize = 24;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN11: usize = 25;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN12: usize = 26;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN13: usize = 27;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN14: usize = 28;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN15: usize = 29;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN16: usize = 30;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN17: usize = 31;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN18: usize = 32;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN19: usize = 33;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN20: usize = 34;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN21: usize = 35;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN22: usize = 36;
    pub const TRGM1_OUTPUT_SRC_PWM1_IN23: usize = 37;
    pub const TRGM1_OUTPUT_SRC_QEI1_A: usize = 38;
    pub const TRGM1_OUTPUT_SRC_QEI1_B: usize = 39;
    pub const TRGM1_OUTPUT_SRC_QEI1_Z: usize = 40;
    pub const TRGM1_OUTPUT_SRC_QEI1_H: usize = 41;
    pub const TRGM1_OUTPUT_SRC_QEI1_PAUSE: usize = 42;
    pub const TRGM1_OUTPUT_SRC_QEI1_SNAPI: usize = 43;
    pub const TRGM1_OUTPUT_SRC_HALL1_U: usize = 44;
    pub const TRGM1_OUTPUT_SRC_HALL1_V: usize = 45;
    pub const TRGM1_OUTPUT_SRC_HALL1_W: usize = 46;
    pub const TRGM1_OUTPUT_SRC_HALL1_SNAPI: usize = 47;
    pub const TRGM1_OUTPUT_SRC_ADC0_STRGI_ADCX_PTRGI3A: usize = 48;
    pub const TRGM1_OUTPUT_SRC_ADC1_STRGI_ADCX_PTRGI3B: usize = 49;
    pub const TRGM1_OUTPUT_SRC_ADC2_STRGI_ADCX_PTRGI3C: usize = 50;
    pub const TRGM1_OUTPUT_SRC_DAC_BUFF_TRIGGER: usize = 51;
    pub const TRGM1_OUTPUT_SRC_ADCX_PTRGI1A: usize = 52;
    pub const TRGM1_OUTPUT_SRC_ADCX_PTRGI1B: usize = 53;
    pub const TRGM1_OUTPUT_SRC_ADCX_PTRGI1C: usize = 54;
    pub const TRGM1_OUTPUT_SRC_GPTMR2_SYNCI: usize = 55;
    pub const TRGM1_OUTPUT_SRC_GPTMR2_IN2: usize = 56;
    pub const TRGM1_OUTPUT_SRC_GPTMR2_IN3: usize = 57;
    pub const TRGM1_OUTPUT_SRC_GPTMR3_SYNCI: usize = 58;
    pub const TRGM1_OUTPUT_SRC_GPTMR3_IN2: usize = 59;
    pub const TRGM1_OUTPUT_SRC_GPTMR3_IN3: usize = 60;
    pub const TRGM1_OUTPUT_SRC_ACMP1_WIN: usize = 61;
    pub const TRGM1_OUTPUT_SRC_PTPC_CAP0: usize = 62;
    pub const TRGM1_OUTPUT_SRC_PTPC_CAP1: usize = 63;
    pub const TRGM1_OUTPUT_SRC_DAC_STEP_TRIGGER_IN0: usize = 64;
    pub const TRGM1_OUTPUT_SRC_DAC_STEP_TRIGGER_IN1: usize = 65;
    pub const TRGM1_OUTPUT_SRC_DAC_STEP_TRIGGER_IN2: usize = 66;
    pub const TRGM1_OUTPUT_SRC_DAC_STEP_TRIGGER_IN3: usize = 67;
}
