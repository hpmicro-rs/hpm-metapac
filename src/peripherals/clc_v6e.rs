#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "CLC0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc {
    ptr: *mut u8,
}
unsafe impl Send for Clc {}
unsafe impl Sync for Clc {}
impl Clc {
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
    pub const fn vdvq_chan(self, n: usize) -> VdvqChan {
        assert!(n < 2usize);
        unsafe { VdvqChan::from_ptr(self.ptr.add(0x0usize + n * 256usize) as _) }
    }
    #[doc = "enable d/q chan software inject adc value."]
    #[inline(always)]
    pub const fn dq_adc_sw_ready(
        self,
    ) -> crate::common::Reg<regs::DqAdcSwReady, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Coeff {
    ptr: *mut u8,
}
unsafe impl Send for Coeff {}
unsafe impl Sync for Coeff {}
impl Coeff {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "&index0 zone &index1 b0."]
    #[inline(always)]
    pub const fn coeff_b0(self) -> crate::common::Reg<regs::CoeffB0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "&index0 zone &index1 b1."]
    #[inline(always)]
    pub const fn coeff_b1(self) -> crate::common::Reg<regs::CoeffB1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "&index0 zone &index1 b2."]
    #[inline(always)]
    pub const fn coeff_b2(self) -> crate::common::Reg<regs::CoeffB2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "&index0 zone &index1 b3."]
    #[inline(always)]
    pub const fn coeff_b3(self) -> crate::common::Reg<regs::CoeffB3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "&index0 zone &index1 a0."]
    #[inline(always)]
    pub const fn coeff_a0(self) -> crate::common::Reg<regs::CoeffA0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "&index0 zone &index1 a1."]
    #[inline(always)]
    pub const fn coeff_a1(self) -> crate::common::Reg<regs::CoeffA1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "&index0 zone &index1 a2."]
    #[inline(always)]
    pub const fn coeff_a2(self) -> crate::common::Reg<regs::CoeffA2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "&index0 zone &index1 kscaling."]
    #[inline(always)]
    pub const fn coeff_ks(self) -> crate::common::Reg<regs::CoeffKs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VdvqChan {
    ptr: *mut u8,
}
unsafe impl Send for VdvqChan {}
unsafe impl Sync for VdvqChan {}
impl VdvqChan {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "&index0 mode ctrl."]
    #[inline(always)]
    pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "&index0 adc expect."]
    #[inline(always)]
    pub const fn adc_expect(self) -> crate::common::Reg<regs::AdcExpect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "&index0 adc used channel."]
    #[inline(always)]
    pub const fn adc_chan(self) -> crate::common::Reg<regs::AdcChan, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "&index0 adc used offset."]
    #[inline(always)]
    pub const fn adc_offset(self) -> crate::common::Reg<regs::AdcOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "&index0 eadc_lowth value used in error adc cofficient selection."]
    #[inline(always)]
    pub const fn eadc_lowth(self) -> crate::common::Reg<regs::EadcLowth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "&index0 eadc_highth value used in error adc cofficient selection."]
    #[inline(always)]
    pub const fn eadc_highth(self) -> crate::common::Reg<regs::EadcHighth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "&index0 eadc_midlowth value used in error adc cofficient selection."]
    #[inline(always)]
    pub const fn eadc_midlowth(self) -> crate::common::Reg<regs::EadcMidlowth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "&index0 eadc_midhighth value used in error adc cofficient selection."]
    #[inline(always)]
    pub const fn eadc_midhighth(
        self,
    ) -> crate::common::Reg<regs::EadcMidhighth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "&index0 2p2z output clamp low threshold."]
    #[inline(always)]
    pub const fn p2z2_clamp_lo(self) -> crate::common::Reg<regs::P2z2ClampLo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "&index0 2p2z output clamp high threshold."]
    #[inline(always)]
    pub const fn p2z2_clamp_hi(self) -> crate::common::Reg<regs::P2z2ClampHi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "&index0 3p3z output clamp low threshold."]
    #[inline(always)]
    pub const fn p3z3_clamp_lo(self) -> crate::common::Reg<regs::P3z3ClampLo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "&index0 3p3z output clamp high threshold."]
    #[inline(always)]
    pub const fn p3z3_clamp_hi(self) -> crate::common::Reg<regs::P3z3ClampHi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn coeff(self, n: usize) -> Coeff {
        assert!(n < 3usize);
        unsafe { Coeff::from_ptr(self.ptr.add(0x40usize + n * 32usize) as _) }
    }
    #[doc = "&index0 pwm_period."]
    #[inline(always)]
    pub const fn pwm_period(self) -> crate::common::Reg<regs::PwmPeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "&index0 output value."]
    #[inline(always)]
    pub const fn output_value(self) -> crate::common::Reg<regs::OutputValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "&index0 adc timestamp used."]
    #[inline(always)]
    pub const fn timestamp(self) -> crate::common::Reg<regs::Timestamp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "&index0 error adc latest value."]
    #[inline(always)]
    pub const fn eadc_curr(self) -> crate::common::Reg<regs::EadcCurr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "&index0 error adc previous0 value."]
    #[inline(always)]
    pub const fn eadc_pre0(self) -> crate::common::Reg<regs::EadcPre0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "&index0 error adc previous1 value."]
    #[inline(always)]
    pub const fn eadc_pre1(self) -> crate::common::Reg<regs::EadcPre1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "&index0 2p2z latest value."]
    #[inline(always)]
    pub const fn p2z2_curr(self) -> crate::common::Reg<regs::P2z2Curr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "&index0 2p2z previous0 value."]
    #[inline(always)]
    pub const fn p2z2_pre0(self) -> crate::common::Reg<regs::P2z2Pre0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "&index0 3p3z latest value."]
    #[inline(always)]
    pub const fn p3z3_curr(self) -> crate::common::Reg<regs::P3z3Curr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "&index0 3p3z output forbid low threshold."]
    #[inline(always)]
    pub const fn p3z3_forbid_lo(self) -> crate::common::Reg<regs::P3z3ForbidLo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "&index0 3p3z output forbid middle threshold."]
    #[inline(always)]
    pub const fn p3z3_forbid_md(self) -> crate::common::Reg<regs::P3z3ForbidMd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "&index0 3p3z output forbid high threshold."]
    #[inline(always)]
    pub const fn p3z3_forbid_hi(self) -> crate::common::Reg<regs::P3z3ForbidHi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "&index0 adc software inject value."]
    #[inline(always)]
    pub const fn adc_sw(self) -> crate::common::Reg<regs::AdcSw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "&index0 irq_status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
}
pub mod regs {
    #[doc = "&index0 adc used channel."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcChan(pub u32);
    impl AdcChan {
        #[doc = "adc used chan ID."]
        #[inline(always)]
        pub const fn adc_chan(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "adc used chan ID."]
        #[inline(always)]
        pub fn set_adc_chan(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for AdcChan {
        #[inline(always)]
        fn default() -> AdcChan {
            AdcChan(0)
        }
    }
    #[doc = "&index0 adc expect."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcExpect(pub u32);
    impl AdcExpect {
        #[doc = "adc expect value."]
        #[inline(always)]
        pub const fn adc_expect(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "adc expect value."]
        #[inline(always)]
        pub fn set_adc_expect(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AdcExpect {
        #[inline(always)]
        fn default() -> AdcExpect {
            AdcExpect(0)
        }
    }
    #[doc = "&index0 adc used offset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcOffset(pub u32);
    impl AdcOffset {
        #[doc = "adc used offset."]
        #[inline(always)]
        pub const fn adc_offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "adc used offset."]
        #[inline(always)]
        pub fn set_adc_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AdcOffset {
        #[inline(always)]
        fn default() -> AdcOffset {
            AdcOffset(0)
        }
    }
    #[doc = "&index0 adc software inject value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcSw(pub u32);
    impl AdcSw {
        #[doc = "adc software inject value."]
        #[inline(always)]
        pub const fn adc_sw(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "adc software inject value."]
        #[inline(always)]
        pub fn set_adc_sw(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AdcSw {
        #[inline(always)]
        fn default() -> AdcSw {
            AdcSw(0)
        }
    }
    #[doc = "&index0 zone &index1 a0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CoeffA0(pub u32);
    impl CoeffA0 {
        #[doc = "coefficient a0."]
        #[inline(always)]
        pub const fn coeff_a0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "coefficient a0."]
        #[inline(always)]
        pub fn set_coeff_a0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CoeffA0 {
        #[inline(always)]
        fn default() -> CoeffA0 {
            CoeffA0(0)
        }
    }
    #[doc = "&index0 zone &index1 a1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CoeffA1(pub u32);
    impl CoeffA1 {
        #[doc = "coefficient a1."]
        #[inline(always)]
        pub const fn coeff_a1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "coefficient a1."]
        #[inline(always)]
        pub fn set_coeff_a1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CoeffA1 {
        #[inline(always)]
        fn default() -> CoeffA1 {
            CoeffA1(0)
        }
    }
    #[doc = "&index0 zone &index1 a2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CoeffA2(pub u32);
    impl CoeffA2 {
        #[doc = "coefficient a2."]
        #[inline(always)]
        pub const fn coeff_a2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "coefficient a2."]
        #[inline(always)]
        pub fn set_coeff_a2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CoeffA2 {
        #[inline(always)]
        fn default() -> CoeffA2 {
            CoeffA2(0)
        }
    }
    #[doc = "&index0 zone &index1 b0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CoeffB0(pub u32);
    impl CoeffB0 {
        #[doc = "coefficient b0."]
        #[inline(always)]
        pub const fn coeff_b0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "coefficient b0."]
        #[inline(always)]
        pub fn set_coeff_b0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CoeffB0 {
        #[inline(always)]
        fn default() -> CoeffB0 {
            CoeffB0(0)
        }
    }
    #[doc = "&index0 zone &index1 b1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CoeffB1(pub u32);
    impl CoeffB1 {
        #[doc = "coefficient b1."]
        #[inline(always)]
        pub const fn coeff_b1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "coefficient b1."]
        #[inline(always)]
        pub fn set_coeff_b1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CoeffB1 {
        #[inline(always)]
        fn default() -> CoeffB1 {
            CoeffB1(0)
        }
    }
    #[doc = "&index0 zone &index1 b2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CoeffB2(pub u32);
    impl CoeffB2 {
        #[doc = "coefficient b2."]
        #[inline(always)]
        pub const fn coeff_b2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "coefficient b2."]
        #[inline(always)]
        pub fn set_coeff_b2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CoeffB2 {
        #[inline(always)]
        fn default() -> CoeffB2 {
            CoeffB2(0)
        }
    }
    #[doc = "&index0 zone &index1 b3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CoeffB3(pub u32);
    impl CoeffB3 {
        #[doc = "coefficient b3."]
        #[inline(always)]
        pub const fn coeff_b3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "coefficient b3."]
        #[inline(always)]
        pub fn set_coeff_b3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CoeffB3 {
        #[inline(always)]
        fn default() -> CoeffB3 {
            CoeffB3(0)
        }
    }
    #[doc = "&index0 zone &index1 kscaling."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CoeffKs(pub u32);
    impl CoeffKs {
        #[doc = "coefficient kscaling."]
        #[inline(always)]
        pub const fn coeff_kscaling(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "coefficient kscaling."]
        #[inline(always)]
        pub fn set_coeff_kscaling(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for CoeffKs {
        #[inline(always)]
        fn default() -> CoeffKs {
            CoeffKs(0)
        }
    }
    #[doc = "enable d/q chan software inject adc value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DqAdcSwReady(pub u32);
    impl DqAdcSwReady {
        #[doc = "enable d/q chan software inject adc value."]
        #[inline(always)]
        pub const fn dq_adc_sw_ready(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable d/q chan software inject adc value."]
        #[inline(always)]
        pub fn set_dq_adc_sw_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for DqAdcSwReady {
        #[inline(always)]
        fn default() -> DqAdcSwReady {
            DqAdcSwReady(0)
        }
    }
    #[doc = "&index0 error adc latest value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EadcCurr(pub u32);
    impl EadcCurr {
        #[doc = "error adc latest value."]
        #[inline(always)]
        pub const fn eadc_curr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "error adc latest value."]
        #[inline(always)]
        pub fn set_eadc_curr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EadcCurr {
        #[inline(always)]
        fn default() -> EadcCurr {
            EadcCurr(0)
        }
    }
    #[doc = "&index0 eadc_highth value used in error adc cofficient selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EadcHighth(pub u32);
    impl EadcHighth {
        #[doc = "if error adc not bigger than eadc_lowth or not less than eadc_highth, use zone 2 cofficient；if not less than midlowth and not bigger than midhighth, use zone 0 cofficient；otherwire, use zone 1 cofficient."]
        #[inline(always)]
        pub const fn eadc_highth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "if error adc not bigger than eadc_lowth or not less than eadc_highth, use zone 2 cofficient；if not less than midlowth and not bigger than midhighth, use zone 0 cofficient；otherwire, use zone 1 cofficient."]
        #[inline(always)]
        pub fn set_eadc_highth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EadcHighth {
        #[inline(always)]
        fn default() -> EadcHighth {
            EadcHighth(0)
        }
    }
    #[doc = "&index0 eadc_lowth value used in error adc cofficient selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EadcLowth(pub u32);
    impl EadcLowth {
        #[doc = "if error adc not bigger than eadc_lowth or not less than eadc_highth, use zone 2 cofficient；if not less than midlowth and not bigger than midhighth, use zone 0 cofficient；otherwire, use zone 1 cofficient."]
        #[inline(always)]
        pub const fn eadc_lowth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "if error adc not bigger than eadc_lowth or not less than eadc_highth, use zone 2 cofficient；if not less than midlowth and not bigger than midhighth, use zone 0 cofficient；otherwire, use zone 1 cofficient."]
        #[inline(always)]
        pub fn set_eadc_lowth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EadcLowth {
        #[inline(always)]
        fn default() -> EadcLowth {
            EadcLowth(0)
        }
    }
    #[doc = "&index0 eadc_midhighth value used in error adc cofficient selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EadcMidhighth(pub u32);
    impl EadcMidhighth {
        #[doc = "if error adc not bigger than eadc_lowth or not less than eadc_highth, use zone 2 cofficient；if not less than midlowth and not bigger than midhighth, use zone 0 cofficient；otherwire, use zone 1 cofficient."]
        #[inline(always)]
        pub const fn eadc_midhighth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "if error adc not bigger than eadc_lowth or not less than eadc_highth, use zone 2 cofficient；if not less than midlowth and not bigger than midhighth, use zone 0 cofficient；otherwire, use zone 1 cofficient."]
        #[inline(always)]
        pub fn set_eadc_midhighth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EadcMidhighth {
        #[inline(always)]
        fn default() -> EadcMidhighth {
            EadcMidhighth(0)
        }
    }
    #[doc = "&index0 eadc_midlowth value used in error adc cofficient selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EadcMidlowth(pub u32);
    impl EadcMidlowth {
        #[doc = "if error adc not bigger than eadc_lowth or not less than eadc_highth, use zone 2 cofficient；if not less than midlowth and not bigger than midhighth, use zone 0 cofficient；otherwire, use zone 1 cofficient."]
        #[inline(always)]
        pub const fn eadc_midlowth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "if error adc not bigger than eadc_lowth or not less than eadc_highth, use zone 2 cofficient；if not less than midlowth and not bigger than midhighth, use zone 0 cofficient；otherwire, use zone 1 cofficient."]
        #[inline(always)]
        pub fn set_eadc_midlowth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EadcMidlowth {
        #[inline(always)]
        fn default() -> EadcMidlowth {
            EadcMidlowth(0)
        }
    }
    #[doc = "&index0 error adc previous0 value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EadcPre0(pub u32);
    impl EadcPre0 {
        #[doc = "error adc previous 0 value."]
        #[inline(always)]
        pub const fn eadc_pre0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "error adc previous 0 value."]
        #[inline(always)]
        pub fn set_eadc_pre0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EadcPre0 {
        #[inline(always)]
        fn default() -> EadcPre0 {
            EadcPre0(0)
        }
    }
    #[doc = "&index0 error adc previous1 value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EadcPre1(pub u32);
    impl EadcPre1 {
        #[doc = "error adc previous 1 value."]
        #[inline(always)]
        pub const fn eadc_pre1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "error adc previous 1 value."]
        #[inline(always)]
        pub fn set_eadc_pre1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EadcPre1 {
        #[inline(always)]
        fn default() -> EadcPre1 {
            EadcPre1(0)
        }
    }
    #[doc = "&index0 mode ctrl."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mode(pub u32);
    impl Mode {
        #[doc = "enable irq: irq_data_in_forbid , // 10 irq_forb_err_boundary , // 9 irq_p3z3_over_lo , // 8 irq_p3z3_over_hi , // 7 irq_p3z3_err_boundary , // 6 irq_z2_over_sf , // 5 irq_z2_over_lo , // 4 irq_z2_over_hi , // 3 irq_z2_err_boundary , // 2 irq_coef_err_boundary , // 1 irq_valid_clc // 0."]
        #[inline(always)]
        pub const fn enable_irq(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "enable irq: irq_data_in_forbid , // 10 irq_forb_err_boundary , // 9 irq_p3z3_over_lo , // 8 irq_p3z3_over_hi , // 7 irq_p3z3_err_boundary , // 6 irq_z2_over_sf , // 5 irq_z2_over_lo , // 4 irq_z2_over_hi , // 3 irq_z2_err_boundary , // 2 irq_coef_err_boundary , // 1 irq_valid_clc // 0."]
        #[inline(always)]
        pub fn set_enable_irq(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "dq mode."]
        #[inline(always)]
        pub const fn dq_mode(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "dq mode."]
        #[inline(always)]
        pub fn set_dq_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "open mode: CLC keep working even if bad irq status ocurred."]
        #[inline(always)]
        pub const fn mask_mode(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "open mode: CLC keep working even if bad irq status ocurred."]
        #[inline(always)]
        pub fn set_mask_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "enable CLC."]
        #[inline(always)]
        pub const fn enable_clc(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "enable CLC."]
        #[inline(always)]
        pub fn set_enable_clc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Mode {
        #[inline(always)]
        fn default() -> Mode {
            Mode(0)
        }
    }
    #[doc = "&index0 output value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutputValue(pub u32);
    impl OutputValue {
        #[doc = "output_value."]
        #[inline(always)]
        pub const fn output_value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "output_value."]
        #[inline(always)]
        pub fn set_output_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OutputValue {
        #[inline(always)]
        fn default() -> OutputValue {
            OutputValue(0)
        }
    }
    #[doc = "&index0 2p2z output clamp high threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2z2ClampHi(pub u32);
    impl P2z2ClampHi {
        #[doc = "2p2z output clamp high threshold."]
        #[inline(always)]
        pub const fn _2p2z_clamp_hi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "2p2z output clamp high threshold."]
        #[inline(always)]
        pub fn set__2p2z_clamp_hi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P2z2ClampHi {
        #[inline(always)]
        fn default() -> P2z2ClampHi {
            P2z2ClampHi(0)
        }
    }
    #[doc = "&index0 2p2z output clamp low threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2z2ClampLo(pub u32);
    impl P2z2ClampLo {
        #[doc = "2p2z output clamp low threshold."]
        #[inline(always)]
        pub const fn _2p2z_clamp_lo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "2p2z output clamp low threshold."]
        #[inline(always)]
        pub fn set__2p2z_clamp_lo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P2z2ClampLo {
        #[inline(always)]
        fn default() -> P2z2ClampLo {
            P2z2ClampLo(0)
        }
    }
    #[doc = "&index0 2p2z latest value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2z2Curr(pub u32);
    impl P2z2Curr {
        #[doc = "2p2z latest value."]
        #[inline(always)]
        pub const fn _2p2z_curr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "2p2z latest value."]
        #[inline(always)]
        pub fn set__2p2z_curr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P2z2Curr {
        #[inline(always)]
        fn default() -> P2z2Curr {
            P2z2Curr(0)
        }
    }
    #[doc = "&index0 2p2z previous0 value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2z2Pre0(pub u32);
    impl P2z2Pre0 {
        #[doc = "2p2z previous 0 value."]
        #[inline(always)]
        pub const fn _2p2z_pre0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "2p2z previous 0 value."]
        #[inline(always)]
        pub fn set__2p2z_pre0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P2z2Pre0 {
        #[inline(always)]
        fn default() -> P2z2Pre0 {
            P2z2Pre0(0)
        }
    }
    #[doc = "&index0 3p3z output clamp high threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P3z3ClampHi(pub u32);
    impl P3z3ClampHi {
        #[doc = "3p3z output clamp high threshold."]
        #[inline(always)]
        pub const fn _3p3z_clamp_hi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "3p3z output clamp high threshold."]
        #[inline(always)]
        pub fn set__3p3z_clamp_hi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P3z3ClampHi {
        #[inline(always)]
        fn default() -> P3z3ClampHi {
            P3z3ClampHi(0)
        }
    }
    #[doc = "&index0 3p3z output clamp low threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P3z3ClampLo(pub u32);
    impl P3z3ClampLo {
        #[doc = "3p3z output clamp low threshold."]
        #[inline(always)]
        pub const fn _3p3z_clamp_lo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "3p3z output clamp low threshold."]
        #[inline(always)]
        pub fn set__3p3z_clamp_lo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P3z3ClampLo {
        #[inline(always)]
        fn default() -> P3z3ClampLo {
            P3z3ClampLo(0)
        }
    }
    #[doc = "&index0 3p3z latest value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P3z3Curr(pub u32);
    impl P3z3Curr {
        #[doc = "3p3z latest value."]
        #[inline(always)]
        pub const fn _3p3z_curr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "3p3z latest value."]
        #[inline(always)]
        pub fn set__3p3z_curr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P3z3Curr {
        #[inline(always)]
        fn default() -> P3z3Curr {
            P3z3Curr(0)
        }
    }
    #[doc = "&index0 3p3z output forbid high threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P3z3ForbidHi(pub u32);
    impl P3z3ForbidHi {
        #[doc = "3p3z output forbid high threshold."]
        #[inline(always)]
        pub const fn _3p3z_forbid_hi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "3p3z output forbid high threshold."]
        #[inline(always)]
        pub fn set__3p3z_forbid_hi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P3z3ForbidHi {
        #[inline(always)]
        fn default() -> P3z3ForbidHi {
            P3z3ForbidHi(0)
        }
    }
    #[doc = "&index0 3p3z output forbid low threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P3z3ForbidLo(pub u32);
    impl P3z3ForbidLo {
        #[doc = "3p3z output forbid low threshold."]
        #[inline(always)]
        pub const fn _3p3z_forbid_lo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "3p3z output forbid low threshold."]
        #[inline(always)]
        pub fn set__3p3z_forbid_lo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P3z3ForbidLo {
        #[inline(always)]
        fn default() -> P3z3ForbidLo {
            P3z3ForbidLo(0)
        }
    }
    #[doc = "&index0 3p3z output forbid middle threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P3z3ForbidMd(pub u32);
    impl P3z3ForbidMd {
        #[doc = "3p3z output forbid middle threshold."]
        #[inline(always)]
        pub const fn _3p3z_forbid_md(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "3p3z output forbid middle threshold."]
        #[inline(always)]
        pub fn set__3p3z_forbid_md(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for P3z3ForbidMd {
        #[inline(always)]
        fn default() -> P3z3ForbidMd {
            P3z3ForbidMd(0)
        }
    }
    #[doc = "&index0 pwm_period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmPeriod(pub u32);
    impl PwmPeriod {
        #[doc = "pwm_period."]
        #[inline(always)]
        pub const fn pwm_period(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "pwm_period."]
        #[inline(always)]
        pub fn set_pwm_period(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PwmPeriod {
        #[inline(always)]
        fn default() -> PwmPeriod {
            PwmPeriod(0)
        }
    }
    #[doc = "&index0 irq_status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "status, write 1 to clear it. : irq_data_in_forbid , // 10 irq_forb_err_boundary , // 9 irq_p3z3_over_lo , // 8 irq_p3z3_over_hi , // 7 irq_p3z3_err_boundary , // 6 irq_z2_over_sf , // 5 irq_z2_over_lo , // 4 irq_z2_over_hi , // 3 irq_z2_err_boundary , // 2 irq_coef_err_boundary , // 1 irq_valid_clc // 0."]
        #[inline(always)]
        pub const fn status(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "status, write 1 to clear it. : irq_data_in_forbid , // 10 irq_forb_err_boundary , // 9 irq_p3z3_over_lo , // 8 irq_p3z3_over_hi , // 7 irq_p3z3_err_boundary , // 6 irq_z2_over_sf , // 5 irq_z2_over_lo , // 4 irq_z2_over_hi , // 3 irq_z2_err_boundary , // 2 irq_coef_err_boundary , // 1 irq_valid_clc // 0."]
        #[inline(always)]
        pub fn set_status(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    #[doc = "&index0 adc timestamp used."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timestamp(pub u32);
    impl Timestamp {
        #[doc = "timestamp."]
        #[inline(always)]
        pub const fn timestamp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "timestamp."]
        #[inline(always)]
        pub fn set_timestamp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Timestamp {
        #[inline(always)]
        fn default() -> Timestamp {
            Timestamp(0)
        }
    }
}
