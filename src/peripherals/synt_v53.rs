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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Counter reload register."]
    #[inline(always)]
    pub const fn rld(self) -> crate::common::Reg<regs::Rld, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "timestamp new value register."]
    #[inline(always)]
    pub const fn timestamp_new(self) -> crate::common::Reg<regs::TimestampNew, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Counter."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "timestamp trig save value."]
    #[inline(always)]
    pub const fn timestamp_sav(self) -> crate::common::Reg<regs::TimestampSav, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "timestamp read value."]
    #[inline(always)]
    pub const fn timestamp_cur(self) -> crate::common::Reg<regs::TimestampCur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmp(self, n: usize) -> crate::common::Reg<regs::Cmp, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmp(pub u32);
    impl Cmp {
        #[doc = "comparator value, the output will assert when counter count to this value."]
        #[inline(always)]
        pub const fn cmp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "comparator value, the output will assert when counter count to this value."]
        #[inline(always)]
        pub fn set_cmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cmp {
        #[inline(always)]
        fn default() -> Cmp {
            Cmp(0)
        }
    }
    #[doc = "Counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "counter."]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "counter."]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    #[doc = "Global control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "1- Enable counter."]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1- Enable counter."]
        #[inline(always)]
        pub fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "1- Reset counter."]
        #[inline(always)]
        pub const fn crst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1- Reset counter."]
        #[inline(always)]
        pub fn set_crst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "set to enable cpu_debug_mode to stop the counter."]
        #[inline(always)]
        pub const fn counter_debug_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable cpu_debug_mode to stop the counter."]
        #[inline(always)]
        pub fn set_counter_debug_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "set to enable the timesamp , clr to stop."]
        #[inline(always)]
        pub const fn timestamp_enable(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable the timesamp , clr to stop."]
        #[inline(always)]
        pub fn set_timestamp_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "set to enable cpu_debug_mode to stop the timesamp."]
        #[inline(always)]
        pub const fn timestamp_debug_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable cpu_debug_mode to stop the timesamp."]
        #[inline(always)]
        pub fn set_timestamp_debug_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "reset timesamp to 0, auto clr."]
        #[inline(always)]
        pub const fn timestamp_reset(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "reset timesamp to 0, auto clr."]
        #[inline(always)]
        pub fn set_timestamp_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "set the timesamp to new value, auto clr."]
        #[inline(always)]
        pub const fn timestamp_set_new(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "set the timesamp to new value, auto clr."]
        #[inline(always)]
        pub fn set_timestamp_set_new(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "set to decrease the timesamp with new value, auto clr."]
        #[inline(always)]
        pub const fn timestamp_dec_new(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "set to decrease the timesamp with new value, auto clr."]
        #[inline(always)]
        pub fn set_timestamp_dec_new(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "set to increase the timesamp with new value, auto clr."]
        #[inline(always)]
        pub const fn timestamp_inc_new(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "set to increase the timesamp with new value, auto clr."]
        #[inline(always)]
        pub fn set_timestamp_inc_new(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    #[doc = "Counter reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rld(pub u32);
    impl Rld {
        #[doc = "counter reload value."]
        #[inline(always)]
        pub const fn rld(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "counter reload value."]
        #[inline(always)]
        pub fn set_rld(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rld {
        #[inline(always)]
        fn default() -> Rld {
            Rld(0)
        }
    }
    #[doc = "timestamp read value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TimestampCur(pub u32);
    impl TimestampCur {
        #[doc = "current timesamp value."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "current timesamp value."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TimestampCur {
        #[inline(always)]
        fn default() -> TimestampCur {
            TimestampCur(0)
        }
    }
    #[doc = "timestamp new value register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TimestampNew(pub u32);
    impl TimestampNew {
        #[doc = "new value for timesamp , can be used as set/inc/dec."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "new value for timesamp , can be used as set/inc/dec."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TimestampNew {
        #[inline(always)]
        fn default() -> TimestampNew {
            TimestampNew(0)
        }
    }
    #[doc = "timestamp trig save value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TimestampSav(pub u32);
    impl TimestampSav {
        #[doc = "use the trigger to save timesamp here."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "use the trigger to save timesamp here."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TimestampSav {
        #[inline(always)]
        fn default() -> TimestampSav {
            TimestampSav(0)
        }
    }
}
