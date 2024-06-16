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
    pub const fn mtime(self) -> crate::common::Reg<u64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Machine Time Compare."]
    #[inline(always)]
    pub const fn mtimecmp(self) -> crate::common::Reg<u64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
