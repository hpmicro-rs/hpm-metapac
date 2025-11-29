#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chctrl {
    ptr: *mut u8,
}
unsafe impl Send for Chctrl {}
unsafe impl Sync for Chctrl {}
impl Chctrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Channel &index0 Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Channel &index0Transfer Size Register."]
    #[inline(always)]
    pub const fn tran_size(self) -> crate::common::Reg<regs::TranSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel &index0 Source Address Low Part Register."]
    #[inline(always)]
    pub const fn src_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel &index0 DMA Request Control Register."]
    #[inline(always)]
    pub const fn chan_req_ctrl(self) -> crate::common::Reg<regs::ChanReqCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel &index0 Destination Address Low Part Register."]
    #[inline(always)]
    pub const fn dst_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "swap table register."]
    #[inline(always)]
    pub const fn swap_table(self) -> crate::common::Reg<regs::SwapTable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Channel &index0 Linked List Pointer Low Part Register."]
    #[inline(always)]
    pub const fn llpointer(self) -> crate::common::Reg<regs::Llpointer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
#[doc = "HDMA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
unsafe impl Send for Dma {}
unsafe impl Sync for Dma {}
impl Dma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ID Misc."]
    #[inline(always)]
    pub const fn idmisc(self) -> crate::common::Reg<regs::Idmisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DMAC Configuration Register."]
    #[inline(always)]
    pub const fn dmacfg(self) -> crate::common::Reg<regs::Dmacfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DMAC Control Register."]
    #[inline(always)]
    pub const fn dmactrl(self) -> crate::common::Reg<regs::Dmactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Channel Abort Register."]
    #[inline(always)]
    pub const fn ch_abort(self) -> crate::common::Reg<regs::ChAbort, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Harlf Complete Interrupt Status."]
    #[inline(always)]
    pub const fn inthalfsts(self) -> crate::common::Reg<regs::Inthalfsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Trans Complete Interrupt Status Register."]
    #[inline(always)]
    pub const fn inttcsts(self) -> crate::common::Reg<regs::Inttcsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Abort Interrupt Status Register."]
    #[inline(always)]
    pub const fn intabortsts(self) -> crate::common::Reg<regs::Intabortsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Error Interrupt Status Register."]
    #[inline(always)]
    pub const fn interrsts(self) -> crate::common::Reg<regs::Interrsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Channel Enable Register."]
    #[inline(always)]
    pub const fn ch_en(self) -> crate::common::Reg<regs::ChEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn chctrl(self, n: usize) -> Chctrl {
        assert!(n < 32usize);
        unsafe { Chctrl::from_ptr(self.ptr.wrapping_add(0x40usize + n * 32usize) as _) }
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
    #[doc = "Channel Abort Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChAbort(pub u32);
    impl ChAbort {
        #[doc = "Write 1 to bit n to abort channel n. The bits should only be set when the corresponding channels are enabled. Otherwise, the writes will be ignored for channels that are not enabled. (N: Number of channels)."]
        #[must_use]
        #[inline(always)]
        pub const fn chabort(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Write 1 to bit n to abort channel n. The bits should only be set when the corresponding channels are enabled. Otherwise, the writes will be ignored for channels that are not enabled. (N: Number of channels)."]
        #[inline(always)]
        pub const fn set_chabort(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for ChAbort {
        #[inline(always)]
        fn default() -> ChAbort {
            ChAbort(0)
        }
    }
    impl core::fmt::Debug for ChAbort {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ChAbort")
                .field("chabort[0]", &self.chabort(0usize))
                .field("chabort[1]", &self.chabort(1usize))
                .field("chabort[2]", &self.chabort(2usize))
                .field("chabort[3]", &self.chabort(3usize))
                .field("chabort[4]", &self.chabort(4usize))
                .field("chabort[5]", &self.chabort(5usize))
                .field("chabort[6]", &self.chabort(6usize))
                .field("chabort[7]", &self.chabort(7usize))
                .field("chabort[8]", &self.chabort(8usize))
                .field("chabort[9]", &self.chabort(9usize))
                .field("chabort[10]", &self.chabort(10usize))
                .field("chabort[11]", &self.chabort(11usize))
                .field("chabort[12]", &self.chabort(12usize))
                .field("chabort[13]", &self.chabort(13usize))
                .field("chabort[14]", &self.chabort(14usize))
                .field("chabort[15]", &self.chabort(15usize))
                .field("chabort[16]", &self.chabort(16usize))
                .field("chabort[17]", &self.chabort(17usize))
                .field("chabort[18]", &self.chabort(18usize))
                .field("chabort[19]", &self.chabort(19usize))
                .field("chabort[20]", &self.chabort(20usize))
                .field("chabort[21]", &self.chabort(21usize))
                .field("chabort[22]", &self.chabort(22usize))
                .field("chabort[23]", &self.chabort(23usize))
                .field("chabort[24]", &self.chabort(24usize))
                .field("chabort[25]", &self.chabort(25usize))
                .field("chabort[26]", &self.chabort(26usize))
                .field("chabort[27]", &self.chabort(27usize))
                .field("chabort[28]", &self.chabort(28usize))
                .field("chabort[29]", &self.chabort(29usize))
                .field("chabort[30]", &self.chabort(30usize))
                .field("chabort[31]", &self.chabort(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ChAbort {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ChAbort {{ chabort[0]: {=bool:?}, chabort[1]: {=bool:?}, chabort[2]: {=bool:?}, chabort[3]: {=bool:?}, chabort[4]: {=bool:?}, chabort[5]: {=bool:?}, chabort[6]: {=bool:?}, chabort[7]: {=bool:?}, chabort[8]: {=bool:?}, chabort[9]: {=bool:?}, chabort[10]: {=bool:?}, chabort[11]: {=bool:?}, chabort[12]: {=bool:?}, chabort[13]: {=bool:?}, chabort[14]: {=bool:?}, chabort[15]: {=bool:?}, chabort[16]: {=bool:?}, chabort[17]: {=bool:?}, chabort[18]: {=bool:?}, chabort[19]: {=bool:?}, chabort[20]: {=bool:?}, chabort[21]: {=bool:?}, chabort[22]: {=bool:?}, chabort[23]: {=bool:?}, chabort[24]: {=bool:?}, chabort[25]: {=bool:?}, chabort[26]: {=bool:?}, chabort[27]: {=bool:?}, chabort[28]: {=bool:?}, chabort[29]: {=bool:?}, chabort[30]: {=bool:?}, chabort[31]: {=bool:?} }}" , self . chabort (0usize) , self . chabort (1usize) , self . chabort (2usize) , self . chabort (3usize) , self . chabort (4usize) , self . chabort (5usize) , self . chabort (6usize) , self . chabort (7usize) , self . chabort (8usize) , self . chabort (9usize) , self . chabort (10usize) , self . chabort (11usize) , self . chabort (12usize) , self . chabort (13usize) , self . chabort (14usize) , self . chabort (15usize) , self . chabort (16usize) , self . chabort (17usize) , self . chabort (18usize) , self . chabort (19usize) , self . chabort (20usize) , self . chabort (21usize) , self . chabort (22usize) , self . chabort (23usize) , self . chabort (24usize) , self . chabort (25usize) , self . chabort (26usize) , self . chabort (27usize) , self . chabort (28usize) , self . chabort (29usize) , self . chabort (30usize) , self . chabort (31usize))
        }
    }
    #[doc = "Channel Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChEn(pub u32);
    impl ChEn {
        #[doc = "Alias of the Enable field of all ChnCtrl registers."]
        #[must_use]
        #[inline(always)]
        pub const fn chen(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alias of the Enable field of all ChnCtrl registers."]
        #[inline(always)]
        pub const fn set_chen(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for ChEn {
        #[inline(always)]
        fn default() -> ChEn {
            ChEn(0)
        }
    }
    impl core::fmt::Debug for ChEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ChEn")
                .field("chen[0]", &self.chen(0usize))
                .field("chen[1]", &self.chen(1usize))
                .field("chen[2]", &self.chen(2usize))
                .field("chen[3]", &self.chen(3usize))
                .field("chen[4]", &self.chen(4usize))
                .field("chen[5]", &self.chen(5usize))
                .field("chen[6]", &self.chen(6usize))
                .field("chen[7]", &self.chen(7usize))
                .field("chen[8]", &self.chen(8usize))
                .field("chen[9]", &self.chen(9usize))
                .field("chen[10]", &self.chen(10usize))
                .field("chen[11]", &self.chen(11usize))
                .field("chen[12]", &self.chen(12usize))
                .field("chen[13]", &self.chen(13usize))
                .field("chen[14]", &self.chen(14usize))
                .field("chen[15]", &self.chen(15usize))
                .field("chen[16]", &self.chen(16usize))
                .field("chen[17]", &self.chen(17usize))
                .field("chen[18]", &self.chen(18usize))
                .field("chen[19]", &self.chen(19usize))
                .field("chen[20]", &self.chen(20usize))
                .field("chen[21]", &self.chen(21usize))
                .field("chen[22]", &self.chen(22usize))
                .field("chen[23]", &self.chen(23usize))
                .field("chen[24]", &self.chen(24usize))
                .field("chen[25]", &self.chen(25usize))
                .field("chen[26]", &self.chen(26usize))
                .field("chen[27]", &self.chen(27usize))
                .field("chen[28]", &self.chen(28usize))
                .field("chen[29]", &self.chen(29usize))
                .field("chen[30]", &self.chen(30usize))
                .field("chen[31]", &self.chen(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ChEn {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ChEn {{ chen[0]: {=bool:?}, chen[1]: {=bool:?}, chen[2]: {=bool:?}, chen[3]: {=bool:?}, chen[4]: {=bool:?}, chen[5]: {=bool:?}, chen[6]: {=bool:?}, chen[7]: {=bool:?}, chen[8]: {=bool:?}, chen[9]: {=bool:?}, chen[10]: {=bool:?}, chen[11]: {=bool:?}, chen[12]: {=bool:?}, chen[13]: {=bool:?}, chen[14]: {=bool:?}, chen[15]: {=bool:?}, chen[16]: {=bool:?}, chen[17]: {=bool:?}, chen[18]: {=bool:?}, chen[19]: {=bool:?}, chen[20]: {=bool:?}, chen[21]: {=bool:?}, chen[22]: {=bool:?}, chen[23]: {=bool:?}, chen[24]: {=bool:?}, chen[25]: {=bool:?}, chen[26]: {=bool:?}, chen[27]: {=bool:?}, chen[28]: {=bool:?}, chen[29]: {=bool:?}, chen[30]: {=bool:?}, chen[31]: {=bool:?} }}" , self . chen (0usize) , self . chen (1usize) , self . chen (2usize) , self . chen (3usize) , self . chen (4usize) , self . chen (5usize) , self . chen (6usize) , self . chen (7usize) , self . chen (8usize) , self . chen (9usize) , self . chen (10usize) , self . chen (11usize) , self . chen (12usize) , self . chen (13usize) , self . chen (14usize) , self . chen (15usize) , self . chen (16usize) , self . chen (17usize) , self . chen (18usize) , self . chen (19usize) , self . chen (20usize) , self . chen (21usize) , self . chen (22usize) , self . chen (23usize) , self . chen (24usize) , self . chen (25usize) , self . chen (26usize) , self . chen (27usize) , self . chen (28usize) , self . chen (29usize) , self . chen (30usize) , self . chen (31usize))
        }
    }
    #[doc = "Channel &index0 DMA Request Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChanReqCtrl(pub u32);
    impl ChanReqCtrl {
        #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
        #[must_use]
        #[inline(always)]
        pub const fn dstreqsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
        #[inline(always)]
        pub const fn set_dstreqsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
        #[must_use]
        #[inline(always)]
        pub const fn srcreqsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
        #[inline(always)]
        pub const fn set_srcreqsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for ChanReqCtrl {
        #[inline(always)]
        fn default() -> ChanReqCtrl {
            ChanReqCtrl(0)
        }
    }
    impl core::fmt::Debug for ChanReqCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ChanReqCtrl")
                .field("dstreqsel", &self.dstreqsel())
                .field("srcreqsel", &self.srcreqsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ChanReqCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ChanReqCtrl {{ dstreqsel: {=u8:?}, srcreqsel: {=u8:?} }}",
                self.dstreqsel(),
                self.srcreqsel()
            )
        }
    }
    #[doc = "Channel &index0 Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Channel enable bit 0x0: Disable 0x1: Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel enable bit 0x0: Disable 0x1: Enable."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn inttcmask(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt."]
        #[inline(always)]
        pub const fn set_inttcmask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn interrmask(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt."]
        #[inline(always)]
        pub const fn set_interrmask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn intabtmask(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt."]
        #[inline(always)]
        pub const fn set_intabtmask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Channel half interrupt mask 0x0: Allow the half interrupt to be triggered 0x1: Disable the half interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn inthalfcntmask(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Channel half interrupt mask 0x0: Allow the half interrupt to be triggered 0x1: Disable the half interrupt."]
        #[inline(always)]
        pub const fn set_inthalfcntmask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "00 ：use swap table(if swap table all 0, then act nothing) 01 : byte swap 10 : 16bit swap 11 : 32bit swap."]
        #[must_use]
        #[inline(always)]
        pub const fn swap_ctl(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "00 ：use swap table(if swap table all 0, then act nothing) 01 : byte swap 10 : 16bit swap 11 : 32bit swap."]
        #[inline(always)]
        pub const fn set_swap_ctl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "set to loop current burst, omit setting in dst_addr_ctrl."]
        #[must_use]
        #[inline(always)]
        pub const fn dst_fixburst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "set to loop current burst, omit setting in dst_addr_ctrl."]
        #[inline(always)]
        pub const fn set_dst_fixburst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "set to loop current burst, omit setting in src_addr_ctrl."]
        #[must_use]
        #[inline(always)]
        pub const fn src_fixburst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "set to loop current burst, omit setting in src_addr_ctrl."]
        #[inline(always)]
        pub const fn set_src_fixburst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception."]
        #[must_use]
        #[inline(always)]
        pub const fn dstaddrctrl(&self) -> super::vals::AddrCtrl {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::AddrCtrl::from_bits(val as u8)
        }
        #[doc = "Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception."]
        #[inline(always)]
        pub const fn set_dstaddrctrl(&mut self, val: super::vals::AddrCtrl) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception."]
        #[must_use]
        #[inline(always)]
        pub const fn srcaddrctrl(&self) -> super::vals::AddrCtrl {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::AddrCtrl::from_bits(val as u8)
        }
        #[doc = "Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception."]
        #[inline(always)]
        pub const fn set_srcaddrctrl(&mut self, val: super::vals::AddrCtrl) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode the difference bewteen Source/Destination handshake mode is: the dma block will response hardware request after read in Source handshake mode; the dma block will response hardware request after write in Destination handshake mode; NOTE: can't set SrcMode and DstMode at same time, otherwise result unknown."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Mode {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode the difference bewteen Source/Destination handshake mode is: the dma block will response hardware request after read in Source handshake mode; the dma block will response hardware request after write in Destination handshake mode; NOTE: can't set SrcMode and DstMode at same time, otherwise result unknown."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode Normal mode is enabled and started by software set Enable bit; Handshake mode is enabled by software set Enable bit, started by hardware dma request from DMAMUX block."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Mode {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode Normal mode is enabled and started by software set Enable bit; Handshake mode is enabled by software set Enable bit, started by hardware dma request from DMAMUX block."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6 - 0x7: Reserved, setting this field with a reserved value triggers the error exception."]
        #[must_use]
        #[inline(always)]
        pub const fn dstwidth(&self) -> super::vals::Width {
            let val = (self.0 >> 18usize) & 0x07;
            super::vals::Width::from_bits(val as u8)
        }
        #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6 - 0x7: Reserved, setting this field with a reserved value triggers the error exception."]
        #[inline(always)]
        pub const fn set_dstwidth(&mut self, val: super::vals::Width) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
        }
        #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6 - 0x7: Reserved, setting this field with a reserved value triggers the error exception."]
        #[must_use]
        #[inline(always)]
        pub const fn srcwidth(&self) -> super::vals::Width {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Width::from_bits(val as u8)
        }
        #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6 - 0x7: Reserved, setting this field with a reserved value triggers the error exception."]
        #[inline(always)]
        pub const fn set_srcwidth(&mut self, val: super::vals::Width) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb - 0xf: Reserved, setting this field with a reserved value triggers the error exception."]
        #[must_use]
        #[inline(always)]
        pub const fn srcburstsize(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb - 0xf: Reserved, setting this field with a reserved value triggers the error exception."]
        #[inline(always)]
        pub const fn set_srcburstsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "set to change burst_size definition."]
        #[must_use]
        #[inline(always)]
        pub const fn burstopt(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "set to change burst_size definition."]
        #[inline(always)]
        pub const fn set_burstopt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Channel priority level 0x0: Lower priority 0x1: Higher priority."]
        #[must_use]
        #[inline(always)]
        pub const fn priority(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Channel priority level 0x0: Lower priority 0x1: Higher priority."]
        #[inline(always)]
        pub const fn set_priority(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "0: one request to transfer one burst 1: one request to transfer all the data defined in ch_tts."]
        #[must_use]
        #[inline(always)]
        pub const fn handshakeopt(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "0: one request to transfer one burst 1: one request to transfer all the data defined in ch_tts."]
        #[inline(always)]
        pub const fn set_handshakeopt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "set to loop current config infinitely."]
        #[must_use]
        #[inline(always)]
        pub const fn infiniteloop(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "set to loop current config infinitely."]
        #[inline(always)]
        pub const fn set_infiniteloop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl")
                .field("enable", &self.enable())
                .field("inttcmask", &self.inttcmask())
                .field("interrmask", &self.interrmask())
                .field("intabtmask", &self.intabtmask())
                .field("inthalfcntmask", &self.inthalfcntmask())
                .field("swap_ctl", &self.swap_ctl())
                .field("dst_fixburst", &self.dst_fixburst())
                .field("src_fixburst", &self.src_fixburst())
                .field("dstaddrctrl", &self.dstaddrctrl())
                .field("srcaddrctrl", &self.srcaddrctrl())
                .field("dstmode", &self.dstmode())
                .field("srcmode", &self.srcmode())
                .field("dstwidth", &self.dstwidth())
                .field("srcwidth", &self.srcwidth())
                .field("srcburstsize", &self.srcburstsize())
                .field("burstopt", &self.burstopt())
                .field("priority", &self.priority())
                .field("handshakeopt", &self.handshakeopt())
                .field("infiniteloop", &self.infiniteloop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl {{ enable: {=bool:?}, inttcmask: {=bool:?}, interrmask: {=bool:?}, intabtmask: {=bool:?}, inthalfcntmask: {=bool:?}, swap_ctl: {=u8:?}, dst_fixburst: {=bool:?}, src_fixburst: {=bool:?}, dstaddrctrl: {:?}, srcaddrctrl: {:?}, dstmode: {:?}, srcmode: {:?}, dstwidth: {:?}, srcwidth: {:?}, srcburstsize: {=u8:?}, burstopt: {=bool:?}, priority: {=bool:?}, handshakeopt: {=bool:?}, infiniteloop: {=bool:?} }}" , self . enable () , self . inttcmask () , self . interrmask () , self . intabtmask () , self . inthalfcntmask () , self . swap_ctl () , self . dst_fixburst () , self . src_fixburst () , self . dstaddrctrl () , self . srcaddrctrl () , self . dstmode () , self . srcmode () , self . dstwidth () , self . srcwidth () , self . srcburstsize () , self . burstopt () , self . priority () , self . handshakeopt () , self . infiniteloop ())
        }
    }
    #[doc = "DMAC Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacfg(pub u32);
    impl Dmacfg {
        #[doc = "Channel number 0x1: 1 channel 0x2: 2 channels ... 0x8: 8 channels Others: Invalid."]
        #[must_use]
        #[inline(always)]
        pub const fn channelnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Channel number 0x1: 1 channel 0x2: 2 channels ... 0x8: 8 channels Others: Invalid."]
        #[inline(always)]
        pub const fn set_channelnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "FIFO depth 0x4: 4 entries 0x8: 8 entries 0x10: 16 entries 0x20: 32 entries Others: Invalid."]
        #[must_use]
        #[inline(always)]
        pub const fn fifodepth(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x3f;
            val as u8
        }
        #[doc = "FIFO depth 0x4: 4 entries 0x8: 8 entries 0x10: 16 entries 0x20: 32 entries Others: Invalid."]
        #[inline(always)]
        pub const fn set_fifodepth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
        }
        #[doc = "Request/acknowledge pair number 0x0: 0 pair 0x1: 1 pair 0x2: 2 pairs ... 0x10: 16 pairs."]
        #[must_use]
        #[inline(always)]
        pub const fn reqnum(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "Request/acknowledge pair number 0x0: 0 pair 0x1: 1 pair 0x2: 2 pairs ... 0x10: 16 pairs."]
        #[inline(always)]
        pub const fn set_reqnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "AXI bus interface number 0x0: 1 AXI bus 0x1: 2 AXI busses."]
        #[must_use]
        #[inline(always)]
        pub const fn busnum(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "AXI bus interface number 0x0: 1 AXI bus 0x1: 2 AXI busses."]
        #[inline(always)]
        pub const fn set_busnum(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "DMA core number 0x0: 1 core 0x1: 2 cores."]
        #[must_use]
        #[inline(always)]
        pub const fn corenum(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DMA core number 0x0: 1 core 0x1: 2 cores."]
        #[inline(always)]
        pub const fn set_corenum(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "AXI bus address width 0x18: 24 bits 0x19: 25 bits ... 0x40: 64 bits Others: Invalid."]
        #[must_use]
        #[inline(always)]
        pub const fn addrwidth(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[doc = "AXI bus address width 0x18: 24 bits 0x19: 25 bits ... 0x40: 64 bits Others: Invalid."]
        #[inline(always)]
        pub const fn set_addrwidth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
        #[doc = "AXI bus data width 0x0: 32 bits 0x1: 64 bits 0x2: 128 bits 0x3: 256 bits."]
        #[must_use]
        #[inline(always)]
        pub const fn datawidth(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "AXI bus data width 0x0: 32 bits 0x1: 64 bits 0x2: 128 bits 0x3: 256 bits."]
        #[inline(always)]
        pub const fn set_datawidth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "DMA request synchronization. The DMA request synchronization should be configured to avoid signal integrity problems when the request signal is not clocked by the system bus clock, which the DMA control logic operates in. If the request synchronization is not configured, the request signal is sampled directly without synchronization. 0x0: Request synchronization is not configured 0x1: Request synchronization is configured."]
        #[must_use]
        #[inline(always)]
        pub const fn reqsync(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DMA request synchronization. The DMA request synchronization should be configured to avoid signal integrity problems when the request signal is not clocked by the system bus clock, which the DMA control logic operates in. If the request synchronization is not configured, the request signal is sampled directly without synchronization. 0x0: Request synchronization is not configured 0x1: Request synchronization is configured."]
        #[inline(always)]
        pub const fn set_reqsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Chain transfer 0x0: Chain transfer is not configured 0x1: Chain transfer is configured."]
        #[must_use]
        #[inline(always)]
        pub const fn chainxfr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Chain transfer 0x0: Chain transfer is not configured 0x1: Chain transfer is configured."]
        #[inline(always)]
        pub const fn set_chainxfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dmacfg {
        #[inline(always)]
        fn default() -> Dmacfg {
            Dmacfg(0)
        }
    }
    impl core::fmt::Debug for Dmacfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmacfg")
                .field("channelnum", &self.channelnum())
                .field("fifodepth", &self.fifodepth())
                .field("reqnum", &self.reqnum())
                .field("busnum", &self.busnum())
                .field("corenum", &self.corenum())
                .field("addrwidth", &self.addrwidth())
                .field("datawidth", &self.datawidth())
                .field("reqsync", &self.reqsync())
                .field("chainxfr", &self.chainxfr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmacfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dmacfg {{ channelnum: {=u8:?}, fifodepth: {=u8:?}, reqnum: {=u8:?}, busnum: {=bool:?}, corenum: {=bool:?}, addrwidth: {=u8:?}, datawidth: {=u8:?}, reqsync: {=bool:?}, chainxfr: {=bool:?} }}" , self . channelnum () , self . fifodepth () , self . reqnum () , self . busnum () , self . corenum () , self . addrwidth () , self . datawidth () , self . reqsync () , self . chainxfr ())
        }
    }
    #[doc = "DMAC Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmactrl(pub u32);
    impl Dmactrl {
        #[doc = "Software reset control. Write 1 to this bit to reset the DMA core and disable all channels. Note: The software reset may cause the in-completion of AXI transaction."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset control. Write 1 to this bit to reset the DMA core and disable all channels. Note: The software reset may cause the in-completion of AXI transaction."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Dmactrl {
        #[inline(always)]
        fn default() -> Dmactrl {
            Dmactrl(0)
        }
    }
    impl core::fmt::Debug for Dmactrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dmactrl")
                .field("reset", &self.reset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dmactrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dmactrl {{ reset: {=bool:?} }}", self.reset())
        }
    }
    #[doc = "ID Misc."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idmisc(pub u32);
    impl Idmisc {
        #[doc = "current channel in used."]
        #[must_use]
        #[inline(always)]
        pub const fn curchan(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "current channel in used."]
        #[inline(always)]
        pub const fn set_curchan(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "DMA state machine localparam ST_IDLE = 3'b000; localparam ST_READ = 3'b001; localparam ST_READ_ACK = 3'b010; localparam ST_WRITE = 3'b011; localparam ST_WRITE_ACK = 3'b100; localparam ST_LL = 3'b101; localparam ST_END = 3'b110; localparam ST_END_WAIT = 3'b111;."]
        #[must_use]
        #[inline(always)]
        pub const fn dmastate(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "DMA state machine localparam ST_IDLE = 3'b000; localparam ST_READ = 3'b001; localparam ST_READ_ACK = 3'b010; localparam ST_WRITE = 3'b011; localparam ST_WRITE_ACK = 3'b100; localparam ST_LL = 3'b101; localparam ST_END = 3'b110; localparam ST_END_WAIT = 3'b111;."]
        #[inline(always)]
        pub const fn set_dmastate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
    }
    impl Default for Idmisc {
        #[inline(always)]
        fn default() -> Idmisc {
            Idmisc(0)
        }
    }
    impl core::fmt::Debug for Idmisc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idmisc")
                .field("curchan", &self.curchan())
                .field("dmastate", &self.dmastate())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idmisc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idmisc {{ curchan: {=u8:?}, dmastate: {=u8:?} }}",
                self.curchan(),
                self.dmastate()
            )
        }
    }
    #[doc = "Abort Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intabortsts(pub u32);
    impl Intabortsts {
        #[doc = "The abort status of channel, one bit per channel. The abort status is set when a channel transfer is aborted. 0x0: Channel n has no abort status 0x1: Channel n has abort status."]
        #[must_use]
        #[inline(always)]
        pub const fn sts(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "The abort status of channel, one bit per channel. The abort status is set when a channel transfer is aborted. 0x0: Channel n has no abort status 0x1: Channel n has abort status."]
        #[inline(always)]
        pub const fn set_sts(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Intabortsts {
        #[inline(always)]
        fn default() -> Intabortsts {
            Intabortsts(0)
        }
    }
    impl core::fmt::Debug for Intabortsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intabortsts")
                .field("sts[0]", &self.sts(0usize))
                .field("sts[1]", &self.sts(1usize))
                .field("sts[2]", &self.sts(2usize))
                .field("sts[3]", &self.sts(3usize))
                .field("sts[4]", &self.sts(4usize))
                .field("sts[5]", &self.sts(5usize))
                .field("sts[6]", &self.sts(6usize))
                .field("sts[7]", &self.sts(7usize))
                .field("sts[8]", &self.sts(8usize))
                .field("sts[9]", &self.sts(9usize))
                .field("sts[10]", &self.sts(10usize))
                .field("sts[11]", &self.sts(11usize))
                .field("sts[12]", &self.sts(12usize))
                .field("sts[13]", &self.sts(13usize))
                .field("sts[14]", &self.sts(14usize))
                .field("sts[15]", &self.sts(15usize))
                .field("sts[16]", &self.sts(16usize))
                .field("sts[17]", &self.sts(17usize))
                .field("sts[18]", &self.sts(18usize))
                .field("sts[19]", &self.sts(19usize))
                .field("sts[20]", &self.sts(20usize))
                .field("sts[21]", &self.sts(21usize))
                .field("sts[22]", &self.sts(22usize))
                .field("sts[23]", &self.sts(23usize))
                .field("sts[24]", &self.sts(24usize))
                .field("sts[25]", &self.sts(25usize))
                .field("sts[26]", &self.sts(26usize))
                .field("sts[27]", &self.sts(27usize))
                .field("sts[28]", &self.sts(28usize))
                .field("sts[29]", &self.sts(29usize))
                .field("sts[30]", &self.sts(30usize))
                .field("sts[31]", &self.sts(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intabortsts {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Intabortsts {{ sts[0]: {=bool:?}, sts[1]: {=bool:?}, sts[2]: {=bool:?}, sts[3]: {=bool:?}, sts[4]: {=bool:?}, sts[5]: {=bool:?}, sts[6]: {=bool:?}, sts[7]: {=bool:?}, sts[8]: {=bool:?}, sts[9]: {=bool:?}, sts[10]: {=bool:?}, sts[11]: {=bool:?}, sts[12]: {=bool:?}, sts[13]: {=bool:?}, sts[14]: {=bool:?}, sts[15]: {=bool:?}, sts[16]: {=bool:?}, sts[17]: {=bool:?}, sts[18]: {=bool:?}, sts[19]: {=bool:?}, sts[20]: {=bool:?}, sts[21]: {=bool:?}, sts[22]: {=bool:?}, sts[23]: {=bool:?}, sts[24]: {=bool:?}, sts[25]: {=bool:?}, sts[26]: {=bool:?}, sts[27]: {=bool:?}, sts[28]: {=bool:?}, sts[29]: {=bool:?}, sts[30]: {=bool:?}, sts[31]: {=bool:?} }}" , self . sts (0usize) , self . sts (1usize) , self . sts (2usize) , self . sts (3usize) , self . sts (4usize) , self . sts (5usize) , self . sts (6usize) , self . sts (7usize) , self . sts (8usize) , self . sts (9usize) , self . sts (10usize) , self . sts (11usize) , self . sts (12usize) , self . sts (13usize) , self . sts (14usize) , self . sts (15usize) , self . sts (16usize) , self . sts (17usize) , self . sts (18usize) , self . sts (19usize) , self . sts (20usize) , self . sts (21usize) , self . sts (22usize) , self . sts (23usize) , self . sts (24usize) , self . sts (25usize) , self . sts (26usize) , self . sts (27usize) , self . sts (28usize) , self . sts (29usize) , self . sts (30usize) , self . sts (31usize))
        }
    }
    #[doc = "Error Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Interrsts(pub u32);
    impl Interrsts {
        #[doc = "The error status, one bit per channel. The error status is set when a channel transfer encounters the following error events: - Bus error - Unaligned address - Unaligned transfer width - Reserved configuration 0x0: Channel n has no error status 0x1: Channel n has error status."]
        #[must_use]
        #[inline(always)]
        pub const fn sts(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "The error status, one bit per channel. The error status is set when a channel transfer encounters the following error events: - Bus error - Unaligned address - Unaligned transfer width - Reserved configuration 0x0: Channel n has no error status 0x1: Channel n has error status."]
        #[inline(always)]
        pub const fn set_sts(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Interrsts {
        #[inline(always)]
        fn default() -> Interrsts {
            Interrsts(0)
        }
    }
    impl core::fmt::Debug for Interrsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Interrsts")
                .field("sts[0]", &self.sts(0usize))
                .field("sts[1]", &self.sts(1usize))
                .field("sts[2]", &self.sts(2usize))
                .field("sts[3]", &self.sts(3usize))
                .field("sts[4]", &self.sts(4usize))
                .field("sts[5]", &self.sts(5usize))
                .field("sts[6]", &self.sts(6usize))
                .field("sts[7]", &self.sts(7usize))
                .field("sts[8]", &self.sts(8usize))
                .field("sts[9]", &self.sts(9usize))
                .field("sts[10]", &self.sts(10usize))
                .field("sts[11]", &self.sts(11usize))
                .field("sts[12]", &self.sts(12usize))
                .field("sts[13]", &self.sts(13usize))
                .field("sts[14]", &self.sts(14usize))
                .field("sts[15]", &self.sts(15usize))
                .field("sts[16]", &self.sts(16usize))
                .field("sts[17]", &self.sts(17usize))
                .field("sts[18]", &self.sts(18usize))
                .field("sts[19]", &self.sts(19usize))
                .field("sts[20]", &self.sts(20usize))
                .field("sts[21]", &self.sts(21usize))
                .field("sts[22]", &self.sts(22usize))
                .field("sts[23]", &self.sts(23usize))
                .field("sts[24]", &self.sts(24usize))
                .field("sts[25]", &self.sts(25usize))
                .field("sts[26]", &self.sts(26usize))
                .field("sts[27]", &self.sts(27usize))
                .field("sts[28]", &self.sts(28usize))
                .field("sts[29]", &self.sts(29usize))
                .field("sts[30]", &self.sts(30usize))
                .field("sts[31]", &self.sts(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Interrsts {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Interrsts {{ sts[0]: {=bool:?}, sts[1]: {=bool:?}, sts[2]: {=bool:?}, sts[3]: {=bool:?}, sts[4]: {=bool:?}, sts[5]: {=bool:?}, sts[6]: {=bool:?}, sts[7]: {=bool:?}, sts[8]: {=bool:?}, sts[9]: {=bool:?}, sts[10]: {=bool:?}, sts[11]: {=bool:?}, sts[12]: {=bool:?}, sts[13]: {=bool:?}, sts[14]: {=bool:?}, sts[15]: {=bool:?}, sts[16]: {=bool:?}, sts[17]: {=bool:?}, sts[18]: {=bool:?}, sts[19]: {=bool:?}, sts[20]: {=bool:?}, sts[21]: {=bool:?}, sts[22]: {=bool:?}, sts[23]: {=bool:?}, sts[24]: {=bool:?}, sts[25]: {=bool:?}, sts[26]: {=bool:?}, sts[27]: {=bool:?}, sts[28]: {=bool:?}, sts[29]: {=bool:?}, sts[30]: {=bool:?}, sts[31]: {=bool:?} }}" , self . sts (0usize) , self . sts (1usize) , self . sts (2usize) , self . sts (3usize) , self . sts (4usize) , self . sts (5usize) , self . sts (6usize) , self . sts (7usize) , self . sts (8usize) , self . sts (9usize) , self . sts (10usize) , self . sts (11usize) , self . sts (12usize) , self . sts (13usize) , self . sts (14usize) , self . sts (15usize) , self . sts (16usize) , self . sts (17usize) , self . sts (18usize) , self . sts (19usize) , self . sts (20usize) , self . sts (21usize) , self . sts (22usize) , self . sts (23usize) , self . sts (24usize) , self . sts (25usize) , self . sts (26usize) , self . sts (27usize) , self . sts (28usize) , self . sts (29usize) , self . sts (30usize) , self . sts (31usize))
        }
    }
    #[doc = "Harlf Complete Interrupt Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Inthalfsts(pub u32);
    impl Inthalfsts {
        #[doc = "half transfer done irq status."]
        #[must_use]
        #[inline(always)]
        pub const fn sts(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "half transfer done irq status."]
        #[inline(always)]
        pub const fn set_sts(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Inthalfsts {
        #[inline(always)]
        fn default() -> Inthalfsts {
            Inthalfsts(0)
        }
    }
    impl core::fmt::Debug for Inthalfsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Inthalfsts")
                .field("sts[0]", &self.sts(0usize))
                .field("sts[1]", &self.sts(1usize))
                .field("sts[2]", &self.sts(2usize))
                .field("sts[3]", &self.sts(3usize))
                .field("sts[4]", &self.sts(4usize))
                .field("sts[5]", &self.sts(5usize))
                .field("sts[6]", &self.sts(6usize))
                .field("sts[7]", &self.sts(7usize))
                .field("sts[8]", &self.sts(8usize))
                .field("sts[9]", &self.sts(9usize))
                .field("sts[10]", &self.sts(10usize))
                .field("sts[11]", &self.sts(11usize))
                .field("sts[12]", &self.sts(12usize))
                .field("sts[13]", &self.sts(13usize))
                .field("sts[14]", &self.sts(14usize))
                .field("sts[15]", &self.sts(15usize))
                .field("sts[16]", &self.sts(16usize))
                .field("sts[17]", &self.sts(17usize))
                .field("sts[18]", &self.sts(18usize))
                .field("sts[19]", &self.sts(19usize))
                .field("sts[20]", &self.sts(20usize))
                .field("sts[21]", &self.sts(21usize))
                .field("sts[22]", &self.sts(22usize))
                .field("sts[23]", &self.sts(23usize))
                .field("sts[24]", &self.sts(24usize))
                .field("sts[25]", &self.sts(25usize))
                .field("sts[26]", &self.sts(26usize))
                .field("sts[27]", &self.sts(27usize))
                .field("sts[28]", &self.sts(28usize))
                .field("sts[29]", &self.sts(29usize))
                .field("sts[30]", &self.sts(30usize))
                .field("sts[31]", &self.sts(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Inthalfsts {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Inthalfsts {{ sts[0]: {=bool:?}, sts[1]: {=bool:?}, sts[2]: {=bool:?}, sts[3]: {=bool:?}, sts[4]: {=bool:?}, sts[5]: {=bool:?}, sts[6]: {=bool:?}, sts[7]: {=bool:?}, sts[8]: {=bool:?}, sts[9]: {=bool:?}, sts[10]: {=bool:?}, sts[11]: {=bool:?}, sts[12]: {=bool:?}, sts[13]: {=bool:?}, sts[14]: {=bool:?}, sts[15]: {=bool:?}, sts[16]: {=bool:?}, sts[17]: {=bool:?}, sts[18]: {=bool:?}, sts[19]: {=bool:?}, sts[20]: {=bool:?}, sts[21]: {=bool:?}, sts[22]: {=bool:?}, sts[23]: {=bool:?}, sts[24]: {=bool:?}, sts[25]: {=bool:?}, sts[26]: {=bool:?}, sts[27]: {=bool:?}, sts[28]: {=bool:?}, sts[29]: {=bool:?}, sts[30]: {=bool:?}, sts[31]: {=bool:?} }}" , self . sts (0usize) , self . sts (1usize) , self . sts (2usize) , self . sts (3usize) , self . sts (4usize) , self . sts (5usize) , self . sts (6usize) , self . sts (7usize) , self . sts (8usize) , self . sts (9usize) , self . sts (10usize) , self . sts (11usize) , self . sts (12usize) , self . sts (13usize) , self . sts (14usize) , self . sts (15usize) , self . sts (16usize) , self . sts (17usize) , self . sts (18usize) , self . sts (19usize) , self . sts (20usize) , self . sts (21usize) , self . sts (22usize) , self . sts (23usize) , self . sts (24usize) , self . sts (25usize) , self . sts (26usize) , self . sts (27usize) , self . sts (28usize) , self . sts (29usize) , self . sts (30usize) , self . sts (31usize))
        }
    }
    #[doc = "Trans Complete Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Inttcsts(pub u32);
    impl Inttcsts {
        #[doc = "The terminal count status, one bit per channel. The terminal count status is set when a channel transfer finishes without the abort or error event. 0x0: Channel n has no terminal count status 0x1: Channel n has terminal count status."]
        #[must_use]
        #[inline(always)]
        pub const fn sts(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "The terminal count status, one bit per channel. The terminal count status is set when a channel transfer finishes without the abort or error event. 0x0: Channel n has no terminal count status 0x1: Channel n has terminal count status."]
        #[inline(always)]
        pub const fn set_sts(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Inttcsts {
        #[inline(always)]
        fn default() -> Inttcsts {
            Inttcsts(0)
        }
    }
    impl core::fmt::Debug for Inttcsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Inttcsts")
                .field("sts[0]", &self.sts(0usize))
                .field("sts[1]", &self.sts(1usize))
                .field("sts[2]", &self.sts(2usize))
                .field("sts[3]", &self.sts(3usize))
                .field("sts[4]", &self.sts(4usize))
                .field("sts[5]", &self.sts(5usize))
                .field("sts[6]", &self.sts(6usize))
                .field("sts[7]", &self.sts(7usize))
                .field("sts[8]", &self.sts(8usize))
                .field("sts[9]", &self.sts(9usize))
                .field("sts[10]", &self.sts(10usize))
                .field("sts[11]", &self.sts(11usize))
                .field("sts[12]", &self.sts(12usize))
                .field("sts[13]", &self.sts(13usize))
                .field("sts[14]", &self.sts(14usize))
                .field("sts[15]", &self.sts(15usize))
                .field("sts[16]", &self.sts(16usize))
                .field("sts[17]", &self.sts(17usize))
                .field("sts[18]", &self.sts(18usize))
                .field("sts[19]", &self.sts(19usize))
                .field("sts[20]", &self.sts(20usize))
                .field("sts[21]", &self.sts(21usize))
                .field("sts[22]", &self.sts(22usize))
                .field("sts[23]", &self.sts(23usize))
                .field("sts[24]", &self.sts(24usize))
                .field("sts[25]", &self.sts(25usize))
                .field("sts[26]", &self.sts(26usize))
                .field("sts[27]", &self.sts(27usize))
                .field("sts[28]", &self.sts(28usize))
                .field("sts[29]", &self.sts(29usize))
                .field("sts[30]", &self.sts(30usize))
                .field("sts[31]", &self.sts(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Inttcsts {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Inttcsts {{ sts[0]: {=bool:?}, sts[1]: {=bool:?}, sts[2]: {=bool:?}, sts[3]: {=bool:?}, sts[4]: {=bool:?}, sts[5]: {=bool:?}, sts[6]: {=bool:?}, sts[7]: {=bool:?}, sts[8]: {=bool:?}, sts[9]: {=bool:?}, sts[10]: {=bool:?}, sts[11]: {=bool:?}, sts[12]: {=bool:?}, sts[13]: {=bool:?}, sts[14]: {=bool:?}, sts[15]: {=bool:?}, sts[16]: {=bool:?}, sts[17]: {=bool:?}, sts[18]: {=bool:?}, sts[19]: {=bool:?}, sts[20]: {=bool:?}, sts[21]: {=bool:?}, sts[22]: {=bool:?}, sts[23]: {=bool:?}, sts[24]: {=bool:?}, sts[25]: {=bool:?}, sts[26]: {=bool:?}, sts[27]: {=bool:?}, sts[28]: {=bool:?}, sts[29]: {=bool:?}, sts[30]: {=bool:?}, sts[31]: {=bool:?} }}" , self . sts (0usize) , self . sts (1usize) , self . sts (2usize) , self . sts (3usize) , self . sts (4usize) , self . sts (5usize) , self . sts (6usize) , self . sts (7usize) , self . sts (8usize) , self . sts (9usize) , self . sts (10usize) , self . sts (11usize) , self . sts (12usize) , self . sts (13usize) , self . sts (14usize) , self . sts (15usize) , self . sts (16usize) , self . sts (17usize) , self . sts (18usize) , self . sts (19usize) , self . sts (20usize) , self . sts (21usize) , self . sts (22usize) , self . sts (23usize) , self . sts (24usize) , self . sts (25usize) , self . sts (26usize) , self . sts (27usize) , self . sts (28usize) , self . sts (29usize) , self . sts (30usize) , self . sts (31usize))
        }
    }
    #[doc = "Channel &index0 Linked List Pointer Low Part Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Llpointer(pub u32);
    impl Llpointer {
        #[doc = "Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
        #[must_use]
        #[inline(always)]
        pub const fn llpointerl(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
        #[inline(always)]
        pub const fn set_llpointerl(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
        }
    }
    impl Default for Llpointer {
        #[inline(always)]
        fn default() -> Llpointer {
            Llpointer(0)
        }
    }
    impl core::fmt::Debug for Llpointer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Llpointer")
                .field("llpointerl", &self.llpointerl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Llpointer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Llpointer {{ llpointerl: {=u32:?} }}", self.llpointerl())
        }
    }
    #[doc = "swap table register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwapTable(pub u32);
    impl SwapTable {
        #[doc = "\"4bit for one byte, define how to swap, default is no swap. For HDMA, support up to 4-byte, the register default value is 0x3210\"."]
        #[must_use]
        #[inline(always)]
        pub const fn table(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "\"4bit for one byte, define how to swap, default is no swap. For HDMA, support up to 4-byte, the register default value is 0x3210\"."]
        #[inline(always)]
        pub const fn set_table(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SwapTable {
        #[inline(always)]
        fn default() -> SwapTable {
            SwapTable(0)
        }
    }
    impl core::fmt::Debug for SwapTable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwapTable")
                .field("table", &self.table())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwapTable {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SwapTable {{ table: {=u32:?} }}", self.table())
        }
    }
    #[doc = "Channel &index0Transfer Size Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TranSize(pub u32);
    impl TranSize {
        #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
        #[must_use]
        #[inline(always)]
        pub const fn transize(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
        #[inline(always)]
        pub const fn set_transize(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for TranSize {
        #[inline(always)]
        fn default() -> TranSize {
            TranSize(0)
        }
    }
    impl core::fmt::Debug for TranSize {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TranSize")
                .field("transize", &self.transize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TranSize {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TranSize {{ transize: {=u32:?} }}", self.transize())
        }
    }
}
pub mod vals {
    #[doc = "Source address control."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum AddrCtrl {
        #[doc = "Increment address."]
        INCREMENT = 0x0,
        #[doc = "Decrement address."]
        DECREMENT = 0x01,
        #[doc = "Fixed address."]
        FIXED = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl AddrCtrl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> AddrCtrl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for AddrCtrl {
        #[inline(always)]
        fn from(val: u8) -> AddrCtrl {
            AddrCtrl::from_bits(val)
        }
    }
    impl From<AddrCtrl> for u8 {
        #[inline(always)]
        fn from(val: AddrCtrl) -> u8 {
            AddrCtrl::to_bits(val)
        }
    }
    #[doc = "Source DMA handshake mode."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mode {
        #[doc = "Normal mode."]
        NORMAL = 0x0,
        #[doc = "Handshake mode."]
        HANDSHAKE = 0x01,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mode {
        #[inline(always)]
        fn from(val: u8) -> Mode {
            Mode::from_bits(val)
        }
    }
    impl From<Mode> for u8 {
        #[inline(always)]
        fn from(val: Mode) -> u8 {
            Mode::to_bits(val)
        }
    }
    #[doc = "Source transfer width."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Width {
        #[doc = "Byte transfer. 8 bits."]
        BYTE = 0x0,
        #[doc = "Half-word transfer. 16 bits."]
        HALF_WORD = 0x01,
        #[doc = "Word transfer. 32 bits."]
        WORD = 0x02,
        #[doc = "Double word transfer. 64 bits."]
        DOUBLE_WORD = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Width {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Width {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Width {
        #[inline(always)]
        fn from(val: u8) -> Width {
            Width::from_bits(val)
        }
    }
    impl From<Width> for u8 {
        #[inline(always)]
        fn from(val: Width) -> u8 {
            Width::to_bits(val)
        }
    }
}
