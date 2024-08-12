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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "resolution of wave0/1/2."]
    #[inline(always)]
    pub const fn wave_resolution(
        self,
    ) -> crate::common::Reg<regs::WaveResolution, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wave_phase_shift(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WavePhaseShift, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "wave vd inject value."]
    #[inline(always)]
    pub const fn wave_vd_inject(self) -> crate::common::Reg<regs::WaveVdInject, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "wave vq inject value."]
    #[inline(always)]
    pub const fn wave_vq_inject(self) -> crate::common::Reg<regs::WaveVqInject, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "load wave0/1/2 vd vq value."]
    #[inline(always)]
    pub const fn wave_vd_vq_load(
        self,
    ) -> crate::common::Reg<regs::WaveVdVqLoad, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wave_amplitude(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WaveAmplitude, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wave_mid_point(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WaveMidPoint, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wave_limit0(self, n: usize) -> WaveLimit0 {
        assert!(n < 3usize);
        unsafe { WaveLimit0::from_ptr(self.ptr.add(0x48usize + n * 8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wave_limit1(self, n: usize) -> WaveLimit1 {
        assert!(n < 3usize);
        unsafe { WaveLimit1::from_ptr(self.ptr.add(0x60usize + n * 8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn wave_deadzone_shift(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::WaveDeadzoneShift, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize + n * 4usize) as _) }
    }
    #[doc = "pwm_cycle."]
    #[inline(always)]
    pub const fn wave_pwm_cycle(self) -> crate::common::Reg<regs::WavePwmCycle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "wave_a/b/z output mode."]
    #[inline(always)]
    pub const fn abz_mode(self) -> crate::common::Reg<regs::AbzMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "resolution of wave_a/b/z."]
    #[inline(always)]
    pub const fn abz_resolution(
        self,
    ) -> crate::common::Reg<regs::AbzResolution, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn abz_phase_shift(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::AbzPhaseShift, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize + n * 4usize) as _) }
    }
    #[doc = "Two-phase orthogonality wave 1/4 period."]
    #[inline(always)]
    pub const fn abz_line_width(self) -> crate::common::Reg<regs::AbzLineWidth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "wdog width of qeo."]
    #[inline(always)]
    pub const fn abz_wdog_width(self) -> crate::common::Reg<regs::AbzWdogWidth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "sync abz owned postion."]
    #[inline(always)]
    pub const fn abz_postion_sync(
        self,
    ) -> crate::common::Reg<regs::AbzPostionSync, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "abz overall position offset."]
    #[inline(always)]
    pub const fn abz_overall_offset(
        self,
    ) -> crate::common::Reg<regs::AbzOverallOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "zero phase start line num."]
    #[inline(always)]
    pub const fn abz_z_start(self) -> crate::common::Reg<regs::AbzZStart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "zero phase end line num."]
    #[inline(always)]
    pub const fn abz_z_end(self) -> crate::common::Reg<regs::AbzZEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "zero phase start and end 1/4 line num."]
    #[inline(always)]
    pub const fn abz_z_offset(self) -> crate::common::Reg<regs::AbzZOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "zero pulse witdth."]
    #[inline(always)]
    pub const fn abz_z_pulse_width(
        self,
    ) -> crate::common::Reg<regs::AbzZPulseWidth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "pwm mode."]
    #[inline(always)]
    pub const fn pwm_mode(self) -> crate::common::Reg<regs::PwmMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "resolution of pwm."]
    #[inline(always)]
    pub const fn pwm_resolution(
        self,
    ) -> crate::common::Reg<regs::PwmResolution, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pwm_phase_shift(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PwmPhaseShift, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pwm_phase_table(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PwmPhaseTable, crate::common::RW> {
        assert!(n < 24usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize + n * 4usize) as _) }
    }
    #[doc = "softwave inject postion."]
    #[inline(always)]
    pub const fn pwm_postion_software(
        self,
    ) -> crate::common::Reg<regs::PwmPostionSoftware, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f8usize) as _) }
    }
    #[doc = "select softwave inject postion."]
    #[inline(always)]
    pub const fn pwm_postion_sel(
        self,
    ) -> crate::common::Reg<regs::PwmPostionSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01fcusize) as _) }
    }
    #[doc = "qeo status."]
    #[inline(always)]
    pub const fn pwm_status(self) -> crate::common::Reg<regs::PwmStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "qeo debug 0."]
    #[inline(always)]
    pub const fn pwm_debug0(self) -> crate::common::Reg<regs::PwmDebug0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "qeo debug 1."]
    #[inline(always)]
    pub const fn pwm_debug1(self) -> crate::common::Reg<regs::PwmDebug1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "qeo debug 2."]
    #[inline(always)]
    pub const fn pwm_debug2(self) -> crate::common::Reg<regs::PwmDebug2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "qeo debug 3."]
    #[inline(always)]
    pub const fn pwm_debug3(self) -> crate::common::Reg<regs::PwmDebug3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "qeo debug 4."]
    #[inline(always)]
    pub const fn pwm_debug4(self) -> crate::common::Reg<regs::PwmDebug4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0214usize) as _) }
    }
    #[doc = "qeo debug 5."]
    #[inline(always)]
    pub const fn pwm_debug5(self) -> crate::common::Reg<regs::PwmDebug5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WaveLimit0 {
    ptr: *mut u8,
}
unsafe impl Send for WaveLimit0 {}
unsafe impl Sync for WaveLimit0 {}
impl WaveLimit0 {
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
    pub const fn min_level0(self) -> crate::common::Reg<regs::MinLevel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "wave0 high area limit value."]
    #[inline(always)]
    pub const fn max_level0(self) -> crate::common::Reg<regs::MaxLevel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WaveLimit1 {
    ptr: *mut u8,
}
unsafe impl Send for WaveLimit1 {}
unsafe impl Sync for WaveLimit1 {}
impl WaveLimit1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "wave0 low area limit value level1."]
    #[inline(always)]
    pub const fn min_level1(self) -> crate::common::Reg<regs::MinLevel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "wave0 high area limit value level1."]
    #[inline(always)]
    pub const fn max_level1(self) -> crate::common::Reg<regs::MaxLevel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "Two-phase orthogonality wave 1/4 period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzLineWidth(pub u32);
    impl AbzLineWidth {
        #[doc = "the num of system clk by 1/4 period when using as Two-phase orthogonality."]
        #[inline(always)]
        pub const fn line(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the num of system clk by 1/4 period when using as Two-phase orthogonality."]
        #[inline(always)]
        pub fn set_line(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AbzLineWidth {
        #[inline(always)]
        fn default() -> AbzLineWidth {
            AbzLineWidth(0)
        }
    }
    #[doc = "wave_a/b/z output mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzMode(pub u32);
    impl AbzMode {
        #[doc = "wave_a type: 0: Two-phase orthogonality wave_a. 1: pulse wave of pulse/reverse type. 2: up wave of up/down type. 3: Three-phase orthogonality wave_a."]
        #[inline(always)]
        pub const fn a_type(&self) -> super::vals::AWaveType {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::AWaveType::from_bits(val as u8)
        }
        #[doc = "wave_a type: 0: Two-phase orthogonality wave_a. 1: pulse wave of pulse/reverse type. 2: up wave of up/down type. 3: Three-phase orthogonality wave_a."]
        #[inline(always)]
        pub fn set_a_type(&mut self, val: super::vals::AWaveType) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "wave_b type: 0: Two-phase orthogonality wave_b. 1: reverse wave of pulse/reverse type. 2: down wave of up/down type. 3: Three-phase orthogonality wave_b."]
        #[inline(always)]
        pub const fn b_type(&self) -> super::vals::BWaveType {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::BWaveType::from_bits(val as u8)
        }
        #[doc = "wave_b type: 0: Two-phase orthogonality wave_b. 1: reverse wave of pulse/reverse type. 2: down wave of up/down type. 3: Three-phase orthogonality wave_b."]
        #[inline(always)]
        pub fn set_b_type(&mut self, val: super::vals::BWaveType) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "wave_z type: 0: zero pulse type, start and end line number decided by z_start、z_end and z_offset. 1: zero pulse type, z output start to high when position= z_start, and mantain numbers of 1/4 line cfg in z_pulse_width register 2: reserved 3: wave_z output as tree-phase wave same as wave_a/wave_b."]
        #[inline(always)]
        pub const fn z_type(&self) -> super::vals::ZWaveType {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::ZWaveType::from_bits(val as u8)
        }
        #[doc = "wave_z type: 0: zero pulse type, start and end line number decided by z_start、z_end and z_offset. 1: zero pulse type, z output start to high when position= z_start, and mantain numbers of 1/4 line cfg in z_pulse_width register 2: reserved 3: wave_z output as tree-phase wave same as wave_a/wave_b."]
        #[inline(always)]
        pub fn set_z_type(&mut self, val: super::vals::ZWaveType) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "wave_a polarity. 0: normal output. 1: invert normal output."]
        #[inline(always)]
        pub const fn a_polarity(&self) -> super::vals::WavePolarity {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::WavePolarity::from_bits(val as u8)
        }
        #[doc = "wave_a polarity. 0: normal output. 1: invert normal output."]
        #[inline(always)]
        pub fn set_a_polarity(&mut self, val: super::vals::WavePolarity) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "wave_b polarity. 0: normal output. 1: invert normal output."]
        #[inline(always)]
        pub const fn b_polarity(&self) -> super::vals::WavePolarity {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::WavePolarity::from_bits(val as u8)
        }
        #[doc = "wave_b polarity. 0: normal output. 1: invert normal output."]
        #[inline(always)]
        pub fn set_b_polarity(&mut self, val: super::vals::WavePolarity) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "wave_z polarity. 0: normal output. 1: invert normal output."]
        #[inline(always)]
        pub const fn z_polarity(&self) -> super::vals::WavePolarity {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::WavePolarity::from_bits(val as u8)
        }
        #[doc = "wave_z polarity. 0: normal output. 1: invert normal output."]
        #[inline(always)]
        pub fn set_z_polarity(&mut self, val: super::vals::WavePolarity) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "enable abz wdog: 0: disable abz wdog. 1: enable abz wdog."]
        #[inline(always)]
        pub const fn en_wdog(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "enable abz wdog: 0: disable abz wdog. 1: enable abz wdog."]
        #[inline(always)]
        pub fn set_en_wdog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "position sync mode: 0: only sync integer line part into qeo own position. 1: sync integer and fraction part into qeo own position."]
        #[inline(always)]
        pub const fn position_sync_mode(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "position sync mode: 0: only sync integer line part into qeo own position. 1: sync integer and fraction part into qeo own position."]
        #[inline(always)]
        pub fn set_position_sync_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "pulse reverse wave，reverse edge point: 0: between pulse's posedge and negedge, min period dedicated by the num line_width 1: edge change point flow pulse's negedge."]
        #[inline(always)]
        pub const fn reverse_edge_type(&self) -> super::vals::ReverseEdgeType {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::ReverseEdgeType::from_bits(val as u8)
        }
        #[doc = "pulse reverse wave，reverse edge point: 0: between pulse's posedge and negedge, min period dedicated by the num line_width 1: edge change point flow pulse's negedge."]
        #[inline(always)]
        pub fn set_reverse_edge_type(&mut self, val: super::vals::ReverseEdgeType) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "abz output enable： 0：abz output disable, all keep 0 1：abz output enable."]
        #[inline(always)]
        pub const fn abz_output_enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "abz output enable： 0：abz output disable, all keep 0 1：abz output enable."]
        #[inline(always)]
        pub fn set_abz_output_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for AbzMode {
        #[inline(always)]
        fn default() -> AbzMode {
            AbzMode(0)
        }
    }
    #[doc = "abz overall position offset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzOverallOffset(pub u32);
    impl AbzOverallOffset {
        #[doc = "abz position overall offset, it affects abz position before resolution convert."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "abz position overall offset, it affects abz position before resolution convert."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AbzOverallOffset {
        #[inline(always)]
        fn default() -> AbzOverallOffset {
            AbzOverallOffset(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzPhaseShift(pub u32);
    impl AbzPhaseShift {
        #[doc = "wave_a phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^32) period."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave_a phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^32) period."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AbzPhaseShift {
        #[inline(always)]
        fn default() -> AbzPhaseShift {
            AbzPhaseShift(0)
        }
    }
    #[doc = "sync abz owned postion."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzPostionSync(pub u32);
    impl AbzPostionSync {
        #[doc = "load next valid postion into abz owned postion. always read 0 0: sync abz owned postion with next valid postion. 1: not sync."]
        #[inline(always)]
        pub const fn postion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "load next valid postion into abz owned postion. always read 0 0: sync abz owned postion with next valid postion. 1: not sync."]
        #[inline(always)]
        pub fn set_postion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for AbzPostionSync {
        #[inline(always)]
        fn default() -> AbzPostionSync {
            AbzPostionSync(0)
        }
    }
    #[doc = "resolution of wave_a/b/z."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzResolution(pub u32);
    impl AbzResolution {
        #[doc = "wave_a/b/z resolution."]
        #[inline(always)]
        pub const fn lines(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave_a/b/z resolution."]
        #[inline(always)]
        pub fn set_lines(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AbzResolution {
        #[inline(always)]
        fn default() -> AbzResolution {
            AbzResolution(0)
        }
    }
    #[doc = "wdog width of qeo."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzWdogWidth(pub u32);
    impl AbzWdogWidth {
        #[doc = "wave will step 1/4 line to reminder user QEO still in controlled if QEO has no any toggle after the num of wdog_width sys clk."]
        #[inline(always)]
        pub const fn width(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave will step 1/4 line to reminder user QEO still in controlled if QEO has no any toggle after the num of wdog_width sys clk."]
        #[inline(always)]
        pub fn set_width(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AbzWdogWidth {
        #[inline(always)]
        fn default() -> AbzWdogWidth {
            AbzWdogWidth(0)
        }
    }
    #[doc = "zero phase end line num."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzZEnd(pub u32);
    impl AbzZEnd {
        #[doc = "number of Z end line."]
        #[inline(always)]
        pub const fn z_end(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "number of Z end line."]
        #[inline(always)]
        pub fn set_z_end(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AbzZEnd {
        #[inline(always)]
        fn default() -> AbzZEnd {
            AbzZEnd(0)
        }
    }
    #[doc = "zero phase start and end 1/4 line num."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzZOffset(pub u32);
    impl AbzZOffset {
        #[doc = "number of Z start 1/4 line."]
        #[inline(always)]
        pub const fn z_start_offset(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "number of Z start 1/4 line."]
        #[inline(always)]
        pub fn set_z_start_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "number of Z end 1/4 line."]
        #[inline(always)]
        pub const fn z_end_offset(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "number of Z end 1/4 line."]
        #[inline(always)]
        pub fn set_z_end_offset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for AbzZOffset {
        #[inline(always)]
        fn default() -> AbzZOffset {
            AbzZOffset(0)
        }
    }
    #[doc = "zero pulse witdth."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzZPulseWidth(pub u32);
    impl AbzZPulseWidth {
        #[doc = "number of z_pulse_width."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "number of z_pulse_width."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AbzZPulseWidth {
        #[inline(always)]
        fn default() -> AbzZPulseWidth {
            AbzZPulseWidth(0)
        }
    }
    #[doc = "zero phase start line num."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbzZStart(pub u32);
    impl AbzZStart {
        #[doc = "number of Z start line."]
        #[inline(always)]
        pub const fn z_start(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "number of Z start line."]
        #[inline(always)]
        pub fn set_z_start(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AbzZStart {
        #[inline(always)]
        fn default() -> AbzZStart {
            AbzZStart(0)
        }
    }
    #[doc = "wave0 high area limit value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MaxLevel0(pub u32);
    impl MaxLevel0 {
        #[doc = "high area limit level0."]
        #[inline(always)]
        pub const fn limit_level0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "high area limit level0."]
        #[inline(always)]
        pub fn set_limit_level0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MaxLevel0 {
        #[inline(always)]
        fn default() -> MaxLevel0 {
            MaxLevel0(0)
        }
    }
    #[doc = "wave0 high area limit value level1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MaxLevel1(pub u32);
    impl MaxLevel1 {
        #[doc = "high area limit level1."]
        #[inline(always)]
        pub const fn limit_level1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "high area limit level1."]
        #[inline(always)]
        pub fn set_limit_level1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MaxLevel1 {
        #[inline(always)]
        fn default() -> MaxLevel1 {
            MaxLevel1(0)
        }
    }
    #[doc = "wave0 low area limit value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MinLevel0(pub u32);
    impl MinLevel0 {
        #[doc = "low area limit level0."]
        #[inline(always)]
        pub const fn limit_level0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "low area limit level0."]
        #[inline(always)]
        pub fn set_limit_level0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MinLevel0 {
        #[inline(always)]
        fn default() -> MinLevel0 {
            MinLevel0(0)
        }
    }
    #[doc = "wave0 low area limit value level1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MinLevel1(pub u32);
    impl MinLevel1 {
        #[doc = "low area limit level1."]
        #[inline(always)]
        pub const fn limit_level1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "low area limit level1."]
        #[inline(always)]
        pub fn set_limit_level1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MinLevel1 {
        #[inline(always)]
        fn default() -> MinLevel1 {
            MinLevel1(0)
        }
    }
    #[doc = "qeo debug 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmDebug0(pub u32);
    impl PwmDebug0 {
        #[doc = "wave0."]
        #[inline(always)]
        pub const fn value_dac0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave0."]
        #[inline(always)]
        pub fn set_value_dac0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PwmDebug0 {
        #[inline(always)]
        fn default() -> PwmDebug0 {
            PwmDebug0(0)
        }
    }
    #[doc = "qeo debug 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmDebug1(pub u32);
    impl PwmDebug1 {
        #[doc = "pad_a observe."]
        #[inline(always)]
        pub const fn pad_a(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "pad_a observe."]
        #[inline(always)]
        pub fn set_pad_a(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "pad_b observe."]
        #[inline(always)]
        pub const fn pad_b(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "pad_b observe."]
        #[inline(always)]
        pub fn set_pad_b(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "pad_z observe."]
        #[inline(always)]
        pub const fn pad_z(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "pad_z observe."]
        #[inline(always)]
        pub fn set_pad_z(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "qeo finish observe."]
        #[inline(always)]
        pub const fn qeo_finish(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "qeo finish observe."]
        #[inline(always)]
        pub fn set_qeo_finish(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for PwmDebug1 {
        #[inline(always)]
        fn default() -> PwmDebug1 {
            PwmDebug1(0)
        }
    }
    #[doc = "qeo debug 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmDebug2(pub u32);
    impl PwmDebug2 {
        #[doc = "abz_own_postion observe."]
        #[inline(always)]
        pub const fn abz_own_postion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "abz_own_postion observe."]
        #[inline(always)]
        pub fn set_abz_own_postion(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PwmDebug2 {
        #[inline(always)]
        fn default() -> PwmDebug2 {
            PwmDebug2(0)
        }
    }
    #[doc = "qeo debug 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmDebug3(pub u32);
    impl PwmDebug3 {
        #[doc = "abz_own_postion observe."]
        #[inline(always)]
        pub const fn abz_own_postion(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "abz_own_postion observe."]
        #[inline(always)]
        pub fn set_abz_own_postion(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PwmDebug3 {
        #[inline(always)]
        fn default() -> PwmDebug3 {
            PwmDebug3(0)
        }
    }
    #[doc = "qeo debug 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmDebug4(pub u32);
    impl PwmDebug4 {
        #[doc = "wave1."]
        #[inline(always)]
        pub const fn value_dac1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave1."]
        #[inline(always)]
        pub fn set_value_dac1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PwmDebug4 {
        #[inline(always)]
        fn default() -> PwmDebug4 {
            PwmDebug4(0)
        }
    }
    #[doc = "qeo debug 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmDebug5(pub u32);
    impl PwmDebug5 {
        #[doc = "wave2."]
        #[inline(always)]
        pub const fn value_dac2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave2."]
        #[inline(always)]
        pub fn set_value_dac2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PwmDebug5 {
        #[inline(always)]
        fn default() -> PwmDebug5 {
            PwmDebug5(0)
        }
    }
    #[doc = "pwm mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmMode(pub u32);
    impl PwmMode {
        #[doc = "pwm force phase number."]
        #[inline(always)]
        pub const fn phase_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "pwm force phase number."]
        #[inline(always)]
        pub fn set_phase_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "exchange PWM pairs’ output 0: not exchange. 1: exchange."]
        #[inline(always)]
        pub const fn revise_up_dn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "exchange PWM pairs’ output 0: not exchange. 1: exchange."]
        #[inline(always)]
        pub fn set_revise_up_dn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PWM safety mode bypass 0: not bypass 1: bypass."]
        #[inline(always)]
        pub const fn pwm_safety_bypass(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PWM safety mode bypass 0: not bypass 1: bypass."]
        #[inline(always)]
        pub fn set_pwm_safety_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PWM enter safety mode 0: not enter 1: enter."]
        #[inline(always)]
        pub const fn pwm_enter_safety_mode(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PWM enter safety mode 0: not enter 1: enter."]
        #[inline(always)]
        pub fn set_pwm_enter_safety_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "enable PWM force output 0: disable 1: enable."]
        #[inline(always)]
        pub const fn enable_pwm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "enable PWM force output 0: disable 1: enable."]
        #[inline(always)]
        pub fn set_enable_pwm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PWM safety mode phase table."]
        #[inline(always)]
        pub const fn pwm_safety(&self, n: usize) -> super::vals::PwmMode {
            assert!(n < 8usize);
            let offs = 16usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::PwmMode::from_bits(val as u8)
        }
        #[doc = "PWM safety mode phase table."]
        #[inline(always)]
        pub fn set_pwm_safety(&mut self, n: usize, val: super::vals::PwmMode) {
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
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmPhaseShift(pub u32);
    impl PwmPhaseShift {
        #[doc = "pwm_a phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^32) period."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "pwm_a phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^32) period."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PwmPhaseShift {
        #[inline(always)]
        fn default() -> PwmPhaseShift {
            PwmPhaseShift(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmPhaseTable(pub u32);
    impl PwmPhaseTable {
        #[doc = "pwm phase table value."]
        #[inline(always)]
        pub const fn pwm(&self, n: usize) -> super::vals::PwmMode {
            assert!(n < 8usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::PwmMode::from_bits(val as u8)
        }
        #[doc = "pwm phase table value."]
        #[inline(always)]
        pub fn set_pwm(&mut self, n: usize, val: super::vals::PwmMode) {
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
    #[doc = "select softwave inject postion."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmPostionSel(pub u32);
    impl PwmPostionSel {
        #[doc = "enable softwave inject postion. 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn postion_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable softwave inject postion. 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_postion_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PwmPostionSel {
        #[inline(always)]
        fn default() -> PwmPostionSel {
            PwmPostionSel(0)
        }
    }
    #[doc = "softwave inject postion."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmPostionSoftware(pub u32);
    impl PwmPostionSoftware {
        #[doc = "softwave inject postion."]
        #[inline(always)]
        pub const fn postion_softwave(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "softwave inject postion."]
        #[inline(always)]
        pub fn set_postion_softwave(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PwmPostionSoftware {
        #[inline(always)]
        fn default() -> PwmPostionSoftware {
            PwmPostionSoftware(0)
        }
    }
    #[doc = "resolution of pwm."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmResolution(pub u32);
    impl PwmResolution {
        #[doc = "pwm resolution."]
        #[inline(always)]
        pub const fn lines(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "pwm resolution."]
        #[inline(always)]
        pub fn set_lines(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PwmResolution {
        #[inline(always)]
        fn default() -> PwmResolution {
            PwmResolution(0)
        }
    }
    #[doc = "qeo status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwmStatus(pub u32);
    impl PwmStatus {
        #[doc = "pwm_fault status."]
        #[inline(always)]
        pub const fn pwm_safety(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "pwm_fault status."]
        #[inline(always)]
        pub fn set_pwm_safety(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "qeo_pwm_force observe."]
        #[inline(always)]
        pub const fn pwm_fource(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "qeo_pwm_force observe."]
        #[inline(always)]
        pub fn set_pwm_fource(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for PwmStatus {
        #[inline(always)]
        fn default() -> PwmStatus {
            PwmStatus(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveAmplitude(pub u32);
    impl WaveAmplitude {
        #[doc = "amplitude scaling value. bit15-12 are integer part value. bit11-0 are fraction value."]
        #[inline(always)]
        pub const fn amp_val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "amplitude scaling value. bit15-12 are integer part value. bit11-0 are fraction value."]
        #[inline(always)]
        pub fn set_amp_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "enable wave amplitude scaling. 0: disable; 1: enable."]
        #[inline(always)]
        pub const fn en_scal(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "enable wave amplitude scaling. 0: disable; 1: enable."]
        #[inline(always)]
        pub fn set_en_scal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for WaveAmplitude {
        #[inline(always)]
        fn default() -> WaveAmplitude {
            WaveAmplitude(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveDeadzoneShift(pub u32);
    impl WaveDeadzoneShift {
        #[doc = "wave0 deadzone shifter value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave0 deadzone shifter value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WaveDeadzoneShift {
        #[inline(always)]
        fn default() -> WaveDeadzoneShift {
            WaveDeadzoneShift(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveMidPoint(pub u32);
    impl WaveMidPoint {
        #[doc = "wave0 output middle point, use this value as 32 bit signed value. bit 31 is signed bit. bit30-27 is integer part value. bit26-0 is fraction value."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave0 output middle point, use this value as 32 bit signed value. bit 31 is signed bit. bit30-27 is integer part value. bit26-0 is fraction value."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WaveMidPoint {
        #[inline(always)]
        fn default() -> WaveMidPoint {
            WaveMidPoint(0)
        }
    }
    #[doc = "analog waves mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveMode(pub u32);
    impl WaveMode {
        #[doc = "wave0/1/2 output mode. 0: cosine wave. 1: saddle wave. 2. abs cosine wave. 3. saw wave."]
        #[inline(always)]
        pub const fn waves_output_type(&self) -> super::vals::WavesOutputType {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::WavesOutputType::from_bits(val as u8)
        }
        #[doc = "wave0/1/2 output mode. 0: cosine wave. 1: saddle wave. 2. abs cosine wave. 3. saw wave."]
        #[inline(always)]
        pub fn set_waves_output_type(&mut self, val: super::vals::WavesOutputType) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "vd_vq sel ctrl: 0: from CLC. 1: from software."]
        #[inline(always)]
        pub const fn vd_vq_sel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "vd_vq sel ctrl: 0: from CLC. 1: from software."]
        #[inline(always)]
        pub fn set_vd_vq_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "wave VdVq inject enable. 0: disable VdVq inject. 1: enable VdVq inject."]
        #[inline(always)]
        pub const fn en_wave_vd_vq_inject(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "wave VdVq inject enable. 0: disable VdVq inject. 1: enable VdVq inject."]
        #[inline(always)]
        pub fn set_en_wave_vd_vq_inject(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "enable position valid to trigger analog wave calcuation 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn enable_pos_valid(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable position valid to trigger analog wave calcuation 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_enable_pos_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "enable vd or vq valid to trigger analog wave calcuation 0: disable. 1: enable."]
        #[inline(always)]
        pub const fn enable_dq_valid(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable vd or vq valid to trigger analog wave calcuation 0: disable. 1: enable."]
        #[inline(always)]
        pub fn set_enable_dq_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "saddle type seclect; 0:standard saddle. 1: triple-cos saddle."]
        #[inline(always)]
        pub const fn saddle_type(&self) -> super::vals::SaddleType {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::SaddleType::from_bits(val as u8)
        }
        #[doc = "saddle type seclect; 0:standard saddle. 1: triple-cos saddle."]
        #[inline(always)]
        pub fn set_saddle_type(&mut self, val: super::vals::SaddleType) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "wave0 below min limit mode. 0: output 0. 1: output all bits are 1. 2: output as level_min_limit0.level1_min_limit."]
        #[inline(always)]
        pub const fn wave0_below_min_limit(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "wave0 below min limit mode. 0: output 0. 1: output all bits are 1. 2: output as level_min_limit0.level1_min_limit."]
        #[inline(always)]
        pub fn set_wave0_below_min_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "wave0 low area0 limit mode. 0: output 0. 1: output as level_min_limit0.level1_min_limit."]
        #[inline(always)]
        pub const fn wave0_low_area0_limit(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "wave0 low area0 limit mode. 0: output 0. 1: output as level_min_limit0.level1_min_limit."]
        #[inline(always)]
        pub fn set_wave0_low_area0_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "wave0 low area1 limit mode. 0: output 0. 1: output as level_min_limit0.level1_min_limit."]
        #[inline(always)]
        pub const fn wave0_low_area1_limit(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "wave0 low area1 limit mode. 0: output 0. 1: output as level_min_limit0.level1_min_limit."]
        #[inline(always)]
        pub fn set_wave0_low_area1_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "wave0 high area0 limit mode. 0: output all bits are 1. 1: output as level_max_limit0.level0_max_limit."]
        #[inline(always)]
        pub const fn wave0_high_area0_limit(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "wave0 high area0 limit mode. 0: output all bits are 1. 1: output as level_max_limit0.level0_max_limit."]
        #[inline(always)]
        pub fn set_wave0_high_area0_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "wave0 high area1 limit mode. 0: output all bits are 1. 1: output as level_max_limit0.level0_max_limit."]
        #[inline(always)]
        pub const fn wave0_high_area1_limit(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "wave0 high area1 limit mode. 0: output all bits are 1. 1: output as level_max_limit0.level0_max_limit."]
        #[inline(always)]
        pub fn set_wave0_high_area1_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "wave0 above max limit mode. 0: output all bits are 1. 1: output 0x0. 2: output as level_max_limit0.level0_max_limit."]
        #[inline(always)]
        pub const fn wave0_above_max_limit(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "wave0 above max limit mode. 0: output all bits are 1. 1: output 0x0. 2: output as level_max_limit0.level0_max_limit."]
        #[inline(always)]
        pub fn set_wave0_above_max_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "wave1 below min limit mode. 0: output 0. 1: output all bits are 1. 2: output as level_min_limit1.level1_min_limit."]
        #[inline(always)]
        pub const fn wave1_below_min_limit(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "wave1 below min limit mode. 0: output 0. 1: output all bits are 1. 2: output as level_min_limit1.level1_min_limit."]
        #[inline(always)]
        pub fn set_wave1_below_min_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "wave1 low area0 limit mode. 0: output 0. 1: output as level_min_limit1.level1_min_limit."]
        #[inline(always)]
        pub const fn wave1_low_area0_limit(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "wave1 low area0 limit mode. 0: output 0. 1: output as level_min_limit1.level1_min_limit."]
        #[inline(always)]
        pub fn set_wave1_low_area0_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "wave1 low area1 limit mode. 0: output 0. 1: output as level_min_limit1.level1_min_limit."]
        #[inline(always)]
        pub const fn wave1_low_area1_limit(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "wave1 low area1 limit mode. 0: output 0. 1: output as level_min_limit1.level1_min_limit."]
        #[inline(always)]
        pub fn set_wave1_low_area1_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "wave1 high area0 limit mode. 0: output all bits are 1. 1: output as level_max_limit1.level0_max_limit."]
        #[inline(always)]
        pub const fn wave1_high_area0_limit(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "wave1 high area0 limit mode. 0: output all bits are 1. 1: output as level_max_limit1.level0_max_limit."]
        #[inline(always)]
        pub fn set_wave1_high_area0_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "wave1 high area1 limit mode. 0: output all bits are 1. 1: output as level_max_limit1.level0_max_limit."]
        #[inline(always)]
        pub const fn wave1_high_area1_limit(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "wave1 high area1 limit mode. 0: output all bits are 1. 1: output as level_max_limit1.level0_max_limit."]
        #[inline(always)]
        pub fn set_wave1_high_area1_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "wave1 above max limit mode. 0: output all bits are 1. 1: output 0x0. 2: output as level_max_limit1.level0_max_limit."]
        #[inline(always)]
        pub const fn wave1_above_max_limit(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "wave1 above max limit mode. 0: output all bits are 1. 1: output 0x0. 2: output as level_max_limit1.level0_max_limit."]
        #[inline(always)]
        pub fn set_wave1_above_max_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "wave2 below min limit mode. 0: output 0. 1: output all bits are 1. 2: output as level_min_limit2.level1_min_limit."]
        #[inline(always)]
        pub const fn wave2_below_min_limit(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "wave2 below min limit mode. 0: output 0. 1: output all bits are 1. 2: output as level_min_limit2.level1_min_limit."]
        #[inline(always)]
        pub fn set_wave2_below_min_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "wave2 low area0 limit mode. 0: output 0. 1: output as level_min_limit2.level1_min_limit."]
        #[inline(always)]
        pub const fn wave2_low_area0_limit(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "wave2 low area0 limit mode. 0: output 0. 1: output as level_min_limit2.level1_min_limit."]
        #[inline(always)]
        pub fn set_wave2_low_area0_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "wave2 low area1 limit mode. 0: output 0. 1: output as level_min_limit2.level1_min_limit."]
        #[inline(always)]
        pub const fn wave2_low_area1_limit(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "wave2 low area1 limit mode. 0: output 0. 1: output as level_min_limit2.level1_min_limit."]
        #[inline(always)]
        pub fn set_wave2_low_area1_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "wave2 high area0 limit mode. 0: output all bits are 1. 1: output as level_max_limit2.level0_max_limit."]
        #[inline(always)]
        pub const fn wave2_high_area0_limit(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "wave2 high area0 limit mode. 0: output all bits are 1. 1: output as level_max_limit2.level0_max_limit."]
        #[inline(always)]
        pub fn set_wave2_high_area0_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "wave2 high area1 limit mode. 0: output all bits are 1. 1: output as level_max_limit2.level0_max_limit."]
        #[inline(always)]
        pub const fn wave2_high_area1_limit(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "wave2 high area1 limit mode. 0: output all bits are 1. 1: output as level_max_limit2.level0_max_limit."]
        #[inline(always)]
        pub fn set_wave2_high_area1_limit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "wave2 above max limit mode. 0: output all bits are 1. 1: output 0x0. 2: output as level_max_limit2.level0_max_limit."]
        #[inline(always)]
        pub const fn wave2_above_max_limit(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "wave2 above max limit mode. 0: output all bits are 1. 1: output 0x0. 2: output as level_max_limit2.level0_max_limit."]
        #[inline(always)]
        pub fn set_wave2_above_max_limit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for WaveMode {
        #[inline(always)]
        fn default() -> WaveMode {
            WaveMode(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WavePhaseShift(pub u32);
    impl WavePhaseShift {
        #[doc = "wave0 phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^32) period."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave0 phase shifter value, default is 0x0. write other value will shift phase early as (cfg_value/2^32) period."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WavePhaseShift {
        #[inline(always)]
        fn default() -> WavePhaseShift {
            WavePhaseShift(0)
        }
    }
    #[doc = "pwm_cycle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WavePwmCycle(pub u32);
    impl WavePwmCycle {
        #[doc = "pwm_cycle."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "pwm_cycle."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WavePwmCycle {
        #[inline(always)]
        fn default() -> WavePwmCycle {
            WavePwmCycle(0)
        }
    }
    #[doc = "resolution of wave0/1/2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveResolution(pub u32);
    impl WaveResolution {
        #[doc = "wave0/1/2 resolution."]
        #[inline(always)]
        pub const fn lines(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "wave0/1/2 resolution."]
        #[inline(always)]
        pub fn set_lines(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WaveResolution {
        #[inline(always)]
        fn default() -> WaveResolution {
            WaveResolution(0)
        }
    }
    #[doc = "wave vd inject value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveVdInject(pub u32);
    impl WaveVdInject {
        #[doc = "Vd inject value."]
        #[inline(always)]
        pub const fn vd_val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Vd inject value."]
        #[inline(always)]
        pub fn set_vd_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WaveVdInject {
        #[inline(always)]
        fn default() -> WaveVdInject {
            WaveVdInject(0)
        }
    }
    #[doc = "load wave0/1/2 vd vq value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveVdVqLoad(pub u32);
    impl WaveVdVqLoad {
        #[doc = "load wave0/1/2 vd vq value. always read 0 0: vd vq keep previous value. 1: load wave0/1/2 vd vq value at sametime."]
        #[inline(always)]
        pub const fn load(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "load wave0/1/2 vd vq value. always read 0 0: vd vq keep previous value. 1: load wave0/1/2 vd vq value at sametime."]
        #[inline(always)]
        pub fn set_load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for WaveVdVqLoad {
        #[inline(always)]
        fn default() -> WaveVdVqLoad {
            WaveVdVqLoad(0)
        }
    }
    #[doc = "wave vq inject value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WaveVqInject(pub u32);
    impl WaveVqInject {
        #[doc = "Vq inject value."]
        #[inline(always)]
        pub const fn vq_val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Vq inject value."]
        #[inline(always)]
        pub fn set_vq_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WaveVqInject {
        #[inline(always)]
        fn default() -> WaveVqInject {
            WaveVqInject(0)
        }
    }
}
pub mod vals {
    #[doc = "wave_a type."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
