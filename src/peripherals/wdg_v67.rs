#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "WDG0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdg {
    ptr: *mut u8,
}
unsafe impl Send for Wdg {}
unsafe impl Sync for Wdg {}
impl Wdg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Restart Register."]
    #[inline(always)]
    pub const fn restart(self) -> crate::common::Reg<regs::Restart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Write Protection Register."]
    #[inline(always)]
    pub const fn wr_en(self) -> crate::common::Reg<regs::WrEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn st(self) -> crate::common::Reg<regs::St, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
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
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Enable or disable the watchdog timer 0: Disable 1: Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable or disable the watchdog timer 0: Disable 1: Enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock source of timer: 0: EXTCLK 1: PCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock source of timer: 0: EXTCLK 1: PCLK."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable or disable the watchdog interrupt 0: Disable 1: Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn inten(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable or disable the watchdog interrupt 0: Disable 1: Enable."]
        #[inline(always)]
        pub const fn set_inten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable or disable the watchdog reset 0: Disable 1: Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rsten(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable or disable the watchdog reset 0: Disable 1: Enable."]
        #[inline(always)]
        pub const fn set_rsten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "The timer interval of the interrupt stage: 0: Clock period x 2^6 1: Clock period x 2^8 2: Clock period x 2^10 3: Clock period x 2^11 4: Clock period x 2^12 5: Clock period x 2^13 6: Clock period x 2^14 7: Clock period x 2^15 8: Clock period x 2^17 9: Clock period x 2^19 10: Clock period x 2^21 11: Clock period x 2^23 12: Clock period x 2^25 13: Clock period x 2^27 14: Clock period x 2^29 15: Clock period x 2^31."]
        #[must_use]
        #[inline(always)]
        pub const fn inttime(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "The timer interval of the interrupt stage: 0: Clock period x 2^6 1: Clock period x 2^8 2: Clock period x 2^10 3: Clock period x 2^11 4: Clock period x 2^12 5: Clock period x 2^13 6: Clock period x 2^14 7: Clock period x 2^15 8: Clock period x 2^17 9: Clock period x 2^19 10: Clock period x 2^21 11: Clock period x 2^23 12: Clock period x 2^25 13: Clock period x 2^27 14: Clock period x 2^29 15: Clock period x 2^31."]
        #[inline(always)]
        pub const fn set_inttime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "The time interval of the reset stage: 0: Clock period x 2^7 1: Clock period x 2^8 2: Clock period x 2^9 3: Clock period x 2^10 4: Clock period x 2^11 5: Clock period x 2^12 6: Clock period x 2^13 7: Clock period x 2^14."]
        #[must_use]
        #[inline(always)]
        pub const fn rsttime(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "The time interval of the reset stage: 0: Clock period x 2^7 1: Clock period x 2^8 2: Clock period x 2^9 3: Clock period x 2^10 4: Clock period x 2^11 5: Clock period x 2^12 6: Clock period x 2^13 7: Clock period x 2^14."]
        #[inline(always)]
        pub const fn set_rsttime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl")
                .field("en", &self.en())
                .field("clksel", &self.clksel())
                .field("inten", &self.inten())
                .field("rsten", &self.rsten())
                .field("inttime", &self.inttime())
                .field("rsttime", &self.rsttime())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl {{ en: {=bool:?}, clksel: {=bool:?}, inten: {=bool:?}, rsten: {=bool:?}, inttime: {=u8:?}, rsttime: {=u8:?} }}" , self . en () , self . clksel () , self . inten () , self . rsten () , self . inttime () , self . rsttime ())
        }
    }
    #[doc = "Restart Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Restart(pub u32);
    impl Restart {
        #[doc = "Write the magic number ATCWDT200_RESTART_NUM to restart the watchdog timer."]
        #[must_use]
        #[inline(always)]
        pub const fn restart(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Write the magic number ATCWDT200_RESTART_NUM to restart the watchdog timer."]
        #[inline(always)]
        pub const fn set_restart(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Restart {
        #[inline(always)]
        fn default() -> Restart {
            Restart(0)
        }
    }
    impl core::fmt::Debug for Restart {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Restart")
                .field("restart", &self.restart())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Restart {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Restart {{ restart: {=u16:?} }}", self.restart())
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct St(pub u32);
    impl St {
        #[doc = "The status of the watchdog interrupt timer 0: timer is not expired yet 1: timer is expired."]
        #[must_use]
        #[inline(always)]
        pub const fn intexpired(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "The status of the watchdog interrupt timer 0: timer is not expired yet 1: timer is expired."]
        #[inline(always)]
        pub const fn set_intexpired(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for St {
        #[inline(always)]
        fn default() -> St {
            St(0)
        }
    }
    impl core::fmt::Debug for St {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("St")
                .field("intexpired", &self.intexpired())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for St {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "St {{ intexpired: {=bool:?} }}", self.intexpired())
        }
    }
    #[doc = "Write Protection Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WrEn(pub u32);
    impl WrEn {
        #[doc = "Write the magic code to disable the write protection of the Control Register and the Restart Register."]
        #[must_use]
        #[inline(always)]
        pub const fn wen(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Write the magic code to disable the write protection of the Control Register and the Restart Register."]
        #[inline(always)]
        pub const fn set_wen(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for WrEn {
        #[inline(always)]
        fn default() -> WrEn {
            WrEn(0)
        }
    }
    impl core::fmt::Debug for WrEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WrEn").field("wen", &self.wen()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WrEn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WrEn {{ wen: {=u16:?} }}", self.wen())
        }
    }
}
