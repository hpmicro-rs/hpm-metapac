#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Affiliate {
    ptr: *mut u8,
}
unsafe impl Send for Affiliate {}
unsafe impl Sync for Affiliate {}
impl Affiliate {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Affiliate of Group."]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<regs::AffiliateValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Affiliate of Group."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::AffiliateSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Affiliate of Group."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::AffiliateClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Affiliate of Group."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::AffiliateToggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu {
    ptr: *mut u8,
}
unsafe impl Send for Cpu {}
unsafe impl Sync for Cpu {}
impl Cpu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CPU0 LP control."]
    #[inline(always)]
    pub const fn lp(self) -> crate::common::Reg<regs::Lp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CPU0 Lock GPR."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn gpr(self, n: usize) -> crate::common::Reg<regs::Gpr, crate::common::RW> {
        assert!(n < 14usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wakeup_status(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WakeupStatus, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wakeup_enable(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WakeupEnable, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Group0 {
    ptr: *mut u8,
}
unsafe impl Send for Group0 {}
unsafe impl Sync for Group0 {}
impl Group0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Group setting."]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<regs::Group0Value, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Group setting."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::Group0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Group setting."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::Group0Clear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Group setting."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::Group0Toggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monitor {
    ptr: *mut u8,
}
unsafe impl Send for Monitor {}
unsafe impl Sync for Monitor {}
impl Monitor {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Clock measure and monitor control."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::MonitorControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Clock measure result."]
    #[inline(always)]
    pub const fn current(self) -> crate::common::Reg<regs::Current, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Clock lower limit."]
    #[inline(always)]
    pub const fn low_limit(self) -> crate::common::Reg<regs::LowLimit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Clock upper limit."]
    #[inline(always)]
    pub const fn high_limit(self) -> crate::common::Reg<regs::HighLimit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Power {
    ptr: *mut u8,
}
unsafe impl Send for Power {}
unsafe impl Sync for Power {}
impl Power {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Power Setting."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Power Setting."]
    #[inline(always)]
    pub const fn lf_wait(self) -> crate::common::Reg<regs::LfWait, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Power Setting."]
    #[inline(always)]
    pub const fn off_wait(self) -> crate::common::Reg<regs::OffWait, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reset {
    ptr: *mut u8,
}
unsafe impl Send for Reset {}
unsafe impl Sync for Reset {}
impl Reset {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Reset Setting."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::ResetControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Reset Setting."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Reset Setting."]
    #[inline(always)]
    pub const fn counter(self) -> crate::common::Reg<regs::Counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Retention {
    ptr: *mut u8,
}
unsafe impl Send for Retention {}
unsafe impl Sync for Retention {}
impl Retention {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Retention Control."]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<regs::Retention, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Retention Control."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::Retention, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Retention Control."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::Retention, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Retention Control."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::Retention, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "SYSCTL."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysctl {
    ptr: *mut u8,
}
unsafe impl Send for Sysctl {}
unsafe impl Sync for Sysctl {}
impl Sysctl {
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
    pub const fn resource(self, n: usize) -> crate::common::Reg<regs::Resource, crate::common::RW> {
        assert!(n < 318usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn group0(self, n: usize) -> Group0 {
        assert!(n < 2usize);
        unsafe { Group0::from_ptr(self.ptr.wrapping_add(0x0800usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn affiliate(self, n: usize) -> Affiliate {
        assert!(n < 1usize);
        unsafe { Affiliate::from_ptr(self.ptr.wrapping_add(0x0900usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn retention(self, n: usize) -> Retention {
        assert!(n < 1usize);
        unsafe { Retention::from_ptr(self.ptr.wrapping_add(0x0920usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn power(self, n: usize) -> Power {
        assert!(n < 1usize);
        unsafe { Power::from_ptr(self.ptr.wrapping_add(0x1000usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn reset(self, n: usize) -> Reset {
        assert!(n < 2usize);
        unsafe { Reset::from_ptr(self.ptr.wrapping_add(0x1400usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn clock_cpu(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ClockCpu, crate::common::RW> {
        assert!(n < 1usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1800usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn clock(self, n: usize) -> crate::common::Reg<regs::Clock, crate::common::RW> {
        assert!(n < 39usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1804usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn adcclk(self, n: usize) -> crate::common::Reg<regs::Adcclk, crate::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1c00usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn dacclk(self, n: usize) -> crate::common::Reg<regs::Dacclk, crate::common::RW> {
        assert!(n < 1usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1c0cusize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn i2sclk(self, n: usize) -> crate::common::Reg<regs::I2sclk, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1c10usize + n * 4usize) as _)
        }
    }
    #[doc = "Clock senario."]
    #[inline(always)]
    pub const fn global00(self) -> crate::common::Reg<regs::Global00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn monitor(self, n: usize) -> Monitor {
        assert!(n < 4usize);
        unsafe { Monitor::from_ptr(self.ptr.wrapping_add(0x2400usize + n * 32usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cpu(self, n: usize) -> Cpu {
        assert!(n < 1usize);
        unsafe { Cpu::from_ptr(self.ptr.wrapping_add(0x2800usize + n * 1024usize) as _) }
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
    pub struct Adcclk(pub u32);
    impl Adcclk {
        #[doc = "current mux 0: ana clock 1: ahb clock."]
        #[must_use]
        #[inline(always)]
        pub const fn mux(&self) -> super::vals::AnaClkMux {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::AnaClkMux::from_bits(val as u8)
        }
        #[doc = "current mux 0: ana clock 1: ahb clock."]
        #[inline(always)]
        pub const fn set_mux(&mut self, val: super::vals::AnaClkMux) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[must_use]
        #[inline(always)]
        pub const fn preserve(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[inline(always)]
        pub const fn set_preserve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn loc_busy(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub const fn set_loc_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn glb_busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[inline(always)]
        pub const fn set_glb_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Adcclk {
        #[inline(always)]
        fn default() -> Adcclk {
            Adcclk(0)
        }
    }
    impl core::fmt::Debug for Adcclk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adcclk")
                .field("mux", &self.mux())
                .field("preserve", &self.preserve())
                .field("loc_busy", &self.loc_busy())
                .field("glb_busy", &self.glb_busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adcclk {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Adcclk {{ mux: {:?}, preserve: {=bool:?}, loc_busy: {=bool:?}, glb_busy: {=bool:?} }}" , self . mux () , self . preserve () , self . loc_busy () , self . glb_busy ())
        }
    }
    #[doc = "Affiliate of Group."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AffiliateClear(pub u32);
    impl AffiliateClear {
        #[doc = "Affiliate groups of cpu0, each bit represents a group 0: no effect 1: the group is not assigned to CPU0."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Affiliate groups of cpu0, each bit represents a group 0: no effect 1: the group is not assigned to CPU0."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for AffiliateClear {
        #[inline(always)]
        fn default() -> AffiliateClear {
            AffiliateClear(0)
        }
    }
    impl core::fmt::Debug for AffiliateClear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AffiliateClear")
                .field("link", &self.link())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AffiliateClear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AffiliateClear {{ link: {=u8:?} }}", self.link())
        }
    }
    #[doc = "Affiliate of Group."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AffiliateSet(pub u32);
    impl AffiliateSet {
        #[doc = "Affiliate groups of cpu0，each bit represents a group 0: no effect 1: the group is assigned to CPU0."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Affiliate groups of cpu0，each bit represents a group 0: no effect 1: the group is assigned to CPU0."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for AffiliateSet {
        #[inline(always)]
        fn default() -> AffiliateSet {
            AffiliateSet(0)
        }
    }
    impl core::fmt::Debug for AffiliateSet {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AffiliateSet")
                .field("link", &self.link())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AffiliateSet {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AffiliateSet {{ link: {=u8:?} }}", self.link())
        }
    }
    #[doc = "Affiliate of Group."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AffiliateToggle(pub u32);
    impl AffiliateToggle {
        #[doc = "Affiliate groups of cpu0, each bit represents a group 0: no effect 1: toggle the result that whether the group is assigned to CPU0 before."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Affiliate groups of cpu0, each bit represents a group 0: no effect 1: toggle the result that whether the group is assigned to CPU0 before."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for AffiliateToggle {
        #[inline(always)]
        fn default() -> AffiliateToggle {
            AffiliateToggle(0)
        }
    }
    impl core::fmt::Debug for AffiliateToggle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AffiliateToggle")
                .field("link", &self.link())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AffiliateToggle {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AffiliateToggle {{ link: {=u8:?} }}", self.link())
        }
    }
    #[doc = "Affiliate of Group."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AffiliateValue(pub u32);
    impl AffiliateValue {
        #[doc = "Affiliate groups of cpu0, each bit represents a group bit0: cpu0 depends on group0 bit1: cpu0 depends on group1 bit2: cpu0 depends on group2 bit3: cpu0 depends on group3."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Affiliate groups of cpu0, each bit represents a group bit0: cpu0 depends on group0 bit1: cpu0 depends on group1 bit2: cpu0 depends on group2 bit3: cpu0 depends on group3."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for AffiliateValue {
        #[inline(always)]
        fn default() -> AffiliateValue {
            AffiliateValue(0)
        }
    }
    impl core::fmt::Debug for AffiliateValue {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AffiliateValue")
                .field("link", &self.link())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AffiliateValue {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AffiliateValue {{ link: {=u8:?} }}", self.link())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clock(pub u32);
    impl Clock {
        #[doc = "clock divider 0: divider by 1 1: divider by 2 2: divider by 3 . . . 255: divider by 256."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "clock divider 0: divider by 1 1: divider by 2 2: divider by 3 . . . 255: divider by 256."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "current mux in clock component 0:osc0_clk0 1:pll0_clk0 2:pll0_clk1 3:pll0_clk2 4:pll1_clk0 5:pll1_clk1 6:pll2_clk0 7:pll2_clk1."]
        #[must_use]
        #[inline(always)]
        pub const fn mux(&self) -> super::vals::ClockMux {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::ClockMux::from_bits(val as u8)
        }
        #[doc = "current mux in clock component 0:osc0_clk0 1:pll0_clk0 2:pll0_clk1 3:pll0_clk2 4:pll1_clk0 5:pll1_clk1 6:pll2_clk0 7:pll2_clk1."]
        #[inline(always)]
        pub const fn set_mux(&mut self, val: super::vals::ClockMux) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[must_use]
        #[inline(always)]
        pub const fn preserve(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[inline(always)]
        pub const fn set_preserve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn loc_busy(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub const fn set_loc_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn glb_busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[inline(always)]
        pub const fn set_glb_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Clock {
        #[inline(always)]
        fn default() -> Clock {
            Clock(0)
        }
    }
    impl core::fmt::Debug for Clock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clock")
                .field("div", &self.div())
                .field("mux", &self.mux())
                .field("preserve", &self.preserve())
                .field("loc_busy", &self.loc_busy())
                .field("glb_busy", &self.glb_busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clock {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Clock {{ div: {=u8:?}, mux: {:?}, preserve: {=bool:?}, loc_busy: {=bool:?}, glb_busy: {=bool:?} }}" , self . div () , self . mux () , self . preserve () , self . loc_busy () , self . glb_busy ())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClockCpu(pub u32);
    impl ClockCpu {
        #[doc = "clock divider 0: divider by 1 1: divider by 2 2: divider by 3 . . . 255: divider by 256."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "clock divider 0: divider by 1 1: divider by 2 2: divider by 3 . . . 255: divider by 256."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "current mux in clock component 0:osc0_clk0 1:pll0_clk0 2:pll0_clk1 3:pll0_clk2 4:pll1_clk0 5:pll1_clk1 6:pll2_clk0 7:pll2_clk1."]
        #[must_use]
        #[inline(always)]
        pub const fn mux(&self) -> super::vals::ClockMux {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::ClockMux::from_bits(val as u8)
        }
        #[doc = "current mux in clock component 0:osc0_clk0 1:pll0_clk0 2:pll0_clk1 3:pll0_clk2 4:pll1_clk0 5:pll1_clk1 6:pll2_clk0 7:pll2_clk1."]
        #[inline(always)]
        pub const fn set_mux(&mut self, val: super::vals::ClockMux) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "axi bus divider, the bus clock is generated by cpu_clock/div 0: divider by 1 1: divider by 2 …."]
        #[must_use]
        #[inline(always)]
        pub const fn sub0_div(&self) -> super::vals::SubDiv {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::SubDiv::from_bits(val as u8)
        }
        #[doc = "axi bus divider, the bus clock is generated by cpu_clock/div 0: divider by 1 1: divider by 2 …."]
        #[inline(always)]
        pub const fn set_sub0_div(&mut self, val: super::vals::SubDiv) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "ahb bus divider, the bus clock is generated by cpu_clock/div 0: divider by 1 1: divider by 2 …."]
        #[must_use]
        #[inline(always)]
        pub const fn sub1_div(&self) -> super::vals::SubDiv {
            let val = (self.0 >> 20usize) & 0x0f;
            super::vals::SubDiv::from_bits(val as u8)
        }
        #[doc = "ahb bus divider, the bus clock is generated by cpu_clock/div 0: divider by 1 1: divider by 2 …."]
        #[inline(always)]
        pub const fn set_sub1_div(&mut self, val: super::vals::SubDiv) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[must_use]
        #[inline(always)]
        pub const fn preserve(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[inline(always)]
        pub const fn set_preserve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn loc_busy(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub const fn set_loc_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn glb_busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[inline(always)]
        pub const fn set_glb_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ClockCpu {
        #[inline(always)]
        fn default() -> ClockCpu {
            ClockCpu(0)
        }
    }
    impl core::fmt::Debug for ClockCpu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClockCpu")
                .field("div", &self.div())
                .field("mux", &self.mux())
                .field("sub0_div", &self.sub0_div())
                .field("sub1_div", &self.sub1_div())
                .field("preserve", &self.preserve())
                .field("loc_busy", &self.loc_busy())
                .field("glb_busy", &self.glb_busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ClockCpu {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ClockCpu {{ div: {=u8:?}, mux: {:?}, sub0_div: {:?}, sub1_div: {:?}, preserve: {=bool:?}, loc_busy: {=bool:?}, glb_busy: {=bool:?} }}" , self . div () , self . mux () , self . sub0_div () , self . sub1_div () , self . preserve () , self . loc_busy () , self . glb_busy ())
        }
    }
    #[doc = "Reset Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config(pub u32);
    impl Config {
        #[doc = "time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M."]
        #[must_use]
        #[inline(always)]
        pub const fn post_wait(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M."]
        #[inline(always)]
        pub const fn set_post_wait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M."]
        #[must_use]
        #[inline(always)]
        pub const fn rstclk_num(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M."]
        #[inline(always)]
        pub const fn set_rstclk_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M."]
        #[must_use]
        #[inline(always)]
        pub const fn pre_wait(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M."]
        #[inline(always)]
        pub const fn set_pre_wait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Config {
        #[inline(always)]
        fn default() -> Config {
            Config(0)
        }
    }
    impl core::fmt::Debug for Config {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Config")
                .field("post_wait", &self.post_wait())
                .field("rstclk_num", &self.rstclk_num())
                .field("pre_wait", &self.pre_wait())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Config {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Config {{ post_wait: {=u8:?}, rstclk_num: {=u8:?}, pre_wait: {=u8:?} }}",
                self.post_wait(),
                self.rstclk_num(),
                self.pre_wait()
            )
        }
    }
    #[doc = "Reset Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Counter(pub u32);
    impl Counter {
        #[doc = "self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M."]
        #[must_use]
        #[inline(always)]
        pub const fn counter(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M."]
        #[inline(always)]
        pub const fn set_counter(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Counter {
        #[inline(always)]
        fn default() -> Counter {
            Counter(0)
        }
    }
    impl core::fmt::Debug for Counter {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Counter")
                .field("counter", &self.counter())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Counter {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Counter {{ counter: {=u32:?} }}", self.counter())
        }
    }
    #[doc = "Clock measure result."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Current(pub u32);
    impl Current {
        #[doc = "self updating measure result."]
        #[must_use]
        #[inline(always)]
        pub const fn frequency(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "self updating measure result."]
        #[inline(always)]
        pub const fn set_frequency(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Current {
        #[inline(always)]
        fn default() -> Current {
            Current(0)
        }
    }
    impl core::fmt::Debug for Current {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Current")
                .field("frequency", &self.frequency())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Current {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Current {{ frequency: {=u32:?} }}", self.frequency())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dacclk(pub u32);
    impl Dacclk {
        #[doc = "current mux 0: ana clock 1: ahb clock."]
        #[must_use]
        #[inline(always)]
        pub const fn mux(&self) -> super::vals::AnaClkMux {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::AnaClkMux::from_bits(val as u8)
        }
        #[doc = "current mux 0: ana clock 1: ahb clock."]
        #[inline(always)]
        pub const fn set_mux(&mut self, val: super::vals::AnaClkMux) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[must_use]
        #[inline(always)]
        pub const fn preserve(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[inline(always)]
        pub const fn set_preserve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn loc_busy(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub const fn set_loc_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn glb_busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[inline(always)]
        pub const fn set_glb_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dacclk {
        #[inline(always)]
        fn default() -> Dacclk {
            Dacclk(0)
        }
    }
    impl core::fmt::Debug for Dacclk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dacclk")
                .field("mux", &self.mux())
                .field("preserve", &self.preserve())
                .field("loc_busy", &self.loc_busy())
                .field("glb_busy", &self.glb_busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dacclk {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dacclk {{ mux: {:?}, preserve: {=bool:?}, loc_busy: {=bool:?}, glb_busy: {=bool:?} }}" , self . mux () , self . preserve () , self . loc_busy () , self . glb_busy ())
        }
    }
    #[doc = "Clock senario."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Global00(pub u32);
    impl Global00 {
        #[doc = "global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3."]
        #[must_use]
        #[inline(always)]
        pub const fn mux(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3."]
        #[inline(always)]
        pub const fn set_mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Global00 {
        #[inline(always)]
        fn default() -> Global00 {
            Global00(0)
        }
    }
    impl core::fmt::Debug for Global00 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Global00")
                .field("mux", &self.mux())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Global00 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Global00 {{ mux: {=u8:?} }}", self.mux())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpr(pub u32);
    impl Gpr {
        #[doc = "register for software to handle resume, can save resume address or status."]
        #[must_use]
        #[inline(always)]
        pub const fn gpr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "register for software to handle resume, can save resume address or status."]
        #[inline(always)]
        pub const fn set_gpr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Gpr {
        #[inline(always)]
        fn default() -> Gpr {
            Gpr(0)
        }
    }
    impl core::fmt::Debug for Gpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gpr").field("gpr", &self.gpr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpr {{ gpr: {=u32:?} }}", self.gpr())
        }
    }
    #[doc = "Group setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Group0Clear(pub u32);
    impl Group0Clear {
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: delete periphera in this group，periphera is not needed."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: delete periphera in this group，periphera is not needed."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Group0Clear {
        #[inline(always)]
        fn default() -> Group0Clear {
            Group0Clear(0)
        }
    }
    impl core::fmt::Debug for Group0Clear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Group0Clear")
                .field("link", &self.link())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Group0Clear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Group0Clear {{ link: {=u32:?} }}", self.link())
        }
    }
    #[doc = "Group setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Group0Set(pub u32);
    impl Group0Set {
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: add periphera into this group，periphera is needed."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: add periphera into this group，periphera is needed."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Group0Set {
        #[inline(always)]
        fn default() -> Group0Set {
            Group0Set(0)
        }
    }
    impl core::fmt::Debug for Group0Set {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Group0Set")
                .field("link", &self.link())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Group0Set {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Group0Set {{ link: {=u32:?} }}", self.link())
        }
    }
    #[doc = "Group setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Group0Toggle(pub u32);
    impl Group0Toggle {
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Group0Toggle {
        #[inline(always)]
        fn default() -> Group0Toggle {
            Group0Toggle(0)
        }
    }
    impl core::fmt::Debug for Group0Toggle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Group0Toggle")
                .field("link", &self.link())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Group0Toggle {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Group0Toggle {{ link: {=u32:?} }}", self.link())
        }
    }
    #[doc = "Group setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Group0Value(pub u32);
    impl Group0Value {
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Group0Value {
        #[inline(always)]
        fn default() -> Group0Value {
            Group0Value(0)
        }
    }
    impl core::fmt::Debug for Group0Value {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Group0Value")
                .field("link", &self.link())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Group0Value {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Group0Value {{ link: {=u32:?} }}", self.link())
        }
    }
    #[doc = "Clock upper limit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HighLimit(pub u32);
    impl HighLimit {
        #[doc = "upper frequency."]
        #[must_use]
        #[inline(always)]
        pub const fn frequency(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "upper frequency."]
        #[inline(always)]
        pub const fn set_frequency(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HighLimit {
        #[inline(always)]
        fn default() -> HighLimit {
            HighLimit(0)
        }
    }
    impl core::fmt::Debug for HighLimit {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HighLimit")
                .field("frequency", &self.frequency())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HighLimit {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HighLimit {{ frequency: {=u32:?} }}", self.frequency())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2sclk(pub u32);
    impl I2sclk {
        #[doc = "current mux 0: aud clock 0 1: aud clock 1."]
        #[must_use]
        #[inline(always)]
        pub const fn mux(&self) -> super::vals::I2sClkMux {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::I2sClkMux::from_bits(val as u8)
        }
        #[doc = "current mux 0: aud clock 0 1: aud clock 1."]
        #[inline(always)]
        pub const fn set_mux(&mut self, val: super::vals::I2sClkMux) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[must_use]
        #[inline(always)]
        pub const fn preserve(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[inline(always)]
        pub const fn set_preserve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn loc_busy(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub const fn set_loc_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn glb_busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[inline(always)]
        pub const fn set_glb_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for I2sclk {
        #[inline(always)]
        fn default() -> I2sclk {
            I2sclk(0)
        }
    }
    impl core::fmt::Debug for I2sclk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I2sclk")
                .field("mux", &self.mux())
                .field("preserve", &self.preserve())
                .field("loc_busy", &self.loc_busy())
                .field("glb_busy", &self.glb_busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I2sclk {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "I2sclk {{ mux: {:?}, preserve: {=bool:?}, loc_busy: {=bool:?}, glb_busy: {=bool:?} }}" , self . mux () , self . preserve () , self . loc_busy () , self . glb_busy ())
        }
    }
    #[doc = "Power Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LfWait(pub u32);
    impl LfWait {
        #[doc = "wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz."]
        #[must_use]
        #[inline(always)]
        pub const fn wait(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz."]
        #[inline(always)]
        pub const fn set_wait(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for LfWait {
        #[inline(always)]
        fn default() -> LfWait {
            LfWait(0)
        }
    }
    impl core::fmt::Debug for LfWait {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LfWait")
                .field("wait", &self.wait())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LfWait {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LfWait {{ wait: {=u32:?} }}", self.wait())
        }
    }
    #[doc = "CPU0 Lock GPR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lock(pub u32);
    impl Lock {
        #[doc = "Lock bit for CPU_LOCK."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit for CPU_LOCK."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpr(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset."]
        #[inline(always)]
        pub const fn set_gpr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
    }
    impl Default for Lock {
        #[inline(always)]
        fn default() -> Lock {
            Lock(0)
        }
    }
    impl core::fmt::Debug for Lock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lock")
                .field("lock", &self.lock())
                .field("gpr", &self.gpr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lock {{ lock: {=bool:?}, gpr: {=u16:?} }}",
                self.lock(),
                self.gpr()
            )
        }
    }
    #[doc = "Clock lower limit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LowLimit(pub u32);
    impl LowLimit {
        #[doc = "lower frequency."]
        #[must_use]
        #[inline(always)]
        pub const fn frequency(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "lower frequency."]
        #[inline(always)]
        pub const fn set_frequency(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LowLimit {
        #[inline(always)]
        fn default() -> LowLimit {
            LowLimit(0)
        }
    }
    impl core::fmt::Debug for LowLimit {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LowLimit")
                .field("frequency", &self.frequency())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LowLimit {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LowLimit {{ frequency: {=u32:?} }}", self.frequency())
        }
    }
    #[doc = "CPU0 LP control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lp(pub u32);
    impl Lp {
        #[doc = "Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::LpMode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::LpMode::from_bits(val as u8)
        }
        #[doc = "Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: super::vals::LpMode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 reset not happened 1: CPU0 reset happened."]
        #[must_use]
        #[inline(always)]
        pub const fn reset_flag(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 reset not happened 1: CPU0 reset happened."]
        #[inline(always)]
        pub const fn set_reset_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened."]
        #[must_use]
        #[inline(always)]
        pub const fn sleep_flag(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened."]
        #[inline(always)]
        pub const fn set_sleep_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wake up happened."]
        #[must_use]
        #[inline(always)]
        pub const fn wake_flag(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wake up happened."]
        #[inline(always)]
        pub const fn set_wake_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CPU0 is executing 0: CPU0 is not executing 1: CPU0 is executing."]
        #[must_use]
        #[inline(always)]
        pub const fn exec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CPU0 is executing 0: CPU0 is not executing 1: CPU0 is executing."]
        #[inline(always)]
        pub const fn set_exec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CPU0 is waking up 0: CPU0 wake up not asserted 1: CPU0 wake up asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn wake(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CPU0 is waking up 0: CPU0 wake up not asserted 1: CPU0 wake up asserted."]
        #[inline(always)]
        pub const fn set_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI."]
        #[must_use]
        #[inline(always)]
        pub const fn halt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI."]
        #[inline(always)]
        pub const fn set_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CPU0 wake up counter, counter satuated at 255, write 0x00 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn wake_cnt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "CPU0 wake up counter, counter satuated at 255, write 0x00 to clear."]
        #[inline(always)]
        pub const fn set_wake_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Lp {
        #[inline(always)]
        fn default() -> Lp {
            Lp(0)
        }
    }
    impl core::fmt::Debug for Lp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lp")
                .field("mode", &self.mode())
                .field("reset_flag", &self.reset_flag())
                .field("sleep_flag", &self.sleep_flag())
                .field("wake_flag", &self.wake_flag())
                .field("exec", &self.exec())
                .field("wake", &self.wake())
                .field("halt", &self.halt())
                .field("wake_cnt", &self.wake_cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Lp {{ mode: {:?}, reset_flag: {=bool:?}, sleep_flag: {=bool:?}, wake_flag: {=bool:?}, exec: {=bool:?}, wake: {=bool:?}, halt: {=bool:?}, wake_cnt: {=u8:?} }}" , self . mode () , self . reset_flag () , self . sleep_flag () , self . wake_flag () , self . exec () , self . wake () , self . halt () , self . wake_cnt ())
        }
    }
    #[doc = "Clock measure and monitor control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorControl(pub u32);
    impl MonitorControl {
        #[doc = "clock measurement selection."]
        #[must_use]
        #[inline(always)]
        pub const fn selection(&self) -> super::vals::MonitorSelection {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::MonitorSelection::from_bits(val as u8)
        }
        #[doc = "clock measurement selection."]
        #[inline(always)]
        pub const fn set_selection(&mut self, val: super::vals::MonitorSelection) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "reference clock selection, 0: 32k 1: 24M."]
        #[must_use]
        #[inline(always)]
        pub const fn reference(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "reference clock selection, 0: 32k 1: 24M."]
        #[inline(always)]
        pub const fn set_reference(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz."]
        #[must_use]
        #[inline(always)]
        pub const fn accuracy(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz."]
        #[inline(always)]
        pub const fn set_accuracy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "start measurement."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "start measurement."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "clock frequency lower than lower limit."]
        #[must_use]
        #[inline(always)]
        pub const fn low(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "clock frequency lower than lower limit."]
        #[inline(always)]
        pub const fn set_low(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "clock frequency higher than upper limit."]
        #[must_use]
        #[inline(always)]
        pub const fn high(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "clock frequency higher than upper limit."]
        #[inline(always)]
        pub const fn set_high(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "output divider."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "output divider."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "enable clock output."]
        #[must_use]
        #[inline(always)]
        pub const fn outen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "enable clock output."]
        #[inline(always)]
        pub const fn set_outen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "divider is applying new setting."]
        #[must_use]
        #[inline(always)]
        pub const fn div_busy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "divider is applying new setting."]
        #[inline(always)]
        pub const fn set_div_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "result is ready for read 0: not ready 1: result is ready."]
        #[must_use]
        #[inline(always)]
        pub const fn valid(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "result is ready for read 0: not ready 1: result is ready."]
        #[inline(always)]
        pub const fn set_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MonitorControl {
        #[inline(always)]
        fn default() -> MonitorControl {
            MonitorControl(0)
        }
    }
    impl core::fmt::Debug for MonitorControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorControl")
                .field("selection", &self.selection())
                .field("reference", &self.reference())
                .field("accuracy", &self.accuracy())
                .field("mode", &self.mode())
                .field("start", &self.start())
                .field("low", &self.low())
                .field("high", &self.high())
                .field("div", &self.div())
                .field("outen", &self.outen())
                .field("div_busy", &self.div_busy())
                .field("valid", &self.valid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorControl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MonitorControl {{ selection: {:?}, reference: {=bool:?}, accuracy: {=bool:?}, mode: {=bool:?}, start: {=bool:?}, low: {=bool:?}, high: {=bool:?}, div: {=u8:?}, outen: {=bool:?}, div_busy: {=bool:?}, valid: {=bool:?} }}" , self . selection () , self . reference () , self . accuracy () , self . mode () , self . start () , self . low () , self . high () , self . div () , self . outen () , self . div_busy () , self . valid ())
        }
    }
    #[doc = "Power Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OffWait(pub u32);
    impl OffWait {
        #[doc = "wait time for power switch turn off, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz."]
        #[must_use]
        #[inline(always)]
        pub const fn wait(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "wait time for power switch turn off, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz."]
        #[inline(always)]
        pub const fn set_wait(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for OffWait {
        #[inline(always)]
        fn default() -> OffWait {
            OffWait(0)
        }
    }
    impl core::fmt::Debug for OffWait {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OffWait")
                .field("wait", &self.wait())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OffWait {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OffWait {{ wait: {=u32:?} }}", self.wait())
        }
    }
    #[doc = "Reset Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetControl(pub u32);
    impl ResetControl {
        #[doc = "perform reset and release imediately 0: reset is released 1 reset is asserted and will release automatically."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "perform reset and release imediately 0: reset is released 1 reset is asserted and will release automatically."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold."]
        #[must_use]
        #[inline(always)]
        pub const fn hold(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold."]
        #[inline(always)]
        pub const fn set_hold(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_wake(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit."]
        #[inline(always)]
        pub const fn set_flag_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn flag(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit."]
        #[inline(always)]
        pub const fn set_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ResetControl {
        #[inline(always)]
        fn default() -> ResetControl {
            ResetControl(0)
        }
    }
    impl core::fmt::Debug for ResetControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ResetControl")
                .field("reset", &self.reset())
                .field("hold", &self.hold())
                .field("flag_wake", &self.flag_wake())
                .field("flag", &self.flag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ResetControl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ResetControl {{ reset: {=bool:?}, hold: {=bool:?}, flag_wake: {=bool:?}, flag: {=bool:?} }}" , self . reset () , self . hold () , self . flag_wake () , self . flag ())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Resource(pub u32);
    impl Resource {
        #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "local busy 0: no change is pending for current node 1: current node is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn loc_busy(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "local busy 0: no change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub const fn set_loc_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status."]
        #[must_use]
        #[inline(always)]
        pub const fn glb_busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status."]
        #[inline(always)]
        pub const fn set_glb_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Resource {
        #[inline(always)]
        fn default() -> Resource {
            Resource(0)
        }
    }
    impl core::fmt::Debug for Resource {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Resource")
                .field("mode", &self.mode())
                .field("loc_busy", &self.loc_busy())
                .field("glb_busy", &self.glb_busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Resource {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Resource {{ mode: {=u8:?}, loc_busy: {=bool:?}, glb_busy: {=bool:?} }}",
                self.mode(),
                self.loc_busy(),
                self.glb_busy()
            )
        }
    }
    #[doc = "Retention Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Retention(pub u32);
    impl Retention {
        #[doc = "retention setting while CPU0 enter stop mode, each bit represents a resource bit00: soc_mem is kept on while cpu stop, bit01: soc_ctx is kept on while cpu stop, bit02: cpu0_mem is kept on while cpu stop, bit03: cpu0_ctx is kept on while cpu stop, bit04: xtal_hold is kept on while cpu stop, bit05: pll0_hold is kept on while cpu stop, bit06: pll1_hold is kept on while cpu stop, bit07: pll2_hold is kept on while cpu stop,."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "retention setting while CPU0 enter stop mode, each bit represents a resource bit00: soc_mem is kept on while cpu stop, bit01: soc_ctx is kept on while cpu stop, bit02: cpu0_mem is kept on while cpu stop, bit03: cpu0_ctx is kept on while cpu stop, bit04: xtal_hold is kept on while cpu stop, bit05: pll0_hold is kept on while cpu stop, bit06: pll1_hold is kept on while cpu stop, bit07: pll2_hold is kept on while cpu stop,."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Retention {
        #[inline(always)]
        fn default() -> Retention {
            Retention(0)
        }
    }
    impl core::fmt::Debug for Retention {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Retention")
                .field("link", &self.link())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Retention {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Retention {{ link: {=u8:?} }}", self.link())
        }
    }
    #[doc = "Power Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off."]
        #[must_use]
        #[inline(always)]
        pub const fn lf_ack(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off."]
        #[inline(always)]
        pub const fn set_lf_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off."]
        #[must_use]
        #[inline(always)]
        pub const fn lf_disable(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off."]
        #[inline(always)]
        pub const fn set_lf_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn flag_wake(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit."]
        #[inline(always)]
        pub const fn set_flag_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn flag(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit."]
        #[inline(always)]
        pub const fn set_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    impl core::fmt::Debug for Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Status")
                .field("lf_ack", &self.lf_ack())
                .field("lf_disable", &self.lf_disable())
                .field("flag_wake", &self.flag_wake())
                .field("flag", &self.flag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Status {{ lf_ack: {=bool:?}, lf_disable: {=bool:?}, flag_wake: {=bool:?}, flag: {=bool:?} }}" , self . lf_ack () , self . lf_disable () , self . flag_wake () , self . flag ())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WakeupEnable(pub u32);
    impl WakeupEnable {
        #[doc = "IRQ wakeup enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IRQ wakeup enable."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WakeupEnable {
        #[inline(always)]
        fn default() -> WakeupEnable {
            WakeupEnable(0)
        }
    }
    impl core::fmt::Debug for WakeupEnable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WakeupEnable")
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WakeupEnable {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WakeupEnable {{ enable: {=u32:?} }}", self.enable())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WakeupStatus(pub u32);
    impl WakeupStatus {
        #[doc = "IRQ values."]
        #[must_use]
        #[inline(always)]
        pub const fn status(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IRQ values."]
        #[inline(always)]
        pub const fn set_status(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WakeupStatus {
        #[inline(always)]
        fn default() -> WakeupStatus {
            WakeupStatus(0)
        }
    }
    impl core::fmt::Debug for WakeupStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WakeupStatus")
                .field("status", &self.status())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WakeupStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WakeupStatus {{ status: {=u32:?} }}", self.status())
        }
    }
}
pub mod vals {
    #[doc = "no description available."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AnaClkMux {
        AHB = 0x0,
        ANA = 0x01,
    }
    impl AnaClkMux {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AnaClkMux {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AnaClkMux {
        #[inline(always)]
        fn from(val: u8) -> AnaClkMux {
            AnaClkMux::from_bits(val)
        }
    }
    impl From<AnaClkMux> for u8 {
        #[inline(always)]
        fn from(val: AnaClkMux) -> u8 {
            AnaClkMux::to_bits(val)
        }
    }
    #[doc = "no description available."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ClockMux {
        #[doc = "24MHz"]
        CLK_24M = 0x0,
        #[doc = "Dfaults to 400MHz. Default clock source for CPU0"]
        PLL0CLK0 = 0x01,
        #[doc = "Defaults to 333MHz"]
        PLL0CLK1 = 0x02,
        #[doc = "Defaults to 250MHz"]
        PLL0CLK2 = 0x03,
        #[doc = "Defaults to 48MHz"]
        PLL1CLK0 = 0x04,
        #[doc = "Defaults to 320MHz"]
        PLL1CLK1 = 0x05,
        #[doc = "Defaults to 516.096MHz"]
        PLL2CLK0 = 0x06,
        #[doc = "Defaults to 451.584MHz"]
        PLL2CLK1 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl ClockMux {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ClockMux {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ClockMux {
        #[inline(always)]
        fn from(val: u8) -> ClockMux {
            ClockMux::from_bits(val)
        }
    }
    impl From<ClockMux> for u8 {
        #[inline(always)]
        fn from(val: ClockMux) -> u8 {
            ClockMux::to_bits(val)
        }
    }
    #[doc = "no description available."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2sClkMux {
        AUD0 = 0x0,
        AUD1 = 0x01,
    }
    impl I2sClkMux {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2sClkMux {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2sClkMux {
        #[inline(always)]
        fn from(val: u8) -> I2sClkMux {
            I2sClkMux::from_bits(val)
        }
    }
    impl From<I2sClkMux> for u8 {
        #[inline(always)]
        fn from(val: I2sClkMux) -> u8 {
            I2sClkMux::to_bits(val)
        }
    }
    #[doc = "In low power mode, the behavior after setting CPU WFI"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum LpMode {
        WAIT = 0x0,
        STOP = 0x01,
        RUN = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl LpMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> LpMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for LpMode {
        #[inline(always)]
        fn from(val: u8) -> LpMode {
            LpMode::from_bits(val)
        }
    }
    impl From<LpMode> for u8 {
        #[inline(always)]
        fn from(val: LpMode) -> u8 {
            LpMode::to_bits(val)
        }
    }
    #[doc = "clock measurement selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct MonitorSelection(u8);
    impl MonitorSelection {
        pub const CLK_32K: Self = Self(0x0);
        pub const CLK_RC24M: Self = Self(0x01);
        pub const CLK_XTAL24M: Self = Self(0x02);
        pub const CLK_USB0_PHY: Self = Self(0x03);
        pub const CLK_24M: Self = Self(0x08);
        pub const PLL0CLK0: Self = Self(0x09);
        pub const PLL1CLK0: Self = Self(0x0a);
        pub const PLL2CLK0: Self = Self(0x0b);
        pub const PLL0CLK1: Self = Self(0x0c);
        pub const PLL1CLK1: Self = Self(0x0d);
        pub const PLL0CLK2: Self = Self(0x0e);
        pub const PLL1CLK2: Self = Self(0x0f);
        pub const CLK_TOP_CPU0: Self = Self(0x80);
        pub const CLK_TOP_MCHTMR0: Self = Self(0x81);
        pub const CLK_TOP_FEMC: Self = Self(0x88);
        pub const CLK_TOP_XPI0: Self = Self(0x89);
        pub const CLK_TOP_XPI1: Self = Self(0x8a);
        pub const CLK_TOP_GPTMR0: Self = Self(0x8b);
        pub const CLK_TOP_GPTMR1: Self = Self(0x8c);
        pub const CLK_TOP_GPTMR2: Self = Self(0x8d);
        pub const CLK_TOP_GPTMR3: Self = Self(0x8e);
        pub const CLK_TOP_UART0: Self = Self(0x93);
        pub const CLK_TOP_UART1: Self = Self(0x94);
        pub const CLK_TOP_UART2: Self = Self(0x95);
        pub const CLK_TOP_UART3: Self = Self(0x96);
        pub const CLK_TOP_UART4: Self = Self(0x97);
        pub const CLK_TOP_UART5: Self = Self(0x98);
        pub const CLK_TOP_UART6: Self = Self(0x99);
        pub const CLK_TOP_UART7: Self = Self(0x9a);
        pub const CLK_TOP_I2C0: Self = Self(0xa3);
        pub const CLK_TOP_I2C1: Self = Self(0xa4);
        pub const CLK_TOP_I2C2: Self = Self(0xa5);
        pub const CLK_TOP_I2C3: Self = Self(0xa6);
        pub const CLK_TOP_SPI0: Self = Self(0xa7);
        pub const CLK_TOP_SPI1: Self = Self(0xa8);
        pub const CLK_TOP_SPI2: Self = Self(0xa9);
        pub const CLK_TOP_SPI3: Self = Self(0xaa);
        pub const CLK_TOP_CAN0: Self = Self(0xab);
        pub const CLK_TOP_CAN1: Self = Self(0xac);
        pub const CLK_TOP_PTPC: Self = Self(0xaf);
        pub const CLK_TOP_ANA0: Self = Self(0xb0);
        pub const CLK_TOP_ANA1: Self = Self(0xb1);
        pub const CLK_TOP_ANA2: Self = Self(0xb2);
        pub const CLK_TOP_ANA3: Self = Self(0xb3);
        pub const CLK_TOP_AUD0: Self = Self(0xb4);
        pub const CLK_TOP_AUD1: Self = Self(0xb5);
        pub const CLK_TOP_ETH0: Self = Self(0xb9);
        pub const CLK_TOP_PTP0: Self = Self(0xbb);
        pub const CLK_TOP_REF0: Self = Self(0xbd);
        pub const CLK_TOP_REF1: Self = Self(0xbe);
        pub const CLK_TOP_NTMR0: Self = Self(0xbf);
        pub const CLK_TOP_SDXC0: Self = Self(0xc1);
    }
    impl MonitorSelection {
        pub const fn from_bits(val: u8) -> MonitorSelection {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for MonitorSelection {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("CLK_32K"),
                0x01 => f.write_str("CLK_RC24M"),
                0x02 => f.write_str("CLK_XTAL24M"),
                0x03 => f.write_str("CLK_USB0_PHY"),
                0x08 => f.write_str("CLK_24M"),
                0x09 => f.write_str("PLL0CLK0"),
                0x0a => f.write_str("PLL1CLK0"),
                0x0b => f.write_str("PLL2CLK0"),
                0x0c => f.write_str("PLL0CLK1"),
                0x0d => f.write_str("PLL1CLK1"),
                0x0e => f.write_str("PLL0CLK2"),
                0x0f => f.write_str("PLL1CLK2"),
                0x80 => f.write_str("CLK_TOP_CPU0"),
                0x81 => f.write_str("CLK_TOP_MCHTMR0"),
                0x88 => f.write_str("CLK_TOP_FEMC"),
                0x89 => f.write_str("CLK_TOP_XPI0"),
                0x8a => f.write_str("CLK_TOP_XPI1"),
                0x8b => f.write_str("CLK_TOP_GPTMR0"),
                0x8c => f.write_str("CLK_TOP_GPTMR1"),
                0x8d => f.write_str("CLK_TOP_GPTMR2"),
                0x8e => f.write_str("CLK_TOP_GPTMR3"),
                0x93 => f.write_str("CLK_TOP_UART0"),
                0x94 => f.write_str("CLK_TOP_UART1"),
                0x95 => f.write_str("CLK_TOP_UART2"),
                0x96 => f.write_str("CLK_TOP_UART3"),
                0x97 => f.write_str("CLK_TOP_UART4"),
                0x98 => f.write_str("CLK_TOP_UART5"),
                0x99 => f.write_str("CLK_TOP_UART6"),
                0x9a => f.write_str("CLK_TOP_UART7"),
                0xa3 => f.write_str("CLK_TOP_I2C0"),
                0xa4 => f.write_str("CLK_TOP_I2C1"),
                0xa5 => f.write_str("CLK_TOP_I2C2"),
                0xa6 => f.write_str("CLK_TOP_I2C3"),
                0xa7 => f.write_str("CLK_TOP_SPI0"),
                0xa8 => f.write_str("CLK_TOP_SPI1"),
                0xa9 => f.write_str("CLK_TOP_SPI2"),
                0xaa => f.write_str("CLK_TOP_SPI3"),
                0xab => f.write_str("CLK_TOP_CAN0"),
                0xac => f.write_str("CLK_TOP_CAN1"),
                0xaf => f.write_str("CLK_TOP_PTPC"),
                0xb0 => f.write_str("CLK_TOP_ANA0"),
                0xb1 => f.write_str("CLK_TOP_ANA1"),
                0xb2 => f.write_str("CLK_TOP_ANA2"),
                0xb3 => f.write_str("CLK_TOP_ANA3"),
                0xb4 => f.write_str("CLK_TOP_AUD0"),
                0xb5 => f.write_str("CLK_TOP_AUD1"),
                0xb9 => f.write_str("CLK_TOP_ETH0"),
                0xbb => f.write_str("CLK_TOP_PTP0"),
                0xbd => f.write_str("CLK_TOP_REF0"),
                0xbe => f.write_str("CLK_TOP_REF1"),
                0xbf => f.write_str("CLK_TOP_NTMR0"),
                0xc1 => f.write_str("CLK_TOP_SDXC0"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorSelection {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "CLK_32K"),
                0x01 => defmt::write!(f, "CLK_RC24M"),
                0x02 => defmt::write!(f, "CLK_XTAL24M"),
                0x03 => defmt::write!(f, "CLK_USB0_PHY"),
                0x08 => defmt::write!(f, "CLK_24M"),
                0x09 => defmt::write!(f, "PLL0CLK0"),
                0x0a => defmt::write!(f, "PLL1CLK0"),
                0x0b => defmt::write!(f, "PLL2CLK0"),
                0x0c => defmt::write!(f, "PLL0CLK1"),
                0x0d => defmt::write!(f, "PLL1CLK1"),
                0x0e => defmt::write!(f, "PLL0CLK2"),
                0x0f => defmt::write!(f, "PLL1CLK2"),
                0x80 => defmt::write!(f, "CLK_TOP_CPU0"),
                0x81 => defmt::write!(f, "CLK_TOP_MCHTMR0"),
                0x88 => defmt::write!(f, "CLK_TOP_FEMC"),
                0x89 => defmt::write!(f, "CLK_TOP_XPI0"),
                0x8a => defmt::write!(f, "CLK_TOP_XPI1"),
                0x8b => defmt::write!(f, "CLK_TOP_GPTMR0"),
                0x8c => defmt::write!(f, "CLK_TOP_GPTMR1"),
                0x8d => defmt::write!(f, "CLK_TOP_GPTMR2"),
                0x8e => defmt::write!(f, "CLK_TOP_GPTMR3"),
                0x93 => defmt::write!(f, "CLK_TOP_UART0"),
                0x94 => defmt::write!(f, "CLK_TOP_UART1"),
                0x95 => defmt::write!(f, "CLK_TOP_UART2"),
                0x96 => defmt::write!(f, "CLK_TOP_UART3"),
                0x97 => defmt::write!(f, "CLK_TOP_UART4"),
                0x98 => defmt::write!(f, "CLK_TOP_UART5"),
                0x99 => defmt::write!(f, "CLK_TOP_UART6"),
                0x9a => defmt::write!(f, "CLK_TOP_UART7"),
                0xa3 => defmt::write!(f, "CLK_TOP_I2C0"),
                0xa4 => defmt::write!(f, "CLK_TOP_I2C1"),
                0xa5 => defmt::write!(f, "CLK_TOP_I2C2"),
                0xa6 => defmt::write!(f, "CLK_TOP_I2C3"),
                0xa7 => defmt::write!(f, "CLK_TOP_SPI0"),
                0xa8 => defmt::write!(f, "CLK_TOP_SPI1"),
                0xa9 => defmt::write!(f, "CLK_TOP_SPI2"),
                0xaa => defmt::write!(f, "CLK_TOP_SPI3"),
                0xab => defmt::write!(f, "CLK_TOP_CAN0"),
                0xac => defmt::write!(f, "CLK_TOP_CAN1"),
                0xaf => defmt::write!(f, "CLK_TOP_PTPC"),
                0xb0 => defmt::write!(f, "CLK_TOP_ANA0"),
                0xb1 => defmt::write!(f, "CLK_TOP_ANA1"),
                0xb2 => defmt::write!(f, "CLK_TOP_ANA2"),
                0xb3 => defmt::write!(f, "CLK_TOP_ANA3"),
                0xb4 => defmt::write!(f, "CLK_TOP_AUD0"),
                0xb5 => defmt::write!(f, "CLK_TOP_AUD1"),
                0xb9 => defmt::write!(f, "CLK_TOP_ETH0"),
                0xbb => defmt::write!(f, "CLK_TOP_PTP0"),
                0xbd => defmt::write!(f, "CLK_TOP_REF0"),
                0xbe => defmt::write!(f, "CLK_TOP_REF1"),
                0xbf => defmt::write!(f, "CLK_TOP_NTMR0"),
                0xc1 => defmt::write!(f, "CLK_TOP_SDXC0"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for MonitorSelection {
        #[inline(always)]
        fn from(val: u8) -> MonitorSelection {
            MonitorSelection::from_bits(val)
        }
    }
    impl From<MonitorSelection> for u8 {
        #[inline(always)]
        fn from(val: MonitorSelection) -> u8 {
            MonitorSelection::to_bits(val)
        }
    }
    #[doc = "AHB div"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SubDiv {
        DIV1 = 0x0,
        DIV2 = 0x01,
        DIV3 = 0x02,
        DIV4 = 0x03,
        DIV5 = 0x04,
        DIV6 = 0x05,
        DIV7 = 0x06,
        DIV8 = 0x07,
        DIV9 = 0x08,
        DIV10 = 0x09,
        DIV11 = 0x0a,
        DIV12 = 0x0b,
        DIV13 = 0x0c,
        DIV14 = 0x0d,
        DIV15 = 0x0e,
        DIV16 = 0x0f,
    }
    impl SubDiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SubDiv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SubDiv {
        #[inline(always)]
        fn from(val: u8) -> SubDiv {
            SubDiv::from_bits(val)
        }
    }
    impl From<SubDiv> for u8 {
        #[inline(always)]
        fn from(val: SubDiv) -> u8 {
            SubDiv::to_bits(val)
        }
    }
}
