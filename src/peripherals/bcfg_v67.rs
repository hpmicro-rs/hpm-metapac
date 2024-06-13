#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BCFG."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcfg {
    ptr: *mut u8,
}
unsafe impl Send for Bcfg {}
unsafe impl Sync for Bcfg {}
impl Bcfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Bandgap config."]
    #[inline(always)]
    pub const fn vbg_cfg(self) -> crate::common::Reg<regs::VbgCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "LDO config."]
    #[inline(always)]
    pub const fn ldo_cfg(self) -> crate::common::Reg<regs::LdoCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "On-chip 32k oscillator config."]
    #[inline(always)]
    pub const fn irc32k_cfg(self) -> crate::common::Reg<regs::Irc32kCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "XTAL 32K config."]
    #[inline(always)]
    pub const fn xtal32k_cfg(self) -> crate::common::Reg<regs::Xtal32kCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Clock config."]
    #[inline(always)]
    pub const fn clk_cfg(self) -> crate::common::Reg<regs::ClkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "Clock config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClkCfg(pub u32);
    impl ClkCfg {
        #[doc = "force switch to crystal."]
        #[inline(always)]
        pub const fn force_xtal(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "force switch to crystal."]
        #[inline(always)]
        pub fn set_force_xtal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "force irc32k run."]
        #[inline(always)]
        pub const fn keep_irc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "force irc32k run."]
        #[inline(always)]
        pub fn set_keep_irc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "crystal selected."]
        #[inline(always)]
        pub const fn xtal_sel(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "crystal selected."]
        #[inline(always)]
        pub fn set_xtal_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for ClkCfg {
        #[inline(always)]
        fn default() -> ClkCfg {
            ClkCfg(0)
        }
    }
    #[doc = "On-chip 32k oscillator config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irc32kCfg(pub u32);
    impl Irc32kCfg {
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
    impl Default for Irc32kCfg {
        #[inline(always)]
        fn default() -> Irc32kCfg {
            Irc32kCfg(0)
        }
    }
    #[doc = "LDO config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LdoCfg(pub u32);
    impl LdoCfg {
        #[doc = "LDO voltage setting in mV, valid range through 600mV to 1100mV, step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1100mV. 600: 600mV 620: 620mV . . . 1100:1100mV."]
        #[inline(always)]
        pub const fn volt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "LDO voltage setting in mV, valid range through 600mV to 1100mV, step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1100mV. 600: 600mV 620: 620mV . . . 1100:1100mV."]
        #[inline(always)]
        pub fn set_volt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "LDO enable 0: LDO is disabled 1: LDO is enabled."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LDO enable 0: LDO is disabled 1: LDO is enabled."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "disable pull down resistor, enable pull down may lead to more power but better response 0: pulldown resistor enabled 1: pulldown resistor disabled."]
        #[inline(always)]
        pub const fn dis_pd(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "disable pull down resistor, enable pull down may lead to more power but better response 0: pulldown resistor enabled 1: pulldown resistor disabled."]
        #[inline(always)]
        pub fn set_dis_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "enable selfload, this bit helps improve LDO performance when current less than 200nA 0: self load disabled 1: selfload enabled."]
        #[inline(always)]
        pub const fn en_sl(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "enable selfload, this bit helps improve LDO performance when current less than 200nA 0: self load disabled 1: selfload enabled."]
        #[inline(always)]
        pub fn set_en_sl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Capacitor trim."]
        #[inline(always)]
        pub const fn cp_trim(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Capacitor trim."]
        #[inline(always)]
        pub fn set_cp_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Resistor trim."]
        #[inline(always)]
        pub const fn res_trim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Resistor trim."]
        #[inline(always)]
        pub fn set_res_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for LdoCfg {
        #[inline(always)]
        fn default() -> LdoCfg {
            LdoCfg(0)
        }
    }
    #[doc = "Bandgap config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VbgCfg(pub u32);
    impl VbgCfg {
        #[doc = "Bandgap 0.50V output trim."]
        #[inline(always)]
        pub const fn vbg_p50(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Bandgap 0.50V output trim."]
        #[inline(always)]
        pub fn set_vbg_p50(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Bandgap 0.65V output trim."]
        #[inline(always)]
        pub const fn vbg_p65(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Bandgap 0.65V output trim."]
        #[inline(always)]
        pub fn set_vbg_p65(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Bandgap 1.0V output trim."]
        #[inline(always)]
        pub const fn vbg_1p0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Bandgap 1.0V output trim."]
        #[inline(always)]
        pub fn set_vbg_1p0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode."]
        #[inline(always)]
        pub const fn power_save(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode."]
        #[inline(always)]
        pub fn set_power_save(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Bandgap works in low power mode 0: not in low power mode 1: bandgap work in low power mode."]
        #[inline(always)]
        pub const fn lp_mode(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Bandgap works in low power mode 0: not in low power mode 1: bandgap work in low power mode."]
        #[inline(always)]
        pub fn set_lp_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed."]
        #[inline(always)]
        pub const fn vbg_trimmed(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed."]
        #[inline(always)]
        pub fn set_vbg_trimmed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for VbgCfg {
        #[inline(always)]
        fn default() -> VbgCfg {
            VbgCfg(0)
        }
    }
    #[doc = "XTAL 32K config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xtal32kCfg(pub u32);
    impl Xtal32kCfg {
        #[doc = "crystal 32k amplifier."]
        #[inline(always)]
        pub const fn amp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "crystal 32k amplifier."]
        #[inline(always)]
        pub fn set_amp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "crystal 32k config."]
        #[inline(always)]
        pub const fn cfg(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "crystal 32k config."]
        #[inline(always)]
        pub fn set_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "crystal 32k gm selection."]
        #[inline(always)]
        pub const fn gmsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "crystal 32k gm selection."]
        #[inline(always)]
        pub fn set_gmsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "crystal 32k hysteres enable."]
        #[inline(always)]
        pub const fn hyst_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "crystal 32k hysteres enable."]
        #[inline(always)]
        pub fn set_hyst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Xtal32kCfg {
        #[inline(always)]
        fn default() -> Xtal32kCfg {
            Xtal32kCfg(0)
        }
    }
}
