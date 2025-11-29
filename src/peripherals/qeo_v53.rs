#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "QEO0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qeo {
    ptr: *mut u8,
}
unsafe impl Send for Qeo {}
unsafe impl Sync for Qeo {}
impl Qeo {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "analog waves mode."]
    #[inline(always)]
    pub const fn wave_mode(self) -> crate::common::Reg<regs::WaveMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "resolution of wave0/1/2."]
    #[inline(always)]
    pub const fn wave_resolution(
        self,
    ) -> crate::common::Reg<regs::WaveResolution, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wave_phase_shift(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WavePhaseShift, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wave_vd_vq_inject(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WaveVdVqInject, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize + n * 4usize) as _) }
    }
    #[doc = "load wave0/1/2 vd vq value."]
    #[inline(always)]
    pub const fn wave_vd_vq_load(
        self,
    ) -> crate::common::Reg<regs::WaveVdVqLoad, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wave_amplitude(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WaveAmplitude, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wave_mid_point(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WaveMidPoint, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wave_limit(self, n: usize) -> WaveLimit {
        assert!(n < 3usize);
        unsafe { WaveLimit::from_ptr(self.ptr.wrapping_add(0x3cusize + n * 8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wave_deadzone_shift(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WaveDeadzoneShift, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize + n * 4usize) as _) }
    }
    #[doc = "wave_a/b/z output mode."]
    #[inline(always)]
    pub const fn abz_mode(self) -> crate::common::Reg<regs::AbzMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "resolution of wave_a/b/z."]
    #[inline(always)]
    pub const fn abz_resolution(
        self,
    ) -> crate::common::Reg<regs::AbzResolution, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn abz_phase_shift(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::AbzPhaseShift, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize + n * 4usize) as _) }
    }
    #[doc = "Two-phase orthogonality wave 1/4 period."]
    #[inline(always)]
    pub const fn abz_line_width(self) -> crate::common::Reg<regs::AbzLineWidth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "wdog width of qeo."]
    #[inline(always)]
    pub const fn abz_wdog_width(self) -> crate::common::Reg<regs::AbzWdogWidth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "sync abz owned postion."]
    #[inline(always)]
    pub const fn abz_postion_sync(
        self,
    ) -> crate::common::Reg<regs::AbzPostionSync, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "pwm mode."]
    #[inline(always)]
    pub const fn pwm_mode(self) -> crate::common::Reg<regs::PwmMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "resolution of pwm."]
    #[inline(always)]
    pub const fn pwm_resolution(
        self,
    ) -> crate::common::Reg<regs::PwmResolution, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pwm_phase_shift(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PwmPhaseShift, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pwm_phase_table(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PwmPhaseTable, crate::common::RW> {
        assert!(n < 24usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize + n * 4usize) as _) }
    }
    #[doc = "softwave inject postion."]
    #[inline(always)]
    pub const fn pwm_postion_software(
        self,
    ) -> crate::common::Reg<regs::PwmPostionSoftware, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "select softwave inject postion."]
    #[inline(always)]
    pub const fn pwm_postion_sel(
        self,
    ) -> crate::common::Reg<regs::PwmPostionSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "qeo status."]
    #[inline(always)]
    pub const fn pwm_status(self) -> crate::common::Reg<regs::PwmStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "qeo debug 0."]
    #[inline(always)]
    pub const fn pwm_debug0(self) -> crate::common::Reg<regs::PwmDebug0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "qeo debug 1."]
    #[inline(always)]
    pub const fn pwm_debug1(self) -> crate::common::Reg<regs::PwmDebug1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "qeo debug 2."]
    #[inline(always)]
    pub const fn pwm_debug2(self) -> crate::common::Reg<regs::PwmDebug2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "qeo debug 3."]
    #[inline(always)]
    pub const fn pwm_debug3(self) -> crate::common::Reg<regs::PwmDebug3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WaveLimit {
    ptr: *mut u8,
}
unsafe impl Send for WaveLimit {}
unsafe impl Sync for WaveLimit {}
impl WaveLimit {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "wave0 low area limit value."]
    #[inline(always)]
    pub const fn min(self) -> crate::common::Reg<regs::Min, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "wave0 high area limit value."]
    #[inline(always)]
    pub const fn max(self) -> crate::common::Reg<regs::Max, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
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
    #[doc = "Two-phase orthogonality wave 1/4 period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzLineWidth(pub u32);
    impl AbzLineWidth {
        #[doc = "the num of system clk by 1/4 period when using as Two-phase orthogonality."]
        #[must_use]
        #[inline(always)]
        pub const fn line(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the num of system clk by 1/4 period when using as Two-phase orthogonality."]
        #[inline(always)]
        pub const fn set_line(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AbzLineWidth {
        #[inline(always)]
        fn default() -> AbzLineWidth {
            AbzLineWidth(0)
        }
    }
    impl core::fmt::Debug for AbzLineWidth {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AbzLineWidth")
                .field("line", &self.line())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AbzLineWidth {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AbzLineWidth {{ line: {=u32:?} }}", self.line())
        }
    }
    #[doc = "wave_a/b/z output mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzMode(pub u32);
    impl AbzMode {
        #[doc = "wave_a type: 0: Two-phase orthogonality wave_a. 1: pulse wave of pulse/reverse type. 2: up wave of up/down type. 3: Three-phase orthogonality wave_a."]
        #[must_use]
        #[inline(always)]
        pub const fn a_type(&self) -> super::vals::AWaveType {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::AWaveType::from_bits(val as u8)
        }
        #[doc = "wave_a type: 0: Two-phase orthogonality wave_a. 1: pulse wave of pulse/reverse type. 2: up wave of up/down type. 3: Three-phase orthogonality wave_a."]
        #[inline(always)]
        pub const fn set_a_type(&mut self, val: super::vals::AWaveType) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "wave_b type: 0: Two-phase orthogonality wave_b. 1: reverse wave of pulse/reverse type. 2: down wave of up/down type. 3: Three-phase orthogonality wave_b."]
        #[must_use]
        #[inline(always)]
        pub const fn b_type(&self) -> super::vals::BWaveType {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::BWaveType::from_bits(val as u8)
        }
        #[doc = "wave_b type: 0: Two-phase orthogonality wave_b. 1: reverse wave of pulse/reverse type. 2: down wave of up/down type. 3: Three-phase orthogonality wave_b."]
        #[inline(always)]
        pub const fn set_b_type(&mut self, val: super::vals::BWaveType) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "wave_z type: 0: zero pulse and output high at both wave_a and wave_b are high. mantain about 25% period. 1: zero pulse output high about 75% period. start from 0 to 75% period. 2: zero pulse output high about 100% period. 3: wave_z output as tree-phase wave same as wave_a/wave_b."]
        #[must_use]
        #[inline(always)]
        pub const fn z_type(&self) -> super::vals::ZWaveType {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::ZWaveType::from_bits(val as u8)
        }
        #[doc = "wave_z type: 0: zero pulse and output high at both wave_a and wave_b are high. mantain about 25% period. 1: zero pulse output high about 75% period. start from 0 to 75% period. 2: zero pulse output high about 100% period. 3: wave_z output as tree-phase wave same as wave_a/wave_b."]
        #[inline(always)]
        pub const fn set_z_type(&mut self, val: super::vals::ZWaveType) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "wave_a polarity. 0: normal output. 1: invert normal output."]
        #[must_use]
        #[inline(always)]
        pub const fn a_polarity(&self) -> super::vals::WavePolarity {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::WavePolarity::from_bits(val as u8)
        }
        #[doc = "wave_a polarity. 0: normal output. 1: invert normal output."]
        #[inline(always)]
        pub const fn set_a_polarity(&mut self, val: super::vals::WavePolarity) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "wave_b polarity. 0: normal output. 1: invert normal output."]
        #[must_use]
        #[inline(always)]
        pub const fn b_polarity(&self) -> super::vals::WavePolarity {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::WavePolarity::from_bits(val as u8)
        }
        #[doc = "wave_b polarity. 0: normal output. 1: invert normal output."]
        #[inline(always)]
        pub const fn set_b_polarity(&mut self, val: super::vals::WavePolarity) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "wave_z polarity. 0: normal output. 1: invert normal output."]
        #[must_use]
        #[inline(always)]
        pub const fn z_polarity(&self) -> super::vals::WavePolarity {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::WavePolarity::from_bits(val as u8)
        }
        #[doc = "wave_z polarity. 0: normal output. 1: invert normal output."]
        #[inline(always)]
        pub const fn set_z_polarity(&mut self, val: super::vals::WavePolarity) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "enable abz wdog: 0: disable abz wdog. 1: enable abz wdog."]
        #[must_use]
        #[inline(always)]
        pub const fn en_wdog(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "enable abz wdog: 0: disable abz wdog. 1: enable abz wdog."]
        #[inline(always)]
        pub const fn set_en_wdog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "pulse reverse wave，reverse edge point: 0: between pulse's posedge and negedge, min period dedicated by the num line_width 1: edge change point flow pulse's negedge."]
        #[must_use]
        #[inline(always)]
        pub const fn reverse_edge_type(&self) -> super::vals::ReverseEdgeType {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::ReverseEdgeType::from_bits(val as u8)
        }
        #[doc = "pulse reverse wave，reverse edge point: 0: between pulse's posedge and negedge, min period dedicated by the num line_width 1: edge change point flow pulse's negedge."]
        #[inline(always)]
        pub const fn set_reverse_edge_type(&mut self, val: super::vals::ReverseEdgeType) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
    }
    impl Default for AbzMode {
        #[inline(always)]
        fn default() -> AbzMode {
            AbzMode(0)
        }
    }
    impl core::fmt::Debug for AbzMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AbzMode")
                .field("a_type", &self.a_type())
                .field("b_type", &self.b_type())
                .field("z_type", &self.z_type())
                .field("a_polarity", &self.a_polarity())
                .field("b_polarity", &self.b_polarity())
                .field("z_polarity", &self.z_polarity())
                .field("en_wdog", &self.en_wdog())
                .field("reverse_edge_type", &self.reverse_edge_type())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AbzMode {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AbzMode {{ a_type: {:?}, b_type: {:?}, z_type: {:?}, a_polarity: {:?}, b_polarity: {:?}, z_polarity: {:?}, en_wdog: {=bool:?}, reverse_edge_type: {:?} }}" , self . a_type () , self . b_type () , self . z_type () , self . a_polarity () , self . b_polarity () , self . z_polarity () , self . en_wdog () , self . reverse_edge_type ())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzPhaseShift(pub u32);
    impl AbzPhaseShift {
        #[doc = "wave_a phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^16) period."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "wave_a phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^16) period."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for AbzPhaseShift {
        #[inline(always)]
        fn default() -> AbzPhaseShift {
            AbzPhaseShift(0)
        }
    }
    impl core::fmt::Debug for AbzPhaseShift {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AbzPhaseShift")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AbzPhaseShift {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AbzPhaseShift {{ val: {=u16:?} }}", self.val())
        }
    }
    #[doc = "sync abz owned postion."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzPostionSync(pub u32);
    impl AbzPostionSync {
        #[doc = "load next valid postion into abz owned postion. always read 0 0: sync abz owned postion with next valid postion. 1: not sync."]
        #[must_use]
        #[inline(always)]
        pub const fn postion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "load next valid postion into abz owned postion. always read 0 0: sync abz owned postion with next valid postion. 1: not sync."]
        #[inline(always)]
        pub const fn set_postion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for AbzPostionSync {
        #[inline(always)]
        fn default() -> AbzPostionSync {
            AbzPostionSync(0)
        }
    }
    impl core::fmt::Debug for AbzPostionSync {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AbzPostionSync")
                .field("postion", &self.postion())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AbzPostionSync {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AbzPostionSync {{ postion: {=bool:?} }}", self.postion())
        }
    }
    #[doc = "resolution of wave_a/b/z."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzResolution(pub u32);
    impl AbzResolution {
        #[doc = "wave_a/b/z resolution."]
        #[must_use]
        #[inline(always)]
        pub const fn lines(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave_a/b/z resolution."]
        #[inline(always)]
        pub const fn set_lines(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AbzResolution {
        #[inline(always)]
        fn default() -> AbzResolution {
            AbzResolution(0)
        }
    }
    impl core::fmt::Debug for AbzResolution {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AbzResolution")
                .field("lines", &self.lines())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AbzResolution {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AbzResolution {{ lines: {=u32:?} }}", self.lines())
        }
    }
    #[doc = "wdog width of qeo."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzWdogWidth(pub u32);
    impl AbzWdogWidth {
        #[doc = "wave will step 1/4 line to reminder user QEO still in controlled if QEO has no any toggle after the num of wdog_width sys clk."]
        #[must_use]
        #[inline(always)]
        pub const fn width(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave will step 1/4 line to reminder user QEO still in controlled if QEO has no any toggle after the num of wdog_width sys clk."]
        #[inline(always)]
        pub const fn set_width(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AbzWdogWidth {
        #[inline(always)]
        fn default() -> AbzWdogWidth {
            AbzWdogWidth(0)
        }
    }
    impl core::fmt::Debug for AbzWdogWidth {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AbzWdogWidth")
                .field("width", &self.width())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AbzWdogWidth {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AbzWdogWidth {{ width: {=u32:?} }}", self.width())
        }
    }
    #[doc = "wave0 high area limit value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Max(pub u32);
    impl Max {
        #[doc = "high area limit level0."]
        #[must_use]
        #[inline(always)]
        pub const fn limit0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "high area limit level0."]
        #[inline(always)]
        pub const fn set_limit0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "high area limit level1."]
        #[must_use]
        #[inline(always)]
        pub const fn limit1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "high area limit level1."]
        #[inline(always)]
        pub const fn set_limit1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Max {
        #[inline(always)]
        fn default() -> Max {
            Max(0)
        }
    }
    impl core::fmt::Debug for Max {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Max")
                .field("limit0", &self.limit0())
                .field("limit1", &self.limit1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Max {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Max {{ limit0: {=u16:?}, limit1: {=u16:?} }}",
                self.limit0(),
                self.limit1()
            )
        }
    }
    #[doc = "wave0 low area limit value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Min(pub u32);
    impl Min {
        #[doc = "low area limit level0."]
        #[must_use]
        #[inline(always)]
        pub const fn limit0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "low area limit level0."]
        #[inline(always)]
        pub const fn set_limit0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "low area limit level1."]
        #[must_use]
        #[inline(always)]
        pub const fn limit1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "low area limit level1."]
        #[inline(always)]
        pub const fn set_limit1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Min {
        #[inline(always)]
        fn default() -> Min {
            Min(0)
        }
    }
    impl core::fmt::Debug for Min {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Min")
                .field("limit0", &self.limit0())
                .field("limit1", &self.limit1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Min {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Min {{ limit0: {=u16:?}, limit1: {=u16:?} }}",
                self.limit0(),
                self.limit1()
            )
        }
    }
    #[doc = "qeo debug 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmDebug0(pub u32);
    impl PwmDebug0 {
        #[doc = "wave0 observe."]
        #[must_use]
        #[inline(always)]
        pub const fn wave0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "wave0 observe."]
        #[inline(always)]
        pub const fn set_wave0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "wave1 observe."]
        #[must_use]
        #[inline(always)]
        pub const fn wave1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "wave1 observe."]
        #[inline(always)]
        pub const fn set_wave1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PwmDebug0 {
        #[inline(always)]
        fn default() -> PwmDebug0 {
            PwmDebug0(0)
        }
    }
    impl core::fmt::Debug for PwmDebug0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmDebug0")
                .field("wave0", &self.wave0())
                .field("wave1", &self.wave1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmDebug0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PwmDebug0 {{ wave0: {=u16:?}, wave1: {=u16:?} }}",
                self.wave0(),
                self.wave1()
            )
        }
    }
    #[doc = "qeo debug 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmDebug1(pub u32);
    impl PwmDebug1 {
        #[doc = "wave2 observe."]
        #[must_use]
        #[inline(always)]
        pub const fn wave2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "wave2 observe."]
        #[inline(always)]
        pub const fn set_wave2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "wave_a observe."]
        #[must_use]
        #[inline(always)]
        pub const fn wave_a(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "wave_a observe."]
        #[inline(always)]
        pub const fn set_wave_a(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "wave_b observe."]
        #[must_use]
        #[inline(always)]
        pub const fn wave_b(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "wave_b observe."]
        #[inline(always)]
        pub const fn set_wave_b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "wave_z observe."]
        #[must_use]
        #[inline(always)]
        pub const fn wave_z(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "wave_z observe."]
        #[inline(always)]
        pub const fn set_wave_z(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "qeo finish observe."]
        #[must_use]
        #[inline(always)]
        pub const fn qeo_finish(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "qeo finish observe."]
        #[inline(always)]
        pub const fn set_qeo_finish(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for PwmDebug1 {
        #[inline(always)]
        fn default() -> PwmDebug1 {
            PwmDebug1(0)
        }
    }
    impl core::fmt::Debug for PwmDebug1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmDebug1")
                .field("wave2", &self.wave2())
                .field("wave_a", &self.wave_a())
                .field("wave_b", &self.wave_b())
                .field("wave_z", &self.wave_z())
                .field("qeo_finish", &self.qeo_finish())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmDebug1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PwmDebug1 {{ wave2: {=u16:?}, wave_a: {=bool:?}, wave_b: {=bool:?}, wave_z: {=bool:?}, qeo_finish: {=bool:?} }}" , self . wave2 () , self . wave_a () , self . wave_b () , self . wave_z () , self . qeo_finish ())
        }
    }
    #[doc = "qeo debug 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmDebug2(pub u32);
    impl PwmDebug2 {
        #[doc = "abz_own_postion observe."]
        #[must_use]
        #[inline(always)]
        pub const fn abz_own_postion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "abz_own_postion observe."]
        #[inline(always)]
        pub const fn set_abz_own_postion(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PwmDebug2 {
        #[inline(always)]
        fn default() -> PwmDebug2 {
            PwmDebug2(0)
        }
    }
    impl core::fmt::Debug for PwmDebug2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmDebug2")
                .field("abz_own_postion", &self.abz_own_postion())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmDebug2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PwmDebug2 {{ abz_own_postion: {=u32:?} }}",
                self.abz_own_postion()
            )
        }
    }
    #[doc = "qeo debug 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmDebug3(pub u32);
    impl PwmDebug3 {
        #[doc = "abz_own_postion observe."]
        #[must_use]
        #[inline(always)]
        pub const fn abz_own_postion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "abz_own_postion observe."]
        #[inline(always)]
        pub const fn set_abz_own_postion(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PwmDebug3 {
        #[inline(always)]
        fn default() -> PwmDebug3 {
            PwmDebug3(0)
        }
    }
    impl core::fmt::Debug for PwmDebug3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmDebug3")
                .field("abz_own_postion", &self.abz_own_postion())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmDebug3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PwmDebug3 {{ abz_own_postion: {=u32:?} }}",
                self.abz_own_postion()
            )
        }
    }
    #[doc = "pwm mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmMode(pub u32);
    impl PwmMode {
        #[doc = "pwm force phase number."]
        #[must_use]
        #[inline(always)]
        pub const fn phase_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "pwm force phase number."]
        #[inline(always)]
        pub const fn set_phase_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "exchange PWM pairs’ output 0: not exchange. 1: exchange."]
        #[must_use]
        #[inline(always)]
        pub const fn revise_up_dn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "exchange PWM pairs’ output 0: not exchange. 1: exchange."]
        #[inline(always)]
        pub const fn set_revise_up_dn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PWM safety mode bypass 0: not bypass 1: bypass."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm_safety_bypass(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PWM safety mode bypass 0: not bypass 1: bypass."]
        #[inline(always)]
        pub const fn set_pwm_safety_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PWM enter safety mode 0: not enter 1: enter."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm_enter_safety_mode(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PWM enter safety mode 0: not enter 1: enter."]
        #[inline(always)]
        pub const fn set_pwm_enter_safety_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PWM safety mode phase table."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm_safety(&self, n: usize) -> super::vals::PwmMode {
            assert!(n < 8usize);
            let offs = 16usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::PwmMode::from_bits(val as u8)
        }
        #[doc = "PWM safety mode phase table."]
        #[inline(always)]
        pub const fn set_pwm_safety(&mut self, n: usize, val: super::vals::PwmMode) {
            assert!(n < 8usize);
            let offs = 16usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for PwmMode {
        #[inline(always)]
        fn default() -> PwmMode {
            PwmMode(0)
        }
    }
    impl core::fmt::Debug for PwmMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmMode")
                .field("phase_num", &self.phase_num())
                .field("revise_up_dn", &self.revise_up_dn())
                .field("pwm_safety_bypass", &self.pwm_safety_bypass())
                .field("pwm_enter_safety_mode", &self.pwm_enter_safety_mode())
                .field("pwm_safety[0]", &self.pwm_safety(0usize))
                .field("pwm_safety[1]", &self.pwm_safety(1usize))
                .field("pwm_safety[2]", &self.pwm_safety(2usize))
                .field("pwm_safety[3]", &self.pwm_safety(3usize))
                .field("pwm_safety[4]", &self.pwm_safety(4usize))
                .field("pwm_safety[5]", &self.pwm_safety(5usize))
                .field("pwm_safety[6]", &self.pwm_safety(6usize))
                .field("pwm_safety[7]", &self.pwm_safety(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmMode {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PwmMode {{ phase_num: {=u8:?}, revise_up_dn: {=bool:?}, pwm_safety_bypass: {=bool:?}, pwm_enter_safety_mode: {=bool:?}, pwm_safety[0]: {:?}, pwm_safety[1]: {:?}, pwm_safety[2]: {:?}, pwm_safety[3]: {:?}, pwm_safety[4]: {:?}, pwm_safety[5]: {:?}, pwm_safety[6]: {:?}, pwm_safety[7]: {:?} }}" , self . phase_num () , self . revise_up_dn () , self . pwm_safety_bypass () , self . pwm_enter_safety_mode () , self . pwm_safety (0usize) , self . pwm_safety (1usize) , self . pwm_safety (2usize) , self . pwm_safety (3usize) , self . pwm_safety (4usize) , self . pwm_safety (5usize) , self . pwm_safety (6usize) , self . pwm_safety (7usize))
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmPhaseShift(pub u32);
    impl PwmPhaseShift {
        #[doc = "pwm_a phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^16) period."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "pwm_a phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^16) period."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for PwmPhaseShift {
        #[inline(always)]
        fn default() -> PwmPhaseShift {
            PwmPhaseShift(0)
        }
    }
    impl core::fmt::Debug for PwmPhaseShift {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmPhaseShift")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmPhaseShift {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PwmPhaseShift {{ val: {=u16:?} }}", self.val())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmPhaseTable(pub u32);
    impl PwmPhaseTable {
        #[doc = "pwm phase table value."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm(&self, n: usize) -> super::vals::PwmMode {
            assert!(n < 8usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::PwmMode::from_bits(val as u8)
        }
        #[doc = "pwm phase table value."]
        #[inline(always)]
        pub const fn set_pwm(&mut self, n: usize, val: super::vals::PwmMode) {
            assert!(n < 8usize);
            let offs = 0usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for PwmPhaseTable {
        #[inline(always)]
        fn default() -> PwmPhaseTable {
            PwmPhaseTable(0)
        }
    }
    impl core::fmt::Debug for PwmPhaseTable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmPhaseTable")
                .field("pwm[0]", &self.pwm(0usize))
                .field("pwm[1]", &self.pwm(1usize))
                .field("pwm[2]", &self.pwm(2usize))
                .field("pwm[3]", &self.pwm(3usize))
                .field("pwm[4]", &self.pwm(4usize))
                .field("pwm[5]", &self.pwm(5usize))
                .field("pwm[6]", &self.pwm(6usize))
                .field("pwm[7]", &self.pwm(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmPhaseTable {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PwmPhaseTable {{ pwm[0]: {:?}, pwm[1]: {:?}, pwm[2]: {:?}, pwm[3]: {:?}, pwm[4]: {:?}, pwm[5]: {:?}, pwm[6]: {:?}, pwm[7]: {:?} }}" , self . pwm (0usize) , self . pwm (1usize) , self . pwm (2usize) , self . pwm (3usize) , self . pwm (4usize) , self . pwm (5usize) , self . pwm (6usize) , self . pwm (7usize))
        }
    }
    #[doc = "select softwave inject postion."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmPostionSel(pub u32);
    impl PwmPostionSel {
        #[doc = "enable softwave inject postion. 0: disable. 1: enable."]
        #[must_use]
        #[inline(always)]
        pub const fn postion_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable softwave inject postion. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn set_postion_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PwmPostionSel {
        #[inline(always)]
        fn default() -> PwmPostionSel {
            PwmPostionSel(0)
        }
    }
    impl core::fmt::Debug for PwmPostionSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmPostionSel")
                .field("postion_sel", &self.postion_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmPostionSel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PwmPostionSel {{ postion_sel: {=bool:?} }}",
                self.postion_sel()
            )
        }
    }
    #[doc = "softwave inject postion."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmPostionSoftware(pub u32);
    impl PwmPostionSoftware {
        #[doc = "softwave inject postion."]
        #[must_use]
        #[inline(always)]
        pub const fn postion_softwave(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "softwave inject postion."]
        #[inline(always)]
        pub const fn set_postion_softwave(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PwmPostionSoftware {
        #[inline(always)]
        fn default() -> PwmPostionSoftware {
            PwmPostionSoftware(0)
        }
    }
    impl core::fmt::Debug for PwmPostionSoftware {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmPostionSoftware")
                .field("postion_softwave", &self.postion_softwave())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmPostionSoftware {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PwmPostionSoftware {{ postion_softwave: {=u32:?} }}",
                self.postion_softwave()
            )
        }
    }
    #[doc = "resolution of pwm."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmResolution(pub u32);
    impl PwmResolution {
        #[doc = "pwm resolution."]
        #[must_use]
        #[inline(always)]
        pub const fn lines(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "pwm resolution."]
        #[inline(always)]
        pub const fn set_lines(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PwmResolution {
        #[inline(always)]
        fn default() -> PwmResolution {
            PwmResolution(0)
        }
    }
    impl core::fmt::Debug for PwmResolution {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmResolution")
                .field("lines", &self.lines())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmResolution {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PwmResolution {{ lines: {=u32:?} }}", self.lines())
        }
    }
    #[doc = "qeo status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmStatus(pub u32);
    impl PwmStatus {
        #[doc = "pwm_fault status."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm_safety(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "pwm_fault status."]
        #[inline(always)]
        pub const fn set_pwm_safety(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "qeo_pwm_force observe."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm_fource(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "qeo_pwm_force observe."]
        #[inline(always)]
        pub const fn set_pwm_fource(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PwmStatus {
        #[inline(always)]
        fn default() -> PwmStatus {
            PwmStatus(0)
        }
    }
    impl core::fmt::Debug for PwmStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PwmStatus")
                .field("pwm_safety", &self.pwm_safety())
                .field("pwm_fource", &self.pwm_fource())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PwmStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PwmStatus {{ pwm_safety: {=bool:?}, pwm_fource: {=u16:?} }}",
                self.pwm_safety(),
                self.pwm_fource()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveAmplitude(pub u32);
    impl WaveAmplitude {
        #[doc = "amplitude scaling value. bit15-12 are integer part value. bit11-0 are fraction value."]
        #[must_use]
        #[inline(always)]
        pub const fn amp_val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "amplitude scaling value. bit15-12 are integer part value. bit11-0 are fraction value."]
        #[inline(always)]
        pub const fn set_amp_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "enable wave amplitude scaling. 0: disable; 1: enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en_scal(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "enable wave amplitude scaling. 0: disable; 1: enable."]
        #[inline(always)]
        pub const fn set_en_scal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for WaveAmplitude {
        #[inline(always)]
        fn default() -> WaveAmplitude {
            WaveAmplitude(0)
        }
    }
    impl core::fmt::Debug for WaveAmplitude {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WaveAmplitude")
                .field("amp_val", &self.amp_val())
                .field("en_scal", &self.en_scal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WaveAmplitude {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "WaveAmplitude {{ amp_val: {=u16:?}, en_scal: {=bool:?} }}",
                self.amp_val(),
                self.en_scal()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveDeadzoneShift(pub u32);
    impl WaveDeadzoneShift {
        #[doc = "wave0 deadzone shifter value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "wave0 deadzone shifter value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for WaveDeadzoneShift {
        #[inline(always)]
        fn default() -> WaveDeadzoneShift {
            WaveDeadzoneShift(0)
        }
    }
    impl core::fmt::Debug for WaveDeadzoneShift {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WaveDeadzoneShift")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WaveDeadzoneShift {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WaveDeadzoneShift {{ val: {=u16:?} }}", self.val())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveMidPoint(pub u32);
    impl WaveMidPoint {
        #[doc = "wave0 output middle point, use this value as 32 bit signed value. bit 31 is signed bit. bit30-27 is integer part value. bit26-0 is fraction value."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave0 output middle point, use this value as 32 bit signed value. bit 31 is signed bit. bit30-27 is integer part value. bit26-0 is fraction value."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WaveMidPoint {
        #[inline(always)]
        fn default() -> WaveMidPoint {
            WaveMidPoint(0)
        }
    }
    impl core::fmt::Debug for WaveMidPoint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WaveMidPoint")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WaveMidPoint {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WaveMidPoint {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "analog waves mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveMode(pub u32);
    impl WaveMode {
        #[doc = "wave0/1/2 output mode. 0: cosine wave. 1: saddle wave. 2. abs cosine wave. 3. saw wave."]
        #[must_use]
        #[inline(always)]
        pub const fn waves_output_type(&self) -> super::vals::WavesOutputType {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::WavesOutputType::from_bits(val as u8)
        }
        #[doc = "wave0/1/2 output mode. 0: cosine wave. 1: saddle wave. 2. abs cosine wave. 3. saw wave."]
        #[inline(always)]
        pub const fn set_waves_output_type(&mut self, val: super::vals::WavesOutputType) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "wave0 VdVq inject enable. 0: disable VdVq inject. 1: enable VdVq inject."]
        #[must_use]
        #[inline(always)]
        pub const fn en_wave0_vd_vq_inject(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "wave0 VdVq inject enable. 0: disable VdVq inject. 1: enable VdVq inject."]
        #[inline(always)]
        pub const fn set_en_wave0_vd_vq_inject(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "wave1 VdVq inject enable. 0: disable VdVq inject. 1: enable VdVq inject."]
        #[must_use]
        #[inline(always)]
        pub const fn en_wave1_vd_vq_inject(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "wave1 VdVq inject enable. 0: disable VdVq inject. 1: enable VdVq inject."]
        #[inline(always)]
        pub const fn set_en_wave1_vd_vq_inject(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "wave2 VdVq inject enable. 0: disable VdVq inject. 1: enable VdVq inject."]
        #[must_use]
        #[inline(always)]
        pub const fn en_wave2_vd_vq_inject(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "wave2 VdVq inject enable. 0: disable VdVq inject. 1: enable VdVq inject."]
        #[inline(always)]
        pub const fn set_en_wave2_vd_vq_inject(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "saddle type seclect; 0:standard saddle. 1: triple-cos saddle."]
        #[must_use]
        #[inline(always)]
        pub const fn saddle_type(&self) -> super::vals::SaddleType {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::SaddleType::from_bits(val as u8)
        }
        #[doc = "saddle type seclect; 0:standard saddle. 1: triple-cos saddle."]
        #[inline(always)]
        pub const fn set_saddle_type(&mut self, val: super::vals::SaddleType) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "wave0 below min limit mode. 0: output 0. 1: output 0xffff. 2: output as level_min_limit0.level1_min_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave0_below_min_limit(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "wave0 below min limit mode. 0: output 0. 1: output 0xffff. 2: output as level_min_limit0.level1_min_limit."]
        #[inline(always)]
        pub const fn set_wave0_below_min_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "wave0 low area0 limit mode. 0: output 0. 1: output as level_min_limit0.level1_min_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave0_low_area0_limit(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "wave0 low area0 limit mode. 0: output 0. 1: output as level_min_limit0.level1_min_limit."]
        #[inline(always)]
        pub const fn set_wave0_low_area0_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "wave0 low area1 limit mode. 0: output 0. 1: output as level_min_limit0.level1_min_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave0_low_area1_limit(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "wave0 low area1 limit mode. 0: output 0. 1: output as level_min_limit0.level1_min_limit."]
        #[inline(always)]
        pub const fn set_wave0_low_area1_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "wave0 high area0 limit mode. 0: output 0xffff. 1: output as level_max_limit0.level0_max_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave0_high_area0_limit(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "wave0 high area0 limit mode. 0: output 0xffff. 1: output as level_max_limit0.level0_max_limit."]
        #[inline(always)]
        pub const fn set_wave0_high_area0_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "wave0 high area1 limit mode. 0: output 0xffff. 1: output as level_max_limit0.level0_max_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave0_high_area1_limit(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "wave0 high area1 limit mode. 0: output 0xffff. 1: output as level_max_limit0.level0_max_limit."]
        #[inline(always)]
        pub const fn set_wave0_high_area1_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "wave0 above max limit mode. 0: output 0xffff. 1: output 0x0. 2: output as level_max_limit0.level0_max_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave0_above_max_limit(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "wave0 above max limit mode. 0: output 0xffff. 1: output 0x0. 2: output as level_max_limit0.level0_max_limit."]
        #[inline(always)]
        pub const fn set_wave0_above_max_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "wave1 below min limit mode. 0: output 0. 1: output 0xffff. 2: output as level_min_limit1.level1_min_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave1_below_min_limit(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "wave1 below min limit mode. 0: output 0. 1: output 0xffff. 2: output as level_min_limit1.level1_min_limit."]
        #[inline(always)]
        pub const fn set_wave1_below_min_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "wave1 low area0 limit mode. 0: output 0. 1: output as level_min_limit1.level1_min_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave1_low_area0_limit(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "wave1 low area0 limit mode. 0: output 0. 1: output as level_min_limit1.level1_min_limit."]
        #[inline(always)]
        pub const fn set_wave1_low_area0_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "wave1 low area1 limit mode. 0: output 0. 1: output as level_min_limit1.level1_min_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave1_low_area1_limit(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "wave1 low area1 limit mode. 0: output 0. 1: output as level_min_limit1.level1_min_limit."]
        #[inline(always)]
        pub const fn set_wave1_low_area1_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "wave1 high area0 limit mode. 0: output 0xffff. 1: output as level_max_limit1.level0_max_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave1_high_area0_limit(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "wave1 high area0 limit mode. 0: output 0xffff. 1: output as level_max_limit1.level0_max_limit."]
        #[inline(always)]
        pub const fn set_wave1_high_area0_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "wave1 high area1 limit mode. 0: output 0xffff. 1: output as level_max_limit1.level0_max_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave1_high_area1_limit(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "wave1 high area1 limit mode. 0: output 0xffff. 1: output as level_max_limit1.level0_max_limit."]
        #[inline(always)]
        pub const fn set_wave1_high_area1_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "wave1 above max limit mode. 0: output 0xffff. 1: output 0x0. 2: output as level_max_limit1.level0_max_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave1_above_max_limit(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "wave1 above max limit mode. 0: output 0xffff. 1: output 0x0. 2: output as level_max_limit1.level0_max_limit."]
        #[inline(always)]
        pub const fn set_wave1_above_max_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "wave2 below min limit mode. 0: output 0. 1: output 0xffff. 2: output as level_min_limit2.level1_min_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave2_below_min_limit(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "wave2 below min limit mode. 0: output 0. 1: output 0xffff. 2: output as level_min_limit2.level1_min_limit."]
        #[inline(always)]
        pub const fn set_wave2_below_min_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "wave2 low area0 limit mode. 0: output 0. 1: output as level_min_limit2.level1_min_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave2_low_area0_limit(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "wave2 low area0 limit mode. 0: output 0. 1: output as level_min_limit2.level1_min_limit."]
        #[inline(always)]
        pub const fn set_wave2_low_area0_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "wave2 low area1 limit mode. 0: output 0. 1: output as level_min_limit2.level1_min_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave2_low_area1_limit(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "wave2 low area1 limit mode. 0: output 0. 1: output as level_min_limit2.level1_min_limit."]
        #[inline(always)]
        pub const fn set_wave2_low_area1_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "wave2 high area0 limit mode. 0: output 0xffff. 1: output as level_max_limit2.level0_max_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave2_high_area0_limit(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "wave2 high area0 limit mode. 0: output 0xffff. 1: output as level_max_limit2.level0_max_limit."]
        #[inline(always)]
        pub const fn set_wave2_high_area0_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "wave2 high area1 limit mode. 0: output 0xffff. 1: output as level_max_limit2.level0_max_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave2_high_area1_limit(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "wave2 high area1 limit mode. 0: output 0xffff. 1: output as level_max_limit2.level0_max_limit."]
        #[inline(always)]
        pub const fn set_wave2_high_area1_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "wave2 above max limit mode. 0: output 0xffff. 1: output 0x0. 2: output as level_max_limit2.level0_max_limit."]
        #[must_use]
        #[inline(always)]
        pub const fn wave2_above_max_limit(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "wave2 above max limit mode. 0: output 0xffff. 1: output 0x0. 2: output as level_max_limit2.level0_max_limit."]
        #[inline(always)]
        pub const fn set_wave2_above_max_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for WaveMode {
        #[inline(always)]
        fn default() -> WaveMode {
            WaveMode(0)
        }
    }
    impl core::fmt::Debug for WaveMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WaveMode")
                .field("waves_output_type", &self.waves_output_type())
                .field("en_wave0_vd_vq_inject", &self.en_wave0_vd_vq_inject())
                .field("en_wave1_vd_vq_inject", &self.en_wave1_vd_vq_inject())
                .field("en_wave2_vd_vq_inject", &self.en_wave2_vd_vq_inject())
                .field("saddle_type", &self.saddle_type())
                .field("wave0_below_min_limit", &self.wave0_below_min_limit())
                .field("wave0_low_area0_limit", &self.wave0_low_area0_limit())
                .field("wave0_low_area1_limit", &self.wave0_low_area1_limit())
                .field("wave0_high_area0_limit", &self.wave0_high_area0_limit())
                .field("wave0_high_area1_limit", &self.wave0_high_area1_limit())
                .field("wave0_above_max_limit", &self.wave0_above_max_limit())
                .field("wave1_below_min_limit", &self.wave1_below_min_limit())
                .field("wave1_low_area0_limit", &self.wave1_low_area0_limit())
                .field("wave1_low_area1_limit", &self.wave1_low_area1_limit())
                .field("wave1_high_area0_limit", &self.wave1_high_area0_limit())
                .field("wave1_high_area1_limit", &self.wave1_high_area1_limit())
                .field("wave1_above_max_limit", &self.wave1_above_max_limit())
                .field("wave2_below_min_limit", &self.wave2_below_min_limit())
                .field("wave2_low_area0_limit", &self.wave2_low_area0_limit())
                .field("wave2_low_area1_limit", &self.wave2_low_area1_limit())
                .field("wave2_high_area0_limit", &self.wave2_high_area0_limit())
                .field("wave2_high_area1_limit", &self.wave2_high_area1_limit())
                .field("wave2_above_max_limit", &self.wave2_above_max_limit())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WaveMode {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "WaveMode {{ waves_output_type: {:?}, en_wave0_vd_vq_inject: {=bool:?}, en_wave1_vd_vq_inject: {=bool:?}, en_wave2_vd_vq_inject: {=bool:?}, saddle_type: {:?}, wave0_below_min_limit: {=u8:?}, wave0_low_area0_limit: {=bool:?}, wave0_low_area1_limit: {=bool:?}, wave0_high_area0_limit: {=bool:?}, wave0_high_area1_limit: {=bool:?}, wave0_above_max_limit: {=u8:?}, wave1_below_min_limit: {=u8:?}, wave1_low_area0_limit: {=bool:?}, wave1_low_area1_limit: {=bool:?}, wave1_high_area0_limit: {=bool:?}, wave1_high_area1_limit: {=bool:?}, wave1_above_max_limit: {=u8:?}, wave2_below_min_limit: {=u8:?}, wave2_low_area0_limit: {=bool:?}, wave2_low_area1_limit: {=bool:?}, wave2_high_area0_limit: {=bool:?}, wave2_high_area1_limit: {=bool:?}, wave2_above_max_limit: {=u8:?} }}" , self . waves_output_type () , self . en_wave0_vd_vq_inject () , self . en_wave1_vd_vq_inject () , self . en_wave2_vd_vq_inject () , self . saddle_type () , self . wave0_below_min_limit () , self . wave0_low_area0_limit () , self . wave0_low_area1_limit () , self . wave0_high_area0_limit () , self . wave0_high_area1_limit () , self . wave0_above_max_limit () , self . wave1_below_min_limit () , self . wave1_low_area0_limit () , self . wave1_low_area1_limit () , self . wave1_high_area0_limit () , self . wave1_high_area1_limit () , self . wave1_above_max_limit () , self . wave2_below_min_limit () , self . wave2_low_area0_limit () , self . wave2_low_area1_limit () , self . wave2_high_area0_limit () , self . wave2_high_area1_limit () , self . wave2_above_max_limit ())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WavePhaseShift(pub u32);
    impl WavePhaseShift {
        #[doc = "wave0 phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^16) period."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "wave0 phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^16) period."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for WavePhaseShift {
        #[inline(always)]
        fn default() -> WavePhaseShift {
            WavePhaseShift(0)
        }
    }
    impl core::fmt::Debug for WavePhaseShift {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WavePhaseShift")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WavePhaseShift {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WavePhaseShift {{ val: {=u16:?} }}", self.val())
        }
    }
    #[doc = "resolution of wave0/1/2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveResolution(pub u32);
    impl WaveResolution {
        #[doc = "wave0/1/2 resolution."]
        #[must_use]
        #[inline(always)]
        pub const fn lines(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave0/1/2 resolution."]
        #[inline(always)]
        pub const fn set_lines(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WaveResolution {
        #[inline(always)]
        fn default() -> WaveResolution {
            WaveResolution(0)
        }
    }
    impl core::fmt::Debug for WaveResolution {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WaveResolution")
                .field("lines", &self.lines())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WaveResolution {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WaveResolution {{ lines: {=u32:?} }}", self.lines())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveVdVqInject(pub u32);
    impl WaveVdVqInject {
        #[doc = "Vd inject value."]
        #[must_use]
        #[inline(always)]
        pub const fn vd_val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Vd inject value."]
        #[inline(always)]
        pub const fn set_vd_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Vq inject value."]
        #[must_use]
        #[inline(always)]
        pub const fn vq_val(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Vq inject value."]
        #[inline(always)]
        pub const fn set_vq_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for WaveVdVqInject {
        #[inline(always)]
        fn default() -> WaveVdVqInject {
            WaveVdVqInject(0)
        }
    }
    impl core::fmt::Debug for WaveVdVqInject {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WaveVdVqInject")
                .field("vd_val", &self.vd_val())
                .field("vq_val", &self.vq_val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WaveVdVqInject {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "WaveVdVqInject {{ vd_val: {=u16:?}, vq_val: {=u16:?} }}",
                self.vd_val(),
                self.vq_val()
            )
        }
    }
    #[doc = "load wave0/1/2 vd vq value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveVdVqLoad(pub u32);
    impl WaveVdVqLoad {
        #[doc = "load wave0/1/2 vd vq value. always read 0 0: vd vq keep previous value. 1: load wave0/1/2 vd vq value at sametime."]
        #[must_use]
        #[inline(always)]
        pub const fn load(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "load wave0/1/2 vd vq value. always read 0 0: vd vq keep previous value. 1: load wave0/1/2 vd vq value at sametime."]
        #[inline(always)]
        pub const fn set_load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for WaveVdVqLoad {
        #[inline(always)]
        fn default() -> WaveVdVqLoad {
            WaveVdVqLoad(0)
        }
    }
    impl core::fmt::Debug for WaveVdVqLoad {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WaveVdVqLoad")
                .field("load", &self.load())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WaveVdVqLoad {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WaveVdVqLoad {{ load: {=bool:?} }}", self.load())
        }
    }
}
pub mod vals {
    #[doc = "wave_a type."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AWaveType {
        #[doc = "Two-phase orthogonality wave_a"]
        TWO_PHASE = 0x0,
        #[doc = "pulse wave of pulse/reverse type"]
        PULSE = 0x01,
        #[doc = "up wave of up/down type"]
        UP = 0x02,
        #[doc = "Three-phase orthogonality wave_a"]
        THREE_PHASE = 0x03,
    }
    impl AWaveType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AWaveType {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AWaveType {
        #[inline(always)]
        fn from(val: u8) -> AWaveType {
            AWaveType::from_bits(val)
        }
    }
    impl From<AWaveType> for u8 {
        #[inline(always)]
        fn from(val: AWaveType) -> u8 {
            AWaveType::to_bits(val)
        }
    }
    #[doc = "wave_b type."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum BWaveType {
        #[doc = "Two-phase orthogonality wave_b"]
        TWO_PHASE = 0x0,
        #[doc = "reverse wave of pulse/reverse type"]
        REVERSE = 0x01,
        #[doc = "down wave of up/down type"]
        DOWN = 0x02,
        #[doc = "Three-phase orthogonality wave_b"]
        THREE_PHASE = 0x03,
    }
    impl BWaveType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BWaveType {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BWaveType {
        #[inline(always)]
        fn from(val: u8) -> BWaveType {
            BWaveType::from_bits(val)
        }
    }
    impl From<BWaveType> for u8 {
        #[inline(always)]
        fn from(val: BWaveType) -> u8 {
            BWaveType::to_bits(val)
        }
    }
    #[doc = "PWM safety mode phase table."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum PwmMode {
        #[doc = "normal output"]
        NORMAL = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "force output 0"]
        FORCE_0 = 0x02,
        #[doc = "force output 1"]
        FORCE_1 = 0x03,
    }
    impl PwmMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> PwmMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for PwmMode {
        #[inline(always)]
        fn from(val: u8) -> PwmMode {
            PwmMode::from_bits(val)
        }
    }
    impl From<PwmMode> for u8 {
        #[inline(always)]
        fn from(val: PwmMode) -> u8 {
            PwmMode::to_bits(val)
        }
    }
    #[doc = "pulse reverse wave，reverse edge point."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ReverseEdgeType {
        #[doc = "between pulse's posedge and negedge"]
        BETWEEN_POS_NEG = 0x0,
        #[doc = "edge change point flow pulse's negedge"]
        EDGE_CHANGE_POINT = 0x01,
    }
    impl ReverseEdgeType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ReverseEdgeType {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ReverseEdgeType {
        #[inline(always)]
        fn from(val: u8) -> ReverseEdgeType {
            ReverseEdgeType::from_bits(val)
        }
    }
    impl From<ReverseEdgeType> for u8 {
        #[inline(always)]
        fn from(val: ReverseEdgeType) -> u8 {
            ReverseEdgeType::to_bits(val)
        }
    }
    #[doc = "saddle type seclect."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SaddleType {
        #[doc = "standard saddle"]
        STANDARD = 0x0,
        #[doc = "triple-cos saddle"]
        TRIPLE_COS = 0x01,
    }
    impl SaddleType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SaddleType {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SaddleType {
        #[inline(always)]
        fn from(val: u8) -> SaddleType {
            SaddleType::from_bits(val)
        }
    }
    impl From<SaddleType> for u8 {
        #[inline(always)]
        fn from(val: SaddleType) -> u8 {
            SaddleType::to_bits(val)
        }
    }
    #[doc = "wave polarity."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WavePolarity {
        #[doc = "normal output"]
        NORMAL = 0x0,
        #[doc = "invert normal output"]
        INVERT = 0x01,
    }
    impl WavePolarity {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WavePolarity {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WavePolarity {
        #[inline(always)]
        fn from(val: u8) -> WavePolarity {
            WavePolarity::from_bits(val)
        }
    }
    impl From<WavePolarity> for u8 {
        #[inline(always)]
        fn from(val: WavePolarity) -> u8 {
            WavePolarity::to_bits(val)
        }
    }
    #[doc = "wave0/1/2 output mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum WavesOutputType {
        #[doc = "cosine wave"]
        COSINE = 0x0,
        #[doc = "saddle wave"]
        SADDLE = 0x01,
        #[doc = "abs cosine wave"]
        ABS_COSINE = 0x02,
        #[doc = "saw wave"]
        SAW = 0x03,
    }
    impl WavesOutputType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WavesOutputType {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WavesOutputType {
        #[inline(always)]
        fn from(val: u8) -> WavesOutputType {
            WavesOutputType::from_bits(val)
        }
    }
    impl From<WavesOutputType> for u8 {
        #[inline(always)]
        fn from(val: WavesOutputType) -> u8 {
            WavesOutputType::to_bits(val)
        }
    }
    #[doc = "wave_z type."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum ZWaveType {
        #[doc = "zero pulse and output high at both wave_a and wave_b are high. mantain about 25% period"]
        ZERO_PULSE_HIGH_25 = 0x0,
        #[doc = "zero pulse output high about 75% period. start from 0 to 75% period"]
        ZERO_PULSE_HIGH_75 = 0x01,
        #[doc = "zero pulse output high about 100% period"]
        ZERO_PULSE_HIGH_100 = 0x02,
        #[doc = "wave_z output as tree-phase wave same as wave_a/wave_b"]
        THREE_PHASE = 0x03,
    }
    impl ZWaveType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ZWaveType {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ZWaveType {
        #[inline(always)]
        fn from(val: u8) -> ZWaveType {
            ZWaveType::from_bits(val)
        }
    }
    impl From<ZWaveType> for u8 {
        #[inline(always)]
        fn from(val: ZWaveType) -> u8 {
            ZWaveType::to_bits(val)
        }
    }
}
