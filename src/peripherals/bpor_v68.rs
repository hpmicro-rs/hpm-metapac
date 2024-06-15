#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BPOR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bpor {
    ptr: *mut u8,
}
unsafe impl Send for Bpor {}
unsafe impl Sync for Bpor {}
impl Bpor {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Power on reset config."]
    #[inline(always)]
    pub const fn por_config(self) -> crate::common::Reg<regs::PorConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "Power on reset config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PorConfig(pub u32);
    impl PorConfig {
        #[doc = "retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen."]
        #[inline(always)]
        pub const fn retention(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen."]
        #[inline(always)]
        pub fn set_retention(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PorConfig {
        #[inline(always)]
        fn default() -> PorConfig {
            PorConfig(0)
        }
    }
}
