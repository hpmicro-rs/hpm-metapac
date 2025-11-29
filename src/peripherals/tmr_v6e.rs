#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel {
    ptr: *mut u8,
}
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmp(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 4usize) as _) }
    }
    #[doc = "Reload register."]
    #[inline(always)]
    pub const fn rld(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Counter update value register."]
    #[inline(always)]
    pub const fn cntuptval(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Capture rising edge register."]
    #[inline(always)]
    pub const fn cappos(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Capture falling edge register."]
    #[inline(always)]
    pub const fn capneg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "PWM period measure register."]
    #[inline(always)]
    pub const fn capprd(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "PWM duty cycle measure register."]
    #[inline(always)]
    pub const fn capdty(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Counter."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
}
#[doc = "GPTMR0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr {
    ptr: *mut u8,
}
unsafe impl Send for Tmr {}
unsafe impl Sync for Tmr {}
impl Tmr {
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
    pub const fn channel(self, n: usize) -> Channel {
        assert!(n < 4usize);
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x0usize + n * 64usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Interrupt request enable register."]
    #[inline(always)]
    pub const fn irqen(self) -> crate::common::Reg<regs::Irqen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Global control register."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
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
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture."]
        #[must_use]
        #[inline(always)]
        pub const fn capmode(&self) -> super::vals::Capmode {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Capmode::from_bits(val as u8)
        }
        #[doc = "This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture."]
        #[inline(always)]
        pub const fn set_capmode(&mut self, val: super::vals::Capmode) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "1- counter will pause if chip is in debug mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dbgpause(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "1- counter will pause if chip is in debug mode."]
        #[inline(always)]
        pub const fn set_dbgpause(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set."]
        #[must_use]
        #[inline(always)]
        pub const fn swsyncien(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set."]
        #[inline(always)]
        pub const fn set_swsyncien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "1- enable dma."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable dma."]
        #[inline(always)]
        pub const fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "select one of DMA request: 00- CMP0 flag 01- CMP1 flag 10- Input signal toggle captured 11- RLD flag, counter reload;."]
        #[must_use]
        #[inline(always)]
        pub const fn dmasel(&self) -> super::vals::Dmasel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Dmasel::from_bits(val as u8)
        }
        #[doc = "select one of DMA request: 00- CMP0 flag 01- CMP1 flag 10- Input signal toggle captured 11- RLD flag, counter reload;."]
        #[inline(always)]
        pub const fn set_dmasel(&mut self, val: super::vals::Dmasel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
        #[inline(always)]
        pub const fn set_cmpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpinit(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
        #[inline(always)]
        pub const fn set_cmpinit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "1- counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "1- counter enable."]
        #[inline(always)]
        pub const fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "1- SYNCI is valid on its rising edge."]
        #[must_use]
        #[inline(always)]
        pub const fn synciren(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "1- SYNCI is valid on its rising edge."]
        #[inline(always)]
        pub const fn set_synciren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "1- SYNCI is valid on its falling edge."]
        #[must_use]
        #[inline(always)]
        pub const fn syncifen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "1- SYNCI is valid on its falling edge."]
        #[inline(always)]
        pub const fn set_syncifen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn syncflw(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
        #[inline(always)]
        pub const fn set_syncflw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "1- reset counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cntrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "1- reset counter."]
        #[inline(always)]
        pub const fn set_cntrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "set to monitor input signal period or high level time. When this bit is set, if detected period less than val_0 or more than val_1, will set related irq_sts * only can be used when trig_mode is selected as measure mode(100) * the time may not correct after reload, so monitor is disabled after reload point, and enabled again after two continul posedge. if no posedge after reload for more than val_1, will also assert irq_capt."]
        #[must_use]
        #[inline(always)]
        pub const fn monitor_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "set to monitor input signal period or high level time. When this bit is set, if detected period less than val_0 or more than val_1, will set related irq_sts * only can be used when trig_mode is selected as measure mode(100) * the time may not correct after reload, so monitor is disabled after reload point, and enabled again after two continul posedge. if no posedge after reload for more than val_1, will also assert irq_capt."]
        #[inline(always)]
        pub const fn set_monitor_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "set to monitor input signal high level time(chan_meas_high) clr to monitor input signal period(chan_meas_prd)."]
        #[must_use]
        #[inline(always)]
        pub const fn monitor_sel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "set to monitor input signal high level time(chan_meas_high) clr to monitor input signal period(chan_meas_prd)."]
        #[inline(always)]
        pub const fn set_monitor_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "0: round mode 1: one-shot mode, timer will stopped at reload point. NOTE: reload irq will be always set at one-shot mode at end."]
        #[must_use]
        #[inline(always)]
        pub const fn opmode(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "0: round mode 1: one-shot mode, timer will stopped at reload point. NOTE: reload irq will be always set at one-shot mode at end."]
        #[inline(always)]
        pub const fn set_opmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "0: internal counting mode, timer increase each gptmr clock cycle. 1: external counting mode, timer increase at each input signal posedge, reload/compare feature can still work but change at input signal posedge."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_mode(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "0: internal counting mode, timer increase each gptmr clock cycle. 1: external counting mode, timer increase at each input signal posedge, reload/compare feature can still work but change at input signal posedge."]
        #[inline(always)]
        pub const fn set_cnt_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn cntupt(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle."]
        #[inline(always)]
        pub const fn set_cntupt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("capmode", &self.capmode())
                .field("dbgpause", &self.dbgpause())
                .field("swsyncien", &self.swsyncien())
                .field("dmaen", &self.dmaen())
                .field("dmasel", &self.dmasel())
                .field("cmpen", &self.cmpen())
                .field("cmpinit", &self.cmpinit())
                .field("cen", &self.cen())
                .field("synciren", &self.synciren())
                .field("syncifen", &self.syncifen())
                .field("syncflw", &self.syncflw())
                .field("cntrst", &self.cntrst())
                .field("monitor_en", &self.monitor_en())
                .field("monitor_sel", &self.monitor_sel())
                .field("opmode", &self.opmode())
                .field("cnt_mode", &self.cnt_mode())
                .field("cntupt", &self.cntupt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ capmode: {:?}, dbgpause: {=bool:?}, swsyncien: {=bool:?}, dmaen: {=bool:?}, dmasel: {:?}, cmpen: {=bool:?}, cmpinit: {=bool:?}, cen: {=bool:?}, synciren: {=bool:?}, syncifen: {=bool:?}, syncflw: {=bool:?}, cntrst: {=bool:?}, monitor_en: {=bool:?}, monitor_sel: {=bool:?}, opmode: {=bool:?}, cnt_mode: {=bool:?}, cntupt: {=bool:?} }}" , self . capmode () , self . dbgpause () , self . swsyncien () , self . dmaen () , self . dmasel () , self . cmpen () , self . cmpinit () , self . cen () , self . synciren () , self . syncifen () , self . syncflw () , self . cntrst () , self . monitor_en () , self . monitor_sel () , self . opmode () , self . cnt_mode () , self . cntupt ())
        }
    }
    #[doc = "Global control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "set this bitfield to trigger software counter sync event."]
        #[must_use]
        #[inline(always)]
        pub const fn swsynct(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "set this bitfield to trigger software counter sync event."]
        #[inline(always)]
        pub const fn set_swsynct(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    impl core::fmt::Debug for Gcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gcr")
                .field("swsynct", &self.swsynct())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gcr {{ swsynct: {=u8:?} }}", self.swsynct())
        }
    }
    #[doc = "Interrupt request enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irqen(pub u32);
    impl Irqen {
        #[doc = "1- generate interrupt request when ch0rldf flag is set."]
        #[must_use]
        #[inline(always)]
        pub const fn chrlden(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch0rldf flag is set."]
        #[inline(always)]
        pub const fn set_chrlden(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "1- generate interrupt request when ch0capf flag is set."]
        #[must_use]
        #[inline(always)]
        pub const fn chcapen(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch0capf flag is set."]
        #[inline(always)]
        pub const fn set_chcapen(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "1- generate interrupt request when ch0cmp0f flag is set."]
        #[must_use]
        #[inline(always)]
        pub const fn chcmp0en(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch0cmp0f flag is set."]
        #[inline(always)]
        pub const fn set_chcmp0en(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "1- generate interrupt request when ch0cmp1f flag is set."]
        #[must_use]
        #[inline(always)]
        pub const fn chcmp1en(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch0cmp1f flag is set."]
        #[inline(always)]
        pub const fn set_chcmp1en(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Irqen {
        #[inline(always)]
        fn default() -> Irqen {
            Irqen(0)
        }
    }
    impl core::fmt::Debug for Irqen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irqen")
                .field("chrlden[0]", &self.chrlden(0usize))
                .field("chrlden[1]", &self.chrlden(1usize))
                .field("chrlden[2]", &self.chrlden(2usize))
                .field("chrlden[3]", &self.chrlden(3usize))
                .field("chcapen[0]", &self.chcapen(0usize))
                .field("chcapen[1]", &self.chcapen(1usize))
                .field("chcapen[2]", &self.chcapen(2usize))
                .field("chcapen[3]", &self.chcapen(3usize))
                .field("chcmp0en[0]", &self.chcmp0en(0usize))
                .field("chcmp0en[1]", &self.chcmp0en(1usize))
                .field("chcmp0en[2]", &self.chcmp0en(2usize))
                .field("chcmp0en[3]", &self.chcmp0en(3usize))
                .field("chcmp1en[0]", &self.chcmp1en(0usize))
                .field("chcmp1en[1]", &self.chcmp1en(1usize))
                .field("chcmp1en[2]", &self.chcmp1en(2usize))
                .field("chcmp1en[3]", &self.chcmp1en(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Irqen {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Irqen {{ chrlden[0]: {=bool:?}, chrlden[1]: {=bool:?}, chrlden[2]: {=bool:?}, chrlden[3]: {=bool:?}, chcapen[0]: {=bool:?}, chcapen[1]: {=bool:?}, chcapen[2]: {=bool:?}, chcapen[3]: {=bool:?}, chcmp0en[0]: {=bool:?}, chcmp0en[1]: {=bool:?}, chcmp0en[2]: {=bool:?}, chcmp0en[3]: {=bool:?}, chcmp1en[0]: {=bool:?}, chcmp1en[1]: {=bool:?}, chcmp1en[2]: {=bool:?}, chcmp1en[3]: {=bool:?} }}" , self . chrlden (0usize) , self . chrlden (1usize) , self . chrlden (2usize) , self . chrlden (3usize) , self . chcapen (0usize) , self . chcapen (1usize) , self . chcapen (2usize) , self . chcapen (3usize) , self . chcmp0en (0usize) , self . chcmp0en (1usize) , self . chcmp0en (2usize) , self . chcmp0en (3usize) , self . chcmp1en (0usize) , self . chcmp1en (1usize) , self . chcmp1en (2usize) , self . chcmp1en (3usize))
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "channel 1 counter reload flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chrldf(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel 1 counter reload flag."]
        #[inline(always)]
        pub const fn set_chrldf(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
        #[must_use]
        #[inline(always)]
        pub const fn chcapf(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
        #[inline(always)]
        pub const fn set_chcapf(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chcmp0f(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub const fn set_chcmp0f(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[must_use]
        #[inline(always)]
        pub const fn chcmp1f(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub const fn set_chcmp1f(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("chrldf[0]", &self.chrldf(0usize))
                .field("chrldf[1]", &self.chrldf(1usize))
                .field("chrldf[2]", &self.chrldf(2usize))
                .field("chrldf[3]", &self.chrldf(3usize))
                .field("chcapf[0]", &self.chcapf(0usize))
                .field("chcapf[1]", &self.chcapf(1usize))
                .field("chcapf[2]", &self.chcapf(2usize))
                .field("chcapf[3]", &self.chcapf(3usize))
                .field("chcmp0f[0]", &self.chcmp0f(0usize))
                .field("chcmp0f[1]", &self.chcmp0f(1usize))
                .field("chcmp0f[2]", &self.chcmp0f(2usize))
                .field("chcmp0f[3]", &self.chcmp0f(3usize))
                .field("chcmp1f[0]", &self.chcmp1f(0usize))
                .field("chcmp1f[1]", &self.chcmp1f(1usize))
                .field("chcmp1f[2]", &self.chcmp1f(2usize))
                .field("chcmp1f[3]", &self.chcmp1f(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr {{ chrldf[0]: {=bool:?}, chrldf[1]: {=bool:?}, chrldf[2]: {=bool:?}, chrldf[3]: {=bool:?}, chcapf[0]: {=bool:?}, chcapf[1]: {=bool:?}, chcapf[2]: {=bool:?}, chcapf[3]: {=bool:?}, chcmp0f[0]: {=bool:?}, chcmp0f[1]: {=bool:?}, chcmp0f[2]: {=bool:?}, chcmp0f[3]: {=bool:?}, chcmp1f[0]: {=bool:?}, chcmp1f[1]: {=bool:?}, chcmp1f[2]: {=bool:?}, chcmp1f[3]: {=bool:?} }}" , self . chrldf (0usize) , self . chrldf (1usize) , self . chrldf (2usize) , self . chrldf (3usize) , self . chcapf (0usize) , self . chcapf (1usize) , self . chcapf (2usize) , self . chcapf (3usize) , self . chcmp0f (0usize) , self . chcmp0f (1usize) , self . chcmp0f (2usize) , self . chcmp0f (3usize) , self . chcmp1f (0usize) , self . chcmp1f (1usize) , self . chcmp1f (2usize) , self . chcmp1f (3usize))
        }
    }
}
pub mod vals {
    #[doc = "capture mode"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Capmode {
        #[doc = "No capture."]
        NOCAP = 0x0,
        #[doc = "Capture at rising edge."]
        RISING = 0x01,
        #[doc = "Capture at falling edge."]
        FALLING = 0x02,
        #[doc = "Capture at both rising and falling edge."]
        BOTH = 0x03,
        #[doc = "Width measure mode, timer will calculate the input signal period and duty cycle."]
        MEASURE_PWM = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Capmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Capmode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Capmode {
        #[inline(always)]
        fn from(val: u8) -> Capmode {
            Capmode::from_bits(val)
        }
    }
    impl From<Capmode> for u8 {
        #[inline(always)]
        fn from(val: Capmode) -> u8 {
            Capmode::to_bits(val)
        }
    }
    #[doc = "select one of DMA request"]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dmasel {
        #[doc = "CMP0 flag"]
        CMP0 = 0x0,
        #[doc = "CMP1 flag"]
        CMP1 = 0x01,
        #[doc = "Input signal toggle captured, when CAPF = 1"]
        CAPF = 0x02,
        #[doc = "RLD flag, counter reload;"]
        RLD = 0x03,
    }
    impl Dmasel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dmasel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dmasel {
        #[inline(always)]
        fn from(val: u8) -> Dmasel {
            Dmasel::from_bits(val)
        }
    }
    impl From<Dmasel> for u8 {
        #[inline(always)]
        fn from(val: Dmasel) -> u8 {
            Dmasel::to_bits(val)
        }
    }
}
