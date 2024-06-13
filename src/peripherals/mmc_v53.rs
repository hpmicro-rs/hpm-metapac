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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Prediction Timing Offset Register."]
    #[inline(always)]
    pub const fn br_timeoff(self) -> crate::common::Reg<regs::BrTimeoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Prediction Triggering Period Offset Register."]
    #[inline(always)]
    pub const fn br_trg_period(self) -> crate::common::Reg<regs::BrTrgPeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Prediction Triggering First Offset Register."]
    #[inline(always)]
    pub const fn br_trg_f_time(self) -> crate::common::Reg<regs::BrTrgFTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Prediction Status Register."]
    #[inline(always)]
    pub const fn br_st(self) -> crate::common::Reg<regs::BrSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Prediction Configuration postion trigger cfg."]
    #[inline(always)]
    pub const fn br_trg_pos_cfg(self) -> crate::common::Reg<regs::BrTrgPosCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Prediction Configuration postion threshold."]
    #[inline(always)]
    pub const fn br_trg_pos_thr(self) -> crate::common::Reg<regs::BrTrgPosThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Prediction Configuration revolutiom threshold."]
    #[inline(always)]
    pub const fn br_trg_rev_thr(self) -> crate::common::Reg<regs::BrTrgRevThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Prediction Configuration speed trigger cfg."]
    #[inline(always)]
    pub const fn br_trg_speed_cfg(
        self,
    ) -> crate::common::Reg<regs::BrTrgSpeedCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Prediction Configuration speed threshold."]
    #[inline(always)]
    pub const fn br_trg_speed_thr(
        self,
    ) -> crate::common::Reg<regs::BrTrgSpeedThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Initialization timestamp for open-loop mode."]
    #[inline(always)]
    pub const fn br_ini_pos_time(
        self,
    ) -> crate::common::Reg<regs::BrIniPosTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "Initialization position for open-loop mode."]
    #[inline(always)]
    pub const fn br_ini_pos(self) -> crate::common::Reg<regs::BrIniPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "Initialization revolution for open-loop mode."]
    #[inline(always)]
    pub const fn br_ini_rev(self) -> crate::common::Reg<regs::BrIniRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "Initialization speed for open-loop mode."]
    #[inline(always)]
    pub const fn br_ini_speed(self) -> crate::common::Reg<regs::BrIniSpeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "Initialization acceleration for open-loop mode."]
    #[inline(always)]
    pub const fn br_ini_accel(self) -> crate::common::Reg<regs::BrIniAccel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "Initialization timestamp for delta mode in prediction mode."]
    #[inline(always)]
    pub const fn br_ini_delta_pos_time(
        self,
    ) -> crate::common::Reg<regs::BrIniDeltaPosTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "Initialization delta position for delta mode in prediction mode."]
    #[inline(always)]
    pub const fn br_ini_delta_pos(
        self,
    ) -> crate::common::Reg<regs::BrIniDeltaPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "Initialization delta revolution for delta mode in prediction mode."]
    #[inline(always)]
    pub const fn br_ini_delta_rev(
        self,
    ) -> crate::common::Reg<regs::BrIniDeltaRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "Initialization delta speed for delta mode in prediction mode."]
    #[inline(always)]
    pub const fn br_ini_delta_speed(
        self,
    ) -> crate::common::Reg<regs::BrIniDeltaSpeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "Initialization delta acceleration for delta mode in prediction mode."]
    #[inline(always)]
    pub const fn br_ini_delta_accel(
        self,
    ) -> crate::common::Reg<regs::BrIniDeltaAccel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "Monitor of the output timestamp."]
    #[inline(always)]
    pub const fn br_cur_pos_time(
        self,
    ) -> crate::common::Reg<regs::BrCurPosTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "Monitor of the output position."]
    #[inline(always)]
    pub const fn br_cur_pos(self) -> crate::common::Reg<regs::BrCurPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "Monitor of the output revolution."]
    #[inline(always)]
    pub const fn br_cur_rev(self) -> crate::common::Reg<regs::BrCurRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "Monitor of the output speed."]
    #[inline(always)]
    pub const fn br_cur_speed(self) -> crate::common::Reg<regs::BrCurSpeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "Monitor of the output acceleration."]
    #[inline(always)]
    pub const fn br_cur_accel(self) -> crate::common::Reg<regs::BrCurAccel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 P."]
    #[inline(always)]
    pub const fn p(self) -> crate::common::Reg<regs::P, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 I."]
    #[inline(always)]
    pub const fn i(self) -> crate::common::Reg<regs::I, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 A."]
    #[inline(always)]
    pub const fn a(self) -> crate::common::Reg<regs::A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 time."]
    #[inline(always)]
    pub const fn time(self) -> crate::common::Reg<regs::Time, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sta(self) -> crate::common::Reg<regs::Sta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "System Clock Frequency Register."]
    #[inline(always)]
    pub const fn sysclk_freq(self) -> crate::common::Reg<regs::SysclkFreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "System Clock Period Register."]
    #[inline(always)]
    pub const fn sysclk_period(self) -> crate::common::Reg<regs::SysclkPeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Position Out-Of-Sync Threshold Regster."]
    #[inline(always)]
    pub const fn oosync_theta_thr(
        self,
    ) -> crate::common::Reg<regs::OosyncThetaThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Discrete Mode Configuration 0 Register."]
    #[inline(always)]
    pub const fn discrete_cfg0(self) -> crate::common::Reg<regs::DiscreteCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Discrete Mode Configuration 1 Register."]
    #[inline(always)]
    pub const fn discrete_cfg1(self) -> crate::common::Reg<regs::DiscreteCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Continuous Mode Configuration 0 Register."]
    #[inline(always)]
    pub const fn cont_cfg0(self) -> crate::common::Reg<regs::ContCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "The destined timestamp register for position initialization."]
    #[inline(always)]
    pub const fn ini_pos_time(self) -> crate::common::Reg<regs::IniPosTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "The destined position register for position initialization."]
    #[inline(always)]
    pub const fn ini_pos(self) -> crate::common::Reg<regs::IniPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "The destined revolution register for position initialization."]
    #[inline(always)]
    pub const fn ini_rev(self) -> crate::common::Reg<regs::IniRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "The destined speed register for position initialization."]
    #[inline(always)]
    pub const fn ini_speed(self) -> crate::common::Reg<regs::IniSpeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "The destined accelerator register for position initialization."]
    #[inline(always)]
    pub const fn ini_accel(self) -> crate::common::Reg<regs::IniAccel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "The destined timestamp register for coefficients initialization."]
    #[inline(always)]
    pub const fn ini_coef_time(self) -> crate::common::Reg<regs::IniCoefTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "The destined coefficient P register for coefficients initialization."]
    #[inline(always)]
    pub const fn ini_pcoef(self) -> crate::common::Reg<regs::IniPcoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "The destined coefficient I register for coefficients initialization."]
    #[inline(always)]
    pub const fn ini_icoef(self) -> crate::common::Reg<regs::IniIcoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "The destined coefficient A register for coefficients initialization."]
    #[inline(always)]
    pub const fn ini_acoef(self) -> crate::common::Reg<regs::IniAcoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "The timestamp register for internal estimation."]
    #[inline(always)]
    pub const fn estm_tim(self) -> crate::common::Reg<regs::EstmTim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "The position register for the internal estimation."]
    #[inline(always)]
    pub const fn estm_pos(self) -> crate::common::Reg<regs::EstmPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "The revolution register for the internal estimation."]
    #[inline(always)]
    pub const fn estm_rev(self) -> crate::common::Reg<regs::EstmRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "The speed register for the internal estimation."]
    #[inline(always)]
    pub const fn estm_speed(self) -> crate::common::Reg<regs::EstmSpeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "The accelerator register for theinternal estimation."]
    #[inline(always)]
    pub const fn estm_accel(self) -> crate::common::Reg<regs::EstmAccel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "The coefficient P register for the internal estimation."]
    #[inline(always)]
    pub const fn cur_pcoef(self) -> crate::common::Reg<regs::CurPcoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "The coefficient I register for the internal estimation."]
    #[inline(always)]
    pub const fn cur_icoef(self) -> crate::common::Reg<regs::CurIcoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "The coefficient A register for the internal estimation."]
    #[inline(always)]
    pub const fn cur_acoef(self) -> crate::common::Reg<regs::CurAcoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "The destined timestamp register for delta position initialization."]
    #[inline(always)]
    pub const fn ini_delta_pos_time(
        self,
    ) -> crate::common::Reg<regs::IniDeltaPosTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "The destined delta position register for delta position initialization."]
    #[inline(always)]
    pub const fn ini_delta_pos(self) -> crate::common::Reg<regs::IniDeltaPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "The destined delta revolution register for delta position initialization."]
    #[inline(always)]
    pub const fn ini_delta_rev(self) -> crate::common::Reg<regs::IniDeltaRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "The destined delta speed register for delta position initialization."]
    #[inline(always)]
    pub const fn ini_delta_speed(
        self,
    ) -> crate::common::Reg<regs::IniDeltaSpeed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "The destined delta accelerator register for delta position initialization."]
    #[inline(always)]
    pub const fn ini_delta_accel(
        self,
    ) -> crate::common::Reg<regs::IniDeltaAccel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "Tracking Configuration pos trigger cfg."]
    #[inline(always)]
    pub const fn pos_trg_cfg(self) -> crate::common::Reg<regs::PosTrgCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Tracking Configuration position threshold."]
    #[inline(always)]
    pub const fn pos_trg_pos_thr(
        self,
    ) -> crate::common::Reg<regs::PosTrgPosThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Tracking Configuration revolution threshold."]
    #[inline(always)]
    pub const fn pos_trg_rev_thr(
        self,
    ) -> crate::common::Reg<regs::PosTrgRevThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Tracking Configuration speed trigger cfg."]
    #[inline(always)]
    pub const fn speed_trg_cfg(self) -> crate::common::Reg<regs::SpeedTrgCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Tracking Configuration speed threshold."]
    #[inline(always)]
    pub const fn speed_trg_thr(self) -> crate::common::Reg<regs::SpeedTrgThr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn coef_trg_cfg(self, n: usize) -> CoefTrgCfg {
        assert!(n < 3usize);
        unsafe { CoefTrgCfg::from_ptr(self.ptr.add(0xa0usize + n * 20usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn br(self, n: usize) -> Br {
        assert!(n < 2usize);
        unsafe { Br::from_ptr(self.ptr.add(0x0100usize + n * 256usize) as _) }
    }
    #[doc = "Monitor of the just received input timestamp for tracing logic."]
    #[inline(always)]
    pub const fn bk0_timestamp(self) -> crate::common::Reg<regs::Bk0Timestamp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "Monitor of the just received input position for tracing logic."]
    #[inline(always)]
    pub const fn bk0_position(self) -> crate::common::Reg<regs::Bk0Position, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "Monitor of the just received input revolution for tracing logic."]
    #[inline(always)]
    pub const fn bk0_revolution(
        self,
    ) -> crate::common::Reg<regs::Bk0Revolution, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "Monitor of the just received input speed for tracing logic."]
    #[inline(always)]
    pub const fn bk0_speed(self) -> crate::common::Reg<regs::Bk0Speed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "Monitor of the just received input acceleration for tracing logic."]
    #[inline(always)]
    pub const fn bk0_accelerator(
        self,
    ) -> crate::common::Reg<regs::Bk0Accelerator, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[doc = "Monitor of the previous received input timestamp for tracing logic."]
    #[inline(always)]
    pub const fn bk1_timestamp(self) -> crate::common::Reg<regs::Bk1Timestamp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
    }
    #[doc = "Monitor of the previous received input position for tracing logic."]
    #[inline(always)]
    pub const fn bk1_position(self) -> crate::common::Reg<regs::Bk1Position, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0324usize) as _) }
    }
    #[doc = "Monitor of the previous received input revolution for tracing logic."]
    #[inline(always)]
    pub const fn bk1_revolution(
        self,
    ) -> crate::common::Reg<regs::Bk1Revolution, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0328usize) as _) }
    }
    #[doc = "Monitor of the previous received input speed for tracing logic."]
    #[inline(always)]
    pub const fn bk1_speed(self) -> crate::common::Reg<regs::Bk1Speed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x032cusize) as _) }
    }
    #[doc = "Monitor of the previous received input acceleration for tracing logic."]
    #[inline(always)]
    pub const fn bk1_accelerator(
        self,
    ) -> crate::common::Reg<regs::Bk1Accelerator, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0330usize) as _) }
    }
}
pub mod regs {
    #[doc = "Tracking Configuration coef trigger cfg&index0 A."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct A(pub u32);
    impl A {
        #[doc = "A0_Coef，fix<32, 19>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "A0_Coef，fix<32, 19>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for A {
        #[inline(always)]
        fn default() -> A {
            A(0)
        }
    }
    #[doc = "Monitor of the just received input acceleration for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk0Accelerator(pub u32);
    impl Bk0Accelerator {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk0Accelerator {
        #[inline(always)]
        fn default() -> Bk0Accelerator {
            Bk0Accelerator(0)
        }
    }
    #[doc = "Monitor of the just received input position for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk0Position(pub u32);
    impl Bk0Position {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk0Position {
        #[inline(always)]
        fn default() -> Bk0Position {
            Bk0Position(0)
        }
    }
    #[doc = "Monitor of the just received input revolution for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk0Revolution(pub u32);
    impl Bk0Revolution {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk0Revolution {
        #[inline(always)]
        fn default() -> Bk0Revolution {
            Bk0Revolution(0)
        }
    }
    #[doc = "Monitor of the just received input speed for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk0Speed(pub u32);
    impl Bk0Speed {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk0Speed {
        #[inline(always)]
        fn default() -> Bk0Speed {
            Bk0Speed(0)
        }
    }
    #[doc = "Monitor of the just received input timestamp for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk0Timestamp(pub u32);
    impl Bk0Timestamp {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk0Timestamp {
        #[inline(always)]
        fn default() -> Bk0Timestamp {
            Bk0Timestamp(0)
        }
    }
    #[doc = "Monitor of the previous received input acceleration for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk1Accelerator(pub u32);
    impl Bk1Accelerator {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk1Accelerator {
        #[inline(always)]
        fn default() -> Bk1Accelerator {
            Bk1Accelerator(0)
        }
    }
    #[doc = "Monitor of the previous received input position for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk1Position(pub u32);
    impl Bk1Position {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk1Position {
        #[inline(always)]
        fn default() -> Bk1Position {
            Bk1Position(0)
        }
    }
    #[doc = "Monitor of the previous received input revolution for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk1Revolution(pub u32);
    impl Bk1Revolution {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk1Revolution {
        #[inline(always)]
        fn default() -> Bk1Revolution {
            Bk1Revolution(0)
        }
    }
    #[doc = "Monitor of the previous received input speed for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk1Speed(pub u32);
    impl Bk1Speed {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk1Speed {
        #[inline(always)]
        fn default() -> Bk1Speed {
            Bk1Speed(0)
        }
    }
    #[doc = "Monitor of the previous received input timestamp for tracing logic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bk1Timestamp(pub u32);
    impl Bk1Timestamp {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bk1Timestamp {
        #[inline(always)]
        fn default() -> Bk1Timestamp {
            Bk1Timestamp(0)
        }
    }
    #[doc = "Prediction Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrCtrl(pub u32);
    impl BrCtrl {
        #[doc = "Branch Enable."]
        #[inline(always)]
        pub const fn br_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Branch Enable."]
        #[inline(always)]
        pub fn set_br_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "1. First trigger by external trigger pin 0. First trigger by the timer When in CR\\[MANUAL_IO\\]=1 mode, it is the prediction trigger."]
        #[inline(always)]
        pub const fn f_trg_type(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1. First trigger by external trigger pin 0. First trigger by the timer When in CR\\[MANUAL_IO\\]=1 mode, it is the prediction trigger."]
        #[inline(always)]
        pub fn set_f_trg_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "1. Each non-first trigger by external trigger pin 0. Each non-first trigger by the timer."]
        #[inline(always)]
        pub const fn nf_trg_type(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "1. Each non-first trigger by external trigger pin 0. Each non-first trigger by the timer."]
        #[inline(always)]
        pub fn set_nf_trg_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "1:continuously repeat pred, 0:cal the pred based on a definite time-stamp offset, 2:programed one-shot prediction mode."]
        #[inline(always)]
        pub const fn pred_mode(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "1:continuously repeat pred, 0:cal the pred based on a definite time-stamp offset, 2:programed one-shot prediction mode."]
        #[inline(always)]
        pub fn set_pred_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "1: in open loop mode 0: not in open loop mode."]
        #[inline(always)]
        pub const fn open_loop_mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "1: in open loop mode 0: not in open loop mode."]
        #[inline(always)]
        pub fn set_open_loop_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "1: Command to reload the delta pos. Auto clear 0:."]
        #[inline(always)]
        pub const fn ini_delta_pos_req(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1: Command to reload the delta pos. Auto clear 0:."]
        #[inline(always)]
        pub fn set_ini_delta_pos_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "1: change 0: won't change bit 3: for delta accel bit 2: for delta speed bit 1: for delta revolution bit 0: for delta position."]
        #[inline(always)]
        pub const fn ini_delta_pos_cmd_msk(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "1: change 0: won't change bit 3: for delta accel bit 2: for delta speed bit 1: for delta revolution bit 0: for delta position."]
        #[inline(always)]
        pub fn set_ini_delta_pos_cmd_msk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "Interrupt Enable for INI_DELTA_POS_DONE."]
        #[inline(always)]
        pub const fn ini_delta_pos_done_ie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for INI_DELTA_POS_DONE."]
        #[inline(always)]
        pub fn set_ini_delta_pos_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "0: Time Stamp in the configuration 1: Risedge of In Trg\\[0\\]
2: Risedge of In Trg\\[1\\]
3: Risedge of out trg\\[0\\]
4: Risedge of out trg\\[1\\]
5: Risedge of self pos trigger 6: Risedge of self speed trigger Others: no function."]
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
        pub fn set_ini_delta_pos_trg_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "1: change 0: won't change bit 3: for accel bit 2: for speed bit 1: for revolution bit 0: for position."]
        #[inline(always)]
        pub const fn ini_pos_cmd_msk(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[doc = "1: change 0: won't change bit 3: for accel bit 2: for speed bit 1: for revolution bit 0: for position."]
        #[inline(always)]
        pub fn set_ini_pos_cmd_msk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[doc = "0: Time Stamp in the configuration 1: Risedge of In Trg\\[0\\]
2: Risedge of In Trg\\[1\\]
3: Risedge of out trg\\[0\\]
4: Risedge of out trg\\[1\\]
5: Risedge of self pos trigger 6: Risedge of self speed trigger Others: no function."]
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
        pub fn set_ini_pos_trg_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
        }
        #[doc = "Interrupt Enable for POS_TRG_VALID."]
        #[inline(always)]
        pub const fn pos_trg_valid_ie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for POS_TRG_VALID."]
        #[inline(always)]
        pub fn set_pos_trg_valid_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Interrupt Enable for SPEED_TRG_VALID."]
        #[inline(always)]
        pub const fn speed_trg_valid_ie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for SPEED_TRG_VALID."]
        #[inline(always)]
        pub fn set_speed_trg_valid_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for BrCtrl {
        #[inline(always)]
        fn default() -> BrCtrl {
            BrCtrl(0)
        }
    }
    #[doc = "Monitor of the output acceleration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrCurAccel(pub u32);
    impl BrCurAccel {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrCurAccel {
        #[inline(always)]
        fn default() -> BrCurAccel {
            BrCurAccel(0)
        }
    }
    #[doc = "Monitor of the output position."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrCurPos(pub u32);
    impl BrCurPos {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrCurPos {
        #[inline(always)]
        fn default() -> BrCurPos {
            BrCurPos(0)
        }
    }
    #[doc = "Monitor of the output timestamp."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrCurPosTime(pub u32);
    impl BrCurPosTime {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrCurPosTime {
        #[inline(always)]
        fn default() -> BrCurPosTime {
            BrCurPosTime(0)
        }
    }
    #[doc = "Monitor of the output revolution."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrCurRev(pub u32);
    impl BrCurRev {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrCurRev {
        #[inline(always)]
        fn default() -> BrCurRev {
            BrCurRev(0)
        }
    }
    #[doc = "Monitor of the output speed."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrCurSpeed(pub u32);
    impl BrCurSpeed {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrCurSpeed {
        #[inline(always)]
        fn default() -> BrCurSpeed {
            BrCurSpeed(0)
        }
    }
    #[doc = "Initialization acceleration for open-loop mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniAccel(pub u32);
    impl BrIniAccel {
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniAccel {
        #[inline(always)]
        fn default() -> BrIniAccel {
            BrIniAccel(0)
        }
    }
    #[doc = "Initialization delta acceleration for delta mode in prediction mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniDeltaAccel(pub u32);
    impl BrIniDeltaAccel {
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniDeltaAccel {
        #[inline(always)]
        fn default() -> BrIniDeltaAccel {
            BrIniDeltaAccel(0)
        }
    }
    #[doc = "Initialization delta position for delta mode in prediction mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniDeltaPos(pub u32);
    impl BrIniDeltaPos {
        #[doc = "the value continuous mode: ufix<32, 32>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: ufix<32, 32>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniDeltaPos {
        #[inline(always)]
        fn default() -> BrIniDeltaPos {
            BrIniDeltaPos(0)
        }
    }
    #[doc = "Initialization timestamp for delta mode in prediction mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniDeltaPosTime(pub u32);
    impl BrIniDeltaPosTime {
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniDeltaPosTime {
        #[inline(always)]
        fn default() -> BrIniDeltaPosTime {
            BrIniDeltaPosTime(0)
        }
    }
    #[doc = "Initialization delta revolution for delta mode in prediction mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniDeltaRev(pub u32);
    impl BrIniDeltaRev {
        #[doc = "the value continuous mode: fix<32, 0>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 0>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniDeltaRev {
        #[inline(always)]
        fn default() -> BrIniDeltaRev {
            BrIniDeltaRev(0)
        }
    }
    #[doc = "Initialization delta speed for delta mode in prediction mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniDeltaSpeed(pub u32);
    impl BrIniDeltaSpeed {
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniDeltaSpeed {
        #[inline(always)]
        fn default() -> BrIniDeltaSpeed {
            BrIniDeltaSpeed(0)
        }
    }
    #[doc = "Initialization position for open-loop mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniPos(pub u32);
    impl BrIniPos {
        #[doc = "the value ufix<32, 32>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value ufix<32, 32>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniPos {
        #[inline(always)]
        fn default() -> BrIniPos {
            BrIniPos(0)
        }
    }
    #[doc = "Initialization timestamp for open-loop mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniPosTime(pub u32);
    impl BrIniPosTime {
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniPosTime {
        #[inline(always)]
        fn default() -> BrIniPosTime {
            BrIniPosTime(0)
        }
    }
    #[doc = "Initialization revolution for open-loop mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniRev(pub u32);
    impl BrIniRev {
        #[doc = "the value ufix<32, 0>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value ufix<32, 0>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniRev {
        #[inline(always)]
        fn default() -> BrIniRev {
            BrIniRev(0)
        }
    }
    #[doc = "Initialization speed for open-loop mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrIniSpeed(pub u32);
    impl BrIniSpeed {
        #[doc = "the value fix<32, 19>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value fix<32, 19>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrIniSpeed {
        #[inline(always)]
        fn default() -> BrIniSpeed {
            BrIniSpeed(0)
        }
    }
    #[doc = "Prediction Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrSt(pub u32);
    impl BrSt {
        #[doc = "The module's error ID output."]
        #[inline(always)]
        pub const fn err_id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The module's error ID output."]
        #[inline(always)]
        pub fn set_err_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "1: The prediction module is idle. 0: The prediction module is not idle."]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "1: The prediction module is idle. 0: The prediction module is not idle."]
        #[inline(always)]
        pub fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "1: the initialization of delta position command is done 0: the initialization of delta position command is not done."]
        #[inline(always)]
        pub const fn ini_delta_pos_done(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "1: the initialization of delta position command is done 0: the initialization of delta position command is not done."]
        #[inline(always)]
        pub fn set_ini_delta_pos_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "1：self position trigger event found 0：self position trigger event not found yet."]
        #[inline(always)]
        pub const fn pos_trg_vld(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1：self position trigger event found 0：self position trigger event not found yet."]
        #[inline(always)]
        pub fn set_pos_trg_vld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "1：self speed trigger event found 0：self speed trigger event not found yet."]
        #[inline(always)]
        pub const fn speed_trg_vld(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "1：self speed trigger event found 0：self speed trigger event not found yet."]
        #[inline(always)]
        pub fn set_speed_trg_vld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "1：in open loop mode 0：in closed loop mode."]
        #[inline(always)]
        pub const fn open_loop_st(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "1：in open loop mode 0：in closed loop mode."]
        #[inline(always)]
        pub fn set_open_loop_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for BrSt {
        #[inline(always)]
        fn default() -> BrSt {
            BrSt(0)
        }
    }
    #[doc = "Prediction Timing Offset Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTimeoff(pub u32);
    impl BrTimeoff {
        #[doc = "ufix<32, 0> time offset incycles from the trigger time."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ufix<32, 0> time offset incycles from the trigger time."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrTimeoff {
        #[inline(always)]
        fn default() -> BrTimeoff {
            BrTimeoff(0)
        }
    }
    #[doc = "Prediction Triggering First Offset Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgFTime(pub u32);
    impl BrTrgFTime {
        #[doc = "uifx<32, 0> the time for the first trigger."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "uifx<32, 0> the time for the first trigger."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrTrgFTime {
        #[inline(always)]
        fn default() -> BrTrgFTime {
            BrTrgFTime(0)
        }
    }
    #[doc = "Prediction Triggering Period Offset Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgPeriod(pub u32);
    impl BrTrgPeriod {
        #[doc = "uifx<32, 0>, time offset incycles between each trigger time."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "uifx<32, 0>, time offset incycles between each trigger time."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrTrgPeriod {
        #[inline(always)]
        fn default() -> BrTrgPeriod {
            BrTrgPeriod(0)
        }
    }
    #[doc = "Prediction Configuration postion trigger cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgPosCfg(pub u32);
    impl BrTrgPosCfg {
        #[doc = "1-trigger valid; 0-Trigger not valid."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1-trigger valid; 0-Trigger not valid."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "bit1: 0: (rising edge) pos inc greater than, 1: (falling edge) pos dec less than."]
        #[inline(always)]
        pub const fn edge(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "bit1: 0: (rising edge) pos inc greater than, 1: (falling edge) pos dec less than."]
        #[inline(always)]
        pub fn set_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for BrTrgPosCfg {
        #[inline(always)]
        fn default() -> BrTrgPosCfg {
            BrTrgPosCfg(0)
        }
    }
    #[doc = "Prediction Configuration postion threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgPosThr(pub u32);
    impl BrTrgPosThr {
        #[doc = "For pos out trigger (pos). ufix<32, 32>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "For pos out trigger (pos). ufix<32, 32>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrTrgPosThr {
        #[inline(always)]
        fn default() -> BrTrgPosThr {
            BrTrgPosThr(0)
        }
    }
    #[doc = "Prediction Configuration revolutiom threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgRevThr(pub u32);
    impl BrTrgRevThr {
        #[doc = "For pos out trigger (rev) ufix<32, 0>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "For pos out trigger (rev) ufix<32, 0>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrTrgRevThr {
        #[inline(always)]
        fn default() -> BrTrgRevThr {
            BrTrgRevThr(0)
        }
    }
    #[doc = "Prediction Configuration speed trigger cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgSpeedCfg(pub u32);
    impl BrTrgSpeedCfg {
        #[doc = "1-trigger valid; 0-Trigger not valid Normally it means either the max pos speed, or the min negative speed."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1-trigger valid; 0-Trigger not valid Normally it means either the max pos speed, or the min negative speed."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0: (rising edge) speed inc greater than, 1: (falling edge) speed dec less than."]
        #[inline(always)]
        pub const fn edge_sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0: (rising edge) speed inc greater than, 1: (falling edge) speed dec less than."]
        #[inline(always)]
        pub fn set_edge_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Use abs value for comparion. 0: Use the speed with direction info (so not the abs value)."]
        #[inline(always)]
        pub const fn comp_type(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Use abs value for comparion. 0: Use the speed with direction info (so not the abs value)."]
        #[inline(always)]
        pub fn set_comp_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for BrTrgSpeedCfg {
        #[inline(always)]
        fn default() -> BrTrgSpeedCfg {
            BrTrgSpeedCfg(0)
        }
    }
    #[doc = "Prediction Configuration speed threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BrTrgSpeedThr(pub u32);
    impl BrTrgSpeedThr {
        #[doc = "For speed trigger. continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "For speed trigger. continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BrTrgSpeedThr {
        #[inline(always)]
        fn default() -> BrTrgSpeedThr {
            BrTrgSpeedThr(0)
        }
    }
    #[doc = "Continuous Mode Configuration 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ContCfg0(pub u32);
    impl ContCfg0 {
        #[doc = "the theta for cal the clockwise or anticlockwise rotation between two adjacent inputs, ufix<32, 32>."]
        #[inline(always)]
        pub const fn half_circ_theta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the theta for cal the clockwise or anticlockwise rotation between two adjacent inputs, ufix<32, 32>."]
        #[inline(always)]
        pub fn set_half_circ_theta(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ContCfg0 {
        #[inline(always)]
        fn default() -> ContCfg0 {
            ContCfg0(0)
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Module Enable."]
        #[inline(always)]
        pub const fn mod_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Module Enable."]
        #[inline(always)]
        pub fn set_mod_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "1: Discrete position input 0: Continuous position input."]
        #[inline(always)]
        pub const fn discretetrc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: Discrete position input 0: Continuous position input."]
        #[inline(always)]
        pub fn set_discretetrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "1: use the input iposition whenever a new iposition comes, and force the predicted output stop at the boundaries. 0: Continuous tracking mode, without any boundary check."]
        #[inline(always)]
        pub const fn adjop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "1: use the input iposition whenever a new iposition comes, and force the predicted output stop at the boundaries. 0: Continuous tracking mode, without any boundary check."]
        #[inline(always)]
        pub fn set_adjop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "1: Shadow Request for read of tracking parameters. Auto clear 0:."]
        #[inline(always)]
        pub const fn shadow_rd_req(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "1: Shadow Request for read of tracking parameters. Auto clear 0:."]
        #[inline(always)]
        pub fn set_shadow_rd_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "1: Command to reload the coefs. Auto clear 0:."]
        #[inline(always)]
        pub const fn ini_coefs_cmd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "1: Command to reload the coefs. Auto clear 0:."]
        #[inline(always)]
        pub fn set_ini_coefs_cmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "1: change 0: won't change bit 2: for ACOEF bit 1: for ICOEF bit 0: for PCOEF."]
        #[inline(always)]
        pub const fn ini_coefs_cmd_msk(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "1: change 0: won't change bit 2: for ACOEF bit 1: for ICOEF bit 0: for PCOEF."]
        #[inline(always)]
        pub fn set_ini_coefs_cmd_msk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "1: Command to reload the positions. Auto clear 0:."]
        #[inline(always)]
        pub const fn ini_pos_req(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1: Command to reload the positions. Auto clear 0:."]
        #[inline(always)]
        pub fn set_ini_pos_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "1: change 0: won't change bit 3: for accel bit 2: for speed bit 1: for revolution bit 0: for position."]
        #[inline(always)]
        pub const fn ini_pos_cmd_msk(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "1: change 0: won't change bit 3: for accel bit 2: for speed bit 1: for revolution bit 0: for position."]
        #[inline(always)]
        pub fn set_ini_pos_cmd_msk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "1: 32-bit for rev+pos, with each element occupying 16 bits 0: 32-bit for rev, and 32 bit for pos When CR\\[MANUAL_IO\\]==1, 1: means that the INI_POS is acting as INI_POS cmds 0: means that the INI_POS is simulating the input of iposition and itimestamp."]
        #[inline(always)]
        pub const fn pos_type(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "1: 32-bit for rev+pos, with each element occupying 16 bits 0: 32-bit for rev, and 32 bit for pos When CR\\[MANUAL_IO\\]==1, 1: means that the INI_POS is acting as INI_POS cmds 0: means that the INI_POS is simulating the input of iposition and itimestamp."]
        #[inline(always)]
        pub fn set_pos_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "1: in open loop mode 0: not in open loop mode."]
        #[inline(always)]
        pub const fn open_loop_mode(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "1: in open loop mode 0: not in open loop mode."]
        #[inline(always)]
        pub fn set_open_loop_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "1: Command to reload the delta pos. Auto clear 0:."]
        #[inline(always)]
        pub const fn ini_delta_pos_req(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "1: Command to reload the delta pos. Auto clear 0:."]
        #[inline(always)]
        pub fn set_ini_delta_pos_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "1: change 0: won't change bit 3: for delta accel bit 2: for delta speed bit 1: for delta revolution bit 0: for delta position."]
        #[inline(always)]
        pub const fn ini_delta_pos_cmd_msk(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "1: change 0: won't change bit 3: for delta accel bit 2: for delta speed bit 1: for delta revolution bit 0: for delta position."]
        #[inline(always)]
        pub fn set_ini_delta_pos_cmd_msk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "0: Time Stamp in the configuration 1: Risedge of In Trg\\[0\\]
2: Risedge of In Trg\\[1\\]
3: Risedge of out trg\\[0\\]
4: Risedge of out trg\\[1\\]
5: triggered by self position trigger 6: triggered by self speed trigger Otherser: no function."]
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
        pub fn set_ini_pos_trg_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[doc = "0: Time Stamp in the configuration 1: Risedge of In Trg\\[0\\]
2: Risedge of In Trg\\[1\\]
3: Risedge of out trg\\[0\\]
4: Risedge of out trg\\[1\\]
5: triggered by self position trigger 6: triggered by self speed trigger Otherser: no function."]
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
        pub fn set_ini_delta_pos_trg_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
        }
        #[doc = "Multiple Coefficients Enable."]
        #[inline(always)]
        pub const fn ms_coef_en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Multiple Coefficients Enable."]
        #[inline(always)]
        pub fn set_ms_coef_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Zeroise the accelerator calculation."]
        #[inline(always)]
        pub const fn frcaccelzero(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Zeroise the accelerator calculation."]
        #[inline(always)]
        pub fn set_frcaccelzero(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Auto clear. Only effective in open_loop mode."]
        #[inline(always)]
        pub const fn ini_br1_pos_req(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Auto clear. Only effective in open_loop mode."]
        #[inline(always)]
        pub fn set_ini_br1_pos_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Auto clear. Only effective in open_loop mode."]
        #[inline(always)]
        pub const fn ini_br0_pos_req(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Auto clear. Only effective in open_loop mode."]
        #[inline(always)]
        pub fn set_ini_br0_pos_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Software reset, high active. When write 1 ,all internal logical will be reset. 0b - No action 1b - All MMC internal registers are forced into their reset state. Interface registers are not affected."]
        #[inline(always)]
        pub const fn sftrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset, high active. When write 1 ,all internal logical will be reset. 0b - No action 1b - All MMC internal registers are forced into their reset state. Interface registers are not affected."]
        #[inline(always)]
        pub fn set_sftrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "The coefficient A register for the internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CurAcoef(pub u32);
    impl CurAcoef {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CurAcoef {
        #[inline(always)]
        fn default() -> CurAcoef {
            CurAcoef(0)
        }
    }
    #[doc = "The coefficient I register for the internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CurIcoef(pub u32);
    impl CurIcoef {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CurIcoef {
        #[inline(always)]
        fn default() -> CurIcoef {
            CurIcoef(0)
        }
    }
    #[doc = "The coefficient P register for the internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CurPcoef(pub u32);
    impl CurPcoef {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CurPcoef {
        #[inline(always)]
        fn default() -> CurPcoef {
            CurPcoef(0)
        }
    }
    #[doc = "Discrete Mode Configuration 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DiscreteCfg0(pub u32);
    impl DiscreteCfg0 {
        #[doc = "Max ID Of Lines. For example-1, for 512 lines, it is 511. ufix<32, 0>."]
        #[inline(always)]
        pub const fn posmax(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Max ID Of Lines. For example-1, for 512 lines, it is 511. ufix<32, 0>."]
        #[inline(always)]
        pub fn set_posmax(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for DiscreteCfg0 {
        #[inline(always)]
        fn default() -> DiscreteCfg0 {
            DiscreteCfg0(0)
        }
    }
    #[doc = "Discrete Mode Configuration 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DiscreteCfg1(pub u32);
    impl DiscreteCfg1 {
        #[doc = "discrete mode: ufix<32, 0> of 1/(Number Of Lines) continuous mode: the max delta for tracking from the last received position, ufix<32, 32>."]
        #[inline(always)]
        pub const fn inv_posmax(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "discrete mode: ufix<32, 0> of 1/(Number Of Lines) continuous mode: the max delta for tracking from the last received position, ufix<32, 32>."]
        #[inline(always)]
        pub fn set_inv_posmax(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DiscreteCfg1 {
        #[inline(always)]
        fn default() -> DiscreteCfg1 {
            DiscreteCfg1(0)
        }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ErrThr(pub u32);
    impl ErrThr {
        #[doc = "ErrThr0: Error Threshold 0, (abs(tracking error)>= will choose the coefs as below) Note: ErrThr0>ErrThr1>ErrThr2 ufix<31, 28>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ErrThr0: Error Threshold 0, (abs(tracking error)>= will choose the coefs as below) Note: ErrThr0>ErrThr1>ErrThr2 ufix<31, 28>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ErrThr {
        #[inline(always)]
        fn default() -> ErrThr {
            ErrThr(0)
        }
    }
    #[doc = "The accelerator register for theinternal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EstmAccel(pub u32);
    impl EstmAccel {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EstmAccel {
        #[inline(always)]
        fn default() -> EstmAccel {
            EstmAccel(0)
        }
    }
    #[doc = "The position register for the internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EstmPos(pub u32);
    impl EstmPos {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EstmPos {
        #[inline(always)]
        fn default() -> EstmPos {
            EstmPos(0)
        }
    }
    #[doc = "The revolution register for the internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EstmRev(pub u32);
    impl EstmRev {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EstmRev {
        #[inline(always)]
        fn default() -> EstmRev {
            EstmRev(0)
        }
    }
    #[doc = "The speed register for the internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EstmSpeed(pub u32);
    impl EstmSpeed {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EstmSpeed {
        #[inline(always)]
        fn default() -> EstmSpeed {
            EstmSpeed(0)
        }
    }
    #[doc = "The timestamp register for internal estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EstmTim(pub u32);
    impl EstmTim {
        #[doc = "the value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EstmTim {
        #[inline(always)]
        fn default() -> EstmTim {
            EstmTim(0)
        }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 I."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I(pub u32);
    impl I {
        #[doc = "I0_Coef, fix<32, 21>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "I0_Coef, fix<32, 21>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for I {
        #[inline(always)]
        fn default() -> I {
            I(0)
        }
    }
    #[doc = "The destined accelerator register for position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniAccel(pub u32);
    impl IniAccel {
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniAccel {
        #[inline(always)]
        fn default() -> IniAccel {
            IniAccel(0)
        }
    }
    #[doc = "The destined coefficient A register for coefficients initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniAcoef(pub u32);
    impl IniAcoef {
        #[doc = "the value, fix<32, 19>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value, fix<32, 19>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniAcoef {
        #[inline(always)]
        fn default() -> IniAcoef {
            IniAcoef(0)
        }
    }
    #[doc = "The destined timestamp register for coefficients initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniCoefTime(pub u32);
    impl IniCoefTime {
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniCoefTime {
        #[inline(always)]
        fn default() -> IniCoefTime {
            IniCoefTime(0)
        }
    }
    #[doc = "The destined delta accelerator register for delta position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniDeltaAccel(pub u32);
    impl IniDeltaAccel {
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniDeltaAccel {
        #[inline(always)]
        fn default() -> IniDeltaAccel {
            IniDeltaAccel(0)
        }
    }
    #[doc = "The destined delta position register for delta position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniDeltaPos(pub u32);
    impl IniDeltaPos {
        #[doc = "the value continuous mode: ufix <32, 32>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: ufix <32, 32>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniDeltaPos {
        #[inline(always)]
        fn default() -> IniDeltaPos {
            IniDeltaPos(0)
        }
    }
    #[doc = "The destined timestamp register for delta position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniDeltaPosTime(pub u32);
    impl IniDeltaPosTime {
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniDeltaPosTime {
        #[inline(always)]
        fn default() -> IniDeltaPosTime {
            IniDeltaPosTime(0)
        }
    }
    #[doc = "The destined delta revolution register for delta position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniDeltaRev(pub u32);
    impl IniDeltaRev {
        #[doc = "the value continuous mode: fix<32, 0>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value continuous mode: fix<32, 0>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniDeltaRev {
        #[inline(always)]
        fn default() -> IniDeltaRev {
            IniDeltaRev(0)
        }
    }
    #[doc = "The destined delta speed register for delta position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniDeltaSpeed(pub u32);
    impl IniDeltaSpeed {
        #[doc = "the value； continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value； continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniDeltaSpeed {
        #[inline(always)]
        fn default() -> IniDeltaSpeed {
            IniDeltaSpeed(0)
        }
    }
    #[doc = "The destined coefficient I register for coefficients initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniIcoef(pub u32);
    impl IniIcoef {
        #[doc = "the value, fix<32, 21>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value, fix<32, 21>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniIcoef {
        #[inline(always)]
        fn default() -> IniIcoef {
            IniIcoef(0)
        }
    }
    #[doc = "The destined coefficient P register for coefficients initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniPcoef(pub u32);
    impl IniPcoef {
        #[doc = "the value, fix<32, 15>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value, fix<32, 15>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniPcoef {
        #[inline(always)]
        fn default() -> IniPcoef {
            IniPcoef(0)
        }
    }
    #[doc = "The destined position register for position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniPos(pub u32);
    impl IniPos {
        #[doc = "the value； continuous mode: ufix<32, 32>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value； continuous mode: ufix<32, 32>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniPos {
        #[inline(always)]
        fn default() -> IniPos {
            IniPos(0)
        }
    }
    #[doc = "The destined timestamp register for position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniPosTime(pub u32);
    impl IniPosTime {
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "indicate the time to change the values. 0: instant change."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniPosTime {
        #[inline(always)]
        fn default() -> IniPosTime {
            IniPosTime(0)
        }
    }
    #[doc = "The destined revolution register for position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniRev(pub u32);
    impl IniRev {
        #[doc = "the value； continuous mode: ufix<32, 0>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value； continuous mode: ufix<32, 0>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniRev {
        #[inline(always)]
        fn default() -> IniRev {
            IniRev(0)
        }
    }
    #[doc = "The destined speed register for position initialization."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IniSpeed(pub u32);
    impl IniSpeed {
        #[doc = "the value; continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the value; continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IniSpeed {
        #[inline(always)]
        fn default() -> IniSpeed {
            IniSpeed(0)
        }
    }
    #[doc = "Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "Interrupt Enable for SHADOW_RD_DONE."]
        #[inline(always)]
        pub const fn shadow_rd_done_ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for SHADOW_RD_DONE."]
        #[inline(always)]
        pub fn set_shadow_rd_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt Enable for INI_COEFS_CMD_DONE."]
        #[inline(always)]
        pub const fn ini_coefs_cmd_done_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for INI_COEFS_CMD_DONE."]
        #[inline(always)]
        pub fn set_ini_coefs_cmd_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt Enable for INI_POS_REQ_CMD_DONE."]
        #[inline(always)]
        pub const fn ini_pos_req_cmd_done_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for INI_POS_REQ_CMD_DONE."]
        #[inline(always)]
        pub fn set_ini_pos_req_cmd_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Interrupt Enable for OOSYNC."]
        #[inline(always)]
        pub const fn oosync_ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for OOSYNC."]
        #[inline(always)]
        pub fn set_oosync_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Interrupt Enable for INI_BR1_POS_REQ_CMD_DONE."]
        #[inline(always)]
        pub const fn ini_br1_pos_req_cmd_done_ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for INI_BR1_POS_REQ_CMD_DONE."]
        #[inline(always)]
        pub fn set_ini_br1_pos_req_cmd_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Interrupt Enable for INI_BR0_POS_REQ_CMD_DONE."]
        #[inline(always)]
        pub const fn ini_br0_pos_req_cmd_done_ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for INI_BR0_POS_REQ_CMD_DONE."]
        #[inline(always)]
        pub fn set_ini_br0_pos_req_cmd_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Interrupt Enable for INI_DELTA_POS_REQ_CMD_DONE."]
        #[inline(always)]
        pub const fn ini_delta_pos_req_cmd_done_ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for INI_DELTA_POS_REQ_CMD_DONE."]
        #[inline(always)]
        pub fn set_ini_delta_pos_req_cmd_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Interrupt Enable for POS_TRG_VALID."]
        #[inline(always)]
        pub const fn pos_trg_vld_ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for POS_TRG_VALID."]
        #[inline(always)]
        pub fn set_pos_trg_vld_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Interrupt Enable for SPEED_TRG_VALID."]
        #[inline(always)]
        pub const fn speed_trg_vld_ie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable for SPEED_TRG_VALID."]
        #[inline(always)]
        pub fn set_speed_trg_vld_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    #[doc = "Position Out-Of-Sync Threshold Regster."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OosyncThetaThr(pub u32);
    impl OosyncThetaThr {
        #[doc = "the threshold of theta difference between actual and prediction for out-of-sync determination，ufix<32, 32>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the threshold of theta difference between actual and prediction for out-of-sync determination，ufix<32, 32>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OosyncThetaThr {
        #[inline(always)]
        fn default() -> OosyncThetaThr {
            OosyncThetaThr(0)
        }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 P."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P(pub u32);
    impl P {
        #[doc = "P0_Coef, fix<32, 15>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "P0_Coef, fix<32, 15>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P {
        #[inline(always)]
        fn default() -> P {
            P(0)
        }
    }
    #[doc = "Tracking Configuration pos trigger cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosTrgCfg(pub u32);
    impl PosTrgCfg {
        #[doc = "1-trigger valid; 0-Trigger not valid\"."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1-trigger valid; 0-Trigger not valid\"."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0: (rising edge) pos inc greater than, 1: (falling edge) pos dec less than."]
        #[inline(always)]
        pub const fn edge(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0: (rising edge) pos inc greater than, 1: (falling edge) pos dec less than."]
        #[inline(always)]
        pub fn set_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for PosTrgCfg {
        #[inline(always)]
        fn default() -> PosTrgCfg {
            PosTrgCfg(0)
        }
    }
    #[doc = "Tracking Configuration position threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosTrgPosThr(pub u32);
    impl PosTrgPosThr {
        #[doc = "For pos out trigger (pos). ufix<32, 32>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "For pos out trigger (pos). ufix<32, 32>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosTrgPosThr {
        #[inline(always)]
        fn default() -> PosTrgPosThr {
            PosTrgPosThr(0)
        }
    }
    #[doc = "Tracking Configuration revolution threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosTrgRevThr(pub u32);
    impl PosTrgRevThr {
        #[doc = "For pos out trigger (rev) fix<32, 0>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "For pos out trigger (rev) fix<32, 0>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosTrgRevThr {
        #[inline(always)]
        fn default() -> PosTrgRevThr {
            PosTrgRevThr(0)
        }
    }
    #[doc = "Tracking Configuration speed trigger cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpeedTrgCfg(pub u32);
    impl SpeedTrgCfg {
        #[doc = "1-trigger valid; 0-Trigger not valid Normally it means either the max pos speed, or the min negative speed."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1-trigger valid; 0-Trigger not valid Normally it means either the max pos speed, or the min negative speed."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0: (rising edge) speed inc greater than, 1: (falling edge) speed dec less than."]
        #[inline(always)]
        pub const fn edge(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0: (rising edge) speed inc greater than, 1: (falling edge) speed dec less than."]
        #[inline(always)]
        pub fn set_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "1: Use abs value for comparion. 0: Use the speed with direction info (so not the abs value)."]
        #[inline(always)]
        pub const fn comp_type(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "1: Use abs value for comparion. 0: Use the speed with direction info (so not the abs value)."]
        #[inline(always)]
        pub fn set_comp_type(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for SpeedTrgCfg {
        #[inline(always)]
        fn default() -> SpeedTrgCfg {
            SpeedTrgCfg(0)
        }
    }
    #[doc = "Tracking Configuration speed threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpeedTrgThr(pub u32);
    impl SpeedTrgThr {
        #[doc = "For speed trigger. continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "For speed trigger. continuous mode: fix<32, 19>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SpeedTrgThr {
        #[inline(always)]
        fn default() -> SpeedTrgThr {
            SpeedTrgThr(0)
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sta(pub u32);
    impl Sta {
        #[doc = "Shadow ready for read. Auto cleared by setting CR\\[SHADOW_RD_REQ\\]
as 1."]
        #[inline(always)]
        pub const fn shadow_rd_done(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Shadow ready for read. Auto cleared by setting CR\\[SHADOW_RD_REQ\\]
as 1."]
        #[inline(always)]
        pub fn set_shadow_rd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn ini_coefs_cmd_done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub fn set_ini_coefs_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn ini_pos_req_cmd_done(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub fn set_ini_pos_req_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Tracking module out-of sync. W1C."]
        #[inline(always)]
        pub const fn oosync(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Tracking module out-of sync. W1C."]
        #[inline(always)]
        pub fn set_oosync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Tracking Module in Idle status."]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Tracking Module in Idle status."]
        #[inline(always)]
        pub fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn ini_br1_pos_req_cmd_done(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub fn set_ini_br1_pos_req_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn ini_br0_pos_req_cmd_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub fn set_ini_br0_pos_req_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn ini_delta_pos_req_cmd_done(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub fn set_ini_delta_pos_req_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn pos_trg_valid(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub fn set_pos_trg_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub const fn speed_trg_valid(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "W1C."]
        #[inline(always)]
        pub fn set_speed_trg_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Tracking ERR_ID."]
        #[inline(always)]
        pub const fn err_id(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Tracking ERR_ID."]
        #[inline(always)]
        pub fn set_err_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Sta {
        #[inline(always)]
        fn default() -> Sta {
            Sta(0)
        }
    }
    #[doc = "System Clock Frequency Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SysclkFreq(pub u32);
    impl SysclkFreq {
        #[doc = "system clock frequency, ufix<32, 0>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "system clock frequency, ufix<32, 0>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SysclkFreq {
        #[inline(always)]
        fn default() -> SysclkFreq {
            SysclkFreq(0)
        }
    }
    #[doc = "System Clock Period Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SysclkPeriod(pub u32);
    impl SysclkPeriod {
        #[doc = "round( the value of clock period * (2^24)*(2^20) ), ufix<32, 0>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "round( the value of clock period * (2^24)*(2^20) ), ufix<32, 0>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SysclkPeriod {
        #[inline(always)]
        fn default() -> SysclkPeriod {
            SysclkPeriod(0)
        }
    }
    #[doc = "Tracking Configuration coef trigger cfg&index0 time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Time(pub u32);
    impl Time {
        #[doc = "CoefTime0: Time Stayed using this coefs (counted in input samples). Ideal value of tracing cycles should +1. ufix<32,0>."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CoefTime0: Time Stayed using this coefs (counted in input samples). Ideal value of tracing cycles should +1. ufix<32,0>."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Time {
        #[inline(always)]
        fn default() -> Time {
            Time(0)
        }
    }
}
