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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "High part of monotonic counter."]
    #[inline(always)]
    pub const fn monoh(self) -> crate::common::Reg<regs::Monoh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
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
    #[doc = "High part of monotonic counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monoh(pub u32);
    impl Monoh {
        #[doc = "high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn counter(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow."]
        #[inline(always)]
        pub const fn set_counter(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Fuse value for high part of monotonica."]
        #[must_use]
        #[inline(always)]
        pub const fn epoch(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Fuse value for high part of monotonica."]
        #[inline(always)]
        pub const fn set_epoch(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Monoh {
        #[inline(always)]
        fn default() -> Monoh {
            Monoh(0)
        }
    }
    impl core::fmt::Debug for Monoh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monoh")
                .field("counter", &self.counter())
                .field("epoch", &self.epoch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Monoh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Monoh {{ counter: {=u16:?}, epoch: {=u16:?} }}",
                self.counter(),
                self.epoch()
            )
        }
    }
    #[doc = "Low part of monotonic counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Monol(pub u32);
    impl Monol {
        #[doc = "low part of monotonica counter, write to this counter will cause counter increase by 1."]
        #[must_use]
        #[inline(always)]
        pub const fn counter(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "low part of monotonica counter, write to this counter will cause counter increase by 1."]
        #[inline(always)]
        pub const fn set_counter(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Monol {
        #[inline(always)]
        fn default() -> Monol {
            Monol(0)
        }
    }
    impl core::fmt::Debug for Monol {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Monol")
                .field("counter", &self.counter())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Monol {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Monol {{ counter: {=u32:?} }}", self.counter())
        }
    }
}
