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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "config reg 1."]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "reference CRC."]
    #[inline(always)]
    pub const fn refcrc(self) -> crate::common::Reg<regs::Refcrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "calculated CRC."]
    #[inline(always)]
    pub const fn calcrc(self) -> crate::common::Reg<regs::Calcrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "interrupt enable."]
    #[inline(always)]
    pub const fn irq_mask(self) -> crate::common::Reg<regs::IrqMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "interrupt status."]
    #[inline(always)]
    pub const fn irq_sts(self) -> crate::common::Reg<regs::IrqSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn channel(self, n: usize) -> Channel {
        assert!(n < 2usize);
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x10usize + n * 240usize) as _) }
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
    #[doc = "calculated CRC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calcrc(pub u32);
    impl Calcrc {
        #[doc = "calculated CRC for last frame."]
        #[must_use]
        #[inline(always)]
        pub const fn cal_crc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "calculated CRC for last frame."]
        #[inline(always)]
        pub const fn set_cal_crc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Calcrc {
        #[inline(always)]
        fn default() -> Calcrc {
            Calcrc(0)
        }
    }
    impl core::fmt::Debug for Calcrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calcrc")
                .field("cal_crc", &self.cal_crc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calcrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Calcrc {{ cal_crc: {=u32:?} }}", self.cal_crc())
        }
    }
    #[doc = "config reg 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg0(pub u32);
    impl Cfg0 {
        #[doc = "define the window start column number."]
        #[must_use]
        #[inline(always)]
        pub const fn start_col(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "define the window start column number."]
        #[inline(always)]
        pub const fn set_start_col(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "define the window start row number."]
        #[must_use]
        #[inline(always)]
        pub const fn start_row(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "define the window start row number."]
        #[inline(always)]
        pub const fn set_start_row(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "freeze config. set to freeze all other config registers for current channel. can only be cleared by system reset."]
        #[must_use]
        #[inline(always)]
        pub const fn freeze(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "freeze config. set to freeze all other config registers for current channel. can only be cleared by system reset."]
        #[inline(always)]
        pub const fn set_freeze(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "channel enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "channel enable."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg0 {
        #[inline(always)]
        fn default() -> Cfg0 {
            Cfg0(0)
        }
    }
    impl core::fmt::Debug for Cfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg0")
                .field("start_col", &self.start_col())
                .field("start_row", &self.start_row())
                .field("freeze", &self.freeze())
                .field("enable", &self.enable())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfg0 {{ start_col: {=u16:?}, start_row: {=u16:?}, freeze: {=bool:?}, enable: {=bool:?} }}" , self . start_col () , self . start_row () , self . freeze () , self . enable ())
        }
    }
    #[doc = "config reg 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg1(pub u32);
    impl Cfg1 {
        #[doc = "define the window end column number."]
        #[must_use]
        #[inline(always)]
        pub const fn end_col(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "define the window end column number."]
        #[inline(always)]
        pub const fn set_end_col(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "define the window end row number."]
        #[must_use]
        #[inline(always)]
        pub const fn end_row(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "define the window end row number."]
        #[inline(always)]
        pub const fn set_end_row(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Cfg1 {
        #[inline(always)]
        fn default() -> Cfg1 {
            Cfg1(0)
        }
    }
    impl core::fmt::Debug for Cfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg1")
                .field("end_col", &self.end_col())
                .field("end_row", &self.end_row())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg1 {{ end_col: {=u16:?}, end_row: {=u16:?} }}",
                self.end_col(),
                self.end_row()
            )
        }
    }
    #[doc = "control reg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlbCtrl(pub u32);
    impl GlbCtrl {
        #[doc = "graphic window check enable. set to enable the whole block."]
        #[must_use]
        #[inline(always)]
        pub const fn gwc_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "graphic window check enable. set to enable the whole block."]
        #[inline(always)]
        pub const fn set_gwc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "graphic clock polarity. set to invert input graphic clock."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_pol(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "graphic clock polarity. set to invert input graphic clock."]
        #[inline(always)]
        pub const fn set_clk_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for GlbCtrl {
        #[inline(always)]
        fn default() -> GlbCtrl {
            GlbCtrl(0)
        }
    }
    impl core::fmt::Debug for GlbCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GlbCtrl")
                .field("gwc_en", &self.gwc_en())
                .field("clk_pol", &self.clk_pol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GlbCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GlbCtrl {{ gwc_en: {=bool:?}, clk_pol: {=bool:?} }}",
                self.gwc_en(),
                self.clk_pol()
            )
        }
    }
    #[doc = "interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqMask(pub u32);
    impl IrqMask {
        #[doc = "error interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn err_mask(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "error interrupt mask."]
        #[inline(always)]
        pub const fn set_err_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "function interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn func_mask(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "function interrupt mask."]
        #[inline(always)]
        pub const fn set_func_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "freeze mask, set to disable changing ERR_MASK and FUNC_MASK. can only be cleared by system reset."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_rreez(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "freeze mask, set to disable changing ERR_MASK and FUNC_MASK. can only be cleared by system reset."]
        #[inline(always)]
        pub const fn set_mask_rreez(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for IrqMask {
        #[inline(always)]
        fn default() -> IrqMask {
            IrqMask(0)
        }
    }
    impl core::fmt::Debug for IrqMask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqMask")
                .field("err_mask", &self.err_mask())
                .field("func_mask", &self.func_mask())
                .field("mask_rreez", &self.mask_rreez())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqMask {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IrqMask {{ err_mask: {=bool:?}, func_mask: {=bool:?}, mask_rreez: {=bool:?} }}",
                self.err_mask(),
                self.func_mask(),
                self.mask_rreez()
            )
        }
    }
    #[doc = "interrupt status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqSts(pub u32);
    impl IrqSts {
        #[doc = "graphic window check fail interrupt status. will be set if the calculated CRC not equal reference CRC. one bit for each channel. software write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn gwc_fail_sts(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "graphic window check fail interrupt status. will be set if the calculated CRC not equal reference CRC. one bit for each channel. software write 1 to clear."]
        #[inline(always)]
        pub const fn set_gwc_fail_sts(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "error status, it's OR of GWC_FAIL_STS\\[15:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn err_sts(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "error status, it's OR of GWC_FAIL_STS\\[15:0\\]."]
        #[inline(always)]
        pub const fn set_err_sts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "function interrupt status. it's set when detect two VSYNC signals after the block is enabled(GWC_EN is set) software write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn func_sts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "function interrupt status. it's set when detect two VSYNC signals after the block is enabled(GWC_EN is set) software write 1 to clear."]
        #[inline(always)]
        pub const fn set_func_sts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for IrqSts {
        #[inline(always)]
        fn default() -> IrqSts {
            IrqSts(0)
        }
    }
    impl core::fmt::Debug for IrqSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqSts")
                .field("gwc_fail_sts", &self.gwc_fail_sts())
                .field("err_sts", &self.err_sts())
                .field("func_sts", &self.func_sts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IrqSts {{ gwc_fail_sts: {=u16:?}, err_sts: {=bool:?}, func_sts: {=bool:?} }}",
                self.gwc_fail_sts(),
                self.err_sts(),
                self.func_sts()
            )
        }
    }
    #[doc = "reference CRC."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Refcrc(pub u32);
    impl Refcrc {
        #[doc = "reference CRC polynomial function: 0x104C11DB7."]
        #[must_use]
        #[inline(always)]
        pub const fn ref_crc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "reference CRC polynomial function: 0x104C11DB7."]
        #[inline(always)]
        pub const fn set_ref_crc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Refcrc {
        #[inline(always)]
        fn default() -> Refcrc {
            Refcrc(0)
        }
    }
    impl core::fmt::Debug for Refcrc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Refcrc")
                .field("ref_crc", &self.ref_crc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Refcrc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Refcrc {{ ref_crc: {=u32:?} }}", self.ref_crc())
        }
    }
}
