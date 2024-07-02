#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PWM0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm {
    ptr: *mut u8,
}
unsafe impl Send for Pwm {}
unsafe impl Sync for Pwm {}
impl Pwm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Shadow registers unlock register."]
    #[inline(always)]
    pub const fn unlk(self) -> crate::common::Reg<regs::Unlk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Counter start register."]
    #[inline(always)]
    pub const fn sta(self) -> crate::common::Reg<regs::Sta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Counter start register."]
    #[inline(always)]
    pub const fn sta_hrpwm(self) -> crate::common::Reg<regs::StaHrpwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Counter reload register."]
    #[inline(always)]
    pub const fn rld(self) -> crate::common::Reg<regs::Rld, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Counter reload register."]
    #[inline(always)]
    pub const fn rld_hrpwm(self) -> crate::common::Reg<regs::RldHrpwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmp(self, n: usize) -> crate::common::Reg<regs::Cmp, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmp_hrpwm(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::CmpHrpwm, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 4usize) as _) }
    }
    #[doc = "Force output mode register."]
    #[inline(always)]
    pub const fn frcmd(self) -> crate::common::Reg<regs::Frcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "Shadow registers lock register."]
    #[inline(always)]
    pub const fn shlk(self) -> crate::common::Reg<regs::Shlk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn chcfg(self, n: usize) -> crate::common::Reg<regs::Chcfg, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Global control register."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "Shadow register control register."]
    #[inline(always)]
    pub const fn shcr(self) -> crate::common::Reg<regs::Shcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cappos(self, n: usize) -> crate::common::Reg<regs::Cappos, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[doc = "Counter."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn capneg(self, n: usize) -> crate::common::Reg<regs::Capneg, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize + n * 4usize) as _) }
    }
    #[doc = "Counter copy."]
    #[inline(always)]
    pub const fn cntcopy(self) -> crate::common::Reg<regs::Cntcopy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn pwmcfg(self, n: usize) -> crate::common::Reg<regs::Pwmcfg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[doc = "Interrupt request enable register."]
    #[inline(always)]
    pub const fn irqen(self) -> crate::common::Reg<regs::Irqen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0224usize) as _) }
    }
    #[doc = "DMA request enable register."]
    #[inline(always)]
    pub const fn dmaen(self) -> crate::common::Reg<regs::Dmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x022cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmpcfg(self, n: usize) -> crate::common::Reg<regs::Cmpcfg, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn anasts(self, n: usize) -> crate::common::Reg<regs::Anasts, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
    #[doc = "hrpwm config register."]
    #[inline(always)]
    pub const fn hrpwm_cfg(self) -> crate::common::Reg<regs::HrpwmCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "analog config register."]
    #[inline(always)]
    pub const fn ana_cfg0(self) -> crate::common::Reg<regs::AnaCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0424usize) as _) }
    }
}
pub mod regs {
    #[doc = "analog config register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AnaCfg0(pub u32);
    impl AnaCfg0 {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cal_sw_trig_h(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cal_sw_trig_h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for AnaCfg0 {
        #[inline(always)]
        fn default() -> AnaCfg0 {
            AnaCfg0(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Anasts(pub u32);
    impl Anasts {
        #[doc = "calibration status. will be set by hardware after setting cal_start. cleared after calibration finished."]
        #[inline(always)]
        pub const fn calon(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "calibration status. will be set by hardware after setting cal_start. cleared after calibration finished."]
        #[inline(always)]
        pub fn set_calon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Anasts {
        #[inline(always)]
        fn default() -> Anasts {
            Anasts(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Capneg(pub u32);
    impl Capneg {
        #[doc = "counter value captured at input signal falling edge."]
        #[inline(always)]
        pub const fn capneg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "counter value captured at input signal falling edge."]
        #[inline(always)]
        pub fn set_capneg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Capneg {
        #[inline(always)]
        fn default() -> Capneg {
            Capneg(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cappos(pub u32);
    impl Cappos {
        #[doc = "counter value captured at input posedge."]
        #[inline(always)]
        pub const fn cappos(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "counter value captured at input posedge."]
        #[inline(always)]
        pub fn set_cappos(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
        }
    }
    impl Default for Cappos {
        #[inline(always)]
        fn default() -> Cappos {
            Cappos(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Chcfg(pub u32);
    impl Chcfg {
        #[doc = "output polarity, set to 1 will invert the output."]
        #[inline(always)]
        pub const fn outpol(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "output polarity, set to 1 will invert the output."]
        #[inline(always)]
        pub fn set_outpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "assign the first comparator for this output channel."]
        #[inline(always)]
        pub const fn cmpselbeg(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "assign the first comparator for this output channel."]
        #[inline(always)]
        pub fn set_cmpselbeg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "assign the last comparator for this output channel."]
        #[inline(always)]
        pub const fn cmpselend(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "assign the last comparator for this output channel."]
        #[inline(always)]
        pub fn set_cmpselend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Chcfg {
        #[inline(always)]
        fn default() -> Chcfg {
            Chcfg(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmp(pub u32);
    impl Cmp {
        #[doc = "jitter counter compare value."]
        #[inline(always)]
        pub const fn cmpjit(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "jitter counter compare value."]
        #[inline(always)]
        pub fn set_cmpjit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "half clock counter compare value."]
        #[inline(always)]
        pub const fn cmphlf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "half clock counter compare value."]
        #[inline(always)]
        pub fn set_cmphlf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
        #[inline(always)]
        pub const fn cmp(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
        #[inline(always)]
        pub fn set_cmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 4usize)) | (((val as u32) & 0x00ff_ffff) << 4usize);
        }
        #[doc = "extended counter compare value."]
        #[inline(always)]
        pub const fn xcmp(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "extended counter compare value."]
        #[inline(always)]
        pub fn set_xcmp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Cmp {
        #[inline(always)]
        fn default() -> Cmp {
            Cmp(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmpHrpwm(pub u32);
    impl CmpHrpwm {
        #[doc = "high resolution compare value."]
        #[inline(always)]
        pub const fn cmp_hr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "high resolution compare value."]
        #[inline(always)]
        pub fn set_cmp_hr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn cmp(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_cmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for CmpHrpwm {
        #[inline(always)]
        fn default() -> CmpHrpwm {
            CmpHrpwm(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmpcfg(pub u32);
    impl Cmpcfg {
        #[doc = "comparator mode 0- output compare mode 1- input capture mode."]
        #[inline(always)]
        pub const fn cmpmode(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "comparator mode 0- output compare mode 1- input capture mode."]
        #[inline(always)]
        pub fn set_cmpmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert."]
        #[inline(always)]
        pub const fn cmpshdwupt(&self) -> super::vals::ShadowUpdateTrigger {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::ShadowUpdateTrigger::from_bits(val as u8)
        }
        #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert."]
        #[inline(always)]
        pub fn set_cmpshdwupt(&mut self, val: super::vals::ShadowUpdateTrigger) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
        #[inline(always)]
        pub const fn xcntcmpen(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
        #[inline(always)]
        pub fn set_xcntcmpen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Cmpcfg {
        #[inline(always)]
        fn default() -> Cmpcfg {
            Cmpcfg(0)
        }
    }
    #[doc = "Counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "current clock counter value."]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "current clock counter value."]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 4usize)) | (((val as u32) & 0x00ff_ffff) << 4usize);
        }
        #[doc = "current extended counter value."]
        #[inline(always)]
        pub const fn xcnt(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "current extended counter value."]
        #[inline(always)]
        pub fn set_xcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    #[doc = "Counter copy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntcopy(pub u32);
    impl Cntcopy {
        #[doc = "current clock counter value."]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "current clock counter value."]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 4usize)) | (((val as u32) & 0x00ff_ffff) << 4usize);
        }
        #[doc = "current extended counter value."]
        #[inline(always)]
        pub const fn xcnt(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "current extended counter value."]
        #[inline(always)]
        pub fn set_xcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Cntcopy {
        #[inline(always)]
        fn default() -> Cntcopy {
            Cntcopy(0)
        }
    }
    #[doc = "DMA request enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmaen(pub u32);
    impl Dmaen {
        #[doc = "comparator output compare or input capture flag DMA request enable."]
        #[inline(always)]
        pub const fn cmpenx(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "comparator output compare or input capture flag DMA request enable."]
        #[inline(always)]
        pub fn set_cmpenx(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "reload flag DMA request enable."]
        #[inline(always)]
        pub const fn rlden(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "reload flag DMA request enable."]
        #[inline(always)]
        pub fn set_rlden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "half reload flag DMA request enable."]
        #[inline(always)]
        pub const fn halfrlden(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "half reload flag DMA request enable."]
        #[inline(always)]
        pub fn set_halfrlden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "extended reload flag DMA request enable."]
        #[inline(always)]
        pub const fn xrlden(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "extended reload flag DMA request enable."]
        #[inline(always)]
        pub fn set_xrlden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "fault condition DMA request enable."]
        #[inline(always)]
        pub const fn faulten(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "fault condition DMA request enable."]
        #[inline(always)]
        pub fn set_faulten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Dmaen {
        #[inline(always)]
        fn default() -> Dmaen {
            Dmaen(0)
        }
    }
    #[doc = "Force output mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frcmd(pub u32);
    impl Frcmd {
        #[doc = "2bit for each PWM output channel (0-7); 00: force output 0 01: force output 1 10: output highz 11: no force."]
        #[inline(always)]
        pub const fn frcmd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "2bit for each PWM output channel (0-7); 00: force output 0 01: force output 1 10: output highz 11: no force."]
        #[inline(always)]
        pub fn set_frcmd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Frcmd {
        #[inline(always)]
        fn default() -> Frcmd {
            Frcmd(0)
        }
    }
    #[doc = "Global control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "1- write 1 to enable software force, if the frcsrcsel is set to 0, force will take effect."]
        #[inline(always)]
        pub const fn swfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1- write 1 to enable software force, if the frcsrcsel is set to 0, force will take effect."]
        #[inline(always)]
        pub fn set_swfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "This bit field select the force effective time 00: force immediately 01: force at main counter reload time 10: force at FRCSYNCI 11: no force."]
        #[inline(always)]
        pub const fn frctime(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "This bit field select the force effective time 00: force immediately 01: force at main counter reload time 10: force at FRCSYNCI 11: no force."]
        #[inline(always)]
        pub fn set_frctime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "set to clear current timer(total 28bit, main counter and tmout_count ). Auto clear."]
        #[inline(always)]
        pub const fn timerreset(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "set to clear current timer(total 28bit, main counter and tmout_count ). Auto clear."]
        #[inline(always)]
        pub fn set_timerreset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "set to enable high resolution pwm, trig_cmp, start/reload register will have different definition."]
        #[inline(always)]
        pub const fn hr_pwm_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "set to enable high resolution pwm, trig_cmp, start/reload register will have different definition."]
        #[inline(always)]
        pub fn set_hr_pwm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "1- pwm timer extended counter (xcnt) reset to extended reload value (xrld) by synci is enabled."]
        #[inline(always)]
        pub const fn xrldsyncen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "1- pwm timer extended counter (xcnt) reset to extended reload value (xrld) by synci is enabled."]
        #[inline(always)]
        pub fn set_xrldsyncen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "1- Write 1 to clear the fault condition. The output will recover if FAULTRECTIME is set to 2b'11. User should write 1 to this bit after the active FAULT signal de-assert and before it re-assert again."]
        #[inline(always)]
        pub const fn faultclr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "1- Write 1 to clear the fault condition. The output will recover if FAULTRECTIME is set to 2b'11. User should write 1 to this bit after the active FAULT signal de-assert and before it re-assert again."]
        #[inline(always)]
        pub fn set_faultclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "1- enable the pwm timer counter 0- stop the pwm timer counter."]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable the pwm timer counter 0- stop the pwm timer counter."]
        #[inline(always)]
        pub fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "1- pwm timer counter reset to reload value (rld) by synci is enabled."]
        #[inline(always)]
        pub const fn rldsyncen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1- pwm timer counter reset to reload value (rld) by synci is enabled."]
        #[inline(always)]
        pub fn set_rldsyncen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "external fault polarity 1-active low 0-active high."]
        #[inline(always)]
        pub const fn faultexpol(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "external fault polarity 1-active low 0-active high."]
        #[inline(always)]
        pub fn set_faultexpol(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "1- enable the external fault input 0."]
        #[inline(always)]
        pub const fn faulte0en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable the external fault input 0."]
        #[inline(always)]
        pub fn set_faulte0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "1- enable the external fault input 1."]
        #[inline(always)]
        pub const fn faulte1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable the external fault input 1."]
        #[inline(always)]
        pub fn set_faulte1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Selec one of the 24 comparators as fault output recover trigger."]
        #[inline(always)]
        pub const fn faultrechwsel(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x1f;
            val as u8
        }
        #[doc = "Selec one of the 24 comparators as fault output recover trigger."]
        #[inline(always)]
        pub fn set_faultrechwsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 13usize)) | (((val as u32) & 0x1f) << 13usize);
        }
        #[doc = "When hardware load is selected as output fault recover trigger and the selected channel is capture mode. This bit assign its effective edge of fault recover trigger. 1- Falling edge 0- Rising edge."]
        #[inline(always)]
        pub const fn faultrecedg(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "When hardware load is selected as output fault recover trigger and the selected channel is capture mode. This bit assign its effective edge of fault recover trigger. 1- Falling edge 0- Rising edge."]
        #[inline(always)]
        pub fn set_faultrecedg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "This bitfield select one of the comparators as hardware event time to load comparator shadow registers."]
        #[inline(always)]
        pub const fn cmpshdwsel(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x1f;
            val as u8
        }
        #[doc = "This bitfield select one of the comparators as hardware event time to load comparator shadow registers."]
        #[inline(always)]
        pub fn set_cmpshdwsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
        }
        #[doc = "When hardware event is selected as shawdow register effective time and the select comparator is configured as input capture mode. This bit assign its which edge is used as compare shadow register hardware load event. 1- Falling edge 0- Rising edge."]
        #[inline(always)]
        pub const fn hwshdwedg(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "When hardware event is selected as shawdow register effective time and the select comparator is configured as input capture mode. This bit assign its which edge is used as compare shadow register hardware load event. 1- Falling edge 0- Rising edge."]
        #[inline(always)]
        pub fn set_hwshdwedg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "polarity of input pwm_force, 1- active low 0- active high."]
        #[inline(always)]
        pub const fn frcpol(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "polarity of input pwm_force, 1- active low 0- active high."]
        #[inline(always)]
        pub fn set_frcpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "1- enable debug mode output protection."]
        #[inline(always)]
        pub const fn debugfault(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable debug mode output protection."]
        #[inline(always)]
        pub fn set_debugfault(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "1- enable the internal fault input 0."]
        #[inline(always)]
        pub const fn faulti0en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable the internal fault input 0."]
        #[inline(always)]
        pub fn set_faulti0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "1- enable the internal fault input 1."]
        #[inline(always)]
        pub const fn faulti1en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable the internal fault input 1."]
        #[inline(always)]
        pub fn set_faulti1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "1- enable the internal fault input 2."]
        #[inline(always)]
        pub const fn faulti2en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable the internal fault input 2."]
        #[inline(always)]
        pub fn set_faulti2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1- enable the internal fault input 3."]
        #[inline(always)]
        pub const fn faulti3en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable the internal fault input 3."]
        #[inline(always)]
        pub fn set_faulti3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    #[doc = "hrpwm config register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HrpwmCfg(pub u32);
    impl HrpwmCfg {
        #[doc = "calibration start. software setting this bit to start calibration process. each bit for one channel."]
        #[inline(always)]
        pub const fn cal_start(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "calibration start. software setting this bit to start calibration process. each bit for one channel."]
        #[inline(always)]
        pub fn set_cal_start(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "software calibration enable, internal use only."]
        #[inline(always)]
        pub const fn cal_sw_en(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "software calibration enable, internal use only."]
        #[inline(always)]
        pub fn set_cal_sw_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for HrpwmCfg {
        #[inline(always)]
        fn default() -> HrpwmCfg {
            HrpwmCfg(0)
        }
    }
    #[doc = "Interrupt request enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irqen(pub u32);
    impl Irqen {
        #[doc = "comparator output compare or input capture flag interrupt enable."]
        #[inline(always)]
        pub const fn cmpirqex(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "comparator output compare or input capture flag interrupt enable."]
        #[inline(always)]
        pub fn set_cmpirqex(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "reload flag interrupt enable."]
        #[inline(always)]
        pub const fn rldirqe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "reload flag interrupt enable."]
        #[inline(always)]
        pub fn set_rldirqe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "half reload flag interrupt enable."]
        #[inline(always)]
        pub const fn halfrldirqe(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "half reload flag interrupt enable."]
        #[inline(always)]
        pub fn set_halfrldirqe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "extended reload flag interrupt enable."]
        #[inline(always)]
        pub const fn xrldirqe(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "extended reload flag interrupt enable."]
        #[inline(always)]
        pub fn set_xrldirqe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "fault condition interrupt enable."]
        #[inline(always)]
        pub const fn faultirqe(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "fault condition interrupt enable."]
        #[inline(always)]
        pub fn set_faultirqe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Irqen {
        #[inline(always)]
        fn default() -> Irqen {
            Irqen(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwmcfg(pub u32);
    impl Pwmcfg {
        #[doc = "This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
        #[inline(always)]
        pub const fn deadarea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
        #[inline(always)]
        pub fn set_deadarea(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
        #[inline(always)]
        pub const fn pair(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
        #[inline(always)]
        pub fn set_pair(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1."]
        #[inline(always)]
        pub const fn frcsrcsel(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1."]
        #[inline(always)]
        pub fn set_frcsrcsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register."]
        #[inline(always)]
        pub const fn faultrectime(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register."]
        #[inline(always)]
        pub fn set_faultrectime(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz."]
        #[inline(always)]
        pub const fn faultmode(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz."]
        #[inline(always)]
        pub fn set_faultmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert."]
        #[inline(always)]
        pub const fn frcshdwupt(&self) -> super::vals::ShadowUpdateTrigger {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::ShadowUpdateTrigger::from_bits(val as u8)
        }
        #[doc = "This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert."]
        #[inline(always)]
        pub fn set_frcshdwupt(&mut self, val: super::vals::ShadowUpdateTrigger) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "PWM output enable 1- output is enabled 0- output is disabled."]
        #[inline(always)]
        pub const fn oen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PWM output enable 1- output is enabled 0- output is disabled."]
        #[inline(always)]
        pub fn set_oen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "0: update the hr value for the first edge at reload point; 1: update the hr value for the first edge at the last edge; all others will be updated at previous edge for pair mode, only pwm_cfg 0/2/4/6 are used."]
        #[inline(always)]
        pub const fn hr_update_mode(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "0: update the hr value for the first edge at reload point; 1: update the hr value for the first edge at the last edge; all others will be updated at previous edge for pair mode, only pwm_cfg 0/2/4/6 are used."]
        #[inline(always)]
        pub fn set_hr_update_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Pwmcfg {
        #[inline(always)]
        fn default() -> Pwmcfg {
            Pwmcfg(0)
        }
    }
    #[doc = "Counter reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rld(pub u32);
    impl Rld {
        #[doc = "pwm timer counter reload value."]
        #[inline(always)]
        pub const fn rld(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "pwm timer counter reload value."]
        #[inline(always)]
        pub fn set_rld(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 4usize)) | (((val as u32) & 0x00ff_ffff) << 4usize);
        }
        #[doc = "timeout counter extended reload point, counter will reload to xsta after reach this point."]
        #[inline(always)]
        pub const fn xrld(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "timeout counter extended reload point, counter will reload to xsta after reach this point."]
        #[inline(always)]
        pub fn set_xrld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Rld {
        #[inline(always)]
        fn default() -> Rld {
            Rld(0)
        }
    }
    #[doc = "Counter reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RldHrpwm(pub u32);
    impl RldHrpwm {
        #[doc = "pwm timer counter reload value at high resolution, only exist if hwpwm is enabled."]
        #[inline(always)]
        pub const fn rld_hr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "pwm timer counter reload value at high resolution, only exist if hwpwm is enabled."]
        #[inline(always)]
        pub fn set_rld_hr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn rld(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_rld(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for RldHrpwm {
        #[inline(always)]
        fn default() -> RldHrpwm {
            RldHrpwm(0)
        }
    }
    #[doc = "Shadow register control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Shcr(pub u32);
    impl Shcr {
        #[doc = "1- enable shadow registers lock feature, 0- disable shadow registers lock, shlk bit will always be 0."]
        #[inline(always)]
        pub const fn shlken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable shadow registers lock feature, 0- disable shadow registers lock, shlk bit will always be 0."]
        #[inline(always)]
        pub fn set_shlken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "This bitfield select when the counter related shadow registers (STA and RLD) will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert."]
        #[inline(always)]
        pub const fn cntshdwupt(&self) -> super::vals::ShadowUpdateTrigger {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::ShadowUpdateTrigger::from_bits(val as u8)
        }
        #[doc = "This bitfield select when the counter related shadow registers (STA and RLD) will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert."]
        #[inline(always)]
        pub fn set_cntshdwupt(&mut self, val: super::vals::ShadowUpdateTrigger) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "This bitfield select one of the comparators as hardware event time to load the counter related shadow registers (STA and RLD)."]
        #[inline(always)]
        pub const fn cntshdwsel(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "This bitfield select one of the comparators as hardware event time to load the counter related shadow registers (STA and RLD)."]
        #[inline(always)]
        pub fn set_cntshdwsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "This bitfield select one of the comparators as hardware event time to load FRCMD shadow registers."]
        #[inline(always)]
        pub const fn frcshdwsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "This bitfield select one of the comparators as hardware event time to load FRCMD shadow registers."]
        #[inline(always)]
        pub fn set_frcshdwsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "0 for posedge; 1 for negedge if hardware trigger time is selected for update_time, and selected channel is capture mode, for FRCMD shadow registers."]
        #[inline(always)]
        pub const fn force_update_edge(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "0 for posedge; 1 for negedge if hardware trigger time is selected for update_time, and selected channel is capture mode, for FRCMD shadow registers."]
        #[inline(always)]
        pub fn set_force_update_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "0 for posedge; 1 for negedge if hardware trigger time is selected for update_time, and selected channel is capture mode, for counter shadow registers."]
        #[inline(always)]
        pub const fn cnt_update_edge(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "0 for posedge; 1 for negedge if hardware trigger time is selected for update_time, and selected channel is capture mode, for counter shadow registers."]
        #[inline(always)]
        pub fn set_cnt_update_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "set to update counter working register at reload point, clear to use cnt_update_time as old version."]
        #[inline(always)]
        pub const fn cnt_update_reload(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "set to update counter working register at reload point, clear to use cnt_update_time as old version."]
        #[inline(always)]
        pub fn set_cnt_update_reload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Shcr {
        #[inline(always)]
        fn default() -> Shcr {
            Shcr(0)
        }
    }
    #[doc = "Shadow registers lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Shlk(pub u32);
    impl Shlk {
        #[doc = "write 1 to lock all shawdow register, write access is not permitted."]
        #[inline(always)]
        pub const fn shlk(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "write 1 to lock all shawdow register, write access is not permitted."]
        #[inline(always)]
        pub fn set_shlk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Shlk {
        #[inline(always)]
        fn default() -> Shlk {
            Shlk(0)
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "comparator output compare or input capture flag."]
        #[inline(always)]
        pub const fn cmpfx(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "comparator output compare or input capture flag."]
        #[inline(always)]
        pub fn set_cmpfx(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "reload flag, this flag set when cnt count to rld value or when SYNCI assert."]
        #[inline(always)]
        pub const fn rldf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "reload flag, this flag set when cnt count to rld value or when SYNCI assert."]
        #[inline(always)]
        pub fn set_rldf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "half reload flag, this flag set when cnt count to rld/2."]
        #[inline(always)]
        pub const fn halfrldf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "half reload flag, this flag set when cnt count to rld/2."]
        #[inline(always)]
        pub fn set_halfrldf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "extended reload flag, this flag set when xcnt count to xrld value or when SYNCI assert."]
        #[inline(always)]
        pub const fn xrldf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "extended reload flag, this flag set when xcnt count to xrld value or when SYNCI assert."]
        #[inline(always)]
        pub fn set_xrldf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "fault condition flag."]
        #[inline(always)]
        pub const fn faultf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "fault condition flag."]
        #[inline(always)]
        pub fn set_faultf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    #[doc = "Counter start register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sta(pub u32);
    impl Sta {
        #[doc = "pwm timer counter start value sta/rld will be loaded from shadow register to work register at main counter reload time, or software write unlk.shunlk."]
        #[inline(always)]
        pub const fn sta(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "pwm timer counter start value sta/rld will be loaded from shadow register to work register at main counter reload time, or software write unlk.shunlk."]
        #[inline(always)]
        pub fn set_sta(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 4usize)) | (((val as u32) & 0x00ff_ffff) << 4usize);
        }
        #[doc = "pwm timer counter extended start point, should back to this value after reach xrld."]
        #[inline(always)]
        pub const fn xsta(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "pwm timer counter extended start point, should back to this value after reach xrld."]
        #[inline(always)]
        pub fn set_xsta(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Sta {
        #[inline(always)]
        fn default() -> Sta {
            Sta(0)
        }
    }
    #[doc = "Counter start register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StaHrpwm(pub u32);
    impl StaHrpwm {
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn sta(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_sta(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for StaHrpwm {
        #[inline(always)]
        fn default() -> StaHrpwm {
            StaHrpwm(0)
        }
    }
    #[doc = "Shadow registers unlock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Unlk(pub u32);
    impl Unlk {
        #[doc = "write 0xB0382607 to unlock the shadow registers of register offset from 0x04 to 0x78, otherwise the shadow registers can not be written."]
        #[inline(always)]
        pub const fn shunlk(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "write 0xB0382607 to unlock the shadow registers of register offset from 0x04 to 0x78, otherwise the shadow registers can not be written."]
        #[inline(always)]
        pub fn set_shunlk(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Unlk {
        #[inline(always)]
        fn default() -> Unlk {
            Unlk(0)
        }
    }
}
pub mod vals {
    #[doc = "no description available."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ShadowUpdateTrigger {
        #[doc = "after software set shlk bit of shlk register"]
        ON_SHLK = 0x0,
        #[doc = "immediately after the register being modified"]
        ON_MODIFY = 0x01,
        #[doc = "after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode."]
        ON_HW_EVENT = 0x02,
        #[doc = "after SHSYNCI assert."]
        ON_SHSYNCI = 0x03,
    }
    impl ShadowUpdateTrigger {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ShadowUpdateTrigger {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ShadowUpdateTrigger {
        #[inline(always)]
        fn from(val: u8) -> ShadowUpdateTrigger {
            ShadowUpdateTrigger::from_bits(val)
        }
    }
    impl From<ShadowUpdateTrigger> for u8 {
        #[inline(always)]
        fn from(val: ShadowUpdateTrigger) -> u8 {
            ShadowUpdateTrigger::to_bits(val)
        }
    }
}
