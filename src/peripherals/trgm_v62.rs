#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "TRGM0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trgm {
    ptr: *mut u8,
}
unsafe impl Send for Trgm {}
unsafe impl Sync for Trgm {}
impl Trgm {
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
    pub const fn filtcfg(self, n: usize) -> crate::common::Reg<regs::Filtcfg, crate::common::RW> {
        assert!(n < 20usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn trgocfg(self, n: usize) -> crate::common::Reg<regs::Trgocfg, crate::common::RW> {
        assert!(n < 68usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn dmacfg(self, n: usize) -> crate::common::Reg<regs::Dmacfg, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 4usize) as _)
        }
    }
    #[doc = "General Control Register."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
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
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacfg(pub u32);
    impl Dmacfg {
        #[doc = "This field selects one of the DMA requests as the DMA request output."]
        #[must_use]
        #[inline(always)]
        pub const fn dmasrcsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "This field selects one of the DMA requests as the DMA request output."]
        #[inline(always)]
        pub const fn set_dmasrcsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Dmacfg {
        #[inline(always)]
        fn default() -> Dmacfg {
            Dmacfg(0)
        }
    }
    impl core::fmt::Debug for Dmacfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmacfg")
                .field("dmasrcsel", &self.dmasrcsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmacfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmacfg {{ dmasrcsel: {=u8:?} }}", self.dmasrcsel())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Filtcfg(pub u32);
    impl Filtcfg {
        #[doc = "This bitfields defines the filter counter length."]
        #[must_use]
        #[inline(always)]
        pub const fn filtlen(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "This bitfields defines the filter counter length."]
        #[inline(always)]
        pub const fn set_filtlen(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "set to enable sychronization input signal with TRGM clock."]
        #[must_use]
        #[inline(always)]
        pub const fn syncen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable sychronization input signal with TRGM clock."]
        #[inline(always)]
        pub const fn set_syncen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::FilterMode {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::FilterMode::from_bits(val as u8)
        }
        #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::FilterMode) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "1- Filter will invert the output 0- Filter will not invert the output."]
        #[must_use]
        #[inline(always)]
        pub const fn outinv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "1- Filter will invert the output 0- Filter will not invert the output."]
        #[inline(always)]
        pub const fn set_outinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Filtcfg {
        #[inline(always)]
        fn default() -> Filtcfg {
            Filtcfg(0)
        }
    }
    impl core::fmt::Debug for Filtcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Filtcfg")
                .field("filtlen", &self.filtlen())
                .field("syncen", &self.syncen())
                .field("mode", &self.mode())
                .field("outinv", &self.outinv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Filtcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Filtcfg {{ filtlen: {=u16:?}, syncen: {=bool:?}, mode: {:?}, outinv: {=bool:?} }}",
                self.filtlen(),
                self.syncen(),
                self.mode(),
                self.outinv()
            )
        }
    }
    #[doc = "General Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "The bitfield enable the TRGM outputs."]
        #[must_use]
        #[inline(always)]
        pub const fn trgopen(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "The bitfield enable the TRGM outputs."]
        #[inline(always)]
        pub const fn set_trgopen(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    impl core::fmt::Debug for Gcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gcr")
                .field("trgopen", &self.trgopen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gcr {{ trgopen: {=u16:?} }}", self.trgopen())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trgocfg(pub u32);
    impl Trgocfg {
        #[doc = "This bitfield selects one of the TRGM inputs as output."]
        #[must_use]
        #[inline(always)]
        pub const fn trigosel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "This bitfield selects one of the TRGM inputs as output."]
        #[inline(always)]
        pub const fn set_trigosel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
        #[must_use]
        #[inline(always)]
        pub const fn redg2pen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
        #[inline(always)]
        pub const fn set_redg2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
        #[must_use]
        #[inline(always)]
        pub const fn fedg2pen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
        #[inline(always)]
        pub const fn set_fedg2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "1- Invert the output."]
        #[must_use]
        #[inline(always)]
        pub const fn outinv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "1- Invert the output."]
        #[inline(always)]
        pub const fn set_outinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Trgocfg {
        #[inline(always)]
        fn default() -> Trgocfg {
            Trgocfg(0)
        }
    }
    impl core::fmt::Debug for Trgocfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trgocfg")
                .field("trigosel", &self.trigosel())
                .field("redg2pen", &self.redg2pen())
                .field("fedg2pen", &self.fedg2pen())
                .field("outinv", &self.outinv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trgocfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Trgocfg {{ trigosel: {=u8:?}, redg2pen: {=bool:?}, fedg2pen: {=bool:?}, outinv: {=bool:?} }}" , self . trigosel () , self . redg2pen () , self . fedg2pen () , self . outinv ())
        }
    }
}
pub mod vals {
    #[doc = "Filter mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FilterMode {
        #[doc = "Bypass"]
        BYPASS = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "Rapid change mode"]
        RAPID_CHANGE = 0x04,
        #[doc = "Delay filter mode"]
        DELAY = 0x05,
        #[doc = "Stable low mode"]
        STABLE_LOW = 0x06,
        #[doc = "Stable high mode"]
        STABLE_HIGH = 0x07,
    }
    impl FilterMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FilterMode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FilterMode {
        #[inline(always)]
        fn from(val: u8) -> FilterMode {
            FilterMode::from_bits(val)
        }
    }
    impl From<FilterMode> for u8 {
        #[inline(always)]
        fn from(val: FilterMode) -> u8 {
            FilterMode::to_bits(val)
        }
    }
}
