#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ACMP0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acmp {
    ptr: *mut u8,
}
unsafe impl Send for Acmp {}
unsafe impl Sync for Acmp {}
impl Acmp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn channel(self, n: usize) -> Channel {
        assert!(n < 2usize);
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel {
    ptr: *mut u8,
}
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configure Register."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DAC configure register."]
    #[inline(always)]
    pub const fn daccfg(self) -> crate::common::Reg<regs::Daccfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt request enable register."]
    #[inline(always)]
    pub const fn irqen(self) -> crate::common::Reg<regs::Irqen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "DMA request enable register."]
    #[inline(always)]
    pub const fn dmaen(self) -> crate::common::Reg<regs::Dmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
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
pub mod regs {
    #[doc = "Configure Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn fltlen(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
        #[inline(always)]
        pub const fn set_fltlen(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
        #[must_use]
        #[inline(always)]
        pub const fn syncen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
        #[inline(always)]
        pub const fn set_syncen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high."]
        #[must_use]
        #[inline(always)]
        pub const fn fltmode(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high."]
        #[inline(always)]
        pub const fn set_fltmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
        #[must_use]
        #[inline(always)]
        pub const fn opol(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
        #[inline(always)]
        pub const fn set_opol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn winen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled."]
        #[inline(always)]
        pub const fn set_winen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
        #[must_use]
        #[inline(always)]
        pub const fn fltbyps(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
        #[inline(always)]
        pub const fn set_fltbyps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpoen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled."]
        #[inline(always)]
        pub const fn set_cmpoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "MIN select, from pad_ai_acmp\\[7:1\\]
and dac_out."]
        #[must_use]
        #[inline(always)]
        pub const fn pinsel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "MIN select, from pad_ai_acmp\\[7:1\\]
and dac_out."]
        #[inline(always)]
        pub const fn set_pinsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "if set, the dac value is from moto system when valid if clr, use dac_cfg value."]
        #[must_use]
        #[inline(always)]
        pub const fn dac_trig_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "if set, the dac value is from moto system when valid if clr, use dac_cfg value."]
        #[inline(always)]
        pub const fn set_dac_trig_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "PIN select, from pad_ai_acmp\\[7:1\\]
and dac_out."]
        #[must_use]
        #[inline(always)]
        pub const fn minsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "PIN select, from pad_ai_acmp\\[7:1\\]
and dac_out."]
        #[inline(always)]
        pub const fn set_minsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled."]
        #[inline(always)]
        pub const fn set_cmpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn hpmode(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled."]
        #[inline(always)]
        pub const fn set_hpmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn dacen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled."]
        #[inline(always)]
        pub const fn set_dacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3."]
        #[must_use]
        #[inline(always)]
        pub const fn hyst(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3."]
        #[inline(always)]
        pub const fn set_hyst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    impl core::fmt::Debug for Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg")
                .field("fltlen", &self.fltlen())
                .field("syncen", &self.syncen())
                .field("fltmode", &self.fltmode())
                .field("opol", &self.opol())
                .field("winen", &self.winen())
                .field("fltbyps", &self.fltbyps())
                .field("cmpoen", &self.cmpoen())
                .field("pinsel", &self.pinsel())
                .field("dac_trig_en", &self.dac_trig_en())
                .field("minsel", &self.minsel())
                .field("cmpen", &self.cmpen())
                .field("hpmode", &self.hpmode())
                .field("dacen", &self.dacen())
                .field("hyst", &self.hyst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfg {{ fltlen: {=u16:?}, syncen: {=bool:?}, fltmode: {=u8:?}, opol: {=bool:?}, winen: {=bool:?}, fltbyps: {=bool:?}, cmpoen: {=bool:?}, pinsel: {=u8:?}, dac_trig_en: {=bool:?}, minsel: {=u8:?}, cmpen: {=bool:?}, hpmode: {=bool:?}, dacen: {=bool:?}, hyst: {=u8:?} }}" , self . fltlen () , self . syncen () , self . fltmode () , self . opol () , self . winen () , self . fltbyps () , self . cmpoen () , self . pinsel () , self . dac_trig_en () , self . minsel () , self . cmpen () , self . hpmode () , self . dacen () , self . hyst ())
        }
    }
    #[doc = "DAC configure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Daccfg(pub u32);
    impl Daccfg {
        #[doc = "8bit DAC digital value output to analog block."]
        #[must_use]
        #[inline(always)]
        pub const fn daccfg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8bit DAC digital value output to analog block."]
        #[inline(always)]
        pub const fn set_daccfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Daccfg {
        #[inline(always)]
        fn default() -> Daccfg {
            Daccfg(0)
        }
    }
    impl core::fmt::Debug for Daccfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Daccfg")
                .field("daccfg", &self.daccfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Daccfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Daccfg {{ daccfg: {=u8:?} }}", self.daccfg())
        }
    }
    #[doc = "DMA request enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaen(pub u32);
    impl Dmaen {
        #[doc = "Output rising edge flag DMA request enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn redgen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Output rising edge flag DMA request enable bit."]
        #[inline(always)]
        pub const fn set_redgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output falling edge flag DMA request enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn fedgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output falling edge flag DMA request enable bit."]
        #[inline(always)]
        pub const fn set_fedgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Dmaen {
        #[inline(always)]
        fn default() -> Dmaen {
            Dmaen(0)
        }
    }
    impl core::fmt::Debug for Dmaen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmaen")
                .field("redgen", &self.redgen())
                .field("fedgen", &self.fedgen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmaen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dmaen {{ redgen: {=bool:?}, fedgen: {=bool:?} }}",
                self.redgen(),
                self.fedgen()
            )
        }
    }
    #[doc = "Interrupt request enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irqen(pub u32);
    impl Irqen {
        #[doc = "Output rising edge flag interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn redgen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Output rising edge flag interrupt enable bit."]
        #[inline(always)]
        pub const fn set_redgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output falling edge flag interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn fedgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output falling edge flag interrupt enable bit."]
        #[inline(always)]
        pub const fn set_fedgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Irqen {
        #[inline(always)]
        fn default() -> Irqen {
            Irqen(0)
        }
    }
    impl core::fmt::Debug for Irqen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irqen")
                .field("redgen", &self.redgen())
                .field("fedgen", &self.fedgen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Irqen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Irqen {{ redgen: {=bool:?}, fedgen: {=bool:?} }}",
                self.redgen(),
                self.fedgen()
            )
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Output rising edge flag. Write 1 to clear this flag."]
        #[must_use]
        #[inline(always)]
        pub const fn redgf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Output rising edge flag. Write 1 to clear this flag."]
        #[inline(always)]
        pub const fn set_redgf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output falling edge flag. Write 1 to clear this flag."]
        #[must_use]
        #[inline(always)]
        pub const fn fedgf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output falling edge flag. Write 1 to clear this flag."]
        #[inline(always)]
        pub const fn set_fedgf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("redgf", &self.redgf())
                .field("fedgf", &self.fedgf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ redgf: {=bool:?}, fedgf: {=bool:?} }}",
                self.redgf(),
                self.fedgf()
            )
        }
    }
}
