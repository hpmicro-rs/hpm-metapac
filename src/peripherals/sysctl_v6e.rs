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
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wakeup_enable(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WakeupEnable, crate::common::RW> {
        assert!(n < 6usize);
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
pub struct Group1 {
    ptr: *mut u8,
}
unsafe impl Send for Group1 {}
unsafe impl Sync for Group1 {}
impl Group1 {
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
    pub const fn value(self) -> crate::common::Reg<regs::Group1Value, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Group setting."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::Group1Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Group setting."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::Group1Clear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Group setting."]
    #[inline(always)]
    pub const fn toggle(self) -> crate::common::Reg<regs::Group1Toggle, crate::common::RW> {
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
    #[doc = "Retention Contol."]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<regs::RetentionValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Retention Contol."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::RetentionSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Retention Contol."]
    #[inline(always)]
    pub const fn clear(self) -> crate::common::Reg<regs::RetentionClear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Retention Contol."]
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
        assert!(n < 228usize);
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
    pub const fn group1(self, n: usize) -> Group1 {
        assert!(n < 4usize);
        unsafe { Group1::from_ptr(self.ptr.add(0x0840usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn affiliate(self, n: usize) -> Affiliate {
        assert!(n < 2usize);
        unsafe { Affiliate::from_ptr(self.ptr.add(0x0900usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn retention(self, n: usize) -> Retention {
        assert!(n < 2usize);
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
        assert!(n < 4usize);
        unsafe { Reset::from_ptr(self.ptr.add(0x1400usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn clock(self, n: usize) -> crate::common::Reg<regs::Clock, crate::common::RW> {
        assert!(n < 73usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1800usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn adcclk(self, n: usize) -> crate::common::Reg<regs::Adcclk, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c00usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn i2sclk(self, n: usize) -> crate::common::Reg<regs::I2sclk, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1c10usize + n * 4usize) as _) }
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
        assert!(n < 2usize);
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
        #[doc = "current mux in clock component 0:osc0_clk0 1:pll0_clk0 2:pll0_clk1 3:pll1_clk0 4:pll1_clk1 5:pll1_clk2 6:pll2_clk0 7:pll2_clk1."]
        #[inline(always)]
        pub const fn mux(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "current mux in clock component 0:osc0_clk0 1:pll0_clk0 2:pll0_clk1 3:pll1_clk0 4:pll1_clk1 5:pll1_clk2 6:pll2_clk0 7:pll2_clk1."]
        #[inline(always)]
        pub fn set_mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
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
    #[doc = "Group setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Group1Clear(pub u32);
    impl Group1Clear {
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
    impl Default for Group1Clear {
        #[inline(always)]
        fn default() -> Group1Clear {
            Group1Clear(0)
        }
    }
    #[doc = "Group setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Group1Set(pub u32);
    impl Group1Set {
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
    impl Default for Group1Set {
        #[inline(always)]
        fn default() -> Group1Set {
            Group1Set(0)
        }
    }
    #[doc = "Group setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Group1Toggle(pub u32);
    impl Group1Toggle {
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
    impl Default for Group1Toggle {
        #[inline(always)]
        fn default() -> Group1Toggle {
            Group1Toggle(0)
        }
    }
    #[doc = "Group setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Group1Value(pub u32);
    impl Group1Value {
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
    impl Default for Group1Value {
        #[inline(always)]
        fn default() -> Group1Value {
            Group1Value(0)
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
        #[doc = "current mux 0: aud clock 0 1: aud clock 1."]
        #[inline(always)]
        pub const fn mux(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "current mux 0: aud clock 0 1: aud clock 1."]
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
        pub const fn selection(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "clock measurement selection."]
        #[inline(always)]
        pub fn set_selection(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "refrence clock selection, 0: 32k 1: 24M."]
        #[inline(always)]
        pub const fn reference(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "refrence clock selection, 0: 32k 1: 24M."]
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
        #[doc = "perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly."]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly."]
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
    #[doc = "Retention Contol."]
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
    #[doc = "Retention Contol."]
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
    #[doc = "Retention Contol."]
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
    #[doc = "Retention Contol."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RetentionValue(pub u32);
    impl RetentionValue {
        #[doc = "retention setting while CPU0 enter stop mode, each bit represents a resource bit00: soc_mem is kept on while cpu0 stop bit01: soc_ctx is kept on while cpu0 stop bit02: cpu0_mem is kept on while cpu0 stop bit03: cpu0_ctx is kept on while cpu0 stop bit04: cpu1_mem is kept on while cpu0 stop bit05: cpu1_ctx is kept on while cpu0 stop bit06: otn_mem is kept on while cpu0 stop bit07: otn_ctx is kept on while cpu0 stop bit08: xtal_hold is kept on while cpu0 stop bit09: pll0_hold is kept on while cpu0 stop bit10: pll1_hold is kept on while cpu0 stop bit11: pll2_hold is kept on while cpu0 stop."]
        #[inline(always)]
        pub const fn link(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "retention setting while CPU0 enter stop mode, each bit represents a resource bit00: soc_mem is kept on while cpu0 stop bit01: soc_ctx is kept on while cpu0 stop bit02: cpu0_mem is kept on while cpu0 stop bit03: cpu0_ctx is kept on while cpu0 stop bit04: cpu1_mem is kept on while cpu0 stop bit05: cpu1_ctx is kept on while cpu0 stop bit06: otn_mem is kept on while cpu0 stop bit07: otn_ctx is kept on while cpu0 stop bit08: xtal_hold is kept on while cpu0 stop bit09: pll0_hold is kept on while cpu0 stop bit10: pll1_hold is kept on while cpu0 stop bit11: pll2_hold is kept on while cpu0 stop."]
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
