#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RDC0."]
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "accumulate result of i_channel."]
    #[inline(always)]
    pub const fn acc_i(self) -> crate::common::Reg<regs::AccI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "accumulate result of q_channel."]
    #[inline(always)]
    pub const fn acc_q(self) -> crate::common::Reg<regs::AccQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "input channel selection."]
    #[inline(always)]
    pub const fn in_ctl(self) -> crate::common::Reg<regs::InCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "output channel selection."]
    #[inline(always)]
    pub const fn out_ctl(self) -> crate::common::Reg<regs::OutCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "IIR parameter for b branch."]
    #[inline(always)]
    pub const fn iir_b(self) -> crate::common::Reg<regs::IirB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "IIR parameter for a branch."]
    #[inline(always)]
    pub const fn iir_a(self) -> crate::common::Reg<regs::IirA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "excitation signal timming setting."]
    #[inline(always)]
    pub const fn exc_timming(self) -> crate::common::Reg<regs::ExcTimming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "amplitude scaling for excitation."]
    #[inline(always)]
    pub const fn exc_scaling(self) -> crate::common::Reg<regs::ExcScaling, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "amplitude offset setting."]
    #[inline(always)]
    pub const fn exc_offset(self) -> crate::common::Reg<regs::ExcOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "amplitude scaling for excitation."]
    #[inline(always)]
    pub const fn pwm_scaling(self) -> crate::common::Reg<regs::PwmScaling, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "amplitude offset setting."]
    #[inline(always)]
    pub const fn pwm_offset(self) -> crate::common::Reg<regs::PwmOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Configuration for trigger out 0 in clock cycle."]
    #[inline(always)]
    pub const fn trig_out0_cfg(self) -> crate::common::Reg<regs::TrigOut0Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Configuration for trigger out 1 in clock cycle."]
    #[inline(always)]
    pub const fn trig_out1_cfg(self) -> crate::common::Reg<regs::TrigOut1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "pwm dead zone control in clock cycle."]
    #[inline(always)]
    pub const fn pwm_dz(self) -> crate::common::Reg<regs::PwmDz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "synchronize output signal control."]
    #[inline(always)]
    pub const fn sync_out_ctrl(self) -> crate::common::Reg<regs::SyncOutCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "trigger in delay timming in soc bus cycle."]
    #[inline(always)]
    pub const fn exc_sync_dly(self) -> crate::common::Reg<regs::ExcSyncDly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "max min data position of channel."]
    #[inline(always)]
    pub const fn max_min_pos(self) -> crate::common::Reg<regs::MaxMinPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "max value of i_channel."]
    #[inline(always)]
    pub const fn max_i(self) -> crate::common::Reg<regs::MaxI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "min value of i_channel."]
    #[inline(always)]
    pub const fn min_i(self) -> crate::common::Reg<regs::MinI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "max value of q_channel."]
    #[inline(always)]
    pub const fn max_q(self) -> crate::common::Reg<regs::MaxQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "min value of q_channel."]
    #[inline(always)]
    pub const fn min_q(self) -> crate::common::Reg<regs::MinQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "the offset setting for edge detection of the i_channel."]
    #[inline(always)]
    pub const fn thrs_i(self) -> crate::common::Reg<regs::ThrsI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "the offset setting for edge detection of the q_channel."]
    #[inline(always)]
    pub const fn thrs_q(self) -> crate::common::Reg<regs::ThrsQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "the control for edge detection."]
    #[inline(always)]
    pub const fn edg_det_ctl(self) -> crate::common::Reg<regs::EdgDetCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "scaling for accumulation result."]
    #[inline(always)]
    pub const fn acc_scaling(self) -> crate::common::Reg<regs::AccScaling, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "period of excitation."]
    #[inline(always)]
    pub const fn exc_period(self) -> crate::common::Reg<regs::ExcPeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "delay setting in clock cycle for synchronous signal."]
    #[inline(always)]
    pub const fn sync_delay_i(self) -> crate::common::Reg<regs::SyncDelayI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and rising edge of i_channel data."]
    #[inline(always)]
    pub const fn rise_delay_i(self) -> crate::common::Reg<regs::RiseDelayI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and falling edge of i_channel data."]
    #[inline(always)]
    pub const fn fall_delay_i(self) -> crate::common::Reg<regs::FallDelayI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "sample value on rising edge of rectify signal."]
    #[inline(always)]
    pub const fn sample_rise_i(self) -> crate::common::Reg<regs::SampleRiseI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "sample value on falling edge of rectify signal."]
    #[inline(always)]
    pub const fn sample_fall_i(self) -> crate::common::Reg<regs::SampleFallI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "number of accumulation."]
    #[inline(always)]
    pub const fn acc_cnt_i(self) -> crate::common::Reg<regs::AccCntI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "sample counter of opposite sign with rectify signal."]
    #[inline(always)]
    pub const fn sign_cnt_i(self) -> crate::common::Reg<regs::SignCntI, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "delay setting in clock cycle for synchronous signal."]
    #[inline(always)]
    pub const fn sync_delay_q(self) -> crate::common::Reg<regs::SyncDelayQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and rising edge of q_channel data."]
    #[inline(always)]
    pub const fn rise_delay_q(self) -> crate::common::Reg<regs::RiseDelayQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and falling edge of q_channel data."]
    #[inline(always)]
    pub const fn fall_delay_q(self) -> crate::common::Reg<regs::FallDelayQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "sample value on rising edge of rectify signal."]
    #[inline(always)]
    pub const fn sample_rise_q(self) -> crate::common::Reg<regs::SampleRiseQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "sample value on falling edge of rectify signal."]
    #[inline(always)]
    pub const fn sample_fall_q(self) -> crate::common::Reg<regs::SampleFallQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "number of accumulation."]
    #[inline(always)]
    pub const fn acc_cnt_q(self) -> crate::common::Reg<regs::AccCntQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "sample counter of opposite sign with rectify signal."]
    #[inline(always)]
    pub const fn sign_cnt_q(self) -> crate::common::Reg<regs::SignCntQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "the maximum of acc amplitude."]
    #[inline(always)]
    pub const fn amp_max(self) -> crate::common::Reg<regs::AmpMax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "the minimum of acc amplitude."]
    #[inline(always)]
    pub const fn amp_min(self) -> crate::common::Reg<regs::AmpMin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "the interrupt mask control."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "the interrupt state."]
    #[inline(always)]
    pub const fn adc_int_state(self) -> crate::common::Reg<regs::AdcIntState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
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
    #[doc = "number of accumulation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccCntI(pub u32);
    impl AccCntI {
        #[doc = "sample number during the positive of rectify signal 1: 1 2: 2 …."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_pos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "sample number during the positive of rectify signal 1: 1 2: 2 …."]
        #[inline(always)]
        pub const fn set_cnt_pos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "sample number during the negtive of rectify signal 1: 1 2: 2 …."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_neg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "sample number during the negtive of rectify signal 1: 1 2: 2 …."]
        #[inline(always)]
        pub const fn set_cnt_neg(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for AccCntI {
        #[inline(always)]
        fn default() -> AccCntI {
            AccCntI(0)
        }
    }
    impl core::fmt::Debug for AccCntI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AccCntI")
                .field("cnt_pos", &self.cnt_pos())
                .field("cnt_neg", &self.cnt_neg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AccCntI {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AccCntI {{ cnt_pos: {=u16:?}, cnt_neg: {=u16:?} }}",
                self.cnt_pos(),
                self.cnt_neg()
            )
        }
    }
    #[doc = "number of accumulation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccCntQ(pub u32);
    impl AccCntQ {
        #[doc = "sample number during the positive of rectify signal 1: 1 2: 2 …."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_pos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "sample number during the positive of rectify signal 1: 1 2: 2 …."]
        #[inline(always)]
        pub const fn set_cnt_pos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "sample number during the negtive of rectify signal 1: 1 2: 2 …."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_neg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "sample number during the negtive of rectify signal 1: 1 2: 2 …."]
        #[inline(always)]
        pub const fn set_cnt_neg(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for AccCntQ {
        #[inline(always)]
        fn default() -> AccCntQ {
            AccCntQ(0)
        }
    }
    impl core::fmt::Debug for AccCntQ {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AccCntQ")
                .field("cnt_pos", &self.cnt_pos())
                .field("cnt_neg", &self.cnt_neg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AccCntQ {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AccCntQ {{ cnt_pos: {=u16:?}, cnt_neg: {=u16:?} }}",
                self.cnt_pos(),
                self.cnt_neg()
            )
        }
    }
    #[doc = "accumulate result of i_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccI(pub u32);
    impl AccI {
        #[doc = "accumulate result of i_channel, this is a signed number."]
        #[must_use]
        #[inline(always)]
        pub const fn acc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "accumulate result of i_channel, this is a signed number."]
        #[inline(always)]
        pub const fn set_acc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AccI {
        #[inline(always)]
        fn default() -> AccI {
            AccI(0)
        }
    }
    impl core::fmt::Debug for AccI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AccI").field("acc", &self.acc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AccI {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AccI {{ acc: {=u32:?} }}", self.acc())
        }
    }
    #[doc = "accumulate result of q_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccQ(pub u32);
    impl AccQ {
        #[doc = "accumulate result of q_channel, this is a signed number."]
        #[must_use]
        #[inline(always)]
        pub const fn acc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "accumulate result of q_channel, this is a signed number."]
        #[inline(always)]
        pub const fn set_acc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AccQ {
        #[inline(always)]
        fn default() -> AccQ {
            AccQ(0)
        }
    }
    impl core::fmt::Debug for AccQ {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AccQ").field("acc", &self.acc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AccQ {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AccQ {{ acc: {=u32:?} }}", self.acc())
        }
    }
    #[doc = "scaling for accumulation result."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccScaling(pub u32);
    impl AccScaling {
        #[doc = "Accumulation value shift control, this is a sign number. 0: {acc\\[39\\],acc\\[38:8\\]} 1: {acc\\[39\\],acc\\[37:7\\]} 2: {acc\\[39\\],acc\\[36:6\\]} … 7: {acc\\[39\\],acc\\[31:1\\]} 8: {acc\\[39\\],acc\\[30:0\\]} 9: acc/2^9 10: acc/2^10 … 15:acc/2^15."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_shift(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Accumulation value shift control, this is a sign number. 0: {acc\\[39\\],acc\\[38:8\\]} 1: {acc\\[39\\],acc\\[37:7\\]} 2: {acc\\[39\\],acc\\[36:6\\]} … 7: {acc\\[39\\],acc\\[31:1\\]} 8: {acc\\[39\\],acc\\[30:0\\]} 9: acc/2^9 10: acc/2^10 … 15:acc/2^15."]
        #[inline(always)]
        pub const fn set_acc_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Toxic accumulation data be removed control 1: enable 0: disable."]
        #[must_use]
        #[inline(always)]
        pub const fn toxic_lk(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Toxic accumulation data be removed control 1: enable 0: disable."]
        #[inline(always)]
        pub const fn set_toxic_lk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for AccScaling {
        #[inline(always)]
        fn default() -> AccScaling {
            AccScaling(0)
        }
    }
    impl core::fmt::Debug for AccScaling {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AccScaling")
                .field("acc_shift", &self.acc_shift())
                .field("toxic_lk", &self.toxic_lk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AccScaling {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AccScaling {{ acc_shift: {=u8:?}, toxic_lk: {=bool:?} }}",
                self.acc_shift(),
                self.toxic_lk()
            )
        }
    }
    #[doc = "the interrupt state."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcIntState(pub u32);
    impl AdcIntState {
        #[doc = "accumulate ample underflow interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_amp_ovl_sta(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "accumulate ample underflow interrupt status."]
        #[inline(always)]
        pub const fn set_acc_amp_ovl_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "accumulate ample overflow interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_amp_ovh_sta(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "accumulate ample overflow interrupt status."]
        #[inline(always)]
        pub const fn set_acc_amp_ovh_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "q_channel accumulate underflow interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_vld_q_ovl_sta(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel accumulate underflow interrupt status."]
        #[inline(always)]
        pub const fn set_acc_vld_q_ovl_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "i_channel accumulate underflow interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_vld_i_ovl_sta(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel accumulate underflow interrupt status."]
        #[inline(always)]
        pub const fn set_acc_vld_i_ovl_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "q_channel accumulate overflow interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_vld_q_ovh_sta(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel accumulate overflow interrupt status."]
        #[inline(always)]
        pub const fn set_acc_vld_q_ovh_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "i_channel accumulate overflow interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_vld_i_ovh_sta(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel accumulate overflow interrupt status."]
        #[inline(always)]
        pub const fn set_acc_vld_i_ovh_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "q_channel falling edge interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn sample_falling_q_sta(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel falling edge interrupt status."]
        #[inline(always)]
        pub const fn set_sample_falling_q_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "q_channel rising edge interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn sample_rising_q_sta(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel rising edge interrupt status."]
        #[inline(always)]
        pub const fn set_sample_rising_q_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "i_channel falling edge interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn sample_falling_i_sta(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel falling edge interrupt status."]
        #[inline(always)]
        pub const fn set_sample_falling_i_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "i_channel rising edge interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn sample_rising_i_sta(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel rising edge interrupt status."]
        #[inline(always)]
        pub const fn set_sample_rising_i_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "q_channel delayed rectify signal falling edge interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn falling_delay_q_sta(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel delayed rectify signal falling edge interrupt status."]
        #[inline(always)]
        pub const fn set_falling_delay_q_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "q_channel delayed rectify signal rising edge interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn rising_delay_q_sta(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel delayed rectify signal rising edge interrupt status."]
        #[inline(always)]
        pub const fn set_rising_delay_q_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "i_channel delayed rectify signal falling edge interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn falling_delay_i_sta(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel delayed rectify signal falling edge interrupt status."]
        #[inline(always)]
        pub const fn set_falling_delay_i_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "i_channel delayed rectify signal rising edge interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn rising_delay_i_sta(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel delayed rectify signal rising edge interrupt status."]
        #[inline(always)]
        pub const fn set_rising_delay_i_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "q_channel accumulate valid interrupt status for i_channel."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_vld_q_sta(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel accumulate valid interrupt status for i_channel."]
        #[inline(always)]
        pub const fn set_acc_vld_q_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "i_channel accumulate valid interrupt status for i_channel."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_vld_i_sta(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel accumulate valid interrupt status for i_channel."]
        #[inline(always)]
        pub const fn set_acc_vld_i_sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for AdcIntState {
        #[inline(always)]
        fn default() -> AdcIntState {
            AdcIntState(0)
        }
    }
    impl core::fmt::Debug for AdcIntState {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcIntState")
                .field("acc_amp_ovl_sta", &self.acc_amp_ovl_sta())
                .field("acc_amp_ovh_sta", &self.acc_amp_ovh_sta())
                .field("acc_vld_q_ovl_sta", &self.acc_vld_q_ovl_sta())
                .field("acc_vld_i_ovl_sta", &self.acc_vld_i_ovl_sta())
                .field("acc_vld_q_ovh_sta", &self.acc_vld_q_ovh_sta())
                .field("acc_vld_i_ovh_sta", &self.acc_vld_i_ovh_sta())
                .field("sample_falling_q_sta", &self.sample_falling_q_sta())
                .field("sample_rising_q_sta", &self.sample_rising_q_sta())
                .field("sample_falling_i_sta", &self.sample_falling_i_sta())
                .field("sample_rising_i_sta", &self.sample_rising_i_sta())
                .field("falling_delay_q_sta", &self.falling_delay_q_sta())
                .field("rising_delay_q_sta", &self.rising_delay_q_sta())
                .field("falling_delay_i_sta", &self.falling_delay_i_sta())
                .field("rising_delay_i_sta", &self.rising_delay_i_sta())
                .field("acc_vld_q_sta", &self.acc_vld_q_sta())
                .field("acc_vld_i_sta", &self.acc_vld_i_sta())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AdcIntState {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AdcIntState {{ acc_amp_ovl_sta: {=bool:?}, acc_amp_ovh_sta: {=bool:?}, acc_vld_q_ovl_sta: {=bool:?}, acc_vld_i_ovl_sta: {=bool:?}, acc_vld_q_ovh_sta: {=bool:?}, acc_vld_i_ovh_sta: {=bool:?}, sample_falling_q_sta: {=bool:?}, sample_rising_q_sta: {=bool:?}, sample_falling_i_sta: {=bool:?}, sample_rising_i_sta: {=bool:?}, falling_delay_q_sta: {=bool:?}, rising_delay_q_sta: {=bool:?}, falling_delay_i_sta: {=bool:?}, rising_delay_i_sta: {=bool:?}, acc_vld_q_sta: {=bool:?}, acc_vld_i_sta: {=bool:?} }}" , self . acc_amp_ovl_sta () , self . acc_amp_ovh_sta () , self . acc_vld_q_ovl_sta () , self . acc_vld_i_ovl_sta () , self . acc_vld_q_ovh_sta () , self . acc_vld_i_ovh_sta () , self . sample_falling_q_sta () , self . sample_rising_q_sta () , self . sample_falling_i_sta () , self . sample_rising_i_sta () , self . falling_delay_q_sta () , self . rising_delay_q_sta () , self . falling_delay_i_sta () , self . rising_delay_i_sta () , self . acc_vld_q_sta () , self . acc_vld_i_sta ())
        }
    }
    #[doc = "the maximum of acc amplitude."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AmpMax(pub u32);
    impl AmpMax {
        #[doc = "the maximum of acc amplitude."]
        #[must_use]
        #[inline(always)]
        pub const fn max(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the maximum of acc amplitude."]
        #[inline(always)]
        pub const fn set_max(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AmpMax {
        #[inline(always)]
        fn default() -> AmpMax {
            AmpMax(0)
        }
    }
    impl core::fmt::Debug for AmpMax {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AmpMax").field("max", &self.max()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AmpMax {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AmpMax {{ max: {=u32:?} }}", self.max())
        }
    }
    #[doc = "the minimum of acc amplitude."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AmpMin(pub u32);
    impl AmpMin {
        #[doc = "the minimum of acc amplitude."]
        #[must_use]
        #[inline(always)]
        pub const fn min(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the minimum of acc amplitude."]
        #[inline(always)]
        pub const fn set_min(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AmpMin {
        #[inline(always)]
        fn default() -> AmpMin {
            AmpMin(0)
        }
    }
    impl core::fmt::Debug for AmpMin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AmpMin").field("min", &self.min()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AmpMin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AmpMin {{ min: {=u32:?} }}", self.min())
        }
    }
    #[doc = "the control for edge detection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EdgDetCtl(pub u32);
    impl EdgDetCtl {
        #[doc = "The continuous positive or negative number for edge detection 0: 1 1: 2 … 7: 8."]
        #[must_use]
        #[inline(always)]
        pub const fn filter(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "The continuous positive or negative number for edge detection 0: 1 1: 2 … 7: 8."]
        #[inline(always)]
        pub const fn set_filter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "The minimum edge distance in sample 0:1 sample 1:2 sample 2:3 samples … 63:64 samples."]
        #[must_use]
        #[inline(always)]
        pub const fn hold(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x3f;
            val as u8
        }
        #[doc = "The minimum edge distance in sample 0:1 sample 1:2 sample 2:3 samples … 63:64 samples."]
        #[inline(always)]
        pub const fn set_hold(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
        }
    }
    impl Default for EdgDetCtl {
        #[inline(always)]
        fn default() -> EdgDetCtl {
            EdgDetCtl(0)
        }
    }
    impl core::fmt::Debug for EdgDetCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EdgDetCtl")
                .field("filter", &self.filter())
                .field("hold", &self.hold())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EdgDetCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EdgDetCtl {{ filter: {=u8:?}, hold: {=u8:?} }}",
                self.filter(),
                self.hold()
            )
        }
    }
    #[doc = "amplitude offset setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExcOffset(pub u32);
    impl ExcOffset {
        #[doc = "Offset for excitation."]
        #[must_use]
        #[inline(always)]
        pub const fn amp_offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Offset for excitation."]
        #[inline(always)]
        pub const fn set_amp_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for ExcOffset {
        #[inline(always)]
        fn default() -> ExcOffset {
            ExcOffset(0)
        }
    }
    impl core::fmt::Debug for ExcOffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExcOffset")
                .field("amp_offset", &self.amp_offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ExcOffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ExcOffset {{ amp_offset: {=u32:?} }}", self.amp_offset())
        }
    }
    #[doc = "period of excitation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExcPeriod(pub u32);
    impl ExcPeriod {
        #[doc = "The num in clock cycle for period of excitation 0: invalid value 1:1 cycle 2:2 cycles …."]
        #[must_use]
        #[inline(always)]
        pub const fn exc_period(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The num in clock cycle for period of excitation 0: invalid value 1:1 cycle 2:2 cycles …."]
        #[inline(always)]
        pub const fn set_exc_period(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ExcPeriod {
        #[inline(always)]
        fn default() -> ExcPeriod {
            ExcPeriod(0)
        }
    }
    impl core::fmt::Debug for ExcPeriod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExcPeriod")
                .field("exc_period", &self.exc_period())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ExcPeriod {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ExcPeriod {{ exc_period: {=u32:?} }}", self.exc_period())
        }
    }
    #[doc = "amplitude scaling for excitation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExcScaling(pub u32);
    impl ExcScaling {
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[must_use]
        #[inline(always)]
        pub const fn amp_man(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[inline(always)]
        pub const fn set_amp_man(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[must_use]
        #[inline(always)]
        pub const fn amp_exp(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[inline(always)]
        pub const fn set_amp_exp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for ExcScaling {
        #[inline(always)]
        fn default() -> ExcScaling {
            ExcScaling(0)
        }
    }
    impl core::fmt::Debug for ExcScaling {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExcScaling")
                .field("amp_man", &self.amp_man())
                .field("amp_exp", &self.amp_exp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ExcScaling {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ExcScaling {{ amp_man: {=u8:?}, amp_exp: {=u8:?} }}",
                self.amp_man(),
                self.amp_exp()
            )
        }
    }
    #[doc = "trigger in delay timming in soc bus cycle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExcSyncDly(pub u32);
    impl ExcSyncDly {
        #[doc = "Trigger in delay timming in bus cycle from rising edge of trigger signal 0: 1 cycle 1: 2 cycle … 0xffffff: 2^24 cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Trigger in delay timming in bus cycle from rising edge of trigger signal 0: 1 cycle 1: 2 cycle … 0xffffff: 2^24 cycle."]
        #[inline(always)]
        pub const fn set_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "Disable hardware trigger input 0: enable 1: disable."]
        #[must_use]
        #[inline(always)]
        pub const fn disable(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Disable hardware trigger input 0: enable 1: disable."]
        #[inline(always)]
        pub const fn set_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for ExcSyncDly {
        #[inline(always)]
        fn default() -> ExcSyncDly {
            ExcSyncDly(0)
        }
    }
    impl core::fmt::Debug for ExcSyncDly {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExcSyncDly")
                .field("delay", &self.delay())
                .field("disable", &self.disable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ExcSyncDly {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ExcSyncDly {{ delay: {=u32:?}, disable: {=bool:?} }}",
                self.delay(),
                self.disable()
            )
        }
    }
    #[doc = "excitation signal timming setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExcTimming(pub u32);
    impl ExcTimming {
        #[doc = "The period for excitation sample in clock cycle， 0: not allowed 1: 1 cycle 2: 2 cycles … 65535 : 65535 cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn smp_rate(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The period for excitation sample in clock cycle， 0: not allowed 1: 1 cycle 2: 2 cycles … 65535 : 65535 cycles."]
        #[inline(always)]
        pub const fn set_smp_rate(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Number of sample every excitation period 0: 4 point 1: 8 point … 8: 1024 point."]
        #[must_use]
        #[inline(always)]
        pub const fn smp_num(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of sample every excitation period 0: 4 point 1: 8 point … 8: 1024 point."]
        #[inline(always)]
        pub const fn set_smp_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Pwm period in samples， 0：1 sample period 1: 2 sample period ... 15: 16 sample period."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm_prd(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Pwm period in samples， 0：1 sample period 1: 2 sample period ... 15: 16 sample period."]
        #[inline(always)]
        pub const fn set_pwm_prd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Swap output of PWM and DAC 0: disable swap 1: swap output."]
        #[must_use]
        #[inline(always)]
        pub const fn swap(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Swap output of PWM and DAC 0: disable swap 1: swap output."]
        #[inline(always)]
        pub const fn set_swap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for ExcTimming {
        #[inline(always)]
        fn default() -> ExcTimming {
            ExcTimming(0)
        }
    }
    impl core::fmt::Debug for ExcTimming {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ExcTimming")
                .field("smp_rate", &self.smp_rate())
                .field("smp_num", &self.smp_num())
                .field("pwm_prd", &self.pwm_prd())
                .field("swap", &self.swap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ExcTimming {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ExcTimming {{ smp_rate: {=u16:?}, smp_num: {=u8:?}, pwm_prd: {=u8:?}, swap: {=bool:?} }}" , self . smp_rate () , self . smp_num () , self . pwm_prd () , self . swap ())
        }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and falling edge of i_channel data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FallDelayI(pub u32);
    impl FallDelayI {
        #[doc = "Delay value on falling edge of i_channel data 0: 1 cycle 1: 2 cycles …."]
        #[must_use]
        #[inline(always)]
        pub const fn fall_delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Delay value on falling edge of i_channel data 0: 1 cycle 1: 2 cycles …."]
        #[inline(always)]
        pub const fn set_fall_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FallDelayI {
        #[inline(always)]
        fn default() -> FallDelayI {
            FallDelayI(0)
        }
    }
    impl core::fmt::Debug for FallDelayI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FallDelayI")
                .field("fall_delay", &self.fall_delay())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FallDelayI {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FallDelayI {{ fall_delay: {=u32:?} }}",
                self.fall_delay()
            )
        }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and falling edge of q_channel data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FallDelayQ(pub u32);
    impl FallDelayQ {
        #[doc = "Delay value on falling edge of q_channel data 0: 1 cycle 1: 2 cycles …."]
        #[must_use]
        #[inline(always)]
        pub const fn fall_delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Delay value on falling edge of q_channel data 0: 1 cycle 1: 2 cycles …."]
        #[inline(always)]
        pub const fn set_fall_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FallDelayQ {
        #[inline(always)]
        fn default() -> FallDelayQ {
            FallDelayQ(0)
        }
    }
    impl core::fmt::Debug for FallDelayQ {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FallDelayQ")
                .field("fall_delay", &self.fall_delay())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FallDelayQ {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FallDelayQ {{ fall_delay: {=u32:?} }}",
                self.fall_delay()
            )
        }
    }
    #[doc = "IIR parameter for a branch."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IirA(pub u32);
    impl IirA {
        #[doc = "IIR parameter a1 for a branch."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_a1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "IIR parameter a1 for a branch."]
        #[inline(always)]
        pub const fn set_iir_a1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "IIR parameter a2 for a branch."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_a2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "IIR parameter a2 for a branch."]
        #[inline(always)]
        pub const fn set_iir_a2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for IirA {
        #[inline(always)]
        fn default() -> IirA {
            IirA(0)
        }
    }
    impl core::fmt::Debug for IirA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IirA")
                .field("iir_a1", &self.iir_a1())
                .field("iir_a2", &self.iir_a2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IirA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IirA {{ iir_a1: {=u16:?}, iir_a2: {=u8:?} }}",
                self.iir_a1(),
                self.iir_a2()
            )
        }
    }
    #[doc = "IIR parameter for b branch."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IirB(pub u32);
    impl IirB {
        #[doc = "IIR parameter for b branch."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_b(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "IIR parameter for b branch."]
        #[inline(always)]
        pub const fn set_iir_b(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "IIR in lowpass mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lowpass(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "IIR in lowpass mode."]
        #[inline(always)]
        pub const fn set_lowpass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for IirB {
        #[inline(always)]
        fn default() -> IirB {
            IirB(0)
        }
    }
    impl core::fmt::Debug for IirB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IirB")
                .field("iir_b", &self.iir_b())
                .field("lowpass", &self.lowpass())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IirB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IirB {{ iir_b: {=u8:?}, lowpass: {=bool:?} }}",
                self.iir_b(),
                self.lowpass()
            )
        }
    }
    #[doc = "input channel selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InCtl(pub u32);
    impl InCtl {
        #[doc = "Input channel selection for i_channel 0: channel 0 selected 1: channel 1 selected … 31: channel 31 selected."]
        #[must_use]
        #[inline(always)]
        pub const fn ch_i_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel selection for i_channel 0: channel 0 selected 1: channel 1 selected … 31: channel 31 selected."]
        #[inline(always)]
        pub const fn set_ch_i_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Input port selection for i_channel, 0:sel port0 1:sel port1."]
        #[must_use]
        #[inline(always)]
        pub const fn port_i_sel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Input port selection for i_channel, 0:sel port0 1:sel port1."]
        #[inline(always)]
        pub const fn set_port_i_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Input channel selection for q_channel 0: channel 0 selected 1: channel 1 selected … 31: channel 31 selected."]
        #[must_use]
        #[inline(always)]
        pub const fn ch_q_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x1f;
            val as u8
        }
        #[doc = "Input channel selection for q_channel 0: channel 0 selected 1: channel 1 selected … 31: channel 31 selected."]
        #[inline(always)]
        pub const fn set_ch_q_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
        }
        #[doc = "Input port selection for q_channel, 0:sel port0 1:sel port1."]
        #[must_use]
        #[inline(always)]
        pub const fn port_q_sel(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Input port selection for q_channel, 0:sel port0 1:sel port1."]
        #[inline(always)]
        pub const fn set_port_q_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for InCtl {
        #[inline(always)]
        fn default() -> InCtl {
            InCtl(0)
        }
    }
    impl core::fmt::Debug for InCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("InCtl")
                .field("ch_i_sel", &self.ch_i_sel())
                .field("port_i_sel", &self.port_i_sel())
                .field("ch_q_sel", &self.ch_q_sel())
                .field("port_q_sel", &self.port_q_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for InCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "InCtl {{ ch_i_sel: {=u8:?}, port_i_sel: {=bool:?}, ch_q_sel: {=u8:?}, port_q_sel: {=bool:?} }}" , self . ch_i_sel () , self . port_i_sel () , self . ch_q_sel () , self . port_q_sel ())
        }
    }
    #[doc = "the interrupt mask control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "accumulate ample underflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_amp_ovl_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "accumulate ample underflow interrupt enable."]
        #[inline(always)]
        pub const fn set_acc_amp_ovl_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "accumulate ample overflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_amp_ovh_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "accumulate ample overflow interrupt enable."]
        #[inline(always)]
        pub const fn set_acc_amp_ovh_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "q_channel accumulate underflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_vld_q_ovl_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel accumulate underflow interrupt enable."]
        #[inline(always)]
        pub const fn set_acc_vld_q_ovl_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "i_channel accumulate underflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_vld_i_ovl_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel accumulate underflow interrupt enable."]
        #[inline(always)]
        pub const fn set_acc_vld_i_ovl_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "q_channel accumulate overflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_vld_q_ovh_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel accumulate overflow interrupt enable."]
        #[inline(always)]
        pub const fn set_acc_vld_q_ovh_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "i_channel accumulate overflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_vld_i_ovh_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel accumulate overflow interrupt enable."]
        #[inline(always)]
        pub const fn set_acc_vld_i_ovh_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "q_channel falling edge interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sample_falling_q_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel falling edge interrupt enable."]
        #[inline(always)]
        pub const fn set_sample_falling_q_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "q_channel rising edge interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sample_rising_q_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel rising edge interrupt enable."]
        #[inline(always)]
        pub const fn set_sample_rising_q_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "i_channel falling edge interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sample_falling_i_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel falling edge interrupt enable."]
        #[inline(always)]
        pub const fn set_sample_falling_i_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "i_channel rising edge interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sample_rising_i_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel rising edge interrupt enable."]
        #[inline(always)]
        pub const fn set_sample_rising_i_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "q_channel delayed rectify signal falling edge interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn falling_delay_q_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel delayed rectify signal falling edge interrupt enable."]
        #[inline(always)]
        pub const fn set_falling_delay_q_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "q_channel delayed rectify signal rising edge interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rising_delay_q_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel delayed rectify signal rising edge interrupt enable."]
        #[inline(always)]
        pub const fn set_rising_delay_q_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "i_channel delayed rectify signal falling edge interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn falling_delay_i_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel delayed rectify signal falling edge interrupt enable."]
        #[inline(always)]
        pub const fn set_falling_delay_i_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "i_channel delayed rectify signal rising edge interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rising_delay_i_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel delayed rectify signal rising edge interrupt enable."]
        #[inline(always)]
        pub const fn set_rising_delay_i_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "q_channel accumulate valid interrupt enable for i_channel."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_vld_q_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "q_channel accumulate valid interrupt enable for i_channel."]
        #[inline(always)]
        pub const fn set_acc_vld_q_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "i_channel accumulate valid interrupt enable for i_channel."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_vld_i_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "i_channel accumulate valid interrupt enable for i_channel."]
        #[inline(always)]
        pub const fn set_acc_vld_i_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "enable interrupt output."]
        #[must_use]
        #[inline(always)]
        pub const fn int_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt output."]
        #[inline(always)]
        pub const fn set_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("acc_amp_ovl_en", &self.acc_amp_ovl_en())
                .field("acc_amp_ovh_en", &self.acc_amp_ovh_en())
                .field("acc_vld_q_ovl_en", &self.acc_vld_q_ovl_en())
                .field("acc_vld_i_ovl_en", &self.acc_vld_i_ovl_en())
                .field("acc_vld_q_ovh_en", &self.acc_vld_q_ovh_en())
                .field("acc_vld_i_ovh_en", &self.acc_vld_i_ovh_en())
                .field("sample_falling_q_en", &self.sample_falling_q_en())
                .field("sample_rising_q_en", &self.sample_rising_q_en())
                .field("sample_falling_i_en", &self.sample_falling_i_en())
                .field("sample_rising_i_en", &self.sample_rising_i_en())
                .field("falling_delay_q_en", &self.falling_delay_q_en())
                .field("rising_delay_q_en", &self.rising_delay_q_en())
                .field("falling_delay_i_en", &self.falling_delay_i_en())
                .field("rising_delay_i_en", &self.rising_delay_i_en())
                .field("acc_vld_q_en", &self.acc_vld_q_en())
                .field("acc_vld_i_en", &self.acc_vld_i_en())
                .field("int_en", &self.int_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntEn {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntEn {{ acc_amp_ovl_en: {=bool:?}, acc_amp_ovh_en: {=bool:?}, acc_vld_q_ovl_en: {=bool:?}, acc_vld_i_ovl_en: {=bool:?}, acc_vld_q_ovh_en: {=bool:?}, acc_vld_i_ovh_en: {=bool:?}, sample_falling_q_en: {=bool:?}, sample_rising_q_en: {=bool:?}, sample_falling_i_en: {=bool:?}, sample_rising_i_en: {=bool:?}, falling_delay_q_en: {=bool:?}, rising_delay_q_en: {=bool:?}, falling_delay_i_en: {=bool:?}, rising_delay_i_en: {=bool:?}, acc_vld_q_en: {=bool:?}, acc_vld_i_en: {=bool:?}, int_en: {=bool:?} }}" , self . acc_amp_ovl_en () , self . acc_amp_ovh_en () , self . acc_vld_q_ovl_en () , self . acc_vld_i_ovl_en () , self . acc_vld_q_ovh_en () , self . acc_vld_i_ovh_en () , self . sample_falling_q_en () , self . sample_rising_q_en () , self . sample_falling_i_en () , self . sample_rising_i_en () , self . falling_delay_q_en () , self . rising_delay_q_en () , self . falling_delay_i_en () , self . rising_delay_i_en () , self . acc_vld_q_en () , self . acc_vld_i_en () , self . int_en ())
        }
    }
    #[doc = "max value of i_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MaxI(pub u32);
    impl MaxI {
        #[doc = "Max value valid, write clear 0: max value is not valid 1: max value is valid."]
        #[must_use]
        #[inline(always)]
        pub const fn valid(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Max value valid, write clear 0: max value is not valid 1: max value is valid."]
        #[inline(always)]
        pub const fn set_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Max value of i_channel, write clear."]
        #[must_use]
        #[inline(always)]
        pub const fn max(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Max value of i_channel, write clear."]
        #[inline(always)]
        pub const fn set_max(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for MaxI {
        #[inline(always)]
        fn default() -> MaxI {
            MaxI(0)
        }
    }
    impl core::fmt::Debug for MaxI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MaxI")
                .field("valid", &self.valid())
                .field("max", &self.max())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MaxI {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MaxI {{ valid: {=bool:?}, max: {=u32:?} }}",
                self.valid(),
                self.max()
            )
        }
    }
    #[doc = "max min data position of channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MaxMinPos(pub u32);
    impl MaxMinPos {
        #[doc = "max min value position 0: max min value at adc input 1: max min value at IIR output."]
        #[must_use]
        #[inline(always)]
        pub const fn max_min_pos(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "max min value position 0: max min value at adc input 1: max min value at IIR output."]
        #[inline(always)]
        pub const fn set_max_min_pos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for MaxMinPos {
        #[inline(always)]
        fn default() -> MaxMinPos {
            MaxMinPos(0)
        }
    }
    impl core::fmt::Debug for MaxMinPos {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MaxMinPos")
                .field("max_min_pos", &self.max_min_pos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MaxMinPos {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MaxMinPos {{ max_min_pos: {=bool:?} }}",
                self.max_min_pos()
            )
        }
    }
    #[doc = "max value of q_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MaxQ(pub u32);
    impl MaxQ {
        #[doc = "Max value valid, write clear 0: max value is not valid 1: max value is valid."]
        #[must_use]
        #[inline(always)]
        pub const fn valid(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Max value valid, write clear 0: max value is not valid 1: max value is valid."]
        #[inline(always)]
        pub const fn set_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Max value of q_channel, write clear."]
        #[must_use]
        #[inline(always)]
        pub const fn max(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Max value of q_channel, write clear."]
        #[inline(always)]
        pub const fn set_max(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for MaxQ {
        #[inline(always)]
        fn default() -> MaxQ {
            MaxQ(0)
        }
    }
    impl core::fmt::Debug for MaxQ {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MaxQ")
                .field("valid", &self.valid())
                .field("max", &self.max())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MaxQ {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MaxQ {{ valid: {=bool:?}, max: {=u32:?} }}",
                self.valid(),
                self.max()
            )
        }
    }
    #[doc = "min value of i_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MinI(pub u32);
    impl MinI {
        #[doc = "Min value valid, write clear 0: min value is not valid 1: min value is valid."]
        #[must_use]
        #[inline(always)]
        pub const fn valid(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Min value valid, write clear 0: min value is not valid 1: min value is valid."]
        #[inline(always)]
        pub const fn set_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Min value of i_channel, write clear."]
        #[must_use]
        #[inline(always)]
        pub const fn min(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Min value of i_channel, write clear."]
        #[inline(always)]
        pub const fn set_min(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for MinI {
        #[inline(always)]
        fn default() -> MinI {
            MinI(0)
        }
    }
    impl core::fmt::Debug for MinI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MinI")
                .field("valid", &self.valid())
                .field("min", &self.min())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MinI {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MinI {{ valid: {=bool:?}, min: {=u32:?} }}",
                self.valid(),
                self.min()
            )
        }
    }
    #[doc = "min value of q_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MinQ(pub u32);
    impl MinQ {
        #[doc = "Min value valid, write clear 0: min value is not valid 1: min value is valid."]
        #[must_use]
        #[inline(always)]
        pub const fn valid(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Min value valid, write clear 0: min value is not valid 1: min value is valid."]
        #[inline(always)]
        pub const fn set_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Min value of q_channel, write clear."]
        #[must_use]
        #[inline(always)]
        pub const fn min(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Min value of q_channel, write clear."]
        #[inline(always)]
        pub const fn set_min(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for MinQ {
        #[inline(always)]
        fn default() -> MinQ {
            MinQ(0)
        }
    }
    impl core::fmt::Debug for MinQ {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MinQ")
                .field("valid", &self.valid())
                .field("min", &self.min())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MinQ {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MinQ {{ valid: {=bool:?}, min: {=u32:?} }}",
                self.valid(),
                self.min()
            )
        }
    }
    #[doc = "output channel selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutCtl(pub u32);
    impl OutCtl {
        #[doc = "Output channel selection for i_channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch_i_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Output channel selection for i_channel."]
        #[inline(always)]
        pub const fn set_ch_i_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Output channel selection for q_channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ch_q_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Output channel selection for q_channel."]
        #[inline(always)]
        pub const fn set_ch_q_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for OutCtl {
        #[inline(always)]
        fn default() -> OutCtl {
            OutCtl(0)
        }
    }
    impl core::fmt::Debug for OutCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OutCtl")
                .field("ch_i_sel", &self.ch_i_sel())
                .field("ch_q_sel", &self.ch_q_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OutCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OutCtl {{ ch_i_sel: {=u8:?}, ch_q_sel: {=u8:?} }}",
                self.ch_i_sel(),
                self.ch_q_sel()
            )
        }
    }
    #[doc = "pwm dead zone control in clock cycle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmDz(pub u32);
    impl PwmDz {
        #[doc = "Exc_p dead zone in clock cycle before swap 0: no dead zone 1: 1 cycle dead zone 2: 2 cycle dead zone …."]
        #[must_use]
        #[inline(always)]
        pub const fn dz_p(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Exc_p dead zone in clock cycle before swap 0: no dead zone 1: 1 cycle dead zone 2: 2 cycle dead zone …."]
        #[inline(always)]
        pub const fn set_dz_p(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Exc_n dead zone in clock cycle before swap 0: no dead zone 1: 1 cycle dead zone 2: 2 cycle dead zone …."]
        #[must_use]
        #[inline(always)]
        pub const fn dz_n(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Exc_n dead zone in clock cycle before swap 0: no dead zone 1: 1 cycle dead zone 2: 2 cycle dead zone …."]
        #[inline(always)]
        pub const fn set_dz_n(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for PwmDz {
        #[inline(always)]
        fn default() -> PwmDz {
            PwmDz(0)
        }
    }
    impl core::fmt::Debug for PwmDz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmDz")
                .field("dz_p", &self.dz_p())
                .field("dz_n", &self.dz_n())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmDz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PwmDz {{ dz_p: {=u8:?}, dz_n: {=u8:?} }}",
                self.dz_p(),
                self.dz_n()
            )
        }
    }
    #[doc = "amplitude offset setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmOffset(pub u32);
    impl PwmOffset {
        #[doc = "Offset for excitation."]
        #[must_use]
        #[inline(always)]
        pub const fn amp_offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Offset for excitation."]
        #[inline(always)]
        pub const fn set_amp_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for PwmOffset {
        #[inline(always)]
        fn default() -> PwmOffset {
            PwmOffset(0)
        }
    }
    impl core::fmt::Debug for PwmOffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmOffset")
                .field("amp_offset", &self.amp_offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmOffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PwmOffset {{ amp_offset: {=u32:?} }}", self.amp_offset())
        }
    }
    #[doc = "amplitude scaling for excitation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmScaling(pub u32);
    impl PwmScaling {
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[must_use]
        #[inline(always)]
        pub const fn amp_man(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[inline(always)]
        pub const fn set_amp_man(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[must_use]
        #[inline(always)]
        pub const fn amp_exp(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Amplitude scaling for excitation, amplitude = \\[table value\\]
x man / 2^exp."]
        #[inline(always)]
        pub const fn set_amp_exp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Enable dither of pwm 0: disable 1: enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dither(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable dither of pwm 0: disable 1: enable."]
        #[inline(always)]
        pub const fn set_dither(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Polarity of exc_p signal 0: high active 1: low active."]
        #[must_use]
        #[inline(always)]
        pub const fn p_pol(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Polarity of exc_p signal 0: high active 1: low active."]
        #[inline(always)]
        pub const fn set_p_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Polarity of exc_n signal 0: high active 1: low active."]
        #[must_use]
        #[inline(always)]
        pub const fn n_pol(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Polarity of exc_n signal 0: high active 1: low active."]
        #[inline(always)]
        pub const fn set_n_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for PwmScaling {
        #[inline(always)]
        fn default() -> PwmScaling {
            PwmScaling(0)
        }
    }
    impl core::fmt::Debug for PwmScaling {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmScaling")
                .field("amp_man", &self.amp_man())
                .field("amp_exp", &self.amp_exp())
                .field("dither", &self.dither())
                .field("p_pol", &self.p_pol())
                .field("n_pol", &self.n_pol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmScaling {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PwmScaling {{ amp_man: {=u8:?}, amp_exp: {=u8:?}, dither: {=bool:?}, p_pol: {=bool:?}, n_pol: {=bool:?} }}" , self . amp_man () , self . amp_exp () , self . dither () , self . p_pol () , self . n_pol ())
        }
    }
    #[doc = "rdc control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RdcCtl(pub u32);
    impl RdcCtl {
        #[doc = "Enable rdc excite signal 0: rdc disable 1: rdc enable."]
        #[must_use]
        #[inline(always)]
        pub const fn exc_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable rdc excite signal 0: rdc disable 1: rdc enable."]
        #[inline(always)]
        pub const fn set_exc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write 1 start excite signal, always read 0 0: no effect 1: start excite signal."]
        #[must_use]
        #[inline(always)]
        pub const fn exc_start(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 start excite signal, always read 0 0: no effect 1: start excite signal."]
        #[inline(always)]
        pub const fn set_exc_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable rdc accumulate 0: rdc disable 1: rdc enable."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable rdc accumulate 0: rdc disable 1: rdc enable."]
        #[inline(always)]
        pub const fn set_acc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "every adc value can be as one accumulate value."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_fast(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "every adc value can be as one accumulate value."]
        #[inline(always)]
        pub const fn set_acc_fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Select reference point of rectify signal 0: 0 phase of internal exciting signal 1: 90 phase of internal exciting signal 2: 180 phase of internal exciting signal 3: 270 phase of internal exciting signal 4: use value on external pin 5: use invert value on external pin."]
        #[must_use]
        #[inline(always)]
        pub const fn rectify_sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Select reference point of rectify signal 0: 0 phase of internal exciting signal 1: 90 phase of internal exciting signal 2: 180 phase of internal exciting signal 3: 270 phase of internal exciting signal 4: use value on external pin 5: use invert value on external pin."]
        #[inline(always)]
        pub const fn set_rectify_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "IIR enable for adc input."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IIR enable for adc input."]
        #[inline(always)]
        pub const fn set_iir_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "rdc output mask."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_out_mask(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "rdc output mask."]
        #[inline(always)]
        pub const fn set_acc_out_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Accumulate time, support on the fly change 0：1 cycle 1：2 cycles … 255: 256 cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_len(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[doc = "Accumulate time, support on the fly change 0：1 cycle 1：2 cycles … 255: 256 cycles."]
        #[inline(always)]
        pub const fn set_acc_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
        #[doc = "Time stamp selection for accumulation 0: end of accumulation 1: start of accumulation 2: center of accumulation."]
        #[must_use]
        #[inline(always)]
        pub const fn ts_sel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Time stamp selection for accumulation 0: end of accumulation 1: start of accumulation 2: center of accumulation."]
        #[inline(always)]
        pub const fn set_ts_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for RdcCtl {
        #[inline(always)]
        fn default() -> RdcCtl {
            RdcCtl(0)
        }
    }
    impl core::fmt::Debug for RdcCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RdcCtl")
                .field("exc_en", &self.exc_en())
                .field("exc_start", &self.exc_start())
                .field("acc_en", &self.acc_en())
                .field("acc_fast", &self.acc_fast())
                .field("rectify_sel", &self.rectify_sel())
                .field("iir_en", &self.iir_en())
                .field("acc_out_mask", &self.acc_out_mask())
                .field("acc_len", &self.acc_len())
                .field("ts_sel", &self.ts_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RdcCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RdcCtl {{ exc_en: {=bool:?}, exc_start: {=bool:?}, acc_en: {=bool:?}, acc_fast: {=bool:?}, rectify_sel: {=u8:?}, iir_en: {=bool:?}, acc_out_mask: {=bool:?}, acc_len: {=u8:?}, ts_sel: {=u8:?} }}" , self . exc_en () , self . exc_start () , self . acc_en () , self . acc_fast () , self . rectify_sel () , self . iir_en () , self . acc_out_mask () , self . acc_len () , self . ts_sel ())
        }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and rising edge of i_channel data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RiseDelayI(pub u32);
    impl RiseDelayI {
        #[doc = "Delay value on rising edge of i_channel data 0: 1 cycle 1: 2 cycles …."]
        #[must_use]
        #[inline(always)]
        pub const fn rise_delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Delay value on rising edge of i_channel data 0: 1 cycle 1: 2 cycles …."]
        #[inline(always)]
        pub const fn set_rise_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RiseDelayI {
        #[inline(always)]
        fn default() -> RiseDelayI {
            RiseDelayI(0)
        }
    }
    impl core::fmt::Debug for RiseDelayI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RiseDelayI")
                .field("rise_delay", &self.rise_delay())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RiseDelayI {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RiseDelayI {{ rise_delay: {=u32:?} }}",
                self.rise_delay()
            )
        }
    }
    #[doc = "delay in clock cycle between excitation synchrnous signal and rising edge of q_channel data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RiseDelayQ(pub u32);
    impl RiseDelayQ {
        #[doc = "Delay value on rising edge of q_channel data 0: 1 cycle 1: 2 cycles …."]
        #[must_use]
        #[inline(always)]
        pub const fn rise_delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Delay value on rising edge of q_channel data 0: 1 cycle 1: 2 cycles …."]
        #[inline(always)]
        pub const fn set_rise_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RiseDelayQ {
        #[inline(always)]
        fn default() -> RiseDelayQ {
            RiseDelayQ(0)
        }
    }
    impl core::fmt::Debug for RiseDelayQ {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RiseDelayQ")
                .field("rise_delay", &self.rise_delay())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RiseDelayQ {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RiseDelayQ {{ rise_delay: {=u32:?} }}",
                self.rise_delay()
            )
        }
    }
    #[doc = "sample value on falling edge of rectify signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SampleFallI(pub u32);
    impl SampleFallI {
        #[doc = "sample value on falling edge of rectify signal."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "sample value on falling edge of rectify signal."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for SampleFallI {
        #[inline(always)]
        fn default() -> SampleFallI {
            SampleFallI(0)
        }
    }
    impl core::fmt::Debug for SampleFallI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SampleFallI")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SampleFallI {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SampleFallI {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "sample value on falling edge of rectify signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SampleFallQ(pub u32);
    impl SampleFallQ {
        #[doc = "sample value on falling edge of rectify signal."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "sample value on falling edge of rectify signal."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for SampleFallQ {
        #[inline(always)]
        fn default() -> SampleFallQ {
            SampleFallQ(0)
        }
    }
    impl core::fmt::Debug for SampleFallQ {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SampleFallQ")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SampleFallQ {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SampleFallQ {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "sample value on rising edge of rectify signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SampleRiseI(pub u32);
    impl SampleRiseI {
        #[doc = "sample value on rising edge of rectify signal."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "sample value on rising edge of rectify signal."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for SampleRiseI {
        #[inline(always)]
        fn default() -> SampleRiseI {
            SampleRiseI(0)
        }
    }
    impl core::fmt::Debug for SampleRiseI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SampleRiseI")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SampleRiseI {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SampleRiseI {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "sample value on rising edge of rectify signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SampleRiseQ(pub u32);
    impl SampleRiseQ {
        #[doc = "sample value on rising edge of rectify signal."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "sample value on rising edge of rectify signal."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for SampleRiseQ {
        #[inline(always)]
        fn default() -> SampleRiseQ {
            SampleRiseQ(0)
        }
    }
    impl core::fmt::Debug for SampleRiseQ {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SampleRiseQ")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SampleRiseQ {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SampleRiseQ {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "sample counter of opposite sign with rectify signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SignCntI(pub u32);
    impl SignCntI {
        #[doc = "Negative sample counter during positive rectify signal."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_pos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Negative sample counter during positive rectify signal."]
        #[inline(always)]
        pub const fn set_cnt_pos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Positive sample counter during negative rectify signal."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_neg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Positive sample counter during negative rectify signal."]
        #[inline(always)]
        pub const fn set_cnt_neg(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SignCntI {
        #[inline(always)]
        fn default() -> SignCntI {
            SignCntI(0)
        }
    }
    impl core::fmt::Debug for SignCntI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SignCntI")
                .field("cnt_pos", &self.cnt_pos())
                .field("cnt_neg", &self.cnt_neg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SignCntI {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SignCntI {{ cnt_pos: {=u16:?}, cnt_neg: {=u16:?} }}",
                self.cnt_pos(),
                self.cnt_neg()
            )
        }
    }
    #[doc = "sample counter of opposite sign with rectify signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SignCntQ(pub u32);
    impl SignCntQ {
        #[doc = "Negative sample counter during positive rectify signal."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_pos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Negative sample counter during positive rectify signal."]
        #[inline(always)]
        pub const fn set_cnt_pos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Positive sample counter during negative rectify signal."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_neg(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Positive sample counter during negative rectify signal."]
        #[inline(always)]
        pub const fn set_cnt_neg(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SignCntQ {
        #[inline(always)]
        fn default() -> SignCntQ {
            SignCntQ(0)
        }
    }
    impl core::fmt::Debug for SignCntQ {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SignCntQ")
                .field("cnt_pos", &self.cnt_pos())
                .field("cnt_neg", &self.cnt_neg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SignCntQ {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SignCntQ {{ cnt_pos: {=u16:?}, cnt_neg: {=u16:?} }}",
                self.cnt_pos(),
                self.cnt_neg()
            )
        }
    }
    #[doc = "delay setting in clock cycle for synchronous signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncDelayI(pub u32);
    impl SyncDelayI {
        #[doc = "Delay in clock cycle for synchronous signal, the value shoud less than half of exc_period.exc_period. 0: invalid value 1: 1 cycles 2: 2 cycles ..."]
        #[must_use]
        #[inline(always)]
        pub const fn delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Delay in clock cycle for synchronous signal, the value shoud less than half of exc_period.exc_period. 0: invalid value 1: 1 cycles 2: 2 cycles ..."]
        #[inline(always)]
        pub const fn set_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SyncDelayI {
        #[inline(always)]
        fn default() -> SyncDelayI {
            SyncDelayI(0)
        }
    }
    impl core::fmt::Debug for SyncDelayI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncDelayI")
                .field("delay", &self.delay())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncDelayI {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SyncDelayI {{ delay: {=u32:?} }}", self.delay())
        }
    }
    #[doc = "delay setting in clock cycle for synchronous signal."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncDelayQ(pub u32);
    impl SyncDelayQ {
        #[doc = "Delay in clock cycle for synchronous signal, the value shoud less than half of exc_period.exc_period. 0: invalid value 1: 1 cycles 2: 2 cycles ..."]
        #[must_use]
        #[inline(always)]
        pub const fn delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Delay in clock cycle for synchronous signal, the value shoud less than half of exc_period.exc_period. 0: invalid value 1: 1 cycles 2: 2 cycles ..."]
        #[inline(always)]
        pub const fn set_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SyncDelayQ {
        #[inline(always)]
        fn default() -> SyncDelayQ {
            SyncDelayQ(0)
        }
    }
    impl core::fmt::Debug for SyncDelayQ {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncDelayQ")
                .field("delay", &self.delay())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncDelayQ {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SyncDelayQ {{ delay: {=u32:?} }}", self.delay())
        }
    }
    #[doc = "synchronize output signal control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncOutCtrl(pub u32);
    impl SyncOutCtrl {
        #[doc = "Select output synchornize signal 0: 0 phase of internal exciting signal 1: 90 phase of internal exciting signal 2: 180 phase of internal exciting signal 3: 270 phase of internal exciting signal."]
        #[must_use]
        #[inline(always)]
        pub const fn sync_out_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Select output synchornize signal 0: 0 phase of internal exciting signal 1: 90 phase of internal exciting signal 2: 180 phase of internal exciting signal 3: 270 phase of internal exciting signal."]
        #[inline(always)]
        pub const fn set_sync_out_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Enable trigger out from the max point of exciting signal 1: enable 0: disable."]
        #[must_use]
        #[inline(always)]
        pub const fn max2trig_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger out from the max point of exciting signal 1: enable 0: disable."]
        #[inline(always)]
        pub const fn set_max2trig_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable trigger out from the min point of exciting signal 1: enable 0: disable."]
        #[must_use]
        #[inline(always)]
        pub const fn min2trig_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger out from the min point of exciting signal 1: enable 0: disable."]
        #[inline(always)]
        pub const fn set_min2trig_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Delay bettween the delyed trigger and the first pwm pulse in clock cycle 1: 1 cycle 2: 2 cycle …."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm_out_dly(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Delay bettween the delyed trigger and the first pwm pulse in clock cycle 1: 1 cycle 2: 2 cycle …."]
        #[inline(always)]
        pub const fn set_pwm_out_dly(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SyncOutCtrl {
        #[inline(always)]
        fn default() -> SyncOutCtrl {
            SyncOutCtrl(0)
        }
    }
    impl core::fmt::Debug for SyncOutCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncOutCtrl")
                .field("sync_out_sel", &self.sync_out_sel())
                .field("max2trig_en", &self.max2trig_en())
                .field("min2trig_en", &self.min2trig_en())
                .field("pwm_out_dly", &self.pwm_out_dly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncOutCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SyncOutCtrl {{ sync_out_sel: {=u8:?}, max2trig_en: {=bool:?}, min2trig_en: {=bool:?}, pwm_out_dly: {=u16:?} }}" , self . sync_out_sel () , self . max2trig_en () , self . min2trig_en () , self . pwm_out_dly ())
        }
    }
    #[doc = "the offset setting for edge detection of the i_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ThrsI(pub u32);
    impl ThrsI {
        #[doc = "enable thrs data for accumulate."]
        #[must_use]
        #[inline(always)]
        pub const fn thrs4acc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable thrs data for accumulate."]
        #[inline(always)]
        pub const fn set_thrs4acc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "The offset setting for edge detection of the i_channel, signed number … 2: the offset is 0x800000+2 1: the offset is 0x800000+1 0: the offset is 0x800000 -1: the offset is 0x800000-1 -2: the offset is 0x800000-2 …."]
        #[must_use]
        #[inline(always)]
        pub const fn thrs(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "The offset setting for edge detection of the i_channel, signed number … 2: the offset is 0x800000+2 1: the offset is 0x800000+1 0: the offset is 0x800000 -1: the offset is 0x800000-1 -2: the offset is 0x800000-2 …."]
        #[inline(always)]
        pub const fn set_thrs(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for ThrsI {
        #[inline(always)]
        fn default() -> ThrsI {
            ThrsI(0)
        }
    }
    impl core::fmt::Debug for ThrsI {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ThrsI")
                .field("thrs4acc", &self.thrs4acc())
                .field("thrs", &self.thrs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ThrsI {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ThrsI {{ thrs4acc: {=bool:?}, thrs: {=u32:?} }}",
                self.thrs4acc(),
                self.thrs()
            )
        }
    }
    #[doc = "the offset setting for edge detection of the q_channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ThrsQ(pub u32);
    impl ThrsQ {
        #[doc = "enable thrs data for accumulate."]
        #[must_use]
        #[inline(always)]
        pub const fn thrs4acc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable thrs data for accumulate."]
        #[inline(always)]
        pub const fn set_thrs4acc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "The offset setting for edge detection of the q_channel, signed number … 2: the offset is 0x800000+2 1: the offset is 0x800000+1 0: the offset is 0x800000 -1: the offset is 0x800000-1 -2: the offset is 0x800000-2 …."]
        #[must_use]
        #[inline(always)]
        pub const fn thrs(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "The offset setting for edge detection of the q_channel, signed number … 2: the offset is 0x800000+2 1: the offset is 0x800000+1 0: the offset is 0x800000 -1: the offset is 0x800000-1 -2: the offset is 0x800000-2 …."]
        #[inline(always)]
        pub const fn set_thrs(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for ThrsQ {
        #[inline(always)]
        fn default() -> ThrsQ {
            ThrsQ(0)
        }
    }
    impl core::fmt::Debug for ThrsQ {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ThrsQ")
                .field("thrs4acc", &self.thrs4acc())
                .field("thrs", &self.thrs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ThrsQ {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ThrsQ {{ thrs4acc: {=bool:?}, thrs: {=u32:?} }}",
                self.thrs4acc(),
                self.thrs()
            )
        }
    }
    #[doc = "Configuration for trigger out 0 in clock cycle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrigOut0Cfg(pub u32);
    impl TrigOut0Cfg {
        #[doc = "Lead time for trigger out0 from center of low level , this is a signed value … 2: 2 cycle befor center of low level 1: 1 cycle before center of low level 0: center of low level -1: 1cycle after center of low level -2: 2cycle after center of low level."]
        #[must_use]
        #[inline(always)]
        pub const fn lead_tim(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Lead time for trigger out0 from center of low level , this is a signed value … 2: 2 cycle befor center of low level 1: 1 cycle before center of low level 0: center of low level -1: 1cycle after center of low level -2: 2cycle after center of low level."]
        #[inline(always)]
        pub const fn set_lead_tim(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Enable trigger out0 0: disable 1: enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger out0 0: disable 1: enable."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for TrigOut0Cfg {
        #[inline(always)]
        fn default() -> TrigOut0Cfg {
            TrigOut0Cfg(0)
        }
    }
    impl core::fmt::Debug for TrigOut0Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrigOut0Cfg")
                .field("lead_tim", &self.lead_tim())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrigOut0Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TrigOut0Cfg {{ lead_tim: {=u32:?}, enable: {=bool:?} }}",
                self.lead_tim(),
                self.enable()
            )
        }
    }
    #[doc = "Configuration for trigger out 1 in clock cycle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrigOut1Cfg(pub u32);
    impl TrigOut1Cfg {
        #[doc = "Lead time for trigger out0 from center of hight level , this is a signed value … 2: 2 cycle befor center of hight level 1: 1 cycle before center of hight level 0: center of hight level -1: 1cycle after center of hight level -2: 2cycle after center of hight level."]
        #[must_use]
        #[inline(always)]
        pub const fn lead_tim(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Lead time for trigger out0 from center of hight level , this is a signed value … 2: 2 cycle befor center of hight level 1: 1 cycle before center of hight level 0: center of hight level -1: 1cycle after center of hight level -2: 2cycle after center of hight level."]
        #[inline(always)]
        pub const fn set_lead_tim(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Enable trigger out1 0: disable 1: enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger out1 0: disable 1: enable."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for TrigOut1Cfg {
        #[inline(always)]
        fn default() -> TrigOut1Cfg {
            TrigOut1Cfg(0)
        }
    }
    impl core::fmt::Debug for TrigOut1Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrigOut1Cfg")
                .field("lead_tim", &self.lead_tim())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrigOut1Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TrigOut1Cfg {{ lead_tim: {=u32:?}, enable: {=bool:?} }}",
                self.lead_tim(),
                self.enable()
            )
        }
    }
}
