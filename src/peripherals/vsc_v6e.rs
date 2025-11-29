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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "assign adc_chan for value_a/b/c."]
    #[inline(always)]
    pub const fn adc_chan_assign(
        self,
    ) -> crate::common::Reg<regs::AdcChanAssign, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "value_a data operation mode."]
    #[inline(always)]
    pub const fn value_a_data_opt(
        self,
    ) -> crate::common::Reg<regs::ValueADataOpt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "value_b data operation mode."]
    #[inline(always)]
    pub const fn value_b_data_opt(
        self,
    ) -> crate::common::Reg<regs::ValueBDataOpt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "value_c data operation mode."]
    #[inline(always)]
    pub const fn value_c_data_opt(
        self,
    ) -> crate::common::Reg<regs::ValueCDataOpt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "value_a offset."]
    #[inline(always)]
    pub const fn value_a_offset(self) -> crate::common::Reg<regs::ValueAOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "value_b_offset."]
    #[inline(always)]
    pub const fn value_b_offset(self) -> crate::common::Reg<regs::ValueBOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "value_c offset."]
    #[inline(always)]
    pub const fn value_c_offset(self) -> crate::common::Reg<regs::ValueCOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "irq status."]
    #[inline(always)]
    pub const fn irq_status(self) -> crate::common::Reg<regs::IrqStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "value_a software inject value."]
    #[inline(always)]
    pub const fn value_a_sw(self) -> crate::common::Reg<regs::ValueASw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "value_b software inject value."]
    #[inline(always)]
    pub const fn value_b_sw(self) -> crate::common::Reg<regs::ValueBSw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "value_c software inject value."]
    #[inline(always)]
    pub const fn value_c_sw(self) -> crate::common::Reg<regs::ValueCSw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "software inject value_a/value_b/value_c ready."]
    #[inline(always)]
    pub const fn value_sw_ready(self) -> crate::common::Reg<regs::ValueSwReady, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "software trigger event."]
    #[inline(always)]
    pub const fn trigger_sw(self) -> crate::common::Reg<regs::TriggerSw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "timestamp mode and postion capture ctrl."]
    #[inline(always)]
    pub const fn timelock(self) -> crate::common::Reg<regs::Timelock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "position software inject value."]
    #[inline(always)]
    pub const fn position_sw(self) -> crate::common::Reg<regs::PositionSw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "adc wait cycle after trigger adc capture event."]
    #[inline(always)]
    pub const fn adc_wait_cycle(self) -> crate::common::Reg<regs::AdcWaitCycle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "pos wait cycle after trigger adc capture event."]
    #[inline(always)]
    pub const fn pos_wait_cycle(self) -> crate::common::Reg<regs::PosWaitCycle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "irq bit enable."]
    #[inline(always)]
    pub const fn irq_enable(self) -> crate::common::Reg<regs::IrqEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "adc phase tolerate."]
    #[inline(always)]
    pub const fn adc_phase_tolerate(
        self,
    ) -> crate::common::Reg<regs::AdcPhaseTolerate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "position pole num."]
    #[inline(always)]
    pub const fn pos_pole(self) -> crate::common::Reg<regs::PosPole, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "posedge order Id value."]
    #[inline(always)]
    pub const fn id_posedge(self) -> crate::common::Reg<regs::IdPosedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "posedge order Iq value."]
    #[inline(always)]
    pub const fn iq_posedge(self) -> crate::common::Reg<regs::IqPosedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "negedge order Id value."]
    #[inline(always)]
    pub const fn id_negedge(self) -> crate::common::Reg<regs::IdNegedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "negedge order Iq value."]
    #[inline(always)]
    pub const fn iq_negedge(self) -> crate::common::Reg<regs::IqNegedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "posedge order alpha value."]
    #[inline(always)]
    pub const fn alpha_posedge(self) -> crate::common::Reg<regs::AlphaPosedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "posedge order beta value."]
    #[inline(always)]
    pub const fn beta_posedge(self) -> crate::common::Reg<regs::BetaPosedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "negedge order alpha value."]
    #[inline(always)]
    pub const fn alpha_negedge(self) -> crate::common::Reg<regs::AlphaNegedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "negedge order beta value."]
    #[inline(always)]
    pub const fn beta_negedge(self) -> crate::common::Reg<regs::BetaNegedge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "timestamp_locked."]
    #[inline(always)]
    pub const fn timestamp_locked(
        self,
    ) -> crate::common::Reg<regs::TimestampLocked, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "debug_status0."]
    #[inline(always)]
    pub const fn debug_status0(self) -> crate::common::Reg<regs::DebugStatus0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
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
    #[doc = "abc mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbcMode(pub u32);
    impl AbcMode {
        #[doc = "enable vsc convert: 0: disable vsc convert 1: enable vsc convert."]
        #[must_use]
        #[inline(always)]
        pub const fn enable_vsc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable vsc convert: 0: disable vsc convert 1: enable vsc convert."]
        #[inline(always)]
        pub const fn set_enable_vsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "the adc index of value_a: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;."]
        #[must_use]
        #[inline(always)]
        pub const fn value_a_loc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "the adc index of value_a: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;."]
        #[inline(always)]
        pub const fn set_value_a_loc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "the adc index of value_b: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;."]
        #[must_use]
        #[inline(always)]
        pub const fn value_b_loc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "the adc index of value_b: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;."]
        #[inline(always)]
        pub const fn set_value_b_loc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "the adc index of value_c: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;."]
        #[must_use]
        #[inline(always)]
        pub const fn value_c_loc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "the adc index of value_c: 2'b:00: resevered; 2'b:01: from adc0; 2'b:10: from adc1; 2'b:11: from adc2;."]
        #[inline(always)]
        pub const fn set_value_c_loc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "numbers of value_a for each convert."]
        #[must_use]
        #[inline(always)]
        pub const fn value_a_width(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "numbers of value_a for each convert."]
        #[inline(always)]
        pub const fn set_value_a_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "numbers of value_b for each convert."]
        #[must_use]
        #[inline(always)]
        pub const fn value_b_width(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "numbers of value_b for each convert."]
        #[inline(always)]
        pub const fn set_value_b_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "numbers of value_c for each convert."]
        #[must_use]
        #[inline(always)]
        pub const fn value_c_width(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "numbers of value_c for each convert."]
        #[inline(always)]
        pub const fn set_value_c_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "whether using value_a and value_b instead of three phase."]
        #[must_use]
        #[inline(always)]
        pub const fn phase_absent_mode(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "whether using value_a and value_b instead of three phase."]
        #[inline(always)]
        pub const fn set_phase_absent_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for AbcMode {
        #[inline(always)]
        fn default() -> AbcMode {
            AbcMode(0)
        }
    }
    impl core::fmt::Debug for AbcMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AbcMode")
                .field("enable_vsc", &self.enable_vsc())
                .field("value_a_loc", &self.value_a_loc())
                .field("value_b_loc", &self.value_b_loc())
                .field("value_c_loc", &self.value_c_loc())
                .field("value_a_width", &self.value_a_width())
                .field("value_b_width", &self.value_b_width())
                .field("value_c_width", &self.value_c_width())
                .field("phase_absent_mode", &self.phase_absent_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AbcMode {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AbcMode {{ enable_vsc: {=bool:?}, value_a_loc: {=u8:?}, value_b_loc: {=u8:?}, value_c_loc: {=u8:?}, value_a_width: {=u8:?}, value_b_width: {=u8:?}, value_c_width: {=u8:?}, phase_absent_mode: {=bool:?} }}" , self . enable_vsc () , self . value_a_loc () , self . value_b_loc () , self . value_c_loc () , self . value_a_width () , self . value_b_width () , self . value_c_width () , self . phase_absent_mode ())
        }
    }
    #[doc = "assign adc_chan for value_a/b/c."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcChanAssign(pub u32);
    impl AdcChanAssign {
        #[doc = "value_a's adc chan."]
        #[must_use]
        #[inline(always)]
        pub const fn value_a_chan(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "value_a's adc chan."]
        #[inline(always)]
        pub const fn set_value_a_chan(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "value_b's adc chan."]
        #[must_use]
        #[inline(always)]
        pub const fn value_b_chan(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "value_b's adc chan."]
        #[inline(always)]
        pub const fn set_value_b_chan(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "value_c's adc chan."]
        #[must_use]
        #[inline(always)]
        pub const fn value_c_chan(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "value_c's adc chan."]
        #[inline(always)]
        pub const fn set_value_c_chan(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for AdcChanAssign {
        #[inline(always)]
        fn default() -> AdcChanAssign {
            AdcChanAssign(0)
        }
    }
    impl core::fmt::Debug for AdcChanAssign {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcChanAssign")
                .field("value_a_chan", &self.value_a_chan())
                .field("value_b_chan", &self.value_b_chan())
                .field("value_c_chan", &self.value_c_chan())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AdcChanAssign {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AdcChanAssign {{ value_a_chan: {=u8:?}, value_b_chan: {=u8:?}, value_c_chan: {=u8:?} }}" , self . value_a_chan () , self . value_b_chan () , self . value_c_chan ())
        }
    }
    #[doc = "adc phase tolerate."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcPhaseTolerate(pub u32);
    impl AdcPhaseTolerate {
        #[doc = "in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq."]
        #[must_use]
        #[inline(always)]
        pub const fn adc_phase_tolerate(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq."]
        #[inline(always)]
        pub const fn set_adc_phase_tolerate(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AdcPhaseTolerate {
        #[inline(always)]
        fn default() -> AdcPhaseTolerate {
            AdcPhaseTolerate(0)
        }
    }
    impl core::fmt::Debug for AdcPhaseTolerate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcPhaseTolerate")
                .field("adc_phase_tolerate", &self.adc_phase_tolerate())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AdcPhaseTolerate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AdcPhaseTolerate {{ adc_phase_tolerate: {=u32:?} }}",
                self.adc_phase_tolerate()
            )
        }
    }
    #[doc = "adc wait cycle after trigger adc capture event."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdcWaitCycle(pub u32);
    impl AdcWaitCycle {
        #[doc = "adc wait cycle after trigger adc capture event."]
        #[must_use]
        #[inline(always)]
        pub const fn adc_wait_cycle(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "adc wait cycle after trigger adc capture event."]
        #[inline(always)]
        pub const fn set_adc_wait_cycle(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AdcWaitCycle {
        #[inline(always)]
        fn default() -> AdcWaitCycle {
            AdcWaitCycle(0)
        }
    }
    impl core::fmt::Debug for AdcWaitCycle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AdcWaitCycle")
                .field("adc_wait_cycle", &self.adc_wait_cycle())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AdcWaitCycle {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AdcWaitCycle {{ adc_wait_cycle: {=u32:?} }}",
                self.adc_wait_cycle()
            )
        }
    }
    #[doc = "negedge order alpha value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlphaNegedge(pub u32);
    impl AlphaNegedge {
        #[doc = "negedge order alpha value."]
        #[must_use]
        #[inline(always)]
        pub const fn alpha_negedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "negedge order alpha value."]
        #[inline(always)]
        pub const fn set_alpha_negedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AlphaNegedge {
        #[inline(always)]
        fn default() -> AlphaNegedge {
            AlphaNegedge(0)
        }
    }
    impl core::fmt::Debug for AlphaNegedge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AlphaNegedge")
                .field("alpha_negedge", &self.alpha_negedge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlphaNegedge {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AlphaNegedge {{ alpha_negedge: {=u32:?} }}",
                self.alpha_negedge()
            )
        }
    }
    #[doc = "posedge order alpha value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlphaPosedge(pub u32);
    impl AlphaPosedge {
        #[doc = "posedge order alpha value."]
        #[must_use]
        #[inline(always)]
        pub const fn alpha_posedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "posedge order alpha value."]
        #[inline(always)]
        pub const fn set_alpha_posedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for AlphaPosedge {
        #[inline(always)]
        fn default() -> AlphaPosedge {
            AlphaPosedge(0)
        }
    }
    impl core::fmt::Debug for AlphaPosedge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AlphaPosedge")
                .field("alpha_posedge", &self.alpha_posedge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlphaPosedge {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AlphaPosedge {{ alpha_posedge: {=u32:?} }}",
                self.alpha_posedge()
            )
        }
    }
    #[doc = "negedge order beta value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BetaNegedge(pub u32);
    impl BetaNegedge {
        #[doc = "negedge order beta value."]
        #[must_use]
        #[inline(always)]
        pub const fn beta_negedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "negedge order beta value."]
        #[inline(always)]
        pub const fn set_beta_negedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BetaNegedge {
        #[inline(always)]
        fn default() -> BetaNegedge {
            BetaNegedge(0)
        }
    }
    impl core::fmt::Debug for BetaNegedge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BetaNegedge")
                .field("beta_negedge", &self.beta_negedge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BetaNegedge {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BetaNegedge {{ beta_negedge: {=u32:?} }}",
                self.beta_negedge()
            )
        }
    }
    #[doc = "posedge order beta value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BetaPosedge(pub u32);
    impl BetaPosedge {
        #[doc = "posedge order beta value."]
        #[must_use]
        #[inline(always)]
        pub const fn beta_posedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "posedge order beta value."]
        #[inline(always)]
        pub const fn set_beta_posedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BetaPosedge {
        #[inline(always)]
        fn default() -> BetaPosedge {
            BetaPosedge(0)
        }
    }
    impl core::fmt::Debug for BetaPosedge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BetaPosedge")
                .field("beta_posedge", &self.beta_posedge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BetaPosedge {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BetaPosedge {{ beta_posedge: {=u32:?} }}",
                self.beta_posedge()
            )
        }
    }
    #[doc = "debug_status0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DebugStatus0(pub u32);
    impl DebugStatus0 {
        #[doc = "value_c_counter."]
        #[must_use]
        #[inline(always)]
        pub const fn value_c_counter(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "value_c_counter."]
        #[inline(always)]
        pub const fn set_value_c_counter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "value_b_counter."]
        #[must_use]
        #[inline(always)]
        pub const fn value_b_counter(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "value_b_counter."]
        #[inline(always)]
        pub const fn set_value_b_counter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "value_a_counter."]
        #[must_use]
        #[inline(always)]
        pub const fn value_a_counter(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "value_a_counter."]
        #[inline(always)]
        pub const fn set_value_a_counter(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for DebugStatus0 {
        #[inline(always)]
        fn default() -> DebugStatus0 {
            DebugStatus0(0)
        }
    }
    impl core::fmt::Debug for DebugStatus0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DebugStatus0")
                .field("value_c_counter", &self.value_c_counter())
                .field("value_b_counter", &self.value_b_counter())
                .field("value_a_counter", &self.value_a_counter())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DebugStatus0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DebugStatus0 {{ value_c_counter: {=u8:?}, value_b_counter: {=u8:?}, value_a_counter: {=u8:?} }}" , self . value_c_counter () , self . value_b_counter () , self . value_a_counter ())
        }
    }
    #[doc = "negedge order Id value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IdNegedge(pub u32);
    impl IdNegedge {
        #[doc = "negedge order Id value."]
        #[must_use]
        #[inline(always)]
        pub const fn id_negedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "negedge order Id value."]
        #[inline(always)]
        pub const fn set_id_negedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IdNegedge {
        #[inline(always)]
        fn default() -> IdNegedge {
            IdNegedge(0)
        }
    }
    impl core::fmt::Debug for IdNegedge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IdNegedge")
                .field("id_negedge", &self.id_negedge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IdNegedge {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IdNegedge {{ id_negedge: {=u32:?} }}", self.id_negedge())
        }
    }
    #[doc = "posedge order Id value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IdPosedge(pub u32);
    impl IdPosedge {
        #[doc = "posedge order Id value."]
        #[must_use]
        #[inline(always)]
        pub const fn id_posedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "posedge order Id value."]
        #[inline(always)]
        pub const fn set_id_posedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IdPosedge {
        #[inline(always)]
        fn default() -> IdPosedge {
            IdPosedge(0)
        }
    }
    impl core::fmt::Debug for IdPosedge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IdPosedge")
                .field("id_posedge", &self.id_posedge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IdPosedge {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IdPosedge {{ id_posedge: {=u32:?} }}", self.id_posedge())
        }
    }
    #[doc = "negedge order Iq value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IqNegedge(pub u32);
    impl IqNegedge {
        #[doc = "negedge order Iq value."]
        #[must_use]
        #[inline(always)]
        pub const fn iq_negedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "negedge order Iq value."]
        #[inline(always)]
        pub const fn set_iq_negedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IqNegedge {
        #[inline(always)]
        fn default() -> IqNegedge {
            IqNegedge(0)
        }
    }
    impl core::fmt::Debug for IqNegedge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IqNegedge")
                .field("iq_negedge", &self.iq_negedge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IqNegedge {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IqNegedge {{ iq_negedge: {=u32:?} }}", self.iq_negedge())
        }
    }
    #[doc = "posedge order Iq value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IqPosedge(pub u32);
    impl IqPosedge {
        #[doc = "posedge order Iq value."]
        #[must_use]
        #[inline(always)]
        pub const fn iq_posedge(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "posedge order Iq value."]
        #[inline(always)]
        pub const fn set_iq_posedge(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IqPosedge {
        #[inline(always)]
        fn default() -> IqPosedge {
            IqPosedge(0)
        }
    }
    impl core::fmt::Debug for IqPosedge {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IqPosedge")
                .field("iq_posedge", &self.iq_posedge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IqPosedge {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IqPosedge {{ iq_posedge: {=u32:?} }}", self.iq_posedge())
        }
    }
    #[doc = "irq bit enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqEnable(pub u32);
    impl IrqEnable {
        #[doc = "irq enable bit: bit0: vsc convert done irq. bit1: in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq. bit2: value_c overflow during capture process. bit3: value_b_overflow during capture process. bit4: value_a_overflow during capture process. bit5: adc2 chan not capture enough adc value. bit6: adc1 chan not capture enough adc value. bit7: adc0 chan not capture enough adc value. bit8: position not got valid before pos_wait_cycle timeout. bit9: adc2 wait cycle timeout. bit10: adc1 wait cycle timeout. bit11: adc0 wait cycle timeout. bit12: trigger_in break vsc convert even if adc or position is ready."]
        #[must_use]
        #[inline(always)]
        pub const fn irq_enable(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "irq enable bit: bit0: vsc convert done irq. bit1: in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq. bit2: value_c overflow during capture process. bit3: value_b_overflow during capture process. bit4: value_a_overflow during capture process. bit5: adc2 chan not capture enough adc value. bit6: adc1 chan not capture enough adc value. bit7: adc0 chan not capture enough adc value. bit8: position not got valid before pos_wait_cycle timeout. bit9: adc2 wait cycle timeout. bit10: adc1 wait cycle timeout. bit11: adc0 wait cycle timeout. bit12: trigger_in break vsc convert even if adc or position is ready."]
        #[inline(always)]
        pub const fn set_irq_enable(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IrqEnable {
        #[inline(always)]
        fn default() -> IrqEnable {
            IrqEnable(0)
        }
    }
    impl core::fmt::Debug for IrqEnable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqEnable")
                .field("irq_enable", &self.irq_enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqEnable {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqEnable {{ irq_enable: {=u32:?} }}", self.irq_enable())
        }
    }
    #[doc = "irq status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqStatus(pub u32);
    impl IrqStatus {
        #[doc = "irq status bit: bit0: vsc convert done irq. bit1: in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq. bit2: value_c overflow during capture process. bit3: value_b_overflow during capture process. bit4: value_a_overflow during capture process. bit5: adc2 chan not capture enough adc value. bit6: adc1 chan not capture enough adc value. bit7: adc0 chan not capture enough adc value. bit8: position not got valid before pos_wait_cycle timeout. bit9: adc2 wait cycle timeout. bit10: adc1 wait cycle timeout. bit11: adc0 wait cycle timeout. bit12: trigger_in break vsc convert even if adc or position is ready."]
        #[must_use]
        #[inline(always)]
        pub const fn irq_status(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "irq status bit: bit0: vsc convert done irq. bit1: in adc three-phase mode, if ABS(value_a+value_b+value_c) > adc_phase_tolerate, will trigger irq. bit2: value_c overflow during capture process. bit3: value_b_overflow during capture process. bit4: value_a_overflow during capture process. bit5: adc2 chan not capture enough adc value. bit6: adc1 chan not capture enough adc value. bit7: adc0 chan not capture enough adc value. bit8: position not got valid before pos_wait_cycle timeout. bit9: adc2 wait cycle timeout. bit10: adc1 wait cycle timeout. bit11: adc0 wait cycle timeout. bit12: trigger_in break vsc convert even if adc or position is ready."]
        #[inline(always)]
        pub const fn set_irq_status(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IrqStatus {
        #[inline(always)]
        fn default() -> IrqStatus {
            IrqStatus(0)
        }
    }
    impl core::fmt::Debug for IrqStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqStatus")
                .field("irq_status", &self.irq_status())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqStatus {{ irq_status: {=u32:?} }}", self.irq_status())
        }
    }
    #[doc = "position pole num."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosPole(pub u32);
    impl PosPole {
        #[doc = "pole number."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_pole(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "pole number."]
        #[inline(always)]
        pub const fn set_pos_pole(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for PosPole {
        #[inline(always)]
        fn default() -> PosPole {
            PosPole(0)
        }
    }
    impl core::fmt::Debug for PosPole {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosPole")
                .field("pos_pole", &self.pos_pole())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosPole {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosPole {{ pos_pole: {=u16:?} }}", self.pos_pole())
        }
    }
    #[doc = "pos wait cycle after trigger adc capture event."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosWaitCycle(pub u32);
    impl PosWaitCycle {
        #[doc = "position wait cycle after trigger adc capture event."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_wait_cycle(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "position wait cycle after trigger adc capture event."]
        #[inline(always)]
        pub const fn set_pos_wait_cycle(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosWaitCycle {
        #[inline(always)]
        fn default() -> PosWaitCycle {
            PosWaitCycle(0)
        }
    }
    impl core::fmt::Debug for PosWaitCycle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosWaitCycle")
                .field("pos_wait_cycle", &self.pos_wait_cycle())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosWaitCycle {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PosWaitCycle {{ pos_wait_cycle: {=u32:?} }}",
                self.pos_wait_cycle()
            )
        }
    }
    #[doc = "position software inject value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PositionSw(pub u32);
    impl PositionSw {
        #[doc = "position_sw."]
        #[must_use]
        #[inline(always)]
        pub const fn position_sw(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "position_sw."]
        #[inline(always)]
        pub const fn set_position_sw(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PositionSw {
        #[inline(always)]
        fn default() -> PositionSw {
            PositionSw(0)
        }
    }
    impl core::fmt::Debug for PositionSw {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PositionSw")
                .field("position_sw", &self.position_sw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PositionSw {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PositionSw {{ position_sw: {=u32:?} }}",
                self.position_sw()
            )
        }
    }
    #[doc = "timestamp mode and postion capture ctrl."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timelock(pub u32);
    impl Timelock {
        #[doc = "adc timestamp use which number index of adc_timestamp_sel used."]
        #[must_use]
        #[inline(always)]
        pub const fn value_counter_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "adc timestamp use which number index of adc_timestamp_sel used."]
        #[inline(always)]
        pub const fn set_value_counter_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "adc timestamp select： 0：reserved; 1: from value_a; 2: from value_b; 3: from value_c;."]
        #[must_use]
        #[inline(always)]
        pub const fn adc_timestamp_sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "adc timestamp select： 0：reserved; 1: from value_a; 2: from value_b; 3: from value_c;."]
        #[inline(always)]
        pub const fn set_adc_timestamp_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "postion capture mode: 00: position use last valid data when adc value capture finish 01: position use frist valid data after adc value capture 10: position use last valid data before adc value capture other: reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn position_capture_mode(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "postion capture mode: 00: position use last valid data when adc value capture finish 01: position use frist valid data after adc value capture 10: position use last valid data before adc value capture other: reserved."]
        #[inline(always)]
        pub const fn set_position_capture_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for Timelock {
        #[inline(always)]
        fn default() -> Timelock {
            Timelock(0)
        }
    }
    impl core::fmt::Debug for Timelock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timelock")
                .field("value_counter_sel", &self.value_counter_sel())
                .field("adc_timestamp_sel", &self.adc_timestamp_sel())
                .field("position_capture_mode", &self.position_capture_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timelock {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Timelock {{ value_counter_sel: {=u8:?}, adc_timestamp_sel: {=u8:?}, position_capture_mode: {=u8:?} }}" , self . value_counter_sel () , self . adc_timestamp_sel () , self . position_capture_mode ())
        }
    }
    #[doc = "timestamp_locked."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TimestampLocked(pub u32);
    impl TimestampLocked {
        #[doc = "timestamp_locked."]
        #[must_use]
        #[inline(always)]
        pub const fn timestamp_locked(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "timestamp_locked."]
        #[inline(always)]
        pub const fn set_timestamp_locked(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TimestampLocked {
        #[inline(always)]
        fn default() -> TimestampLocked {
            TimestampLocked(0)
        }
    }
    impl core::fmt::Debug for TimestampLocked {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TimestampLocked")
                .field("timestamp_locked", &self.timestamp_locked())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TimestampLocked {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TimestampLocked {{ timestamp_locked: {=u32:?} }}",
                self.timestamp_locked()
            )
        }
    }
    #[doc = "software trigger event."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TriggerSw(pub u32);
    impl TriggerSw {
        #[doc = "software trigger to start waiting adc capture value, same as hardwire trigger_in."]
        #[must_use]
        #[inline(always)]
        pub const fn trigger_sw(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "software trigger to start waiting adc capture value, same as hardwire trigger_in."]
        #[inline(always)]
        pub const fn set_trigger_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for TriggerSw {
        #[inline(always)]
        fn default() -> TriggerSw {
            TriggerSw(0)
        }
    }
    impl core::fmt::Debug for TriggerSw {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TriggerSw")
                .field("trigger_sw", &self.trigger_sw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TriggerSw {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TriggerSw {{ trigger_sw: {=bool:?} }}",
                self.trigger_sw()
            )
        }
    }
    #[doc = "value_a data operation mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueADataOpt(pub u32);
    impl ValueADataOpt {
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[must_use]
        #[inline(always)]
        pub const fn opt_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn set_opt_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[must_use]
        #[inline(always)]
        pub const fn opt_1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn set_opt_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[must_use]
        #[inline(always)]
        pub const fn opt_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn set_opt_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[must_use]
        #[inline(always)]
        pub const fn opt_3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn set_opt_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for ValueADataOpt {
        #[inline(always)]
        fn default() -> ValueADataOpt {
            ValueADataOpt(0)
        }
    }
    impl core::fmt::Debug for ValueADataOpt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ValueADataOpt")
                .field("opt_0", &self.opt_0())
                .field("opt_1", &self.opt_1())
                .field("opt_2", &self.opt_2())
                .field("opt_3", &self.opt_3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ValueADataOpt {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ValueADataOpt {{ opt_0: {=u8:?}, opt_1: {=u8:?}, opt_2: {=u8:?}, opt_3: {=u8:?} }}" , self . opt_0 () , self . opt_1 () , self . opt_2 () , self . opt_3 ())
        }
    }
    #[doc = "value_a offset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueAOffset(pub u32);
    impl ValueAOffset {
        #[doc = "value_a offset."]
        #[must_use]
        #[inline(always)]
        pub const fn value_a_offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "value_a offset."]
        #[inline(always)]
        pub const fn set_value_a_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ValueAOffset {
        #[inline(always)]
        fn default() -> ValueAOffset {
            ValueAOffset(0)
        }
    }
    impl core::fmt::Debug for ValueAOffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ValueAOffset")
                .field("value_a_offset", &self.value_a_offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ValueAOffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ValueAOffset {{ value_a_offset: {=u32:?} }}",
                self.value_a_offset()
            )
        }
    }
    #[doc = "value_a software inject value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueASw(pub u32);
    impl ValueASw {
        #[doc = "value_a_sw."]
        #[must_use]
        #[inline(always)]
        pub const fn value_a_sw(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "value_a_sw."]
        #[inline(always)]
        pub const fn set_value_a_sw(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ValueASw {
        #[inline(always)]
        fn default() -> ValueASw {
            ValueASw(0)
        }
    }
    impl core::fmt::Debug for ValueASw {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ValueASw")
                .field("value_a_sw", &self.value_a_sw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ValueASw {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ValueASw {{ value_a_sw: {=u32:?} }}", self.value_a_sw())
        }
    }
    #[doc = "value_b data operation mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueBDataOpt(pub u32);
    impl ValueBDataOpt {
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[must_use]
        #[inline(always)]
        pub const fn opt_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn set_opt_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[must_use]
        #[inline(always)]
        pub const fn opt_1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn set_opt_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[must_use]
        #[inline(always)]
        pub const fn opt_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn set_opt_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[must_use]
        #[inline(always)]
        pub const fn opt_3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn set_opt_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for ValueBDataOpt {
        #[inline(always)]
        fn default() -> ValueBDataOpt {
            ValueBDataOpt(0)
        }
    }
    impl core::fmt::Debug for ValueBDataOpt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ValueBDataOpt")
                .field("opt_0", &self.opt_0())
                .field("opt_1", &self.opt_1())
                .field("opt_2", &self.opt_2())
                .field("opt_3", &self.opt_3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ValueBDataOpt {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ValueBDataOpt {{ opt_0: {=u8:?}, opt_1: {=u8:?}, opt_2: {=u8:?}, opt_3: {=u8:?} }}" , self . opt_0 () , self . opt_1 () , self . opt_2 () , self . opt_3 ())
        }
    }
    #[doc = "value_b_offset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueBOffset(pub u32);
    impl ValueBOffset {
        #[doc = "value_b_offset."]
        #[must_use]
        #[inline(always)]
        pub const fn value_b_offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "value_b_offset."]
        #[inline(always)]
        pub const fn set_value_b_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ValueBOffset {
        #[inline(always)]
        fn default() -> ValueBOffset {
            ValueBOffset(0)
        }
    }
    impl core::fmt::Debug for ValueBOffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ValueBOffset")
                .field("value_b_offset", &self.value_b_offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ValueBOffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ValueBOffset {{ value_b_offset: {=u32:?} }}",
                self.value_b_offset()
            )
        }
    }
    #[doc = "value_b software inject value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueBSw(pub u32);
    impl ValueBSw {
        #[doc = "value_b_sw."]
        #[must_use]
        #[inline(always)]
        pub const fn value_b_sw(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "value_b_sw."]
        #[inline(always)]
        pub const fn set_value_b_sw(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ValueBSw {
        #[inline(always)]
        fn default() -> ValueBSw {
            ValueBSw(0)
        }
    }
    impl core::fmt::Debug for ValueBSw {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ValueBSw")
                .field("value_b_sw", &self.value_b_sw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ValueBSw {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ValueBSw {{ value_b_sw: {=u32:?} }}", self.value_b_sw())
        }
    }
    #[doc = "value_c data operation mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueCDataOpt(pub u32);
    impl ValueCDataOpt {
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[must_use]
        #[inline(always)]
        pub const fn opt_0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn set_opt_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[must_use]
        #[inline(always)]
        pub const fn opt_1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn set_opt_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[must_use]
        #[inline(always)]
        pub const fn opt_2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn set_opt_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[must_use]
        #[inline(always)]
        pub const fn opt_3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "0: PLUS_MUL_1 1: PLUS_MUL_2 5: PLUS_DIV_2 6: PLUS_DIV_3 7: PLUS_DIV_4 8: MINUS MUL 1 9: MINUS MUL 2 13: MINUS DIV 2 14: MINUS DIV 3 15: MINUS DIV 4."]
        #[inline(always)]
        pub const fn set_opt_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for ValueCDataOpt {
        #[inline(always)]
        fn default() -> ValueCDataOpt {
            ValueCDataOpt(0)
        }
    }
    impl core::fmt::Debug for ValueCDataOpt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ValueCDataOpt")
                .field("opt_0", &self.opt_0())
                .field("opt_1", &self.opt_1())
                .field("opt_2", &self.opt_2())
                .field("opt_3", &self.opt_3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ValueCDataOpt {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ValueCDataOpt {{ opt_0: {=u8:?}, opt_1: {=u8:?}, opt_2: {=u8:?}, opt_3: {=u8:?} }}" , self . opt_0 () , self . opt_1 () , self . opt_2 () , self . opt_3 ())
        }
    }
    #[doc = "value_c offset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueCOffset(pub u32);
    impl ValueCOffset {
        #[doc = "value_c offset."]
        #[must_use]
        #[inline(always)]
        pub const fn value_c_offset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "value_c offset."]
        #[inline(always)]
        pub const fn set_value_c_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ValueCOffset {
        #[inline(always)]
        fn default() -> ValueCOffset {
            ValueCOffset(0)
        }
    }
    impl core::fmt::Debug for ValueCOffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ValueCOffset")
                .field("value_c_offset", &self.value_c_offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ValueCOffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ValueCOffset {{ value_c_offset: {=u32:?} }}",
                self.value_c_offset()
            )
        }
    }
    #[doc = "value_c software inject value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueCSw(pub u32);
    impl ValueCSw {
        #[doc = "value_c_sw."]
        #[must_use]
        #[inline(always)]
        pub const fn value_c_sw(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "value_c_sw."]
        #[inline(always)]
        pub const fn set_value_c_sw(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ValueCSw {
        #[inline(always)]
        fn default() -> ValueCSw {
            ValueCSw(0)
        }
    }
    impl core::fmt::Debug for ValueCSw {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ValueCSw")
                .field("value_c_sw", &self.value_c_sw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ValueCSw {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ValueCSw {{ value_c_sw: {=u32:?} }}", self.value_c_sw())
        }
    }
    #[doc = "software inject value_a/value_b/value_c ready."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValueSwReady(pub u32);
    impl ValueSwReady {
        #[doc = "software inject value_a/value_b/value_c ready."]
        #[must_use]
        #[inline(always)]
        pub const fn value_sw_ready(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "software inject value_a/value_b/value_c ready."]
        #[inline(always)]
        pub const fn set_value_sw_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for ValueSwReady {
        #[inline(always)]
        fn default() -> ValueSwReady {
            ValueSwReady(0)
        }
    }
    impl core::fmt::Debug for ValueSwReady {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ValueSwReady")
                .field("value_sw_ready", &self.value_sw_ready())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ValueSwReady {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ValueSwReady {{ value_sw_ready: {=bool:?} }}",
                self.value_sw_ready()
            )
        }
    }
}
