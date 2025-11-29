#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PPOR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppor {
    ptr: *mut u8,
}
unsafe impl Send for Ppor {}
unsafe impl Sync for Ppor {}
impl Ppor {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "flag indicate reset source."]
    #[inline(always)]
    pub const fn reset_flag(self) -> crate::common::Reg<regs::ResetFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "reset source status."]
    #[inline(always)]
    pub const fn reset_status(self) -> crate::common::Reg<regs::ResetStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "reset hold attribute."]
    #[inline(always)]
    pub const fn reset_hold(self) -> crate::common::Reg<regs::ResetHold, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "reset source enable."]
    #[inline(always)]
    pub const fn reset_enable(self) -> crate::common::Reg<regs::ResetEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "reset type triggered by reset."]
    #[inline(always)]
    pub const fn reset_type(self) -> crate::common::Reg<regs::ResetType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Software reset counter."]
    #[inline(always)]
    pub const fn software_reset(
        self,
    ) -> crate::common::Reg<regs::SoftwareReset, crate::common::RW> {
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
    #[doc = "reset source enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetEnable(pub u32);
    impl ResetEnable {
        #[doc = "enable of reset sources 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "enable of reset sources 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ResetEnable {
        #[inline(always)]
        fn default() -> ResetEnable {
            ResetEnable(0)
        }
    }
    impl core::fmt::Debug for ResetEnable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ResetEnable")
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ResetEnable {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ResetEnable {{ enable: {=u32:?} }}", self.enable())
        }
    }
    #[doc = "flag indicate reset source."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetFlag(pub u32);
    impl ResetFlag {
        #[doc = "reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[must_use]
        #[inline(always)]
        pub const fn flag(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[inline(always)]
        pub const fn set_flag(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ResetFlag {
        #[inline(always)]
        fn default() -> ResetFlag {
            ResetFlag(0)
        }
    }
    impl core::fmt::Debug for ResetFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ResetFlag")
                .field("flag", &self.flag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ResetFlag {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ResetFlag {{ flag: {=u32:?} }}", self.flag())
        }
    }
    #[doc = "reset hold attribute."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetHold(pub u32);
    impl ResetHold {
        #[doc = "hold arrtibute, when set, SOC keep in reset status until reset source release, or, reset will be released after SOC enter reset status 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[must_use]
        #[inline(always)]
        pub const fn hold(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "hold arrtibute, when set, SOC keep in reset status until reset source release, or, reset will be released after SOC enter reset status 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[inline(always)]
        pub const fn set_hold(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ResetHold {
        #[inline(always)]
        fn default() -> ResetHold {
            ResetHold(0)
        }
    }
    impl core::fmt::Debug for ResetHold {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ResetHold")
                .field("hold", &self.hold())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ResetHold {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ResetHold {{ hold: {=u32:?} }}", self.hold())
        }
    }
    #[doc = "reset source status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetStatus(pub u32);
    impl ResetStatus {
        #[doc = "current status of reset sources 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[must_use]
        #[inline(always)]
        pub const fn status(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "current status of reset sources 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[inline(always)]
        pub const fn set_status(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ResetStatus {
        #[inline(always)]
        fn default() -> ResetStatus {
            ResetStatus(0)
        }
    }
    impl core::fmt::Debug for ResetStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ResetStatus")
                .field("status", &self.status())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ResetStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ResetStatus {{ status: {=u32:?} }}", self.status())
        }
    }
    #[doc = "reset type triggered by reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetType(pub u32);
    impl ResetType {
        #[doc = "reset type of reset sources, 0 for cold reset, all system control setting cleared except debug/fuse/ioc; 1 for hot reset, keep system control setting and debug/fuse/ioc setting, only clear some subsystem 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "reset type of reset sources, 0 for cold reset, all system control setting cleared except debug/fuse/ioc; 1 for hot reset, keep system control setting and debug/fuse/ioc setting, only clear some subsystem 0: brownout 1: temperature 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ResetType {
        #[inline(always)]
        fn default() -> ResetType {
            ResetType(0)
        }
    }
    impl core::fmt::Debug for ResetType {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ResetType")
                .field("type_", &self.type_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ResetType {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ResetType {{ type_: {=u32:?} }}", self.type_())
        }
    }
    #[doc = "Software reset counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SoftwareReset(pub u32);
    impl SoftwareReset {
        #[doc = "counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset."]
        #[must_use]
        #[inline(always)]
        pub const fn counter(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset."]
        #[inline(always)]
        pub const fn set_counter(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SoftwareReset {
        #[inline(always)]
        fn default() -> SoftwareReset {
            SoftwareReset(0)
        }
    }
    impl core::fmt::Debug for SoftwareReset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SoftwareReset")
                .field("counter", &self.counter())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SoftwareReset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SoftwareReset {{ counter: {=u32:?} }}", self.counter())
        }
    }
}
