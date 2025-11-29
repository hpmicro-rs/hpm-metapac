#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PCFG."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcfg {
    ptr: *mut u8,
}
unsafe impl Send for Pcfg {}
unsafe impl Sync for Pcfg {}
impl Pcfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "BANGGAP control."]
    #[inline(always)]
    pub const fn bandgap(self) -> crate::common::Reg<regs::Bandgap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "1V LDO config."]
    #[inline(always)]
    pub const fn ldo1p1(self) -> crate::common::Reg<regs::Ldo1p1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "2.5V LDO config."]
    #[inline(always)]
    pub const fn ldo2p5(self) -> crate::common::Reg<regs::Ldo2p5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DCDC mode select."]
    #[inline(always)]
    pub const fn dcdc_mode(self) -> crate::common::Reg<regs::DcdcMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DCDC low power mode."]
    #[inline(always)]
    pub const fn dcdc_lpmode(self) -> crate::common::Reg<regs::DcdcLpmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "DCDC protection."]
    #[inline(always)]
    pub const fn dcdc_prot(self) -> crate::common::Reg<regs::DcdcProt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "DCDC current estimation."]
    #[inline(always)]
    pub const fn dcdc_current(self) -> crate::common::Reg<regs::DcdcCurrent, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "DCDC advance setting."]
    #[inline(always)]
    pub const fn dcdc_advmode(self) -> crate::common::Reg<regs::DcdcAdvmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "DCDC advance parameter."]
    #[inline(always)]
    pub const fn dcdc_advparam(self) -> crate::common::Reg<regs::DcdcAdvparam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "DCDC misc parameter."]
    #[inline(always)]
    pub const fn dcdc_misc(self) -> crate::common::Reg<regs::DcdcMisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "DCDC Debug."]
    #[inline(always)]
    pub const fn dcdc_debug(self) -> crate::common::Reg<regs::DcdcDebug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "DCDC ramp time."]
    #[inline(always)]
    pub const fn dcdc_start_time(
        self,
    ) -> crate::common::Reg<regs::DcdcStartTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "DCDC resume time."]
    #[inline(always)]
    pub const fn dcdc_resume_time(
        self,
    ) -> crate::common::Reg<regs::DcdcResumeTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "SOC power trap."]
    #[inline(always)]
    pub const fn power_trap(self) -> crate::common::Reg<regs::PowerTrap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Wake up source."]
    #[inline(always)]
    pub const fn wake_cause(self) -> crate::common::Reg<regs::WakeCause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Wake up mask."]
    #[inline(always)]
    pub const fn wake_mask(self) -> crate::common::Reg<regs::WakeMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Clock gate control in PMIC."]
    #[inline(always)]
    pub const fn scg_ctrl(self) -> crate::common::Reg<regs::ScgCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "RC 24M config."]
    #[inline(always)]
    pub const fn rc24m(self) -> crate::common::Reg<regs::Rc24m, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "RC 24M track mode."]
    #[inline(always)]
    pub const fn rc24m_track(self) -> crate::common::Reg<regs::Rc24mTrack, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "RC 24M track target."]
    #[inline(always)]
    pub const fn track_target(self) -> crate::common::Reg<regs::TrackTarget, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "RC 24M track status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
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
    #[doc = "BANGGAP control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bandgap(pub u32);
    impl Bandgap {
        #[doc = "Banggap 1.0V output trim value."]
        #[must_use]
        #[inline(always)]
        pub const fn vbg_p50_trim(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Banggap 1.0V output trim value."]
        #[inline(always)]
        pub const fn set_vbg_p50_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Banggap 1.0V output trim value."]
        #[must_use]
        #[inline(always)]
        pub const fn vbg_p65_trim(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Banggap 1.0V output trim value."]
        #[inline(always)]
        pub const fn set_vbg_p65_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Banggap 1.0V output trim value."]
        #[must_use]
        #[inline(always)]
        pub const fn vbg_1p0_trim(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Banggap 1.0V output trim value."]
        #[inline(always)]
        pub const fn set_vbg_1p0_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed."]
        #[must_use]
        #[inline(always)]
        pub const fn vbg_trimmed(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed."]
        #[inline(always)]
        pub const fn set_vbg_trimmed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Bandgap {
        #[inline(always)]
        fn default() -> Bandgap {
            Bandgap(0)
        }
    }
    impl core::fmt::Debug for Bandgap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bandgap")
                .field("vbg_p50_trim", &self.vbg_p50_trim())
                .field("vbg_p65_trim", &self.vbg_p65_trim())
                .field("vbg_1p0_trim", &self.vbg_1p0_trim())
                .field("vbg_trimmed", &self.vbg_trimmed())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bandgap {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bandgap {{ vbg_p50_trim: {=u8:?}, vbg_p65_trim: {=u8:?}, vbg_1p0_trim: {=u8:?}, vbg_trimmed: {=bool:?} }}" , self . vbg_p50_trim () , self . vbg_p65_trim () , self . vbg_1p0_trim () , self . vbg_trimmed ())
        }
    }
    #[doc = "DCDC advance setting."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcdcAdvmode(pub u32);
    impl DcdcAdvmode {
        #[doc = "DCM mode 0: CCM mode 1: DCM mode."]
        #[must_use]
        #[inline(always)]
        pub const fn en_dcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCM mode 0: CCM mode 1: DCM mode."]
        #[inline(always)]
        pub const fn set_en_dcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "enable skip when voltage is higher than threshold 0: do not skip 1: skip if voltage is excess."]
        #[must_use]
        #[inline(always)]
        pub const fn en_idle(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable skip when voltage is higher than threshold 0: do not skip 1: skip if voltage is excess."]
        #[inline(always)]
        pub const fn set_en_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "enable skip on narrow pulse 0: do not skip narrow pulse 1: skip narrow pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn en_skip(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable skip on narrow pulse 0: do not skip narrow pulse 1: skip narrow pulse."]
        #[inline(always)]
        pub const fn set_en_skip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "avoid over voltage 0: stay in DCM mode when voltage excess 1: change to CCM mode when voltage excess."]
        #[must_use]
        #[inline(always)]
        pub const fn en_dcm_exit(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "avoid over voltage 0: stay in DCM mode when voltage excess 1: change to CCM mode when voltage excess."]
        #[inline(always)]
        pub const fn set_en_dcm_exit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "enable auto enter low power mode 0: do not enter low power mode 1: enter low power mode if current is detected low."]
        #[must_use]
        #[inline(always)]
        pub const fn en_autolp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable auto enter low power mode 0: do not enter low power mode 1: enter low power mode if current is detected low."]
        #[inline(always)]
        pub const fn set_en_autolp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "enable feed forward loop 0: feed forward loop is disabled 1: feed forward loop is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn en_ff_loop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable feed forward loop 0: feed forward loop is disabled 1: feed forward loop is enabled."]
        #[inline(always)]
        pub const fn set_en_ff_loop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "enable feed forward detect 0: feed forward detect is disabled 1: feed forward detect is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn en_ff_det(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable feed forward detect 0: feed forward detect is disabled 1: feed forward detect is enabled."]
        #[inline(always)]
        pub const fn set_en_ff_det(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Loop R number."]
        #[must_use]
        #[inline(always)]
        pub const fn dc_r(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Loop R number."]
        #[inline(always)]
        pub const fn set_dc_r(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Loop C number."]
        #[must_use]
        #[inline(always)]
        pub const fn dc_c(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Loop C number."]
        #[inline(always)]
        pub const fn set_dc_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Enable RC scale."]
        #[must_use]
        #[inline(always)]
        pub const fn en_rcscale(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Enable RC scale."]
        #[inline(always)]
        pub const fn set_en_rcscale(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for DcdcAdvmode {
        #[inline(always)]
        fn default() -> DcdcAdvmode {
            DcdcAdvmode(0)
        }
    }
    impl core::fmt::Debug for DcdcAdvmode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DcdcAdvmode")
                .field("en_dcm", &self.en_dcm())
                .field("en_idle", &self.en_idle())
                .field("en_skip", &self.en_skip())
                .field("en_dcm_exit", &self.en_dcm_exit())
                .field("en_autolp", &self.en_autolp())
                .field("en_ff_loop", &self.en_ff_loop())
                .field("en_ff_det", &self.en_ff_det())
                .field("dc_r", &self.dc_r())
                .field("dc_c", &self.dc_c())
                .field("en_rcscale", &self.en_rcscale())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcdcAdvmode {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DcdcAdvmode {{ en_dcm: {=bool:?}, en_idle: {=bool:?}, en_skip: {=bool:?}, en_dcm_exit: {=bool:?}, en_autolp: {=bool:?}, en_ff_loop: {=bool:?}, en_ff_det: {=bool:?}, dc_r: {=u8:?}, dc_c: {=u8:?}, en_rcscale: {=u8:?} }}" , self . en_dcm () , self . en_idle () , self . en_skip () , self . en_dcm_exit () , self . en_autolp () , self . en_ff_loop () , self . en_ff_det () , self . dc_r () , self . dc_c () , self . en_rcscale ())
        }
    }
    #[doc = "DCDC advance parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcdcAdvparam(pub u32);
    impl DcdcAdvparam {
        #[doc = "maximum duty cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn max_dut(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "maximum duty cycle."]
        #[inline(always)]
        pub const fn set_max_dut(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "minimum duty cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn min_dut(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "minimum duty cycle."]
        #[inline(always)]
        pub const fn set_min_dut(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
    }
    impl Default for DcdcAdvparam {
        #[inline(always)]
        fn default() -> DcdcAdvparam {
            DcdcAdvparam(0)
        }
    }
    impl core::fmt::Debug for DcdcAdvparam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DcdcAdvparam")
                .field("max_dut", &self.max_dut())
                .field("min_dut", &self.min_dut())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcdcAdvparam {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DcdcAdvparam {{ max_dut: {=u8:?}, min_dut: {=u8:?} }}",
                self.max_dut(),
                self.min_dut()
            )
        }
    }
    #[doc = "DCDC current estimation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcdcCurrent(pub u32);
    impl DcdcCurrent {
        #[doc = "DCDC current level, current level is num * 50mA."]
        #[must_use]
        #[inline(always)]
        pub const fn level(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "DCDC current level, current level is num * 50mA."]
        #[inline(always)]
        pub const fn set_level(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Current level valid 0: data is invalid 1: data is valid."]
        #[must_use]
        #[inline(always)]
        pub const fn valid(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Current level valid 0: data is invalid 1: data is valid."]
        #[inline(always)]
        pub const fn set_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "enable current measure."]
        #[must_use]
        #[inline(always)]
        pub const fn esti_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "enable current measure."]
        #[inline(always)]
        pub const fn set_esti_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for DcdcCurrent {
        #[inline(always)]
        fn default() -> DcdcCurrent {
            DcdcCurrent(0)
        }
    }
    impl core::fmt::Debug for DcdcCurrent {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DcdcCurrent")
                .field("level", &self.level())
                .field("valid", &self.valid())
                .field("esti_en", &self.esti_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcdcCurrent {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DcdcCurrent {{ level: {=u8:?}, valid: {=bool:?}, esti_en: {=bool:?} }}",
                self.level(),
                self.valid(),
                self.esti_en()
            )
        }
    }
    #[doc = "DCDC Debug."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcdcDebug(pub u32);
    impl DcdcDebug {
        #[doc = "DCDC voltage change time in 24M clock cycles, default value is 1mS."]
        #[must_use]
        #[inline(always)]
        pub const fn update_time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "DCDC voltage change time in 24M clock cycles, default value is 1mS."]
        #[inline(always)]
        pub const fn set_update_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for DcdcDebug {
        #[inline(always)]
        fn default() -> DcdcDebug {
            DcdcDebug(0)
        }
    }
    impl core::fmt::Debug for DcdcDebug {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DcdcDebug")
                .field("update_time", &self.update_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcdcDebug {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DcdcDebug {{ update_time: {=u32:?} }}",
                self.update_time()
            )
        }
    }
    #[doc = "DCDC low power mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcdcLpmode(pub u32);
    impl DcdcLpmode {
        #[doc = "DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV."]
        #[must_use]
        #[inline(always)]
        pub const fn stby_volt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV."]
        #[inline(always)]
        pub const fn set_stby_volt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for DcdcLpmode {
        #[inline(always)]
        fn default() -> DcdcLpmode {
            DcdcLpmode(0)
        }
    }
    impl core::fmt::Debug for DcdcLpmode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DcdcLpmode")
                .field("stby_volt", &self.stby_volt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcdcLpmode {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DcdcLpmode {{ stby_volt: {=u16:?} }}", self.stby_volt())
        }
    }
    #[doc = "DCDC misc parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcdcMisc(pub u32);
    impl DcdcMisc {
        #[doc = "enable stepping in voltage change 0: stepping disabled 1: steping enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn en_step(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable stepping in voltage change 0: stepping disabled 1: steping enabled."]
        #[inline(always)]
        pub const fn set_en_step(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator."]
        #[inline(always)]
        pub const fn set_clk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "enable delay 0: delay disabled, 1: delay enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn delay(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable delay 0: delay disabled, 1: delay enabled."]
        #[inline(always)]
        pub const fn set_delay(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "current hysteres range 0: 12.5mV 1: 25mV."]
        #[must_use]
        #[inline(always)]
        pub const fn ol_hyst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "current hysteres range 0: 12.5mV 1: 25mV."]
        #[inline(always)]
        pub const fn set_ol_hyst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "overload for threshold for lod power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ol_thre(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "overload for threshold for lod power mode."]
        #[inline(always)]
        pub const fn set_ol_thre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Loop feed forward number."]
        #[must_use]
        #[inline(always)]
        pub const fn dc_ff(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Loop feed forward number."]
        #[inline(always)]
        pub const fn set_dc_ff(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Loop RC scale threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn rc_scale(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Loop RC scale threshold."]
        #[inline(always)]
        pub const fn set_rc_scale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "hysteres threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn hyst_thrs(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "hysteres threshold."]
        #[inline(always)]
        pub const fn set_hyst_thrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "hysteres sign."]
        #[must_use]
        #[inline(always)]
        pub const fn hyst_sign(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "hysteres sign."]
        #[inline(always)]
        pub const fn set_hyst_sign(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "hysteres enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en_hyst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "hysteres enable."]
        #[inline(always)]
        pub const fn set_en_hyst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for DcdcMisc {
        #[inline(always)]
        fn default() -> DcdcMisc {
            DcdcMisc(0)
        }
    }
    impl core::fmt::Debug for DcdcMisc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DcdcMisc")
                .field("en_step", &self.en_step())
                .field("clk_sel", &self.clk_sel())
                .field("delay", &self.delay())
                .field("ol_hyst", &self.ol_hyst())
                .field("ol_thre", &self.ol_thre())
                .field("dc_ff", &self.dc_ff())
                .field("rc_scale", &self.rc_scale())
                .field("hyst_thrs", &self.hyst_thrs())
                .field("hyst_sign", &self.hyst_sign())
                .field("en_hyst", &self.en_hyst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcdcMisc {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DcdcMisc {{ en_step: {=bool:?}, clk_sel: {=bool:?}, delay: {=bool:?}, ol_hyst: {=bool:?}, ol_thre: {=u8:?}, dc_ff: {=u8:?}, rc_scale: {=bool:?}, hyst_thrs: {=bool:?}, hyst_sign: {=bool:?}, en_hyst: {=bool:?} }}" , self . en_step () , self . clk_sel () , self . delay () , self . ol_hyst () , self . ol_thre () , self . dc_ff () , self . rc_scale () , self . hyst_thrs () , self . hyst_sign () , self . en_hyst ())
        }
    }
    #[doc = "DCDC mode select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcdcMode(pub u32);
    impl DcdcMode {
        #[doc = "DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV."]
        #[must_use]
        #[inline(always)]
        pub const fn volt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV."]
        #[inline(always)]
        pub const fn set_volt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Ready flag 0: DCDC is applying new change 1: DCDC is ready."]
        #[must_use]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Ready flag 0: DCDC is applying new change 1: DCDC is ready."]
        #[inline(always)]
        pub const fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for DcdcMode {
        #[inline(always)]
        fn default() -> DcdcMode {
            DcdcMode(0)
        }
    }
    impl core::fmt::Debug for DcdcMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DcdcMode")
                .field("volt", &self.volt())
                .field("mode", &self.mode())
                .field("ready", &self.ready())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcdcMode {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DcdcMode {{ volt: {=u16:?}, mode: {=u8:?}, ready: {=bool:?} }}",
                self.volt(),
                self.mode(),
                self.ready()
            )
        }
    }
    #[doc = "DCDC protection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcdcProt(pub u32);
    impl DcdcProt {
        #[doc = "short circuit flag 0: current is within limit 1: short circuits detected."]
        #[must_use]
        #[inline(always)]
        pub const fn short_flag(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "short circuit flag 0: current is within limit 1: short circuits detected."]
        #[inline(always)]
        pub const fn set_short_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "short circuit current setting 0: 2.0A, 1: 1.3A."]
        #[must_use]
        #[inline(always)]
        pub const fn short_current(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "short circuit current setting 0: 2.0A, 1: 1.3A."]
        #[inline(always)]
        pub const fn set_short_current(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "disable output short circuit protection 0: short circuits protection enabled, DCDC shut down if short circuit on ouput detected 1: short circuit protection disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_short(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "disable output short circuit protection 0: short circuits protection enabled, DCDC shut down if short circuit on ouput detected 1: short circuit protection disabled."]
        #[inline(always)]
        pub const fn set_disable_short(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "output over voltage flag 0: output is normal 1: output is unexpected high."]
        #[must_use]
        #[inline(always)]
        pub const fn overvolt_flag(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "output over voltage flag 0: output is normal 1: output is unexpected high."]
        #[inline(always)]
        pub const fn set_overvolt_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ouput over voltage protection 0: protection enabled, DCDC will shut down is output voltage is unexpected high 1: protection disabled, DCDC continue to adjust output voltage."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_overvoltage(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ouput over voltage protection 0: protection enabled, DCDC will shut down is output voltage is unexpected high 1: protection disabled, DCDC continue to adjust output voltage."]
        #[inline(always)]
        pub const fn set_disable_overvoltage(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "power loss 0: input power is good 1: input power is too low."]
        #[must_use]
        #[inline(always)]
        pub const fn power_loss_flag(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "power loss 0: input power is good 1: input power is too low."]
        #[inline(always)]
        pub const fn set_power_loss_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "over current in low power mode 0: current is below setting 1: overcurrent happened in low power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn overload_lp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "over current in low power mode 0: current is below setting 1: overcurrent happened in low power mode."]
        #[inline(always)]
        pub const fn set_overload_lp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "over current setting for low power mode 0:250mA 1:200mA."]
        #[must_use]
        #[inline(always)]
        pub const fn ilimit_lp(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "over current setting for low power mode 0:250mA 1:200mA."]
        #[inline(always)]
        pub const fn set_ilimit_lp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for DcdcProt {
        #[inline(always)]
        fn default() -> DcdcProt {
            DcdcProt(0)
        }
    }
    impl core::fmt::Debug for DcdcProt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DcdcProt")
                .field("short_flag", &self.short_flag())
                .field("short_current", &self.short_current())
                .field("disable_short", &self.disable_short())
                .field("overvolt_flag", &self.overvolt_flag())
                .field("disable_overvoltage", &self.disable_overvoltage())
                .field("power_loss_flag", &self.power_loss_flag())
                .field("overload_lp", &self.overload_lp())
                .field("ilimit_lp", &self.ilimit_lp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcdcProt {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DcdcProt {{ short_flag: {=bool:?}, short_current: {=bool:?}, disable_short: {=bool:?}, overvolt_flag: {=bool:?}, disable_overvoltage: {=bool:?}, power_loss_flag: {=bool:?}, overload_lp: {=bool:?}, ilimit_lp: {=bool:?} }}" , self . short_flag () , self . short_current () , self . disable_short () , self . overvolt_flag () , self . disable_overvoltage () , self . power_loss_flag () , self . overload_lp () , self . ilimit_lp ())
        }
    }
    #[doc = "DCDC resume time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcdcResumeTime(pub u32);
    impl DcdcResumeTime {
        #[doc = "Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS."]
        #[must_use]
        #[inline(always)]
        pub const fn resume_time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS."]
        #[inline(always)]
        pub const fn set_resume_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for DcdcResumeTime {
        #[inline(always)]
        fn default() -> DcdcResumeTime {
            DcdcResumeTime(0)
        }
    }
    impl core::fmt::Debug for DcdcResumeTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DcdcResumeTime")
                .field("resume_time", &self.resume_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcdcResumeTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DcdcResumeTime {{ resume_time: {=u32:?} }}",
                self.resume_time()
            )
        }
    }
    #[doc = "DCDC ramp time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DcdcStartTime(pub u32);
    impl DcdcStartTime {
        #[doc = "Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS."]
        #[must_use]
        #[inline(always)]
        pub const fn start_time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS."]
        #[inline(always)]
        pub const fn set_start_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for DcdcStartTime {
        #[inline(always)]
        fn default() -> DcdcStartTime {
            DcdcStartTime(0)
        }
    }
    impl core::fmt::Debug for DcdcStartTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DcdcStartTime")
                .field("start_time", &self.start_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DcdcStartTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DcdcStartTime {{ start_time: {=u32:?} }}",
                self.start_time()
            )
        }
    }
    #[doc = "1V LDO config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ldo1p1(pub u32);
    impl Ldo1p1 {
        #[doc = "LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV."]
        #[must_use]
        #[inline(always)]
        pub const fn volt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV."]
        #[inline(always)]
        pub const fn set_volt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Ldo1p1 {
        #[inline(always)]
        fn default() -> Ldo1p1 {
            Ldo1p1(0)
        }
    }
    impl core::fmt::Debug for Ldo1p1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ldo1p1")
                .field("volt", &self.volt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ldo1p1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ldo1p1 {{ volt: {=u16:?} }}", self.volt())
        }
    }
    #[doc = "2.5V LDO config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ldo2p5(pub u32);
    impl Ldo2p5 {
        #[doc = "LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV."]
        #[must_use]
        #[inline(always)]
        pub const fn volt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV."]
        #[inline(always)]
        pub const fn set_volt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "LDO enable 0: turn off LDO 1: turn on LDO."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LDO enable 0: turn off LDO 1: turn on LDO."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Ready flag, will set 1ms after enabled or voltage change 0: LDO is not ready for use 1: LDO is ready."]
        #[must_use]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Ready flag, will set 1ms after enabled or voltage change 0: LDO is not ready for use 1: LDO is ready."]
        #[inline(always)]
        pub const fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ldo2p5 {
        #[inline(always)]
        fn default() -> Ldo2p5 {
            Ldo2p5(0)
        }
    }
    impl core::fmt::Debug for Ldo2p5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ldo2p5")
                .field("volt", &self.volt())
                .field("enable", &self.enable())
                .field("ready", &self.ready())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ldo2p5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ldo2p5 {{ volt: {=u16:?}, enable: {=bool:?}, ready: {=bool:?} }}",
                self.volt(),
                self.enable(),
                self.ready()
            )
        }
    }
    #[doc = "SOC power trap."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PowerTrap(pub u32);
    impl PowerTrap {
        #[doc = "Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned."]
        #[must_use]
        #[inline(always)]
        pub const fn trap(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned."]
        #[inline(always)]
        pub const fn set_trap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage."]
        #[must_use]
        #[inline(always)]
        pub const fn retention(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage."]
        #[inline(always)]
        pub const fn set_retention(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered."]
        #[must_use]
        #[inline(always)]
        pub const fn triggered(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered."]
        #[inline(always)]
        pub const fn set_triggered(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PowerTrap {
        #[inline(always)]
        fn default() -> PowerTrap {
            PowerTrap(0)
        }
    }
    impl core::fmt::Debug for PowerTrap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PowerTrap")
                .field("trap", &self.trap())
                .field("retention", &self.retention())
                .field("triggered", &self.triggered())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PowerTrap {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PowerTrap {{ trap: {=bool:?}, retention: {=bool:?}, triggered: {=bool:?} }}",
                self.trap(),
                self.retention(),
                self.triggered()
            )
        }
    }
    #[doc = "RC 24M config."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rc24m(pub u32);
    impl Rc24m {
        #[doc = "Fine trim for RC24M, bigger value means faster."]
        #[must_use]
        #[inline(always)]
        pub const fn trim_f(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Fine trim for RC24M, bigger value means faster."]
        #[inline(always)]
        pub const fn set_trim_f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Coarse trim for RC24M, bigger value means faster."]
        #[must_use]
        #[inline(always)]
        pub const fn trim_c(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Coarse trim for RC24M, bigger value means faster."]
        #[inline(always)]
        pub const fn set_trim_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed."]
        #[must_use]
        #[inline(always)]
        pub const fn rc_trimmed(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed."]
        #[inline(always)]
        pub const fn set_rc_trimmed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rc24m {
        #[inline(always)]
        fn default() -> Rc24m {
            Rc24m(0)
        }
    }
    impl core::fmt::Debug for Rc24m {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rc24m")
                .field("trim_f", &self.trim_f())
                .field("trim_c", &self.trim_c())
                .field("rc_trimmed", &self.rc_trimmed())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rc24m {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rc24m {{ trim_f: {=u8:?}, trim_c: {=u8:?}, rc_trimmed: {=bool:?} }}",
                self.trim_f(),
                self.trim_c(),
                self.rc_trimmed()
            )
        }
    }
    #[doc = "RC 24M track mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rc24mTrack(pub u32);
    impl Rc24mTrack {
        #[doc = "track mode 0: RC24M free running 1: track RC24M to external XTAL."]
        #[must_use]
        #[inline(always)]
        pub const fn track(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "track mode 0: RC24M free running 1: track RC24M to external XTAL."]
        #[inline(always)]
        pub const fn set_track(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value."]
        #[must_use]
        #[inline(always)]
        pub const fn return_(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value."]
        #[inline(always)]
        pub const fn set_return_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Select track reference 0: select 32K as reference 1: select 24M XTAL as reference."]
        #[must_use]
        #[inline(always)]
        pub const fn sel24m(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Select track reference 0: select 32K as reference 1: select 24M XTAL as reference."]
        #[inline(always)]
        pub const fn set_sel24m(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Rc24mTrack {
        #[inline(always)]
        fn default() -> Rc24mTrack {
            Rc24mTrack(0)
        }
    }
    impl core::fmt::Debug for Rc24mTrack {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rc24mTrack")
                .field("track", &self.track())
                .field("return_", &self.return_())
                .field("sel24m", &self.sel24m())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rc24mTrack {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rc24mTrack {{ track: {=bool:?}, return_: {=bool:?}, sel24m: {=bool:?} }}",
                self.track(),
                self.return_(),
                self.sel24m()
            )
        }
    }
    #[doc = "Clock gate control in PMIC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScgCtrl(pub u32);
    impl ScgCtrl {
        #[doc = "control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: reserved 10: clock is always off 11: clock is always on bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart."]
        #[must_use]
        #[inline(always)]
        pub const fn scg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: reserved 10: clock is always off 11: clock is always on bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart."]
        #[inline(always)]
        pub const fn set_scg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ScgCtrl {
        #[inline(always)]
        fn default() -> ScgCtrl {
            ScgCtrl(0)
        }
    }
    impl core::fmt::Debug for ScgCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ScgCtrl").field("scg", &self.scg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ScgCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ScgCtrl {{ scg: {=u32:?} }}", self.scg())
        }
    }
    #[doc = "RC 24M track status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "default fine trim value."]
        #[must_use]
        #[inline(always)]
        pub const fn trim_f(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "default fine trim value."]
        #[inline(always)]
        pub const fn set_trim_f(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "default coarse trim value."]
        #[must_use]
        #[inline(always)]
        pub const fn trim_c(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "default coarse trim value."]
        #[inline(always)]
        pub const fn set_trim_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "default value takes effect 0: default value is invalid 1: default value is valid."]
        #[must_use]
        #[inline(always)]
        pub const fn en_trim(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "default value takes effect 0: default value is invalid 1: default value is valid."]
        #[inline(always)]
        pub const fn set_en_trim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "track is using XTAL24M 0: track is not using XTAL24M 1: track is using XTAL24M."]
        #[must_use]
        #[inline(always)]
        pub const fn sel24m(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "track is using XTAL24M 0: track is not using XTAL24M 1: track is using XTAL24M."]
        #[inline(always)]
        pub const fn set_sel24m(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "track is using XTAL32K 0: track is not using XTAL32K 1: track is using XTAL32K."]
        #[must_use]
        #[inline(always)]
        pub const fn sel32k(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "track is using XTAL32K 0: track is not using XTAL32K 1: track is using XTAL32K."]
        #[inline(always)]
        pub const fn set_sel32k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    impl core::fmt::Debug for Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Status")
                .field("trim_f", &self.trim_f())
                .field("trim_c", &self.trim_c())
                .field("en_trim", &self.en_trim())
                .field("sel24m", &self.sel24m())
                .field("sel32k", &self.sel32k())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Status {{ trim_f: {=u8:?}, trim_c: {=u8:?}, en_trim: {=bool:?}, sel24m: {=bool:?}, sel32k: {=bool:?} }}" , self . trim_f () , self . trim_c () , self . en_trim () , self . sel24m () , self . sel32k ())
        }
    }
    #[doc = "RC 24M track target."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrackTarget(pub u32);
    impl TrackTarget {
        #[doc = "Target frequency multiplier of divided source."]
        #[must_use]
        #[inline(always)]
        pub const fn target(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Target frequency multiplier of divided source."]
        #[inline(always)]
        pub const fn set_target(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Divider for reference source."]
        #[must_use]
        #[inline(always)]
        pub const fn pre_div(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Divider for reference source."]
        #[inline(always)]
        pub const fn set_pre_div(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for TrackTarget {
        #[inline(always)]
        fn default() -> TrackTarget {
            TrackTarget(0)
        }
    }
    impl core::fmt::Debug for TrackTarget {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrackTarget")
                .field("target", &self.target())
                .field("pre_div", &self.pre_div())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrackTarget {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TrackTarget {{ target: {=u16:?}, pre_div: {=u16:?} }}",
                self.target(),
                self.pre_div()
            )
        }
    }
    #[doc = "Wake up source."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WakeCause(pub u32);
    impl WakeCause {
        #[doc = "wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit31: pin wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn cause(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit31: pin wakeup."]
        #[inline(always)]
        pub const fn set_cause(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WakeCause {
        #[inline(always)]
        fn default() -> WakeCause {
            WakeCause(0)
        }
    }
    impl core::fmt::Debug for WakeCause {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WakeCause")
                .field("cause", &self.cause())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WakeCause {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WakeCause {{ cause: {=u32:?} }}", self.cause())
        }
    }
    #[doc = "Wake up mask."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WakeMask(pub u32);
    impl WakeMask {
        #[doc = "mask for wake up sources, each bit represents one wakeup source 0: allow source to wake up system 1: disallow source to wakeup system bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit31: pin wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn mask(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "mask for wake up sources, each bit represents one wakeup source 0: allow source to wake up system 1: disallow source to wakeup system bit 0: pmic_enable bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit31: pin wakeup."]
        #[inline(always)]
        pub const fn set_mask(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WakeMask {
        #[inline(always)]
        fn default() -> WakeMask {
            WakeMask(0)
        }
    }
    impl core::fmt::Debug for WakeMask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WakeMask")
                .field("mask", &self.mask())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WakeMask {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WakeMask {{ mask: {=u32:?} }}", self.mask())
        }
    }
}
