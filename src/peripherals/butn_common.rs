#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BUTN."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Butn {
    ptr: *mut u8,
}
unsafe impl Send for Butn {}
unsafe impl Sync for Butn {}
impl Butn {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Button status."]
    #[inline(always)]
    pub const fn btn_status(self) -> crate::common::Reg<regs::BtnStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Button interrupt mask."]
    #[inline(always)]
    pub const fn btn_irq_mask(self) -> crate::common::Reg<regs::BtnIrqMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Debounce setting."]
    #[inline(always)]
    pub const fn led_intense(self) -> crate::common::Reg<regs::LedIntense, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
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
    #[doc = "Button interrupt mask."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BtnIrqMask(pub u32);
    impl BtnIrqMask {
        #[doc = "Power button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[must_use]
        #[inline(always)]
        pub const fn pbtn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Power button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub const fn set_pbtn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Wake button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[must_use]
        #[inline(always)]
        pub const fn wbtn(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Wake button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub const fn set_wbtn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Dual button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[must_use]
        #[inline(always)]
        pub const fn dbtn(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Dual button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub const fn set_dbtn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "power button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[must_use]
        #[inline(always)]
        pub const fn pclick(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "power button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn set_pclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "power button click status when wake button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[must_use]
        #[inline(always)]
        pub const fn xpclick(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "power button click status when wake button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn set_xpclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "wake button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[must_use]
        #[inline(always)]
        pub const fn wclick(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "wake button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn set_wclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "wake button click status when power button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[must_use]
        #[inline(always)]
        pub const fn xwclick(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "wake button click status when power button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn set_xwclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for BtnIrqMask {
        #[inline(always)]
        fn default() -> BtnIrqMask {
            BtnIrqMask(0)
        }
    }
    impl core::fmt::Debug for BtnIrqMask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BtnIrqMask")
                .field("pbtn", &self.pbtn())
                .field("wbtn", &self.wbtn())
                .field("dbtn", &self.dbtn())
                .field("pclick", &self.pclick())
                .field("xpclick", &self.xpclick())
                .field("wclick", &self.wclick())
                .field("xwclick", &self.xwclick())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BtnIrqMask {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BtnIrqMask {{ pbtn: {=u8:?}, wbtn: {=u8:?}, dbtn: {=u8:?}, pclick: {=u8:?}, xpclick: {=u8:?}, wclick: {=u8:?}, xwclick: {=u8:?} }}" , self . pbtn () , self . wbtn () , self . dbtn () , self . pclick () , self . xpclick () , self . wclick () , self . xwclick ())
        }
    }
    #[doc = "Button status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BtnStatus(pub u32);
    impl BtnStatus {
        #[doc = "Power button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[must_use]
        #[inline(always)]
        pub const fn pbtn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Power button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub const fn set_pbtn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Wake button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[must_use]
        #[inline(always)]
        pub const fn wbtn(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Wake button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub const fn set_wbtn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Dual button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[must_use]
        #[inline(always)]
        pub const fn dbtn(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Dual button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed."]
        #[inline(always)]
        pub const fn set_dbtn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "power button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[must_use]
        #[inline(always)]
        pub const fn pclick(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "power button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn set_pclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "power button click status when wake button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[must_use]
        #[inline(always)]
        pub const fn xpclick(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "power button click status when wake button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn set_xpclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "wake button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[must_use]
        #[inline(always)]
        pub const fn wclick(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "wake button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn set_wclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "wake button click status when power button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[must_use]
        #[inline(always)]
        pub const fn xwclick(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "wake button click status when power button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked."]
        #[inline(always)]
        pub const fn set_xwclick(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for BtnStatus {
        #[inline(always)]
        fn default() -> BtnStatus {
            BtnStatus(0)
        }
    }
    impl core::fmt::Debug for BtnStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BtnStatus")
                .field("pbtn", &self.pbtn())
                .field("wbtn", &self.wbtn())
                .field("dbtn", &self.dbtn())
                .field("pclick", &self.pclick())
                .field("xpclick", &self.xpclick())
                .field("wclick", &self.wclick())
                .field("xwclick", &self.xwclick())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BtnStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BtnStatus {{ pbtn: {=u8:?}, wbtn: {=u8:?}, dbtn: {=u8:?}, pclick: {=u8:?}, xpclick: {=u8:?}, wclick: {=u8:?}, xwclick: {=u8:?} }}" , self . pbtn () , self . wbtn () , self . dbtn () , self . pclick () , self . xpclick () , self . wclick () , self . xwclick ())
        }
    }
    #[doc = "Debounce setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LedIntense(pub u32);
    impl LedIntense {
        #[doc = "Pbutton brightness 0."]
        #[must_use]
        #[inline(always)]
        pub const fn pled(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Pbutton brightness 0."]
        #[inline(always)]
        pub const fn set_pled(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Rbutton brightness 0."]
        #[must_use]
        #[inline(always)]
        pub const fn rled(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Rbutton brightness 0."]
        #[inline(always)]
        pub const fn set_rled(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for LedIntense {
        #[inline(always)]
        fn default() -> LedIntense {
            LedIntense(0)
        }
    }
    impl core::fmt::Debug for LedIntense {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LedIntense")
                .field("pled", &self.pled())
                .field("rled", &self.rled())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LedIntense {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LedIntense {{ pled: {=u8:?}, rled: {=u8:?} }}",
                self.pled(),
                self.rled()
            )
        }
    }
}
