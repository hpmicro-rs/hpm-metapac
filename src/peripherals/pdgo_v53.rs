#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PDGO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdgo {
    ptr: *mut u8,
}
unsafe impl Send for Pdgo {}
unsafe impl Sync for Pdgo {}
impl Pdgo {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "trunoff control."]
    #[inline(always)]
    pub const fn dgo_turnoff(self) -> crate::common::Reg<regs::DgoTurnoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RC32K CLOCK."]
    #[inline(always)]
    pub const fn dgo_rc32k_cfg(self) -> crate::common::Reg<regs::DgoRc32kCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Generic control 0."]
    #[inline(always)]
    pub const fn dgo_gpr00(self) -> crate::common::Reg<regs::DgoGpr00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[doc = "Generic control 1."]
    #[inline(always)]
    pub const fn dgo_gpr01(self) -> crate::common::Reg<regs::DgoGpr01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
    }
    #[doc = "Generic control 2."]
    #[inline(always)]
    pub const fn dgo_gpr02(self) -> crate::common::Reg<regs::DgoGpr02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
    }
    #[doc = "Generic control 3."]
    #[inline(always)]
    pub const fn dgo_gpr03(self) -> crate::common::Reg<regs::DgoGpr03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060cusize) as _) }
    }
    #[doc = "control register 0."]
    #[inline(always)]
    pub const fn dgo_ctr0(self) -> crate::common::Reg<regs::DgoCtr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize) as _) }
    }
    #[doc = "control register 1."]
    #[inline(always)]
    pub const fn dgo_ctr1(self) -> crate::common::Reg<regs::DgoCtr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0704usize) as _) }
    }
    #[doc = "control register 2."]
    #[inline(always)]
    pub const fn dgo_ctr2(self) -> crate::common::Reg<regs::DgoCtr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0708usize) as _) }
    }
    #[doc = "control register 3."]
    #[inline(always)]
    pub const fn dgo_ctr3(self) -> crate::common::Reg<regs::DgoCtr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x070cusize) as _) }
    }
    #[doc = "control register 4."]
    #[inline(always)]
    pub const fn dgo_ctr4(self) -> crate::common::Reg<regs::DgoCtr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0710usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoCtr0(pub u32);
    impl DgoCtr0 {
        #[doc = "dgo register status retenion."]
        #[inline(always)]
        pub const fn retention(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "dgo register status retenion."]
        #[inline(always)]
        pub fn set_retention(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for DgoCtr0 {
        #[inline(always)]
        fn default() -> DgoCtr0 {
            DgoCtr0(0)
        }
    }
    #[doc = "control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoCtr1(pub u32);
    impl DgoCtr1 {
        #[doc = "wakeup pin status."]
        #[inline(always)]
        pub const fn pin_wakeup_status(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup pin status."]
        #[inline(always)]
        pub fn set_pin_wakeup_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "permit wakeup pin or software wakeup."]
        #[inline(always)]
        pub const fn wakeup_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "permit wakeup pin or software wakeup."]
        #[inline(always)]
        pub fn set_wakeup_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "software wakeup： 0 : wakeup once； 1：auto wakeup Continuously."]
        #[inline(always)]
        pub const fn aoto_sys_wakeup(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "software wakeup： 0 : wakeup once； 1：auto wakeup Continuously."]
        #[inline(always)]
        pub fn set_aoto_sys_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DgoCtr1 {
        #[inline(always)]
        fn default() -> DgoCtr1 {
            DgoCtr1(0)
        }
    }
    #[doc = "control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoCtr2(pub u32);
    impl DgoCtr2 {
        #[doc = "wakeup pin pull down disable."]
        #[inline(always)]
        pub const fn wakeup_pulldn_disable(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup pin pull down disable."]
        #[inline(always)]
        pub fn set_wakeup_pulldn_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "resetn pin pull up disable."]
        #[inline(always)]
        pub const fn resetn_pullup_disable(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "resetn pin pull up disable."]
        #[inline(always)]
        pub fn set_resetn_pullup_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for DgoCtr2 {
        #[inline(always)]
        fn default() -> DgoCtr2 {
            DgoCtr2(0)
        }
    }
    #[doc = "control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoCtr3(pub u32);
    impl DgoCtr3 {
        #[doc = "software wakeup counter."]
        #[inline(always)]
        pub const fn wakeup_counter(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "software wakeup counter."]
        #[inline(always)]
        pub fn set_wakeup_counter(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DgoCtr3 {
        #[inline(always)]
        fn default() -> DgoCtr3 {
            DgoCtr3(0)
        }
    }
    #[doc = "control register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoCtr4(pub u32);
    impl DgoCtr4 {
        #[doc = "Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode."]
        #[inline(always)]
        pub const fn bandgap_lp_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode."]
        #[inline(always)]
        pub fn set_bandgap_lp_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode."]
        #[inline(always)]
        pub const fn bandgap_less_power(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode."]
        #[inline(always)]
        pub fn set_bandgap_less_power(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for DgoCtr4 {
        #[inline(always)]
        fn default() -> DgoCtr4 {
            DgoCtr4(0)
        }
    }
    #[doc = "Generic control 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoGpr00(pub u32);
    impl DgoGpr00 {
        #[doc = "Generic control."]
        #[inline(always)]
        pub const fn gpr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Generic control."]
        #[inline(always)]
        pub fn set_gpr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DgoGpr00 {
        #[inline(always)]
        fn default() -> DgoGpr00 {
            DgoGpr00(0)
        }
    }
    #[doc = "Generic control 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoGpr01(pub u32);
    impl DgoGpr01 {
        #[doc = "Generic control."]
        #[inline(always)]
        pub const fn gpr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Generic control."]
        #[inline(always)]
        pub fn set_gpr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DgoGpr01 {
        #[inline(always)]
        fn default() -> DgoGpr01 {
            DgoGpr01(0)
        }
    }
    #[doc = "Generic control 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoGpr02(pub u32);
    impl DgoGpr02 {
        #[doc = "Generic control."]
        #[inline(always)]
        pub const fn gpr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Generic control."]
        #[inline(always)]
        pub fn set_gpr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DgoGpr02 {
        #[inline(always)]
        fn default() -> DgoGpr02 {
            DgoGpr02(0)
        }
    }
    #[doc = "Generic control 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoGpr03(pub u32);
    impl DgoGpr03 {
        #[doc = "Generic control."]
        #[inline(always)]
        pub const fn gpr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Generic control."]
        #[inline(always)]
        pub fn set_gpr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DgoGpr03 {
        #[inline(always)]
        fn default() -> DgoGpr03 {
            DgoGpr03(0)
        }
    }
    #[doc = "RC32K CLOCK."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoRc32kCfg(pub u32);
    impl DgoRc32kCfg {
        #[doc = "capacitor trim bits."]
        #[inline(always)]
        pub const fn cap_trim(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "capacitor trim bits."]
        #[inline(always)]
        pub fn set_cap_trim(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "IRC32K bit 6."]
        #[inline(always)]
        pub const fn capex6_trim(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "IRC32K bit 6."]
        #[inline(always)]
        pub fn set_capex6_trim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "IRC32K bit 7."]
        #[inline(always)]
        pub const fn capex7_trim(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "IRC32K bit 7."]
        #[inline(always)]
        pub fn set_capex7_trim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed."]
        #[inline(always)]
        pub const fn irc_trimmed(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed."]
        #[inline(always)]
        pub fn set_irc_trimmed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DgoRc32kCfg {
        #[inline(always)]
        fn default() -> DgoRc32kCfg {
            DgoRc32kCfg(0)
        }
    }
    #[doc = "trunoff control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DgoTurnoff(pub u32);
    impl DgoTurnoff {
        #[doc = "trunoff counter, counter stops when it counts down to 0, the trunoff occurs when the counter value is 1."]
        #[inline(always)]
        pub const fn counter(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "trunoff counter, counter stops when it counts down to 0, the trunoff occurs when the counter value is 1."]
        #[inline(always)]
        pub fn set_counter(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DgoTurnoff {
        #[inline(always)]
        fn default() -> DgoTurnoff {
            DgoTurnoff(0)
        }
    }
}
