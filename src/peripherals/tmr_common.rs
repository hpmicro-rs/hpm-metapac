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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmp(self, n: usize) -> crate::common::Reg<regs::Cmp, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
    }
    #[doc = "Reload register."]
    #[inline(always)]
    pub const fn rld(self) -> crate::common::Reg<regs::Rld, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Counter update value register."]
    #[inline(always)]
    pub const fn cntuptval(self) -> crate::common::Reg<regs::Cntuptval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Capture rising edge register."]
    #[inline(always)]
    pub const fn cappos(self) -> crate::common::Reg<regs::Cappos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Capture falling edge register."]
    #[inline(always)]
    pub const fn capneg(self) -> crate::common::Reg<regs::Capneg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "PWM period measure register."]
    #[inline(always)]
    pub const fn capprd(self) -> crate::common::Reg<regs::Capprd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "PWM duty cycle measure register."]
    #[inline(always)]
    pub const fn capdty(self) -> crate::common::Reg<regs::Capdty, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Counter."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
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
        unsafe { Channel::from_ptr(self.ptr.add(0x0usize + n * 64usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Interrupt request enable register."]
    #[inline(always)]
    pub const fn irqen(self) -> crate::common::Reg<regs::Irqen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "Global control register."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
}
pub mod regs {
    #[doc = "PWM duty cycle measure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Capdty(pub u32);
    impl Capdty {
        #[doc = "This register contains the input signal duty cycle when channel is configured to input capture measure mode."]
        #[inline(always)]
        pub const fn meas_high(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "This register contains the input signal duty cycle when channel is configured to input capture measure mode."]
        #[inline(always)]
        pub fn set_meas_high(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Capdty {
        #[inline(always)]
        fn default() -> Capdty {
            Capdty(0)
        }
    }
    #[doc = "Capture falling edge register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Capneg(pub u32);
    impl Capneg {
        #[doc = "This register contains the counter value captured at input signal falling edge."]
        #[inline(always)]
        pub const fn capneg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "This register contains the counter value captured at input signal falling edge."]
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
    #[doc = "Capture rising edge register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cappos(pub u32);
    impl Cappos {
        #[doc = "This register contains the counter value captured at input signal rising edge."]
        #[inline(always)]
        pub const fn cappos(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "This register contains the counter value captured at input signal rising edge."]
        #[inline(always)]
        pub fn set_cappos(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cappos {
        #[inline(always)]
        fn default() -> Cappos {
            Cappos(0)
        }
    }
    #[doc = "PWM period measure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Capprd(pub u32);
    impl Capprd {
        #[doc = "This register contains the input signal period when channel is configured to input capture measure mode."]
        #[inline(always)]
        pub const fn capprd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "This register contains the input signal period when channel is configured to input capture measure mode."]
        #[inline(always)]
        pub fn set_capprd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Capprd {
        #[inline(always)]
        fn default() -> Capprd {
            Capprd(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmp(pub u32);
    impl Cmp {
        #[doc = "compare value 0."]
        #[inline(always)]
        pub const fn cmp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "compare value 0."]
        #[inline(always)]
        pub fn set_cmp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cmp {
        #[inline(always)]
        fn default() -> Cmp {
            Cmp(0)
        }
    }
    #[doc = "Counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "32 bit counter value."]
        #[inline(always)]
        pub const fn counter(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "32 bit counter value."]
        #[inline(always)]
        pub fn set_counter(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    #[doc = "Counter update value register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntuptval(pub u32);
    impl Cntuptval {
        #[doc = "counter will be set to this value when software write cntupt bit in CR."]
        #[inline(always)]
        pub const fn cntuptval(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "counter will be set to this value when software write cntupt bit in CR."]
        #[inline(always)]
        pub fn set_cntuptval(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cntuptval {
        #[inline(always)]
        fn default() -> Cntuptval {
            Cntuptval(0)
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture."]
        #[inline(always)]
        pub const fn capmode(&self) -> super::vals::Capmode {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Capmode::from_bits(val as u8)
        }
        #[doc = "This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture."]
        #[inline(always)]
        pub fn set_capmode(&mut self, val: super::vals::Capmode) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "1- counter will pause if chip is in debug mode."]
        #[inline(always)]
        pub const fn dbgpause(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "1- counter will pause if chip is in debug mode."]
        #[inline(always)]
        pub fn set_dbgpause(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set."]
        #[inline(always)]
        pub const fn swsyncien(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set."]
        #[inline(always)]
        pub fn set_swsyncien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "1- enable dma."]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable dma."]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "select one of DMA request: 00- CMP0 flag 01- CMP1 flag 10- Input signal toggle captured 11- RLD flag, counter reload;."]
        #[inline(always)]
        pub const fn dmasel(&self) -> super::vals::Dmasel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Dmasel::from_bits(val as u8)
        }
        #[doc = "select one of DMA request: 00- CMP0 flag 01- CMP1 flag 10- Input signal toggle captured 11- RLD flag, counter reload;."]
        #[inline(always)]
        pub fn set_dmasel(&mut self, val: super::vals::Dmasel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
        #[inline(always)]
        pub const fn cmpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
        #[inline(always)]
        pub fn set_cmpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
        #[inline(always)]
        pub const fn cmpinit(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
        #[inline(always)]
        pub fn set_cmpinit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "1- counter enable."]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "1- counter enable."]
        #[inline(always)]
        pub fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "1- SYNCI is valid on its rising edge."]
        #[inline(always)]
        pub const fn synciren(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "1- SYNCI is valid on its rising edge."]
        #[inline(always)]
        pub fn set_synciren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "1- SYNCI is valid on its falling edge."]
        #[inline(always)]
        pub const fn syncifen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "1- SYNCI is valid on its falling edge."]
        #[inline(always)]
        pub fn set_syncifen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
        #[inline(always)]
        pub const fn syncflw(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
        #[inline(always)]
        pub fn set_syncflw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "1- reset counter."]
        #[inline(always)]
        pub const fn cntrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "1- reset counter."]
        #[inline(always)]
        pub fn set_cntrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle."]
        #[inline(always)]
        pub const fn cntupt(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle."]
        #[inline(always)]
        pub fn set_cntupt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "Global control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "set this bitfield to trigger software counter sync event."]
        #[inline(always)]
        pub const fn swsynct(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "set this bitfield to trigger software counter sync event."]
        #[inline(always)]
        pub fn set_swsynct(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    #[doc = "Interrupt request enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irqen(pub u32);
    impl Irqen {
        #[doc = "1- generate interrupt request when ch0rldf flag is set."]
        #[inline(always)]
        pub const fn chrlden(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch0rldf flag is set."]
        #[inline(always)]
        pub fn set_chrlden(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "1- generate interrupt request when ch0capf flag is set."]
        #[inline(always)]
        pub const fn chcapen(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch0capf flag is set."]
        #[inline(always)]
        pub fn set_chcapen(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "1- generate interrupt request when ch0cmp0f flag is set."]
        #[inline(always)]
        pub const fn chcmp0en(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch0cmp0f flag is set."]
        #[inline(always)]
        pub fn set_chcmp0en(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "1- generate interrupt request when ch0cmp1f flag is set."]
        #[inline(always)]
        pub const fn chcmp1en(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch0cmp1f flag is set."]
        #[inline(always)]
        pub fn set_chcmp1en(&mut self, n: usize, val: bool) {
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
    #[doc = "Reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rld(pub u32);
    impl Rld {
        #[doc = "reload value."]
        #[inline(always)]
        pub const fn rld(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "reload value."]
        #[inline(always)]
        pub fn set_rld(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rld {
        #[inline(always)]
        fn default() -> Rld {
            Rld(0)
        }
    }
    #[doc = "Status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "channel 1 counter reload flag."]
        #[inline(always)]
        pub const fn chrldf(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel 1 counter reload flag."]
        #[inline(always)]
        pub fn set_chrldf(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
        #[inline(always)]
        pub const fn chcapf(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
        #[inline(always)]
        pub fn set_chcapf(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub const fn chcmp0f(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub fn set_chcmp0f(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub const fn chcmp1f(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub fn set_chcmp1f(&mut self, n: usize, val: bool) {
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
}
pub mod vals {
    #[doc = "capture mode"]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
