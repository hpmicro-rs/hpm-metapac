#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "EWDG0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdg {
    ptr: *mut u8,
}
unsafe impl Send for Wdg {}
unsafe impl Sync for Wdg {}
impl Wdg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "wdog ctrl register 0 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits."]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "wdog ctrl register 1 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits."]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "wdog timeout reset counter value."]
    #[inline(always)]
    pub const fn ot_rst_val(self) -> crate::common::Reg<regs::OtRstVal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "wdog refresh register."]
    #[inline(always)]
    pub const fn wdt_refresh_reg(
        self,
    ) -> crate::common::Reg<regs::WdtRefreshReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "wdog status register."]
    #[inline(always)]
    pub const fn wdt_status(self) -> crate::common::Reg<regs::WdtStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "ctrl register protection register."]
    #[inline(always)]
    pub const fn cfg_prot(self) -> crate::common::Reg<regs::CfgProt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "refresh protection register."]
    #[inline(always)]
    pub const fn ref_prot(self) -> crate::common::Reg<regs::RefProt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Wdog enable."]
    #[inline(always)]
    pub const fn wdt_en(self) -> crate::common::Reg<regs::WdtEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Refresh period value."]
    #[inline(always)]
    pub const fn ref_time(self) -> crate::common::Reg<regs::RefTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
}
pub mod regs {
    #[doc = "ctrl register protection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CfgProt(pub u32);
    impl CfgProt {
        #[doc = "The password of unlocking register update."]
        #[inline(always)]
        pub const fn upd_psd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The password of unlocking register update."]
        #[inline(always)]
        pub fn set_upd_psd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The period in which register update has to be in after unlock The required period is less than： 128 * 2 ^ UPD_OT_TIME * bus_clock_cycle."]
        #[inline(always)]
        pub const fn upd_ot_time(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "The period in which register update has to be in after unlock The required period is less than： 128 * 2 ^ UPD_OT_TIME * bus_clock_cycle."]
        #[inline(always)]
        pub fn set_upd_ot_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for CfgProt {
        #[inline(always)]
        fn default() -> CfgProt {
            CfgProt(0)
        }
    }
    #[doc = "wdog ctrl register 0 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl0(pub u32);
    impl Ctrl0 {
        #[doc = "WDT enable or not in low power mode 2'b00: wdt is halted once in low power mode 2'b01: wdt will work with 1/4 normal clock freq in low power mode 2'b10: wdt will work with 1/2 normal clock freq in low power mode 2'b11: wdt will work with normal clock freq in low power mode."]
        #[inline(always)]
        pub const fn en_lp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "WDT enable or not in low power mode 2'b00: wdt is halted once in low power mode 2'b01: wdt will work with 1/4 normal clock freq in low power mode 2'b10: wdt will work with 1/2 normal clock freq in low power mode 2'b11: wdt will work with normal clock freq in low power mode."]
        #[inline(always)]
        pub fn set_en_lp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "WTD enable or not in debug mode."]
        #[inline(always)]
        pub const fn en_dbg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "WTD enable or not in debug mode."]
        #[inline(always)]
        pub fn set_en_dbg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Unlock refresh mechanism 00: the required unlock password is the same with refresh_psd_register 01: the required unlock password is a ring shift left value of refresh_psd_register 10: the required unlock password is always 16'h55AA, no matter what refresh_psd_register is 11: the required unlock password is a LSFR result of refresh_psd_register, the characteristic polynomial is X^15 + 1."]
        #[inline(always)]
        pub const fn ref_unlock_mec(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Unlock refresh mechanism 00: the required unlock password is the same with refresh_psd_register 01: the required unlock password is a ring shift left value of refresh_psd_register 10: the required unlock password is always 16'h55AA, no matter what refresh_psd_register is 11: the required unlock password is a LSFR result of refresh_psd_register, the characteristic polynomial is X^15 + 1."]
        #[inline(always)]
        pub fn set_ref_unlock_mec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "WDT refresh has to be unlocked firstly once refresh lock is enable."]
        #[inline(always)]
        pub const fn ref_lock(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "WDT refresh has to be unlocked firstly once refresh lock is enable."]
        #[inline(always)]
        pub fn set_ref_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "The upper threshold of window value The window period upper limit is: lower_limit + (overtime_rst_value / 16) * upper_reg_value If this register value is zero, then no upper level limitation."]
        #[inline(always)]
        pub const fn win_upper(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "The upper threshold of window value The window period upper limit is: lower_limit + (overtime_rst_value / 16) * upper_reg_value If this register value is zero, then no upper level limitation."]
        #[inline(always)]
        pub fn set_win_upper(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "If refresh event has to be limited into a period after refresh unlocked. Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter."]
        #[inline(always)]
        pub const fn ref_ot_req(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "If refresh event has to be limited into a period after refresh unlocked. Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter."]
        #[inline(always)]
        pub fn set_ref_ot_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "overtime reset can be self released after 32 function cycles."]
        #[inline(always)]
        pub const fn ot_self_clear(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "overtime reset can be self released after 32 function cycles."]
        #[inline(always)]
        pub fn set_ot_self_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "The register is locked and unlock is needed before re-config registers Once the lock mechanism takes effect, the CTRL0, CTRL1, timeout int register, timeout rst register, needs unlock before re-config them. The register update needs to be finished in the required period defined by UPD_OT_TIME register."]
        #[inline(always)]
        pub const fn cfg_lock(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "The register is locked and unlock is needed before re-config registers Once the lock mechanism takes effect, the CTRL0, CTRL1, timeout int register, timeout rst register, needs unlock before re-config them. The register update needs to be finished in the required period defined by UPD_OT_TIME register."]
        #[inline(always)]
        pub fn set_cfg_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Once window mode is opened, the lower counter value to refresh wdt 00: 4/8 overtime value 01: 5/8 of overtime value 10: 6/8 of overtime value 11: 7/8 of overtime value."]
        #[inline(always)]
        pub const fn win_lower(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Once window mode is opened, the lower counter value to refresh wdt 00: 4/8 overtime value 01: 5/8 of overtime value 10: 6/8 of overtime value 11: 7/8 of overtime value."]
        #[inline(always)]
        pub fn set_win_lower(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "window mode enable."]
        #[inline(always)]
        pub const fn win_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "window mode enable."]
        #[inline(always)]
        pub fn set_win_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "clock divider, the clock divider works as 2 ^ div_value for wdt counter."]
        #[inline(always)]
        pub const fn div_value(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x07;
            val as u8
        }
        #[doc = "clock divider, the clock divider works as 2 ^ div_value for wdt counter."]
        #[inline(always)]
        pub fn set_div_value(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
        }
        #[doc = "clock select 0：bus clock 1：ext clock."]
        #[inline(always)]
        pub const fn clk_sel(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "clock select 0：bus clock 1：ext clock."]
        #[inline(always)]
        pub fn set_clk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ctrl0 {
        #[inline(always)]
        fn default() -> Ctrl0 {
            Ctrl0(0)
        }
    }
    #[doc = "wdog ctrl register 1 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl1(pub u32);
    impl Ctrl1 {
        #[doc = "Parity error will trigger a interrupt."]
        #[inline(always)]
        pub const fn parity_fail_int_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error will trigger a interrupt."]
        #[inline(always)]
        pub fn set_parity_fail_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Parity error will trigger a reset A parity check is required once writing to ctrl0 and ctrl1 register. The result should be zero by modular two addition of all bits."]
        #[inline(always)]
        pub const fn parity_fail_rst_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error will trigger a reset A parity check is required once writing to ctrl0 and ctrl1 register. The result should be zero by modular two addition of all bits."]
        #[inline(always)]
        pub fn set_parity_fail_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Unlock register update failure will trigger a interrupt."]
        #[inline(always)]
        pub const fn unl_ctl_fail_int_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Unlock register update failure will trigger a interrupt."]
        #[inline(always)]
        pub fn set_unl_ctl_fail_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Unlock register update failure will trigger a reset."]
        #[inline(always)]
        pub const fn unl_ctl_fail_rst_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Unlock register update failure will trigger a reset."]
        #[inline(always)]
        pub fn set_unl_ctl_fail_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Ctrl update violation will trigger a interrupt."]
        #[inline(always)]
        pub const fn ctl_vio_int_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Ctrl update violation will trigger a interrupt."]
        #[inline(always)]
        pub fn set_ctl_vio_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Ctrl update violation will trigger a reset The violation event is to try updating the locked register before unlock them."]
        #[inline(always)]
        pub const fn ctl_vio_rst_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Ctrl update violation will trigger a reset The violation event is to try updating the locked register before unlock them."]
        #[inline(always)]
        pub fn set_ctl_vio_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "WDT overtime will generate a reset."]
        #[inline(always)]
        pub const fn ot_rst_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "WDT overtime will generate a reset."]
        #[inline(always)]
        pub fn set_ot_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Refresh unlock fail will trigger a interrupt."]
        #[inline(always)]
        pub const fn unl_ref_fail_int_en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh unlock fail will trigger a interrupt."]
        #[inline(always)]
        pub fn set_unl_ref_fail_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Refresh unlock fail will trigger a reset."]
        #[inline(always)]
        pub const fn unl_ref_fail_rst_en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh unlock fail will trigger a reset."]
        #[inline(always)]
        pub fn set_unl_ref_fail_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Refresh violation will trigger an interrupt."]
        #[inline(always)]
        pub const fn ref_fail_int_en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh violation will trigger an interrupt."]
        #[inline(always)]
        pub fn set_ref_fail_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Refresh violation will trigger an reset. These event will be taken as a refresh violation: 1) Not refresh in the window once window mode is enabled 2) Not unlock refresh firstly if unlock is required 3) Not refresh in the required time after unlock, once refresh unlock overtime is enabled. 4) Not write the required word to refresh wdt."]
        #[inline(always)]
        pub const fn ref_fail_rst_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh violation will trigger an reset. These event will be taken as a refresh violation: 1) Not refresh in the window once window mode is enabled 2) Not unlock refresh firstly if unlock is required 3) Not refresh in the required time after unlock, once refresh unlock overtime is enabled. 4) Not write the required word to refresh wdt."]
        #[inline(always)]
        pub fn set_ref_fail_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ctrl1 {
        #[inline(always)]
        fn default() -> Ctrl1 {
            Ctrl1(0)
        }
    }
    #[doc = "wdog timeout reset counter value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtRstVal(pub u32);
    impl OtRstVal {
        #[doc = "WDT timeout reset value."]
        #[inline(always)]
        pub const fn ot_rst_val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "WDT timeout reset value."]
        #[inline(always)]
        pub fn set_ot_rst_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for OtRstVal {
        #[inline(always)]
        fn default() -> OtRstVal {
            OtRstVal(0)
        }
    }
    #[doc = "refresh protection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RefProt(pub u32);
    impl RefProt {
        #[doc = "The password to unlock refreshing."]
        #[inline(always)]
        pub const fn ref_unl_psd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The password to unlock refreshing."]
        #[inline(always)]
        pub fn set_ref_unl_psd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for RefProt {
        #[inline(always)]
        fn default() -> RefProt {
            RefProt(0)
        }
    }
    #[doc = "Refresh period value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RefTime(pub u32);
    impl RefTime {
        #[doc = "The refresh period after refresh unlocked Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter."]
        #[inline(always)]
        pub const fn refresh_period(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The refresh period after refresh unlocked Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter."]
        #[inline(always)]
        pub fn set_refresh_period(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for RefTime {
        #[inline(always)]
        fn default() -> RefTime {
            RefTime(0)
        }
    }
    #[doc = "Wdog enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdtEn(pub u32);
    impl WdtEn {
        #[doc = "Wdog is enabled, the re-written of this register is impacted by enable lock function."]
        #[inline(always)]
        pub const fn wdog_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wdog is enabled, the re-written of this register is impacted by enable lock function."]
        #[inline(always)]
        pub fn set_wdog_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for WdtEn {
        #[inline(always)]
        fn default() -> WdtEn {
            WdtEn(0)
        }
    }
    #[doc = "wdog refresh register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdtRefreshReg(pub u32);
    impl WdtRefreshReg {
        #[doc = "Write this register by 32'h5A45_524F to refresh wdog Note: Reading this register can read back wdt real time counter value, while it is only used by debug purpose."]
        #[inline(always)]
        pub const fn wdt_refresh_reg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Write this register by 32'h5A45_524F to refresh wdog Note: Reading this register can read back wdt real time counter value, while it is only used by debug purpose."]
        #[inline(always)]
        pub fn set_wdt_refresh_reg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WdtRefreshReg {
        #[inline(always)]
        fn default() -> WdtRefreshReg {
            WdtRefreshReg(0)
        }
    }
    #[doc = "wdog status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdtStatus(pub u32);
    impl WdtStatus {
        #[doc = "Refresh fail Write one to clear the bit."]
        #[inline(always)]
        pub const fn ref_vio(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh fail Write one to clear the bit."]
        #[inline(always)]
        pub fn set_ref_vio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Refresh unlock fail Write one to clear the bit."]
        #[inline(always)]
        pub const fn ref_unl_fail(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh unlock fail Write one to clear the bit."]
        #[inline(always)]
        pub fn set_ref_unl_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Violate register update protection mechanism Write one to clear the bit."]
        #[inline(always)]
        pub const fn ctl_vio(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Violate register update protection mechanism Write one to clear the bit."]
        #[inline(always)]
        pub fn set_ctl_vio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Unlock ctrl reg update protection fail Write one to clear the bit."]
        #[inline(always)]
        pub const fn ctl_unl_fail(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Unlock ctrl reg update protection fail Write one to clear the bit."]
        #[inline(always)]
        pub fn set_ctl_unl_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timeout happens, a reset will happen once enable bit set This bit can be cleared only by refreshing wdt or reset."]
        #[inline(always)]
        pub const fn ot_rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout happens, a reset will happen once enable bit set This bit can be cleared only by refreshing wdt or reset."]
        #[inline(always)]
        pub fn set_ot_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "parity error Write one to clear the bit."]
        #[inline(always)]
        pub const fn parity_error(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "parity error Write one to clear the bit."]
        #[inline(always)]
        pub fn set_parity_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for WdtStatus {
        #[inline(always)]
        fn default() -> WdtStatus {
            WdtStatus(0)
        }
    }
}
