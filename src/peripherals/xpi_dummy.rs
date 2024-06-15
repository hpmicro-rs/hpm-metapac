#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Placeholder for XPI device."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xpi {
    ptr: *mut u8,
}
unsafe impl Send for Xpi {}
unsafe impl Sync for Xpi {}
impl Xpi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
}
