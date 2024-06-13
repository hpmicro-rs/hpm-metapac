#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "MONO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mono {
    ptr: *mut u8,
}
unsafe impl Send for Mono {}
unsafe impl Sync for Mono {}
impl Mono {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Low part of monotonic counter."]
    #[inline(always)]
    pub const fn monol(self) -> crate::common::Reg<regs::Monol, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "High part of monotonic counter."]
    #[inline(always)]
    pub const fn monoh(self) -> crate::common::Reg<regs::Monoh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "High part of monotonic counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monoh(pub u32);
    impl Monoh {
        #[doc = "high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow."]
        #[inline(always)]
        pub const fn counter(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow."]
        #[inline(always)]
        pub fn set_counter(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Fuse value for high part of monotonica."]
        #[inline(always)]
        pub const fn epoch(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Fuse value for high part of monotonica."]
        #[inline(always)]
        pub fn set_epoch(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Monoh {
        #[inline(always)]
        fn default() -> Monoh {
            Monoh(0)
        }
    }
    #[doc = "Low part of monotonic counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monol(pub u32);
    impl Monol {
        #[doc = "low part of monotonica counter, write to this counter will cause counter increase by 1."]
        #[inline(always)]
        pub const fn counter(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "low part of monotonica counter, write to this counter will cause counter increase by 1."]
        #[inline(always)]
        pub fn set_counter(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Monol {
        #[inline(always)]
        fn default() -> Monol {
            Monol(0)
        }
    }
}
