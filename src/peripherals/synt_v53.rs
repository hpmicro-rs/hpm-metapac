#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "SYNT."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Synt {
    ptr: *mut u8,
}
unsafe impl Send for Synt {}
unsafe impl Sync for Synt {}
impl Synt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Global control register."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Counter reload register."]
    #[inline(always)]
    pub const fn rld(self) -> crate::common::Reg<regs::Rld, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "timestamp new value register."]
    #[inline(always)]
    pub const fn timestamp_new(self) -> crate::common::Reg<regs::TimestampNew, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Counter."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "timestamp trig save value."]
    #[inline(always)]
    pub const fn timestamp_sav(self) -> crate::common::Reg<regs::TimestampSav, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "timestamp read value."]
    #[inline(always)]
    pub const fn timestamp_cur(self) -> crate::common::Reg<regs::TimestampCur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmp(self, n: usize) -> crate::common::Reg<regs::Cmp, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
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
    pub struct Cmp(pub u32);
    impl Cmp {
        #[doc = "comparator value, the output will assert when counter count to this value."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "comparator value, the output will assert when counter count to this value."]
        #[inline(always)]
        pub const fn set_cmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cmp {
        #[inline(always)]
        fn default() -> Cmp {
            Cmp(0)
        }
    }
    impl core::fmt::Debug for Cmp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmp").field("cmp", &self.cmp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cmp {{ cmp: {=u32:?} }}", self.cmp())
        }
    }
    #[doc = "Counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "counter."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    impl core::fmt::Debug for Cnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt").field("cnt", &self.cnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cnt {{ cnt: {=u32:?} }}", self.cnt())
        }
    }
    #[doc = "Global control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "1- Enable counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1- Enable counter."]
        #[inline(always)]
        pub const fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "1- Reset counter."]
        #[must_use]
        #[inline(always)]
        pub const fn crst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1- Reset counter."]
        #[inline(always)]
        pub const fn set_crst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "set to enable cpu_debug_mode to stop the counter."]
        #[must_use]
        #[inline(always)]
        pub const fn counter_debug_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable cpu_debug_mode to stop the counter."]
        #[inline(always)]
        pub const fn set_counter_debug_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "set to enable the timesamp , clr to stop."]
        #[must_use]
        #[inline(always)]
        pub const fn timestamp_enable(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable the timesamp , clr to stop."]
        #[inline(always)]
        pub const fn set_timestamp_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "set to enable cpu_debug_mode to stop the timesamp."]
        #[must_use]
        #[inline(always)]
        pub const fn timestamp_debug_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable cpu_debug_mode to stop the timesamp."]
        #[inline(always)]
        pub const fn set_timestamp_debug_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "reset timesamp to 0, auto clr."]
        #[must_use]
        #[inline(always)]
        pub const fn timestamp_reset(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "reset timesamp to 0, auto clr."]
        #[inline(always)]
        pub const fn set_timestamp_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "set the timesamp to new value, auto clr."]
        #[must_use]
        #[inline(always)]
        pub const fn timestamp_set_new(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "set the timesamp to new value, auto clr."]
        #[inline(always)]
        pub const fn set_timestamp_set_new(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "set to decrease the timesamp with new value, auto clr."]
        #[must_use]
        #[inline(always)]
        pub const fn timestamp_dec_new(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "set to decrease the timesamp with new value, auto clr."]
        #[inline(always)]
        pub const fn set_timestamp_dec_new(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "set to increase the timesamp with new value, auto clr."]
        #[must_use]
        #[inline(always)]
        pub const fn timestamp_inc_new(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "set to increase the timesamp with new value, auto clr."]
        #[inline(always)]
        pub const fn set_timestamp_inc_new(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    impl core::fmt::Debug for Gcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gcr")
                .field("cen", &self.cen())
                .field("crst", &self.crst())
                .field("counter_debug_en", &self.counter_debug_en())
                .field("timestamp_enable", &self.timestamp_enable())
                .field("timestamp_debug_en", &self.timestamp_debug_en())
                .field("timestamp_reset", &self.timestamp_reset())
                .field("timestamp_set_new", &self.timestamp_set_new())
                .field("timestamp_dec_new", &self.timestamp_dec_new())
                .field("timestamp_inc_new", &self.timestamp_inc_new())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gcr {{ cen: {=bool:?}, crst: {=bool:?}, counter_debug_en: {=bool:?}, timestamp_enable: {=bool:?}, timestamp_debug_en: {=bool:?}, timestamp_reset: {=bool:?}, timestamp_set_new: {=bool:?}, timestamp_dec_new: {=bool:?}, timestamp_inc_new: {=bool:?} }}" , self . cen () , self . crst () , self . counter_debug_en () , self . timestamp_enable () , self . timestamp_debug_en () , self . timestamp_reset () , self . timestamp_set_new () , self . timestamp_dec_new () , self . timestamp_inc_new ())
        }
    }
    #[doc = "Counter reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rld(pub u32);
    impl Rld {
        #[doc = "counter reload value."]
        #[must_use]
        #[inline(always)]
        pub const fn rld(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "counter reload value."]
        #[inline(always)]
        pub const fn set_rld(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rld {
        #[inline(always)]
        fn default() -> Rld {
            Rld(0)
        }
    }
    impl core::fmt::Debug for Rld {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rld").field("rld", &self.rld()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rld {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rld {{ rld: {=u32:?} }}", self.rld())
        }
    }
    #[doc = "timestamp read value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TimestampCur(pub u32);
    impl TimestampCur {
        #[doc = "current timesamp value."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "current timesamp value."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TimestampCur {
        #[inline(always)]
        fn default() -> TimestampCur {
            TimestampCur(0)
        }
    }
    impl core::fmt::Debug for TimestampCur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TimestampCur")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TimestampCur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TimestampCur {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "timestamp new value register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TimestampNew(pub u32);
    impl TimestampNew {
        #[doc = "new value for timesamp , can be used as set/inc/dec."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "new value for timesamp , can be used as set/inc/dec."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TimestampNew {
        #[inline(always)]
        fn default() -> TimestampNew {
            TimestampNew(0)
        }
    }
    impl core::fmt::Debug for TimestampNew {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TimestampNew")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TimestampNew {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TimestampNew {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "timestamp trig save value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TimestampSav(pub u32);
    impl TimestampSav {
        #[doc = "use the trigger to save timesamp here."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "use the trigger to save timesamp here."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TimestampSav {
        #[inline(always)]
        fn default() -> TimestampSav {
            TimestampSav(0)
        }
    }
    impl core::fmt::Debug for TimestampSav {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TimestampSav")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TimestampSav {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TimestampSav {{ value: {=u32:?} }}", self.value())
        }
    }
}
