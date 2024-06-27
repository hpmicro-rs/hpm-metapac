#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "VSC0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vsc {
    ptr: *mut u8,
}
unsafe impl Send for Vsc {}
unsafe impl Sync for Vsc {}
impl Vsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "abc mode."]
    #[inline(always)]
    pub const fn abc_mode(self) -> crate::common::Reg<regs::AbcMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "assign adc_chan for value_a/b/c."]
    #[inline(always)]
    pub const fn adc_chan_assign(
        self,
    ) -> crate::common::Reg<regs::AdcChanAssign, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "value_a data operation mode."]
    #[inline(always)]
    pub const fn value_a_data_opt(
        self,
    ) -> crate::common::Reg<regs::ValueADataOpt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "value_b data operation mode."]
    #[inline(always)]
    pub const fn value_b_data_opt(
        self,
    ) -> crate::common::Reg<regs::ValueBDataOpt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "value_c data operation mode."]
    #[inline(always)]
    pub const fn value_c_data_opt(
        self,
    ) -> crate::common::Reg<regs::ValueCDataOpt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "value_a offset."]
    #[inline(always)]
    pub const fn value_a_offset(self) -> crate::common::Reg<regs::ValueAOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "value_b_offset."]
    #[inline(always)]
    pub const fn value_b_offset(self) -> crate::common::Reg<regs::ValueBOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "value_c offset."]
    #[inline(always)]
    pub const fn value_c_offset(self) -> crate::common::Reg<regs::ValueCOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "irq status."]
    #[inline(always)]
    pub const fn irq_status(self) -> crate::common::Reg<regs::IrqStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "value_a software inject value."]
    #[inline(always)]
    pub const fn value_a_sw(self) -> crate::common::Reg<regs::ValueASw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "value_b software inject value."]
    #[inline(always)]
    pub const fn value_b_sw(self) -> crate::common::Reg<regs::ValueBSw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "value_c software inject value."]
    #[inline(always)]
    pub const fn value_c_sw(self) -> crate::common::Reg<regs::ValueCSw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "software inject value_a/value_b/value_c ready."]
    #[inline(always)]
    pub const fn value_sw_ready(self) -> crate::common::Reg<regs::ValueSwReady, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "software trigger event."]
    #[inline(always)]
    pub const fn trigger_sw(self) -> crate::common::Reg<regs::TriggerSw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "timestamp mode and postion capture ctrl."]
    #[inline(always)]
    pub const fn timelock(self) -> crate::common::Reg<regs::Timelock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "position software inject value."]
    #[inline(always)]
    pub const fn position_sw(self) -> crate::common::Reg<regs::PositionSw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "adc wait cycle after trigger adc capture event."]
    #[inline(always)]
    pub const fn adc_wait_cycle(self) -> crate::common::Reg<regs::AdcWaitCycle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "pos wait cycle after trigger adc capture event."]
    #[inline(always)]
    pub const fn pos_wait_cycle(self) -> crate::common::Reg<regs::PosWaitCycle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "irq bit enable."]
    #[inline(always)]
    pub const fn irq_enable(self) -> crate::common::Reg<regs::IrqEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "adc phase tolerate."]
    #[inline(always)]
    pub const fn adc_phase_tolerate(
        self,
    ) -> crate::common::Reg<regs::AdcPhaseTolerate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "position pole num."]
    #[inline(always)]
    pub const fn pos_pole(self) -> crate::common::Reg<regs::PosPole, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "posedge order Id value."]
    #[inline(always)]
    pub const fn id_posedge(self) -> crate::common::Reg<regs::IdPosedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "posedge order Iq value."]
    #[inline(always)]
    pub const fn iq_posedge(self) -> crate::common::Reg<regs::IqPosedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "negedge order Id value."]
    #[inline(always)]
    pub const fn id_negedge(self) -> crate::common::Reg<regs::IdNegedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "negedge order Iq value."]
    #[inline(always)]
    pub const fn iq_negedge(self) -> crate::common::Reg<regs::IqNegedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "posedge order alpha value."]
    #[inline(always)]
    pub const fn alpha_posedge(self) -> crate::common::Reg<regs::AlphaPosedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "posedge order beta value."]
    #[inline(always)]
    pub const fn beta_posedge(self) -> crate::common::Reg<regs::BetaPosedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "negedge order alpha value."]
    #[inline(always)]
    pub const fn alpha_negedge(self) -> crate::common::Reg<regs::AlphaNegedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "negedge order beta value."]
    #[inline(always)]
    pub const fn beta_negedge(self) -> crate::common::Reg<regs::BetaNegedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "timestamp_locked."]
    #[inline(always)]
    pub const fn timestamp_locked(
        self,
    ) -> crate::common::Reg<regs::TimestampLocked, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "debug_status0."]
    #[inline(always)]
    pub const fn debug_status0(self) -> crate::common::Reg<regs::DebugStatus0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
}
pub mod regs {
    #[doc = "abc mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbcMode(pub u32);
    impl AbcMode {
        #[doc = "enable vsc convert: 0: disable vsc convert 1: enable vsc convert."]
        #[inline(always)]
        pub const fn enable_vsc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable vsc convert: 0: disable vsc convert 1: enable vsc convert."]
        #[inline(always)]
        pub fn set_enable_vsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "the adc index of value_a: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;."]
        #[inline(always)]
        pub const fn value_a_loc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "the adc index of value_a: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;."]
        #[inline(always)]
        pub fn set_value_a_loc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "the adc index of value_b: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;."]
        #[inline(always)]
        pub const fn value_b_loc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "the adc index of value_b: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;."]
        #[inline(always)]
        pub fn set_value_b_loc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "the adc index of value_c: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;."]
        #[inline(always)]
        pub const fn value_c_loc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "the adc index of value_c: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;."]
        #[inline(always)]
        pub fn set_value_c_loc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "numbers of value_a for each convert."]
        #[inline(always)]
        pub const fn value_a_width(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "numbers of value_a for each convert."]
        #[inline(always)]
        pub fn set_value_a_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "numbers of value_b for each convert."]
        #[inline(always)]
        pub const fn value_b_width(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "numbers of value_b for each convert."]
        #[inline(always)]
        pub fn set_value_b_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "numbers of value_c for each convert."]
        #[inline(always)]
        pub const fn value_c_width(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "numbers of value_c for each convert."]
        #[inline(always)]
        pub fn set_value_c_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "whether using value_a and value_b instead of three phase."]
        #[inline(always)]
        pub const fn phase_absent_mode(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "whether using value_a and value_b instead of three phase."]
        #[inline(always)]
        pub fn set_phase_absent_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for AbcMode {
        #[inline(always)]
        fn default() -> AbcMode {
            AbcMode(0)
        }
    }
    #[doc = "assign adc_chan for value_a/b/c."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcChanAssign(pub u32);
    impl AdcChanAssign {
        #[doc = "value_a's adc chan."]
        #[inline(always)]
        pub const fn value_a_chan(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "value_a's adc chan."]
        #[inline(always)]
        pub fn set_value_a_chan(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "value_b's adc chan."]
        #[inline(always)]
        pub const fn value_b_chan(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "value_b's adc chan."]
        #[inline(always)]
        pub fn set_value_b_chan(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "value_c's adc chan."]
        #[inline(always)]
        pub const fn value_c_chan(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "value_c's adc chan."]
        #[inline(always)]
        pub fn set_value_c_chan(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for AdcChanAssign {
        #[inline(always)]
        fn default() -> AdcChanAssign {
            AdcChanAssign(0)
        }
    }
    #[doc = "adc phase tolerate."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcPhaseTolerate(pub u32);
    impl AdcPhaseTolerate {
        #[doc = "in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq."]
        #[inline(always)]
        pub const fn adc_phase_tolerate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq."]
        #[inline(always)]
        pub fn set_adc_phase_tolerate(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AdcPhaseTolerate {
        #[inline(always)]
        fn default() -> AdcPhaseTolerate {
            AdcPhaseTolerate(0)
        }
    }
    #[doc = "adc wait cycle after trigger adc capture event."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcWaitCycle(pub u32);
    impl AdcWaitCycle {
        #[doc = "adc wait cycle after trigger adc capture event."]
        #[inline(always)]
        pub const fn adc_wait_cycle(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "adc wait cycle after trigger adc capture event."]
        #[inline(always)]
        pub fn set_adc_wait_cycle(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AdcWaitCycle {
        #[inline(always)]
        fn default() -> AdcWaitCycle {
            AdcWaitCycle(0)
        }
    }
    #[doc = "negedge order alpha value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlphaNegedge(pub u32);
    impl AlphaNegedge {
        #[doc = "negedge order alpha value."]
        #[inline(always)]
        pub const fn alpha_negedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "negedge order alpha value."]
        #[inline(always)]
        pub fn set_alpha_negedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AlphaNegedge {
        #[inline(always)]
        fn default() -> AlphaNegedge {
            AlphaNegedge(0)
        }
    }
    #[doc = "posedge order alpha value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlphaPosedge(pub u32);
    impl AlphaPosedge {
        #[doc = "posedge order alpha value."]
        #[inline(always)]
        pub const fn alpha_posedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "posedge order alpha value."]
        #[inline(always)]
        pub fn set_alpha_posedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AlphaPosedge {
        #[inline(always)]
        fn default() -> AlphaPosedge {
            AlphaPosedge(0)
        }
    }
    #[doc = "negedge order beta value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BetaNegedge(pub u32);
    impl BetaNegedge {
        #[doc = "negedge order beta value."]
        #[inline(always)]
        pub const fn beta_negedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "negedge order beta value."]
        #[inline(always)]
        pub fn set_beta_negedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BetaNegedge {
        #[inline(always)]
        fn default() -> BetaNegedge {
            BetaNegedge(0)
        }
    }
    #[doc = "posedge order beta value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BetaPosedge(pub u32);
    impl BetaPosedge {
        #[doc = "posedge order beta value."]
        #[inline(always)]
        pub const fn beta_posedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "posedge order beta value."]
        #[inline(always)]
        pub fn set_beta_posedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BetaPosedge {
        #[inline(always)]
        fn default() -> BetaPosedge {
            BetaPosedge(0)
        }
    }
    #[doc = "debug_status0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DebugStatus0(pub u32);
    impl DebugStatus0 {
        #[doc = "value_c_counter."]
        #[inline(always)]
        pub const fn value_c_counter(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "value_c_counter."]
        #[inline(always)]
        pub fn set_value_c_counter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "value_b_counter."]
        #[inline(always)]
        pub const fn value_b_counter(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "value_b_counter."]
        #[inline(always)]
        pub fn set_value_b_counter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "value_a_counter."]
        #[inline(always)]
        pub const fn value_a_counter(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "value_a_counter."]
        #[inline(always)]
        pub fn set_value_a_counter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for DebugStatus0 {
        #[inline(always)]
        fn default() -> DebugStatus0 {
            DebugStatus0(0)
        }
    }
    #[doc = "negedge order Id value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IdNegedge(pub u32);
    impl IdNegedge {
        #[doc = "negedge order Id value."]
        #[inline(always)]
        pub const fn id_negedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "negedge order Id value."]
        #[inline(always)]
        pub fn set_id_negedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IdNegedge {
        #[inline(always)]
        fn default() -> IdNegedge {
            IdNegedge(0)
        }
    }
    #[doc = "posedge order Id value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IdPosedge(pub u32);
    impl IdPosedge {
        #[doc = "posedge order Id value."]
        #[inline(always)]
        pub const fn id_posedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "posedge order Id value."]
        #[inline(always)]
        pub fn set_id_posedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IdPosedge {
        #[inline(always)]
        fn default() -> IdPosedge {
            IdPosedge(0)
        }
    }
    #[doc = "negedge order Iq value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IqNegedge(pub u32);
    impl IqNegedge {
        #[doc = "negedge order Iq value."]
        #[inline(always)]
        pub const fn iq_negedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "negedge order Iq value."]
        #[inline(always)]
        pub fn set_iq_negedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IqNegedge {
        #[inline(always)]
        fn default() -> IqNegedge {
            IqNegedge(0)
        }
    }
    #[doc = "posedge order Iq value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IqPosedge(pub u32);
    impl IqPosedge {
        #[doc = "posedge order Iq value."]
        #[inline(always)]
        pub const fn iq_posedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "posedge order Iq value."]
        #[inline(always)]
        pub fn set_iq_posedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IqPosedge {
        #[inline(always)]
        fn default() -> IqPosedge {
            IqPosedge(0)
        }
    }
    #[doc = "irq bit enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnable(pub u32);
    impl IrqEnable {
        #[doc = "irq enable bit: bit0: vsc convert done irq. bit1: in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq. bit2: value_c overflow during capture process. bit3: value_b_overflow during capture process. bit4: value_a_overflow during capture process. bit5: adc2 chan not capture enough adc value. bit6: adc1 chan not capture enough adc value. bit7: adc0 chan not capture enough adc value. bit8: position not got valid before pos_wait_cycle timeout. bit9: adc2 wait cycle timeout. bit10: adc1 wait cycle timeout. bit11: adc0 wait cycle timeout. bit12: trigger_in break vsc convert even if adc or position is ready."]
        #[inline(always)]
        pub const fn irq_enable(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "irq enable bit: bit0: vsc convert done irq. bit1: in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq. bit2: value_c overflow during capture process. bit3: value_b_overflow during capture process. bit4: value_a_overflow during capture process. bit5: adc2 chan not capture enough adc value. bit6: adc1 chan not capture enough adc value. bit7: adc0 chan not capture enough adc value. bit8: position not got valid before pos_wait_cycle timeout. bit9: adc2 wait cycle timeout. bit10: adc1 wait cycle timeout. bit11: adc0 wait cycle timeout. bit12: trigger_in break vsc convert even if adc or position is ready."]
        #[inline(always)]
        pub fn set_irq_enable(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IrqEnable {
        #[inline(always)]
        fn default() -> IrqEnable {
            IrqEnable(0)
        }
    }
    #[doc = "irq status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStatus(pub u32);
    impl IrqStatus {
        #[doc = "irq status bit: bit0: vsc convert done irq. bit1: in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq. bit2: value_c overflow during capture process. bit3: value_b_overflow during capture process. bit4: value_a_overflow during capture process. bit5: adc2 chan not capture enough adc value. bit6: adc1 chan not capture enough adc value. bit7: adc0 chan not capture enough adc value. bit8: position not got valid before pos_wait_cycle timeout. bit9: adc2 wait cycle timeout. bit10: adc1 wait cycle timeout. bit11: adc0 wait cycle timeout. bit12: trigger_in break vsc convert even if adc or position is ready."]
        #[inline(always)]
        pub const fn irq_status(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "irq status bit: bit0: vsc convert done irq. bit1: in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq. bit2: value_c overflow during capture process. bit3: value_b_overflow during capture process. bit4: value_a_overflow during capture process. bit5: adc2 chan not capture enough adc value. bit6: adc1 chan not capture enough adc value. bit7: adc0 chan not capture enough adc value. bit8: position not got valid before pos_wait_cycle timeout. bit9: adc2 wait cycle timeout. bit10: adc1 wait cycle timeout. bit11: adc0 wait cycle timeout. bit12: trigger_in break vsc convert even if adc or position is ready."]
        #[inline(always)]
        pub fn set_irq_status(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IrqStatus {
        #[inline(always)]
        fn default() -> IrqStatus {
            IrqStatus(0)
        }
    }
    #[doc = "position pole num."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosPole(pub u32);
    impl PosPole {
        #[doc = "pole number."]
        #[inline(always)]
        pub const fn pos_pole(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "pole number."]
        #[inline(always)]
        pub fn set_pos_pole(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for PosPole {
        #[inline(always)]
        fn default() -> PosPole {
            PosPole(0)
        }
    }
    #[doc = "pos wait cycle after trigger adc capture event."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosWaitCycle(pub u32);
    impl PosWaitCycle {
        #[doc = "position wait cycle after trigger adc capture event."]
        #[inline(always)]
        pub const fn pos_wait_cycle(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "position wait cycle after trigger adc capture event."]
        #[inline(always)]
        pub fn set_pos_wait_cycle(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosWaitCycle {
        #[inline(always)]
        fn default() -> PosWaitCycle {
            PosWaitCycle(0)
        }
    }
    #[doc = "position software inject value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PositionSw(pub u32);
    impl PositionSw {
        #[doc = "position_sw."]
        #[inline(always)]
        pub const fn position_sw(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "position_sw."]
        #[inline(always)]
        pub fn set_position_sw(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PositionSw {
        #[inline(always)]
        fn default() -> PositionSw {
            PositionSw(0)
        }
    }
    #[doc = "timestamp mode and postion capture ctrl."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timelock(pub u32);
    impl Timelock {
        #[doc = "adc timestamp use which number index of adc_timestamp_sel used."]
        #[inline(always)]
        pub const fn value_counter_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "adc timestamp use which number index of adc_timestamp_sel used."]
        #[inline(always)]
        pub fn set_value_counter_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "adc timestamp select： 0：reserved; 1: from value_a; 2: from value_b; 3: from value_c;."]
        #[inline(always)]
        pub const fn adc_timestamp_sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "adc timestamp select： 0：reserved; 1: from value_a; 2: from value_b; 3: from value_c;."]
        #[inline(always)]
        pub fn set_adc_timestamp_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "postion capture mode: 00: position use last valid data when adc value capture finish 01: position use frist valid data after adc value capture 10: position use last valid data before adc value capture other: reserved."]
        #[inline(always)]
        pub const fn position_capture_mode(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "postion capture mode: 00: position use last valid data when adc value capture finish 01: position use frist valid data after adc value capture 10: position use last valid data before adc value capture other: reserved."]
        #[inline(always)]
        pub fn set_position_capture_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for Timelock {
        #[inline(always)]
        fn default() -> Timelock {
            Timelock(0)
        }
    }
    #[doc = "timestamp_locked."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TimestampLocked(pub u32);
    impl TimestampLocked {
        #[doc = "timestamp_locked."]
        #[inline(always)]
        pub const fn timestamp_locked(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "timestamp_locked."]
        #[inline(always)]
        pub fn set_timestamp_locked(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TimestampLocked {
        #[inline(always)]
        fn default() -> TimestampLocked {
            TimestampLocked(0)
        }
    }
    #[doc = "software trigger event."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TriggerSw(pub u32);
    impl TriggerSw {
        #[doc = "software trigger to start waiting adc capture value, same as hardwire trigger_in."]
        #[inline(always)]
        pub const fn trigger_sw(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "software trigger to start waiting adc capture value, same as hardwire trigger_in."]
        #[inline(always)]
        pub fn set_trigger_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for TriggerSw {
        #[inline(always)]
        fn default() -> TriggerSw {
            TriggerSw(0)
        }
    }
    #[doc = "value_a data operation mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueADataOpt(pub u32);
    impl ValueADataOpt {
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn opt_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub fn set_opt_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn opt_1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub fn set_opt_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn opt_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub fn set_opt_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn opt_3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub fn set_opt_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for ValueADataOpt {
        #[inline(always)]
        fn default() -> ValueADataOpt {
            ValueADataOpt(0)
        }
    }
    #[doc = "value_a offset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueAOffset(pub u32);
    impl ValueAOffset {
        #[doc = "value_a offset."]
        #[inline(always)]
        pub const fn value_a_offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "value_a offset."]
        #[inline(always)]
        pub fn set_value_a_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ValueAOffset {
        #[inline(always)]
        fn default() -> ValueAOffset {
            ValueAOffset(0)
        }
    }
    #[doc = "value_a software inject value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueASw(pub u32);
    impl ValueASw {
        #[doc = "value_a_sw."]
        #[inline(always)]
        pub const fn value_a_sw(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "value_a_sw."]
        #[inline(always)]
        pub fn set_value_a_sw(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ValueASw {
        #[inline(always)]
        fn default() -> ValueASw {
            ValueASw(0)
        }
    }
    #[doc = "value_b data operation mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueBDataOpt(pub u32);
    impl ValueBDataOpt {
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn opt_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub fn set_opt_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn opt_1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub fn set_opt_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn opt_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub fn set_opt_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn opt_3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub fn set_opt_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for ValueBDataOpt {
        #[inline(always)]
        fn default() -> ValueBDataOpt {
            ValueBDataOpt(0)
        }
    }
    #[doc = "value_b_offset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueBOffset(pub u32);
    impl ValueBOffset {
        #[doc = "value_b_offset."]
        #[inline(always)]
        pub const fn value_b_offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "value_b_offset."]
        #[inline(always)]
        pub fn set_value_b_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ValueBOffset {
        #[inline(always)]
        fn default() -> ValueBOffset {
            ValueBOffset(0)
        }
    }
    #[doc = "value_b software inject value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueBSw(pub u32);
    impl ValueBSw {
        #[doc = "value_b_sw."]
        #[inline(always)]
        pub const fn value_b_sw(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "value_b_sw."]
        #[inline(always)]
        pub fn set_value_b_sw(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ValueBSw {
        #[inline(always)]
        fn default() -> ValueBSw {
            ValueBSw(0)
        }
    }
    #[doc = "value_c data operation mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueCDataOpt(pub u32);
    impl ValueCDataOpt {
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn opt_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub fn set_opt_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn opt_1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub fn set_opt_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn opt_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub fn set_opt_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn opt_3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub fn set_opt_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for ValueCDataOpt {
        #[inline(always)]
        fn default() -> ValueCDataOpt {
            ValueCDataOpt(0)
        }
    }
    #[doc = "value_c offset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueCOffset(pub u32);
    impl ValueCOffset {
        #[doc = "value_c offset."]
        #[inline(always)]
        pub const fn value_c_offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "value_c offset."]
        #[inline(always)]
        pub fn set_value_c_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ValueCOffset {
        #[inline(always)]
        fn default() -> ValueCOffset {
            ValueCOffset(0)
        }
    }
    #[doc = "value_c software inject value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueCSw(pub u32);
    impl ValueCSw {
        #[doc = "value_c_sw."]
        #[inline(always)]
        pub const fn value_c_sw(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "value_c_sw."]
        #[inline(always)]
        pub fn set_value_c_sw(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ValueCSw {
        #[inline(always)]
        fn default() -> ValueCSw {
            ValueCSw(0)
        }
    }
    #[doc = "software inject value_a/value_b/value_c ready."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueSwReady(pub u32);
    impl ValueSwReady {
        #[doc = "software inject value_a/value_b/value_c ready."]
        #[inline(always)]
        pub const fn value_sw_ready(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "software inject value_a/value_b/value_c ready."]
        #[inline(always)]
        pub fn set_value_sw_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for ValueSwReady {
        #[inline(always)]
        fn default() -> ValueSwReady {
            ValueSwReady(0)
        }
    }
}
