#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd {
    ptr: *mut u8,
}
unsafe impl Send for Cmd {}
unsafe impl Sync for Cmd {}
impl Cmd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "tra&index0_cmd&index1_control."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::CmdControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "tra&index0_cmd&index1_rev_preset."]
    #[inline(always)]
    pub const fn rev_preset(self) -> crate::common::Reg<regs::RevPreset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "tra&index0_cmd&index1_pos_preset."]
    #[inline(always)]
    pub const fn pos_preset(self) -> crate::common::Reg<regs::PosPreset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "tra&index0_cmd&index1_vel_preset."]
    #[inline(always)]
    pub const fn vel_preset(self) -> crate::common::Reg<regs::VelPreset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "tra&index0_cmd&index1_acc_preset."]
    #[inline(always)]
    pub const fn acc_preset(self) -> crate::common::Reg<regs::AccPreset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "tra&index0_cmd&index1_jer_preset."]
    #[inline(always)]
    pub const fn jer_preset(self) -> crate::common::Reg<regs::JerPreset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "tra&index0_cmd&index1_timestamp."]
    #[inline(always)]
    pub const fn timestamp(self) -> crate::common::Reg<regs::CmdTimestamp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Event {
    ptr: *mut u8,
}
unsafe impl Send for Event {}
unsafe impl Sync for Event {}
impl Event {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "event&index0_control."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::EventControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "event&index0_preset_0."]
    #[inline(always)]
    pub const fn preset_0(self) -> crate::common::Reg<regs::Preset0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "event&index0_preset_1."]
    #[inline(always)]
    pub const fn preset_1(self) -> crate::common::Reg<regs::Preset1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "event&index0_preset_2."]
    #[inline(always)]
    pub const fn preset_2(self) -> crate::common::Reg<regs::Preset2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "event&index0_preset_3."]
    #[inline(always)]
    pub const fn preset_3(self) -> crate::common::Reg<regs::Preset3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "event&index0_timestamp."]
    #[inline(always)]
    pub const fn timestamp(self) -> crate::common::Reg<regs::EventTimestamp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
