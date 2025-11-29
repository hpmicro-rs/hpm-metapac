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
        assert!(n < 456usize);
        unsafe { Pad::from_ptr(self.ptr.wrapping_add(0x0usize + n * 8usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PAD SETTINGS."]
    #[inline(always)]
    pub const fn pad_ctl(self) -> crate::common::Reg<regs::PadCtl, crate::common::RW> {
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
    #[doc = "ALT SELECT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FuncCtl(pub u32);
    impl FuncCtl {
        #[doc = "alt select 0: ALT0 1: ALT1 ... 31:ALT31."]
        #[must_use]
        #[inline(always)]
        pub const fn alt_select(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "alt select 0: ALT0 1: ALT1 ... 31:ALT31."]
        #[inline(always)]
        pub const fn set_alt_select(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "select analog pin in pad 0: disable 1: enable."]
        #[must_use]
        #[inline(always)]
        pub const fn analog(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "select analog pin in pad 0: disable 1: enable."]
        #[inline(always)]
        pub const fn set_analog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "force input on 0: disable 1: enable."]
        #[must_use]
        #[inline(always)]
        pub const fn loop_back(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "force input on 0: disable 1: enable."]
        #[inline(always)]
        pub const fn set_loop_back(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for FuncCtl {
        #[inline(always)]
        fn default() -> FuncCtl {
            FuncCtl(0)
        }
    }
    impl core::fmt::Debug for FuncCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FuncCtl")
                .field("alt_select", &self.alt_select())
                .field("analog", &self.analog())
                .field("loop_back", &self.loop_back())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FuncCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FuncCtl {{ alt_select: {=u8:?}, analog: {=bool:?}, loop_back: {=bool:?} }}",
                self.alt_select(),
                self.analog(),
                self.loop_back()
            )
        }
    }
    #[doc = "PAD SETTINGS."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PadCtl(pub u32);
    impl PadCtl {
        #[doc = "drive strength 1.8V Mode: 000: 260 Ohm 001: 260 Ohm 010: 130 Ohm 011: 88 Ohm 100: 65 Ohm 101: 52 Ohm 110: 43 Ohm 111: 37 Ohm 3.3V Mode: 000: 157 Ohm 001: 157 Ohm 010: 78 Ohm 011: 53 Ohm 100: 39 Ohm 101: 32 Ohm 110: 26 Ohm 111: 23 Ohm."]
        #[must_use]
        #[inline(always)]
        pub const fn ds(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "drive strength 1.8V Mode: 000: 260 Ohm 001: 260 Ohm 010: 130 Ohm 011: 88 Ohm 100: 65 Ohm 101: 52 Ohm 110: 43 Ohm 111: 37 Ohm 3.3V Mode: 000: 157 Ohm 001: 157 Ohm 010: 78 Ohm 011: 53 Ohm 100: 39 Ohm 101: 32 Ohm 110: 26 Ohm 111: 23 Ohm."]
        #[inline(always)]
        pub const fn set_ds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "additional 2-bit slew rate to select IO cell operation frequency range with reduced switching noise 00: Slow frequency slew rate(50Mhz) 01: Medium frequency slew rate(100 Mhz) 10: Fast frequency slew rate(150 Mhz) 11: Max frequency slew rate(200Mhz)."]
        #[must_use]
        #[inline(always)]
        pub const fn spd(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "additional 2-bit slew rate to select IO cell operation frequency range with reduced switching noise 00: Slow frequency slew rate(50Mhz) 01: Medium frequency slew rate(100 Mhz) 10: Fast frequency slew rate(150 Mhz) 11: Max frequency slew rate(200Mhz)."]
        #[inline(always)]
        pub const fn set_spd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "slew rate 0: Slow slew rate 1: Fast slew rate."]
        #[must_use]
        #[inline(always)]
        pub const fn sr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "slew rate 0: Slow slew rate 1: Fast slew rate."]
        #[inline(always)]
        pub const fn set_sr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "open drain 0: open drain disable 1: open drain enable."]
        #[must_use]
        #[inline(always)]
        pub const fn od(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "open drain 0: open drain disable 1: open drain enable."]
        #[inline(always)]
        pub const fn set_od(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "keeper capability enable 0: keeper disable 1: keeper enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ke(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "keeper capability enable 0: keeper disable 1: keeper enable."]
        #[inline(always)]
        pub const fn set_ke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "pull enable 0: pull disable 1: pull enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "pull enable 0: pull disable 1: pull enable."]
        #[inline(always)]
        pub const fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "pull select 0: pull down 1: pull up."]
        #[must_use]
        #[inline(always)]
        pub const fn ps(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "pull select 0: pull down 1: pull up."]
        #[inline(always)]
        pub const fn set_ps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "select pull up/down internal resistance strength: For pull down, only have 100 Kohm resistance For pull up: 00: 100 KOhm 01: 47 KOhm 10: 22 KOhm 11: 22 KOhm."]
        #[must_use]
        #[inline(always)]
        pub const fn prs(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "select pull up/down internal resistance strength: For pull down, only have 100 Kohm resistance For pull up: 00: 100 KOhm 01: 47 KOhm 10: 22 KOhm 11: 22 KOhm."]
        #[inline(always)]
        pub const fn set_prs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "schmitt trigger enable 0: disable 1: enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hys(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "schmitt trigger enable 0: disable 1: enable."]
        #[inline(always)]
        pub const fn set_hys(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for PadCtl {
        #[inline(always)]
        fn default() -> PadCtl {
            PadCtl(0)
        }
    }
    impl core::fmt::Debug for PadCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PadCtl")
                .field("ds", &self.ds())
                .field("spd", &self.spd())
                .field("sr", &self.sr())
                .field("od", &self.od())
                .field("ke", &self.ke())
                .field("pe", &self.pe())
                .field("ps", &self.ps())
                .field("prs", &self.prs())
                .field("hys", &self.hys())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PadCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PadCtl {{ ds: {=u8:?}, spd: {=u8:?}, sr: {=bool:?}, od: {=bool:?}, ke: {=bool:?}, pe: {=bool:?}, ps: {=bool:?}, prs: {=u8:?}, hys: {=bool:?} }}" , self . ds () , self . spd () , self . sr () , self . od () , self . ke () , self . pe () , self . ps () , self . prs () , self . hys ())
        }
    }
}
