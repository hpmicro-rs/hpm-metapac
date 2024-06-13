#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PLICSW."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plicsw {
    ptr: *mut u8,
}
unsafe impl Send for Plicsw {}
unsafe impl Sync for Plicsw {}
impl Plicsw {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Pending status."]
    #[inline(always)]
    pub const fn pending(self) -> crate::common::Reg<regs::Pending, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1000usize) as _) }
    }
    #[doc = "Interrupt enable."]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2000usize) as _) }
    }
    #[doc = "Claim and complete."]
    #[inline(always)]
    pub const fn claim(self) -> crate::common::Reg<regs::Claim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0020_0004usize) as _) }
    }
}
pub mod regs {
    #[doc = "Claim and complete."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Claim(pub u32);
    impl Claim {
        #[doc = "On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
        #[inline(always)]
        pub const fn interrupt_id(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
        #[inline(always)]
        pub fn set_interrupt_id(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Claim {
        #[inline(always)]
        fn default() -> Claim {
            Claim(0)
        }
    }
    #[doc = "Interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Inten(pub u32);
    impl Inten {
        #[doc = "enable software interrupt."]
        #[inline(always)]
        pub const fn interrupt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable software interrupt."]
        #[inline(always)]
        pub fn set_interrupt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Inten {
        #[inline(always)]
        fn default() -> Inten {
            Inten(0)
        }
    }
    #[doc = "Pending status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pending(pub u32);
    impl Pending {
        #[doc = "writing 1 to trigger software interrupt."]
        #[inline(always)]
        pub const fn interrupt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "writing 1 to trigger software interrupt."]
        #[inline(always)]
        pub fn set_interrupt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Pending {
        #[inline(always)]
        fn default() -> Pending {
            Pending(0)
        }
    }
}
