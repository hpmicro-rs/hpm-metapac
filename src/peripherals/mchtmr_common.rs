#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "MCHTMR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mchtmr {
    ptr: *mut u8,
}
unsafe impl Send for Mchtmr {}
unsafe impl Sync for Mchtmr {}
impl Mchtmr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Machine Time."]
    #[inline(always)]
    pub const fn mtime(self) -> crate::common::Reg<regs::Mtime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Machine Time Compare."]
    #[inline(always)]
    pub const fn mtimecmp(self) -> crate::common::Reg<regs::Mtimecmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "Machine Time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtime(pub u64);
    impl Mtime {
        #[doc = "Machine time."]
        #[inline(always)]
        pub const fn mtime(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "Machine time."]
        #[inline(always)]
        pub fn set_mtime(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for Mtime {
        #[inline(always)]
        fn default() -> Mtime {
            Mtime(0)
        }
    }
    #[doc = "Machine Time Compare."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mtimecmp(pub u64);
    impl Mtimecmp {
        #[doc = "Machine time compare."]
        #[inline(always)]
        pub const fn mtimecmp(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "Machine time compare."]
        #[inline(always)]
        pub fn set_mtimecmp(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for Mtimecmp {
        #[inline(always)]
        fn default() -> Mtimecmp {
            Mtimecmp(0)
        }
    }
}
