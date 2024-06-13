#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RDC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdc {
    ptr: *mut u8,
}
unsafe impl Send for Rdc {}
unsafe impl Sync for Rdc {}
impl Rdc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "rdc control."]
    #[inline(always)]
    pub const fn rdc_ctl(self) -> crate::common::Reg<regs::RdcCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "accumulate result of i_channel."]
    #[inline(always)]
    pub const fn acc_i(self) -> crate::common::Reg<regs::AccI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "accumulate result of q_channel."]
    #[inline(always)]
    pub const fn acc_q(self) -> crate::common::Reg<regs::AccQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "input channel selection."]
    #[inline(always)]
    pub const fn in_ctl(self) -> crate::common::Reg<regs::InCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "output channel selection."]
    #[inline(always)]
    pub const fn out_ctl(self) -> crate::common::Reg<regs::OutCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "excitation signal timming setting."]
    #[inline(always)]
    pub const fn exc_timming(self) -> crate::common::Reg<regs::ExcTimming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "amplitude scaling for excitation."]
    #[inline(always)]
    pub const fn exc_scaling(self) -> crate::common::Reg<regs::ExcScaling, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "amplitude offset setting."]
    #[inline(always)]
    pub const fn exc_offset(self) -> crate::common::Reg<regs::ExcOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "amplitude scaling for excitation."]
    #[inline(always)]
    pub const fn pwm_scaling(self) -> crate::common::Reg<regs::PwmScaling, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "amplitude offset setting."]
    #[inline(always)]
    pub const fn pwm_offset(self) -> crate::common::Reg<regs::PwmOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Configuration for trigger out 0 in clock cycle."]
    #[inline(always)]
    pub const fn trig_out0_cfg(self) -> crate::common::Reg<regs::TrigOut0Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Configuration for trigger out 1 in clock cycle."]
    #[inline(always)]
    pub const fn trig_out1_cfg(self) -> crate::common::Reg<regs::TrigOut1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "pwm dead zone control in clock cycle."]
    #[inline(always)]
    pub const fn pwm_dz(self) -> crate::common::Reg<regs::PwmDz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "synchronize output signal control."]
    #[inline(always)]
    pub const fn sync_out_ctrl(self) -> crate::common::Reg<regs::SyncOutCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "trigger in delay timming in soc bus cycle."]
    #[inline(always)]
    pub const fn exc_sync_dly(self) -> crate::common::Reg<regs::ExcSyncDly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "max value of i_channel."]
    #[inline(always)]
    pub const fn max_i(self) -> crate::common::Reg<regs::MaxI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "min value of i_channel."]
    #[inline(always)]
    pub const fn min_i(self) -> crate::common::Reg<regs::MinI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "max value of q_channel."]
    #[inline(always)]
    pub const fn max_q(self) -> crate::common::Reg<regs::MaxQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "min value of q_channel."]
    #[inline(always)]
    pub const fn min_q(self) -> crate::common::Reg<regs::MinQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "the offset setting for edge detection of the i_channel."]
    #[inline(always)]
    pub const fn thrs_i(self) -> crate::common::Reg<regs::ThrsI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "the offset setting for edge detection of the q_channel."]
    #[inline(always)]
    pub const fn thrs_q(self) -> crate::common::Reg<regs::ThrsQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "the control for edge detection."]
    #[inline(always)]
    pub const fn edg_det_ctl(self) -> crate::common::Reg<regs::EdgDetCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "scaling for accumulation result."]
    #[inline(always)]
    pub const fn acc_scaling(self) -> crate::common::Reg<regs::AccScaling, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "period of excitation."]
    #[inline(always)]
    pub const fn exc_period(self) -> crate::common::Reg<regs::ExcPeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "delay setting in clock cycle for synchronous signal."]
    #[inline(always)]
    pub const fn sync_delay_i(self) -> crate::common::Reg<regs::SyncDelayI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and rising edge of i_channel data."]
    #[inline(always)]
    pub const fn rise_delay_i(self) -> crate::common::Reg<regs::RiseDelayI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and falling edge of i_channel data."]
    #[inline(always)]
    pub const fn fall_delay_i(self) -> crate::common::Reg<regs::FallDelayI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "sample value on rising edge of rectify signal."]
    #[inline(always)]
    pub const fn sample_rise_i(self) -> crate::common::Reg<regs::SampleRiseI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "sample value on falling edge of rectify signal."]
    #[inline(always)]
    pub const fn sample_fall_i(self) -> crate::common::Reg<regs::SampleFallI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "number of accumulation."]
    #[inline(always)]
    pub const fn acc_cnt_i(self) -> crate::common::Reg<regs::AccCntI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "sample counter of opposite sign with rectify signal."]
    #[inline(always)]
    pub const fn sign_cnt_i(self) -> crate::common::Reg<regs::SignCntI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "delay setting in clock cycle for synchronous signal."]
    #[inline(always)]
    pub const fn sync_delay_q(self) -> crate::common::Reg<regs::SyncDelayQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and rising edge of q_channel data."]
    #[inline(always)]
    pub const fn rise_delay_q(self) -> crate::common::Reg<regs::RiseDelayQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and falling edge of q_channel data."]
    #[inline(always)]
    pub const fn fall_delay_q(self) -> crate::common::Reg<regs::FallDelayQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "sample value on rising edge of rectify signal."]
    #[inline(always)]
    pub const fn sample_rise_q(self) -> crate::common::Reg<regs::SampleRiseQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "sample value on falling edge of rectify signal."]
    #[inline(always)]
    pub const fn sample_fall_q(self) -> crate::common::Reg<regs::SampleFallQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "number of accumulation."]
    #[inline(always)]
    pub const fn acc_cnt_q(self) -> crate::common::Reg<regs::AccCntQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "sample counter of opposite sign with rectify signal."]
    #[inline(always)]
    pub const fn sign_cnt_q(self) -> crate::common::Reg<regs::SignCntQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "the maximum of acc amplitude."]
    #[inline(always)]
    pub const fn amp_max(self) -> crate::common::Reg<regs::AmpMax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "the minimum of acc amplitude."]
    #[inline(always)]
    pub const fn amp_min(self) -> crate::common::Reg<regs::AmpMin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "the interrupt mask control."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "the interrupt state."]
    #[inline(always)]
    pub const fn adc_int_state(self) -> crate::common::Reg<regs::AdcIntState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
}
pub mod regs {
    #[doc = "number of accumulation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccCntI(pub u32);
    impl AccCntI {
        #[doc = "sample number during the positive of rectify signal 1: 1 2: 2 …."]
        #[inline(always)]
        pub const fn cnt_pos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "sample number during the positive of rectify signal 1: 1 2: 2 …."]
        #[inline(always)]
        pub fn set_cnt_pos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "sample number during the negtive of rectify signal 1: 1 2: 2 …."]
        #[inline(always)]
        pub const fn cnt_neg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "sample number during the negtive of rectify signal 1: 1 2: 2 …."]
        #[inline(always)]
        pub fn set_cnt_neg(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for AccCntI {
        #[inline(always)]
        fn default() -> AccCntI {
            AccCntI(0)
        }
    }
    #[doc = "number of accumulation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccCntQ(pub u32);
    impl AccCntQ {
        #[doc = "sample number during the positive of rectify signal 1: 1 2: 2 …."]
        #[inline(always)]
        pub const fn cnt_pos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "sample number during the positive of rectify signal 1: 1 2: 2 …."]
        #[inline(always)]
        pub fn set_cnt_pos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "sample number during the negtive of rectify signal 1: 1 2: 2 …."]
        #[inline(always)]
        pub const fn cnt_neg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "sample number during the negtive of rectify signal 1: 1 2: 2 …."]
        #[inline(always)]
        pub fn set_cnt_neg(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for AccCntQ {
        #[inline(always)]
        fn default() -> AccCntQ {
            AccCntQ(0)
        }
    }
    #[doc = "accumulate result of i_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccI(pub u32);
    impl AccI {
        #[doc = "accumulate result of i_channel, this is a signed number."]
        #[inline(always)]
        pub const fn acc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "accumulate result of i_channel, this is a signed number."]
        #[inline(always)]
        pub fn set_acc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AccI {
        #[inline(always)]
        fn default() -> AccI {
            AccI(0)
        }
    }
    #[doc = "accumulate result of q_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccQ(pub u32);
    impl AccQ {
        #[doc = "accumulate result of q_channel, this is a signed number."]
        #[inline(always)]
        pub const fn acc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "accumulate result of q_channel, this is a signed number."]
        #[inline(always)]
        pub fn set_acc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AccQ {
        #[inline(always)]
        fn default() -> AccQ {
            AccQ(0)
        }
    }
    #[doc = "scaling for accumulation result."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccScaling(pub u32);
    impl AccScaling {
        #[doc = "Accumulation value shift control, this is a sign number. 0: {acc\\[39\\],acc\\[38:8\\]} 1: {acc\\[39\\],acc\\[37:7\\]} 2: {acc\\[39\\],acc\\[36:6\\]} … 7: {acc\\[39\\],acc\\[31:1\\]} 8: {acc\\[39\\],acc\\[30:0\\]} 9: acc/2^9 10: acc/2^10 … 15:acc/2^15."]
        #[inline(always)]
        pub const fn acc_shift(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Accumulation value shift control, this is a sign number. 0: {acc\\[39\\],acc\\[38:8\\]} 1: {acc\\[39\\],acc\\[37:7\\]} 2: {acc\\[39\\],acc\\[36:6\\]} … 7: {acc\\[39\\],acc\\[31:1\\]} 8: {acc\\[39\\],acc\\[30:0\\]} 9: acc/2^9 10: acc/2^10 … 15:acc/2^15."]
        #[inline(always)]
        pub fn set_acc_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Toxic accumulation data be removed control 1: enable 0: disable."]
        #[inline(always)]
        pub const fn toxic_lk(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Toxic accumulation data be removed control 1: enable 0: disable."]
        #[inline(always)]
        pub fn set_toxic_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for AccScaling {
        #[inline(always)]
        fn default() -> AccScaling {
            AccScaling(0)
        }
    }
    #[doc = "the interrupt state."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcIntState(pub u32);
    impl AdcIntState {
        #[doc = "accumulate ample underflow interrupt status."]
        #[inline(always)]
        pub const fn acc_amp_ovl_sta(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "accumulate ample underflow interrupt status."]
        #[inline(always)]
        pub fn set_acc_amp_ovl_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "accumulate ample overflow interrupt status."]
        #[inline(always)]
        pub const fn acc_amp_ovh_sta(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "accumulate ample overflow interrupt status."]
        #[inline(always)]
        pub fn set_acc_amp_ovh_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "q_channel accumulate underflow interrupt status."]
        #[inline(always)]
        pub const fn acc_vld_q_ovl_sta(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel accumulate underflow interrupt status."]
        #[inline(always)]
        pub fn set_acc_vld_q_ovl_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "i_channel accumulate underflow interrupt status."]
        #[inline(always)]
        pub const fn acc_vld_i_ovl_sta(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel accumulate underflow interrupt status."]
        #[inline(always)]
        pub fn set_acc_vld_i_ovl_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "q_channel accumulate overflow interrupt status."]
        #[inline(always)]
        pub const fn acc_vld_q_ovh_sta(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel accumulate overflow interrupt status."]
        #[inline(always)]
        pub fn set_acc_vld_q_ovh_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "i_channel accumulate overflow interrupt status."]
        #[inline(always)]
        pub const fn acc_vld_i_ovh_sta(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel accumulate overflow interrupt status."]
        #[inline(always)]
        pub fn set_acc_vld_i_ovh_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "q_channel falling edge interrupt status."]
        #[inline(always)]
        pub const fn sample_falling_q_sta(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel falling edge interrupt status."]
        #[inline(always)]
        pub fn set_sample_falling_q_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "q_channel rising edge interrupt status."]
        #[inline(always)]
        pub const fn sample_rising_q_sta(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel rising edge interrupt status."]
        #[inline(always)]
        pub fn set_sample_rising_q_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "i_channel falling edge interrupt status."]
        #[inline(always)]
        pub const fn sample_falling_i_sta(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel falling edge interrupt status."]
        #[inline(always)]
        pub fn set_sample_falling_i_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "i_channel rising edge interrupt status."]
        #[inline(always)]
        pub const fn sample_rising_i_sta(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel rising edge interrupt status."]
        #[inline(always)]
        pub fn set_sample_rising_i_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "q_channel delayed rectify signal falling edge interrupt status."]
        #[inline(always)]
        pub const fn falling_delay_q_sta(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel delayed rectify signal falling edge interrupt status."]
        #[inline(always)]
        pub fn set_falling_delay_q_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "q_channel delayed rectify signal rising edge interrupt status."]
        #[inline(always)]
        pub const fn rising_delay_q_sta(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel delayed rectify signal rising edge interrupt status."]
        #[inline(always)]
        pub fn set_rising_delay_q_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "i_channel delayed rectify signal falling edge interrupt status."]
        #[inline(always)]
        pub const fn falling_delay_i_sta(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel delayed rectify signal falling edge interrupt status."]
        #[inline(always)]
        pub fn set_falling_delay_i_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "i_channel delayed rectify signal rising edge interrupt status."]
        #[inline(always)]
        pub const fn rising_delay_i_sta(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel delayed rectify signal rising edge interrupt status."]
        #[inline(always)]
        pub fn set_rising_delay_i_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "q_channel accumulate valid interrupt status for i_channel."]
        #[inline(always)]
        pub const fn acc_vld_q_sta(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel accumulate valid interrupt status for i_channel."]
        #[inline(always)]
        pub fn set_acc_vld_q_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "i_channel accumulate valid interrupt status for i_channel."]
        #[inline(always)]
        pub const fn acc_vld_i_sta(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel accumulate valid interrupt status for i_channel."]
        #[inline(always)]
        pub fn set_acc_vld_i_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for AdcIntState {
        #[inline(always)]
        fn default() -> AdcIntState {
            AdcIntState(0)
        }
    }
    #[doc = "the maximum of acc amplitude."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AmpMax(pub u32);
    impl AmpMax {
        #[doc = "the maximum of acc amplitude."]
        #[inline(always)]
        pub const fn max(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the maximum of acc amplitude."]
        #[inline(always)]
        pub fn set_max(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AmpMax {
        #[inline(always)]
        fn default() -> AmpMax {
            AmpMax(0)
        }
    }
    #[doc = "the minimum of acc amplitude."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AmpMin(pub u32);
    impl AmpMin {
        #[doc = "the minimum of acc amplitude."]
        #[inline(always)]
        pub const fn min(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the minimum of acc amplitude."]
        #[inline(always)]
        pub fn set_min(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AmpMin {
        #[inline(always)]
        fn default() -> AmpMin {
            AmpMin(0)
        }
    }
    #[doc = "the control for edge detection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EdgDetCtl(pub u32);
    impl EdgDetCtl {
        #[doc = "The continuous positive or negative number for edge detection 0: 1 1: 2 … 7: 8."]
        #[inline(always)]
        pub const fn filter(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "The continuous positive or negative number for edge detection 0: 1 1: 2 … 7: 8."]
        #[inline(always)]
        pub fn set_filter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "The minimum edge distance in sample 0:1 sample 1:2 sample 2:3 samples … 63:64 samples."]
        #[inline(always)]
        pub const fn hold(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x3f;
            val as u8
        }
        #[doc = "The minimum edge distance in sample 0:1 sample 1:2 sample 2:3 samples … 63:64 samples."]
        #[inline(always)]
        pub fn set_hold(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
        }
    }
    impl Default for EdgDetCtl {
        #[inline(always)]
        fn default() -> EdgDetCtl {
            EdgDetCtl(0)
        }
    }
    #[doc = "amplitude offset setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExcOffset(pub u32);
    impl ExcOffset {
        #[doc = "Offset for excitation."]
        #[inline(always)]
        pub const fn amp_offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Offset for excitation."]
        #[inline(always)]
        pub fn set_amp_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for ExcOffset {
        #[inline(always)]
        fn default() -> ExcOffset {
            ExcOffset(0)
        }
    }
    #[doc = "period of excitation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExcPeriod(pub u32);
    impl ExcPeriod {
        #[doc = "The num in clock cycle for period of excitation 0: invalid value 1:1 cycle 2:2 cycles …."]
        #[inline(always)]
        pub const fn exc_period(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The num in clock cycle for period of excitation 0: invalid value 1:1 cycle 2:2 cycles …."]
        #[inline(always)]
        pub fn set_exc_period(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ExcPeriod {
        #[inline(always)]
        fn default() -> ExcPeriod {
            ExcPeriod(0)
        }
    }
    #[doc = "amplitude scaling for excitation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExcScaling(pub u32);
    impl ExcScaling {
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[inline(always)]
        pub const fn amp_man(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[inline(always)]
        pub fn set_amp_man(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[inline(always)]
        pub const fn amp_exp(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[inline(always)]
        pub fn set_amp_exp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for ExcScaling {
        #[inline(always)]
        fn default() -> ExcScaling {
            ExcScaling(0)
        }
    }
    #[doc = "trigger in delay timming in soc bus cycle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExcSyncDly(pub u32);
    impl ExcSyncDly {
        #[doc = "Trigger in delay timming in bus cycle from rising edge of trigger signal 0: 1 cycle 1: 2 cycle … 0xffffff: 2^24 cycle."]
        #[inline(always)]
        pub const fn delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Trigger in delay timming in bus cycle from rising edge of trigger signal 0: 1 cycle 1: 2 cycle … 0xffffff: 2^24 cycle."]
        #[inline(always)]
        pub fn set_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "Disable hardware trigger input 0: enable 1: disable."]
        #[inline(always)]
        pub const fn disable(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Disable hardware trigger input 0: enable 1: disable."]
        #[inline(always)]
        pub fn set_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for ExcSyncDly {
        #[inline(always)]
        fn default() -> ExcSyncDly {
            ExcSyncDly(0)
        }
    }
    #[doc = "excitation signal timming setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExcTimming(pub u32);
    impl ExcTimming {
        #[doc = "The period for excitation sample in clock cycle， 0: not allowed 1: 1 cycle 2: 2 cycles … 65535 : 65535 cycles."]
        #[inline(always)]
        pub const fn smp_rate(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The period for excitation sample in clock cycle， 0: not allowed 1: 1 cycle 2: 2 cycles … 65535 : 65535 cycles."]
        #[inline(always)]
        pub fn set_smp_rate(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Number of sample every excitation period 0: 4 point 1: 8 point … 8: 1024 point."]
        #[inline(always)]
        pub const fn smp_num(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of sample every excitation period 0: 4 point 1: 8 point … 8: 1024 point."]
        #[inline(always)]
        pub fn set_smp_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Pwm period in samples， 0：1 sample period 1: 2 sample period ... 15: 16 sample period."]
        #[inline(always)]
        pub const fn pwm_prd(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Pwm period in samples， 0：1 sample period 1: 2 sample period ... 15: 16 sample period."]
        #[inline(always)]
        pub fn set_pwm_prd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Swap output of PWM and DAC 0: disable swap 1: swap output."]
        #[inline(always)]
        pub const fn swap(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Swap output of PWM and DAC 0: disable swap 1: swap output."]
        #[inline(always)]
        pub fn set_swap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for ExcTimming {
        #[inline(always)]
        fn default() -> ExcTimming {
            ExcTimming(0)
        }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and falling edge of i_channel data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FallDelayI(pub u32);
    impl FallDelayI {
        #[doc = "Delay value on falling edge of i_channel data 0: 1 cycle 1: 2 cycles …."]
        #[inline(always)]
        pub const fn fall_delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Delay value on falling edge of i_channel data 0: 1 cycle 1: 2 cycles …."]
        #[inline(always)]
        pub fn set_fall_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FallDelayI {
        #[inline(always)]
        fn default() -> FallDelayI {
            FallDelayI(0)
        }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and falling edge of q_channel data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FallDelayQ(pub u32);
    impl FallDelayQ {
        #[doc = "Delay value on falling edge of q_channel data 0: 1 cycle 1: 2 cycles …."]
        #[inline(always)]
        pub const fn fall_delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Delay value on falling edge of q_channel data 0: 1 cycle 1: 2 cycles …."]
        #[inline(always)]
        pub fn set_fall_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FallDelayQ {
        #[inline(always)]
        fn default() -> FallDelayQ {
            FallDelayQ(0)
        }
    }
    #[doc = "input channel selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InCtl(pub u32);
    impl InCtl {
        #[doc = "Input channel selection for i_channel 0: channel 0 selected 1: channel 1 selected … 31: channel 31 selected."]
        #[inline(always)]
        pub const fn ch_i_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel selection for i_channel 0: channel 0 selected 1: channel 1 selected … 31: channel 31 selected."]
        #[inline(always)]
        pub fn set_ch_i_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Input port selection for i_channel, 0:sel port0 1:sel port1."]
        #[inline(always)]
        pub const fn port_i_sel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Input port selection for i_channel, 0:sel port0 1:sel port1."]
        #[inline(always)]
        pub fn set_port_i_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Input channel selection for q_channel 0: channel 0 selected 1: channel 1 selected … 31: channel 31 selected."]
        #[inline(always)]
        pub const fn ch_q_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel selection for q_channel 0: channel 0 selected 1: channel 1 selected … 31: channel 31 selected."]
        #[inline(always)]
        pub fn set_ch_q_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
        }
        #[doc = "Input port selection for q_channel, 0:sel port0 1:sel port1."]
        #[inline(always)]
        pub const fn port_q_sel(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Input port selection for q_channel, 0:sel port0 1:sel port1."]
        #[inline(always)]
        pub fn set_port_q_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for InCtl {
        #[inline(always)]
        fn default() -> InCtl {
            InCtl(0)
        }
    }
    #[doc = "the interrupt mask control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "accumulate ample underflow interrupt enable."]
        #[inline(always)]
        pub const fn acc_amp_ovl_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "accumulate ample underflow interrupt enable."]
        #[inline(always)]
        pub fn set_acc_amp_ovl_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "accumulate ample overflow interrupt enable."]
        #[inline(always)]
        pub const fn acc_amp_ovh_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "accumulate ample overflow interrupt enable."]
        #[inline(always)]
        pub fn set_acc_amp_ovh_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "q_channel accumulate underflow interrupt enable."]
        #[inline(always)]
        pub const fn acc_vld_q_ovl_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel accumulate underflow interrupt enable."]
        #[inline(always)]
        pub fn set_acc_vld_q_ovl_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "i_channel accumulate underflow interrupt enable."]
        #[inline(always)]
        pub const fn acc_vld_i_ovl_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel accumulate underflow interrupt enable."]
        #[inline(always)]
        pub fn set_acc_vld_i_ovl_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "q_channel accumulate overflow interrupt enable."]
        #[inline(always)]
        pub const fn acc_vld_q_ovh_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel accumulate overflow interrupt enable."]
        #[inline(always)]
        pub fn set_acc_vld_q_ovh_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "i_channel accumulate overflow interrupt enable."]
        #[inline(always)]
        pub const fn acc_vld_i_ovh_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel accumulate overflow interrupt enable."]
        #[inline(always)]
        pub fn set_acc_vld_i_ovh_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "q_channel falling edge interrupt enable."]
        #[inline(always)]
        pub const fn sample_falling_q_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel falling edge interrupt enable."]
        #[inline(always)]
        pub fn set_sample_falling_q_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "q_channel rising edge interrupt enable."]
        #[inline(always)]
        pub const fn sample_rising_q_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel rising edge interrupt enable."]
        #[inline(always)]
        pub fn set_sample_rising_q_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "i_channel falling edge interrupt enable."]
        #[inline(always)]
        pub const fn sample_falling_i_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel falling edge interrupt enable."]
        #[inline(always)]
        pub fn set_sample_falling_i_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "i_channel rising edge interrupt enable."]
        #[inline(always)]
        pub const fn sample_rising_i_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel rising edge interrupt enable."]
        #[inline(always)]
        pub fn set_sample_rising_i_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "q_channel delayed rectify signal falling edge interrupt enable."]
        #[inline(always)]
        pub const fn falling_delay_q_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel delayed rectify signal falling edge interrupt enable."]
        #[inline(always)]
        pub fn set_falling_delay_q_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "q_channel delayed rectify signal rising edge interrupt enable."]
        #[inline(always)]
        pub const fn rising_delay_q_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel delayed rectify signal rising edge interrupt enable."]
        #[inline(always)]
        pub fn set_rising_delay_q_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "i_channel delayed rectify signal falling edge interrupt enable."]
        #[inline(always)]
        pub const fn falling_delay_i_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel delayed rectify signal falling edge interrupt enable."]
        #[inline(always)]
        pub fn set_falling_delay_i_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "i_channel delayed rectify signal rising edge interrupt enable."]
        #[inline(always)]
        pub const fn rising_delay_i_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel delayed rectify signal rising edge interrupt enable."]
        #[inline(always)]
        pub fn set_rising_delay_i_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "q_channel accumulate valid interrupt enable for i_channel."]
        #[inline(always)]
        pub const fn acc_vld_q_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel accumulate valid interrupt enable for i_channel."]
        #[inline(always)]
        pub fn set_acc_vld_q_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "i_channel accumulate valid interrupt enable for i_channel."]
        #[inline(always)]
        pub const fn acc_vld_i_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel accumulate valid interrupt enable for i_channel."]
        #[inline(always)]
        pub fn set_acc_vld_i_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "enable interrupt output."]
        #[inline(always)]
        pub const fn int_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt output."]
        #[inline(always)]
        pub fn set_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    #[doc = "max value of i_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MaxI(pub u32);
    impl MaxI {
        #[doc = "Max value valid, write clear 0: max value is not valid 1: max value is valid."]
        #[inline(always)]
        pub const fn valid(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Max value valid, write clear 0: max value is not valid 1: max value is valid."]
        #[inline(always)]
        pub fn set_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Max value of i_channel, write clear."]
        #[inline(always)]
        pub const fn max(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Max value of i_channel, write clear."]
        #[inline(always)]
        pub fn set_max(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for MaxI {
        #[inline(always)]
        fn default() -> MaxI {
            MaxI(0)
        }
    }
    #[doc = "max value of q_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MaxQ(pub u32);
    impl MaxQ {
        #[doc = "Max value valid, write clear 0: max value is not valid 1: max value is valid."]
        #[inline(always)]
        pub const fn valid(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Max value valid, write clear 0: max value is not valid 1: max value is valid."]
        #[inline(always)]
        pub fn set_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Max value of q_channel, write clear."]
        #[inline(always)]
        pub const fn max(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Max value of q_channel, write clear."]
        #[inline(always)]
        pub fn set_max(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for MaxQ {
        #[inline(always)]
        fn default() -> MaxQ {
            MaxQ(0)
        }
    }
    #[doc = "min value of i_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MinI(pub u32);
    impl MinI {
        #[doc = "Min value valid, write clear 0: min value is not valid 1: min value is valid."]
        #[inline(always)]
        pub const fn valid(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Min value valid, write clear 0: min value is not valid 1: min value is valid."]
        #[inline(always)]
        pub fn set_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Min value of i_channel, write clear."]
        #[inline(always)]
        pub const fn min(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Min value of i_channel, write clear."]
        #[inline(always)]
        pub fn set_min(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for MinI {
        #[inline(always)]
        fn default() -> MinI {
            MinI(0)
        }
    }
    #[doc = "min value of q_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MinQ(pub u32);
    impl MinQ {
        #[doc = "Min value valid, write clear 0: min value is not valid 1: min value is valid."]
        #[inline(always)]
        pub const fn valid(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Min value valid, write clear 0: min value is not valid 1: min value is valid."]
        #[inline(always)]
        pub fn set_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Min value of q_channel, write clear."]
        #[inline(always)]
        pub const fn min(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Min value of q_channel, write clear."]
        #[inline(always)]
        pub fn set_min(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for MinQ {
        #[inline(always)]
        fn default() -> MinQ {
            MinQ(0)
        }
    }
    #[doc = "output channel selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutCtl(pub u32);
    impl OutCtl {
        #[doc = "Output channel selection for i_channel."]
        #[inline(always)]
        pub const fn ch_i_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Output channel selection for i_channel."]
        #[inline(always)]
        pub fn set_ch_i_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Output channel selection for q_channel."]
        #[inline(always)]
        pub const fn ch_q_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Output channel selection for q_channel."]
        #[inline(always)]
        pub fn set_ch_q_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for OutCtl {
        #[inline(always)]
        fn default() -> OutCtl {
            OutCtl(0)
        }
    }
    #[doc = "pwm dead zone control in clock cycle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmDz(pub u32);
    impl PwmDz {
        #[doc = "Exc_p dead zone in clock cycle before swap 0: no dead zone 1: 1 cycle dead zone 2: 2 cycle dead zone …."]
        #[inline(always)]
        pub const fn dz_p(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Exc_p dead zone in clock cycle before swap 0: no dead zone 1: 1 cycle dead zone 2: 2 cycle dead zone …."]
        #[inline(always)]
        pub fn set_dz_p(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Exc_n dead zone in clock cycle before swap 0: no dead zone 1: 1 cycle dead zone 2: 2 cycle dead zone …."]
        #[inline(always)]
        pub const fn dz_n(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Exc_n dead zone in clock cycle before swap 0: no dead zone 1: 1 cycle dead zone 2: 2 cycle dead zone …."]
        #[inline(always)]
        pub fn set_dz_n(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for PwmDz {
        #[inline(always)]
        fn default() -> PwmDz {
            PwmDz(0)
        }
    }
    #[doc = "amplitude offset setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmOffset(pub u32);
    impl PwmOffset {
        #[doc = "Offset for excitation."]
        #[inline(always)]
        pub const fn amp_offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Offset for excitation."]
        #[inline(always)]
        pub fn set_amp_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for PwmOffset {
        #[inline(always)]
        fn default() -> PwmOffset {
            PwmOffset(0)
        }
    }
    #[doc = "amplitude scaling for excitation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmScaling(pub u32);
    impl PwmScaling {
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[inline(always)]
        pub const fn amp_man(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[inline(always)]
        pub fn set_amp_man(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[inline(always)]
        pub const fn amp_exp(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[inline(always)]
        pub fn set_amp_exp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Enable dither of pwm 0: disable 1: enable."]
        #[inline(always)]
        pub const fn dither(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable dither of pwm 0: disable 1: enable."]
        #[inline(always)]
        pub fn set_dither(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Polarity of exc_p signal 0: high active 1: low active."]
        #[inline(always)]
        pub const fn p_pol(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Polarity of exc_p signal 0: high active 1: low active."]
        #[inline(always)]
        pub fn set_p_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Polarity of exc_n signal 0: high active 1: low active."]
        #[inline(always)]
        pub const fn n_pol(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Polarity of exc_n signal 0: high active 1: low active."]
        #[inline(always)]
        pub fn set_n_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for PwmScaling {
        #[inline(always)]
        fn default() -> PwmScaling {
            PwmScaling(0)
        }
    }
    #[doc = "rdc control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RdcCtl(pub u32);
    impl RdcCtl {
        #[doc = "Enable rdc excite signal 0: rdc disable 1: rdc enable."]
        #[inline(always)]
        pub const fn exc_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable rdc excite signal 0: rdc disable 1: rdc enable."]
        #[inline(always)]
        pub fn set_exc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write 1 start excite signal, always read 0 0: no effect 1: start excite signal."]
        #[inline(always)]
        pub const fn exc_start(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 start excite signal, always read 0 0: no effect 1: start excite signal."]
        #[inline(always)]
        pub fn set_exc_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable rdc accumulate 0: rdc disable 1: rdc enable."]
        #[inline(always)]
        pub const fn acc_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable rdc accumulate 0: rdc disable 1: rdc enable."]
        #[inline(always)]
        pub fn set_acc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Select reference point of rectify signal 0: 0 phase of internal exciting signal 1: 90 phase of internal exciting signal 2: 180 phase of internal exciting signal 3: 270 phase of internal exciting signal 4: use value on external pin 5: use invert value on external pin."]
        #[inline(always)]
        pub const fn rectify_sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Select reference point of rectify signal 0: 0 phase of internal exciting signal 1: 90 phase of internal exciting signal 2: 180 phase of internal exciting signal 3: 270 phase of internal exciting signal 4: use value on external pin 5: use invert value on external pin."]
        #[inline(always)]
        pub fn set_rectify_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Accumulate time, support on the fly change 0：1 cycle 1：2 cycles … 255: 256 cycles."]
        #[inline(always)]
        pub const fn acc_len(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[doc = "Accumulate time, support on the fly change 0：1 cycle 1：2 cycles … 255: 256 cycles."]
        #[inline(always)]
        pub fn set_acc_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
        #[doc = "Time stamp selection for accumulation 0: end of accumulation 1: start of accumulation 2: center of accumulation."]
        #[inline(always)]
        pub const fn ts_sel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Time stamp selection for accumulation 0: end of accumulation 1: start of accumulation 2: center of accumulation."]
        #[inline(always)]
        pub fn set_ts_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for RdcCtl {
        #[inline(always)]
        fn default() -> RdcCtl {
            RdcCtl(0)
        }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and rising edge of i_channel data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RiseDelayI(pub u32);
    impl RiseDelayI {
        #[doc = "Delay value on rising edge of i_channel data 0: 1 cycle 1: 2 cycles …."]
        #[inline(always)]
        pub const fn rise_delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Delay value on rising edge of i_channel data 0: 1 cycle 1: 2 cycles …."]
        #[inline(always)]
        pub fn set_rise_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RiseDelayI {
        #[inline(always)]
        fn default() -> RiseDelayI {
            RiseDelayI(0)
        }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and rising edge of q_channel data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RiseDelayQ(pub u32);
    impl RiseDelayQ {
        #[doc = "Delay value on rising edge of q_channel data 0: 1 cycle 1: 2 cycles …."]
        #[inline(always)]
        pub const fn rise_delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Delay value on rising edge of q_channel data 0: 1 cycle 1: 2 cycles …."]
        #[inline(always)]
        pub fn set_rise_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RiseDelayQ {
        #[inline(always)]
        fn default() -> RiseDelayQ {
            RiseDelayQ(0)
        }
    }
    #[doc = "sample value on falling edge of rectify signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SampleFallI(pub u32);
    impl SampleFallI {
        #[doc = "sample value on falling edge of rectify signal."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "sample value on falling edge of rectify signal."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for SampleFallI {
        #[inline(always)]
        fn default() -> SampleFallI {
            SampleFallI(0)
        }
    }
    #[doc = "sample value on falling edge of rectify signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SampleFallQ(pub u32);
    impl SampleFallQ {
        #[doc = "sample value on falling edge of rectify signal."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "sample value on falling edge of rectify signal."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for SampleFallQ {
        #[inline(always)]
        fn default() -> SampleFallQ {
            SampleFallQ(0)
        }
    }
    #[doc = "sample value on rising edge of rectify signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SampleRiseI(pub u32);
    impl SampleRiseI {
        #[doc = "sample value on rising edge of rectify signal."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "sample value on rising edge of rectify signal."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for SampleRiseI {
        #[inline(always)]
        fn default() -> SampleRiseI {
            SampleRiseI(0)
        }
    }
    #[doc = "sample value on rising edge of rectify signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SampleRiseQ(pub u32);
    impl SampleRiseQ {
        #[doc = "sample value on rising edge of rectify signal."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "sample value on rising edge of rectify signal."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for SampleRiseQ {
        #[inline(always)]
        fn default() -> SampleRiseQ {
            SampleRiseQ(0)
        }
    }
    #[doc = "sample counter of opposite sign with rectify signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SignCntI(pub u32);
    impl SignCntI {
        #[doc = "Negative sample counter during positive rectify signal."]
        #[inline(always)]
        pub const fn cnt_pos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Negative sample counter during positive rectify signal."]
        #[inline(always)]
        pub fn set_cnt_pos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Positive sample counter during negative rectify signal."]
        #[inline(always)]
        pub const fn cnt_neg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Positive sample counter during negative rectify signal."]
        #[inline(always)]
        pub fn set_cnt_neg(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SignCntI {
        #[inline(always)]
        fn default() -> SignCntI {
            SignCntI(0)
        }
    }
    #[doc = "sample counter of opposite sign with rectify signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SignCntQ(pub u32);
    impl SignCntQ {
        #[doc = "Negative sample counter during positive rectify signal."]
        #[inline(always)]
        pub const fn cnt_pos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Negative sample counter during positive rectify signal."]
        #[inline(always)]
        pub fn set_cnt_pos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Positive sample counter during negative rectify signal."]
        #[inline(always)]
        pub const fn cnt_neg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Positive sample counter during negative rectify signal."]
        #[inline(always)]
        pub fn set_cnt_neg(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SignCntQ {
        #[inline(always)]
        fn default() -> SignCntQ {
            SignCntQ(0)
        }
    }
    #[doc = "delay setting in clock cycle for synchronous signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncDelayI(pub u32);
    impl SyncDelayI {
        #[doc = "Delay in clock cycle for synchronous signal, the value shoud less than half of exc_period.exc_period. 0: invalid value 1: 1 cycles 2: 2 cycles ..."]
        #[inline(always)]
        pub const fn delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Delay in clock cycle for synchronous signal, the value shoud less than half of exc_period.exc_period. 0: invalid value 1: 1 cycles 2: 2 cycles ..."]
        #[inline(always)]
        pub fn set_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SyncDelayI {
        #[inline(always)]
        fn default() -> SyncDelayI {
            SyncDelayI(0)
        }
    }
    #[doc = "delay setting in clock cycle for synchronous signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncDelayQ(pub u32);
    impl SyncDelayQ {
        #[doc = "Delay in clock cycle for synchronous signal, the value shoud less than half of exc_period.exc_period. 0: invalid value 1: 1 cycles 2: 2 cycles ..."]
        #[inline(always)]
        pub const fn delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Delay in clock cycle for synchronous signal, the value shoud less than half of exc_period.exc_period. 0: invalid value 1: 1 cycles 2: 2 cycles ..."]
        #[inline(always)]
        pub fn set_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SyncDelayQ {
        #[inline(always)]
        fn default() -> SyncDelayQ {
            SyncDelayQ(0)
        }
    }
    #[doc = "synchronize output signal control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncOutCtrl(pub u32);
    impl SyncOutCtrl {
        #[doc = "Select output synchornize signal 0: 0 phase of internal exciting signal 1: 90 phase of internal exciting signal 2: 180 phase of internal exciting signal 3: 270 phase of internal exciting signal."]
        #[inline(always)]
        pub const fn sync_out_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Select output synchornize signal 0: 0 phase of internal exciting signal 1: 90 phase of internal exciting signal 2: 180 phase of internal exciting signal 3: 270 phase of internal exciting signal."]
        #[inline(always)]
        pub fn set_sync_out_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Enable trigger out from the max point of exciting signal 1: enable 0: disable."]
        #[inline(always)]
        pub const fn max2trig_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger out from the max point of exciting signal 1: enable 0: disable."]
        #[inline(always)]
        pub fn set_max2trig_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable trigger out from the min point of exciting signal 1: enable 0: disable."]
        #[inline(always)]
        pub const fn min2trig_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger out from the min point of exciting signal 1: enable 0: disable."]
        #[inline(always)]
        pub fn set_min2trig_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Delay bettween the delyed trigger and the first pwm pulse in clock cycle 1: 1 cycle 2: 2 cycle …."]
        #[inline(always)]
        pub const fn pwm_out_dly(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Delay bettween the delyed trigger and the first pwm pulse in clock cycle 1: 1 cycle 2: 2 cycle …."]
        #[inline(always)]
        pub fn set_pwm_out_dly(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SyncOutCtrl {
        #[inline(always)]
        fn default() -> SyncOutCtrl {
            SyncOutCtrl(0)
        }
    }
    #[doc = "the offset setting for edge detection of the i_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ThrsI(pub u32);
    impl ThrsI {
        #[doc = "The offset setting for edge detection of the i_channel, signed number … 2: the offset is 0x800000+2 1: the offset is 0x800000+1 0: the offset is 0x800000 -1: the offset is 0x800000-1 -2: the offset is 0x800000-2 …."]
        #[inline(always)]
        pub const fn thrs(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "The offset setting for edge detection of the i_channel, signed number … 2: the offset is 0x800000+2 1: the offset is 0x800000+1 0: the offset is 0x800000 -1: the offset is 0x800000-1 -2: the offset is 0x800000-2 …."]
        #[inline(always)]
        pub fn set_thrs(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for ThrsI {
        #[inline(always)]
        fn default() -> ThrsI {
            ThrsI(0)
        }
    }
    #[doc = "the offset setting for edge detection of the q_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ThrsQ(pub u32);
    impl ThrsQ {
        #[doc = "The offset setting for edge detection of the q_channel, signed number … 2: the offset is 0x800000+2 1: the offset is 0x800000+1 0: the offset is 0x800000 -1: the offset is 0x800000-1 -2: the offset is 0x800000-2 …."]
        #[inline(always)]
        pub const fn thrs(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "The offset setting for edge detection of the q_channel, signed number … 2: the offset is 0x800000+2 1: the offset is 0x800000+1 0: the offset is 0x800000 -1: the offset is 0x800000-1 -2: the offset is 0x800000-2 …."]
        #[inline(always)]
        pub fn set_thrs(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for ThrsQ {
        #[inline(always)]
        fn default() -> ThrsQ {
            ThrsQ(0)
        }
    }
    #[doc = "Configuration for trigger out 0 in clock cycle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrigOut0Cfg(pub u32);
    impl TrigOut0Cfg {
        #[doc = "Lead time for trigger out0 from center of low level , this is a signed value … 2: 2 cycle befor center of low level 1: 1 cycle before center of low level 0: center of low level -1: 1cycle after center of low level -2: 2cycle after center of low level."]
        #[inline(always)]
        pub const fn lead_tim(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Lead time for trigger out0 from center of low level , this is a signed value … 2: 2 cycle befor center of low level 1: 1 cycle before center of low level 0: center of low level -1: 1cycle after center of low level -2: 2cycle after center of low level."]
        #[inline(always)]
        pub fn set_lead_tim(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Enable trigger out0 0: disable 1: enable."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger out0 0: disable 1: enable."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for TrigOut0Cfg {
        #[inline(always)]
        fn default() -> TrigOut0Cfg {
            TrigOut0Cfg(0)
        }
    }
    #[doc = "Configuration for trigger out 1 in clock cycle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrigOut1Cfg(pub u32);
    impl TrigOut1Cfg {
        #[doc = "Lead time for trigger out0 from center of hight level , this is a signed value … 2: 2 cycle befor center of hight level 1: 1 cycle before center of hight level 0: center of hight level -1: 1cycle after center of hight level -2: 2cycle after center of hight level."]
        #[inline(always)]
        pub const fn lead_tim(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Lead time for trigger out0 from center of hight level , this is a signed value … 2: 2 cycle befor center of hight level 1: 1 cycle before center of hight level 0: center of hight level -1: 1cycle after center of hight level -2: 2cycle after center of hight level."]
        #[inline(always)]
        pub fn set_lead_tim(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Enable trigger out1 0: disable 1: enable."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger out1 0: disable 1: enable."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for TrigOut1Cfg {
        #[inline(always)]
        fn default() -> TrigOut1Cfg {
            TrigOut1Cfg(0)
        }
    }
}
