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
    #[doc = "Channel n Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Channel n Transfer Size Register."]
    #[inline(always)]
    pub const fn tran_size(self) -> crate::common::Reg<regs::TranSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel n Source Address Low Part Register."]
    #[inline(always)]
    pub const fn src_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel n Source Address High Part Register."]
    #[inline(always)]
    pub const fn src_addr_h(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel n Destination Address Low Part Register."]
    #[inline(always)]
    pub const fn dst_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Channel n Destination Address High Part Register."]
    #[inline(always)]
    pub const fn dst_addr_h(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Channel n Linked List Pointer Low Part Register."]
    #[inline(always)]
    pub const fn llpointer(self) -> crate::common::Reg<regs::Llpointer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Channel n Linked List Pointer High Part Register."]
    #[inline(always)]
    pub const fn llpointer_h(self) -> crate::common::Reg<regs::LlpointerH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
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
    #[doc = "DMAC Configuration Register."]
    #[inline(always)]
    pub const fn dmacfg(self) -> crate::common::Reg<regs::Dmacfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DMAC Control Register."]
    #[inline(always)]
    pub const fn dmactrl(self) -> crate::common::Reg<regs::Dmactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Channel Abort Register."]
    #[inline(always)]
    pub const fn ch_abort(self) -> crate::common::Reg<regs::ChAbort, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn int_status(self) -> crate::common::Reg<regs::IntStatus, crate::common::RW> {
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
        assert!(n < 8usize);
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
    #[doc = "Channel n Control Register."]
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
        #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
        #[must_use]
        #[inline(always)]
        pub const fn dstreqsel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
        #[inline(always)]
        pub const fn set_dstreqsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
        #[must_use]
        #[inline(always)]
        pub const fn srcreqsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
        #[inline(always)]
        pub const fn set_srcreqsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
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
        #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Mode {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode."]
        #[inline(always)]
        pub const fn set_dstmode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode."]
        #[must_use]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Mode {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode."]
        #[inline(always)]
        pub const fn set_srcmode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2."]
        #[must_use]
        #[inline(always)]
        pub const fn dstwidth(&self) -> super::vals::Width {
            let val = (self.0 >> 18usize) & 0x07;
            super::vals::Width::from_bits(val as u8)
        }
        #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2."]
        #[inline(always)]
        pub const fn set_dstwidth(&mut self, val: super::vals::Width) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
        }
        #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2."]
        #[must_use]
        #[inline(always)]
        pub const fn srcwidth(&self) -> super::vals::Width {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Width::from_bits(val as u8)
        }
        #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2."]
        #[inline(always)]
        pub const fn set_srcwidth(&mut self, val: super::vals::Width) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb-0xf: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0xa; for HDMA, the maximum allowed value is 0x7."]
        #[must_use]
        #[inline(always)]
        pub const fn srcburstsize(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb-0xf: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0xa; for HDMA, the maximum allowed value is 0x7."]
        #[inline(always)]
        pub const fn set_srcburstsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
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
        #[doc = "Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1."]
        #[must_use]
        #[inline(always)]
        pub const fn dstbusinfidx(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1."]
        #[inline(always)]
        pub const fn set_dstbusinfidx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface."]
        #[must_use]
        #[inline(always)]
        pub const fn srcbusinfidx(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface."]
        #[inline(always)]
        pub const fn set_srcbusinfidx(&mut self, val: bool) {
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
                .field("dstreqsel", &self.dstreqsel())
                .field("srcreqsel", &self.srcreqsel())
                .field("dstaddrctrl", &self.dstaddrctrl())
                .field("srcaddrctrl", &self.srcaddrctrl())
                .field("dstmode", &self.dstmode())
                .field("srcmode", &self.srcmode())
                .field("dstwidth", &self.dstwidth())
                .field("srcwidth", &self.srcwidth())
                .field("srcburstsize", &self.srcburstsize())
                .field("priority", &self.priority())
                .field("dstbusinfidx", &self.dstbusinfidx())
                .field("srcbusinfidx", &self.srcbusinfidx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl {{ enable: {=bool:?}, inttcmask: {=bool:?}, interrmask: {=bool:?}, intabtmask: {=bool:?}, dstreqsel: {=u8:?}, srcreqsel: {=u8:?}, dstaddrctrl: {:?}, srcaddrctrl: {:?}, dstmode: {:?}, srcmode: {:?}, dstwidth: {:?}, srcwidth: {:?}, srcburstsize: {=u8:?}, priority: {=bool:?}, dstbusinfidx: {=bool:?}, srcbusinfidx: {=bool:?} }}" , self . enable () , self . inttcmask () , self . interrmask () , self . intabtmask () , self . dstreqsel () , self . srcreqsel () , self . dstaddrctrl () , self . srcaddrctrl () , self . dstmode () , self . srcmode () , self . dstwidth () , self . srcwidth () , self . srcburstsize () , self . priority () , self . dstbusinfidx () , self . srcbusinfidx ())
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
    #[doc = "Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStatus(pub u32);
    impl IntStatus {
        #[doc = "The error status, one bit per channel. The error status is set when a channel transfer encounters the following error events: - Bus error - Unaligned address - Unaligned transfer width - Reserved configuration 0x0: Channel n has no error status 0x1: Channel n has error status."]
        #[must_use]
        #[inline(always)]
        pub const fn error(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "The error status, one bit per channel. The error status is set when a channel transfer encounters the following error events: - Bus error - Unaligned address - Unaligned transfer width - Reserved configuration 0x0: Channel n has no error status 0x1: Channel n has error status."]
        #[inline(always)]
        pub const fn set_error(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "The abort status of channel, one bit per channel. The abort status is set when a channel transfer is aborted. 0x0: Channel n has no abort status 0x1: Channel n has abort status."]
        #[must_use]
        #[inline(always)]
        pub const fn abort(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "The abort status of channel, one bit per channel. The abort status is set when a channel transfer is aborted. 0x0: Channel n has no abort status 0x1: Channel n has abort status."]
        #[inline(always)]
        pub const fn set_abort(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "The terminal count status, one bit per channel. The terminal count status is set when a channel transfer finishes without the abort or error event. 0x0: Channel n has no terminal count status 0x1: Channel n has terminal count status."]
        #[must_use]
        #[inline(always)]
        pub const fn tc(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "The terminal count status, one bit per channel. The terminal count status is set when a channel transfer finishes without the abort or error event. 0x0: Channel n has no terminal count status 0x1: Channel n has terminal count status."]
        #[inline(always)]
        pub const fn set_tc(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for IntStatus {
        #[inline(always)]
        fn default() -> IntStatus {
            IntStatus(0)
        }
    }
    impl core::fmt::Debug for IntStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntStatus")
                .field("error[0]", &self.error(0usize))
                .field("error[1]", &self.error(1usize))
                .field("error[2]", &self.error(2usize))
                .field("error[3]", &self.error(3usize))
                .field("error[4]", &self.error(4usize))
                .field("error[5]", &self.error(5usize))
                .field("error[6]", &self.error(6usize))
                .field("error[7]", &self.error(7usize))
                .field("abort[0]", &self.abort(0usize))
                .field("abort[1]", &self.abort(1usize))
                .field("abort[2]", &self.abort(2usize))
                .field("abort[3]", &self.abort(3usize))
                .field("abort[4]", &self.abort(4usize))
                .field("abort[5]", &self.abort(5usize))
                .field("abort[6]", &self.abort(6usize))
                .field("abort[7]", &self.abort(7usize))
                .field("tc[0]", &self.tc(0usize))
                .field("tc[1]", &self.tc(1usize))
                .field("tc[2]", &self.tc(2usize))
                .field("tc[3]", &self.tc(3usize))
                .field("tc[4]", &self.tc(4usize))
                .field("tc[5]", &self.tc(5usize))
                .field("tc[6]", &self.tc(6usize))
                .field("tc[7]", &self.tc(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntStatus {{ error[0]: {=bool:?}, error[1]: {=bool:?}, error[2]: {=bool:?}, error[3]: {=bool:?}, error[4]: {=bool:?}, error[5]: {=bool:?}, error[6]: {=bool:?}, error[7]: {=bool:?}, abort[0]: {=bool:?}, abort[1]: {=bool:?}, abort[2]: {=bool:?}, abort[3]: {=bool:?}, abort[4]: {=bool:?}, abort[5]: {=bool:?}, abort[6]: {=bool:?}, abort[7]: {=bool:?}, tc[0]: {=bool:?}, tc[1]: {=bool:?}, tc[2]: {=bool:?}, tc[3]: {=bool:?}, tc[4]: {=bool:?}, tc[5]: {=bool:?}, tc[6]: {=bool:?}, tc[7]: {=bool:?} }}" , self . error (0usize) , self . error (1usize) , self . error (2usize) , self . error (3usize) , self . error (4usize) , self . error (5usize) , self . error (6usize) , self . error (7usize) , self . abort (0usize) , self . abort (1usize) , self . abort (2usize) , self . abort (3usize) , self . abort (4usize) , self . abort (5usize) , self . abort (6usize) , self . abort (7usize) , self . tc (0usize) , self . tc (1usize) , self . tc (2usize) , self . tc (3usize) , self . tc (4usize) , self . tc (5usize) , self . tc (6usize) , self . tc (7usize))
        }
    }
    #[doc = "Channel n Linked List Pointer Low Part Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Llpointer(pub u32);
    impl Llpointer {
        #[doc = "Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0."]
        #[must_use]
        #[inline(always)]
        pub const fn lldbusinfidx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0."]
        #[inline(always)]
        pub const fn set_lldbusinfidx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
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
                .field("lldbusinfidx", &self.lldbusinfidx())
                .field("llpointerl", &self.llpointerl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Llpointer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Llpointer {{ lldbusinfidx: {=bool:?}, llpointerl: {=u32:?} }}",
                self.lldbusinfidx(),
                self.llpointerl()
            )
        }
    }
    #[doc = "Channel n Linked List Pointer High Part Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LlpointerH(pub u32);
    impl LlpointerH {
        #[doc = "High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
        #[must_use]
        #[inline(always)]
        pub const fn llpointerh(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
        #[inline(always)]
        pub const fn set_llpointerh(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LlpointerH {
        #[inline(always)]
        fn default() -> LlpointerH {
            LlpointerH(0)
        }
    }
    impl core::fmt::Debug for LlpointerH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LlpointerH")
                .field("llpointerh", &self.llpointerh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LlpointerH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LlpointerH {{ llpointerh: {=u32:?} }}",
                self.llpointerh()
            )
        }
    }
    #[doc = "Channel n Transfer Size Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TranSize(pub u32);
    impl TranSize {
        #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
        #[must_use]
        #[inline(always)]
        pub const fn transize(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
        #[inline(always)]
        pub const fn set_transize(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
