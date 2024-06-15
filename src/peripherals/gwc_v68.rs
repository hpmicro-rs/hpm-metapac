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
    #[doc = "config reg 0."]
    #[inline(always)]
    pub const fn cfg0(self) -> crate::common::Reg<regs::Cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "config reg 1."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "reference CRC."]
    #[inline(always)]
    pub const fn refcrc(self) -> crate::common::Reg<regs::Refcrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "calculated CRC."]
    #[inline(always)]
    pub const fn calcrc(self) -> crate::common::Reg<regs::Calcrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "GWC0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gwc {
    ptr: *mut u8,
}
unsafe impl Send for Gwc {}
unsafe impl Sync for Gwc {}
impl Gwc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control reg."]
    #[inline(always)]
    pub const fn glb_ctrl(self) -> crate::common::Reg<regs::GlbCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "interrupt enable."]
    #[inline(always)]
    pub const fn irq_mask(self) -> crate::common::Reg<regs::IrqMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "interrupt status."]
    #[inline(always)]
    pub const fn irq_sts(self) -> crate::common::Reg<regs::IrqSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn channel(self, n: usize) -> Channel {
        assert!(n < 2usize);
        unsafe { Channel::from_ptr(self.ptr.add(0x10usize + n * 240usize) as _) }
    }
}
pub mod regs {
    #[doc = "calculated CRC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calcrc(pub u32);
    impl Calcrc {
        #[doc = "calculated CRC for last frame."]
        #[inline(always)]
        pub const fn cal_crc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "calculated CRC for last frame."]
        #[inline(always)]
        pub fn set_cal_crc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Calcrc {
        #[inline(always)]
        fn default() -> Calcrc {
            Calcrc(0)
        }
    }
    #[doc = "config reg 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg0(pub u32);
    impl Cfg0 {
        #[doc = "define the window start column number."]
        #[inline(always)]
        pub const fn start_col(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "define the window start column number."]
        #[inline(always)]
        pub fn set_start_col(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "define the window start row number."]
        #[inline(always)]
        pub const fn start_row(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "define the window start row number."]
        #[inline(always)]
        pub fn set_start_row(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "freeze config. set to freeze all other config registers for current channel. can only be cleared by system reset."]
        #[inline(always)]
        pub const fn freeze(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "freeze config. set to freeze all other config registers for current channel. can only be cleared by system reset."]
        #[inline(always)]
        pub fn set_freeze(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "channel enable."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "channel enable."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg0 {
        #[inline(always)]
        fn default() -> Cfg0 {
            Cfg0(0)
        }
    }
    #[doc = "config reg 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg1(pub u32);
    impl Cfg1 {
        #[doc = "define the window end column number."]
        #[inline(always)]
        pub const fn end_col(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "define the window end column number."]
        #[inline(always)]
        pub fn set_end_col(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "define the window end row number."]
        #[inline(always)]
        pub const fn end_row(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "define the window end row number."]
        #[inline(always)]
        pub fn set_end_row(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Cfg1 {
        #[inline(always)]
        fn default() -> Cfg1 {
            Cfg1(0)
        }
    }
    #[doc = "control reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlbCtrl(pub u32);
    impl GlbCtrl {
        #[doc = "graphic window check enable. set to enable the whole block."]
        #[inline(always)]
        pub const fn gwc_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "graphic window check enable. set to enable the whole block."]
        #[inline(always)]
        pub fn set_gwc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "graphic clock polarity. set to invert input graphic clock."]
        #[inline(always)]
        pub const fn clk_pol(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "graphic clock polarity. set to invert input graphic clock."]
        #[inline(always)]
        pub fn set_clk_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for GlbCtrl {
        #[inline(always)]
        fn default() -> GlbCtrl {
            GlbCtrl(0)
        }
    }
    #[doc = "interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqMask(pub u32);
    impl IrqMask {
        #[doc = "error interrupt mask."]
        #[inline(always)]
        pub const fn err_mask(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error interrupt mask."]
        #[inline(always)]
        pub fn set_err_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "function interrupt mask."]
        #[inline(always)]
        pub const fn func_mask(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "function interrupt mask."]
        #[inline(always)]
        pub fn set_func_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "freeze mask, set to disable changing ERR_MASK and FUNC_MASK. can only be cleared by system reset."]
        #[inline(always)]
        pub const fn mask_rreez(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "freeze mask, set to disable changing ERR_MASK and FUNC_MASK. can only be cleared by system reset."]
        #[inline(always)]
        pub fn set_mask_rreez(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for IrqMask {
        #[inline(always)]
        fn default() -> IrqMask {
            IrqMask(0)
        }
    }
    #[doc = "interrupt status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqSts(pub u32);
    impl IrqSts {
        #[doc = "graphic window check fail interrupt status. will be set if the calculated CRC not equal reference CRC. one bit for each channel. software write 1 to clear."]
        #[inline(always)]
        pub const fn gwc_fail_sts(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "graphic window check fail interrupt status. will be set if the calculated CRC not equal reference CRC. one bit for each channel. software write 1 to clear."]
        #[inline(always)]
        pub fn set_gwc_fail_sts(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "error status, it's OR of GWC_FAIL_STS\\[15:0\\]."]
        #[inline(always)]
        pub const fn err_sts(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "error status, it's OR of GWC_FAIL_STS\\[15:0\\]."]
        #[inline(always)]
        pub fn set_err_sts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "function interrupt status. it's set when detect two VSYNC signals after the block is enabled(GWC_EN is set) software write 1 to clear."]
        #[inline(always)]
        pub const fn func_sts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "function interrupt status. it's set when detect two VSYNC signals after the block is enabled(GWC_EN is set) software write 1 to clear."]
        #[inline(always)]
        pub fn set_func_sts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for IrqSts {
        #[inline(always)]
        fn default() -> IrqSts {
            IrqSts(0)
        }
    }
    #[doc = "reference CRC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Refcrc(pub u32);
    impl Refcrc {
        #[doc = "reference CRC polynomial function: 0x104C11DB7."]
        #[inline(always)]
        pub const fn ref_crc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "reference CRC polynomial function: 0x104C11DB7."]
        #[inline(always)]
        pub fn set_ref_crc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Refcrc {
        #[inline(always)]
        fn default() -> Refcrc {
            Refcrc(0)
        }
    }
}
