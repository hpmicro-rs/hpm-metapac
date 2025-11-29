#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Assign {
    ptr: *mut u8,
}
unsafe impl Send for Assign {}
unsafe impl Sync for Assign {}
impl Assign {
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
    pub const fn pin(self, n: usize) -> crate::common::Reg<regs::Pin, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
#[doc = "GPIOM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpiom {
    ptr: *mut u8,
}
unsafe impl Send for Gpiom {}
unsafe impl Sync for Gpiom {}
impl Gpiom {
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
    pub const fn assign(self, n: usize) -> Assign {
        assert!(n < 16usize);
        unsafe { Assign::from_ptr(self.ptr.wrapping_add(0x0usize + n * 128usize) as _) }
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
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pin(pub u32);
    impl Pin {
        #[doc = "select which gpio controls chip pin, 0: soc gpio0; 2: cpu0 fastgpio."]
        #[must_use]
        #[inline(always)]
        pub const fn select(&self) -> super::vals::PinSelect {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::PinSelect::from_bits(val as u8)
        }
        #[doc = "select which gpio controls chip pin, 0: soc gpio0; 2: cpu0 fastgpio."]
        #[inline(always)]
        pub const fn set_select(&mut self, val: super::vals::PinSelect) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit1: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio."]
        #[must_use]
        #[inline(always)]
        pub const fn hide(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit1: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio."]
        #[inline(always)]
        pub const fn set_hide(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Pin {
        #[inline(always)]
        fn default() -> Pin {
            Pin(0)
        }
    }
    impl core::fmt::Debug for Pin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pin")
                .field("select", &self.select())
                .field("hide", &self.hide())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pin {{ select: {:?}, hide: {=u8:?}, lock: {=bool:?} }}",
                self.select(),
                self.hide(),
                self.lock()
            )
        }
    }
}
pub mod vals {
    #[doc = "select which gpio controls chip pin"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PinSelect {
        #[doc = "soc gpio0"]
        GPIO0 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "cpu0 fastgpio"]
        CPU0_FGPIO = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl PinSelect {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PinSelect {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PinSelect {
        #[inline(always)]
        fn from(val: u8) -> PinSelect {
            PinSelect::from_bits(val)
        }
    }
    impl From<PinSelect> for u8 {
        #[inline(always)]
        fn from(val: PinSelect) -> u8 {
            PinSelect::to_bits(val)
        }
    }
}
