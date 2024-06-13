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
        pub const fn capmode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture."]
        #[inline(always)]
        pub fn set_capmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
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
        pub const fn dmasel(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "select one of DMA request: 00- CMP0 flag 01- CMP1 flag 10- Input signal toggle captured 11- RLD flag, counter reload;."]
        #[inline(always)]
        pub fn set_dmasel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
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
        pub const fn ch0rlden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch0rldf flag is set."]
        #[inline(always)]
        pub fn set_ch0rlden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "1- generate interrupt request when ch0capf flag is set."]
        #[inline(always)]
        pub const fn ch0capen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch0capf flag is set."]
        #[inline(always)]
        pub fn set_ch0capen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "1- generate interrupt request when ch0cmp0f flag is set."]
        #[inline(always)]
        pub const fn ch0cmp0en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch0cmp0f flag is set."]
        #[inline(always)]
        pub fn set_ch0cmp0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "1- generate interrupt request when ch0cmp1f flag is set."]
        #[inline(always)]
        pub const fn ch0cmp1en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch0cmp1f flag is set."]
        #[inline(always)]
        pub fn set_ch0cmp1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "1- generate interrupt request when ch1rldf flag is set."]
        #[inline(always)]
        pub const fn ch1rlden(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch1rldf flag is set."]
        #[inline(always)]
        pub fn set_ch1rlden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "1- generate interrupt request when ch1capf flag is set."]
        #[inline(always)]
        pub const fn ch1capen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch1capf flag is set."]
        #[inline(always)]
        pub fn set_ch1capen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "1- generate interrupt request when ch1cmp0f flag is set."]
        #[inline(always)]
        pub const fn ch1cmp0en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch1cmp0f flag is set."]
        #[inline(always)]
        pub fn set_ch1cmp0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "1- generate interrupt request when ch1cmp1f flag is set."]
        #[inline(always)]
        pub const fn ch1cmp1en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch1cmp1f flag is set."]
        #[inline(always)]
        pub fn set_ch1cmp1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "1- generate interrupt request when ch2rldf flag is set."]
        #[inline(always)]
        pub const fn ch2rlden(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch2rldf flag is set."]
        #[inline(always)]
        pub fn set_ch2rlden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "1- generate interrupt request when ch2capf flag is set."]
        #[inline(always)]
        pub const fn ch2capen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch2capf flag is set."]
        #[inline(always)]
        pub fn set_ch2capen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "1- generate interrupt request when ch2cmp0f flag is set."]
        #[inline(always)]
        pub const fn ch2cmp0en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch2cmp0f flag is set."]
        #[inline(always)]
        pub fn set_ch2cmp0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "1- generate interrupt request when ch2cmp1f flag is set."]
        #[inline(always)]
        pub const fn ch2cmp1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch2cmp1f flag is set."]
        #[inline(always)]
        pub fn set_ch2cmp1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "1- generate interrupt request when ch3rldf flag is set."]
        #[inline(always)]
        pub const fn ch3rlden(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch3rldf flag is set."]
        #[inline(always)]
        pub fn set_ch3rlden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "1- generate interrupt request when ch3capf flag is set."]
        #[inline(always)]
        pub const fn ch3capen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch3capf flag is set."]
        #[inline(always)]
        pub fn set_ch3capen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "1- generate interrupt request when ch3cmp0f flag is set."]
        #[inline(always)]
        pub const fn ch3cmp0en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch3cmp0f flag is set."]
        #[inline(always)]
        pub fn set_ch3cmp0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "1- generate interrupt request when ch3cmp1f flag is set."]
        #[inline(always)]
        pub const fn ch3cmp1en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "1- generate interrupt request when ch3cmp1f flag is set."]
        #[inline(always)]
        pub fn set_ch3cmp1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
        pub const fn ch0rldf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "channel 1 counter reload flag."]
        #[inline(always)]
        pub fn set_ch0rldf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
        #[inline(always)]
        pub const fn ch0capf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
        #[inline(always)]
        pub fn set_ch0capf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub const fn ch0cmp0f(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub fn set_ch0cmp0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub const fn ch0cmp1f(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub fn set_ch0cmp1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "channel 1 counter reload flag."]
        #[inline(always)]
        pub const fn ch1rldf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "channel 1 counter reload flag."]
        #[inline(always)]
        pub fn set_ch1rldf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
        #[inline(always)]
        pub const fn ch1capf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
        #[inline(always)]
        pub fn set_ch1capf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub const fn ch1cmp0f(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub fn set_ch1cmp0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub const fn ch1cmp1f(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "channel 1 compare value 1 match flag."]
        #[inline(always)]
        pub fn set_ch1cmp1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "channel 2 counter reload flag."]
        #[inline(always)]
        pub const fn ch2rldf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "channel 2 counter reload flag."]
        #[inline(always)]
        pub fn set_ch2rldf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "channel 2 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
        #[inline(always)]
        pub const fn ch2capf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "channel 2 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
        #[inline(always)]
        pub fn set_ch2capf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "channel 2 compare value 1 match flag."]
        #[inline(always)]
        pub const fn ch2cmp0f(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "channel 2 compare value 1 match flag."]
        #[inline(always)]
        pub fn set_ch2cmp0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "channel 2 compare value 1 match flag."]
        #[inline(always)]
        pub const fn ch2cmp1f(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "channel 2 compare value 1 match flag."]
        #[inline(always)]
        pub fn set_ch2cmp1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "channel 3 counter reload flag."]
        #[inline(always)]
        pub const fn ch3rldf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "channel 3 counter reload flag."]
        #[inline(always)]
        pub fn set_ch3rldf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "channel 3 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
        #[inline(always)]
        pub const fn ch3capf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "channel 3 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
        #[inline(always)]
        pub fn set_ch3capf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "channel 3 compare value 1 match flag."]
        #[inline(always)]
        pub const fn ch3cmp0f(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "channel 3 compare value 1 match flag."]
        #[inline(always)]
        pub fn set_ch3cmp0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "channel 3 compare value 1 match flag."]
        #[inline(always)]
        pub const fn ch3cmp1f(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "channel 3 compare value 1 match flag."]
        #[inline(always)]
        pub fn set_ch3cmp1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
}
