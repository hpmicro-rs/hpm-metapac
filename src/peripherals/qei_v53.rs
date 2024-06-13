#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Count {
    ptr: *mut u8,
}
unsafe impl Send for Count {}
unsafe impl Sync for Count {}
impl Count {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Z counter."]
    #[inline(always)]
    pub const fn z(self) -> crate::common::Reg<regs::Z, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Phase counter."]
    #[inline(always)]
    pub const fn ph(self) -> crate::common::Reg<regs::Ph, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Speed counter."]
    #[inline(always)]
    pub const fn spd(self) -> crate::common::Reg<regs::Spd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Timer counter."]
    #[inline(always)]
    pub const fn tmr(self) -> crate::common::Reg<regs::Tmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "QEI0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qei {
    ptr: *mut u8,
}
unsafe impl Send for Qei {}
unsafe impl Sync for Qei {}
impl Qei {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Phase configure register."]
    #[inline(always)]
    pub const fn phcfg(self) -> crate::common::Reg<regs::Phcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Watchdog configure register."]
    #[inline(always)]
    pub const fn wdgcfg(self) -> crate::common::Reg<regs::Wdgcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Phase index register."]
    #[inline(always)]
    pub const fn phidx(self) -> crate::common::Reg<regs::Phidx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Tigger output enable register."]
    #[inline(always)]
    pub const fn trgoen(self) -> crate::common::Reg<regs::Trgoen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Read event enable register."]
    #[inline(always)]
    pub const fn readen(self) -> crate::common::Reg<regs::Readen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Z comparator."]
    #[inline(always)]
    pub const fn zcmp(self) -> crate::common::Reg<regs::Zcmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Phase comparator."]
    #[inline(always)]
    pub const fn phcmp(self) -> crate::common::Reg<regs::Phcmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Speed comparator."]
    #[inline(always)]
    pub const fn spdcmp(self) -> crate::common::Reg<regs::Spdcmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "DMA request enable register."]
    #[inline(always)]
    pub const fn dmaen(self) -> crate::common::Reg<regs::Dmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Interrupt request register."]
    #[inline(always)]
    pub const fn irqen(self) -> crate::common::Reg<regs::Irqen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn count(self, n: usize) -> Count {
        assert!(n < 4usize);
        unsafe { Count::from_ptr(self.ptr.add(0x30usize + n * 16usize) as _) }
    }
    #[doc = "Z comparator."]
    #[inline(always)]
    pub const fn zcmp2(self) -> crate::common::Reg<regs::Zcmp2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Phase comparator."]
    #[inline(always)]
    pub const fn phcmp2(self) -> crate::common::Reg<regs::Phcmp2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Speed comparator."]
    #[inline(always)]
    pub const fn spdcmp2(self) -> crate::common::Reg<regs::Spdcmp2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn match_cfg(self) -> crate::common::Reg<regs::MatchCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn filt_cfg(self, n: usize) -> crate::common::Reg<regs::FiltCfg, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize + n * 4usize) as _) }
    }
    #[doc = "qei config register."]
    #[inline(always)]
    pub const fn qei_cfg(self) -> crate::common::Reg<regs::QeiCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "pulse0_num."]
    #[inline(always)]
    pub const fn pulse0_num(self) -> crate::common::Reg<regs::Pulse0Num, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "pulse1_num."]
    #[inline(always)]
    pub const fn pulse1_num(self) -> crate::common::Reg<regs::Pulse1Num, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "cycle0_cnt."]
    #[inline(always)]
    pub const fn cycle0_cnt(self) -> crate::common::Reg<regs::Cycle0Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "cycle0pulse_cnt."]
    #[inline(always)]
    pub const fn cycle0pulse_cnt(
        self,
    ) -> crate::common::Reg<regs::Cycle0pulseCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "cycle1_cnt."]
    #[inline(always)]
    pub const fn cycle1_cnt(self) -> crate::common::Reg<regs::Cycle1Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "cycle1pulse_cnt."]
    #[inline(always)]
    pub const fn cycle1pulse_cnt(
        self,
    ) -> crate::common::Reg<regs::Cycle1pulseCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "cycle0_snap0."]
    #[inline(always)]
    pub const fn cycle0_snap0(self) -> crate::common::Reg<regs::Cycle0Snap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "cycle0_snap1."]
    #[inline(always)]
    pub const fn cycle0_snap1(self) -> crate::common::Reg<regs::Cycle0Snap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "cycle1_snap0."]
    #[inline(always)]
    pub const fn cycle1_snap0(self) -> crate::common::Reg<regs::Cycle1Snap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "cycle1_snap1."]
    #[inline(always)]
    pub const fn cycle1_snap1(self) -> crate::common::Reg<regs::Cycle1Snap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "cycle0_num."]
    #[inline(always)]
    pub const fn cycle0_num(self) -> crate::common::Reg<regs::Cycle0Num, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "cycle1_num."]
    #[inline(always)]
    pub const fn cycle1_num(self) -> crate::common::Reg<regs::Cycle1Num, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "pulse0_cnt."]
    #[inline(always)]
    pub const fn pulse0_cnt(self) -> crate::common::Reg<regs::Pulse0Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "pulse0cycle_cnt."]
    #[inline(always)]
    pub const fn pulse0cycle_cnt(
        self,
    ) -> crate::common::Reg<regs::Pulse0cycleCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "pulse1_cnt."]
    #[inline(always)]
    pub const fn pulse1_cnt(self) -> crate::common::Reg<regs::Pulse1Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "pulse1cycle_cnt."]
    #[inline(always)]
    pub const fn pulse1cycle_cnt(
        self,
    ) -> crate::common::Reg<regs::Pulse1cycleCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "pulse0_snap0."]
    #[inline(always)]
    pub const fn pulse0_snap0(self) -> crate::common::Reg<regs::Pulse0Snap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "pulse0cycle_snap0."]
    #[inline(always)]
    pub const fn pulse0cycle_snap0(
        self,
    ) -> crate::common::Reg<regs::Pulse0cycleSnap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "pulse0_snap1."]
    #[inline(always)]
    pub const fn pulse0_snap1(self) -> crate::common::Reg<regs::Pulse0Snap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "pulse0cycle_snap1."]
    #[inline(always)]
    pub const fn pulse0cycle_snap1(
        self,
    ) -> crate::common::Reg<regs::Pulse0cycleSnap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[doc = "pulse1_snap0."]
    #[inline(always)]
    pub const fn pulse1_snap0(self) -> crate::common::Reg<regs::Pulse1Snap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "pulse1cycle_snap0."]
    #[inline(always)]
    pub const fn pulse1cycle_snap0(
        self,
    ) -> crate::common::Reg<regs::Pulse1cycleSnap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[doc = "pulse1_snap1."]
    #[inline(always)]
    pub const fn pulse1_snap1(self) -> crate::common::Reg<regs::Pulse1Snap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[doc = "pulse1cycle_snap1."]
    #[inline(always)]
    pub const fn pulse1cycle_snap1(
        self,
    ) -> crate::common::Reg<regs::Pulse1cycleSnap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
    }
    #[doc = "adcx_cfg0."]
    #[inline(always)]
    pub const fn adcx_cfg0(self) -> crate::common::Reg<regs::AdcxCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "adcx_cfg1."]
    #[inline(always)]
    pub const fn adcx_cfg1(self) -> crate::common::Reg<regs::AdcxCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "adcx_cfg2."]
    #[inline(always)]
    pub const fn adcx_cfg2(self) -> crate::common::Reg<regs::AdcxCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "adcy_cfg0."]
    #[inline(always)]
    pub const fn adcy_cfg0(self) -> crate::common::Reg<regs::AdcyCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "adcy_cfg1."]
    #[inline(always)]
    pub const fn adcy_cfg1(self) -> crate::common::Reg<regs::AdcyCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0214usize) as _) }
    }
    #[doc = "adcy_cfg2."]
    #[inline(always)]
    pub const fn adcy_cfg2(self) -> crate::common::Reg<regs::AdcyCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[doc = "cal_cfg."]
    #[inline(always)]
    pub const fn cal_cfg(self) -> crate::common::Reg<regs::CalCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[doc = "phase_param."]
    #[inline(always)]
    pub const fn phase_param(self) -> crate::common::Reg<regs::PhaseParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize) as _) }
    }
    #[doc = "angle_adj."]
    #[inline(always)]
    pub const fn angle_adj(self) -> crate::common::Reg<regs::AngleAdj, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0234usize) as _) }
    }
    #[doc = "pos_threshold."]
    #[inline(always)]
    pub const fn pos_threshold(self) -> crate::common::Reg<regs::PosThreshold, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0238usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn uvw_pos(self, n: usize) -> crate::common::Reg<regs::UvwPos, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn uvw_pos_cfg(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UvwPosCfg, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0258usize + n * 4usize) as _) }
    }
    #[doc = "phase_cnt."]
    #[inline(always)]
    pub const fn phase_cnt(self) -> crate::common::Reg<regs::PhaseCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[doc = "phase_update."]
    #[inline(always)]
    pub const fn phase_update(self) -> crate::common::Reg<regs::PhaseUpdate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0284usize) as _) }
    }
    #[doc = "position."]
    #[inline(always)]
    pub const fn position(self) -> crate::common::Reg<regs::Position, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0288usize) as _) }
    }
    #[doc = "position_update."]
    #[inline(always)]
    pub const fn position_update(
        self,
    ) -> crate::common::Reg<regs::PositionUpdate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x028cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn angle(self) -> crate::common::Reg<regs::Angle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0290usize) as _) }
    }
    #[doc = "pos_timeout."]
    #[inline(always)]
    pub const fn pos_timeout(self) -> crate::common::Reg<regs::PosTimeout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0294usize) as _) }
    }
}
pub mod regs {
    #[doc = "adcx_cfg0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcxCfg0(pub u32);
    impl AdcxCfg0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn x_chan(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_x_chan(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn x_adc_enable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_x_adc_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn x_adcsel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_x_adcsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for AdcxCfg0 {
        #[inline(always)]
        fn default() -> AdcxCfg0 {
            AdcxCfg0(0)
        }
    }
    #[doc = "adcx_cfg1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcxCfg1(pub u32);
    impl AdcxCfg1 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn x_param0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_x_param0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn x_param1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_x_param1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for AdcxCfg1 {
        #[inline(always)]
        fn default() -> AdcxCfg1 {
            AdcxCfg1(0)
        }
    }
    #[doc = "adcx_cfg2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcxCfg2(pub u32);
    impl AdcxCfg2 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn x_offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_x_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AdcxCfg2 {
        #[inline(always)]
        fn default() -> AdcxCfg2 {
            AdcxCfg2(0)
        }
    }
    #[doc = "adcy_cfg0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcyCfg0(pub u32);
    impl AdcyCfg0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn y_chan(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_y_chan(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn y_adc_enable(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_y_adc_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn y_adcsel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_y_adcsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for AdcyCfg0 {
        #[inline(always)]
        fn default() -> AdcyCfg0 {
            AdcyCfg0(0)
        }
    }
    #[doc = "adcy_cfg1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcyCfg1(pub u32);
    impl AdcyCfg1 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn y_param0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_y_param0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn y_param1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_y_param1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for AdcyCfg1 {
        #[inline(always)]
        fn default() -> AdcyCfg1 {
            AdcyCfg1(0)
        }
    }
    #[doc = "adcy_cfg2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcyCfg2(pub u32);
    impl AdcyCfg2 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn y_offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_y_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AdcyCfg2 {
        #[inline(always)]
        fn default() -> AdcyCfg2 {
            AdcyCfg2(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Angle(pub u32);
    impl Angle {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn angle(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_angle(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Angle {
        #[inline(always)]
        fn default() -> Angle {
            Angle(0)
        }
    }
    #[doc = "angle_adj."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AngleAdj(pub u32);
    impl AngleAdj {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn angle_adj(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_angle_adj(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AngleAdj {
        #[inline(always)]
        fn default() -> AngleAdj {
            AngleAdj(0)
        }
    }
    #[doc = "cal_cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CalCfg(pub u32);
    impl CalCfg {
        #[doc = "valid x/y delay, larger than this delay will be treated as invalid data. Default 1.25us@200MHz; max 80ms;."]
        #[inline(always)]
        pub const fn xy_delay(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "valid x/y delay, larger than this delay will be treated as invalid data. Default 1.25us@200MHz; max 80ms;."]
        #[inline(always)]
        pub fn set_xy_delay(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for CalCfg {
        #[inline(always)]
        fn default() -> CalCfg {
            CalCfg(0)
        }
    }
    #[doc = "Control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "000-abz; 001-pd; 010-ud; 011-UVW(hal) 100-single A; 101-single sin; 110: sin&cos."]
        #[inline(always)]
        pub const fn enctyp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "000-abz; 001-pd; 010-ud; 011-UVW(hal) 100-single A; 101-single sin; 110: sin&cos."]
        #[inline(always)]
        pub fn set_enctyp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "define the width/counter value(affect width_match, width_match2, width_cur, timer_cur, width_read, timer_read, width_snap0,width_snap1, timer_snap0, timer_snap1) 0 : same as hpm1000/500/500s; 1: use width for position; use timer for angle."]
        #[inline(always)]
        pub const fn rd_sel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "define the width/counter value(affect width_match, width_match2, width_cur, timer_cur, width_read, timer_read, width_snap0,width_snap1, timer_snap0, timer_snap1) 0 : same as hpm1000/500/500s; 1: use width for position; use timer for angle."]
        #[inline(always)]
        pub fn set_rd_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx."]
        #[inline(always)]
        pub const fn rstcnt(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx."]
        #[inline(always)]
        pub fn set_rstcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert."]
        #[inline(always)]
        pub const fn snapen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert."]
        #[inline(always)]
        pub fn set_snapen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn faultpos(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_faultpos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction)."]
        #[inline(always)]
        pub const fn hrdir1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction)."]
        #[inline(always)]
        pub fn set_hrdir1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction)."]
        #[inline(always)]
        pub const fn hrdir0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction)."]
        #[inline(always)]
        pub fn set_hrdir0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction)."]
        #[inline(always)]
        pub const fn hfdir1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction)."]
        #[inline(always)]
        pub fn set_hfdir1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction)."]
        #[inline(always)]
        pub const fn hfdir0(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction)."]
        #[inline(always)]
        pub fn set_hfdir0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "1- pause zcnt when PAUSE assert."]
        #[inline(always)]
        pub const fn pausez(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "1- pause zcnt when PAUSE assert."]
        #[inline(always)]
        pub fn set_pausez(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "1- pause phcnt when PAUSE assert."]
        #[inline(always)]
        pub const fn pauseph(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "1- pause phcnt when PAUSE assert."]
        #[inline(always)]
        pub fn set_pauseph(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "1- pause spdcnt when PAUSE assert."]
        #[inline(always)]
        pub const fn pausespd(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "1- pause spdcnt when PAUSE assert."]
        #[inline(always)]
        pub fn set_pausespd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "1- pause position output valid when PAUSE assert."]
        #[inline(always)]
        pub const fn pausepos(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "1- pause position output valid when PAUSE assert."]
        #[inline(always)]
        pub fn set_pausepos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn h2rdir1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_h2rdir1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn h2rdir0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_h2rdir0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn h2fdir1(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_h2fdir1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn h2fdir0(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_h2fdir0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "1- phcnt will set to phidx when Z input assert(for xy analog signal and digital z, also need set phcaliz)."]
        #[inline(always)]
        pub const fn z_only_en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "1- phcnt will set to phidx when Z input assert(for xy analog signal and digital z, also need set phcaliz)."]
        #[inline(always)]
        pub fn set_z_only_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "1- phcnt will set to phidx when Z input assert(for abz digital signsl)."]
        #[inline(always)]
        pub const fn phcaliz(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "1- phcnt will set to phidx when Z input assert(for abz digital signsl)."]
        #[inline(always)]
        pub fn set_phcaliz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert."]
        #[inline(always)]
        pub const fn zcntcfg(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert."]
        #[inline(always)]
        pub fn set_zcntcfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "1- load phcnt, zcnt, spdcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0."]
        #[inline(always)]
        pub const fn read(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- load phcnt, zcnt, spdcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0."]
        #[inline(always)]
        pub fn set_read(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "cycle0_cnt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cycle0Cnt(pub u32);
    impl Cycle0Cnt {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle0_cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle0_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cycle0Cnt {
        #[inline(always)]
        fn default() -> Cycle0Cnt {
            Cycle0Cnt(0)
        }
    }
    #[doc = "cycle0_num."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cycle0Num(pub u32);
    impl Cycle0Num {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle0_num(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle0_num(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cycle0Num {
        #[inline(always)]
        fn default() -> Cycle0Num {
            Cycle0Num(0)
        }
    }
    #[doc = "cycle0_snap0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cycle0Snap0(pub u32);
    impl Cycle0Snap0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle0_snap0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle0_snap0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cycle0Snap0 {
        #[inline(always)]
        fn default() -> Cycle0Snap0 {
            Cycle0Snap0(0)
        }
    }
    #[doc = "cycle0_snap1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cycle0Snap1(pub u32);
    impl Cycle0Snap1 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle0_snap1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle0_snap1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cycle0Snap1 {
        #[inline(always)]
        fn default() -> Cycle0Snap1 {
            Cycle0Snap1(0)
        }
    }
    #[doc = "cycle0pulse_cnt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cycle0pulseCnt(pub u32);
    impl Cycle0pulseCnt {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle0pulse_cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle0pulse_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cycle0pulseCnt {
        #[inline(always)]
        fn default() -> Cycle0pulseCnt {
            Cycle0pulseCnt(0)
        }
    }
    #[doc = "cycle1_cnt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cycle1Cnt(pub u32);
    impl Cycle1Cnt {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle1_cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle1_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cycle1Cnt {
        #[inline(always)]
        fn default() -> Cycle1Cnt {
            Cycle1Cnt(0)
        }
    }
    #[doc = "cycle1_num."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cycle1Num(pub u32);
    impl Cycle1Num {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle1_num(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle1_num(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cycle1Num {
        #[inline(always)]
        fn default() -> Cycle1Num {
            Cycle1Num(0)
        }
    }
    #[doc = "cycle1_snap0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cycle1Snap0(pub u32);
    impl Cycle1Snap0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle1_snap0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle1_snap0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cycle1Snap0 {
        #[inline(always)]
        fn default() -> Cycle1Snap0 {
            Cycle1Snap0(0)
        }
    }
    #[doc = "cycle1_snap1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cycle1Snap1(pub u32);
    impl Cycle1Snap1 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle1_snap1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle1_snap1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cycle1Snap1 {
        #[inline(always)]
        fn default() -> Cycle1Snap1 {
            Cycle1Snap1(0)
        }
    }
    #[doc = "cycle1pulse_cnt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cycle1pulseCnt(pub u32);
    impl Cycle1pulseCnt {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle1pulse_cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle1pulse_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cycle1pulseCnt {
        #[inline(always)]
        fn default() -> Cycle1pulseCnt {
            Cycle1pulseCnt(0)
        }
    }
    #[doc = "DMA request enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaen(pub u32);
    impl Dmaen {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn faultfen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_faultfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn home2fen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_home2fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse1fen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse1fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse0fen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse0fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle1fen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle1fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle0fen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle0fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dirchgfen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dirchgfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pos2cmpfen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pos2cmpfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn widthtmfen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_widthtmfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn zmissfen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_zmissfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "1- generate dma request when zphf flag set."]
        #[inline(always)]
        pub const fn zphfen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when zphf flag set."]
        #[inline(always)]
        pub fn set_zphfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "1- generate dma request when poscmpf flag set."]
        #[inline(always)]
        pub const fn poscmpfen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when poscmpf flag set."]
        #[inline(always)]
        pub fn set_poscmpfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- generate dma request when homef flag set."]
        #[inline(always)]
        pub const fn homefen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when homef flag set."]
        #[inline(always)]
        pub fn set_homefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- generate dma request when wdg flag set."]
        #[inline(always)]
        pub const fn wdgfen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate dma request when wdg flag set."]
        #[inline(always)]
        pub fn set_wdgfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dmaen {
        #[inline(always)]
        fn default() -> Dmaen {
            Dmaen(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FiltCfg(pub u32);
    impl FiltCfg {
        #[doc = "This bitfields defines the filter counter length."]
        #[inline(always)]
        pub const fn filtlen(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "This bitfields defines the filter counter length."]
        #[inline(always)]
        pub fn set_filtlen(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "set to enable sychronization input signal with TRGM clock."]
        #[inline(always)]
        pub const fn syncen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable sychronization input signal with TRGM clock."]
        #[inline(always)]
        pub fn set_syncen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stable low mode; 111-stable high mode."]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stable low mode; 111-stable high mode."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "1- Filter will invert the output 0- Filter will not invert the output."]
        #[inline(always)]
        pub const fn outinv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "1- Filter will invert the output 0- Filter will not invert the output."]
        #[inline(always)]
        pub fn set_outinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for FiltCfg {
        #[inline(always)]
        fn default() -> FiltCfg {
            FiltCfg(0)
        }
    }
    #[doc = "Interrupt request register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irqen(pub u32);
    impl Irqen {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn faulte(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_faulte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn home2e(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_home2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse1e(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse0e(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse0e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle1e(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle0e(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle0e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dirchge(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dirchge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pos2cmpe(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pos2cmpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn widthtme(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_widthtme(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn zmisse(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_zmisse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "1- generate interrupt when zphf flag set."]
        #[inline(always)]
        pub const fn zphie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt when zphf flag set."]
        #[inline(always)]
        pub fn set_zphie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "1- generate interrupt when poscmpf flag set."]
        #[inline(always)]
        pub const fn poscmpie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt when poscmpf flag set."]
        #[inline(always)]
        pub fn set_poscmpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- generate interrupt when homef flag set."]
        #[inline(always)]
        pub const fn homeie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt when homef flag set."]
        #[inline(always)]
        pub fn set_homeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- generate interrupt when wdg flag set."]
        #[inline(always)]
        pub const fn wdgie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt when wdg flag set."]
        #[inline(always)]
        pub fn set_wdgie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Irqen {
        #[inline(always)]
        fn default() -> Irqen {
            Irqen(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MatchCfg(pub u32);
    impl MatchCfg {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pos_match2_opt(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pos_match2_opt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pos_match2_dir(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pos_match2_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn phase_match_dis2(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_phase_match_dis2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn spdcmp2dis(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_spdcmp2dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dircmp2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dircmp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dircmp2dis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dircmp2dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn zcmp2dis(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_zcmp2dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pos_match_opt(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pos_match_opt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pos_match_dir(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pos_match_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn phase_match_dis(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_phase_match_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn spdcmpdis(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_spdcmpdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "0- position compare need positive rotation 1- position compare need negative rotation."]
        #[inline(always)]
        pub const fn dircmp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "0- position compare need positive rotation 1- position compare need negative rotation."]
        #[inline(always)]
        pub fn set_dircmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- postion compare not include rotation direction."]
        #[inline(always)]
        pub const fn dircmpdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- postion compare not include rotation direction."]
        #[inline(always)]
        pub fn set_dircmpdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- postion compare not include zcnt."]
        #[inline(always)]
        pub const fn zcmpdis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- postion compare not include zcnt."]
        #[inline(always)]
        pub fn set_zcmpdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MatchCfg {
        #[inline(always)]
        fn default() -> MatchCfg {
            MatchCfg(0)
        }
    }
    #[doc = "Phase counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ph(pub u32);
    impl Ph {
        #[doc = "phcnt value."]
        #[inline(always)]
        pub const fn phcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x001f_ffff;
            val as u32
        }
        #[doc = "phcnt value."]
        #[inline(always)]
        pub fn set_phcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
        }
        #[doc = "1- b input is high 0- b input is low."]
        #[inline(always)]
        pub const fn bstat(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "1- b input is high 0- b input is low."]
        #[inline(always)]
        pub fn set_bstat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "1- a input is high 0- a input is low."]
        #[inline(always)]
        pub const fn astat(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "1- a input is high 0- a input is low."]
        #[inline(always)]
        pub fn set_astat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "1- reverse rotation 0- forward rotation."]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- reverse rotation 0- forward rotation."]
        #[inline(always)]
        pub fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ph {
        #[inline(always)]
        fn default() -> Ph {
            Ph(0)
        }
    }
    #[doc = "phase_cnt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhaseCnt(pub u32);
    impl PhaseCnt {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn phase_cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_phase_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PhaseCnt {
        #[inline(always)]
        fn default() -> PhaseCnt {
            PhaseCnt(0)
        }
    }
    #[doc = "phase_param."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhaseParam(pub u32);
    impl PhaseParam {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn phase_param(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_phase_param(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PhaseParam {
        #[inline(always)]
        fn default() -> PhaseParam {
            PhaseParam(0)
        }
    }
    #[doc = "phase_update."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhaseUpdate(pub u32);
    impl PhaseUpdate {
        #[doc = "value to be added or minus from phase_cnt. only valid when inc or dec is set in one 32bit write operation."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "value to be added or minus from phase_cnt. only valid when inc or dec is set in one 32bit write operation."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
        #[doc = "set to minus value from phase_cnt(set inc and dec same time willl act inc)."]
        #[inline(always)]
        pub const fn dec(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "set to minus value from phase_cnt(set inc and dec same time willl act inc)."]
        #[inline(always)]
        pub fn set_dec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "set to add value to phase_cnt."]
        #[inline(always)]
        pub const fn inc(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "set to add value to phase_cnt."]
        #[inline(always)]
        pub fn set_inc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PhaseUpdate {
        #[inline(always)]
        fn default() -> PhaseUpdate {
            PhaseUpdate(0)
        }
    }
    #[doc = "Phase configure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phcfg(pub u32);
    impl Phcfg {
        #[doc = "maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax."]
        #[inline(always)]
        pub const fn phmax(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax."]
        #[inline(always)]
        pub fn set_phmax(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Phcfg {
        #[inline(always)]
        fn default() -> Phcfg {
            Phcfg(0)
        }
    }
    #[doc = "Phase comparator."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phcmp(pub u32);
    impl Phcmp {
        #[doc = "phcnt position compare value."]
        #[inline(always)]
        pub const fn phcmp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "phcnt position compare value."]
        #[inline(always)]
        pub fn set_phcmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Phcmp {
        #[inline(always)]
        fn default() -> Phcmp {
            Phcmp(0)
        }
    }
    #[doc = "Phase comparator."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phcmp2(pub u32);
    impl Phcmp2 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn phcmp2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_phcmp2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Phcmp2 {
        #[inline(always)]
        fn default() -> Phcmp2 {
            Phcmp2(0)
        }
    }
    #[doc = "Phase index register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Phidx(pub u32);
    impl Phidx {
        #[doc = "phcnt reset value, phcnt will reset to phidx when phcaliz set to 1."]
        #[inline(always)]
        pub const fn phidx(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "phcnt reset value, phcnt will reset to phidx when phcaliz set to 1."]
        #[inline(always)]
        pub fn set_phidx(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Phidx {
        #[inline(always)]
        fn default() -> Phidx {
            Phidx(0)
        }
    }
    #[doc = "pos_threshold."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosThreshold(pub u32);
    impl PosThreshold {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pos_threshold(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pos_threshold(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosThreshold {
        #[inline(always)]
        fn default() -> PosThreshold {
            PosThreshold(0)
        }
    }
    #[doc = "pos_timeout."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosTimeout(pub u32);
    impl PosTimeout {
        #[doc = "postion timeout value."]
        #[inline(always)]
        pub const fn timeout(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "postion timeout value."]
        #[inline(always)]
        pub fn set_timeout(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "enable position timeout feature, if timeout, send valid again."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "enable position timeout feature, if timeout, send valid again."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PosTimeout {
        #[inline(always)]
        fn default() -> PosTimeout {
            PosTimeout(0)
        }
    }
    #[doc = "position."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Position(pub u32);
    impl Position {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn position(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_position(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Position {
        #[inline(always)]
        fn default() -> Position {
            Position(0)
        }
    }
    #[doc = "position_update."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PositionUpdate(pub u32);
    impl PositionUpdate {
        #[doc = "value to be added or minus from position. only valid when inc or dec is set in one 32bit write operation."]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "value to be added or minus from position. only valid when inc or dec is set in one 32bit write operation."]
        #[inline(always)]
        pub fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
        #[doc = "set to minus value from position(set inc and dec same time willl act inc)."]
        #[inline(always)]
        pub const fn dec(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "set to minus value from position(set inc and dec same time willl act inc)."]
        #[inline(always)]
        pub fn set_dec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "set to add value to position."]
        #[inline(always)]
        pub const fn inc(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "set to add value to position."]
        #[inline(always)]
        pub fn set_inc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PositionUpdate {
        #[inline(always)]
        fn default() -> PositionUpdate {
            PositionUpdate(0)
        }
    }
    #[doc = "pulse0_cnt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse0Cnt(pub u32);
    impl Pulse0Cnt {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse0_cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse0_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse0Cnt {
        #[inline(always)]
        fn default() -> Pulse0Cnt {
            Pulse0Cnt(0)
        }
    }
    #[doc = "pulse0_num."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse0Num(pub u32);
    impl Pulse0Num {
        #[doc = "for speed detection, will count the cycle number for configed pulse_num."]
        #[inline(always)]
        pub const fn pulse0_num(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "for speed detection, will count the cycle number for configed pulse_num."]
        #[inline(always)]
        pub fn set_pulse0_num(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse0Num {
        #[inline(always)]
        fn default() -> Pulse0Num {
            Pulse0Num(0)
        }
    }
    #[doc = "pulse0_snap0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse0Snap0(pub u32);
    impl Pulse0Snap0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse0_snap0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse0_snap0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse0Snap0 {
        #[inline(always)]
        fn default() -> Pulse0Snap0 {
            Pulse0Snap0(0)
        }
    }
    #[doc = "pulse0_snap1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse0Snap1(pub u32);
    impl Pulse0Snap1 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse0_snap1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse0_snap1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse0Snap1 {
        #[inline(always)]
        fn default() -> Pulse0Snap1 {
            Pulse0Snap1(0)
        }
    }
    #[doc = "pulse0cycle_cnt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse0cycleCnt(pub u32);
    impl Pulse0cycleCnt {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse0cycle_cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse0cycle_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse0cycleCnt {
        #[inline(always)]
        fn default() -> Pulse0cycleCnt {
            Pulse0cycleCnt(0)
        }
    }
    #[doc = "pulse0cycle_snap0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse0cycleSnap0(pub u32);
    impl Pulse0cycleSnap0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse0cycle_snap0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse0cycle_snap0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse0cycleSnap0 {
        #[inline(always)]
        fn default() -> Pulse0cycleSnap0 {
            Pulse0cycleSnap0(0)
        }
    }
    #[doc = "pulse0cycle_snap1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse0cycleSnap1(pub u32);
    impl Pulse0cycleSnap1 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse0cycle_snap1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse0cycle_snap1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse0cycleSnap1 {
        #[inline(always)]
        fn default() -> Pulse0cycleSnap1 {
            Pulse0cycleSnap1(0)
        }
    }
    #[doc = "pulse1_cnt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse1Cnt(pub u32);
    impl Pulse1Cnt {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse1_cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse1_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse1Cnt {
        #[inline(always)]
        fn default() -> Pulse1Cnt {
            Pulse1Cnt(0)
        }
    }
    #[doc = "pulse1_num."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse1Num(pub u32);
    impl Pulse1Num {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse1_num(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse1_num(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse1Num {
        #[inline(always)]
        fn default() -> Pulse1Num {
            Pulse1Num(0)
        }
    }
    #[doc = "pulse1_snap0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse1Snap0(pub u32);
    impl Pulse1Snap0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse1_snap0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse1_snap0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse1Snap0 {
        #[inline(always)]
        fn default() -> Pulse1Snap0 {
            Pulse1Snap0(0)
        }
    }
    #[doc = "pulse1_snap1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse1Snap1(pub u32);
    impl Pulse1Snap1 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse1_snap1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse1_snap1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse1Snap1 {
        #[inline(always)]
        fn default() -> Pulse1Snap1 {
            Pulse1Snap1(0)
        }
    }
    #[doc = "pulse1cycle_cnt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse1cycleCnt(pub u32);
    impl Pulse1cycleCnt {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse1cycle_cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse1cycle_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse1cycleCnt {
        #[inline(always)]
        fn default() -> Pulse1cycleCnt {
            Pulse1cycleCnt(0)
        }
    }
    #[doc = "pulse1cycle_snap0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse1cycleSnap0(pub u32);
    impl Pulse1cycleSnap0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse1cycle_snap0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse1cycle_snap0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse1cycleSnap0 {
        #[inline(always)]
        fn default() -> Pulse1cycleSnap0 {
            Pulse1cycleSnap0(0)
        }
    }
    #[doc = "pulse1cycle_snap1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pulse1cycleSnap1(pub u32);
    impl Pulse1cycleSnap1 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse1cycle_snap1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse1cycle_snap1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pulse1cycleSnap1 {
        #[inline(always)]
        fn default() -> Pulse1cycleSnap1 {
            Pulse1cycleSnap1(0)
        }
    }
    #[doc = "qei config register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QeiCfg(pub u32);
    impl QeiCfg {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn siga_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_siga_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sigb_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sigb_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sigz_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sigz_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn posidge_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_posidge_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "bit4: negedge enable bit3: posedge enable bit2: W in hal enable bit1: signal b(or V in hal) enable bit0: signal a(or U in hal) enable such as: 01001: use posedge A 11010: use both edge of signal B 11111: use both edge of all HAL siganls."]
        #[inline(always)]
        pub const fn negedge_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "bit4: negedge enable bit3: posedge enable bit2: W in hal enable bit1: signal b(or V in hal) enable bit0: signal a(or U in hal) enable such as: 01001: use posedge A 11010: use both edge of signal B 11111: use both edge of all HAL siganls."]
        #[inline(always)]
        pub fn set_negedge_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "set to output next area position for QEO use; clr to output exact point position for MMC use."]
        #[inline(always)]
        pub const fn uvw_pos_opt0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "set to output next area position for QEO use; clr to output exact point position for MMC use."]
        #[inline(always)]
        pub fn set_uvw_pos_opt0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "clear counter if detect direction change."]
        #[inline(always)]
        pub const fn speed_dir_chg_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "clear counter if detect direction change."]
        #[inline(always)]
        pub fn set_speed_dir_chg_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for QeiCfg {
        #[inline(always)]
        fn default() -> QeiCfg {
            QeiCfg(0)
        }
    }
    #[doc = "Read event enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Readen(pub u32);
    impl Readen {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn faultfen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_faultfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn home2fen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_home2fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse1fen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse1fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse0fen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse0fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle1fen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle1fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle0fen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle0fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dirchgfen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dirchgfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pos2cmpfen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pos2cmpfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn widthtmfen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_widthtmfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn zmissfen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_zmissfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "1- load counters to their read registers when zphf flag set."]
        #[inline(always)]
        pub const fn zphfen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when zphf flag set."]
        #[inline(always)]
        pub fn set_zphfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "1- load counters to their read registers when poscmpf flag set."]
        #[inline(always)]
        pub const fn poscmpfen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when poscmpf flag set."]
        #[inline(always)]
        pub fn set_poscmpfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- load counters to their read registers when homef flag set."]
        #[inline(always)]
        pub const fn homefen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when homef flag set."]
        #[inline(always)]
        pub fn set_homefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- load counters to their read registers when wdg flag set."]
        #[inline(always)]
        pub const fn wdgfen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- load counters to their read registers when wdg flag set."]
        #[inline(always)]
        pub fn set_wdgfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Readen {
        #[inline(always)]
        fn default() -> Readen {
            Readen(0)
        }
    }
    #[doc = "Speed counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spd(pub u32);
    impl Spd {
        #[doc = "spdcnt value."]
        #[inline(always)]
        pub const fn spdcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "spdcnt value."]
        #[inline(always)]
        pub fn set_spdcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
        #[doc = "1- b input is high 0- b input is low."]
        #[inline(always)]
        pub const fn bstat(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- b input is high 0- b input is low."]
        #[inline(always)]
        pub fn set_bstat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- a input is high 0- a input is low."]
        #[inline(always)]
        pub const fn astat(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- a input is high 0- a input is low."]
        #[inline(always)]
        pub fn set_astat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- reverse rotation 0- forward rotation."]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- reverse rotation 0- forward rotation."]
        #[inline(always)]
        pub fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Spd {
        #[inline(always)]
        fn default() -> Spd {
            Spd(0)
        }
    }
    #[doc = "Speed comparator."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spdcmp(pub u32);
    impl Spdcmp {
        #[doc = "spdcnt position compare value."]
        #[inline(always)]
        pub const fn spdcmp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "spdcnt position compare value."]
        #[inline(always)]
        pub fn set_spdcmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Spdcmp {
        #[inline(always)]
        fn default() -> Spdcmp {
            Spdcmp(0)
        }
    }
    #[doc = "Speed comparator."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spdcmp2(pub u32);
    impl Spdcmp2 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn spdcmp2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_spdcmp2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Spdcmp2 {
        #[inline(always)]
        fn default() -> Spdcmp2 {
            Spdcmp2(0)
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn faultf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_faultf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn home2f(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_home2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse1f(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse0f(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle1f(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle0f(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dirchgf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dirchgf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pos2cmpf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pos2cmpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn widthtmf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_widthtmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn zmissf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_zmissf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "z input flag."]
        #[inline(always)]
        pub const fn zphf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "z input flag."]
        #[inline(always)]
        pub fn set_zphf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "postion compare match flag."]
        #[inline(always)]
        pub const fn poscmpf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "postion compare match flag."]
        #[inline(always)]
        pub fn set_poscmpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "home flag."]
        #[inline(always)]
        pub const fn homef(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "home flag."]
        #[inline(always)]
        pub fn set_homef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "watchdog flag."]
        #[inline(always)]
        pub const fn wdgf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "watchdog flag."]
        #[inline(always)]
        pub fn set_wdgf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    #[doc = "Timer counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmr(pub u32);
    impl Tmr {
        #[doc = "32 bit free run timer."]
        #[inline(always)]
        pub const fn tmrcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "32 bit free run timer."]
        #[inline(always)]
        pub fn set_tmrcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Tmr {
        #[inline(always)]
        fn default() -> Tmr {
            Tmr(0)
        }
    }
    #[doc = "Tigger output enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trgoen(pub u32);
    impl Trgoen {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn faultfen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_faultfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn home2fen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_home2fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse1fen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse1fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pulse0fen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pulse0fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle1fen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle1fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cycle0fen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cycle0fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn dirchgfen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_dirchgfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pos2cmpfen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pos2cmpfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn widthtmfen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_widthtmfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn zmissfen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_zmissfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "1- enable trigger output when zphf flag set."]
        #[inline(always)]
        pub const fn zphfen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when zphf flag set."]
        #[inline(always)]
        pub fn set_zphfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "1- enable trigger output when poscmpf flag set."]
        #[inline(always)]
        pub const fn poscmpfen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when poscmpf flag set."]
        #[inline(always)]
        pub fn set_poscmpfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- enable trigger output when homef flag set."]
        #[inline(always)]
        pub const fn homefen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when homef flag set."]
        #[inline(always)]
        pub fn set_homefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- enable trigger output when wdg flag set."]
        #[inline(always)]
        pub const fn wdgfen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable trigger output when wdg flag set."]
        #[inline(always)]
        pub fn set_wdgfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Trgoen {
        #[inline(always)]
        fn default() -> Trgoen {
            Trgoen(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UvwPos(pub u32);
    impl UvwPos {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn uvw_pos0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_uvw_pos0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for UvwPos {
        #[inline(always)]
        fn default() -> UvwPos {
            UvwPos(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UvwPosCfg(pub u32);
    impl UvwPosCfg {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn w_pos_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_w_pos_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn v_pos_sel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_v_pos_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn u_pos_sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_u_pos_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn pos_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_pos_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for UvwPosCfg {
        #[inline(always)]
        fn default() -> UvwPosCfg {
            UvwPosCfg(0)
        }
    }
    #[doc = "Watchdog configure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdgcfg(pub u32);
    impl Wdgcfg {
        #[doc = "watch dog timeout value."]
        #[inline(always)]
        pub const fn wdgto(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "watch dog timeout value."]
        #[inline(always)]
        pub fn set_wdgto(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
        #[doc = "define as stop if phase_cnt change is less than it if 0, then each change of phase_cnt will clear wdog counter; if 2, then phase_cnt change larger than 2 will clear wdog counter."]
        #[inline(always)]
        pub const fn wdog_cfg(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "define as stop if phase_cnt change is less than it if 0, then each change of phase_cnt will clear wdog counter; if 2, then phase_cnt change larger than 2 will clear wdog counter."]
        #[inline(always)]
        pub fn set_wdog_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[doc = "1- enable wdog counter."]
        #[inline(always)]
        pub const fn wdgen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable wdog counter."]
        #[inline(always)]
        pub fn set_wdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Wdgcfg {
        #[inline(always)]
        fn default() -> Wdgcfg {
            Wdgcfg(0)
        }
    }
    #[doc = "Z counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Z(pub u32);
    impl Z {
        #[doc = "zcnt value."]
        #[inline(always)]
        pub const fn zcnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "zcnt value."]
        #[inline(always)]
        pub fn set_zcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Z {
        #[inline(always)]
        fn default() -> Z {
            Z(0)
        }
    }
    #[doc = "Z comparator."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Zcmp(pub u32);
    impl Zcmp {
        #[doc = "zcnt postion compare value."]
        #[inline(always)]
        pub const fn zcmp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "zcnt postion compare value."]
        #[inline(always)]
        pub fn set_zcmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Zcmp {
        #[inline(always)]
        fn default() -> Zcmp {
            Zcmp(0)
        }
    }
    #[doc = "Z comparator."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Zcmp2(pub u32);
    impl Zcmp2 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn zcmp2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_zcmp2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Zcmp2 {
        #[inline(always)]
        fn default() -> Zcmp2 {
            Zcmp2(0)
        }
    }
}
