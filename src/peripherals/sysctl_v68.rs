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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Affiliate of Group."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::AffiliateSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Affiliate of Group."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::AffiliateClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Affiliate of Group."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::AffiliateToggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "CPU0 Lock GPR."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn gpr(self, n: usize) -> crate::common::Reg<regs::Gpr, crate::common::RW> {
        assert!(n < 14usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wakeup_status(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WakeupStatus, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wakeup_enable(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WakeupEnable, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Group setting."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::Group0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Group setting."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::Group0Clear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Group setting."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::Group0Toggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Clock measure result."]
    #[inline(always)]
    pub const fn current(self) -> crate::common::Reg<regs::Current, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Clock lower limit."]
    #[inline(always)]
    pub const fn low_limit(self) -> crate::common::Reg<regs::LowLimit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Clock upper limit."]
    #[inline(always)]
    pub const fn high_limit(self) -> crate::common::Reg<regs::HighLimit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Power Setting."]
    #[inline(always)]
    pub const fn lf_wait(self) -> crate::common::Reg<regs::LfWait, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Power Setting."]
    #[inline(always)]
    pub const fn off_wait(self) -> crate::common::Reg<regs::OffWait, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Reset Setting."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Reset Setting."]
    #[inline(always)]
    pub const fn counter(self) -> crate::common::Reg<regs::Counter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
    pub const fn value(self) -> crate::common::Reg<regs::RetentionValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Retention Control."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::RetentionSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Retention Control."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::RetentionClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Retention Control."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::RetentionToggle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
        assert!(n < 353usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn group0(self, n: usize) -> Group0 {
        assert!(n < 4usize);
        unsafe { Group0::from_ptr(self.ptr.add(0x0800usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn affiliate(self, n: usize) -> Affiliate {
        assert!(n < 1usize);
        unsafe { Affiliate::from_ptr(self.ptr.add(0x0900usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn retention(self, n: usize) -> Retention {
        assert!(n < 1usize);
        unsafe { Retention::from_ptr(self.ptr.add(0x0920usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn power(self, n: usize) -> Power {
        assert!(n < 3usize);
        unsafe { Power::from_ptr(self.ptr.add(0x1000usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn reset(self, n: usize) -> Reset {
        assert!(n < 5usize);
        unsafe { Reset::from_ptr(self.ptr.add(0x1400usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn clock(self, n: usize) -> crate::common::Reg<regs::Clock, crate::common::RW> {
        assert!(n < 69usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1800usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn adcclk(self, n: usize) -> crate::common::Reg<regs::Adcclk, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c00usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn i2sclk(self, n: usize) -> crate::common::Reg<regs::I2sclk, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c08usize + n * 4usize) as _) }
    }
    #[doc = "Clock senario."]
    #[inline(always)]
    pub const fn global00(self) -> crate::common::Reg<regs::Global00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2000usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn monitor(self, n: usize) -> Monitor {
        assert!(n < 4usize);
        unsafe { Monitor::from_ptr(self.ptr.add(0x2400usize + n * 32usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cpu(self, n: usize) -> Cpu {
        assert!(n < 1usize);
        unsafe { Cpu::from_ptr(self.ptr.add(0x2800usize + n * 1024usize) as _) }
    }
}
pub mod regs {
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adcclk(pub u32);
    impl Adcclk {
        #[doc = "current mux 0: ana clock N 1: axi clock."]
        #[inline(always)]
        pub const fn mux(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "current mux 0: ana clock N 1: axi clock."]
        #[inline(always)]
        pub fn set_mux(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[inline(always)]
        pub const fn preserve(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[inline(always)]
        pub fn set_preserve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub const fn loc_busy(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub fn set_loc_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[inline(always)]
        pub const fn glb_busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[inline(always)]
        pub fn set_glb_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Adcclk {
        #[inline(always)]
        fn default() -> Adcclk {
            Adcclk(0)
        }
    }
    #[doc = "Affiliate of Group."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AffiliateClear(pub u32);
    impl AffiliateClear {
        #[doc = "Affiliate groups of cpu0, each bit represents a group 0: no effect 1: the group is not assigned to CPU0."]
        #[inline(always)]
        pub const fn link(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Affiliate groups of cpu0, each bit represents a group 0: no effect 1: the group is not assigned to CPU0."]
        #[inline(always)]
        pub fn set_link(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for AffiliateClear {
        #[inline(always)]
        fn default() -> AffiliateClear {
            AffiliateClear(0)
        }
    }
    #[doc = "Affiliate of Group."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AffiliateSet(pub u32);
    impl AffiliateSet {
        #[doc = "Affiliate groups of cpu0，each bit represents a group 0: no effect 1: the group is assigned to CPU0."]
        #[inline(always)]
        pub const fn link(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Affiliate groups of cpu0，each bit represents a group 0: no effect 1: the group is assigned to CPU0."]
        #[inline(always)]
        pub fn set_link(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for AffiliateSet {
        #[inline(always)]
        fn default() -> AffiliateSet {
            AffiliateSet(0)
        }
    }
    #[doc = "Affiliate of Group."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AffiliateToggle(pub u32);
    impl AffiliateToggle {
        #[doc = "Affiliate groups of cpu0, each bit represents a group 0: no effect 1: toggle the result that whether the group is assigned to CPU0 before."]
        #[inline(always)]
        pub const fn link(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Affiliate groups of cpu0, each bit represents a group 0: no effect 1: toggle the result that whether the group is assigned to CPU0 before."]
        #[inline(always)]
        pub fn set_link(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for AffiliateToggle {
        #[inline(always)]
        fn default() -> AffiliateToggle {
            AffiliateToggle(0)
        }
    }
    #[doc = "Affiliate of Group."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AffiliateValue(pub u32);
    impl AffiliateValue {
        #[doc = "Affiliate groups of cpu0, each bit represents a group bit0: cpu0 depends on group0 bit1: cpu0 depends on group1 bit2: cpu0 depends on group2 bit3: cpu0 depends on group3."]
        #[inline(always)]
        pub const fn link(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Affiliate groups of cpu0, each bit represents a group bit0: cpu0 depends on group0 bit1: cpu0 depends on group1 bit2: cpu0 depends on group2 bit3: cpu0 depends on group3."]
        #[inline(always)]
        pub fn set_link(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for AffiliateValue {
        #[inline(always)]
        fn default() -> AffiliateValue {
            AffiliateValue(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clock(pub u32);
    impl Clock {
        #[doc = "clock divider 0: divider by 1 1: divider by 2 2: divider by 3 . . . 255: divider by 256."]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "clock divider 0: divider by 1 1: divider by 2 2: divider by 3 . . . 255: divider by 256."]
        #[inline(always)]
        pub fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "current mux in clock component 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0."]
        #[inline(always)]
        pub const fn mux(&self) -> super::vals::ClockMux {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::ClockMux::from_bits(val as u8)
        }
        #[doc = "current mux in clock component 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0."]
        #[inline(always)]
        pub fn set_mux(&mut self, val: super::vals::ClockMux) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[inline(always)]
        pub const fn preserve(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[inline(always)]
        pub fn set_preserve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub const fn loc_busy(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub fn set_loc_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[inline(always)]
        pub const fn glb_busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[inline(always)]
        pub fn set_glb_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Clock {
        #[inline(always)]
        fn default() -> Clock {
            Clock(0)
        }
    }
    #[doc = "Reset Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config(pub u32);
    impl Config {
        #[doc = "time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M."]
        #[inline(always)]
        pub const fn post_wait(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M."]
        #[inline(always)]
        pub fn set_post_wait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M."]
        #[inline(always)]
        pub const fn rstclk_num(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M."]
        #[inline(always)]
        pub fn set_rstclk_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M."]
        #[inline(always)]
        pub const fn pre_wait(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M."]
        #[inline(always)]
        pub fn set_pre_wait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Config {
        #[inline(always)]
        fn default() -> Config {
            Config(0)
        }
    }
    #[doc = "Reset Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Counter(pub u32);
    impl Counter {
        #[doc = "self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M."]
        #[inline(always)]
        pub const fn counter(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M."]
        #[inline(always)]
        pub fn set_counter(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Counter {
        #[inline(always)]
        fn default() -> Counter {
            Counter(0)
        }
    }
    #[doc = "Clock measure result."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Current(pub u32);
    impl Current {
        #[doc = "self updating measure result."]
        #[inline(always)]
        pub const fn frequency(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "self updating measure result."]
        #[inline(always)]
        pub fn set_frequency(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Current {
        #[inline(always)]
        fn default() -> Current {
            Current(0)
        }
    }
    #[doc = "Clock senario."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Global00(pub u32);
    impl Global00 {
        #[doc = "global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3 bit4: override to preset4 bit5: override to preset5 bit6: override to preset6 bit7: override to preset7."]
        #[inline(always)]
        pub const fn mux(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3 bit4: override to preset4 bit5: override to preset5 bit6: override to preset6 bit7: override to preset7."]
        #[inline(always)]
        pub fn set_mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Global00 {
        #[inline(always)]
        fn default() -> Global00 {
            Global00(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpr(pub u32);
    impl Gpr {
        #[doc = "register for software to handle resume, can save resume address or status."]
        #[inline(always)]
        pub const fn gpr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "register for software to handle resume, can save resume address or status."]
        #[inline(always)]
        pub fn set_gpr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Gpr {
        #[inline(always)]
        fn default() -> Gpr {
            Gpr(0)
        }
    }
    #[doc = "Group setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Group0Clear(pub u32);
    impl Group0Clear {
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: delete periphera in this group，periphera is not needed."]
        #[inline(always)]
        pub const fn link(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: delete periphera in this group，periphera is not needed."]
        #[inline(always)]
        pub fn set_link(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Group0Clear {
        #[inline(always)]
        fn default() -> Group0Clear {
            Group0Clear(0)
        }
    }
    #[doc = "Group setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Group0Set(pub u32);
    impl Group0Set {
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: add periphera into this group，periphera is needed."]
        #[inline(always)]
        pub const fn link(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: add periphera into this group，periphera is needed."]
        #[inline(always)]
        pub fn set_link(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Group0Set {
        #[inline(always)]
        fn default() -> Group0Set {
            Group0Set(0)
        }
    }
    #[doc = "Group setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Group0Toggle(pub u32);
    impl Group0Toggle {
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before."]
        #[inline(always)]
        pub const fn link(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: no effect 1: toggle the result that whether periphera is needed before."]
        #[inline(always)]
        pub fn set_link(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Group0Toggle {
        #[inline(always)]
        fn default() -> Group0Toggle {
            Group0Toggle(0)
        }
    }
    #[doc = "Group setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Group0Value(pub u32);
    impl Group0Value {
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed."]
        #[inline(always)]
        pub const fn link(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "denpendency on peripherals, index count from resource ahbp(0x400), each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed."]
        #[inline(always)]
        pub fn set_link(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Group0Value {
        #[inline(always)]
        fn default() -> Group0Value {
            Group0Value(0)
        }
    }
    #[doc = "Clock upper limit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HighLimit(pub u32);
    impl HighLimit {
        #[doc = "upper frequency."]
        #[inline(always)]
        pub const fn frequency(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "upper frequency."]
        #[inline(always)]
        pub fn set_frequency(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HighLimit {
        #[inline(always)]
        fn default() -> HighLimit {
            HighLimit(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2sclk(pub u32);
    impl I2sclk {
        #[doc = "current mux 0: aud clock N 1: aud clock 0."]
        #[inline(always)]
        pub const fn mux(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "current mux 0: aud clock N 1: aud clock 0."]
        #[inline(always)]
        pub fn set_mux(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[inline(always)]
        pub const fn preserve(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "preserve function against global select 0: select global clock setting 1: not select global clock setting."]
        #[inline(always)]
        pub fn set_preserve(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub const fn loc_busy(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "local busy 0: a change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub fn set_loc_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[inline(always)]
        pub const fn glb_busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status."]
        #[inline(always)]
        pub fn set_glb_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for I2sclk {
        #[inline(always)]
        fn default() -> I2sclk {
            I2sclk(0)
        }
    }
    #[doc = "Power Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LfWait(pub u32);
    impl LfWait {
        #[doc = "wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz."]
        #[inline(always)]
        pub const fn wait(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz."]
        #[inline(always)]
        pub fn set_wait(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for LfWait {
        #[inline(always)]
        fn default() -> LfWait {
            LfWait(0)
        }
    }
    #[doc = "CPU0 Lock GPR."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lock(pub u32);
    impl Lock {
        #[doc = "Lock bit for CPU_LOCK."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Lock bit for CPU_LOCK."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset."]
        #[inline(always)]
        pub const fn gpr(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset."]
        #[inline(always)]
        pub fn set_gpr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
    }
    impl Default for Lock {
        #[inline(always)]
        fn default() -> Lock {
            Lock(0)
        }
    }
    #[doc = "Clock lower limit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LowLimit(pub u32);
    impl LowLimit {
        #[doc = "lower frequency."]
        #[inline(always)]
        pub const fn frequency(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "lower frequency."]
        #[inline(always)]
        pub fn set_frequency(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LowLimit {
        #[inline(always)]
        fn default() -> LowLimit {
            LowLimit(0)
        }
    }
    #[doc = "CPU0 LP control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lp(pub u32);
    impl Lp {
        #[doc = "Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved."]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 reset not happened 1: CPU0 reset happened."]
        #[inline(always)]
        pub const fn reset_flag(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 reset not happened 1: CPU0 reset happened."]
        #[inline(always)]
        pub fn set_reset_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened."]
        #[inline(always)]
        pub const fn sleep_flag(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened."]
        #[inline(always)]
        pub fn set_sleep_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wake up happened."]
        #[inline(always)]
        pub const fn wake_flag(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wake up happened."]
        #[inline(always)]
        pub fn set_wake_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CPU0 is executing 0: CPU0 is not executing 1: CPU0 is executing."]
        #[inline(always)]
        pub const fn exec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CPU0 is executing 0: CPU0 is not executing 1: CPU0 is executing."]
        #[inline(always)]
        pub fn set_exec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CPU0 is waking up 0: CPU0 wake up not asserted 1: CPU0 wake up asserted."]
        #[inline(always)]
        pub const fn wake(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CPU0 is waking up 0: CPU0 wake up not asserted 1: CPU0 wake up asserted."]
        #[inline(always)]
        pub fn set_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI."]
        #[inline(always)]
        pub const fn halt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI."]
        #[inline(always)]
        pub fn set_halt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CPU0 wake up counter, counter satuated at 255, write 0x00 to clear."]
        #[inline(always)]
        pub const fn wake_cnt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "CPU0 wake up counter, counter satuated at 255, write 0x00 to clear."]
        #[inline(always)]
        pub fn set_wake_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Lp {
        #[inline(always)]
        fn default() -> Lp {
            Lp(0)
        }
    }
    #[doc = "Clock measure and monitor control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorControl(pub u32);
    impl MonitorControl {
        #[doc = "clock measurement selection."]
        #[inline(always)]
        pub const fn selection(&self) -> super::vals::MonitorSelection {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::MonitorSelection::from_bits(val as u8)
        }
        #[doc = "clock measurement selection."]
        #[inline(always)]
        pub fn set_selection(&mut self, val: super::vals::MonitorSelection) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "reference clock selection, 0: 32k 1: 24M."]
        #[inline(always)]
        pub const fn reference(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "reference clock selection, 0: 32k 1: 24M."]
        #[inline(always)]
        pub fn set_reference(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz."]
        #[inline(always)]
        pub const fn accuracy(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz."]
        #[inline(always)]
        pub fn set_accuracy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register."]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "start measurement."]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "start measurement."]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "clock frequency lower than lower limit."]
        #[inline(always)]
        pub const fn low(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "clock frequency lower than lower limit."]
        #[inline(always)]
        pub fn set_low(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "clock frequency higher than upper limit."]
        #[inline(always)]
        pub const fn high(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "clock frequency higher than upper limit."]
        #[inline(always)]
        pub fn set_high(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "output divider."]
        #[inline(always)]
        pub const fn div(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "output divider."]
        #[inline(always)]
        pub fn set_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "enable clock output."]
        #[inline(always)]
        pub const fn outen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "enable clock output."]
        #[inline(always)]
        pub fn set_outen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "divider is applying new setting."]
        #[inline(always)]
        pub const fn div_busy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "divider is applying new setting."]
        #[inline(always)]
        pub fn set_div_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "result is ready for read 0: not ready 1: result is ready."]
        #[inline(always)]
        pub const fn valid(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "result is ready for read 0: not ready 1: result is ready."]
        #[inline(always)]
        pub fn set_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MonitorControl {
        #[inline(always)]
        fn default() -> MonitorControl {
            MonitorControl(0)
        }
    }
    #[doc = "Power Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OffWait(pub u32);
    impl OffWait {
        #[doc = "wait time for power switch turn off, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz."]
        #[inline(always)]
        pub const fn wait(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "wait time for power switch turn off, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz."]
        #[inline(always)]
        pub fn set_wait(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for OffWait {
        #[inline(always)]
        fn default() -> OffWait {
            OffWait(0)
        }
    }
    #[doc = "Reset Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetControl(pub u32);
    impl ResetControl {
        #[doc = "perform reset and release imediately 0: reset is released 1 reset is asserted and will release automatically."]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "perform reset and release imediately 0: reset is released 1 reset is asserted and will release automatically."]
        #[inline(always)]
        pub fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold."]
        #[inline(always)]
        pub const fn hold(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold."]
        #[inline(always)]
        pub fn set_hold(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit."]
        #[inline(always)]
        pub const fn flag_wake(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit."]
        #[inline(always)]
        pub fn set_flag_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit."]
        #[inline(always)]
        pub const fn flag(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit."]
        #[inline(always)]
        pub fn set_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ResetControl {
        #[inline(always)]
        fn default() -> ResetControl {
            ResetControl(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Resource(pub u32);
    impl Resource {
        #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved."]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "local busy 0: no change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub const fn loc_busy(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "local busy 0: no change is pending for current node 1: current node is changing status."]
        #[inline(always)]
        pub fn set_loc_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status."]
        #[inline(always)]
        pub const fn glb_busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status."]
        #[inline(always)]
        pub fn set_glb_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Resource {
        #[inline(always)]
        fn default() -> Resource {
            Resource(0)
        }
    }
    #[doc = "Retention Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RetentionClear(pub u32);
    impl RetentionClear {
        #[doc = "retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: no keep."]
        #[inline(always)]
        pub const fn link(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: no keep."]
        #[inline(always)]
        pub fn set_link(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for RetentionClear {
        #[inline(always)]
        fn default() -> RetentionClear {
            RetentionClear(0)
        }
    }
    #[doc = "Retention Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RetentionSet(pub u32);
    impl RetentionSet {
        #[doc = "retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: keep."]
        #[inline(always)]
        pub const fn link(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: keep."]
        #[inline(always)]
        pub fn set_link(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for RetentionSet {
        #[inline(always)]
        fn default() -> RetentionSet {
            RetentionSet(0)
        }
    }
    #[doc = "Retention Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RetentionToggle(pub u32);
    impl RetentionToggle {
        #[doc = "retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: toggle the result that whether the resource is kept on while CPU0 stop before."]
        #[inline(always)]
        pub const fn link(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "retention setting while CPU0 enter stop mode, each bit represents a resource 0: no effect 1: toggle the result that whether the resource is kept on while CPU0 stop before."]
        #[inline(always)]
        pub fn set_link(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for RetentionToggle {
        #[inline(always)]
        fn default() -> RetentionToggle {
            RetentionToggle(0)
        }
    }
    #[doc = "Retention Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RetentionValue(pub u32);
    impl RetentionValue {
        #[doc = "retention setting while CPU0 enter stop mode, each bit represents a resource bit00: soc_mem is kept on while cpu0 stop bit01: soc_ctx is kept on while cpu0 stop bit02: cpu0_mem is kept on while cpu0 stop bit03: cpu0_ctx is kept on while cpu0 stop bit04: con_ctx is kept on while cpu0 stop bit05: vis_mem is kept on while cpu0 stop bit06: vis_ctx is kept on while cpu0 stop bit07: gpu_mem is kept on while cpu0 stop bit08: gpu_ctx is kept on while cpu0 stop bit09: xtal_hold is kept on while cpu0 stop bit10: pll0_hold is kept on while cpu0 stop bit11: pll1_hold is kept on while cpu0 stop bit12: pll2_hold is kept on while cpu0 stop bit13: pll3 is kept on while cpu0 stop bit14: pll4 is kept on while cpu0 stop."]
        #[inline(always)]
        pub const fn link(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "retention setting while CPU0 enter stop mode, each bit represents a resource bit00: soc_mem is kept on while cpu0 stop bit01: soc_ctx is kept on while cpu0 stop bit02: cpu0_mem is kept on while cpu0 stop bit03: cpu0_ctx is kept on while cpu0 stop bit04: con_ctx is kept on while cpu0 stop bit05: vis_mem is kept on while cpu0 stop bit06: vis_ctx is kept on while cpu0 stop bit07: gpu_mem is kept on while cpu0 stop bit08: gpu_ctx is kept on while cpu0 stop bit09: xtal_hold is kept on while cpu0 stop bit10: pll0_hold is kept on while cpu0 stop bit11: pll1_hold is kept on while cpu0 stop bit12: pll2_hold is kept on while cpu0 stop bit13: pll3 is kept on while cpu0 stop bit14: pll4 is kept on while cpu0 stop."]
        #[inline(always)]
        pub fn set_link(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for RetentionValue {
        #[inline(always)]
        fn default() -> RetentionValue {
            RetentionValue(0)
        }
    }
    #[doc = "Power Setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off."]
        #[inline(always)]
        pub const fn lf_ack(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off."]
        #[inline(always)]
        pub fn set_lf_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off."]
        #[inline(always)]
        pub const fn lf_disable(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off."]
        #[inline(always)]
        pub fn set_lf_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit."]
        #[inline(always)]
        pub const fn flag_wake(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit."]
        #[inline(always)]
        pub fn set_flag_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit."]
        #[inline(always)]
        pub const fn flag(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit."]
        #[inline(always)]
        pub fn set_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WakeupEnable(pub u32);
    impl WakeupEnable {
        #[doc = "IRQ wakeup enable."]
        #[inline(always)]
        pub const fn enable(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IRQ wakeup enable."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WakeupEnable {
        #[inline(always)]
        fn default() -> WakeupEnable {
            WakeupEnable(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WakeupStatus(pub u32);
    impl WakeupStatus {
        #[doc = "IRQ values."]
        #[inline(always)]
        pub const fn status(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IRQ values."]
        #[inline(always)]
        pub fn set_status(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WakeupStatus {
        #[inline(always)]
        fn default() -> WakeupStatus {
            WakeupStatus(0)
        }
    }
}
pub mod vals {
    #[doc = "no description available."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ClockMux {
        #[doc = "24MHz"]
        CLK_24M = 0x0,
        #[doc = "Dfaults to 500MHz. Default clock source for CPU0"]
        PLL0CLK0 = 0x01,
        #[doc = "Defaults to 800MHz"]
        PLL1CLK0 = 0x02,
        #[doc = "Defaults to 666MHz"]
        PLL1CLK1 = 0x03,
        #[doc = "Defaults to 600MHz"]
        PLL2CLK0 = 0x04,
        #[doc = "Defaults to 500MHz"]
        PLL2CLK1 = 0x05,
        #[doc = "Defaults to 516.096MHz"]
        PLL3CLK0 = 0x06,
        #[doc = "Defaults to 594MHz"]
        PLL4CLK0 = 0x07,
    }
    impl ClockMux {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ClockMux {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum MonitorSelection {
        CLK_32K = 0x0,
        CLK_IRC24M = 0x01,
        CLK_XTAL24M = 0x02,
        CLK_USB0_PHY = 0x03,
        CLK_USB1_PHY = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        CLK_24M = 0x08,
        PLL0CLK0 = 0x09,
        PLL1CLK0 = 0x0a,
        PLL1CLK1 = 0x0b,
        PLL2CLK0 = 0x0c,
        PLL2CLK1 = 0x0d,
        PLL3CLK0 = 0x0e,
        PLL4CLK0 = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        _RESERVED_20 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        _RESERVED_3f = 0x3f,
        _RESERVED_40 = 0x40,
        _RESERVED_41 = 0x41,
        _RESERVED_42 = 0x42,
        _RESERVED_43 = 0x43,
        _RESERVED_44 = 0x44,
        _RESERVED_45 = 0x45,
        _RESERVED_46 = 0x46,
        _RESERVED_47 = 0x47,
        _RESERVED_48 = 0x48,
        _RESERVED_49 = 0x49,
        _RESERVED_4a = 0x4a,
        _RESERVED_4b = 0x4b,
        _RESERVED_4c = 0x4c,
        _RESERVED_4d = 0x4d,
        _RESERVED_4e = 0x4e,
        _RESERVED_4f = 0x4f,
        _RESERVED_50 = 0x50,
        _RESERVED_51 = 0x51,
        _RESERVED_52 = 0x52,
        _RESERVED_53 = 0x53,
        _RESERVED_54 = 0x54,
        _RESERVED_55 = 0x55,
        _RESERVED_56 = 0x56,
        _RESERVED_57 = 0x57,
        _RESERVED_58 = 0x58,
        _RESERVED_59 = 0x59,
        _RESERVED_5a = 0x5a,
        _RESERVED_5b = 0x5b,
        _RESERVED_5c = 0x5c,
        _RESERVED_5d = 0x5d,
        _RESERVED_5e = 0x5e,
        _RESERVED_5f = 0x5f,
        _RESERVED_60 = 0x60,
        _RESERVED_61 = 0x61,
        _RESERVED_62 = 0x62,
        _RESERVED_63 = 0x63,
        _RESERVED_64 = 0x64,
        _RESERVED_65 = 0x65,
        _RESERVED_66 = 0x66,
        _RESERVED_67 = 0x67,
        _RESERVED_68 = 0x68,
        _RESERVED_69 = 0x69,
        _RESERVED_6a = 0x6a,
        _RESERVED_6b = 0x6b,
        _RESERVED_6c = 0x6c,
        _RESERVED_6d = 0x6d,
        _RESERVED_6e = 0x6e,
        _RESERVED_6f = 0x6f,
        _RESERVED_70 = 0x70,
        _RESERVED_71 = 0x71,
        _RESERVED_72 = 0x72,
        _RESERVED_73 = 0x73,
        _RESERVED_74 = 0x74,
        _RESERVED_75 = 0x75,
        _RESERVED_76 = 0x76,
        _RESERVED_77 = 0x77,
        _RESERVED_78 = 0x78,
        _RESERVED_79 = 0x79,
        _RESERVED_7a = 0x7a,
        _RESERVED_7b = 0x7b,
        _RESERVED_7c = 0x7c,
        _RESERVED_7d = 0x7d,
        _RESERVED_7e = 0x7e,
        _RESERVED_7f = 0x7f,
        CLK_TOP_CPU0 = 0x80,
        CLK_TOP_MCHTMR0 = 0x81,
        CLK_TOP_CPU1 = 0x82,
        CLK_TOP_MCHTMR1 = 0x83,
        CLK_TOP_AXI0 = 0x84,
        CLK_TOP_AXI1 = 0x85,
        CLK_TOP_AXI2 = 0x86,
        CLK_TOP_AHB0 = 0x87,
        CLK_TOP_DRAM = 0x88,
        CLK_TOP_XPI0 = 0x89,
        CLK_TOP_XPI1 = 0x8a,
        CLK_TOP_GPTMR0 = 0x8b,
        CLK_TOP_GPTMR1 = 0x8c,
        CLK_TOP_GPTMR2 = 0x8d,
        CLK_TOP_GPTMR3 = 0x8e,
        CLK_TOP_GPTMR4 = 0x8f,
        CLK_TOP_GPTMR5 = 0x90,
        CLK_TOP_GPTMR6 = 0x91,
        CLK_TOP_GPTMR7 = 0x92,
        CLK_TOP_UART0 = 0x93,
        CLK_TOP_UART1 = 0x94,
        CLK_TOP_UART2 = 0x95,
        CLK_TOP_UART3 = 0x96,
        CLK_TOP_UART4 = 0x97,
        CLK_TOP_UART5 = 0x98,
        CLK_TOP_UART6 = 0x99,
        CLK_TOP_UART7 = 0x9a,
        CLK_TOP_UART8 = 0x9b,
        CLK_TOP_UART9 = 0x9c,
        CLK_TOP_UARTA = 0x9d,
        CLK_TOP_UARTB = 0x9e,
        CLK_TOP_UARTC = 0x9f,
        CLK_TOP_UARTD = 0xa0,
        CLK_TOP_UARTE = 0xa1,
        CLK_TOP_UARTF = 0xa2,
        CLK_TOP_I2C0 = 0xa3,
        CLK_TOP_I2C1 = 0xa4,
        CLK_TOP_I2C2 = 0xa5,
        CLK_TOP_I2C3 = 0xa6,
        CLK_TOP_SPI0 = 0xa7,
        CLK_TOP_SPI1 = 0xa8,
        CLK_TOP_SPI2 = 0xa9,
        CLK_TOP_SPI3 = 0xaa,
        CLK_TOP_CAN0 = 0xab,
        CLK_TOP_CAN1 = 0xac,
        CLK_TOP_CAN2 = 0xad,
        CLK_TOP_CAN3 = 0xae,
        CLK_TOP_PTPC = 0xaf,
        CLK_TOP_ANA0 = 0xb0,
        CLK_TOP_ANA1 = 0xb1,
        CLK_TOP_ANA2 = 0xb2,
        CLK_TOP_AUD0 = 0xb3,
        CLK_TOP_AUD1 = 0xb4,
        CLK_TOP_AUD2 = 0xb5,
        CLK_TOP_DIS0 = 0xb6,
        CLK_TOP_CAM0 = 0xb7,
        CLK_TOP_CAM1 = 0xb8,
        CLK_TOP_ETH0 = 0xb9,
        CLK_TOP_ETH1 = 0xba,
        CLK_TOP_PTP0 = 0xbb,
        CLK_TOP_PTP1 = 0xbc,
        CLK_TOP_REF0 = 0xbd,
        CLK_TOP_REF1 = 0xbe,
        CLK_TOP_NTMR0 = 0xbf,
        CLK_TOP_NTMR1 = 0xc0,
        CLK_TOP_SDXC0 = 0xc1,
        CLK_TOP_SDXC1 = 0xc2,
        _RESERVED_c3 = 0xc3,
        _RESERVED_c4 = 0xc4,
        _RESERVED_c5 = 0xc5,
        _RESERVED_c6 = 0xc6,
        _RESERVED_c7 = 0xc7,
        _RESERVED_c8 = 0xc8,
        _RESERVED_c9 = 0xc9,
        _RESERVED_ca = 0xca,
        _RESERVED_cb = 0xcb,
        _RESERVED_cc = 0xcc,
        _RESERVED_cd = 0xcd,
        _RESERVED_ce = 0xce,
        _RESERVED_cf = 0xcf,
        _RESERVED_d0 = 0xd0,
        _RESERVED_d1 = 0xd1,
        _RESERVED_d2 = 0xd2,
        _RESERVED_d3 = 0xd3,
        _RESERVED_d4 = 0xd4,
        _RESERVED_d5 = 0xd5,
        _RESERVED_d6 = 0xd6,
        _RESERVED_d7 = 0xd7,
        _RESERVED_d8 = 0xd8,
        _RESERVED_d9 = 0xd9,
        _RESERVED_da = 0xda,
        _RESERVED_db = 0xdb,
        _RESERVED_dc = 0xdc,
        _RESERVED_dd = 0xdd,
        _RESERVED_de = 0xde,
        _RESERVED_df = 0xdf,
        _RESERVED_e0 = 0xe0,
        _RESERVED_e1 = 0xe1,
        _RESERVED_e2 = 0xe2,
        _RESERVED_e3 = 0xe3,
        _RESERVED_e4 = 0xe4,
        _RESERVED_e5 = 0xe5,
        _RESERVED_e6 = 0xe6,
        _RESERVED_e7 = 0xe7,
        _RESERVED_e8 = 0xe8,
        _RESERVED_e9 = 0xe9,
        _RESERVED_ea = 0xea,
        _RESERVED_eb = 0xeb,
        _RESERVED_ec = 0xec,
        _RESERVED_ed = 0xed,
        _RESERVED_ee = 0xee,
        _RESERVED_ef = 0xef,
        _RESERVED_f0 = 0xf0,
        _RESERVED_f1 = 0xf1,
        _RESERVED_f2 = 0xf2,
        _RESERVED_f3 = 0xf3,
        _RESERVED_f4 = 0xf4,
        _RESERVED_f5 = 0xf5,
        _RESERVED_f6 = 0xf6,
        _RESERVED_f7 = 0xf7,
        _RESERVED_f8 = 0xf8,
        _RESERVED_f9 = 0xf9,
        _RESERVED_fa = 0xfa,
        _RESERVED_fb = 0xfb,
        _RESERVED_fc = 0xfc,
        _RESERVED_fd = 0xfd,
        _RESERVED_fe = 0xfe,
        _RESERVED_ff = 0xff,
    }
    impl MonitorSelection {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MonitorSelection {
            unsafe { core::mem::transmute(val & 0xff) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
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
}
