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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "reset source status."]
    #[inline(always)]
    pub const fn reset_status(self) -> crate::common::Reg<regs::ResetStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "reset hold attribute."]
    #[inline(always)]
    pub const fn reset_hold(self) -> crate::common::Reg<regs::ResetHold, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "reset source enable."]
    #[inline(always)]
    pub const fn reset_enable(self) -> crate::common::Reg<regs::ResetEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Software reset counter."]
    #[inline(always)]
    pub const fn software_reset(
        self,
    ) -> crate::common::Reg<regs::SoftwareReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "reset source enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetEnable(pub u32);
    impl ResetEnable {
        #[doc = "enable of reset sources 0: brownout 1: temperature(not available) 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[inline(always)]
        pub const fn enable(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "enable of reset sources 0: brownout 1: temperature(not available) 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ResetEnable {
        #[inline(always)]
        fn default() -> ResetEnable {
            ResetEnable(0)
        }
    }
    #[doc = "flag indicate reset source."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetFlag(pub u32);
    impl ResetFlag {
        #[doc = "reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature(not available) 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[inline(always)]
        pub const fn flag(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature(not available) 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[inline(always)]
        pub fn set_flag(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ResetFlag {
        #[inline(always)]
        fn default() -> ResetFlag {
            ResetFlag(0)
        }
    }
    #[doc = "reset hold attribute."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetHold(pub u32);
    impl ResetHold {
        #[doc = "hold arrtibute, when set, SOC keep in reset status until reset source release, or, reset will be released after SOC enter reset status 0: brownout 1: temperature(not available) 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[inline(always)]
        pub const fn hold(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "hold arrtibute, when set, SOC keep in reset status until reset source release, or, reset will be released after SOC enter reset status 0: brownout 1: temperature(not available) 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[inline(always)]
        pub fn set_hold(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ResetHold {
        #[inline(always)]
        fn default() -> ResetHold {
            ResetHold(0)
        }
    }
    #[doc = "reset source status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetStatus(pub u32);
    impl ResetStatus {
        #[doc = "current status of reset sources 0: brownout 1: temperature(not available) 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[inline(always)]
        pub const fn status(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "current status of reset sources 0: brownout 1: temperature(not available) 4: debug reset 5: jtag soft reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2(not available) 19: watch dog 3(not available) 24: pmic watch dog 30: jtag ieee reset 31: software."]
        #[inline(always)]
        pub fn set_status(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ResetStatus {
        #[inline(always)]
        fn default() -> ResetStatus {
            ResetStatus(0)
        }
    }
    #[doc = "Software reset counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SoftwareReset(pub u32);
    impl SoftwareReset {
        #[doc = "counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset."]
        #[inline(always)]
        pub const fn counter(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset."]
        #[inline(always)]
        pub fn set_counter(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SoftwareReset {
        #[inline(always)]
        fn default() -> SoftwareReset {
            SoftwareReset(0)
        }
    }
}
