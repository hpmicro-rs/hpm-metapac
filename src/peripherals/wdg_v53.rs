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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "wdog ctrl register 1 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits."]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "wdog timeout interrupt counter value."]
    #[inline(always)]
    pub const fn ot_int_val(self) -> crate::common::Reg<regs::OtIntVal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "wdog timeout reset counter value."]
    #[inline(always)]
    pub const fn ot_rst_val(self) -> crate::common::Reg<regs::OtRstVal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "wdog refresh register."]
    #[inline(always)]
    pub const fn wdt_refresh_reg(
        self,
    ) -> crate::common::Reg<regs::WdtRefreshReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "wdog status register."]
    #[inline(always)]
    pub const fn wdt_status(self) -> crate::common::Reg<regs::WdtStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "ctrl register protection register."]
    #[inline(always)]
    pub const fn cfg_prot(self) -> crate::common::Reg<regs::CfgProt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "refresh protection register."]
    #[inline(always)]
    pub const fn ref_prot(self) -> crate::common::Reg<regs::RefProt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Wdog enable."]
    #[inline(always)]
    pub const fn wdt_en(self) -> crate::common::Reg<regs::WdtEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Refresh period value."]
    #[inline(always)]
    pub const fn ref_time(self) -> crate::common::Reg<regs::RefTime, crate::common::RW> {
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
    #[doc = "ctrl register protection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CfgProt(pub u32);
    impl CfgProt {
        #[doc = "The password of unlocking register update."]
        #[must_use]
        #[inline(always)]
        pub const fn upd_psd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The password of unlocking register update."]
        #[inline(always)]
        pub const fn set_upd_psd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The period in which register update has to be in after unlock The required period is less than： 128 * 2 ^ UPD_OT_TIME * bus_clock_cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn upd_ot_time(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "The period in which register update has to be in after unlock The required period is less than： 128 * 2 ^ UPD_OT_TIME * bus_clock_cycle."]
        #[inline(always)]
        pub const fn set_upd_ot_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for CfgProt {
        #[inline(always)]
        fn default() -> CfgProt {
            CfgProt(0)
        }
    }
    impl core::fmt::Debug for CfgProt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CfgProt")
                .field("upd_psd", &self.upd_psd())
                .field("upd_ot_time", &self.upd_ot_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CfgProt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CfgProt {{ upd_psd: {=u16:?}, upd_ot_time: {=u8:?} }}",
                self.upd_psd(),
                self.upd_ot_time()
            )
        }
    }
    #[doc = "wdog ctrl register 0 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl0(pub u32);
    impl Ctrl0 {
        #[doc = "WDT enable or not in low power mode 2'b00: wdt is halted once in low power mode 2'b01: wdt will work with 1/4 normal clock freq in low power mode 2'b10: wdt will work with 1/2 normal clock freq in low power mode 2'b11: wdt will work with normal clock freq in low power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn en_lp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "WDT enable or not in low power mode 2'b00: wdt is halted once in low power mode 2'b01: wdt will work with 1/4 normal clock freq in low power mode 2'b10: wdt will work with 1/2 normal clock freq in low power mode 2'b11: wdt will work with normal clock freq in low power mode."]
        #[inline(always)]
        pub const fn set_en_lp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "WTD enable or not in debug mode."]
        #[must_use]
        #[inline(always)]
        pub const fn en_dbg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "WTD enable or not in debug mode."]
        #[inline(always)]
        pub const fn set_en_dbg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Unlock refresh mechanism 00: the required unlock password is the same with refresh_psd_register 01: the required unlock password is a ring shift left value of refresh_psd_register 10: the required unlock password is always 16'h55AA, no matter what refresh_psd_register is 11: the required unlock password is a LSFR result of refresh_psd_register, the characteristic polynomial is X^15 + 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ref_unlock_mec(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Unlock refresh mechanism 00: the required unlock password is the same with refresh_psd_register 01: the required unlock password is a ring shift left value of refresh_psd_register 10: the required unlock password is always 16'h55AA, no matter what refresh_psd_register is 11: the required unlock password is a LSFR result of refresh_psd_register, the characteristic polynomial is X^15 + 1."]
        #[inline(always)]
        pub const fn set_ref_unlock_mec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "WDT refresh has to be unlocked firstly once refresh lock is enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ref_lock(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "WDT refresh has to be unlocked firstly once refresh lock is enable."]
        #[inline(always)]
        pub const fn set_ref_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "The upper threshold of window value The window period upper limit is: lower_limit + (overtime_rst_value / 16) * upper_reg_value If this register value is zero, then no upper level limitation."]
        #[must_use]
        #[inline(always)]
        pub const fn win_upper(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "The upper threshold of window value The window period upper limit is: lower_limit + (overtime_rst_value / 16) * upper_reg_value If this register value is zero, then no upper level limitation."]
        #[inline(always)]
        pub const fn set_win_upper(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "If refresh event has to be limited into a period after refresh unlocked. Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter."]
        #[must_use]
        #[inline(always)]
        pub const fn ref_ot_req(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "If refresh event has to be limited into a period after refresh unlocked. Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter."]
        #[inline(always)]
        pub const fn set_ref_ot_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "overtime reset can be self released after 32 function cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn ot_self_clear(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "overtime reset can be self released after 32 function cycles."]
        #[inline(always)]
        pub const fn set_ot_self_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "The register is locked and unlock is needed before re-config registers Once the lock mechanism takes effect, the CTRL0, CTRL1, timeout int register, timeout rst register, needs unlock before re-config them. The register update needs to be finished in the required period defined by UPD_OT_TIME register."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_lock(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "The register is locked and unlock is needed before re-config registers Once the lock mechanism takes effect, the CTRL0, CTRL1, timeout int register, timeout rst register, needs unlock before re-config them. The register update needs to be finished in the required period defined by UPD_OT_TIME register."]
        #[inline(always)]
        pub const fn set_cfg_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Once window mode is opened, the lower counter value to refresh wdt 00: 4/8 overtime value 01: 5/8 of overtime value 10: 6/8 of overtime value 11: 7/8 of overtime value."]
        #[must_use]
        #[inline(always)]
        pub const fn win_lower(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Once window mode is opened, the lower counter value to refresh wdt 00: 4/8 overtime value 01: 5/8 of overtime value 10: 6/8 of overtime value 11: 7/8 of overtime value."]
        #[inline(always)]
        pub const fn set_win_lower(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "window mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn win_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "window mode enable."]
        #[inline(always)]
        pub const fn set_win_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "clock divider, the clock divider works as 2 ^ div_value for wdt counter."]
        #[must_use]
        #[inline(always)]
        pub const fn div_value(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x07;
            val as u8
        }
        #[doc = "clock divider, the clock divider works as 2 ^ div_value for wdt counter."]
        #[inline(always)]
        pub const fn set_div_value(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
        }
        #[doc = "clock select 0：bus clock 1：ext clock."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_sel(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "clock select 0：bus clock 1：ext clock."]
        #[inline(always)]
        pub const fn set_clk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ctrl0 {
        #[inline(always)]
        fn default() -> Ctrl0 {
            Ctrl0(0)
        }
    }
    impl core::fmt::Debug for Ctrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl0")
                .field("en_lp", &self.en_lp())
                .field("en_dbg", &self.en_dbg())
                .field("ref_unlock_mec", &self.ref_unlock_mec())
                .field("ref_lock", &self.ref_lock())
                .field("win_upper", &self.win_upper())
                .field("ref_ot_req", &self.ref_ot_req())
                .field("ot_self_clear", &self.ot_self_clear())
                .field("cfg_lock", &self.cfg_lock())
                .field("win_lower", &self.win_lower())
                .field("win_en", &self.win_en())
                .field("div_value", &self.div_value())
                .field("clk_sel", &self.clk_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl0 {{ en_lp: {=u8:?}, en_dbg: {=bool:?}, ref_unlock_mec: {=u8:?}, ref_lock: {=bool:?}, win_upper: {=u8:?}, ref_ot_req: {=bool:?}, ot_self_clear: {=bool:?}, cfg_lock: {=bool:?}, win_lower: {=u8:?}, win_en: {=bool:?}, div_value: {=u8:?}, clk_sel: {=bool:?} }}" , self . en_lp () , self . en_dbg () , self . ref_unlock_mec () , self . ref_lock () , self . win_upper () , self . ref_ot_req () , self . ot_self_clear () , self . cfg_lock () , self . win_lower () , self . win_en () , self . div_value () , self . clk_sel ())
        }
    }
    #[doc = "wdog ctrl register 1 Note: Parity check is required once writing to this register. The result should be zero by modular two addition of all bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl1(pub u32);
    impl Ctrl1 {
        #[doc = "Parity error will trigger a interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn parity_fail_int_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error will trigger a interrupt."]
        #[inline(always)]
        pub const fn set_parity_fail_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Parity error will trigger a reset A parity check is required once writing to ctrl0 and ctrl1 register. The result should be zero by modular two addition of all bits."]
        #[must_use]
        #[inline(always)]
        pub const fn parity_fail_rst_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error will trigger a reset A parity check is required once writing to ctrl0 and ctrl1 register. The result should be zero by modular two addition of all bits."]
        #[inline(always)]
        pub const fn set_parity_fail_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Unlock register update failure will trigger a interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn unl_ctl_fail_int_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Unlock register update failure will trigger a interrupt."]
        #[inline(always)]
        pub const fn set_unl_ctl_fail_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Unlock register update failure will trigger a reset."]
        #[must_use]
        #[inline(always)]
        pub const fn unl_ctl_fail_rst_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Unlock register update failure will trigger a reset."]
        #[inline(always)]
        pub const fn set_unl_ctl_fail_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Ctrl update violation will trigger a interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ctl_vio_int_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Ctrl update violation will trigger a interrupt."]
        #[inline(always)]
        pub const fn set_ctl_vio_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Ctrl update violation will trigger a reset The violation event is to try updating the locked register before unlock them."]
        #[must_use]
        #[inline(always)]
        pub const fn ctl_vio_rst_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Ctrl update violation will trigger a reset The violation event is to try updating the locked register before unlock them."]
        #[inline(always)]
        pub const fn set_ctl_vio_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "WDT can generate an interrupt warning before timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn ot_int_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "WDT can generate an interrupt warning before timeout."]
        #[inline(always)]
        pub const fn set_ot_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "WDT overtime will generate a reset."]
        #[must_use]
        #[inline(always)]
        pub const fn ot_rst_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "WDT overtime will generate a reset."]
        #[inline(always)]
        pub const fn set_ot_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Refresh unlock fail will trigger a interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn unl_ref_fail_int_en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh unlock fail will trigger a interrupt."]
        #[inline(always)]
        pub const fn set_unl_ref_fail_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Refresh unlock fail will trigger a reset."]
        #[must_use]
        #[inline(always)]
        pub const fn unl_ref_fail_rst_en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh unlock fail will trigger a reset."]
        #[inline(always)]
        pub const fn set_unl_ref_fail_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Refresh violation will trigger an interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ref_fail_int_en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh violation will trigger an interrupt."]
        #[inline(always)]
        pub const fn set_ref_fail_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Refresh violation will trigger an reset. These event will be taken as a refresh violation: 1) Not refresh in the window once window mode is enabled 2) Not unlock refresh firstly if unlock is required 3) Not refresh in the required time after unlock, once refresh unlock overtime is enabled. 4) Not write the required word to refresh wdt."]
        #[must_use]
        #[inline(always)]
        pub const fn ref_fail_rst_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh violation will trigger an reset. These event will be taken as a refresh violation: 1) Not refresh in the window once window mode is enabled 2) Not unlock refresh firstly if unlock is required 3) Not refresh in the required time after unlock, once refresh unlock overtime is enabled. 4) Not write the required word to refresh wdt."]
        #[inline(always)]
        pub const fn set_ref_fail_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ctrl1 {
        #[inline(always)]
        fn default() -> Ctrl1 {
            Ctrl1(0)
        }
    }
    impl core::fmt::Debug for Ctrl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl1")
                .field("parity_fail_int_en", &self.parity_fail_int_en())
                .field("parity_fail_rst_en", &self.parity_fail_rst_en())
                .field("unl_ctl_fail_int_en", &self.unl_ctl_fail_int_en())
                .field("unl_ctl_fail_rst_en", &self.unl_ctl_fail_rst_en())
                .field("ctl_vio_int_en", &self.ctl_vio_int_en())
                .field("ctl_vio_rst_en", &self.ctl_vio_rst_en())
                .field("ot_int_en", &self.ot_int_en())
                .field("ot_rst_en", &self.ot_rst_en())
                .field("unl_ref_fail_int_en", &self.unl_ref_fail_int_en())
                .field("unl_ref_fail_rst_en", &self.unl_ref_fail_rst_en())
                .field("ref_fail_int_en", &self.ref_fail_int_en())
                .field("ref_fail_rst_en", &self.ref_fail_rst_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl1 {{ parity_fail_int_en: {=bool:?}, parity_fail_rst_en: {=bool:?}, unl_ctl_fail_int_en: {=bool:?}, unl_ctl_fail_rst_en: {=bool:?}, ctl_vio_int_en: {=bool:?}, ctl_vio_rst_en: {=bool:?}, ot_int_en: {=bool:?}, ot_rst_en: {=bool:?}, unl_ref_fail_int_en: {=bool:?}, unl_ref_fail_rst_en: {=bool:?}, ref_fail_int_en: {=bool:?}, ref_fail_rst_en: {=bool:?} }}" , self . parity_fail_int_en () , self . parity_fail_rst_en () , self . unl_ctl_fail_int_en () , self . unl_ctl_fail_rst_en () , self . ctl_vio_int_en () , self . ctl_vio_rst_en () , self . ot_int_en () , self . ot_rst_en () , self . unl_ref_fail_int_en () , self . unl_ref_fail_rst_en () , self . ref_fail_int_en () , self . ref_fail_rst_en ())
        }
    }
    #[doc = "wdog timeout interrupt counter value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtIntVal(pub u32);
    impl OtIntVal {
        #[doc = "WDT timeout interrupt value."]
        #[must_use]
        #[inline(always)]
        pub const fn ot_int_val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "WDT timeout interrupt value."]
        #[inline(always)]
        pub const fn set_ot_int_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for OtIntVal {
        #[inline(always)]
        fn default() -> OtIntVal {
            OtIntVal(0)
        }
    }
    impl core::fmt::Debug for OtIntVal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OtIntVal")
                .field("ot_int_val", &self.ot_int_val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OtIntVal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OtIntVal {{ ot_int_val: {=u16:?} }}", self.ot_int_val())
        }
    }
    #[doc = "wdog timeout reset counter value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtRstVal(pub u32);
    impl OtRstVal {
        #[doc = "WDT timeout reset value."]
        #[must_use]
        #[inline(always)]
        pub const fn ot_rst_val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "WDT timeout reset value."]
        #[inline(always)]
        pub const fn set_ot_rst_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for OtRstVal {
        #[inline(always)]
        fn default() -> OtRstVal {
            OtRstVal(0)
        }
    }
    impl core::fmt::Debug for OtRstVal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OtRstVal")
                .field("ot_rst_val", &self.ot_rst_val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OtRstVal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OtRstVal {{ ot_rst_val: {=u16:?} }}", self.ot_rst_val())
        }
    }
    #[doc = "refresh protection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RefProt(pub u32);
    impl RefProt {
        #[doc = "The password to unlock refreshing."]
        #[must_use]
        #[inline(always)]
        pub const fn ref_unl_psd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The password to unlock refreshing."]
        #[inline(always)]
        pub const fn set_ref_unl_psd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for RefProt {
        #[inline(always)]
        fn default() -> RefProt {
            RefProt(0)
        }
    }
    impl core::fmt::Debug for RefProt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RefProt")
                .field("ref_unl_psd", &self.ref_unl_psd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RefProt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RefProt {{ ref_unl_psd: {=u16:?} }}", self.ref_unl_psd())
        }
    }
    #[doc = "Refresh period value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RefTime(pub u32);
    impl RefTime {
        #[doc = "The refresh period after refresh unlocked Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter."]
        #[must_use]
        #[inline(always)]
        pub const fn refresh_period(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The refresh period after refresh unlocked Note: the refresh overtime counter works in bus clock domain, not in wdt function clock domain. The wdt divider doesn't take effect for refresh counter."]
        #[inline(always)]
        pub const fn set_refresh_period(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for RefTime {
        #[inline(always)]
        fn default() -> RefTime {
            RefTime(0)
        }
    }
    impl core::fmt::Debug for RefTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RefTime")
                .field("refresh_period", &self.refresh_period())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RefTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RefTime {{ refresh_period: {=u16:?} }}",
                self.refresh_period()
            )
        }
    }
    #[doc = "Wdog enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdtEn(pub u32);
    impl WdtEn {
        #[doc = "Wdog is enabled, the re-written of this register is impacted by enable lock function."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wdog is enabled, the re-written of this register is impacted by enable lock function."]
        #[inline(always)]
        pub const fn set_wdog_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for WdtEn {
        #[inline(always)]
        fn default() -> WdtEn {
            WdtEn(0)
        }
    }
    impl core::fmt::Debug for WdtEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdtEn")
                .field("wdog_en", &self.wdog_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WdtEn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WdtEn {{ wdog_en: {=bool:?} }}", self.wdog_en())
        }
    }
    #[doc = "wdog refresh register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdtRefreshReg(pub u32);
    impl WdtRefreshReg {
        #[doc = "Write this register by 32'h5A45_524F to refresh wdog Note: Reading this register can read back wdt real time counter value, while it is only used by debug purpose."]
        #[must_use]
        #[inline(always)]
        pub const fn wdt_refresh_reg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Write this register by 32'h5A45_524F to refresh wdog Note: Reading this register can read back wdt real time counter value, while it is only used by debug purpose."]
        #[inline(always)]
        pub const fn set_wdt_refresh_reg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WdtRefreshReg {
        #[inline(always)]
        fn default() -> WdtRefreshReg {
            WdtRefreshReg(0)
        }
    }
    impl core::fmt::Debug for WdtRefreshReg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdtRefreshReg")
                .field("wdt_refresh_reg", &self.wdt_refresh_reg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WdtRefreshReg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "WdtRefreshReg {{ wdt_refresh_reg: {=u32:?} }}",
                self.wdt_refresh_reg()
            )
        }
    }
    #[doc = "wdog status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdtStatus(pub u32);
    impl WdtStatus {
        #[doc = "Refresh fail Write one to clear the bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ref_vio(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh fail Write one to clear the bit."]
        #[inline(always)]
        pub const fn set_ref_vio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Refresh unlock fail Write one to clear the bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ref_unl_fail(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Refresh unlock fail Write one to clear the bit."]
        #[inline(always)]
        pub const fn set_ref_unl_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Violate register update protection mechanism Write one to clear the bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ctl_vio(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Violate register update protection mechanism Write one to clear the bit."]
        #[inline(always)]
        pub const fn set_ctl_vio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Unlock ctrl reg update protection fail Write one to clear the bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ctl_unl_fail(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Unlock ctrl reg update protection fail Write one to clear the bit."]
        #[inline(always)]
        pub const fn set_ctl_unl_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timeout happens, a interrupt will happen once enable bit set This bit can be cleared only by refreshing wdt or reset."]
        #[must_use]
        #[inline(always)]
        pub const fn ot_int(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout happens, a interrupt will happen once enable bit set This bit can be cleared only by refreshing wdt or reset."]
        #[inline(always)]
        pub const fn set_ot_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timeout happens, a reset will happen once enable bit set This bit can be cleared only by refreshing wdt or reset."]
        #[must_use]
        #[inline(always)]
        pub const fn ot_rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout happens, a reset will happen once enable bit set This bit can be cleared only by refreshing wdt or reset."]
        #[inline(always)]
        pub const fn set_ot_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "parity error Write one to clear the bit."]
        #[must_use]
        #[inline(always)]
        pub const fn parity_error(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "parity error Write one to clear the bit."]
        #[inline(always)]
        pub const fn set_parity_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for WdtStatus {
        #[inline(always)]
        fn default() -> WdtStatus {
            WdtStatus(0)
        }
    }
    impl core::fmt::Debug for WdtStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdtStatus")
                .field("ref_vio", &self.ref_vio())
                .field("ref_unl_fail", &self.ref_unl_fail())
                .field("ctl_vio", &self.ctl_vio())
                .field("ctl_unl_fail", &self.ctl_unl_fail())
                .field("ot_int", &self.ot_int())
                .field("ot_rst", &self.ot_rst())
                .field("parity_error", &self.parity_error())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WdtStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "WdtStatus {{ ref_vio: {=bool:?}, ref_unl_fail: {=bool:?}, ctl_vio: {=bool:?}, ctl_unl_fail: {=bool:?}, ot_int: {=bool:?}, ot_rst: {=bool:?}, parity_error: {=bool:?} }}" , self . ref_vio () , self . ref_unl_fail () , self . ctl_vio () , self . ctl_unl_fail () , self . ot_int () , self . ot_rst () , self . parity_error ())
        }
    }
}
