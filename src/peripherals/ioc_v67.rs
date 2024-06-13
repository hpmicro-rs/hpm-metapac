#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "IOC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ioc {
    ptr: *mut u8,
}
unsafe impl Send for Ioc {}
unsafe impl Sync for Ioc {}
impl Ioc {
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
    pub const fn pad(self, n: usize) -> Pad {
        assert!(n < 492usize);
        unsafe { Pad::from_ptr(self.ptr.add(0x0usize + n * 8usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pad {
    ptr: *mut u8,
}
unsafe impl Send for Pad {}
unsafe impl Sync for Pad {}
impl Pad {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ALT SELECT."]
    #[inline(always)]
    pub const fn func_ctl(self) -> crate::common::Reg<regs::FuncCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PAD SETTINGS."]
    #[inline(always)]
    pub const fn pad_ctl(self) -> crate::common::Reg<regs::PadCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "ALT SELECT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FuncCtl(pub u32);
    impl FuncCtl {
        #[doc = "alt select 0: ALT0 1: ALT1 … 31:ALT31."]
        #[inline(always)]
        pub const fn alt_select(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "alt select 0: ALT0 1: ALT1 … 31:ALT31."]
        #[inline(always)]
        pub fn set_alt_select(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "select analog pin in pad 0: disable 1: enable."]
        #[inline(always)]
        pub const fn analog(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "select analog pin in pad 0: disable 1: enable."]
        #[inline(always)]
        pub fn set_analog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "force input on 0: disable 1: enable."]
        #[inline(always)]
        pub const fn loop_back(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "force input on 0: disable 1: enable."]
        #[inline(always)]
        pub fn set_loop_back(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for FuncCtl {
        #[inline(always)]
        fn default() -> FuncCtl {
            FuncCtl(0)
        }
    }
    #[doc = "PAD SETTINGS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PadCtl(pub u32);
    impl PadCtl {
        #[doc = "drive strength for high-speed IO 3.3V: 000: 85.61Ohm 001: 61.2 Ohm 010: 42.88Ohm 011: 35.76Ohm 111: 30.67Ohm for high-speed IO 1.8V: 000: 84.07Ohm 001: 60.14Ohm 010: 42.15Ohm 011: 35.19Ohm 111: 30.2 Ohm for general IO: 00: 4mA 01: 8mA 11: 12mA."]
        #[inline(always)]
        pub const fn ds(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "drive strength for high-speed IO 3.3V: 000: 85.61Ohm 001: 61.2 Ohm 010: 42.88Ohm 011: 35.76Ohm 111: 30.67Ohm for high-speed IO 1.8V: 000: 84.07Ohm 001: 60.14Ohm 010: 42.15Ohm 011: 35.19Ohm 111: 30.2 Ohm for general IO: 00: 4mA 01: 8mA 11: 12mA."]
        #[inline(always)]
        pub fn set_ds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "pull enable 0: pull disable 1: pull enable."]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "pull enable 0: pull disable 1: pull enable."]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "pull select 0: pull down 1: pull up."]
        #[inline(always)]
        pub const fn ps(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "pull select 0: pull down 1: pull up."]
        #[inline(always)]
        pub fn set_ps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "schmitt trigger enable, only available in high-speed IO 0: disable 1: enable."]
        #[inline(always)]
        pub const fn smt(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "schmitt trigger enable, only available in high-speed IO 0: disable 1: enable."]
        #[inline(always)]
        pub fn set_smt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "open drain 0: open drain disable 1: open drain enable."]
        #[inline(always)]
        pub const fn od(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "open drain 0: open drain disable 1: open drain enable."]
        #[inline(always)]
        pub fn set_od(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "pin voltage select, only available in high-speed IO 0: 3.3V 1: 1.8V."]
        #[inline(always)]
        pub const fn ms(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "pin voltage select, only available in high-speed IO 0: 3.3V 1: 1.8V."]
        #[inline(always)]
        pub fn set_ms(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for PadCtl {
        #[inline(always)]
        fn default() -> PadCtl {
            PadCtl(0)
        }
    }
}
