#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cal {
    ptr: *mut u8,
}
unsafe impl Send for Cal {}
unsafe impl Sync for Cal {}
impl Cal {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg0(self) -> crate::common::Reg<regs::CalCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::CalCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp {
    ptr: *mut u8,
}
unsafe impl Send for Cmp {}
unsafe impl Sync for Cmp {}
impl Cmp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt {
    ptr: *mut u8,
}
unsafe impl Send for Cnt {}
unsafe impl Sync for Cnt {}
impl Cnt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg0(self) -> crate::common::Reg<regs::CntCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::CntCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg2(self) -> crate::common::Reg<regs::Cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg3(self) -> crate::common::Reg<regs::Cfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm {
    ptr: *mut u8,
}
unsafe impl Send for Pwm {}
unsafe impl Sync for Pwm {}
impl Pwm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg0(self) -> crate::common::Reg<regs::PwmCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::PwmCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn dead_area(self) -> crate::common::Reg<regs::DeadArea, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[doc = "PWM0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwmv2 {
    ptr: *mut u8,
}
unsafe impl Send for Pwmv2 {}
unsafe impl Sync for Pwmv2 {}
impl Pwmv2 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn work_ctrl0(self) -> crate::common::Reg<regs::WorkCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn unlock(self) -> crate::common::Reg<regs::Unlock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn shadow_val(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ShadowVal, crate::common::RW> {
        assert!(n < 28usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn force_mode(self) -> crate::common::Reg<regs::ForceMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn work_ctrl1(self) -> crate::common::Reg<regs::WorkCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pwm(self, n: usize) -> Pwm {
        assert!(n < 8usize);
        unsafe { Pwm::from_ptr(self.ptr.add(0x0100usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn trigger_cfg(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::TriggerCfg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize + n * 4usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn glb_ctrl(self) -> crate::common::Reg<regs::GlbCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn glb_ctrl2(self) -> crate::common::Reg<regs::GlbCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cnt_reload_work(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::CntReloadWork, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmp_val_work(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::CmpValWork, crate::common::RW> {
        assert!(n < 24usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize + n * 4usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn force_work(self) -> crate::common::Reg<regs::ForceWork, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x027cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cnt_val(self, n: usize) -> crate::common::Reg<regs::CntVal, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn dac_value_sv(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::DacValueSv, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b0usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn capture_pos(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::CapturePos, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn capture_neg(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::CaptureNeg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize + n * 4usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_sts(self) -> crate::common::Reg<regs::IrqSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_en(self) -> crate::common::Reg<regs::IrqEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_sts_cmp(self) -> crate::common::Reg<regs::IrqStsCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_sts_reload(self) -> crate::common::Reg<regs::IrqStsReload, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_sts_cap_pos(
        self,
    ) -> crate::common::Reg<regs::IrqStsCapPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_sts_cap_neg(
        self,
    ) -> crate::common::Reg<regs::IrqStsCapNeg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_sts_fault(self) -> crate::common::Reg<regs::IrqStsFault, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_sts_burstend(
        self,
    ) -> crate::common::Reg<regs::IrqStsBurstend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0424usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_en_cmp(self) -> crate::common::Reg<regs::IrqEnCmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0430usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_en_reload(self) -> crate::common::Reg<regs::IrqEnReload, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0434usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_en_cap_pos(self) -> crate::common::Reg<regs::IrqEnCapPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0438usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_en_cap_neg(self) -> crate::common::Reg<regs::IrqEnCapNeg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x043cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_en_fault(self) -> crate::common::Reg<regs::IrqEnFault, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn irq_en_burstend(
        self,
    ) -> crate::common::Reg<regs::IrqEnBurstend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0444usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn dma_en(self) -> crate::common::Reg<regs::DmaEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cnt(self, n: usize) -> Cnt {
        assert!(n < 4usize);
        unsafe { Cnt::from_ptr(self.ptr.add(0x0500usize + n * 16usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cnt_glbcfg(self) -> crate::common::Reg<regs::CntGlbcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cal(self, n: usize) -> Cal {
        assert!(n < 16usize);
        unsafe { Cal::from_ptr(self.ptr.add(0x0600usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmp(self, n: usize) -> Cmp {
        assert!(n < 24usize);
        unsafe { Cmp::from_ptr(self.ptr.add(0x0800usize + n * 16usize) as _) }
    }
}
pub mod regs {
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CalCfg0(pub u32);
    impl CalCfg0 {
        #[doc = "dac/counter value parameter."]
        #[inline(always)]
        pub const fn cal_d_param(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "dac/counter value parameter."]
        #[inline(always)]
        pub fn set_cal_d_param(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "period parameter."]
        #[inline(always)]
        pub const fn cal_t_param(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "period parameter."]
        #[inline(always)]
        pub fn set_cal_t_param(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "low limit parameter."]
        #[inline(always)]
        pub const fn cal_ll_param(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "low limit parameter."]
        #[inline(always)]
        pub fn set_cal_ll_param(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "up limit parameter."]
        #[inline(always)]
        pub const fn cal_lu_param(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "up limit parameter."]
        #[inline(always)]
        pub fn set_cal_lu_param(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for CalCfg0 {
        #[inline(always)]
        fn default() -> CalCfg0 {
            CalCfg0(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CalCfg1(pub u32);
    impl CalCfg1 {
        #[doc = "offset for calculation unit, select from one of the shadow_val."]
        #[inline(always)]
        pub const fn cal_in_off(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "offset for calculation unit, select from one of the shadow_val."]
        #[inline(always)]
        pub fn set_cal_in_off(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "low limit offset selection, select from one of the shadow_val."]
        #[inline(always)]
        pub const fn cal_lim_lo(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "low limit offset selection, select from one of the shadow_val."]
        #[inline(always)]
        pub fn set_cal_lim_lo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "set to enable low limit."]
        #[inline(always)]
        pub const fn cal_ll_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable low limit."]
        #[inline(always)]
        pub fn set_cal_ll_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "up limit offset selection, select from one of the shadow_val."]
        #[inline(always)]
        pub const fn cal_lim_up(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "up limit offset selection, select from one of the shadow_val."]
        #[inline(always)]
        pub fn set_cal_lim_up(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "set to enable up limit."]
        #[inline(always)]
        pub const fn cal_lu_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable up limit."]
        #[inline(always)]
        pub fn set_cal_lu_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "0~3 to select one of the dac input value; 4~7 to select one of the current counter value."]
        #[inline(always)]
        pub const fn cal_in_index(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "0~3 to select one of the dac input value; 4~7 to select one of the current counter value."]
        #[inline(always)]
        pub fn set_cal_in_index(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "select one of 4 counter reload time."]
        #[inline(always)]
        pub const fn cal_t_index(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "select one of 4 counter reload time."]
        #[inline(always)]
        pub fn set_cal_t_index(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for CalCfg1 {
        #[inline(always)]
        fn default() -> CalCfg1 {
            CalCfg1(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CaptureNeg(pub u32);
    impl CaptureNeg {
        #[doc = "counter value captured at input negedge."]
        #[inline(always)]
        pub const fn capture_neg(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "counter value captured at input negedge."]
        #[inline(always)]
        pub fn set_capture_neg(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for CaptureNeg {
        #[inline(always)]
        fn default() -> CaptureNeg {
            CaptureNeg(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CapturePos(pub u32);
    impl CapturePos {
        #[doc = "related counter."]
        #[inline(always)]
        pub const fn cnt_index(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "related counter."]
        #[inline(always)]
        pub fn set_cnt_index(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "0: result from CAP\\[ 7:0\\], from trgm 1: result from CAP\\[15:8\\], from gpio."]
        #[inline(always)]
        pub const fn capture_selgpio(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "0: result from CAP\\[ 7:0\\], from trgm 1: result from CAP\\[15:8\\], from gpio."]
        #[inline(always)]
        pub fn set_capture_selgpio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "related counter value captured at input negedge."]
        #[inline(always)]
        pub const fn capture_pos(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "related counter value captured at input negedge."]
        #[inline(always)]
        pub fn set_capture_pos(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for CapturePos {
        #[inline(always)]
        fn default() -> CapturePos {
            CapturePos(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "select one from 4 counters, only for N>=16. for N<16, this field is0, every 4 compare point related to one counter(0123 for counter0, 4567 for counter1….)."]
        #[inline(always)]
        pub const fn cmp_cnt(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "select one from 4 counters, only for N>=16. for N<16, this field is0, every 4 compare point related to one counter(0123 for counter0, 4567 for counter1….)."]
        #[inline(always)]
        pub fn set_cmp_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "0x00~0x1B select one of the shadow_val directly 0x20~0x2F select one of the calculation cell output 0x30~0x37 select one of capture_pos value(low 8bit are 0) 0x38+k select T/4 0x3E select 0xFFFFF000 0x3F select 0xFFFFFF00 others select 0."]
        #[inline(always)]
        pub const fn cmp_in_sel(&self) -> super::vals::CmpSource {
            let val = (self.0 >> 16usize) & 0x3f;
            super::vals::CmpSource::from_bits(val as u8)
        }
        #[doc = "0x00~0x1B select one of the shadow_val directly 0x20~0x2F select one of the calculation cell output 0x30~0x37 select one of capture_pos value(low 8bit are 0) 0x38+k select T/4 0x3E select 0xFFFFF000 0x3F select 0xFFFFFF00 others select 0."]
        #[inline(always)]
        pub fn set_cmp_in_sel(&mut self, val: super::vals::CmpSource) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
        }
        #[doc = "define when to use the shadow register value for working register(trig_cmp) 000: software set work_ctrl1.shadow_lock bit 001: update immediately(at next cycle) 010: related counter reload time 011: use cmp_update_trigger(from trig_mux, selected by cmp_trig_sel) 100: use the related counter rld_cmp_sel0 to select one compare point 101: use the related counter rld_cmp_sel1, to select one compare point 11x: reserved, no update."]
        #[inline(always)]
        pub const fn cmp_update_time(&self) -> super::vals::CmpShadowUpdateTrigger {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::CmpShadowUpdateTrigger::from_bits(val as u8)
        }
        #[doc = "define when to use the shadow register value for working register(trig_cmp) 000: software set work_ctrl1.shadow_lock bit 001: update immediately(at next cycle) 010: related counter reload time 011: use cmp_update_trigger(from trig_mux, selected by cmp_trig_sel) 100: use the related counter rld_cmp_sel0 to select one compare point 101: use the related counter rld_cmp_sel1, to select one compare point 11x: reserved, no update."]
        #[inline(always)]
        pub fn set_cmp_update_time(&mut self, val: super::vals::CmpShadowUpdateTrigger) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux."]
        #[inline(always)]
        pub const fn cmp_trig_sel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux."]
        #[inline(always)]
        pub fn set_cmp_trig_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg2(pub u32);
    impl Cfg2 {
        #[doc = "change counter value to one of the calculation cell output when cnt_update_triger0 issued."]
        #[inline(always)]
        pub const fn cnt_trig0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "change counter value to one of the calculation cell output when cnt_update_triger0 issued."]
        #[inline(always)]
        pub fn set_cnt_trig0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "set to enable using trig0 to load calculation cell output to counter."]
        #[inline(always)]
        pub const fn cnt_update_en0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable using trig0 to load calculation cell output to counter."]
        #[inline(always)]
        pub fn set_cnt_update_en0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux."]
        #[inline(always)]
        pub const fn cnt_update_trig0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux."]
        #[inline(always)]
        pub fn set_cnt_update_trig0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "change counter value to one of the calculation cell output when cnt_update_triger1 issued."]
        #[inline(always)]
        pub const fn cnt_trig1(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "change counter value to one of the calculation cell output when cnt_update_triger1 issued."]
        #[inline(always)]
        pub fn set_cnt_trig1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "set to enable using trig1 to load calculation cell output to counter."]
        #[inline(always)]
        pub const fn cnt_update_en1(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable using trig1 to load calculation cell output to counter."]
        #[inline(always)]
        pub fn set_cnt_update_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux."]
        #[inline(always)]
        pub const fn cnt_update_trig1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux."]
        #[inline(always)]
        pub fn set_cnt_update_trig1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux."]
        #[inline(always)]
        pub const fn cnt_reload_trig(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux."]
        #[inline(always)]
        pub fn set_cnt_reload_trig(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "set to use input signal(selected by cnt_reload_trig) to reload timer."]
        #[inline(always)]
        pub const fn cnt_reload_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "set to use input signal(selected by cnt_reload_trig) to reload timer."]
        #[inline(always)]
        pub fn set_cnt_reload_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg2 {
        #[inline(always)]
        fn default() -> Cfg2 {
            Cfg2(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg3(pub u32);
    impl Cfg3 {
        #[doc = "output pwm wave for configured burst(timer period), 0 for one burst; 1 for two burst. set to 0xFFFF for always output pwm wave bit's only used when setting cnt_sw_start or trigger selected by cnt_start_sel."]
        #[inline(always)]
        pub const fn cnt_burst(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "output pwm wave for configured burst(timer period), 0 for one burst; 1 for two burst. set to 0xFFFF for always output pwm wave bit's only used when setting cnt_sw_start or trigger selected by cnt_start_sel."]
        #[inline(always)]
        pub fn set_cnt_burst(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "enable use trigger to start pwm output(at next reload point), by cnt_start_sel."]
        #[inline(always)]
        pub const fn cnt_hw_start_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "enable use trigger to start pwm output(at next reload point), by cnt_start_sel."]
        #[inline(always)]
        pub fn set_cnt_hw_start_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux."]
        #[inline(always)]
        pub const fn cnt_start_sel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux."]
        #[inline(always)]
        pub fn set_cnt_start_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for Cfg3 {
        #[inline(always)]
        fn default() -> Cfg3 {
            Cfg3(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmpValWork(pub u32);
    impl CmpValWork {
        #[doc = "compare point working register."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "compare point working register."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CmpValWork {
        #[inline(always)]
        fn default() -> CmpValWork {
            CmpValWork(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CntCfg0(pub u32);
    impl CntCfg0 {
        #[doc = "input dac data parameter."]
        #[inline(always)]
        pub const fn cnt_d_param(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "input dac data parameter."]
        #[inline(always)]
        pub fn set_cnt_d_param(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "define when to use the calculation output value as reload time 00: software set work_ctrl1.shadow_lock bit 01: use compare point selected by rld_cmp_sel0 or rld_cmp_sel1 10: counter reload time 11: use rld_trig_sel to select one of the input trigger NOTE: 00 is not recommended since the update time is not controllable, may cause error in complex application."]
        #[inline(always)]
        pub const fn rld_update_time(&self) -> super::vals::ReloadUpdateTrigger {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::ReloadUpdateTrigger::from_bits(val as u8)
        }
        #[doc = "define when to use the calculation output value as reload time 00: software set work_ctrl1.shadow_lock bit 01: use compare point selected by rld_cmp_sel0 or rld_cmp_sel1 10: counter reload time 11: use rld_trig_sel to select one of the input trigger NOTE: 00 is not recommended since the update time is not controllable, may cause error in complex application."]
        #[inline(always)]
        pub fn set_rld_update_time(&mut self, val: super::vals::ReloadUpdateTrigger) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux."]
        #[inline(always)]
        pub const fn rld_trig_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux."]
        #[inline(always)]
        pub fn set_rld_trig_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "select one compare point from 24, set to 0x1F to disable current selection."]
        #[inline(always)]
        pub const fn rld_cmp_sel0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "select one compare point from 24, set to 0x1F to disable current selection."]
        #[inline(always)]
        pub fn set_rld_cmp_sel0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "select one compare point from 24, set to 0x1F to disable current selection, used for reload value, compare value, force value update."]
        #[inline(always)]
        pub const fn rld_cmp_sel1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "select one compare point from 24, set to 0x1F to disable current selection, used for reload value, compare value, force value update."]
        #[inline(always)]
        pub fn set_rld_cmp_sel1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for CntCfg0 {
        #[inline(always)]
        fn default() -> CntCfg0 {
            CntCfg0(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CntCfg1(pub u32);
    impl CntCfg1 {
        #[doc = "input data offset selection, from one of the shadow_val, default just shadow reload time."]
        #[inline(always)]
        pub const fn cnt_in_off(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "input data offset selection, from one of the shadow_val, default just shadow reload time."]
        #[inline(always)]
        pub fn set_cnt_in_off(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "low limit offset selection, from one of the shadow_val."]
        #[inline(always)]
        pub const fn cnt_lim_lo(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "low limit offset selection, from one of the shadow_val."]
        #[inline(always)]
        pub fn set_cnt_lim_lo(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "set to enable low limit."]
        #[inline(always)]
        pub const fn cnt_ll_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable low limit."]
        #[inline(always)]
        pub fn set_cnt_ll_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "up limit offset selection, from one of the shadow_val."]
        #[inline(always)]
        pub const fn cnt_lim_up(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "up limit offset selection, from one of the shadow_val."]
        #[inline(always)]
        pub fn set_cnt_lim_up(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "set to enable up limit, use cnt_lu_off to select one of the shadow register value as limitation."]
        #[inline(always)]
        pub const fn cnt_lu_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable up limit, use cnt_lu_off to select one of the shadow register value as limitation."]
        #[inline(always)]
        pub fn set_cnt_lu_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "select one of the dac value."]
        #[inline(always)]
        pub const fn cnt_dac_index(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "select one of the dac value."]
        #[inline(always)]
        pub fn set_cnt_dac_index(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
    }
    impl Default for CntCfg1 {
        #[inline(always)]
        fn default() -> CntCfg1 {
            CntCfg1(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CntGlbcfg(pub u32);
    impl CntGlbcfg {
        #[doc = "1 to enable the main cycle counter; 0 to stop the counter; NOTE: when counter stopped, the related trigger_out will be cleared to 0, the related pwm output will keep value not changed."]
        #[inline(always)]
        pub const fn timer_enable(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "1 to enable the main cycle counter; 0 to stop the counter; NOTE: when counter stopped, the related trigger_out will be cleared to 0, the related pwm output will keep value not changed."]
        #[inline(always)]
        pub fn set_timer_enable(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "set to clear current timer. Auto clear."]
        #[inline(always)]
        pub const fn timer_reset(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "set to clear current timer. Auto clear."]
        #[inline(always)]
        pub fn set_timer_reset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "set to start pwm output(at next reload point), write only, Auto clear. User can disable pwm output before burst end by start again with cnt_burst=0."]
        #[inline(always)]
        pub const fn cnt_sw_start(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "set to start pwm output(at next reload point), write only, Auto clear. User can disable pwm output before burst end by start again with cnt_burst=0."]
        #[inline(always)]
        pub fn set_cnt_sw_start(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for CntGlbcfg {
        #[inline(always)]
        fn default() -> CntGlbcfg {
            CntGlbcfg(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CntReloadWork(pub u32);
    impl CntReloadWork {
        #[doc = "counter0 reload working register."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "counter0 reload working register."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CntReloadWork {
        #[inline(always)]
        fn default() -> CntReloadWork {
            CntReloadWork(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CntVal(pub u32);
    impl CntVal {
        #[doc = "main counter value."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "main counter value."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CntVal {
        #[inline(always)]
        fn default() -> CntVal {
            CntVal(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DacValueSv(pub u32);
    impl DacValueSv {
        #[doc = "save dac0_value when dac0_valid if dac_sw_mode is 0; software write dac_value directly if dac_sw_mode is 1."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "save dac0_value when dac0_valid if dac_sw_mode is 0; software write dac_value directly if dac_sw_mode is 1."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DacValueSv {
        #[inline(always)]
        fn default() -> DacValueSv {
            DacValueSv(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DeadArea(pub u32);
    impl DeadArea {
        #[doc = "16bit cycle delay plus 8bit hr_delay min value is 2 cycles, less than 0x200 will be treated as no dead area; NOTE: dead insertion must be configured with pair, that is, for pwm 01/23/45/67. otherwise the result maybe UNKNOWN!!!."]
        #[inline(always)]
        pub const fn dead_area(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "16bit cycle delay plus 8bit hr_delay min value is 2 cycles, less than 0x200 will be treated as no dead area; NOTE: dead insertion must be configured with pair, that is, for pwm 01/23/45/67. otherwise the result maybe UNKNOWN!!!."]
        #[inline(always)]
        pub fn set_dead_area(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for DeadArea {
        #[inline(always)]
        fn default() -> DeadArea {
            DeadArea(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaEn(pub u32);
    impl DmaEn {
        #[doc = "selelct one of compare point(0~23) or one reload point(24~27) as dma0."]
        #[inline(always)]
        pub const fn dma0_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "selelct one of compare point(0~23) or one reload point(24~27) as dma0."]
        #[inline(always)]
        pub fn set_dma0_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "enable dma0."]
        #[inline(always)]
        pub const fn dma0_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable dma0."]
        #[inline(always)]
        pub fn set_dma0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "selelct one of compare point(0~23) or one reload point(24~27) as dma0."]
        #[inline(always)]
        pub const fn dma1_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "selelct one of compare point(0~23) or one reload point(24~27) as dma0."]
        #[inline(always)]
        pub fn set_dma1_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "enable dma1."]
        #[inline(always)]
        pub const fn dma1_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "enable dma1."]
        #[inline(always)]
        pub fn set_dma1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "selelct one of compare point(0~23) or one reload point(24~27) as dma0."]
        #[inline(always)]
        pub const fn dma2_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "selelct one of compare point(0~23) or one reload point(24~27) as dma0."]
        #[inline(always)]
        pub fn set_dma2_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "enable dma2."]
        #[inline(always)]
        pub const fn dma2_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "enable dma2."]
        #[inline(always)]
        pub fn set_dma2_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "selelct one of compare point(0~23) or one reload point(24~27) as dma0."]
        #[inline(always)]
        pub const fn dma3_sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "selelct one of compare point(0~23) or one reload point(24~27) as dma0."]
        #[inline(always)]
        pub fn set_dma3_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "enable dma3."]
        #[inline(always)]
        pub const fn dma3_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "enable dma3."]
        #[inline(always)]
        pub fn set_dma3_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DmaEn {
        #[inline(always)]
        fn default() -> DmaEn {
            DmaEn(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ForceMode(pub u32);
    impl ForceMode {
        #[doc = "2bit for each PWM channel(0~7); 00: force output 0 01: force output 1 10: output highz(pad_oe_*=0) 11: no force this field may be changed by software as shadow register , the update time should be defined by chan_cfg.load, only for PWM channels."]
        #[inline(always)]
        pub const fn force_mode(&self, n: usize) -> super::vals::ForceMode {
            assert!(n < 8usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::ForceMode::from_bits(val as u8)
        }
        #[doc = "2bit for each PWM channel(0~7); 00: force output 0 01: force output 1 10: output highz(pad_oe_*=0) 11: no force this field may be changed by software as shadow register , the update time should be defined by chan_cfg.load, only for PWM channels."]
        #[inline(always)]
        pub fn set_force_mode(&mut self, n: usize, val: super::vals::ForceMode) {
            assert!(n < 8usize);
            let offs = 0usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "one bit for one pwm channel, it's used as shadow register when pwm_cfg0.polarity_opt0 is set. output polarity, set to 1 will invert the output(after pwm selection, pair mode, dead area insertion, before force/fault)."]
        #[inline(always)]
        pub const fn polarity(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "one bit for one pwm channel, it's used as shadow register when pwm_cfg0.polarity_opt0 is set. output polarity, set to 1 will invert the output(after pwm selection, pair mode, dead area insertion, before force/fault)."]
        #[inline(always)]
        pub fn set_polarity(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for ForceMode {
        #[inline(always)]
        fn default() -> ForceMode {
            ForceMode(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ForceWork(pub u32);
    impl ForceWork {
        #[doc = "force_mode work register."]
        #[inline(always)]
        pub const fn force_mode(&self, n: usize) -> super::vals::ForceMode {
            assert!(n < 8usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::ForceMode::from_bits(val as u8)
        }
        #[doc = "force_mode work register."]
        #[inline(always)]
        pub fn set_force_mode(&mut self, n: usize, val: super::vals::ForceMode) {
            assert!(n < 8usize);
            let offs = 0usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "force working register."]
        #[inline(always)]
        pub const fn out_polarity(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "force working register."]
        #[inline(always)]
        pub fn set_out_polarity(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for ForceWork {
        #[inline(always)]
        fn default() -> ForceWork {
            ForceWork(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlbCtrl(pub u32);
    impl GlbCtrl {
        #[doc = "set to disable bit\\[7:0\\]
in DAC value when Calculation Unit use it."]
        #[inline(always)]
        pub const fn frac_disable(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "set to disable bit\\[7:0\\]
in DAC value when Calculation Unit use it."]
        #[inline(always)]
        pub fn set_frac_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "set to enable hr pwm, clear to bypass delay chain."]
        #[inline(always)]
        pub const fn hr_pwm_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable hr pwm, clear to bypass delay chain."]
        #[inline(always)]
        pub fn set_hr_pwm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "add delay after dead_area insertiong logic, for hr_pwm."]
        #[inline(always)]
        pub const fn output_delay(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "add delay after dead_area insertiong logic, for hr_pwm."]
        #[inline(always)]
        pub fn set_output_delay(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "software write 1 to start software force, if the pwm_cfg\\[n\\].sw_force_en is set, force will take effort."]
        #[inline(always)]
        pub const fn sw_force(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "software write 1 to start software force, if the pwm_cfg\\[n\\].sw_force_en is set, force will take effort."]
        #[inline(always)]
        pub fn set_sw_force(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for GlbCtrl {
        #[inline(always)]
        fn default() -> GlbCtrl {
            GlbCtrl(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlbCtrl2(pub u32);
    impl GlbCtrl2 {
        #[doc = "enable shadow_lock feature, if cleared, shadow_lock will be always 0."]
        #[inline(always)]
        pub const fn shadow_lock_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable shadow_lock feature, if cleared, shadow_lock will be always 0."]
        #[inline(always)]
        pub fn set_shadow_lock_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "software write 1 to clear fault event if pwm_cfg.fault_rec_time is 2'b11. software need to clear it after the fault signal is de-assert and before next fault one bit for one pwm channel."]
        #[inline(always)]
        pub const fn fault_clear(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "software write 1 to clear fault event if pwm_cfg.fault_rec_time is 2'b11. software need to clear it after the fault signal is de-assert and before next fault one bit for one pwm channel."]
        #[inline(always)]
        pub fn set_fault_clear(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "set to enable debug_in signal as fault signal, generally disable pwm output."]
        #[inline(always)]
        pub const fn debug_in_en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable debug_in signal as fault signal, generally disable pwm output."]
        #[inline(always)]
        pub fn set_debug_in_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "set for software DAC mode, software can write dac_value*_sv directly, and dac_valid from moto system is ignored."]
        #[inline(always)]
        pub const fn dac_sw_mode(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "set for software DAC mode, software can write dac_value*_sv directly, and dac_valid from moto system is ignored."]
        #[inline(always)]
        pub fn set_dac_sw_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for GlbCtrl2 {
        #[inline(always)]
        fn default() -> GlbCtrl2 {
            GlbCtrl2(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEn(pub u32);
    impl IrqEn {
        #[doc = "enable interrupt when calculation unit overflow."]
        #[inline(always)]
        pub const fn irq_en_overflow(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt when calculation unit overflow."]
        #[inline(always)]
        pub fn set_irq_en_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IrqEn {
        #[inline(always)]
        fn default() -> IrqEn {
            IrqEn(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnBurstend(pub u32);
    impl IrqEnBurstend {
        #[doc = "interrupt enable field for output burst done event , and each bit means one main counter."]
        #[inline(always)]
        pub const fn irq_en_burstend(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "interrupt enable field for output burst done event , and each bit means one main counter."]
        #[inline(always)]
        pub fn set_irq_en_burstend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for IrqEnBurstend {
        #[inline(always)]
        fn default() -> IrqEnBurstend {
            IrqEnBurstend(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnCapNeg(pub u32);
    impl IrqEnCapNeg {
        #[doc = "interrupt enable field for negedge capture event , and each bit means one capture channel."]
        #[inline(always)]
        pub const fn irq_en_cap_neg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "interrupt enable field for negedge capture event , and each bit means one capture channel."]
        #[inline(always)]
        pub fn set_irq_en_cap_neg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for IrqEnCapNeg {
        #[inline(always)]
        fn default() -> IrqEnCapNeg {
            IrqEnCapNeg(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnCapPos(pub u32);
    impl IrqEnCapPos {
        #[doc = "interrupt enable field for posedge capture event , and each bit means one capture channel."]
        #[inline(always)]
        pub const fn irq_en_cap_pos(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "interrupt enable field for posedge capture event , and each bit means one capture channel."]
        #[inline(always)]
        pub fn set_irq_en_cap_pos(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for IrqEnCapPos {
        #[inline(always)]
        fn default() -> IrqEnCapPos {
            IrqEnCapPos(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnCmp(pub u32);
    impl IrqEnCmp {
        #[doc = "interrupt enable field for compare point match event, and each bit means one compare point."]
        #[inline(always)]
        pub const fn irq_en_cmp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "interrupt enable field for compare point match event, and each bit means one compare point."]
        #[inline(always)]
        pub fn set_irq_en_cmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for IrqEnCmp {
        #[inline(always)]
        fn default() -> IrqEnCmp {
            IrqEnCmp(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnFault(pub u32);
    impl IrqEnFault {
        #[doc = "interrupt enable field for external fault event , and each bit means one external fault channel."]
        #[inline(always)]
        pub const fn irq_en_fault(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "interrupt enable field for external fault event , and each bit means one external fault channel."]
        #[inline(always)]
        pub fn set_irq_en_fault(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for IrqEnFault {
        #[inline(always)]
        fn default() -> IrqEnFault {
            IrqEnFault(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnReload(pub u32);
    impl IrqEnReload {
        #[doc = "interrupt enable field for reload event , and each bit means one main counter."]
        #[inline(always)]
        pub const fn irq_en_reload(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "interrupt enable field for reload event , and each bit means one main counter."]
        #[inline(always)]
        pub fn set_irq_en_reload(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for IrqEnReload {
        #[inline(always)]
        fn default() -> IrqEnReload {
            IrqEnReload(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqSts(pub u32);
    impl IrqSts {
        #[doc = "for 24 channel, compare event."]
        #[inline(always)]
        pub const fn irq_cmp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "for 24 channel, compare event."]
        #[inline(always)]
        pub fn set_irq_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "when clock counter reach the reload time."]
        #[inline(always)]
        pub const fn irq_reload(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "when clock counter reach the reload time."]
        #[inline(always)]
        pub fn set_irq_reload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "capture posedge status."]
        #[inline(always)]
        pub const fn irq_capture_pos(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "capture posedge status."]
        #[inline(always)]
        pub fn set_irq_capture_pos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "capture negedge status."]
        #[inline(always)]
        pub const fn irq_capture_neg(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "capture negedge status."]
        #[inline(always)]
        pub fn set_irq_capture_neg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "for external fault event."]
        #[inline(always)]
        pub const fn irq_fault(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "for external fault event."]
        #[inline(always)]
        pub fn set_irq_fault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "end of output burst."]
        #[inline(always)]
        pub const fn irq_burstend(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "end of output burst."]
        #[inline(always)]
        pub fn set_irq_burstend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "end of output burst."]
        #[inline(always)]
        pub const fn irq_cal_overflow(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "end of output burst."]
        #[inline(always)]
        pub fn set_irq_cal_overflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IrqSts {
        #[inline(always)]
        fn default() -> IrqSts {
            IrqSts(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStsBurstend(pub u32);
    impl IrqStsBurstend {
        #[doc = "interrupt flag for output burst done event , and each bit means one main counter."]
        #[inline(always)]
        pub const fn irq_sts_burstend(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "interrupt flag for output burst done event , and each bit means one main counter."]
        #[inline(always)]
        pub fn set_irq_sts_burstend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for IrqStsBurstend {
        #[inline(always)]
        fn default() -> IrqStsBurstend {
            IrqStsBurstend(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStsCapNeg(pub u32);
    impl IrqStsCapNeg {
        #[doc = "interrupt flag for negedge capture event , and each bit means one capture channel."]
        #[inline(always)]
        pub const fn irq_sts_cap_neg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "interrupt flag for negedge capture event , and each bit means one capture channel."]
        #[inline(always)]
        pub fn set_irq_sts_cap_neg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for IrqStsCapNeg {
        #[inline(always)]
        fn default() -> IrqStsCapNeg {
            IrqStsCapNeg(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStsCapPos(pub u32);
    impl IrqStsCapPos {
        #[doc = "interrupt flag for posedge capture event , and each bit means one capture channel."]
        #[inline(always)]
        pub const fn irq_sts_cap_pos(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "interrupt flag for posedge capture event , and each bit means one capture channel."]
        #[inline(always)]
        pub fn set_irq_sts_cap_pos(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for IrqStsCapPos {
        #[inline(always)]
        fn default() -> IrqStsCapPos {
            IrqStsCapPos(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStsCmp(pub u32);
    impl IrqStsCmp {
        #[doc = "interrupt flag for compare point match event, and each bit means one compare point."]
        #[inline(always)]
        pub const fn irq_sts_cmp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "interrupt flag for compare point match event, and each bit means one compare point."]
        #[inline(always)]
        pub fn set_irq_sts_cmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for IrqStsCmp {
        #[inline(always)]
        fn default() -> IrqStsCmp {
            IrqStsCmp(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStsFault(pub u32);
    impl IrqStsFault {
        #[doc = "interrupt flag for external fault event , and each bit means one external fault channel."]
        #[inline(always)]
        pub const fn irq_sts_fault(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "interrupt flag for external fault event , and each bit means one external fault channel."]
        #[inline(always)]
        pub fn set_irq_sts_fault(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for IrqStsFault {
        #[inline(always)]
        fn default() -> IrqStsFault {
            IrqStsFault(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStsReload(pub u32);
    impl IrqStsReload {
        #[doc = "interrupt flag for reload event , and each bit means one main counter."]
        #[inline(always)]
        pub const fn irq_sts_reload(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "interrupt flag for reload event , and each bit means one main counter."]
        #[inline(always)]
        pub fn set_irq_sts_reload(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for IrqStsReload {
        #[inline(always)]
        fn default() -> IrqStsReload {
            IrqStsReload(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmCfg0(pub u32);
    impl PwmCfg0 {
        #[doc = "set to use shadow polarity."]
        #[inline(always)]
        pub const fn polarity_opt0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "set to use shadow polarity."]
        #[inline(always)]
        pub fn set_polarity_opt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "output polarity, set to 1 will invert the output(after pwm selection, pair mode, dead area insertion, before force/fault) when polarity_opt0 is set, this bit is controlled by shadow register, can't be writable; read as working register use compare channel settings(in cmp_cfg) as shadow register update."]
        #[inline(always)]
        pub const fn out_polarity(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "output polarity, set to 1 will invert the output(after pwm selection, pair mode, dead area insertion, before force/fault) when polarity_opt0 is set, this bit is controlled by shadow register, can't be writable; read as working register use compare channel settings(in cmp_cfg) as shadow register update."]
        #[inline(always)]
        pub fn set_out_polarity(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "used when polarity_opt0 is set, define when to update polarity working register. 0: software set work_ctrl1.shadow_lock bit 1: update at reload point;."]
        #[inline(always)]
        pub const fn pol_update_sel(&self) -> super::vals::ShadowOutputPolarity {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::ShadowOutputPolarity::from_bits(val as u8)
        }
        #[doc = "used when polarity_opt0 is set, define when to update polarity working register. 0: software set work_ctrl1.shadow_lock bit 1: update at reload point;."]
        #[inline(always)]
        pub fn set_pol_update_sel(&mut self, val: super::vals::ShadowOutputPolarity) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "set to enable the input faults from trig_mux(trigger_in\\[0\\]
for channel0/1, 1 for 23, 2 for 45, 3 for 67)."]
        #[inline(always)]
        pub const fn fault_en_sync(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable the input faults from trig_mux(trigger_in\\[0\\]
for channel0/1, 1 for 23, 2 for 45, 3 for 67)."]
        #[inline(always)]
        pub fn set_fault_en_sync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "set to enable the input async faults from pad directly."]
        #[inline(always)]
        pub const fn fault_en_async(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable the input async faults from pad directly."]
        #[inline(always)]
        pub fn set_fault_en_async(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "fault polarity for input fault from pad, 1-active low; 0-active high;."]
        #[inline(always)]
        pub const fn fault_pol_async(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "fault polarity for input fault from pad, 1-active low; 0-active high;."]
        #[inline(always)]
        pub fn set_fault_pol_async(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "select from 16bit async fault from pad."]
        #[inline(always)]
        pub const fn fault_sel_async(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "select from 16bit async fault from pad."]
        #[inline(always)]
        pub fn set_fault_sel_async(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "for N=0/2/4/6, clear to select 2 compare point(N*2~N*2+1); set to select 4 compare point(N*2~N*2+3); or use 2 compare point(N*2+2~N*2+3); for N=1/3/5/7, this bit is no means, it can work on pair mode, or use 2 compare point (N*2+2~N*2+3); assume select ab or abcd, abcd can between 0 and 2T. output will be 1 when counter value between a and b; if b<=a then output all 0; if b>=(T+a), then output all 1;."]
        #[inline(always)]
        pub const fn trig_sel4(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "for N=0/2/4/6, clear to select 2 compare point(N*2~N*2+1); set to select 4 compare point(N*2~N*2+3); or use 2 compare point(N*2+2~N*2+3); for N=1/3/5/7, this bit is no means, it can work on pair mode, or use 2 compare point (N*2+2~N*2+3); assume select ab or abcd, abcd can between 0 and 2T. output will be 1 when counter value between a and b; if b<=a then output all 0; if b>=(T+a), then output all 1;."]
        #[inline(always)]
        pub fn set_trig_sel4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for PwmCfg0 {
        #[inline(always)]
        fn default() -> PwmCfg0 {
            PwmCfg0(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmCfg1(pub u32);
    impl PwmCfg1 {
        #[doc = "select one trigger from 8, should set to pulse in trig_mux, used for fault recovery if fault_rec_time is set to 2'b10."]
        #[inline(always)]
        pub const fn fault_rec_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux, used for fault recovery if fault_rec_time is set to 2'b10."]
        #[inline(always)]
        pub fn set_fault_rec_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "select one trigger from 8 as force signal, should be level signal, 1 for force active, 0 for no force."]
        #[inline(always)]
        pub const fn pwm_force_sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "select one trigger from 8 as force signal, should be level signal, 1 for force active, 0 for no force."]
        #[inline(always)]
        pub fn set_pwm_force_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux, will load hw/sw force at this time."]
        #[inline(always)]
        pub const fn force_act_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux, will load hw/sw force at this time."]
        #[inline(always)]
        pub fn set_force_act_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux, will load shadow register(force)mode) to force_mode_work at this time."]
        #[inline(always)]
        pub const fn force_trig_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "select one trigger from 8, should set to pulse in trig_mux, will load shadow register(force)mode) to force_mode_work at this time."]
        #[inline(always)]
        pub fn set_force_trig_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "00: force immediately 01: force at main counter reload time 10: force at trig signal selected by force_act_sel 11: no force the force assert/deassert will happen at the force_time; qeo force and value also latched at this time."]
        #[inline(always)]
        pub const fn force_time(&self) -> super::vals::ForceTrigger {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::ForceTrigger::from_bits(val as u8)
        }
        #[doc = "00: force immediately 01: force at main counter reload time 10: force at trig signal selected by force_act_sel 11: no force the force assert/deassert will happen at the force_time; qeo force and value also latched at this time."]
        #[inline(always)]
        pub fn set_force_time(&mut self, val: super::vals::ForceTrigger) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "valid only for pwm0/2/4/6 when trig_sel4 is set 00: ab OR cd; 01: ab AND cd; 10: ab XOR cd; 11: cd."]
        #[inline(always)]
        pub const fn pwm_logic(&self) -> super::vals::PwmLogic {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::PwmLogic::from_bits(val as u8)
        }
        #[doc = "valid only for pwm0/2/4/6 when trig_sel4 is set 00: ab OR cd; 01: ab AND cd; 10: ab XOR cd; 11: cd."]
        #[inline(always)]
        pub fn set_pwm_logic(&mut self, val: super::vals::PwmLogic) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "if set to 1, PWM work at pair mode, pwm_cfg for channel 2m is used for channel 2m+1(m=0,1,2,3), except the dead area, which is separate for each channel even in pair mode software need set this bit for both channel of one pair, otherwise result unknown."]
        #[inline(always)]
        pub const fn pair_mode(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "if set to 1, PWM work at pair mode, pwm_cfg for channel 2m is used for channel 2m+1(m=0,1,2,3), except the dead area, which is separate for each channel even in pair mode software need set this bit for both channel of one pair, otherwise result unknown."]
        #[inline(always)]
        pub fn set_pair_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "0 for hardware force, from trig_mux selected by pwm_force_sel 1 for software force, from glb_ctrl.sw_force."]
        #[inline(always)]
        pub const fn sw_force_en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "0 for hardware force, from trig_mux selected by pwm_force_sel 1 for software force, from glb_ctrl.sw_force."]
        #[inline(always)]
        pub fn set_sw_force_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "00: immediately 01: after main counter reload time 10: use fault_rec_sel to select one of the input trigger 11: software write fault_clear in glb_ctrl2, no effort if pwm_fault is still assert."]
        #[inline(always)]
        pub const fn fault_rec_time(&self) -> super::vals::FaultRecoveryTrigger {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::FaultRecoveryTrigger::from_bits(val as u8)
        }
        #[doc = "00: immediately 01: after main counter reload time 10: use fault_rec_sel to select one of the input trigger 11: software write fault_clear in glb_ctrl2, no effort if pwm_fault is still assert."]
        #[inline(always)]
        pub fn set_fault_rec_time(&mut self, val: super::vals::FaultRecoveryTrigger) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "00: force output 0 01: force output 1 1x: output highz(pad_oe_*=0)."]
        #[inline(always)]
        pub const fn fault_mode(&self) -> super::vals::FaultMode {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::FaultMode::from_bits(val as u8)
        }
        #[doc = "00: force output 0 01: force output 1 1x: output highz(pad_oe_*=0)."]
        #[inline(always)]
        pub fn set_fault_mode(&mut self, val: super::vals::FaultMode) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "define when to use the shadow register value for working register(force_mode) 00: software set work_ctrl1.shadow_lock bit 01: use the related counter rld_cmp_sel0 and rld_cmp_sel1, to select one compare point 10: related counter reload time(selected by pwm_cnt) 11: use force_trig_sel to select one of the input trigger NOTE: 00/01 are not recommended since the update time is not controllable, may cause error in complex application. 00 is used for initialization or debug, not suggest for real time update."]
        #[inline(always)]
        pub const fn force_update_time(&self) -> super::vals::ForceShadowUpdateTrigger {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::ForceShadowUpdateTrigger::from_bits(val as u8)
        }
        #[doc = "define when to use the shadow register value for working register(force_mode) 00: software set work_ctrl1.shadow_lock bit 01: use the related counter rld_cmp_sel0 and rld_cmp_sel1, to select one compare point 10: related counter reload time(selected by pwm_cnt) 11: use force_trig_sel to select one of the input trigger NOTE: 00/01 are not recommended since the update time is not controllable, may cause error in complex application. 00 is used for initialization or debug, not suggest for real time update."]
        #[inline(always)]
        pub fn set_force_update_time(&mut self, val: super::vals::ForceShadowUpdateTrigger) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "0 to highz pwm outputs(pad_oe*=0), software need set this bit to 1 to enable pwm output."]
        #[inline(always)]
        pub const fn highz_en_n(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "0 to highz pwm outputs(pad_oe*=0), software need set this bit to 1 to enable pwm output."]
        #[inline(always)]
        pub fn set_highz_en_n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for PwmCfg1 {
        #[inline(always)]
        fn default() -> PwmCfg1 {
            PwmCfg1(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ShadowVal(pub u32);
    impl ShadowVal {
        #[doc = "Fractional part of the shadow value."]
        #[inline(always)]
        pub const fn frac(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Fractional part of the shadow value."]
        #[inline(always)]
        pub fn set_frac(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "shadow registers, if used as reload or compare point, shall be 24bit clock cycles plus 1bit half cycle and 7bit high-resolution delay."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "shadow registers, if used as reload or compare point, shall be 24bit clock cycles plus 1bit half cycle and 7bit high-resolution delay."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
        #[doc = "Integer part of the shadow value."]
        #[inline(always)]
        pub const fn int(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Integer part of the shadow value."]
        #[inline(always)]
        pub fn set_int(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for ShadowVal {
        #[inline(always)]
        fn default() -> ShadowVal {
            ShadowVal(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TriggerCfg(pub u32);
    impl TriggerCfg {
        #[doc = "select one from 24 compare result as trigger out, set at compare point, clear at reload point."]
        #[inline(always)]
        pub const fn trigger_out_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "select one from 24 compare result as trigger out, set at compare point, clear at reload point."]
        #[inline(always)]
        pub fn set_trigger_out_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for TriggerCfg {
        #[inline(always)]
        fn default() -> TriggerCfg {
            TriggerCfg(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Unlock(pub u32);
    impl Unlock {
        #[doc = "bit2 to bit 29 for value_shadow, bit30 for force_mode the shadow registers can be updated only when related unlock_bit is set; this register can only be updated after unlock."]
        #[inline(always)]
        pub const fn unlock_bit(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "bit2 to bit 29 for value_shadow, bit30 for force_mode the shadow registers can be updated only when related unlock_bit is set; this register can only be updated after unlock."]
        #[inline(always)]
        pub fn set_unlock_bit(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Unlock {
        #[inline(always)]
        fn default() -> Unlock {
            Unlock(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WorkCtrl0(pub u32);
    impl WorkCtrl0 {
        #[doc = "write 0x… first to unlock, then set related bits in unlock_sel to unlock following shadow registers(from 0x04 to 0x78), otherwise the shadow registers can not be written. The shadow registers will be loaded to work registers only when shadow_lock is 1 or lock is not enabled This bit can be cleared by set shadow_lock bit in work_ctrl1."]
        #[inline(always)]
        pub const fn shadow_unlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "write 0x… first to unlock, then set related bits in unlock_sel to unlock following shadow registers(from 0x04 to 0x78), otherwise the shadow registers can not be written. The shadow registers will be loaded to work registers only when shadow_lock is 1 or lock is not enabled This bit can be cleared by set shadow_lock bit in work_ctrl1."]
        #[inline(always)]
        pub fn set_shadow_unlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for WorkCtrl0 {
        #[inline(always)]
        fn default() -> WorkCtrl0 {
            WorkCtrl0(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WorkCtrl1(pub u32);
    impl WorkCtrl1 {
        #[doc = "one to lock, sofware can't write any shadow registers Software have to write 0x…. to work_ctrl0 to clear this bit."]
        #[inline(always)]
        pub const fn shadow_lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "one to lock, sofware can't write any shadow registers Software have to write 0x…. to work_ctrl0 to clear this bit."]
        #[inline(always)]
        pub fn set_shadow_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for WorkCtrl1 {
        #[inline(always)]
        fn default() -> WorkCtrl1 {
            WorkCtrl1(0)
        }
    }
}
pub mod vals {
    #[doc = "define when to use the shadow register value for working register(trig_cmp)"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CmpShadowUpdateTrigger {
        #[doc = "software set work_ctrl1.shadow_lock bit"]
        ON_SHLK = 0x0,
        #[doc = "update immediately(at next cycle)"]
        ON_MODIFY = 0x01,
        #[doc = "related counter reload time"]
        ON_RELOAD = 0x02,
        #[doc = "use cmp_update_trigger(from trig_mux, selected by cmp_trig_sel)"]
        ON_TRIGMUX = 0x03,
        #[doc = "use the related counter rld_cmp_sel0 to select one compare point"]
        ON_RLD_CMP_SEL0 = 0x04,
        #[doc = "use the related counter rld_cmp_sel1, to select one compare point"]
        ON_RLD_CMP_SEL1 = 0x05,
        #[doc = "reserved, no update"]
        NONE = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl CmpShadowUpdateTrigger {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CmpShadowUpdateTrigger {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CmpShadowUpdateTrigger {
        #[inline(always)]
        fn from(val: u8) -> CmpShadowUpdateTrigger {
            CmpShadowUpdateTrigger::from_bits(val)
        }
    }
    impl From<CmpShadowUpdateTrigger> for u8 {
        #[inline(always)]
        fn from(val: CmpShadowUpdateTrigger) -> u8 {
            CmpShadowUpdateTrigger::to_bits(val)
        }
    }
    #[doc = "select one of the calculation cell output"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CmpSource {
        SHADOW_VAL = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
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
        CALCULATE = 0x20,
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
        CAPTURE_POS = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        #[doc = "select T/4"]
        COUNTERS = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        #[doc = "select 0xFFFFF000"]
        _0XFFFFF000 = 0x3e,
        #[doc = "select 0xFFFFFF00"]
        _0XFFFFFF00 = 0x3f,
    }
    impl CmpSource {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CmpSource {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CmpSource {
        #[inline(always)]
        fn from(val: u8) -> CmpSource {
            CmpSource::from_bits(val)
        }
    }
    impl From<CmpSource> for u8 {
        #[inline(always)]
        fn from(val: CmpSource) -> u8 {
            CmpSource::to_bits(val)
        }
    }
    #[doc = "00: force output 0 01: force output 1 1x: output highz(pad_oe_*=0)"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum FaultMode {
        #[doc = "force output 0"]
        _0 = 0x0,
        #[doc = "force output 1"]
        _1 = 0x01,
        #[doc = "output highz(pad_oe_*=0)"]
        HIGH_Z = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl FaultMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FaultMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FaultMode {
        #[inline(always)]
        fn from(val: u8) -> FaultMode {
            FaultMode::from_bits(val)
        }
    }
    impl From<FaultMode> for u8 {
        #[inline(always)]
        fn from(val: FaultMode) -> u8 {
            FaultMode::to_bits(val)
        }
    }
    #[doc = "define when to use the shadow register value for working register(force_mode)"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum FaultRecoveryTrigger {
        #[doc = "immediately"]
        IMMEDIATELY = 0x0,
        #[doc = "after main counter reload time"]
        ON_RELOAD = 0x01,
        #[doc = "after hardware event assert"]
        ON_HW_EVENT = 0x02,
        #[doc = "after software write faultclr bit in GCR register"]
        ON_FAULT_CLEAR = 0x03,
    }
    impl FaultRecoveryTrigger {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FaultRecoveryTrigger {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FaultRecoveryTrigger {
        #[inline(always)]
        fn from(val: u8) -> FaultRecoveryTrigger {
            FaultRecoveryTrigger::from_bits(val)
        }
    }
    impl From<FaultRecoveryTrigger> for u8 {
        #[inline(always)]
        fn from(val: FaultRecoveryTrigger) -> u8 {
            FaultRecoveryTrigger::to_bits(val)
        }
    }
    #[doc = "00: force output 0 01: force output 1 1x: output highz(pad_oe_*=0)"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ForceMode {
        #[doc = "force output 0"]
        _0 = 0x0,
        #[doc = "force output 1"]
        _1 = 0x01,
        #[doc = "output highz(pad_oe_*=0)"]
        HIGH_Z = 0x02,
        #[doc = "no force"]
        NO_FORCE = 0x03,
    }
    impl ForceMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ForceMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ForceMode {
        #[inline(always)]
        fn from(val: u8) -> ForceMode {
            ForceMode::from_bits(val)
        }
    }
    impl From<ForceMode> for u8 {
        #[inline(always)]
        fn from(val: ForceMode) -> u8 {
            ForceMode::to_bits(val)
        }
    }
    #[doc = "define when to use the shadow register value for working register(force_mode)"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ForceShadowUpdateTrigger {
        #[doc = "software set work_ctrl1.shadow_lock bit"]
        IMMEDIATELY = 0x0,
        #[doc = "related counter reload time(selected by pwm_cnt)"]
        ON_CMP_POINT = 0x01,
        #[doc = "use the related counter rld_cmp_sel0 and rld_cmp_sel1, to select one compare point"]
        ON_RELOAD = 0x02,
        #[doc = "after SHSYNCI assert"]
        NONE = 0x03,
    }
    impl ForceShadowUpdateTrigger {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ForceShadowUpdateTrigger {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ForceShadowUpdateTrigger {
        #[inline(always)]
        fn from(val: u8) -> ForceShadowUpdateTrigger {
            ForceShadowUpdateTrigger::from_bits(val)
        }
    }
    impl From<ForceShadowUpdateTrigger> for u8 {
        #[inline(always)]
        fn from(val: ForceShadowUpdateTrigger) -> u8 {
            ForceShadowUpdateTrigger::to_bits(val)
        }
    }
    #[doc = "define when to use the shadow register value for working register(force_mode)"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ForceTrigger {
        #[doc = "force immediately"]
        IMMEDIATELY = 0x0,
        #[doc = "force at main counter reload time"]
        ON_RELOAD = 0x01,
        #[doc = "force at trig signal selected by force_act_sel"]
        ON_TRIGMUX = 0x02,
        #[doc = "no force the force assert/deassert will happen at the force_time; qeo force and value also latched at this time"]
        NNONE = 0x03,
    }
    impl ForceTrigger {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ForceTrigger {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ForceTrigger {
        #[inline(always)]
        fn from(val: u8) -> ForceTrigger {
            ForceTrigger::from_bits(val)
        }
    }
    impl From<ForceTrigger> for u8 {
        #[inline(always)]
        fn from(val: ForceTrigger) -> u8 {
            ForceTrigger::to_bits(val)
        }
    }
    #[doc = "valid only for pwm0/2/4/6 when trig_sel4 is set"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum PwmLogic {
        #[doc = "ab OR cd"]
        AB_OR_CD = 0x0,
        #[doc = "ab AND cd"]
        AB_AND_CD = 0x01,
        #[doc = "ab XOR cd"]
        AB_XOR_CD = 0x02,
        #[doc = "cd"]
        CD = 0x03,
    }
    impl PwmLogic {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PwmLogic {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PwmLogic {
        #[inline(always)]
        fn from(val: u8) -> PwmLogic {
            PwmLogic::from_bits(val)
        }
    }
    impl From<PwmLogic> for u8 {
        #[inline(always)]
        fn from(val: PwmLogic) -> u8 {
            PwmLogic::to_bits(val)
        }
    }
    #[doc = "define when to use the calculation output value as reload time"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ReloadUpdateTrigger {
        #[doc = "software set work_ctrl1.shadow_lock bit"]
        ON_SHLK = 0x0,
        #[doc = "use compare point selected by rld_cmp_sel0 or rld_cmp_sel1"]
        ON_COMPARE_POINT = 0x01,
        #[doc = "counter reload time"]
        ON_RELOAD = 0x02,
        #[doc = "use rld_trig_sel to select one of the input trigger"]
        ON_TRIGGER = 0x03,
    }
    impl ReloadUpdateTrigger {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ReloadUpdateTrigger {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ReloadUpdateTrigger {
        #[inline(always)]
        fn from(val: u8) -> ReloadUpdateTrigger {
            ReloadUpdateTrigger::from_bits(val)
        }
    }
    impl From<ReloadUpdateTrigger> for u8 {
        #[inline(always)]
        fn from(val: ReloadUpdateTrigger) -> u8 {
            ReloadUpdateTrigger::to_bits(val)
        }
    }
    #[doc = "used when polarity_opt0 is set"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ShadowOutputPolarity {
        #[doc = "software set work_ctrl1.shadow_lock bit"]
        ON_SHLK = 0x0,
        #[doc = "update at reload point"]
        ON_RELOAD = 0x01,
    }
    impl ShadowOutputPolarity {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ShadowOutputPolarity {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ShadowOutputPolarity {
        #[inline(always)]
        fn from(val: u8) -> ShadowOutputPolarity {
            ShadowOutputPolarity::from_bits(val)
        }
    }
    impl From<ShadowOutputPolarity> for u8 {
        #[inline(always)]
        fn from(val: ShadowOutputPolarity) -> u8 {
            ShadowOutputPolarity::to_bits(val)
        }
    }
}
