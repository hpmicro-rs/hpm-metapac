#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BMON."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmon {
    ptr: *mut u8,
}
unsafe impl Send for Bmon {}
unsafe impl Sync for Bmon {}
impl Bmon {
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
    pub const fn monitor(self, n: usize) -> Monitor {
        assert!(n < 2usize);
        unsafe { Monitor::from_ptr(self.ptr.add(0x0usize + n * 16usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monitor {
    ptr: *mut u8,
}
unsafe impl Send for Monitor {}
unsafe impl Sync for Monitor {}
impl Monitor {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Glitch and clock monitor control."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Glitch and clock monitor status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "Glitch and clock monitor control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control(pub u32);
    impl Control {
        #[doc = "enable glitch detector 0: detector disabled 1: detector enabled."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable glitch detector 0: detector disabled 1: detector enabled."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destroy DFF value 1: active mode, check glitch by DFF chain."]
        #[inline(always)]
        pub const fn active(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destroy DFF value 1: active mode, check glitch by DFF chain."]
        #[inline(always)]
        pub fn set_active(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Control {
        #[inline(always)]
        fn default() -> Control {
            Control(0)
        }
    }
    #[doc = "Glitch and clock monitor status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected."]
        #[inline(always)]
        pub const fn flag(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected."]
        #[inline(always)]
        pub fn set_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
}
