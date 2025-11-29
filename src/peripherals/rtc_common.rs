#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RTC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc {
    ptr: *mut u8,
}
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Second counter."]
    #[inline(always)]
    pub const fn second(self) -> crate::common::Reg<regs::Second, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Sub-second counter."]
    #[inline(always)]
    pub const fn subsec(self) -> crate::common::Reg<regs::Subsec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Second counter snap shot."]
    #[inline(always)]
    pub const fn sec_snap(self) -> crate::common::Reg<regs::SecSnap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Sub-second counter snap shot."]
    #[inline(always)]
    pub const fn sub_snap(self) -> crate::common::Reg<regs::SubSnap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RTC alarm0."]
    #[inline(always)]
    pub const fn alarm0(self) -> crate::common::Reg<regs::Alarm0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Alarm0 incremental."]
    #[inline(always)]
    pub const fn alarm0_inc(self) -> crate::common::Reg<regs::Alarm0Inc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RTC alarm1."]
    #[inline(always)]
    pub const fn alarm1(self) -> crate::common::Reg<regs::Alarm1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Alarm1 incremental."]
    #[inline(always)]
    pub const fn alarm1_inc(self) -> crate::common::Reg<regs::Alarm1Inc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RTC alarm flag."]
    #[inline(always)]
    pub const fn alarm_flag(self) -> crate::common::Reg<regs::AlarmFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RTC alarm enable."]
    #[inline(always)]
    pub const fn alarm_en(self) -> crate::common::Reg<regs::AlarmEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
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
    #[doc = "RTC alarm0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alarm0(pub u32);
    impl Alarm0 {
        #[doc = "Alarm time for second counter, on each alarm match, alarm increase ALARM0_INC."]
        #[must_use]
        #[inline(always)]
        pub const fn alarm(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Alarm time for second counter, on each alarm match, alarm increase ALARM0_INC."]
        #[inline(always)]
        pub const fn set_alarm(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Alarm0 {
        #[inline(always)]
        fn default() -> Alarm0 {
            Alarm0(0)
        }
    }
    impl core::fmt::Debug for Alarm0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alarm0")
                .field("alarm", &self.alarm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alarm0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Alarm0 {{ alarm: {=u32:?} }}", self.alarm())
        }
    }
    #[doc = "Alarm0 incremental."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alarm0Inc(pub u32);
    impl Alarm0Inc {
        #[doc = "adder when ARLAM0 happen, helps to create periodical alarm."]
        #[must_use]
        #[inline(always)]
        pub const fn increase(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "adder when ARLAM0 happen, helps to create periodical alarm."]
        #[inline(always)]
        pub const fn set_increase(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Alarm0Inc {
        #[inline(always)]
        fn default() -> Alarm0Inc {
            Alarm0Inc(0)
        }
    }
    impl core::fmt::Debug for Alarm0Inc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alarm0Inc")
                .field("increase", &self.increase())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alarm0Inc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Alarm0Inc {{ increase: {=u32:?} }}", self.increase())
        }
    }
    #[doc = "RTC alarm1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alarm1(pub u32);
    impl Alarm1 {
        #[doc = "Alarm time for second counter, on each alarm match, alarm increase ALARM0_INC."]
        #[must_use]
        #[inline(always)]
        pub const fn alarm(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Alarm time for second counter, on each alarm match, alarm increase ALARM0_INC."]
        #[inline(always)]
        pub const fn set_alarm(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Alarm1 {
        #[inline(always)]
        fn default() -> Alarm1 {
            Alarm1(0)
        }
    }
    impl core::fmt::Debug for Alarm1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alarm1")
                .field("alarm", &self.alarm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alarm1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Alarm1 {{ alarm: {=u32:?} }}", self.alarm())
        }
    }
    #[doc = "Alarm1 incremental."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alarm1Inc(pub u32);
    impl Alarm1Inc {
        #[doc = "adder when ARLAM0 happen, helps to create periodical alarm."]
        #[must_use]
        #[inline(always)]
        pub const fn increase(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "adder when ARLAM0 happen, helps to create periodical alarm."]
        #[inline(always)]
        pub const fn set_increase(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Alarm1Inc {
        #[inline(always)]
        fn default() -> Alarm1Inc {
            Alarm1Inc(0)
        }
    }
    impl core::fmt::Debug for Alarm1Inc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alarm1Inc")
                .field("increase", &self.increase())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alarm1Inc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Alarm1Inc {{ increase: {=u32:?} }}", self.increase())
        }
    }
    #[doc = "RTC alarm enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlarmEn(pub u32);
    impl AlarmEn {
        #[doc = "alarm0 mask 0: alarm0 disabled 1: alarm0 enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn enable0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "alarm0 mask 0: alarm0 disabled 1: alarm0 enabled."]
        #[inline(always)]
        pub const fn set_enable0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "alarm1 mask 0: alarm1 disabled 1: alarm1 enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn enable1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "alarm1 mask 0: alarm1 disabled 1: alarm1 enabled."]
        #[inline(always)]
        pub const fn set_enable1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for AlarmEn {
        #[inline(always)]
        fn default() -> AlarmEn {
            AlarmEn(0)
        }
    }
    impl core::fmt::Debug for AlarmEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AlarmEn")
                .field("enable0", &self.enable0())
                .field("enable1", &self.enable1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlarmEn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AlarmEn {{ enable0: {=bool:?}, enable1: {=bool:?} }}",
                self.enable0(),
                self.enable1()
            )
        }
    }
    #[doc = "RTC alarm flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlarmFlag(pub u32);
    impl AlarmFlag {
        #[doc = "alarm0 happen."]
        #[must_use]
        #[inline(always)]
        pub const fn alarm0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "alarm0 happen."]
        #[inline(always)]
        pub const fn set_alarm0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "alarm1 happen."]
        #[must_use]
        #[inline(always)]
        pub const fn alarm1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "alarm1 happen."]
        #[inline(always)]
        pub const fn set_alarm1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for AlarmFlag {
        #[inline(always)]
        fn default() -> AlarmFlag {
            AlarmFlag(0)
        }
    }
    impl core::fmt::Debug for AlarmFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AlarmFlag")
                .field("alarm0", &self.alarm0())
                .field("alarm1", &self.alarm1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlarmFlag {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AlarmFlag {{ alarm0: {=bool:?}, alarm1: {=bool:?} }}",
                self.alarm0(),
                self.alarm1()
            )
        }
    }
    #[doc = "Second counter snap shot."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SecSnap(pub u32);
    impl SecSnap {
        #[doc = "second snap shot, write to take snap shot."]
        #[must_use]
        #[inline(always)]
        pub const fn sec_snap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "second snap shot, write to take snap shot."]
        #[inline(always)]
        pub const fn set_sec_snap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SecSnap {
        #[inline(always)]
        fn default() -> SecSnap {
            SecSnap(0)
        }
    }
    impl core::fmt::Debug for SecSnap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SecSnap")
                .field("sec_snap", &self.sec_snap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SecSnap {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SecSnap {{ sec_snap: {=u32:?} }}", self.sec_snap())
        }
    }
    #[doc = "Second counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Second(pub u32);
    impl Second {
        #[doc = "second counter."]
        #[must_use]
        #[inline(always)]
        pub const fn second(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "second counter."]
        #[inline(always)]
        pub const fn set_second(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Second {
        #[inline(always)]
        fn default() -> Second {
            Second(0)
        }
    }
    impl core::fmt::Debug for Second {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Second")
                .field("second", &self.second())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Second {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Second {{ second: {=u32:?} }}", self.second())
        }
    }
    #[doc = "Sub-second counter snap shot."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SubSnap(pub u32);
    impl SubSnap {
        #[doc = "sub second snap shot, write to take snap shot."]
        #[must_use]
        #[inline(always)]
        pub const fn sub_snap(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "sub second snap shot, write to take snap shot."]
        #[inline(always)]
        pub const fn set_sub_snap(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SubSnap {
        #[inline(always)]
        fn default() -> SubSnap {
            SubSnap(0)
        }
    }
    impl core::fmt::Debug for SubSnap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SubSnap")
                .field("sub_snap", &self.sub_snap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SubSnap {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SubSnap {{ sub_snap: {=u32:?} }}", self.sub_snap())
        }
    }
    #[doc = "Sub-second counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Subsec(pub u32);
    impl Subsec {
        #[doc = "sub second counter."]
        #[must_use]
        #[inline(always)]
        pub const fn subsec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "sub second counter."]
        #[inline(always)]
        pub const fn set_subsec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Subsec {
        #[inline(always)]
        fn default() -> Subsec {
            Subsec(0)
        }
    }
    impl core::fmt::Debug for Subsec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Subsec")
                .field("subsec", &self.subsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Subsec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Subsec {{ subsec: {=u32:?} }}", self.subsec())
        }
    }
}
