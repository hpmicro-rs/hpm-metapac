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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
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
        assert!(n < 15usize);
        unsafe { Assign::from_ptr(self.ptr.add(0x0usize + n * 128usize) as _) }
    }
}
pub mod regs {
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pin(pub u32);
    impl Pin {
        #[doc = "select which gpio controls chip pin, 0: soc gpio0; 2: cpu0 fastgpio."]
        #[inline(always)]
        pub const fn select(&self) -> super::vals::PinSelect {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::PinSelect::from_bits(val as u8)
        }
        #[doc = "select which gpio controls chip pin, 0: soc gpio0; 2: cpu0 fastgpio."]
        #[inline(always)]
        pub fn set_select(&mut self, val: super::vals::PinSelect) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio."]
        #[inline(always)]
        pub const fn hide(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "pin value visibility to gpios, bit0: 1, invisible to soc gpio0; 0: visible to soc gpio0 bit2: 1, invisible to cpu0 fast gpio; 0: visible to cpu0 fast gpio."]
        #[inline(always)]
        pub fn set_hide(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "pin value visibility to soc gpio0, 0: visible; 1: invisible."]
        #[inline(always)]
        pub const fn hide_gpio0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "pin value visibility to soc gpio0, 0: visible; 1: invisible."]
        #[inline(always)]
        pub fn set_hide_gpio0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "pin value visibility to cpu0 fast gpio, 0: visible; 1: invisible."]
        #[inline(always)]
        pub const fn hide_cpu0_fgpio(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "pin value visibility to cpu0 fast gpio, 0: visible; 1: invisible."]
        #[inline(always)]
        pub fn set_hide_cpu0_fgpio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "lock fields in this register, lock can only be cleared by soc reset 0: fields can be changed 1: fields locked to current value, not changeable."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Pin {
        #[inline(always)]
        fn default() -> Pin {
            Pin(0)
        }
    }
}
pub mod vals {
    #[doc = "select which gpio controls chip pin"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