#[doc = "MTG0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtg {
    ptr: *mut u8,
}
unsafe impl Send for Mtg {}
unsafe impl Sync for Mtg {}
impl Mtg {
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
    pub const fn tra(self, n: usize) -> Tra {
        assert!(n < 2usize);
        unsafe { Tra::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4096usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn event(self, n: usize) -> Event {
        assert!(n < 4usize);
        unsafe { Event::from_ptr(self.ptr.wrapping_add(0x2000usize + n * 32usize) as _) }
    }
    #[doc = "sw_event."]
    #[inline(always)]
    pub const fn sw_event(self) -> crate::common::Reg<regs::SwEvent, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2080usize) as _) }
    }
    #[doc = "sw_glb_reset."]
    #[inline(always)]
    pub const fn sw_glb_reset(self) -> crate::common::Reg<regs::SwGlbReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2084usize) as _) }
    }
    #[doc = "filter_control."]
    #[inline(always)]
    pub const fn filter_control(
        self,
    ) -> crate::common::Reg<regs::FilterControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3000usize) as _) }
    }
    #[doc = "filter_rev_value."]
    #[inline(always)]
    pub const fn filter_rev_value(
        self,
    ) -> crate::common::Reg<regs::FilterRevValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "filter_pos_value."]
    #[inline(always)]
    pub const fn filter_pos_value(
        self,
    ) -> crate::common::Reg<regs::FilterPosValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "filter_vel_value."]
    #[inline(always)]
    pub const fn filter_vel_value(
        self,
    ) -> crate::common::Reg<regs::FilterVelValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "filter_acc_value."]
    #[inline(always)]
    pub const fn filter_acc_value(
        self,
    ) -> crate::common::Reg<regs::FilterAccValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x301cusize) as _) }
    }
    #[doc = "filter_mot_sel."]
    #[inline(always)]
    pub const fn filter_mot_sel(self) -> crate::common::Reg<regs::FilterMotSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize) as _) }
    }
    #[doc = "filter_stage_sel."]
    #[inline(always)]
    pub const fn filter_stage_sel(
        self,
    ) -> crate::common::Reg<regs::FilterStageSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3024usize) as _) }
    }
    #[doc = "filter_time_constant_tp."]
    #[inline(always)]
    pub const fn filter_time_constant_tp(
        self,
    ) -> crate::common::Reg<regs::FilterTimeConstantTp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3028usize) as _) }
    }
    #[doc = "filter_time_constant_tz."]
    #[inline(always)]
    pub const fn filter_time_constant_tz(
        self,
    ) -> crate::common::Reg<regs::FilterTimeConstantTz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x302cusize) as _) }
    }
    #[doc = "filter_time_constant_tz_1."]
    #[inline(always)]
    pub const fn filter_time_constant_tz_1(
        self,
    ) -> crate::common::Reg<regs::FilterTimeConstantTz1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3030usize) as _) }
    }
    #[doc = "filter_zero_tz_sel."]
    #[inline(always)]
    pub const fn filter_zero_tz_sel(
        self,
    ) -> crate::common::Reg<regs::FilterZeroTzSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3034usize) as _) }
    }
    #[doc = "filter_gain."]
    #[inline(always)]
    pub const fn filter_gain(self) -> crate::common::Reg<regs::FilterGain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3038usize) as _) }
    }
    #[doc = "filter_stage_shift0."]
    #[inline(always)]
    pub const fn filter_stage_shift0(
        self,
    ) -> crate::common::Reg<regs::FilterStageShift0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x303cusize) as _) }
    }
    #[doc = "filter_stage_shift1."]
    #[inline(always)]
    pub const fn filter_stage_shift1(
        self,
    ) -> crate::common::Reg<regs::FilterStageShift1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize) as _) }
    }
    #[doc = "filter_param_shift."]
    #[inline(always)]
    pub const fn filter_param_shift(
        self,
    ) -> crate::common::Reg<regs::FilterParamShift, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3044usize) as _) }
    }
    #[doc = "filter_time_shift."]
    #[inline(always)]
    pub const fn filter_time_shift(
        self,
    ) -> crate::common::Reg<regs::FilterTimeShift, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3048usize) as _) }
    }
    #[doc = "filter_ff_shift."]
    #[inline(always)]
    pub const fn filter_ff_shift(
        self,
    ) -> crate::common::Reg<regs::FilterFfShift, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x304cusize) as _) }
    }
    #[doc = "filter_time1_sw_adjust."]
    #[inline(always)]
    pub const fn filter_time1_sw_adjust(
        self,
    ) -> crate::common::Reg<regs::FilterTime1SwAdjust, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3050usize) as _) }
    }
    #[doc = "filter_time0_sw_adjust."]
    #[inline(always)]
    pub const fn filter_time0_sw_adjust(
        self,
    ) -> crate::common::Reg<regs::FilterTime0SwAdjust, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3054usize) as _) }
    }
    #[doc = "filter_error_limit."]
    #[inline(always)]
    pub const fn filter_error_limit_l(
        self,
    ) -> crate::common::Reg<regs::FilterErrorLimitL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3060usize) as _) }
    }
    #[doc = "filter_error_limit."]
    #[inline(always)]
    pub const fn filter_error_limit_h(
        self,
    ) -> crate::common::Reg<regs::FilterErrorLimitH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3064usize) as _) }
    }
    #[doc = "filter_timeout_cnt."]
    #[inline(always)]
    pub const fn filter_timeout_cnt(
        self,
    ) -> crate::common::Reg<regs::FilterTimeoutCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x306cusize) as _) }
    }
    #[doc = "filter_rev_lock."]
    #[inline(always)]
    pub const fn filter_rev_lock(
        self,
    ) -> crate::common::Reg<regs::FilterRevLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3070usize) as _) }
    }
    #[doc = "filter_pos_lock."]
    #[inline(always)]
    pub const fn filter_pos_lock(
        self,
    ) -> crate::common::Reg<regs::FilterPosLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3074usize) as _) }
    }
    #[doc = "filter_vel_lock."]
    #[inline(always)]
    pub const fn filter_vel_lock(
        self,
    ) -> crate::common::Reg<regs::FilterVelLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3078usize) as _) }
    }
    #[doc = "filter_acc_lock."]
    #[inline(always)]
    pub const fn filter_acc_lock(
        self,
    ) -> crate::common::Reg<regs::FilterAccLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x307cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tra {
    ptr: *mut u8,
}
unsafe impl Send for Tra {}
unsafe impl Sync for Tra {}
impl Tra {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "tra&index0_control."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::TraControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "tra&index0_shift."]
    #[inline(always)]
    pub const fn shift(self) -> crate::common::Reg<regs::Shift, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "tra&index0_link."]
    #[inline(always)]
    pub const fn link(self) -> crate::common::Reg<regs::Link, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmd(self, n: usize) -> Cmd {
        assert!(n < 4usize);
        unsafe { Cmd::from_ptr(self.ptr.wrapping_add(0x20usize + n * 32usize) as _) }
    }
    #[doc = "tra&index0_lock_rev."]
    #[inline(always)]
    pub const fn lock_rev(self) -> crate::common::Reg<regs::LockRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "tra&index0_lock_pos."]
    #[inline(always)]
    pub const fn lock_pos(self) -> crate::common::Reg<regs::LockPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "tra&index0_lock_vel."]
    #[inline(always)]
    pub const fn lock_vel(self) -> crate::common::Reg<regs::LockVel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "tra&index0_lock_acc."]
    #[inline(always)]
    pub const fn lock_acc(self) -> crate::common::Reg<regs::LockAcc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "tra&index0_lock_time."]
    #[inline(always)]
    pub const fn lock_time(self) -> crate::common::Reg<regs::LockTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "tra&index0_step_limit_ctrl."]
    #[inline(always)]
    pub const fn step_limit_ctrl(
        self,
    ) -> crate::common::Reg<regs::StepLimitCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "tra&index0_vel_step_max."]
    #[inline(always)]
    pub const fn vel_step_max(self) -> crate::common::Reg<regs::VelStepMax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "tra&index0_vel_step_min."]
    #[inline(always)]
    pub const fn vel_step_min(self) -> crate::common::Reg<regs::VelStepMin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "tra&index0_pos_step_max."]
    #[inline(always)]
    pub const fn pos_step_max(self) -> crate::common::Reg<regs::PosStepMax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "tra&index0_pos_step_min."]
    #[inline(always)]
    pub const fn pos_step_min(self) -> crate::common::Reg<regs::PosStepMin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "tra&index0_vel_limit_p."]
    #[inline(always)]
    pub const fn vel_limit_p(self) -> crate::common::Reg<regs::VelLimitP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "tra&index0_vel_limit_n."]
    #[inline(always)]
    pub const fn vel_limit_n(self) -> crate::common::Reg<regs::VelLimitN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
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
    #[doc = "tra&index0_cmd&index1_acc_preset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccPreset(pub u32);
    impl AccPreset {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_preset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_acc_preset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AccPreset {
        #[inline(always)]
        fn default() -> AccPreset {
            AccPreset(0)
        }
    }
    impl core::fmt::Debug for AccPreset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AccPreset")
                .field("acc_preset", &self.acc_preset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AccPreset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AccPreset {{ acc_preset: {=u32:?} }}", self.acc_preset())
        }
    }
    #[doc = "tra&index0_cmd&index1_control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdControl(pub u32);
    impl CmdControl {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn object(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_object(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pass_irq_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pass_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pass_irq(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pass_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CmdControl {
        #[inline(always)]
        fn default() -> CmdControl {
            CmdControl(0)
        }
    }
    impl core::fmt::Debug for CmdControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmdControl")
                .field("object", &self.object())
                .field("mode", &self.mode())
                .field("pass_irq_en", &self.pass_irq_en())
                .field("pass_irq", &self.pass_irq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmdControl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CmdControl {{ object: {=u8:?}, mode: {=bool:?}, pass_irq_en: {=bool:?}, pass_irq: {=bool:?} }}" , self . object () , self . mode () , self . pass_irq_en () , self . pass_irq ())
        }
    }
    #[doc = "tra&index0_cmd&index1_timestamp."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdTimestamp(pub u32);
    impl CmdTimestamp {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn timestamp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_timestamp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CmdTimestamp {
        #[inline(always)]
        fn default() -> CmdTimestamp {
            CmdTimestamp(0)
        }
    }
    impl core::fmt::Debug for CmdTimestamp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmdTimestamp")
                .field("timestamp", &self.timestamp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmdTimestamp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CmdTimestamp {{ timestamp: {=u32:?} }}",
                self.timestamp()
            )
        }
    }
    #[doc = "event&index0_control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EventControl(pub u32);
    impl EventControl {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn event_irq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_event_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn event_over_irq(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_event_over_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn event_irq_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_event_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn event_over_irq_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_event_over_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn trig_num(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_trig_num(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn over_mode_cmp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_over_mode_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn dir_mode(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_dir_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn object(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_object(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 23usize)) | (((val as u32) & 0x0f) << 23usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn source_mux(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_source_mux(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EventControl {
        #[inline(always)]
        fn default() -> EventControl {
            EventControl(0)
        }
    }
    impl core::fmt::Debug for EventControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EventControl")
                .field("event_irq", &self.event_irq())
                .field("event_over_irq", &self.event_over_irq())
                .field("event_irq_en", &self.event_irq_en())
                .field("event_over_irq_en", &self.event_over_irq_en())
                .field("trig_num", &self.trig_num())
                .field("over_mode_cmp", &self.over_mode_cmp())
                .field("dir_mode", &self.dir_mode())
                .field("dir", &self.dir())
                .field("mode", &self.mode())
                .field("object", &self.object())
                .field("source_mux", &self.source_mux())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EventControl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EventControl {{ event_irq: {=bool:?}, event_over_irq: {=bool:?}, event_irq_en: {=bool:?}, event_over_irq_en: {=bool:?}, trig_num: {=bool:?}, over_mode_cmp: {=bool:?}, dir_mode: {=bool:?}, dir: {=u8:?}, mode: {=u8:?}, object: {=u8:?}, source_mux: {=u8:?}, enable: {=bool:?} }}" , self . event_irq () , self . event_over_irq () , self . event_irq_en () , self . event_over_irq_en () , self . trig_num () , self . over_mode_cmp () , self . dir_mode () , self . dir () , self . mode () , self . object () , self . source_mux () , self . enable ())
        }
    }
    #[doc = "event&index0_timestamp."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EventTimestamp(pub u32);
    impl EventTimestamp {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn timestamp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_timestamp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EventTimestamp {
        #[inline(always)]
        fn default() -> EventTimestamp {
            EventTimestamp(0)
        }
    }
    impl core::fmt::Debug for EventTimestamp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EventTimestamp")
                .field("timestamp", &self.timestamp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EventTimestamp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EventTimestamp {{ timestamp: {=u32:?} }}",
                self.timestamp()
            )
        }
    }
    #[doc = "filter_acc_lock."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterAccLock(pub u32);
    impl FilterAccLock {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_status(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_acc_status(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FilterAccLock {
        #[inline(always)]
        fn default() -> FilterAccLock {
            FilterAccLock(0)
        }
    }
    impl core::fmt::Debug for FilterAccLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterAccLock")
                .field("acc_status", &self.acc_status())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterAccLock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FilterAccLock {{ acc_status: {=u32:?} }}",
                self.acc_status()
            )
        }
    }
    #[doc = "filter_acc_value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterAccValue(pub u32);
    impl FilterAccValue {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FilterAccValue {
        #[inline(always)]
        fn default() -> FilterAccValue {
            FilterAccValue(0)
        }
    }
    impl core::fmt::Debug for FilterAccValue {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterAccValue")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterAccValue {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FilterAccValue {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "filter_control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterControl(pub u32);
    impl FilterControl {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn init_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_init_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ff_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ff_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ff_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ff_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn err_bypass_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_err_bypass_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn err_ini(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_err_ini(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn a_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_a_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn en_time0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_en_time0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn en_time1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_en_time1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_time0(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sel_time0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sel_time1(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sel_time1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn rev_ini_mode(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_rev_ini_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn timeout_en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_timeout_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_lock(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sw_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn err_bypass_i_f_en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_err_bypass_i_f_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn err_bypass_f_i_en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_err_bypass_f_i_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn err_bypass_status(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_err_bypass_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mul_err_irq_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mul_err_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mul_err_irq_1(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mul_err_irq_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn mul_err_irq_0(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_mul_err_irq_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FilterControl {
        #[inline(always)]
        fn default() -> FilterControl {
            FilterControl(0)
        }
    }
    impl core::fmt::Debug for FilterControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterControl")
                .field("enable", &self.enable())
                .field("init_en", &self.init_en())
                .field("ff_en", &self.ff_en())
                .field("ff_mode", &self.ff_mode())
                .field("err_bypass_en", &self.err_bypass_en())
                .field("err_ini", &self.err_ini())
                .field("a_en", &self.a_en())
                .field("en_time0", &self.en_time0())
                .field("en_time1", &self.en_time1())
                .field("sel_time0", &self.sel_time0())
                .field("sel_time1", &self.sel_time1())
                .field("rev_ini_mode", &self.rev_ini_mode())
                .field("timeout_en", &self.timeout_en())
                .field("sw_lock", &self.sw_lock())
                .field("err_bypass_i_f_en", &self.err_bypass_i_f_en())
                .field("err_bypass_f_i_en", &self.err_bypass_f_i_en())
                .field("err_bypass_status", &self.err_bypass_status())
                .field("mul_err_irq_en", &self.mul_err_irq_en())
                .field("mul_err_irq_1", &self.mul_err_irq_1())
                .field("mul_err_irq_0", &self.mul_err_irq_0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterControl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FilterControl {{ enable: {=bool:?}, init_en: {=bool:?}, ff_en: {=bool:?}, ff_mode: {=bool:?}, err_bypass_en: {=bool:?}, err_ini: {=bool:?}, a_en: {=bool:?}, en_time0: {=bool:?}, en_time1: {=bool:?}, sel_time0: {=u8:?}, sel_time1: {=u8:?}, rev_ini_mode: {=bool:?}, timeout_en: {=bool:?}, sw_lock: {=bool:?}, err_bypass_i_f_en: {=bool:?}, err_bypass_f_i_en: {=bool:?}, err_bypass_status: {=bool:?}, mul_err_irq_en: {=bool:?}, mul_err_irq_1: {=bool:?}, mul_err_irq_0: {=bool:?} }}" , self . enable () , self . init_en () , self . ff_en () , self . ff_mode () , self . err_bypass_en () , self . err_ini () , self . a_en () , self . en_time0 () , self . en_time1 () , self . sel_time0 () , self . sel_time1 () , self . rev_ini_mode () , self . timeout_en () , self . sw_lock () , self . err_bypass_i_f_en () , self . err_bypass_f_i_en () , self . err_bypass_status () , self . mul_err_irq_en () , self . mul_err_irq_1 () , self . mul_err_irq_0 ())
        }
    }
    #[doc = "filter_error_limit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterErrorLimitH(pub u32);
    impl FilterErrorLimitH {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn error_limit_h(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_error_limit_h(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FilterErrorLimitH {
        #[inline(always)]
        fn default() -> FilterErrorLimitH {
            FilterErrorLimitH(0)
        }
    }
    impl core::fmt::Debug for FilterErrorLimitH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterErrorLimitH")
                .field("error_limit_h", &self.error_limit_h())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterErrorLimitH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FilterErrorLimitH {{ error_limit_h: {=u32:?} }}",
                self.error_limit_h()
            )
        }
    }
    #[doc = "filter_error_limit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterErrorLimitL(pub u32);
    impl FilterErrorLimitL {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn error_limit_l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_error_limit_l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FilterErrorLimitL {
        #[inline(always)]
        fn default() -> FilterErrorLimitL {
            FilterErrorLimitL(0)
        }
    }
    impl core::fmt::Debug for FilterErrorLimitL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterErrorLimitL")
                .field("error_limit_l", &self.error_limit_l())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterErrorLimitL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FilterErrorLimitL {{ error_limit_l: {=u32:?} }}",
                self.error_limit_l()
            )
        }
    }
    #[doc = "filter_ff_shift."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterFfShift(pub u32);
    impl FilterFfShift {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn filter_vel_shift(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_filter_vel_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn output_vel_shift(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_output_vel_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn filter_acc_shift(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_filter_acc_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn output_acc_shift(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_output_acc_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for FilterFfShift {
        #[inline(always)]
        fn default() -> FilterFfShift {
            FilterFfShift(0)
        }
    }
    impl core::fmt::Debug for FilterFfShift {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterFfShift")
                .field("filter_vel_shift", &self.filter_vel_shift())
                .field("output_vel_shift", &self.output_vel_shift())
                .field("filter_acc_shift", &self.filter_acc_shift())
                .field("output_acc_shift", &self.output_acc_shift())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterFfShift {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FilterFfShift {{ filter_vel_shift: {=u8:?}, output_vel_shift: {=u8:?}, filter_acc_shift: {=u8:?}, output_acc_shift: {=u8:?} }}" , self . filter_vel_shift () , self . output_vel_shift () , self . filter_acc_shift () , self . output_acc_shift ())
        }
    }
    #[doc = "filter_gain."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterGain(pub u32);
    impl FilterGain {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn k(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_k(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn gain_t1_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_gain_t1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn gain_t0_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_gain_t0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FilterGain {
        #[inline(always)]
        fn default() -> FilterGain {
            FilterGain(0)
        }
    }
    impl core::fmt::Debug for FilterGain {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterGain")
                .field("k", &self.k())
                .field("gain_t1_en", &self.gain_t1_en())
                .field("gain_t0_en", &self.gain_t0_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterGain {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FilterGain {{ k: {=u32:?}, gain_t1_en: {=bool:?}, gain_t0_en: {=bool:?} }}",
                self.k(),
                self.gain_t1_en(),
                self.gain_t0_en()
            )
        }
    }
    #[doc = "filter_mot_sel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterMotSel(pub u32);
    impl FilterMotSel {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn filter_acc_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_filter_acc_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn filter_vel_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_filter_vel_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn output_acc_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_output_acc_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn output_vel_sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_output_vel_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for FilterMotSel {
        #[inline(always)]
        fn default() -> FilterMotSel {
            FilterMotSel(0)
        }
    }
    impl core::fmt::Debug for FilterMotSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterMotSel")
                .field("filter_acc_sel", &self.filter_acc_sel())
                .field("filter_vel_sel", &self.filter_vel_sel())
                .field("output_acc_sel", &self.output_acc_sel())
                .field("output_vel_sel", &self.output_vel_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterMotSel {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FilterMotSel {{ filter_acc_sel: {=u8:?}, filter_vel_sel: {=u8:?}, output_acc_sel: {=u8:?}, output_vel_sel: {=u8:?} }}" , self . filter_acc_sel () , self . filter_vel_sel () , self . output_acc_sel () , self . output_vel_sel ())
        }
    }
    #[doc = "filter_param_shift."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterParamShift(pub u32);
    impl FilterParamShift {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tz_shift(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tz_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tz_1_shift(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tz_1_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tp_shift(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tp_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn gain_t1_shift(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_gain_t1_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn gain_t0_shift(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_gain_t0_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn gain_k_shift(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_gain_k_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_shift_param(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_shift_param(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_shift_param(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_acc_shift_param(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for FilterParamShift {
        #[inline(always)]
        fn default() -> FilterParamShift {
            FilterParamShift(0)
        }
    }
    impl core::fmt::Debug for FilterParamShift {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterParamShift")
                .field("tz_shift", &self.tz_shift())
                .field("tz_1_shift", &self.tz_1_shift())
                .field("tp_shift", &self.tp_shift())
                .field("gain_t1_shift", &self.gain_t1_shift())
                .field("gain_t0_shift", &self.gain_t0_shift())
                .field("gain_k_shift", &self.gain_k_shift())
                .field("vel_shift_param", &self.vel_shift_param())
                .field("acc_shift_param", &self.acc_shift_param())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterParamShift {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FilterParamShift {{ tz_shift: {=u8:?}, tz_1_shift: {=u8:?}, tp_shift: {=u8:?}, gain_t1_shift: {=u8:?}, gain_t0_shift: {=u8:?}, gain_k_shift: {=u8:?}, vel_shift_param: {=u8:?}, acc_shift_param: {=u8:?} }}" , self . tz_shift () , self . tz_1_shift () , self . tp_shift () , self . gain_t1_shift () , self . gain_t0_shift () , self . gain_k_shift () , self . vel_shift_param () , self . acc_shift_param ())
        }
    }
    #[doc = "filter_pos_lock."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterPosLock(pub u32);
    impl FilterPosLock {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_status(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pos_status(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FilterPosLock {
        #[inline(always)]
        fn default() -> FilterPosLock {
            FilterPosLock(0)
        }
    }
    impl core::fmt::Debug for FilterPosLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterPosLock")
                .field("pos_status", &self.pos_status())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterPosLock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FilterPosLock {{ pos_status: {=u32:?} }}",
                self.pos_status()
            )
        }
    }
    #[doc = "filter_pos_value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterPosValue(pub u32);
    impl FilterPosValue {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FilterPosValue {
        #[inline(always)]
        fn default() -> FilterPosValue {
            FilterPosValue(0)
        }
    }
    impl core::fmt::Debug for FilterPosValue {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterPosValue")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterPosValue {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FilterPosValue {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "filter_rev_lock."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterRevLock(pub u32);
    impl FilterRevLock {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn rev_status(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_rev_status(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FilterRevLock {
        #[inline(always)]
        fn default() -> FilterRevLock {
            FilterRevLock(0)
        }
    }
    impl core::fmt::Debug for FilterRevLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterRevLock")
                .field("rev_status", &self.rev_status())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterRevLock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FilterRevLock {{ rev_status: {=u32:?} }}",
                self.rev_status()
            )
        }
    }
    #[doc = "filter_rev_value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterRevValue(pub u32);
    impl FilterRevValue {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FilterRevValue {
        #[inline(always)]
        fn default() -> FilterRevValue {
            FilterRevValue(0)
        }
    }
    impl core::fmt::Debug for FilterRevValue {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterRevValue")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterRevValue {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FilterRevValue {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "filter_stage_sel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterStageSel(pub u32);
    impl FilterStageSel {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage0_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage0_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage1_sel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage1_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage2_sel(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage2_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage3_sel(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage3_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage4_sel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage4_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage5_sel(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage5_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
    }
    impl Default for FilterStageSel {
        #[inline(always)]
        fn default() -> FilterStageSel {
            FilterStageSel(0)
        }
    }
    impl core::fmt::Debug for FilterStageSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterStageSel")
                .field("stage0_sel", &self.stage0_sel())
                .field("stage1_sel", &self.stage1_sel())
                .field("stage2_sel", &self.stage2_sel())
                .field("stage3_sel", &self.stage3_sel())
                .field("stage4_sel", &self.stage4_sel())
                .field("stage5_sel", &self.stage5_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterStageSel {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FilterStageSel {{ stage0_sel: {=u8:?}, stage1_sel: {=u8:?}, stage2_sel: {=u8:?}, stage3_sel: {=u8:?}, stage4_sel: {=u8:?}, stage5_sel: {=u8:?} }}" , self . stage0_sel () , self . stage1_sel () , self . stage2_sel () , self . stage3_sel () , self . stage4_sel () , self . stage5_sel ())
        }
    }
    #[doc = "filter_stage_shift0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterStageShift0(pub u32);
    impl FilterStageShift0 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage0_shift0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage0_shift0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage0_shift1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage0_shift1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage1_shift0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage1_shift0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage1_shift1(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage1_shift1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage2_shift0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage2_shift0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage2_shift1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage2_shift1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage3_shift0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage3_shift0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage3_shift1(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage3_shift1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for FilterStageShift0 {
        #[inline(always)]
        fn default() -> FilterStageShift0 {
            FilterStageShift0(0)
        }
    }
    impl core::fmt::Debug for FilterStageShift0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterStageShift0")
                .field("stage0_shift0", &self.stage0_shift0())
                .field("stage0_shift1", &self.stage0_shift1())
                .field("stage1_shift0", &self.stage1_shift0())
                .field("stage1_shift1", &self.stage1_shift1())
                .field("stage2_shift0", &self.stage2_shift0())
                .field("stage2_shift1", &self.stage2_shift1())
                .field("stage3_shift0", &self.stage3_shift0())
                .field("stage3_shift1", &self.stage3_shift1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterStageShift0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FilterStageShift0 {{ stage0_shift0: {=u8:?}, stage0_shift1: {=u8:?}, stage1_shift0: {=u8:?}, stage1_shift1: {=u8:?}, stage2_shift0: {=u8:?}, stage2_shift1: {=u8:?}, stage3_shift0: {=u8:?}, stage3_shift1: {=u8:?} }}" , self . stage0_shift0 () , self . stage0_shift1 () , self . stage1_shift0 () , self . stage1_shift1 () , self . stage2_shift0 () , self . stage2_shift1 () , self . stage3_shift0 () , self . stage3_shift1 ())
        }
    }
    #[doc = "filter_stage_shift1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterStageShift1(pub u32);
    impl FilterStageShift1 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage4_shift0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage4_shift0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage4_shift1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage4_shift1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage5_shift0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage5_shift0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage5_shift1(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage5_shift1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for FilterStageShift1 {
        #[inline(always)]
        fn default() -> FilterStageShift1 {
            FilterStageShift1(0)
        }
    }
    impl core::fmt::Debug for FilterStageShift1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterStageShift1")
                .field("stage4_shift0", &self.stage4_shift0())
                .field("stage4_shift1", &self.stage4_shift1())
                .field("stage5_shift0", &self.stage5_shift0())
                .field("stage5_shift1", &self.stage5_shift1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterStageShift1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FilterStageShift1 {{ stage4_shift0: {=u8:?}, stage4_shift1: {=u8:?}, stage5_shift0: {=u8:?}, stage5_shift1: {=u8:?} }}" , self . stage4_shift0 () , self . stage4_shift1 () , self . stage5_shift0 () , self . stage5_shift1 ())
        }
    }
    #[doc = "filter_time0_sw_adjust."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterTime0SwAdjust(pub u32);
    impl FilterTime0SwAdjust {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FilterTime0SwAdjust {
        #[inline(always)]
        fn default() -> FilterTime0SwAdjust {
            FilterTime0SwAdjust(0)
        }
    }
    impl core::fmt::Debug for FilterTime0SwAdjust {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterTime0SwAdjust")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterTime0SwAdjust {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FilterTime0SwAdjust {{ time: {=u32:?} }}", self.time())
        }
    }
    #[doc = "filter_time1_sw_adjust."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterTime1SwAdjust(pub u32);
    impl FilterTime1SwAdjust {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FilterTime1SwAdjust {
        #[inline(always)]
        fn default() -> FilterTime1SwAdjust {
            FilterTime1SwAdjust(0)
        }
    }
    impl core::fmt::Debug for FilterTime1SwAdjust {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterTime1SwAdjust")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterTime1SwAdjust {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FilterTime1SwAdjust {{ time: {=u32:?} }}", self.time())
        }
    }
    #[doc = "filter_time_constant_tp."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterTimeConstantTp(pub u32);
    impl FilterTimeConstantTp {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tp(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for FilterTimeConstantTp {
        #[inline(always)]
        fn default() -> FilterTimeConstantTp {
            FilterTimeConstantTp(0)
        }
    }
    impl core::fmt::Debug for FilterTimeConstantTp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterTimeConstantTp")
                .field("tp", &self.tp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterTimeConstantTp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FilterTimeConstantTp {{ tp: {=u32:?} }}", self.tp())
        }
    }
    #[doc = "filter_time_constant_tz."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterTimeConstantTz(pub u32);
    impl FilterTimeConstantTz {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for FilterTimeConstantTz {
        #[inline(always)]
        fn default() -> FilterTimeConstantTz {
            FilterTimeConstantTz(0)
        }
    }
    impl core::fmt::Debug for FilterTimeConstantTz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterTimeConstantTz")
                .field("tz", &self.tz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterTimeConstantTz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FilterTimeConstantTz {{ tz: {=u32:?} }}", self.tz())
        }
    }
    #[doc = "filter_time_constant_tz_1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterTimeConstantTz1(pub u32);
    impl FilterTimeConstantTz1 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn tz_1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_tz_1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for FilterTimeConstantTz1 {
        #[inline(always)]
        fn default() -> FilterTimeConstantTz1 {
            FilterTimeConstantTz1(0)
        }
    }
    impl core::fmt::Debug for FilterTimeConstantTz1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterTimeConstantTz1")
                .field("tz_1", &self.tz_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterTimeConstantTz1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FilterTimeConstantTz1 {{ tz_1: {=u32:?} }}", self.tz_1())
        }
    }
    #[doc = "filter_time_shift."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterTimeShift(pub u32);
    impl FilterTimeShift {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_shift_time0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_shift_time0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_shift_time0(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_acc_shift_time0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_shift_time1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_shift_time1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_shift_time1(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_acc_shift_time1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for FilterTimeShift {
        #[inline(always)]
        fn default() -> FilterTimeShift {
            FilterTimeShift(0)
        }
    }
    impl core::fmt::Debug for FilterTimeShift {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterTimeShift")
                .field("vel_shift_time0", &self.vel_shift_time0())
                .field("acc_shift_time0", &self.acc_shift_time0())
                .field("vel_shift_time1", &self.vel_shift_time1())
                .field("acc_shift_time1", &self.acc_shift_time1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterTimeShift {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FilterTimeShift {{ vel_shift_time0: {=u8:?}, acc_shift_time0: {=u8:?}, vel_shift_time1: {=u8:?}, acc_shift_time1: {=u8:?} }}" , self . vel_shift_time0 () , self . acc_shift_time0 () , self . vel_shift_time1 () , self . acc_shift_time1 ())
        }
    }
    #[doc = "filter_timeout_cnt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterTimeoutCnt(pub u32);
    impl FilterTimeoutCnt {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn timeout_cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_timeout_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FilterTimeoutCnt {
        #[inline(always)]
        fn default() -> FilterTimeoutCnt {
            FilterTimeoutCnt(0)
        }
    }
    impl core::fmt::Debug for FilterTimeoutCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterTimeoutCnt")
                .field("timeout_cnt", &self.timeout_cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterTimeoutCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FilterTimeoutCnt {{ timeout_cnt: {=u32:?} }}",
                self.timeout_cnt()
            )
        }
    }
    #[doc = "filter_vel_lock."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterVelLock(pub u32);
    impl FilterVelLock {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_status(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_status(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FilterVelLock {
        #[inline(always)]
        fn default() -> FilterVelLock {
            FilterVelLock(0)
        }
    }
    impl core::fmt::Debug for FilterVelLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterVelLock")
                .field("vel_status", &self.vel_status())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterVelLock {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FilterVelLock {{ vel_status: {=u32:?} }}",
                self.vel_status()
            )
        }
    }
    #[doc = "filter_vel_value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterVelValue(pub u32);
    impl FilterVelValue {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for FilterVelValue {
        #[inline(always)]
        fn default() -> FilterVelValue {
            FilterVelValue(0)
        }
    }
    impl core::fmt::Debug for FilterVelValue {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterVelValue")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterVelValue {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FilterVelValue {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "filter_zero_tz_sel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterZeroTzSel(pub u32);
    impl FilterZeroTzSel {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn stage5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_stage5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for FilterZeroTzSel {
        #[inline(always)]
        fn default() -> FilterZeroTzSel {
            FilterZeroTzSel(0)
        }
    }
    impl core::fmt::Debug for FilterZeroTzSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FilterZeroTzSel")
                .field("stage0", &self.stage0())
                .field("stage1", &self.stage1())
                .field("stage2", &self.stage2())
                .field("stage3", &self.stage3())
                .field("stage4", &self.stage4())
                .field("stage5", &self.stage5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FilterZeroTzSel {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FilterZeroTzSel {{ stage0: {=bool:?}, stage1: {=bool:?}, stage2: {=bool:?}, stage3: {=bool:?}, stage4: {=bool:?}, stage5: {=bool:?} }}" , self . stage0 () , self . stage1 () , self . stage2 () , self . stage3 () , self . stage4 () , self . stage5 ())
        }
    }
    #[doc = "tra&index0_cmd&index1_jer_preset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JerPreset(pub u32);
    impl JerPreset {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn jer_preset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_jer_preset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for JerPreset {
        #[inline(always)]
        fn default() -> JerPreset {
            JerPreset(0)
        }
    }
    impl core::fmt::Debug for JerPreset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JerPreset")
                .field("jer_preset", &self.jer_preset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JerPreset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "JerPreset {{ jer_preset: {=u32:?} }}", self.jer_preset())
        }
    }
    #[doc = "tra&index0_link."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Link(pub u32);
    impl Link {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn link_cfg_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_link_cfg_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn link_cfg_1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_link_cfg_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn link_cfg_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_link_cfg_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn link_cfg_3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_link_cfg_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
    }
    impl Default for Link {
        #[inline(always)]
        fn default() -> Link {
            Link(0)
        }
    }
    impl core::fmt::Debug for Link {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Link")
                .field("link_cfg_0", &self.link_cfg_0())
                .field("link_cfg_1", &self.link_cfg_1())
                .field("link_cfg_2", &self.link_cfg_2())
                .field("link_cfg_3", &self.link_cfg_3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Link {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Link {{ link_cfg_0: {=u8:?}, link_cfg_1: {=u8:?}, link_cfg_2: {=u8:?}, link_cfg_3: {=u8:?} }}" , self . link_cfg_0 () , self . link_cfg_1 () , self . link_cfg_2 () , self . link_cfg_3 ())
        }
    }
    #[doc = "tra&index0_lock_acc."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LockAcc(pub u32);
    impl LockAcc {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_acc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_lock_acc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LockAcc {
        #[inline(always)]
        fn default() -> LockAcc {
            LockAcc(0)
        }
    }
    impl core::fmt::Debug for LockAcc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LockAcc")
                .field("lock_acc", &self.lock_acc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LockAcc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LockAcc {{ lock_acc: {=u32:?} }}", self.lock_acc())
        }
    }
    #[doc = "tra&index0_lock_pos."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LockPos(pub u32);
    impl LockPos {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_pos(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_lock_pos(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LockPos {
        #[inline(always)]
        fn default() -> LockPos {
            LockPos(0)
        }
    }
    impl core::fmt::Debug for LockPos {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LockPos")
                .field("lock_pos", &self.lock_pos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LockPos {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LockPos {{ lock_pos: {=u32:?} }}", self.lock_pos())
        }
    }
    #[doc = "tra&index0_lock_rev."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LockRev(pub u32);
    impl LockRev {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_rev(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_lock_rev(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LockRev {
        #[inline(always)]
        fn default() -> LockRev {
            LockRev(0)
        }
    }
    impl core::fmt::Debug for LockRev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LockRev")
                .field("lock_rev", &self.lock_rev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LockRev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LockRev {{ lock_rev: {=u32:?} }}", self.lock_rev())
        }
    }
    #[doc = "tra&index0_lock_time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LockTime(pub u32);
    impl LockTime {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_lock_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LockTime {
        #[inline(always)]
        fn default() -> LockTime {
            LockTime(0)
        }
    }
    impl core::fmt::Debug for LockTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LockTime")
                .field("lock_time", &self.lock_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LockTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LockTime {{ lock_time: {=u32:?} }}", self.lock_time())
        }
    }
    #[doc = "tra&index0_lock_vel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LockVel(pub u32);
    impl LockVel {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_vel(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_lock_vel(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LockVel {
        #[inline(always)]
        fn default() -> LockVel {
            LockVel(0)
        }
    }
    impl core::fmt::Debug for LockVel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LockVel")
                .field("lock_vel", &self.lock_vel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LockVel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LockVel {{ lock_vel: {=u32:?} }}", self.lock_vel())
        }
    }
    #[doc = "tra&index0_cmd&index1_pos_preset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosPreset(pub u32);
    impl PosPreset {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_preset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pos_preset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosPreset {
        #[inline(always)]
        fn default() -> PosPreset {
            PosPreset(0)
        }
    }
    impl core::fmt::Debug for PosPreset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosPreset")
                .field("pos_preset", &self.pos_preset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosPreset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosPreset {{ pos_preset: {=u32:?} }}", self.pos_preset())
        }
    }
    #[doc = "tra&index0_pos_step_max."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosStepMax(pub u32);
    impl PosStepMax {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_step_max(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pos_step_max(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosStepMax {
        #[inline(always)]
        fn default() -> PosStepMax {
            PosStepMax(0)
        }
    }
    impl core::fmt::Debug for PosStepMax {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosStepMax")
                .field("pos_step_max", &self.pos_step_max())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosStepMax {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PosStepMax {{ pos_step_max: {=u32:?} }}",
                self.pos_step_max()
            )
        }
    }
    #[doc = "tra&index0_pos_step_min."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosStepMin(pub u32);
    impl PosStepMin {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_step_min(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pos_step_min(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosStepMin {
        #[inline(always)]
        fn default() -> PosStepMin {
            PosStepMin(0)
        }
    }
    impl core::fmt::Debug for PosStepMin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosStepMin")
                .field("pos_step_min", &self.pos_step_min())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosStepMin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PosStepMin {{ pos_step_min: {=u32:?} }}",
                self.pos_step_min()
            )
        }
    }
    #[doc = "event&index0_preset_0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Preset0(pub u32);
    impl Preset0 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn preset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_preset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Preset0 {
        #[inline(always)]
        fn default() -> Preset0 {
            Preset0(0)
        }
    }
    impl core::fmt::Debug for Preset0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Preset0")
                .field("preset", &self.preset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Preset0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Preset0 {{ preset: {=u32:?} }}", self.preset())
        }
    }
    #[doc = "event&index0_preset_1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Preset1(pub u32);
    impl Preset1 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn preset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_preset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Preset1 {
        #[inline(always)]
        fn default() -> Preset1 {
            Preset1(0)
        }
    }
    impl core::fmt::Debug for Preset1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Preset1")
                .field("preset", &self.preset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Preset1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Preset1 {{ preset: {=u32:?} }}", self.preset())
        }
    }
    #[doc = "event&index0_preset_2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Preset2(pub u32);
    impl Preset2 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn preset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_preset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Preset2 {
        #[inline(always)]
        fn default() -> Preset2 {
            Preset2(0)
        }
    }
    impl core::fmt::Debug for Preset2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Preset2")
                .field("preset", &self.preset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Preset2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Preset2 {{ preset: {=u32:?} }}", self.preset())
        }
    }
    #[doc = "event&index0_preset_3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Preset3(pub u32);
    impl Preset3 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn preset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_preset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Preset3 {
        #[inline(always)]
        fn default() -> Preset3 {
            Preset3(0)
        }
    }
    impl core::fmt::Debug for Preset3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Preset3")
                .field("preset", &self.preset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Preset3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Preset3 {{ preset: {=u32:?} }}", self.preset())
        }
    }
    #[doc = "tra&index0_cmd&index1_rev_preset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RevPreset(pub u32);
    impl RevPreset {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn rev_preset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_rev_preset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RevPreset {
        #[inline(always)]
        fn default() -> RevPreset {
            RevPreset(0)
        }
    }
    impl core::fmt::Debug for RevPreset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RevPreset")
                .field("rev_preset", &self.rev_preset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RevPreset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RevPreset {{ rev_preset: {=u32:?} }}", self.rev_preset())
        }
    }
    #[doc = "tra&index0_shift."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Shift(pub u32);
    impl Shift {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_shift(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_shift(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_acc_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn jer_shift(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_jer_shift(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn shift_fail_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_shift_fail_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_shift_fail_irq(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_shift_fail_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_shift_fail_irq(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_acc_shift_fail_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Shift {
        #[inline(always)]
        fn default() -> Shift {
            Shift(0)
        }
    }
    impl core::fmt::Debug for Shift {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Shift")
                .field("vel_shift", &self.vel_shift())
                .field("acc_shift", &self.acc_shift())
                .field("jer_shift", &self.jer_shift())
                .field("shift_fail_en", &self.shift_fail_en())
                .field("vel_shift_fail_irq", &self.vel_shift_fail_irq())
                .field("acc_shift_fail_irq", &self.acc_shift_fail_irq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Shift {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Shift {{ vel_shift: {=u8:?}, acc_shift: {=u8:?}, jer_shift: {=u8:?}, shift_fail_en: {=bool:?}, vel_shift_fail_irq: {=bool:?}, acc_shift_fail_irq: {=bool:?} }}" , self . vel_shift () , self . acc_shift () , self . jer_shift () , self . shift_fail_en () , self . vel_shift_fail_irq () , self . acc_shift_fail_irq ())
        }
    }
    #[doc = "tra&index0_step_limit_ctrl."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StepLimitCtrl(pub u32);
    impl StepLimitCtrl {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_step_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_step_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_one_way_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_one_way_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_one_way_mode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_one_way_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_step_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pos_step_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_step_mode(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pos_step_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_one_way_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pos_one_way_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_one_way_mode(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pos_one_way_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_one_way_force_mode(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pos_one_way_force_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for StepLimitCtrl {
        #[inline(always)]
        fn default() -> StepLimitCtrl {
            StepLimitCtrl(0)
        }
    }
    impl core::fmt::Debug for StepLimitCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("StepLimitCtrl")
                .field("vel_step_en", &self.vel_step_en())
                .field("vel_one_way_en", &self.vel_one_way_en())
                .field("vel_one_way_mode", &self.vel_one_way_mode())
                .field("pos_step_en", &self.pos_step_en())
                .field("pos_step_mode", &self.pos_step_mode())
                .field("pos_one_way_en", &self.pos_one_way_en())
                .field("pos_one_way_mode", &self.pos_one_way_mode())
                .field("pos_one_way_force_mode", &self.pos_one_way_force_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for StepLimitCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "StepLimitCtrl {{ vel_step_en: {=bool:?}, vel_one_way_en: {=bool:?}, vel_one_way_mode: {=bool:?}, pos_step_en: {=bool:?}, pos_step_mode: {=bool:?}, pos_one_way_en: {=bool:?}, pos_one_way_mode: {=bool:?}, pos_one_way_force_mode: {=bool:?} }}" , self . vel_step_en () , self . vel_one_way_en () , self . vel_one_way_mode () , self . pos_step_en () , self . pos_step_mode () , self . pos_one_way_en () , self . pos_one_way_mode () , self . pos_one_way_force_mode ())
        }
    }
    #[doc = "sw_event."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwEvent(pub u32);
    impl SwEvent {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_event_trig(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sw_event_trig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SwEvent {
        #[inline(always)]
        fn default() -> SwEvent {
            SwEvent(0)
        }
    }
    impl core::fmt::Debug for SwEvent {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwEvent")
                .field("sw_event_trig", &self.sw_event_trig())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwEvent {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SwEvent {{ sw_event_trig: {=bool:?} }}",
                self.sw_event_trig()
            )
        }
    }
    #[doc = "sw_glb_reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwGlbReset(pub u32);
    impl SwGlbReset {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_glb_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sw_glb_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SwGlbReset {
        #[inline(always)]
        fn default() -> SwGlbReset {
            SwGlbReset(0)
        }
    }
    impl core::fmt::Debug for SwGlbReset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwGlbReset")
                .field("sw_glb_reset", &self.sw_glb_reset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwGlbReset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SwGlbReset {{ sw_glb_reset: {=bool:?} }}",
                self.sw_glb_reset()
            )
        }
    }
    #[doc = "tra&index0_control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TraControl(pub u32);
    impl TraControl {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ovalid_clear(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ovalid_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_lock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sw_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_irq(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_lock_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd_fail_irq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_cmd_fail_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn lock_irq_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_lock_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd_fail_irq_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_cmd_fail_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for TraControl {
        #[inline(always)]
        fn default() -> TraControl {
            TraControl(0)
        }
    }
    impl core::fmt::Debug for TraControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TraControl")
                .field("ovalid_clear", &self.ovalid_clear())
                .field("sw_lock", &self.sw_lock())
                .field("lock_irq", &self.lock_irq())
                .field("cmd_fail_irq", &self.cmd_fail_irq())
                .field("lock_irq_en", &self.lock_irq_en())
                .field("cmd_fail_irq_en", &self.cmd_fail_irq_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TraControl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TraControl {{ ovalid_clear: {=bool:?}, sw_lock: {=bool:?}, lock_irq: {=bool:?}, cmd_fail_irq: {=bool:?}, lock_irq_en: {=bool:?}, cmd_fail_irq_en: {=bool:?} }}" , self . ovalid_clear () , self . sw_lock () , self . lock_irq () , self . cmd_fail_irq () , self . lock_irq_en () , self . cmd_fail_irq_en ())
        }
    }
    #[doc = "tra&index0_vel_limit_n."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VelLimitN(pub u32);
    impl VelLimitN {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_limit_n(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_limit_n(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for VelLimitN {
        #[inline(always)]
        fn default() -> VelLimitN {
            VelLimitN(0)
        }
    }
    impl core::fmt::Debug for VelLimitN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VelLimitN")
                .field("vel_limit_n", &self.vel_limit_n())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VelLimitN {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "VelLimitN {{ vel_limit_n: {=u32:?} }}",
                self.vel_limit_n()
            )
        }
    }
    #[doc = "tra&index0_vel_limit_p."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VelLimitP(pub u32);
    impl VelLimitP {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_limit_p(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_limit_p(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for VelLimitP {
        #[inline(always)]
        fn default() -> VelLimitP {
            VelLimitP(0)
        }
    }
    impl core::fmt::Debug for VelLimitP {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VelLimitP")
                .field("vel_limit_p", &self.vel_limit_p())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VelLimitP {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "VelLimitP {{ vel_limit_p: {=u32:?} }}",
                self.vel_limit_p()
            )
        }
    }
    #[doc = "tra&index0_cmd&index1_vel_preset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VelPreset(pub u32);
    impl VelPreset {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_preset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_preset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for VelPreset {
        #[inline(always)]
        fn default() -> VelPreset {
            VelPreset(0)
        }
    }
    impl core::fmt::Debug for VelPreset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VelPreset")
                .field("vel_preset", &self.vel_preset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VelPreset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "VelPreset {{ vel_preset: {=u32:?} }}", self.vel_preset())
        }
    }
    #[doc = "tra&index0_vel_step_max."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VelStepMax(pub u32);
    impl VelStepMax {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_step_max(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_step_max(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for VelStepMax {
        #[inline(always)]
        fn default() -> VelStepMax {
            VelStepMax(0)
        }
    }
    impl core::fmt::Debug for VelStepMax {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VelStepMax")
                .field("vel_step_max", &self.vel_step_max())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VelStepMax {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "VelStepMax {{ vel_step_max: {=u32:?} }}",
                self.vel_step_max()
            )
        }
    }
    #[doc = "tra&index0_vel_step_min."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VelStepMin(pub u32);
    impl VelStepMin {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn vel_step_min(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_vel_step_min(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for VelStepMin {
        #[inline(always)]
        fn default() -> VelStepMin {
            VelStepMin(0)
        }
    }
    impl core::fmt::Debug for VelStepMin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VelStepMin")
                .field("vel_step_min", &self.vel_step_min())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VelStepMin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "VelStepMin {{ vel_step_min: {=u32:?} }}",
                self.vel_step_min()
            )
        }
    }
}
