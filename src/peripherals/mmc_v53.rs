#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Br {
    ptr: *mut u8,
}
unsafe impl Send for Br {}
unsafe impl Sync for Br {}
impl Br {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Prediction Control Register."]
    #[inline(always)]
    pub const fn br_ctrl(self) -> crate::common::Reg<regs::BrCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Prediction Timing Offset Register."]
    #[inline(always)]
    pub const fn br_timeoff(self) -> crate::common::Reg<regs::BrTimeoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Prediction Triggering Period Offset Register."]
    #[inline(always)]
    pub const fn br_trg_period(self) -> crate::common::Reg<regs::BrTrgPeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Prediction Triggering First Offset Register."]
    #[inline(always)]
    pub const fn br_trg_f_time(self) -> crate::common::Reg<regs::BrTrgFTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Prediction Status Register."]
    #[inline(always)]
    pub const fn br_st(self) -> crate::common::Reg<regs::BrSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Prediction Configuration postion trigger cfg."]
    #[inline(always)]
    pub const fn br_trg_pos_cfg(self) -> crate::common::Reg<regs::BrTrgPosCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Prediction Configuration postion threshold."]
    #[inline(always)]
    pub const fn br_trg_pos_thr(self) -> crate::common::Reg<regs::BrTrgPosThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Prediction Configuration revolutiom threshold."]
    #[inline(always)]
    pub const fn br_trg_rev_thr(self) -> crate::common::Reg<regs::BrTrgRevThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Prediction Configuration speed trigger cfg."]
    #[inline(always)]
    pub const fn br_trg_speed_cfg(
        self,
    ) -> crate::common::Reg<regs::BrTrgSpeedCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Prediction Configuration speed threshold."]
    #[inline(always)]
    pub const fn br_trg_speed_thr(
        self,
    ) -> crate::common::Reg<regs::BrTrgSpeedThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Initialization timestamp for open-loop mode."]
    #[inline(always)]
    pub const fn br_ini_pos_time(
        self,
    ) -> crate::common::Reg<regs::BrIniPosTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Initialization position for open-loop mode."]
    #[inline(always)]
    pub const fn br_ini_pos(self) -> crate::common::Reg<regs::BrIniPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Initialization revolution for open-loop mode."]
    #[inline(always)]
    pub const fn br_ini_rev(self) -> crate::common::Reg<regs::BrIniRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Initialization speed for open-loop mode."]
    #[inline(always)]
    pub const fn br_ini_speed(self) -> crate::common::Reg<regs::BrIniSpeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Initialization acceleration for open-loop mode."]
    #[inline(always)]
    pub const fn br_ini_accel(self) -> crate::common::Reg<regs::BrIniAccel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Initialization timestamp for delta mode in prediction mode."]
    #[inline(always)]
    pub const fn br_ini_delta_pos_time(
        self,
    ) -> crate::common::Reg<regs::BrIniDeltaPosTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Initialization delta position for delta mode in prediction mode."]
    #[inline(always)]
    pub const fn br_ini_delta_pos(
        self,
    ) -> crate::common::Reg<regs::BrIniDeltaPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Initialization delta revolution for delta mode in prediction mode."]
    #[inline(always)]
    pub const fn br_ini_delta_rev(
        self,
    ) -> crate::common::Reg<regs::BrIniDeltaRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Initialization delta speed for delta mode in prediction mode."]
    #[inline(always)]
    pub const fn br_ini_delta_speed(
        self,
    ) -> crate::common::Reg<regs::BrIniDeltaSpeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Initialization delta acceleration for delta mode in prediction mode."]
    #[inline(always)]
    pub const fn br_ini_delta_accel(
        self,
    ) -> crate::common::Reg<regs::BrIniDeltaAccel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Monitor of the output timestamp."]
    #[inline(always)]
    pub const fn br_cur_pos_time(
        self,
    ) -> crate::common::Reg<regs::BrCurPosTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Monitor of the output position."]
    #[inline(always)]
    pub const fn br_cur_pos(self) -> crate::common::Reg<regs::BrCurPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Monitor of the output revolution."]
    #[inline(always)]
    pub const fn br_cur_rev(self) -> crate::common::Reg<regs::BrCurRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Monitor of the output speed."]
    #[inline(always)]
    pub const fn br_cur_speed(self) -> crate::common::Reg<regs::BrCurSpeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Monitor of the output acceleration."]
    #[inline(always)]
    pub const fn br_cur_accel(self) -> crate::common::Reg<regs::BrCurAccel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoefTrgCfg {
    ptr: *mut u8,
}
unsafe impl Send for CoefTrgCfg {}
unsafe impl Sync for CoefTrgCfg {}
impl CoefTrgCfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0."]
    #[inline(always)]
    pub const fn err_thr(self) -> crate::common::Reg<regs::ErrThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 P."]
    #[inline(always)]
    pub const fn p(self) -> crate::common::Reg<regs::P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 I."]
    #[inline(always)]
    pub const fn i(self) -> crate::common::Reg<regs::I, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 A."]
    #[inline(always)]
    pub const fn a(self) -> crate::common::Reg<regs::A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 time."]
    #[inline(always)]
    pub const fn time(self) -> crate::common::Reg<regs::Time, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
#[doc = "MMC0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmc {
    ptr: *mut u8,
}
unsafe impl Send for Mmc {}
unsafe impl Sync for Mmc {}
impl Mmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sta(self) -> crate::common::Reg<regs::Sta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "System Clock Frequency Register."]
    #[inline(always)]
    pub const fn sysclk_freq(self) -> crate::common::Reg<regs::SysclkFreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "System Clock Period Register."]
    #[inline(always)]
    pub const fn sysclk_period(self) -> crate::common::Reg<regs::SysclkPeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Position Out-Of-Sync Threshold Regster."]
    #[inline(always)]
    pub const fn oosync_theta_thr(
        self,
    ) -> crate::common::Reg<regs::OosyncThetaThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Discrete Mode Configuration 0 Register."]
    #[inline(always)]
    pub const fn discrete_cfg0(self) -> crate::common::Reg<regs::DiscreteCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Discrete Mode Configuration 1 Register."]
    #[inline(always)]
    pub const fn discrete_cfg1(self) -> crate::common::Reg<regs::DiscreteCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Continuous Mode Configuration 0 Register."]
    #[inline(always)]
    pub const fn cont_cfg0(self) -> crate::common::Reg<regs::ContCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "The destined timestamp register for position initialization."]
    #[inline(always)]
    pub const fn ini_pos_time(self) -> crate::common::Reg<regs::IniPosTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "The destined position register for position initialization."]
    #[inline(always)]
    pub const fn ini_pos(self) -> crate::common::Reg<regs::IniPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "The destined revolution register for position initialization."]
    #[inline(always)]
    pub const fn ini_rev(self) -> crate::common::Reg<regs::IniRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "The destined speed register for position initialization."]
    #[inline(always)]
    pub const fn ini_speed(self) -> crate::common::Reg<regs::IniSpeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "The destined accelerator register for position initialization."]
    #[inline(always)]
    pub const fn ini_accel(self) -> crate::common::Reg<regs::IniAccel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "The destined timestamp register for coefficients initialization."]
    #[inline(always)]
    pub const fn ini_coef_time(self) -> crate::common::Reg<regs::IniCoefTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "The destined coefficient P register for coefficients initialization."]
    #[inline(always)]
    pub const fn ini_pcoef(self) -> crate::common::Reg<regs::IniPcoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "The destined coefficient I register for coefficients initialization."]
    #[inline(always)]
    pub const fn ini_icoef(self) -> crate::common::Reg<regs::IniIcoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "The destined coefficient A register for coefficients initialization."]
    #[inline(always)]
    pub const fn ini_acoef(self) -> crate::common::Reg<regs::IniAcoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "The timestamp register for internal estimation."]
    #[inline(always)]
    pub const fn estm_tim(self) -> crate::common::Reg<regs::EstmTim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "The position register for the internal estimation."]
    #[inline(always)]
    pub const fn estm_pos(self) -> crate::common::Reg<regs::EstmPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "The revolution register for the internal estimation."]
    #[inline(always)]
    pub const fn estm_rev(self) -> crate::common::Reg<regs::EstmRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "The speed register for the internal estimation."]
    #[inline(always)]
    pub const fn estm_speed(self) -> crate::common::Reg<regs::EstmSpeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "The accelerator register for theinternal estimation."]
    #[inline(always)]
    pub const fn estm_accel(self) -> crate::common::Reg<regs::EstmAccel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "The coefficient P register for the internal estimation."]
    #[inline(always)]
    pub const fn cur_pcoef(self) -> crate::common::Reg<regs::CurPcoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "The coefficient I register for the internal estimation."]
    #[inline(always)]
    pub const fn cur_icoef(self) -> crate::common::Reg<regs::CurIcoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "The coefficient A register for the internal estimation."]
    #[inline(always)]
    pub const fn cur_acoef(self) -> crate::common::Reg<regs::CurAcoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "The destined timestamp register for delta position initialization."]
    #[inline(always)]
    pub const fn ini_delta_pos_time(
        self,
    ) -> crate::common::Reg<regs::IniDeltaPosTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "The destined delta position register for delta position initialization."]
    #[inline(always)]
    pub const fn ini_delta_pos(self) -> crate::common::Reg<regs::IniDeltaPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "The destined delta revolution register for delta position initialization."]
    #[inline(always)]
    pub const fn ini_delta_rev(self) -> crate::common::Reg<regs::IniDeltaRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "The destined delta speed register for delta position initialization."]
    #[inline(always)]
    pub const fn ini_delta_speed(
        self,
    ) -> crate::common::Reg<regs::IniDeltaSpeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "The destined delta accelerator register for delta position initialization."]
    #[inline(always)]
    pub const fn ini_delta_accel(
        self,
    ) -> crate::common::Reg<regs::IniDeltaAccel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Tracking Configuration pos trigger cfg."]
    #[inline(always)]
    pub const fn pos_trg_cfg(self) -> crate::common::Reg<regs::PosTrgCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Tracking Configuration position threshold."]
    #[inline(always)]
    pub const fn pos_trg_pos_thr(
        self,
    ) -> crate::common::Reg<regs::PosTrgPosThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Tracking Configuration revolution threshold."]
    #[inline(always)]
    pub const fn pos_trg_rev_thr(
        self,
    ) -> crate::common::Reg<regs::PosTrgRevThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Tracking Configuration speed trigger cfg."]
    #[inline(always)]
    pub const fn speed_trg_cfg(self) -> crate::common::Reg<regs::SpeedTrgCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Tracking Configuration speed threshold."]
    #[inline(always)]
    pub const fn speed_trg_thr(self) -> crate::common::Reg<regs::SpeedTrgThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn coef_trg_cfg(self, n: usize) -> CoefTrgCfg {
        assert!(n < 3usize);
        unsafe { CoefTrgCfg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 20usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn br(self, n: usize) -> Br {
        assert!(n < 2usize);
        unsafe { Br::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 256usize) as _) }
    }
    #[doc = "Monitor of the just received input timestamp for tracing logic."]
    #[inline(always)]
    pub const fn bk0_timestamp(self) -> crate::common::Reg<regs::Bk0Timestamp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Monitor of the just received input position for tracing logic."]
    #[inline(always)]
    pub const fn bk0_position(self) -> crate::common::Reg<regs::Bk0Position, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Monitor of the just received input revolution for tracing logic."]
    #[inline(always)]
    pub const fn bk0_revolution(
        self,
    ) -> crate::common::Reg<regs::Bk0Revolution, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "Monitor of the just received input speed for tracing logic."]
    #[inline(always)]
    pub const fn bk0_speed(self) -> crate::common::Reg<regs::Bk0Speed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "Monitor of the just received input acceleration for tracing logic."]
    #[inline(always)]
    pub const fn bk0_accelerator(
        self,
    ) -> crate::common::Reg<regs::Bk0Accelerator, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "Monitor of the previous received input timestamp for tracing logic."]
    #[inline(always)]
    pub const fn bk1_timestamp(self) -> crate::common::Reg<regs::Bk1Timestamp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Monitor of the previous received input position for tracing logic."]
    #[inline(always)]
    pub const fn bk1_position(self) -> crate::common::Reg<regs::Bk1Position, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "Monitor of the previous received input revolution for tracing logic."]
    #[inline(always)]
    pub const fn bk1_revolution(
        self,
    ) -> crate::common::Reg<regs::Bk1Revolution, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "Monitor of the previous received input speed for tracing logic."]
    #[inline(always)]
    pub const fn bk1_speed(self) -> crate::common::Reg<regs::Bk1Speed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "Monitor of the previous received input acceleration for tracing logic."]
    #[inline(always)]
    pub const fn bk1_accelerator(
        self,
    ) -> crate::common::Reg<regs::Bk1Accelerator, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
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
    #[doc = "Tracking Configuration coef trigger cfg&index0 A."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct A(pub u32);
    impl A {
        #[doc = "A0_Coef，fix<32, 19>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "A0_Coef，fix<32, 19>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for A {
        #[inline(always)]
        fn default() -> A {
            A(0)
        }
    }
    impl core::fmt::Debug for A {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("A").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for A {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "A {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the just received input acceleration for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk0Accelerator(pub u32);
    impl Bk0Accelerator {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk0Accelerator {
        #[inline(always)]
        fn default() -> Bk0Accelerator {
            Bk0Accelerator(0)
        }
    }
    impl core::fmt::Debug for Bk0Accelerator {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bk0Accelerator")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bk0Accelerator {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bk0Accelerator {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the just received input position for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk0Position(pub u32);
    impl Bk0Position {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk0Position {
        #[inline(always)]
        fn default() -> Bk0Position {
            Bk0Position(0)
        }
    }
    impl core::fmt::Debug for Bk0Position {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bk0Position")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bk0Position {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bk0Position {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the just received input revolution for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk0Revolution(pub u32);
    impl Bk0Revolution {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk0Revolution {
        #[inline(always)]
        fn default() -> Bk0Revolution {
            Bk0Revolution(0)
        }
    }
    impl core::fmt::Debug for Bk0Revolution {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bk0Revolution")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bk0Revolution {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bk0Revolution {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the just received input speed for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk0Speed(pub u32);
    impl Bk0Speed {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk0Speed {
        #[inline(always)]
        fn default() -> Bk0Speed {
            Bk0Speed(0)
        }
    }
    impl core::fmt::Debug for Bk0Speed {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bk0Speed")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bk0Speed {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bk0Speed {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the just received input timestamp for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk0Timestamp(pub u32);
    impl Bk0Timestamp {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk0Timestamp {
        #[inline(always)]
        fn default() -> Bk0Timestamp {
            Bk0Timestamp(0)
        }
    }
    impl core::fmt::Debug for Bk0Timestamp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bk0Timestamp")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bk0Timestamp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bk0Timestamp {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the previous received input acceleration for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk1Accelerator(pub u32);
    impl Bk1Accelerator {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk1Accelerator {
        #[inline(always)]
        fn default() -> Bk1Accelerator {
            Bk1Accelerator(0)
        }
    }
    impl core::fmt::Debug for Bk1Accelerator {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bk1Accelerator")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bk1Accelerator {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bk1Accelerator {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the previous received input position for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk1Position(pub u32);
    impl Bk1Position {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk1Position {
        #[inline(always)]
        fn default() -> Bk1Position {
            Bk1Position(0)
        }
    }
    impl core::fmt::Debug for Bk1Position {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bk1Position")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bk1Position {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bk1Position {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the previous received input revolution for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk1Revolution(pub u32);
    impl Bk1Revolution {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk1Revolution {
        #[inline(always)]
        fn default() -> Bk1Revolution {
            Bk1Revolution(0)
        }
    }
    impl core::fmt::Debug for Bk1Revolution {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bk1Revolution")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bk1Revolution {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bk1Revolution {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the previous received input speed for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk1Speed(pub u32);
    impl Bk1Speed {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk1Speed {
        #[inline(always)]
        fn default() -> Bk1Speed {
            Bk1Speed(0)
        }
    }
    impl core::fmt::Debug for Bk1Speed {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bk1Speed")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bk1Speed {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bk1Speed {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the previous received input timestamp for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk1Timestamp(pub u32);
    impl Bk1Timestamp {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk1Timestamp {
        #[inline(always)]
        fn default() -> Bk1Timestamp {
            Bk1Timestamp(0)
        }
    }
    impl core::fmt::Debug for Bk1Timestamp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bk1Timestamp")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bk1Timestamp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bk1Timestamp {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Prediction Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrCtrl(pub u32);
    impl BrCtrl {
        #[doc = "Branch Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn br_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Branch Enable."]
        #[inline(always)]
        pub const fn set_br_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "1. First trigger by external trigger pin 0. First trigger by the timer When in CR\\[MANUAL_IO\\]=1 mode, it is the prediction trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn f_trg_type(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1. First trigger by external trigger pin 0. First trigger by the timer When in CR\\[MANUAL_IO\\]=1 mode, it is the prediction trigger."]
        #[inline(always)]
        pub const fn set_f_trg_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "1. Each non-first trigger by external trigger pin 0. Each non-first trigger by the timer."]
        #[must_use]
        #[inline(always)]
        pub const fn nf_trg_type(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "1. Each non-first trigger by external trigger pin 0. Each non-first trigger by the timer."]
        #[inline(always)]
        pub const fn set_nf_trg_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "1:continuously repeat pred, 0:cal the pred based on a definite time-stamp offset, 2:programed one-shot prediction mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pred_mode(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "1:continuously repeat pred, 0:cal the pred based on a definite time-stamp offset, 2:programed one-shot prediction mode."]
        #[inline(always)]
        pub const fn set_pred_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "1: in open loop mode 0: not in open loop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn open_loop_mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "1: in open loop mode 0: not in open loop mode."]
        #[inline(always)]
        pub const fn set_open_loop_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "1: Command to reload the delta pos. Auto clear 0:."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_delta_pos_req(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1: Command to reload the delta pos. Auto clear 0:."]
        #[inline(always)]
        pub const fn set_ini_delta_pos_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "1: change 0: won't change bit 3: for delta accel bit 2: for delta speed bit 1: for delta revolution bit 0: for delta position."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_delta_pos_cmd_msk(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "1: change 0: won't change bit 3: for delta accel bit 2: for delta speed bit 1: for delta revolution bit 0: for delta position."]
        #[inline(always)]
        pub const fn set_ini_delta_pos_cmd_msk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "Interrupt Enable for INI_DELTA_POS_DONE."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_delta_pos_done_ie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for INI_DELTA_POS_DONE."]
        #[inline(always)]
        pub const fn set_ini_delta_pos_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "0: Time Stamp in the configuration 1: Risedge of In Trg\\[0\\]
2: Risedge of In Trg\\[1\\]
3: Risedge of out trg\\[0\\]
4: Risedge of out trg\\[1\\]
5: Risedge of self pos trigger 6: Risedge of self speed trigger Others: no function."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_delta_pos_trg_type(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "0: Time Stamp in the configuration 1: Risedge of In Trg\\[0\\]
2: Risedge of In Trg\\[1\\]
3: Risedge of out trg\\[0\\]
4: Risedge of out trg\\[1\\]
5: Risedge of self pos trigger 6: Risedge of self speed trigger Others: no function."]
        #[inline(always)]
        pub const fn set_ini_delta_pos_trg_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "1: change 0: won't change bit 3: for accel bit 2: for speed bit 1: for revolution bit 0: for position."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_pos_cmd_msk(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[doc = "1: change 0: won't change bit 3: for accel bit 2: for speed bit 1: for revolution bit 0: for position."]
        #[inline(always)]
        pub const fn set_ini_pos_cmd_msk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[doc = "0: Time Stamp in the configuration 1: Risedge of In Trg\\[0\\]
2: Risedge of In Trg\\[1\\]
3: Risedge of out trg\\[0\\]
4: Risedge of out trg\\[1\\]
5: Risedge of self pos trigger 6: Risedge of self speed trigger Others: no function."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_pos_trg_type(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x07;
            val as u8
        }
        #[doc = "0: Time Stamp in the configuration 1: Risedge of In Trg\\[0\\]
2: Risedge of In Trg\\[1\\]
3: Risedge of out trg\\[0\\]
4: Risedge of out trg\\[1\\]
5: Risedge of self pos trigger 6: Risedge of self speed trigger Others: no function."]
        #[inline(always)]
        pub const fn set_ini_pos_trg_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
        }
        #[doc = "Interrupt Enable for POS_TRG_VALID."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_trg_valid_ie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for POS_TRG_VALID."]
        #[inline(always)]
        pub const fn set_pos_trg_valid_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Interrupt Enable for SPEED_TRG_VALID."]
        #[must_use]
        #[inline(always)]
        pub const fn speed_trg_valid_ie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for SPEED_TRG_VALID."]
        #[inline(always)]
        pub const fn set_speed_trg_valid_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for BrCtrl {
        #[inline(always)]
        fn default() -> BrCtrl {
            BrCtrl(0)
        }
    }
    impl core::fmt::Debug for BrCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrCtrl")
                .field("br_en", &self.br_en())
                .field("f_trg_type", &self.f_trg_type())
                .field("nf_trg_type", &self.nf_trg_type())
                .field("pred_mode", &self.pred_mode())
                .field("open_loop_mode", &self.open_loop_mode())
                .field("ini_delta_pos_req", &self.ini_delta_pos_req())
                .field("ini_delta_pos_cmd_msk", &self.ini_delta_pos_cmd_msk())
                .field("ini_delta_pos_done_ie", &self.ini_delta_pos_done_ie())
                .field("ini_delta_pos_trg_type", &self.ini_delta_pos_trg_type())
                .field("ini_pos_cmd_msk", &self.ini_pos_cmd_msk())
                .field("ini_pos_trg_type", &self.ini_pos_trg_type())
                .field("pos_trg_valid_ie", &self.pos_trg_valid_ie())
                .field("speed_trg_valid_ie", &self.speed_trg_valid_ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BrCtrl {{ br_en: {=bool:?}, f_trg_type: {=bool:?}, nf_trg_type: {=bool:?}, pred_mode: {=u8:?}, open_loop_mode: {=bool:?}, ini_delta_pos_req: {=bool:?}, ini_delta_pos_cmd_msk: {=u8:?}, ini_delta_pos_done_ie: {=bool:?}, ini_delta_pos_trg_type: {=u8:?}, ini_pos_cmd_msk: {=u8:?}, ini_pos_trg_type: {=u8:?}, pos_trg_valid_ie: {=bool:?}, speed_trg_valid_ie: {=bool:?} }}" , self . br_en () , self . f_trg_type () , self . nf_trg_type () , self . pred_mode () , self . open_loop_mode () , self . ini_delta_pos_req () , self . ini_delta_pos_cmd_msk () , self . ini_delta_pos_done_ie () , self . ini_delta_pos_trg_type () , self . ini_pos_cmd_msk () , self . ini_pos_trg_type () , self . pos_trg_valid_ie () , self . speed_trg_valid_ie ())
        }
    }
    #[doc = "Monitor of the output acceleration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrCurAccel(pub u32);
    impl BrCurAccel {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrCurAccel {
        #[inline(always)]
        fn default() -> BrCurAccel {
            BrCurAccel(0)
        }
    }
    impl core::fmt::Debug for BrCurAccel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrCurAccel")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrCurAccel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrCurAccel {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the output position."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrCurPos(pub u32);
    impl BrCurPos {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrCurPos {
        #[inline(always)]
        fn default() -> BrCurPos {
            BrCurPos(0)
        }
    }
    impl core::fmt::Debug for BrCurPos {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrCurPos")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrCurPos {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrCurPos {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the output timestamp."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrCurPosTime(pub u32);
    impl BrCurPosTime {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrCurPosTime {
        #[inline(always)]
        fn default() -> BrCurPosTime {
            BrCurPosTime(0)
        }
    }
    impl core::fmt::Debug for BrCurPosTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrCurPosTime")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrCurPosTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrCurPosTime {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the output revolution."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrCurRev(pub u32);
    impl BrCurRev {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrCurRev {
        #[inline(always)]
        fn default() -> BrCurRev {
            BrCurRev(0)
        }
    }
    impl core::fmt::Debug for BrCurRev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrCurRev")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrCurRev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrCurRev {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Monitor of the output speed."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrCurSpeed(pub u32);
    impl BrCurSpeed {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrCurSpeed {
        #[inline(always)]
        fn default() -> BrCurSpeed {
            BrCurSpeed(0)
        }
    }
    impl core::fmt::Debug for BrCurSpeed {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrCurSpeed")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrCurSpeed {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrCurSpeed {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Initialization acceleration for open-loop mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniAccel(pub u32);
    impl BrIniAccel {
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniAccel {
        #[inline(always)]
        fn default() -> BrIniAccel {
            BrIniAccel(0)
        }
    }
    impl core::fmt::Debug for BrIniAccel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrIniAccel")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrIniAccel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrIniAccel {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Initialization delta acceleration for delta mode in prediction mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniDeltaAccel(pub u32);
    impl BrIniDeltaAccel {
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniDeltaAccel {
        #[inline(always)]
        fn default() -> BrIniDeltaAccel {
            BrIniDeltaAccel(0)
        }
    }
    impl core::fmt::Debug for BrIniDeltaAccel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrIniDeltaAccel")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrIniDeltaAccel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrIniDeltaAccel {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Initialization delta position for delta mode in prediction mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniDeltaPos(pub u32);
    impl BrIniDeltaPos {
        #[doc = "the value continuous mode: ufix<32, 32>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: ufix<32, 32>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniDeltaPos {
        #[inline(always)]
        fn default() -> BrIniDeltaPos {
            BrIniDeltaPos(0)
        }
    }
    impl core::fmt::Debug for BrIniDeltaPos {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrIniDeltaPos")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrIniDeltaPos {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrIniDeltaPos {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Initialization timestamp for delta mode in prediction mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniDeltaPosTime(pub u32);
    impl BrIniDeltaPosTime {
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniDeltaPosTime {
        #[inline(always)]
        fn default() -> BrIniDeltaPosTime {
            BrIniDeltaPosTime(0)
        }
    }
    impl core::fmt::Debug for BrIniDeltaPosTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrIniDeltaPosTime")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrIniDeltaPosTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrIniDeltaPosTime {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Initialization delta revolution for delta mode in prediction mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniDeltaRev(pub u32);
    impl BrIniDeltaRev {
        #[doc = "the value continuous mode: fix<32, 0>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 0>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniDeltaRev {
        #[inline(always)]
        fn default() -> BrIniDeltaRev {
            BrIniDeltaRev(0)
        }
    }
    impl core::fmt::Debug for BrIniDeltaRev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrIniDeltaRev")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrIniDeltaRev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrIniDeltaRev {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Initialization delta speed for delta mode in prediction mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniDeltaSpeed(pub u32);
    impl BrIniDeltaSpeed {
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniDeltaSpeed {
        #[inline(always)]
        fn default() -> BrIniDeltaSpeed {
            BrIniDeltaSpeed(0)
        }
    }
    impl core::fmt::Debug for BrIniDeltaSpeed {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrIniDeltaSpeed")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrIniDeltaSpeed {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrIniDeltaSpeed {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Initialization position for open-loop mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniPos(pub u32);
    impl BrIniPos {
        #[doc = "the value ufix<32, 32>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value ufix<32, 32>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniPos {
        #[inline(always)]
        fn default() -> BrIniPos {
            BrIniPos(0)
        }
    }
    impl core::fmt::Debug for BrIniPos {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrIniPos")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrIniPos {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrIniPos {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Initialization timestamp for open-loop mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniPosTime(pub u32);
    impl BrIniPosTime {
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniPosTime {
        #[inline(always)]
        fn default() -> BrIniPosTime {
            BrIniPosTime(0)
        }
    }
    impl core::fmt::Debug for BrIniPosTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrIniPosTime")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrIniPosTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrIniPosTime {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Initialization revolution for open-loop mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniRev(pub u32);
    impl BrIniRev {
        #[doc = "the value ufix<32, 0>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value ufix<32, 0>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniRev {
        #[inline(always)]
        fn default() -> BrIniRev {
            BrIniRev(0)
        }
    }
    impl core::fmt::Debug for BrIniRev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrIniRev")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrIniRev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrIniRev {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Initialization speed for open-loop mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniSpeed(pub u32);
    impl BrIniSpeed {
        #[doc = "the value fix<32, 19>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value fix<32, 19>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniSpeed {
        #[inline(always)]
        fn default() -> BrIniSpeed {
            BrIniSpeed(0)
        }
    }
    impl core::fmt::Debug for BrIniSpeed {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrIniSpeed")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrIniSpeed {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrIniSpeed {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Prediction Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrSt(pub u32);
    impl BrSt {
        #[doc = "The module's error ID output."]
        #[must_use]
        #[inline(always)]
        pub const fn err_id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The module's error ID output."]
        #[inline(always)]
        pub const fn set_err_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "1: The prediction module is idle. 0: The prediction module is not idle."]
        #[must_use]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "1: The prediction module is idle. 0: The prediction module is not idle."]
        #[inline(always)]
        pub const fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "1: the initialization of delta position command is done 0: the initialization of delta position command is not done."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_delta_pos_done(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "1: the initialization of delta position command is done 0: the initialization of delta position command is not done."]
        #[inline(always)]
        pub const fn set_ini_delta_pos_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "1：self position trigger event found 0：self position trigger event not found yet."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_trg_vld(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1：self position trigger event found 0：self position trigger event not found yet."]
        #[inline(always)]
        pub const fn set_pos_trg_vld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "1：self speed trigger event found 0：self speed trigger event not found yet."]
        #[must_use]
        #[inline(always)]
        pub const fn speed_trg_vld(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "1：self speed trigger event found 0：self speed trigger event not found yet."]
        #[inline(always)]
        pub const fn set_speed_trg_vld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "1：in open loop mode 0：in closed loop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn open_loop_st(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "1：in open loop mode 0：in closed loop mode."]
        #[inline(always)]
        pub const fn set_open_loop_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for BrSt {
        #[inline(always)]
        fn default() -> BrSt {
            BrSt(0)
        }
    }
    impl core::fmt::Debug for BrSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrSt")
                .field("err_id", &self.err_id())
                .field("idle", &self.idle())
                .field("ini_delta_pos_done", &self.ini_delta_pos_done())
                .field("pos_trg_vld", &self.pos_trg_vld())
                .field("speed_trg_vld", &self.speed_trg_vld())
                .field("open_loop_st", &self.open_loop_st())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrSt {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BrSt {{ err_id: {=u8:?}, idle: {=bool:?}, ini_delta_pos_done: {=bool:?}, pos_trg_vld: {=bool:?}, speed_trg_vld: {=bool:?}, open_loop_st: {=bool:?} }}" , self . err_id () , self . idle () , self . ini_delta_pos_done () , self . pos_trg_vld () , self . speed_trg_vld () , self . open_loop_st ())
        }
    }
    #[doc = "Prediction Timing Offset Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTimeoff(pub u32);
    impl BrTimeoff {
        #[doc = "ufix<32, 0> time offset incycles from the trigger time."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ufix<32, 0> time offset incycles from the trigger time."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrTimeoff {
        #[inline(always)]
        fn default() -> BrTimeoff {
            BrTimeoff(0)
        }
    }
    impl core::fmt::Debug for BrTimeoff {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrTimeoff")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrTimeoff {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrTimeoff {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Prediction Triggering First Offset Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgFTime(pub u32);
    impl BrTrgFTime {
        #[doc = "uifx<32, 0> the time for the first trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "uifx<32, 0> the time for the first trigger."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrTrgFTime {
        #[inline(always)]
        fn default() -> BrTrgFTime {
            BrTrgFTime(0)
        }
    }
    impl core::fmt::Debug for BrTrgFTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrTrgFTime")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrTrgFTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrTrgFTime {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Prediction Triggering Period Offset Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgPeriod(pub u32);
    impl BrTrgPeriod {
        #[doc = "uifx<32, 0>, time offset incycles between each trigger time."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "uifx<32, 0>, time offset incycles between each trigger time."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrTrgPeriod {
        #[inline(always)]
        fn default() -> BrTrgPeriod {
            BrTrgPeriod(0)
        }
    }
    impl core::fmt::Debug for BrTrgPeriod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrTrgPeriod")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrTrgPeriod {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrTrgPeriod {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Prediction Configuration postion trigger cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgPosCfg(pub u32);
    impl BrTrgPosCfg {
        #[doc = "1-trigger valid; 0-Trigger not valid."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1-trigger valid; 0-Trigger not valid."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "bit1: 0: (rising edge) pos inc greater than, 1: (falling edge) pos dec less than."]
        #[must_use]
        #[inline(always)]
        pub const fn edge(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "bit1: 0: (rising edge) pos inc greater than, 1: (falling edge) pos dec less than."]
        #[inline(always)]
        pub const fn set_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for BrTrgPosCfg {
        #[inline(always)]
        fn default() -> BrTrgPosCfg {
            BrTrgPosCfg(0)
        }
    }
    impl core::fmt::Debug for BrTrgPosCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrTrgPosCfg")
                .field("en", &self.en())
                .field("edge", &self.edge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrTrgPosCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BrTrgPosCfg {{ en: {=bool:?}, edge: {=bool:?} }}",
                self.en(),
                self.edge()
            )
        }
    }
    #[doc = "Prediction Configuration postion threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgPosThr(pub u32);
    impl BrTrgPosThr {
        #[doc = "For pos out trigger (pos). ufix<32, 32>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "For pos out trigger (pos). ufix<32, 32>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrTrgPosThr {
        #[inline(always)]
        fn default() -> BrTrgPosThr {
            BrTrgPosThr(0)
        }
    }
    impl core::fmt::Debug for BrTrgPosThr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrTrgPosThr")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrTrgPosThr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrTrgPosThr {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Prediction Configuration revolutiom threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgRevThr(pub u32);
    impl BrTrgRevThr {
        #[doc = "For pos out trigger (rev) ufix<32, 0>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "For pos out trigger (rev) ufix<32, 0>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrTrgRevThr {
        #[inline(always)]
        fn default() -> BrTrgRevThr {
            BrTrgRevThr(0)
        }
    }
    impl core::fmt::Debug for BrTrgRevThr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrTrgRevThr")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrTrgRevThr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrTrgRevThr {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Prediction Configuration speed trigger cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgSpeedCfg(pub u32);
    impl BrTrgSpeedCfg {
        #[doc = "1-trigger valid; 0-Trigger not valid Normally it means either the max pos speed, or the min negative speed."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1-trigger valid; 0-Trigger not valid Normally it means either the max pos speed, or the min negative speed."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0: (rising edge) speed inc greater than, 1: (falling edge) speed dec less than."]
        #[must_use]
        #[inline(always)]
        pub const fn edge_sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0: (rising edge) speed inc greater than, 1: (falling edge) speed dec less than."]
        #[inline(always)]
        pub const fn set_edge_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Use abs value for comparion. 0: Use the speed with direction info (so not the abs value)."]
        #[must_use]
        #[inline(always)]
        pub const fn comp_type(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Use abs value for comparion. 0: Use the speed with direction info (so not the abs value)."]
        #[inline(always)]
        pub const fn set_comp_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for BrTrgSpeedCfg {
        #[inline(always)]
        fn default() -> BrTrgSpeedCfg {
            BrTrgSpeedCfg(0)
        }
    }
    impl core::fmt::Debug for BrTrgSpeedCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrTrgSpeedCfg")
                .field("en", &self.en())
                .field("edge_sel", &self.edge_sel())
                .field("comp_type", &self.comp_type())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrTrgSpeedCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BrTrgSpeedCfg {{ en: {=bool:?}, edge_sel: {=bool:?}, comp_type: {=bool:?} }}",
                self.en(),
                self.edge_sel(),
                self.comp_type()
            )
        }
    }
    #[doc = "Prediction Configuration speed threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgSpeedThr(pub u32);
    impl BrTrgSpeedThr {
        #[doc = "For speed trigger. continuous mode: fix<32, 19>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "For speed trigger. continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrTrgSpeedThr {
        #[inline(always)]
        fn default() -> BrTrgSpeedThr {
            BrTrgSpeedThr(0)
        }
    }
    impl core::fmt::Debug for BrTrgSpeedThr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BrTrgSpeedThr")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BrTrgSpeedThr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BrTrgSpeedThr {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Continuous Mode Configuration 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ContCfg0(pub u32);
    impl ContCfg0 {
        #[doc = "the theta for cal the clockwise or anticlockwise rotation between two adjacent inputs, ufix<32, 32>."]
        #[must_use]
        #[inline(always)]
        pub const fn half_circ_theta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the theta for cal the clockwise or anticlockwise rotation between two adjacent inputs, ufix<32, 32>."]
        #[inline(always)]
        pub const fn set_half_circ_theta(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ContCfg0 {
        #[inline(always)]
        fn default() -> ContCfg0 {
            ContCfg0(0)
        }
    }
    impl core::fmt::Debug for ContCfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ContCfg0")
                .field("half_circ_theta", &self.half_circ_theta())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ContCfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ContCfg0 {{ half_circ_theta: {=u32:?} }}",
                self.half_circ_theta()
            )
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Module Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mod_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Module Enable."]
        #[inline(always)]
        pub const fn set_mod_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "1: Discrete position input 0: Continuous position input."]
        #[must_use]
        #[inline(always)]
        pub const fn discretetrc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: Discrete position input 0: Continuous position input."]
        #[inline(always)]
        pub const fn set_discretetrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "1: use the input iposition whenever a new iposition comes, and force the predicted output stop at the boundaries. 0: Continuous tracking mode, without any boundary check."]
        #[must_use]
        #[inline(always)]
        pub const fn adjop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "1: use the input iposition whenever a new iposition comes, and force the predicted output stop at the boundaries. 0: Continuous tracking mode, without any boundary check."]
        #[inline(always)]
        pub const fn set_adjop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "1: Shadow Request for read of tracking parameters. Auto clear 0:."]
        #[must_use]
        #[inline(always)]
        pub const fn shadow_rd_req(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "1: Shadow Request for read of tracking parameters. Auto clear 0:."]
        #[inline(always)]
        pub const fn set_shadow_rd_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "1: Command to reload the coefs. Auto clear 0:."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_coefs_cmd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "1: Command to reload the coefs. Auto clear 0:."]
        #[inline(always)]
        pub const fn set_ini_coefs_cmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "1: change 0: won't change bit 2: for ACOEF bit 1: for ICOEF bit 0: for PCOEF."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_coefs_cmd_msk(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "1: change 0: won't change bit 2: for ACOEF bit 1: for ICOEF bit 0: for PCOEF."]
        #[inline(always)]
        pub const fn set_ini_coefs_cmd_msk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "1: Command to reload the positions. Auto clear 0:."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_pos_req(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1: Command to reload the positions. Auto clear 0:."]
        #[inline(always)]
        pub const fn set_ini_pos_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "1: change 0: won't change bit 3: for accel bit 2: for speed bit 1: for revolution bit 0: for position."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_pos_cmd_msk(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "1: change 0: won't change bit 3: for accel bit 2: for speed bit 1: for revolution bit 0: for position."]
        #[inline(always)]
        pub const fn set_ini_pos_cmd_msk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "1: 32-bit for rev+pos, with each element occupying 16 bits 0: 32-bit for rev, and 32 bit for pos When CR\\[MANUAL_IO\\]==1, 1: means that the INI_POS is acting as INI_POS cmds 0: means that the INI_POS is simulating the input of iposition and itimestamp."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_type(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "1: 32-bit for rev+pos, with each element occupying 16 bits 0: 32-bit for rev, and 32 bit for pos When CR\\[MANUAL_IO\\]==1, 1: means that the INI_POS is acting as INI_POS cmds 0: means that the INI_POS is simulating the input of iposition and itimestamp."]
        #[inline(always)]
        pub const fn set_pos_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "1: in open loop mode 0: not in open loop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn open_loop_mode(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "1: in open loop mode 0: not in open loop mode."]
        #[inline(always)]
        pub const fn set_open_loop_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "1: Command to reload the delta pos. Auto clear 0:."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_delta_pos_req(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "1: Command to reload the delta pos. Auto clear 0:."]
        #[inline(always)]
        pub const fn set_ini_delta_pos_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "1: change 0: won't change bit 3: for delta accel bit 2: for delta speed bit 1: for delta revolution bit 0: for delta position."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_delta_pos_cmd_msk(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "1: change 0: won't change bit 3: for delta accel bit 2: for delta speed bit 1: for delta revolution bit 0: for delta position."]
        #[inline(always)]
        pub const fn set_ini_delta_pos_cmd_msk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "0: Time Stamp in the configuration 1: Risedge of In Trg\\[0\\]
2: Risedge of In Trg\\[1\\]
3: Risedge of out trg\\[0\\]
4: Risedge of out trg\\[1\\]
5: triggered by self position trigger 6: triggered by self speed trigger Otherser: no function."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_pos_trg_type(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "0: Time Stamp in the configuration 1: Risedge of In Trg\\[0\\]
2: Risedge of In Trg\\[1\\]
3: Risedge of out trg\\[0\\]
4: Risedge of out trg\\[1\\]
5: triggered by self position trigger 6: triggered by self speed trigger Otherser: no function."]
        #[inline(always)]
        pub const fn set_ini_pos_trg_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "0: Time Stamp in the configuration 1: Risedge of In Trg\\[0\\]
2: Risedge of In Trg\\[1\\]
3: Risedge of out trg\\[0\\]
4: Risedge of out trg\\[1\\]
5: triggered by self position trigger 6: triggered by self speed trigger Otherser: no function."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_delta_pos_trg_type(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x07;
            val as u8
        }
        #[doc = "0: Time Stamp in the configuration 1: Risedge of In Trg\\[0\\]
2: Risedge of In Trg\\[1\\]
3: Risedge of out trg\\[0\\]
4: Risedge of out trg\\[1\\]
5: triggered by self position trigger 6: triggered by self speed trigger Otherser: no function."]
        #[inline(always)]
        pub const fn set_ini_delta_pos_trg_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
        }
        #[doc = "Multiple Coefficients Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ms_coef_en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Multiple Coefficients Enable."]
        #[inline(always)]
        pub const fn set_ms_coef_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Zeroise the accelerator calculation."]
        #[must_use]
        #[inline(always)]
        pub const fn frcaccelzero(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Zeroise the accelerator calculation."]
        #[inline(always)]
        pub const fn set_frcaccelzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Auto clear. Only effective in open_loop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_br1_pos_req(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Auto clear. Only effective in open_loop mode."]
        #[inline(always)]
        pub const fn set_ini_br1_pos_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Auto clear. Only effective in open_loop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_br0_pos_req(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Auto clear. Only effective in open_loop mode."]
        #[inline(always)]
        pub const fn set_ini_br0_pos_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Software reset, high active. When write 1 ,all internal logical will be reset. 0b - No action 1b - All MMC internal registers are forced into their reset state. Interface registers are not affected."]
        #[must_use]
        #[inline(always)]
        pub const fn sftrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset, high active. When write 1 ,all internal logical will be reset. 0b - No action 1b - All MMC internal registers are forced into their reset state. Interface registers are not affected."]
        #[inline(always)]
        pub const fn set_sftrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("mod_en", &self.mod_en())
                .field("discretetrc", &self.discretetrc())
                .field("adjop", &self.adjop())
                .field("shadow_rd_req", &self.shadow_rd_req())
                .field("ini_coefs_cmd", &self.ini_coefs_cmd())
                .field("ini_coefs_cmd_msk", &self.ini_coefs_cmd_msk())
                .field("ini_pos_req", &self.ini_pos_req())
                .field("ini_pos_cmd_msk", &self.ini_pos_cmd_msk())
                .field("pos_type", &self.pos_type())
                .field("open_loop_mode", &self.open_loop_mode())
                .field("ini_delta_pos_req", &self.ini_delta_pos_req())
                .field("ini_delta_pos_cmd_msk", &self.ini_delta_pos_cmd_msk())
                .field("ini_pos_trg_type", &self.ini_pos_trg_type())
                .field("ini_delta_pos_trg_type", &self.ini_delta_pos_trg_type())
                .field("ms_coef_en", &self.ms_coef_en())
                .field("frcaccelzero", &self.frcaccelzero())
                .field("ini_br1_pos_req", &self.ini_br1_pos_req())
                .field("ini_br0_pos_req", &self.ini_br0_pos_req())
                .field("sftrst", &self.sftrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ mod_en: {=bool:?}, discretetrc: {=bool:?}, adjop: {=bool:?}, shadow_rd_req: {=bool:?}, ini_coefs_cmd: {=bool:?}, ini_coefs_cmd_msk: {=u8:?}, ini_pos_req: {=bool:?}, ini_pos_cmd_msk: {=u8:?}, pos_type: {=bool:?}, open_loop_mode: {=bool:?}, ini_delta_pos_req: {=bool:?}, ini_delta_pos_cmd_msk: {=u8:?}, ini_pos_trg_type: {=u8:?}, ini_delta_pos_trg_type: {=u8:?}, ms_coef_en: {=bool:?}, frcaccelzero: {=bool:?}, ini_br1_pos_req: {=bool:?}, ini_br0_pos_req: {=bool:?}, sftrst: {=bool:?} }}" , self . mod_en () , self . discretetrc () , self . adjop () , self . shadow_rd_req () , self . ini_coefs_cmd () , self . ini_coefs_cmd_msk () , self . ini_pos_req () , self . ini_pos_cmd_msk () , self . pos_type () , self . open_loop_mode () , self . ini_delta_pos_req () , self . ini_delta_pos_cmd_msk () , self . ini_pos_trg_type () , self . ini_delta_pos_trg_type () , self . ms_coef_en () , self . frcaccelzero () , self . ini_br1_pos_req () , self . ini_br0_pos_req () , self . sftrst ())
        }
    }
    #[doc = "The coefficient A register for the internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CurAcoef(pub u32);
    impl CurAcoef {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CurAcoef {
        #[inline(always)]
        fn default() -> CurAcoef {
            CurAcoef(0)
        }
    }
    impl core::fmt::Debug for CurAcoef {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CurAcoef")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CurAcoef {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CurAcoef {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The coefficient I register for the internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CurIcoef(pub u32);
    impl CurIcoef {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CurIcoef {
        #[inline(always)]
        fn default() -> CurIcoef {
            CurIcoef(0)
        }
    }
    impl core::fmt::Debug for CurIcoef {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CurIcoef")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CurIcoef {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CurIcoef {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The coefficient P register for the internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CurPcoef(pub u32);
    impl CurPcoef {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CurPcoef {
        #[inline(always)]
        fn default() -> CurPcoef {
            CurPcoef(0)
        }
    }
    impl core::fmt::Debug for CurPcoef {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CurPcoef")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CurPcoef {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CurPcoef {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Discrete Mode Configuration 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DiscreteCfg0(pub u32);
    impl DiscreteCfg0 {
        #[doc = "Max ID Of Lines. For example-1, for 512 lines, it is 511. ufix<32, 0>."]
        #[must_use]
        #[inline(always)]
        pub const fn posmax(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Max ID Of Lines. For example-1, for 512 lines, it is 511. ufix<32, 0>."]
        #[inline(always)]
        pub const fn set_posmax(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for DiscreteCfg0 {
        #[inline(always)]
        fn default() -> DiscreteCfg0 {
            DiscreteCfg0(0)
        }
    }
    impl core::fmt::Debug for DiscreteCfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DiscreteCfg0")
                .field("posmax", &self.posmax())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DiscreteCfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DiscreteCfg0 {{ posmax: {=u32:?} }}", self.posmax())
        }
    }
    #[doc = "Discrete Mode Configuration 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DiscreteCfg1(pub u32);
    impl DiscreteCfg1 {
        #[doc = "discrete mode: ufix<32, 0> of 1/(Number Of Lines) continuous mode: the max delta for tracking from the last received position, ufix<32, 32>."]
        #[must_use]
        #[inline(always)]
        pub const fn inv_posmax(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "discrete mode: ufix<32, 0> of 1/(Number Of Lines) continuous mode: the max delta for tracking from the last received position, ufix<32, 32>."]
        #[inline(always)]
        pub const fn set_inv_posmax(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DiscreteCfg1 {
        #[inline(always)]
        fn default() -> DiscreteCfg1 {
            DiscreteCfg1(0)
        }
    }
    impl core::fmt::Debug for DiscreteCfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DiscreteCfg1")
                .field("inv_posmax", &self.inv_posmax())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DiscreteCfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DiscreteCfg1 {{ inv_posmax: {=u32:?} }}",
                self.inv_posmax()
            )
        }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ErrThr(pub u32);
    impl ErrThr {
        #[doc = "ErrThr0: Error Threshold 0, (abs(tracking error)>= will choose the coefs as below) Note: ErrThr0>ErrThr1>ErrThr2 ufix<31, 28>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ErrThr0: Error Threshold 0, (abs(tracking error)>= will choose the coefs as below) Note: ErrThr0>ErrThr1>ErrThr2 ufix<31, 28>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ErrThr {
        #[inline(always)]
        fn default() -> ErrThr {
            ErrThr(0)
        }
    }
    impl core::fmt::Debug for ErrThr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ErrThr").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ErrThr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ErrThr {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The accelerator register for theinternal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EstmAccel(pub u32);
    impl EstmAccel {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EstmAccel {
        #[inline(always)]
        fn default() -> EstmAccel {
            EstmAccel(0)
        }
    }
    impl core::fmt::Debug for EstmAccel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EstmAccel")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EstmAccel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EstmAccel {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The position register for the internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EstmPos(pub u32);
    impl EstmPos {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EstmPos {
        #[inline(always)]
        fn default() -> EstmPos {
            EstmPos(0)
        }
    }
    impl core::fmt::Debug for EstmPos {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EstmPos").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EstmPos {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EstmPos {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The revolution register for the internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EstmRev(pub u32);
    impl EstmRev {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EstmRev {
        #[inline(always)]
        fn default() -> EstmRev {
            EstmRev(0)
        }
    }
    impl core::fmt::Debug for EstmRev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EstmRev").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EstmRev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EstmRev {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The speed register for the internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EstmSpeed(pub u32);
    impl EstmSpeed {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EstmSpeed {
        #[inline(always)]
        fn default() -> EstmSpeed {
            EstmSpeed(0)
        }
    }
    impl core::fmt::Debug for EstmSpeed {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EstmSpeed")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EstmSpeed {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EstmSpeed {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The timestamp register for internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EstmTim(pub u32);
    impl EstmTim {
        #[doc = "the value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EstmTim {
        #[inline(always)]
        fn default() -> EstmTim {
            EstmTim(0)
        }
    }
    impl core::fmt::Debug for EstmTim {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EstmTim").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EstmTim {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EstmTim {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 I."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I(pub u32);
    impl I {
        #[doc = "I0_Coef, fix<32, 21>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "I0_Coef, fix<32, 21>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for I {
        #[inline(always)]
        fn default() -> I {
            I(0)
        }
    }
    impl core::fmt::Debug for I {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "I {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined accelerator register for position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniAccel(pub u32);
    impl IniAccel {
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniAccel {
        #[inline(always)]
        fn default() -> IniAccel {
            IniAccel(0)
        }
    }
    impl core::fmt::Debug for IniAccel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniAccel")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniAccel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniAccel {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined coefficient A register for coefficients initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniAcoef(pub u32);
    impl IniAcoef {
        #[doc = "the value, fix<32, 19>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value, fix<32, 19>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniAcoef {
        #[inline(always)]
        fn default() -> IniAcoef {
            IniAcoef(0)
        }
    }
    impl core::fmt::Debug for IniAcoef {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniAcoef")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniAcoef {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniAcoef {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined timestamp register for coefficients initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniCoefTime(pub u32);
    impl IniCoefTime {
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniCoefTime {
        #[inline(always)]
        fn default() -> IniCoefTime {
            IniCoefTime(0)
        }
    }
    impl core::fmt::Debug for IniCoefTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniCoefTime")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniCoefTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniCoefTime {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined delta accelerator register for delta position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniDeltaAccel(pub u32);
    impl IniDeltaAccel {
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniDeltaAccel {
        #[inline(always)]
        fn default() -> IniDeltaAccel {
            IniDeltaAccel(0)
        }
    }
    impl core::fmt::Debug for IniDeltaAccel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniDeltaAccel")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniDeltaAccel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniDeltaAccel {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined delta position register for delta position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniDeltaPos(pub u32);
    impl IniDeltaPos {
        #[doc = "the value continuous mode: ufix <32, 32>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: ufix <32, 32>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniDeltaPos {
        #[inline(always)]
        fn default() -> IniDeltaPos {
            IniDeltaPos(0)
        }
    }
    impl core::fmt::Debug for IniDeltaPos {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniDeltaPos")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniDeltaPos {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniDeltaPos {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined timestamp register for delta position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniDeltaPosTime(pub u32);
    impl IniDeltaPosTime {
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniDeltaPosTime {
        #[inline(always)]
        fn default() -> IniDeltaPosTime {
            IniDeltaPosTime(0)
        }
    }
    impl core::fmt::Debug for IniDeltaPosTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniDeltaPosTime")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniDeltaPosTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniDeltaPosTime {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined delta revolution register for delta position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniDeltaRev(pub u32);
    impl IniDeltaRev {
        #[doc = "the value continuous mode: fix<32, 0>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 0>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniDeltaRev {
        #[inline(always)]
        fn default() -> IniDeltaRev {
            IniDeltaRev(0)
        }
    }
    impl core::fmt::Debug for IniDeltaRev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniDeltaRev")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniDeltaRev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniDeltaRev {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined delta speed register for delta position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniDeltaSpeed(pub u32);
    impl IniDeltaSpeed {
        #[doc = "the value； continuous mode: fix<32, 19>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value； continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniDeltaSpeed {
        #[inline(always)]
        fn default() -> IniDeltaSpeed {
            IniDeltaSpeed(0)
        }
    }
    impl core::fmt::Debug for IniDeltaSpeed {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniDeltaSpeed")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniDeltaSpeed {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniDeltaSpeed {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined coefficient I register for coefficients initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniIcoef(pub u32);
    impl IniIcoef {
        #[doc = "the value, fix<32, 21>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value, fix<32, 21>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniIcoef {
        #[inline(always)]
        fn default() -> IniIcoef {
            IniIcoef(0)
        }
    }
    impl core::fmt::Debug for IniIcoef {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniIcoef")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniIcoef {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniIcoef {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined coefficient P register for coefficients initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniPcoef(pub u32);
    impl IniPcoef {
        #[doc = "the value, fix<32, 15>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value, fix<32, 15>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniPcoef {
        #[inline(always)]
        fn default() -> IniPcoef {
            IniPcoef(0)
        }
    }
    impl core::fmt::Debug for IniPcoef {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniPcoef")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniPcoef {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniPcoef {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined position register for position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniPos(pub u32);
    impl IniPos {
        #[doc = "the value； continuous mode: ufix<32, 32>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value； continuous mode: ufix<32, 32>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniPos {
        #[inline(always)]
        fn default() -> IniPos {
            IniPos(0)
        }
    }
    impl core::fmt::Debug for IniPos {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniPos").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniPos {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniPos {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined timestamp register for position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniPosTime(pub u32);
    impl IniPosTime {
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniPosTime {
        #[inline(always)]
        fn default() -> IniPosTime {
            IniPosTime(0)
        }
    }
    impl core::fmt::Debug for IniPosTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniPosTime")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniPosTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniPosTime {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined revolution register for position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniRev(pub u32);
    impl IniRev {
        #[doc = "the value； continuous mode: ufix<32, 0>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value； continuous mode: ufix<32, 0>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniRev {
        #[inline(always)]
        fn default() -> IniRev {
            IniRev(0)
        }
    }
    impl core::fmt::Debug for IniRev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniRev").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniRev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniRev {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "The destined speed register for position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniSpeed(pub u32);
    impl IniSpeed {
        #[doc = "the value; continuous mode: fix<32, 19>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value; continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniSpeed {
        #[inline(always)]
        fn default() -> IniSpeed {
            IniSpeed(0)
        }
    }
    impl core::fmt::Debug for IniSpeed {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IniSpeed")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IniSpeed {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IniSpeed {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "Interrupt Enable for SHADOW_RD_DONE."]
        #[must_use]
        #[inline(always)]
        pub const fn shadow_rd_done_ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for SHADOW_RD_DONE."]
        #[inline(always)]
        pub const fn set_shadow_rd_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt Enable for INI_COEFS_CMD_DONE."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_coefs_cmd_done_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for INI_COEFS_CMD_DONE."]
        #[inline(always)]
        pub const fn set_ini_coefs_cmd_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt Enable for INI_POS_REQ_CMD_DONE."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_pos_req_cmd_done_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for INI_POS_REQ_CMD_DONE."]
        #[inline(always)]
        pub const fn set_ini_pos_req_cmd_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Interrupt Enable for OOSYNC."]
        #[must_use]
        #[inline(always)]
        pub const fn oosync_ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for OOSYNC."]
        #[inline(always)]
        pub const fn set_oosync_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interrupt Enable for INI_BR1_POS_REQ_CMD_DONE."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_br1_pos_req_cmd_done_ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for INI_BR1_POS_REQ_CMD_DONE."]
        #[inline(always)]
        pub const fn set_ini_br1_pos_req_cmd_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Interrupt Enable for INI_BR0_POS_REQ_CMD_DONE."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_br0_pos_req_cmd_done_ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for INI_BR0_POS_REQ_CMD_DONE."]
        #[inline(always)]
        pub const fn set_ini_br0_pos_req_cmd_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Interrupt Enable for INI_DELTA_POS_REQ_CMD_DONE."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_delta_pos_req_cmd_done_ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for INI_DELTA_POS_REQ_CMD_DONE."]
        #[inline(always)]
        pub const fn set_ini_delta_pos_req_cmd_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Interrupt Enable for POS_TRG_VALID."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_trg_vld_ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for POS_TRG_VALID."]
        #[inline(always)]
        pub const fn set_pos_trg_vld_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Interrupt Enable for SPEED_TRG_VALID."]
        #[must_use]
        #[inline(always)]
        pub const fn speed_trg_vld_ie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for SPEED_TRG_VALID."]
        #[inline(always)]
        pub const fn set_speed_trg_vld_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    impl core::fmt::Debug for IntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntEn")
                .field("shadow_rd_done_ie", &self.shadow_rd_done_ie())
                .field("ini_coefs_cmd_done_ie", &self.ini_coefs_cmd_done_ie())
                .field("ini_pos_req_cmd_done_ie", &self.ini_pos_req_cmd_done_ie())
                .field("oosync_ie", &self.oosync_ie())
                .field(
                    "ini_br1_pos_req_cmd_done_ie",
                    &self.ini_br1_pos_req_cmd_done_ie(),
                )
                .field(
                    "ini_br0_pos_req_cmd_done_ie",
                    &self.ini_br0_pos_req_cmd_done_ie(),
                )
                .field(
                    "ini_delta_pos_req_cmd_done_ie",
                    &self.ini_delta_pos_req_cmd_done_ie(),
                )
                .field("pos_trg_vld_ie", &self.pos_trg_vld_ie())
                .field("speed_trg_vld_ie", &self.speed_trg_vld_ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntEn {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntEn {{ shadow_rd_done_ie: {=bool:?}, ini_coefs_cmd_done_ie: {=bool:?}, ini_pos_req_cmd_done_ie: {=bool:?}, oosync_ie: {=bool:?}, ini_br1_pos_req_cmd_done_ie: {=bool:?}, ini_br0_pos_req_cmd_done_ie: {=bool:?}, ini_delta_pos_req_cmd_done_ie: {=bool:?}, pos_trg_vld_ie: {=bool:?}, speed_trg_vld_ie: {=bool:?} }}" , self . shadow_rd_done_ie () , self . ini_coefs_cmd_done_ie () , self . ini_pos_req_cmd_done_ie () , self . oosync_ie () , self . ini_br1_pos_req_cmd_done_ie () , self . ini_br0_pos_req_cmd_done_ie () , self . ini_delta_pos_req_cmd_done_ie () , self . pos_trg_vld_ie () , self . speed_trg_vld_ie ())
        }
    }
    #[doc = "Position Out-Of-Sync Threshold Regster."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OosyncThetaThr(pub u32);
    impl OosyncThetaThr {
        #[doc = "the threshold of theta difference between actual and prediction for out-of-sync determination，ufix<32, 32>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the threshold of theta difference between actual and prediction for out-of-sync determination，ufix<32, 32>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OosyncThetaThr {
        #[inline(always)]
        fn default() -> OosyncThetaThr {
            OosyncThetaThr(0)
        }
    }
    impl core::fmt::Debug for OosyncThetaThr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OosyncThetaThr")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OosyncThetaThr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OosyncThetaThr {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 P."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P(pub u32);
    impl P {
        #[doc = "P0_Coef, fix<32, 15>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "P0_Coef, fix<32, 15>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P {
        #[inline(always)]
        fn default() -> P {
            P(0)
        }
    }
    impl core::fmt::Debug for P {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("P").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for P {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "P {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Tracking Configuration pos trigger cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosTrgCfg(pub u32);
    impl PosTrgCfg {
        #[doc = "1-trigger valid; 0-Trigger not valid\"."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1-trigger valid; 0-Trigger not valid\"."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0: (rising edge) pos inc greater than, 1: (falling edge) pos dec less than."]
        #[must_use]
        #[inline(always)]
        pub const fn edge(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0: (rising edge) pos inc greater than, 1: (falling edge) pos dec less than."]
        #[inline(always)]
        pub const fn set_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for PosTrgCfg {
        #[inline(always)]
        fn default() -> PosTrgCfg {
            PosTrgCfg(0)
        }
    }
    impl core::fmt::Debug for PosTrgCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosTrgCfg")
                .field("en", &self.en())
                .field("edge", &self.edge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosTrgCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PosTrgCfg {{ en: {=bool:?}, edge: {=bool:?} }}",
                self.en(),
                self.edge()
            )
        }
    }
    #[doc = "Tracking Configuration position threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosTrgPosThr(pub u32);
    impl PosTrgPosThr {
        #[doc = "For pos out trigger (pos). ufix<32, 32>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "For pos out trigger (pos). ufix<32, 32>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosTrgPosThr {
        #[inline(always)]
        fn default() -> PosTrgPosThr {
            PosTrgPosThr(0)
        }
    }
    impl core::fmt::Debug for PosTrgPosThr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosTrgPosThr")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosTrgPosThr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosTrgPosThr {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Tracking Configuration revolution threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosTrgRevThr(pub u32);
    impl PosTrgRevThr {
        #[doc = "For pos out trigger (rev) fix<32, 0>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "For pos out trigger (rev) fix<32, 0>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosTrgRevThr {
        #[inline(always)]
        fn default() -> PosTrgRevThr {
            PosTrgRevThr(0)
        }
    }
    impl core::fmt::Debug for PosTrgRevThr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosTrgRevThr")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosTrgRevThr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosTrgRevThr {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Tracking Configuration speed trigger cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpeedTrgCfg(pub u32);
    impl SpeedTrgCfg {
        #[doc = "1-trigger valid; 0-Trigger not valid Normally it means either the max pos speed, or the min negative speed."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1-trigger valid; 0-Trigger not valid Normally it means either the max pos speed, or the min negative speed."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0: (rising edge) speed inc greater than, 1: (falling edge) speed dec less than."]
        #[must_use]
        #[inline(always)]
        pub const fn edge(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0: (rising edge) speed inc greater than, 1: (falling edge) speed dec less than."]
        #[inline(always)]
        pub const fn set_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "1: Use abs value for comparion. 0: Use the speed with direction info (so not the abs value)."]
        #[must_use]
        #[inline(always)]
        pub const fn comp_type(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "1: Use abs value for comparion. 0: Use the speed with direction info (so not the abs value)."]
        #[inline(always)]
        pub const fn set_comp_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for SpeedTrgCfg {
        #[inline(always)]
        fn default() -> SpeedTrgCfg {
            SpeedTrgCfg(0)
        }
    }
    impl core::fmt::Debug for SpeedTrgCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SpeedTrgCfg")
                .field("en", &self.en())
                .field("edge", &self.edge())
                .field("comp_type", &self.comp_type())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SpeedTrgCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SpeedTrgCfg {{ en: {=bool:?}, edge: {=bool:?}, comp_type: {=bool:?} }}",
                self.en(),
                self.edge(),
                self.comp_type()
            )
        }
    }
    #[doc = "Tracking Configuration speed threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpeedTrgThr(pub u32);
    impl SpeedTrgThr {
        #[doc = "For speed trigger. continuous mode: fix<32, 19>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "For speed trigger. continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SpeedTrgThr {
        #[inline(always)]
        fn default() -> SpeedTrgThr {
            SpeedTrgThr(0)
        }
    }
    impl core::fmt::Debug for SpeedTrgThr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SpeedTrgThr")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SpeedTrgThr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SpeedTrgThr {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sta(pub u32);
    impl Sta {
        #[doc = "Shadow ready for read. Auto cleared by setting CR\\[SHADOW_RD_REQ\\]
as 1."]
        #[must_use]
        #[inline(always)]
        pub const fn shadow_rd_done(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Shadow ready for read. Auto cleared by setting CR\\[SHADOW_RD_REQ\\]
as 1."]
        #[inline(always)]
        pub const fn set_shadow_rd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "W1C."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_coefs_cmd_done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn set_ini_coefs_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "W1C."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_pos_req_cmd_done(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn set_ini_pos_req_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Tracking module out-of sync. W1C."]
        #[must_use]
        #[inline(always)]
        pub const fn oosync(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Tracking module out-of sync. W1C."]
        #[inline(always)]
        pub const fn set_oosync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Tracking Module in Idle status."]
        #[must_use]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Tracking Module in Idle status."]
        #[inline(always)]
        pub const fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "W1C."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_br1_pos_req_cmd_done(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn set_ini_br1_pos_req_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "W1C."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_br0_pos_req_cmd_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn set_ini_br0_pos_req_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "W1C."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_delta_pos_req_cmd_done(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn set_ini_delta_pos_req_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "W1C."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_trg_valid(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn set_pos_trg_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "W1C."]
        #[must_use]
        #[inline(always)]
        pub const fn speed_trg_valid(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn set_speed_trg_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tracking ERR_ID."]
        #[must_use]
        #[inline(always)]
        pub const fn err_id(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Tracking ERR_ID."]
        #[inline(always)]
        pub const fn set_err_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Sta {
        #[inline(always)]
        fn default() -> Sta {
            Sta(0)
        }
    }
    impl core::fmt::Debug for Sta {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sta")
                .field("shadow_rd_done", &self.shadow_rd_done())
                .field("ini_coefs_cmd_done", &self.ini_coefs_cmd_done())
                .field("ini_pos_req_cmd_done", &self.ini_pos_req_cmd_done())
                .field("oosync", &self.oosync())
                .field("idle", &self.idle())
                .field("ini_br1_pos_req_cmd_done", &self.ini_br1_pos_req_cmd_done())
                .field("ini_br0_pos_req_cmd_done", &self.ini_br0_pos_req_cmd_done())
                .field(
                    "ini_delta_pos_req_cmd_done",
                    &self.ini_delta_pos_req_cmd_done(),
                )
                .field("pos_trg_valid", &self.pos_trg_valid())
                .field("speed_trg_valid", &self.speed_trg_valid())
                .field("err_id", &self.err_id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sta {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sta {{ shadow_rd_done: {=bool:?}, ini_coefs_cmd_done: {=bool:?}, ini_pos_req_cmd_done: {=bool:?}, oosync: {=bool:?}, idle: {=bool:?}, ini_br1_pos_req_cmd_done: {=bool:?}, ini_br0_pos_req_cmd_done: {=bool:?}, ini_delta_pos_req_cmd_done: {=bool:?}, pos_trg_valid: {=bool:?}, speed_trg_valid: {=bool:?}, err_id: {=u8:?} }}" , self . shadow_rd_done () , self . ini_coefs_cmd_done () , self . ini_pos_req_cmd_done () , self . oosync () , self . idle () , self . ini_br1_pos_req_cmd_done () , self . ini_br0_pos_req_cmd_done () , self . ini_delta_pos_req_cmd_done () , self . pos_trg_valid () , self . speed_trg_valid () , self . err_id ())
        }
    }
    #[doc = "System Clock Frequency Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SysclkFreq(pub u32);
    impl SysclkFreq {
        #[doc = "system clock frequency, ufix<32, 0>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "system clock frequency, ufix<32, 0>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SysclkFreq {
        #[inline(always)]
        fn default() -> SysclkFreq {
            SysclkFreq(0)
        }
    }
    impl core::fmt::Debug for SysclkFreq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SysclkFreq")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SysclkFreq {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SysclkFreq {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "System Clock Period Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SysclkPeriod(pub u32);
    impl SysclkPeriod {
        #[doc = "round( the value of clock period * (2^24)*(2^20) ), ufix<32, 0>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "round( the value of clock period * (2^24)*(2^20) ), ufix<32, 0>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SysclkPeriod {
        #[inline(always)]
        fn default() -> SysclkPeriod {
            SysclkPeriod(0)
        }
    }
    impl core::fmt::Debug for SysclkPeriod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SysclkPeriod")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SysclkPeriod {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SysclkPeriod {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Time(pub u32);
    impl Time {
        #[doc = "CoefTime0: Time Stayed using this coefs (counted in input samples). Ideal value of tracing cycles should +1. ufix<32,0>."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CoefTime0: Time Stayed using this coefs (counted in input samples). Ideal value of tracing cycles should +1. ufix<32,0>."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Time {
        #[inline(always)]
        fn default() -> Time {
            Time(0)
        }
    }
    impl core::fmt::Debug for Time {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Time").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Time {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Time {{ val: {=u32:?} }}", self.val())
        }
    }
}
