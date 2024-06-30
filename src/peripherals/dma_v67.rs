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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Channel n Transfer Size Register."]
    #[inline(always)]
    pub const fn tran_size(self) -> crate::common::Reg<regs::TranSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Channel n Source Address Low Part Register."]
    #[inline(always)]
    pub const fn src_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Channel n Source Address High Part Register."]
    #[inline(always)]
    pub const fn src_addr_h(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Channel n Destination Address Low Part Register."]
    #[inline(always)]
    pub const fn dst_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Channel n Destination Address High Part Register."]
    #[inline(always)]
    pub const fn dst_addr_h(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Channel n Linked List Pointer Low Part Register."]
    #[inline(always)]
    pub const fn llpointer(self) -> crate::common::Reg<regs::Llpointer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Channel n Linked List Pointer High Part Register."]
    #[inline(always)]
    pub const fn llpointer_h(self) -> crate::common::Reg<regs::LlpointerH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "DMAC Control Register."]
    #[inline(always)]
    pub const fn dmactrl(self) -> crate::common::Reg<regs::Dmactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Channel Abort Register."]
    #[inline(always)]
    pub const fn ch_abort(self) -> crate::common::Reg<regs::ChAbort, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn int_status(self) -> crate::common::Reg<regs::IntStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Channel Enable Register."]
    #[inline(always)]
    pub const fn ch_en(self) -> crate::common::Reg<regs::ChEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn chctrl(self, n: usize) -> Chctrl {
        assert!(n < 8usize);
        unsafe { Chctrl::from_ptr(self.ptr.add(0x40usize + n * 32usize) as _) }
    }
}
pub mod regs {
    #[doc = "Channel Abort Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChAbort(pub u32);
    impl ChAbort {
        #[doc = "Write 1 to bit n to abort channel n. The bits should only be set when the corresponding channels are enabled. Otherwise, the writes will be ignored for channels that are not enabled. (N: Number of channels)."]
        #[inline(always)]
        pub const fn chabort(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Write 1 to bit n to abort channel n. The bits should only be set when the corresponding channels are enabled. Otherwise, the writes will be ignored for channels that are not enabled. (N: Number of channels)."]
        #[inline(always)]
        pub fn set_chabort(&mut self, n: usize, val: bool) {
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
    #[doc = "Channel Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChEn(pub u32);
    impl ChEn {
        #[doc = "Alias of the Enable field of all ChnCtrl registers."]
        #[inline(always)]
        pub const fn chen(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Alias of the Enable field of all ChnCtrl registers."]
        #[inline(always)]
        pub fn set_chen(&mut self, n: usize, val: bool) {
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
    #[doc = "Channel n Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Channel enable bit 0x0: Disable 0x1: Enable."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Channel enable bit 0x0: Disable 0x1: Enable."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt."]
        #[inline(always)]
        pub const fn inttcmask(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt."]
        #[inline(always)]
        pub fn set_inttcmask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt."]
        #[inline(always)]
        pub const fn interrmask(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt."]
        #[inline(always)]
        pub fn set_interrmask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt."]
        #[inline(always)]
        pub const fn intabtmask(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt."]
        #[inline(always)]
        pub fn set_intabtmask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
        #[inline(always)]
        pub const fn dstreqsel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
        #[inline(always)]
        pub fn set_dstreqsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
        #[inline(always)]
        pub const fn srcreqsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
        #[inline(always)]
        pub fn set_srcreqsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception."]
        #[inline(always)]
        pub const fn dstaddrctrl(&self) -> super::vals::AddrCtrl {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::AddrCtrl::from_bits(val as u8)
        }
        #[doc = "Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception."]
        #[inline(always)]
        pub fn set_dstaddrctrl(&mut self, val: super::vals::AddrCtrl) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception."]
        #[inline(always)]
        pub const fn srcaddrctrl(&self) -> super::vals::AddrCtrl {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::AddrCtrl::from_bits(val as u8)
        }
        #[doc = "Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception."]
        #[inline(always)]
        pub fn set_srcaddrctrl(&mut self, val: super::vals::AddrCtrl) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode."]
        #[inline(always)]
        pub const fn dstmode(&self) -> super::vals::Mode {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode."]
        #[inline(always)]
        pub fn set_dstmode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode."]
        #[inline(always)]
        pub const fn srcmode(&self) -> super::vals::Mode {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode."]
        #[inline(always)]
        pub fn set_srcmode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2."]
        #[inline(always)]
        pub const fn dstwidth(&self) -> super::vals::Width {
            let val = (self.0 >> 18usize) & 0x07;
            super::vals::Width::from_bits(val as u8)
        }
        #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2."]
        #[inline(always)]
        pub fn set_dstwidth(&mut self, val: super::vals::Width) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
        }
        #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2."]
        #[inline(always)]
        pub const fn srcwidth(&self) -> super::vals::Width {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Width::from_bits(val as u8)
        }
        #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6-x7: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0x3, for HDMA, the maximum allowed value is 0x2."]
        #[inline(always)]
        pub fn set_srcwidth(&mut self, val: super::vals::Width) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb-0xf: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0xa; for HDMA, the maximum allowed value is 0x7."]
        #[inline(always)]
        pub const fn srcburstsize(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb-0xf: Reserved, setting this field with a reserved value triggers the error exception for XDMA, the maximum allowed value is 0xa; for HDMA, the maximum allowed value is 0x7."]
        #[inline(always)]
        pub fn set_srcburstsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Channel priority level 0x0: Lower priority 0x1: Higher priority."]
        #[inline(always)]
        pub const fn priority(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Channel priority level 0x0: Lower priority 0x1: Higher priority."]
        #[inline(always)]
        pub fn set_priority(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1."]
        #[inline(always)]
        pub const fn dstbusinfidx(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1."]
        #[inline(always)]
        pub fn set_dstbusinfidx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface."]
        #[inline(always)]
        pub const fn srcbusinfidx(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface."]
        #[inline(always)]
        pub fn set_srcbusinfidx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "DMAC Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacfg(pub u32);
    impl Dmacfg {
        #[doc = "Channel number 0x1: 1 channel 0x2: 2 channels ... 0x8: 8 channels Others: Invalid."]
        #[inline(always)]
        pub const fn channelnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Channel number 0x1: 1 channel 0x2: 2 channels ... 0x8: 8 channels Others: Invalid."]
        #[inline(always)]
        pub fn set_channelnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "FIFO depth 0x4: 4 entries 0x8: 8 entries 0x10: 16 entries 0x20: 32 entries Others: Invalid."]
        #[inline(always)]
        pub const fn fifodepth(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x3f;
            val as u8
        }
        #[doc = "FIFO depth 0x4: 4 entries 0x8: 8 entries 0x10: 16 entries 0x20: 32 entries Others: Invalid."]
        #[inline(always)]
        pub fn set_fifodepth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
        }
        #[doc = "Request/acknowledge pair number 0x0: 0 pair 0x1: 1 pair 0x2: 2 pairs ... 0x10: 16 pairs."]
        #[inline(always)]
        pub const fn reqnum(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "Request/acknowledge pair number 0x0: 0 pair 0x1: 1 pair 0x2: 2 pairs ... 0x10: 16 pairs."]
        #[inline(always)]
        pub fn set_reqnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "AXI bus interface number 0x0: 1 AXI bus 0x1: 2 AXI busses."]
        #[inline(always)]
        pub const fn busnum(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "AXI bus interface number 0x0: 1 AXI bus 0x1: 2 AXI busses."]
        #[inline(always)]
        pub fn set_busnum(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "DMA core number 0x0: 1 core 0x1: 2 cores."]
        #[inline(always)]
        pub const fn corenum(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DMA core number 0x0: 1 core 0x1: 2 cores."]
        #[inline(always)]
        pub fn set_corenum(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "AXI bus address width 0x18: 24 bits 0x19: 25 bits ... 0x40: 64 bits Others: Invalid."]
        #[inline(always)]
        pub const fn addrwidth(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[doc = "AXI bus address width 0x18: 24 bits 0x19: 25 bits ... 0x40: 64 bits Others: Invalid."]
        #[inline(always)]
        pub fn set_addrwidth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
        #[doc = "AXI bus data width 0x0: 32 bits 0x1: 64 bits 0x2: 128 bits 0x3: 256 bits."]
        #[inline(always)]
        pub const fn datawidth(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "AXI bus data width 0x0: 32 bits 0x1: 64 bits 0x2: 128 bits 0x3: 256 bits."]
        #[inline(always)]
        pub fn set_datawidth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "DMA request synchronization. The DMA request synchronization should be configured to avoid signal integrity problems when the request signal is not clocked by the system bus clock, which the DMA control logic operates in. If the request synchronization is not configured, the request signal is sampled directly without synchronization. 0x0: Request synchronization is not configured 0x1: Request synchronization is configured."]
        #[inline(always)]
        pub const fn reqsync(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DMA request synchronization. The DMA request synchronization should be configured to avoid signal integrity problems when the request signal is not clocked by the system bus clock, which the DMA control logic operates in. If the request synchronization is not configured, the request signal is sampled directly without synchronization. 0x0: Request synchronization is not configured 0x1: Request synchronization is configured."]
        #[inline(always)]
        pub fn set_reqsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Chain transfer 0x0: Chain transfer is not configured 0x1: Chain transfer is configured."]
        #[inline(always)]
        pub const fn chainxfr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Chain transfer 0x0: Chain transfer is not configured 0x1: Chain transfer is configured."]
        #[inline(always)]
        pub fn set_chainxfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Dmacfg {
        #[inline(always)]
        fn default() -> Dmacfg {
            Dmacfg(0)
        }
    }
    #[doc = "DMAC Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmactrl(pub u32);
    impl Dmactrl {
        #[doc = "Software reset control. Write 1 to this bit to reset the DMA core and disable all channels. Note: The software reset may cause the in-completion of AXI transaction."]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset control. Write 1 to this bit to reset the DMA core and disable all channels. Note: The software reset may cause the in-completion of AXI transaction."]
        #[inline(always)]
        pub fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Dmactrl {
        #[inline(always)]
        fn default() -> Dmactrl {
            Dmactrl(0)
        }
    }
    #[doc = "Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntStatus(pub u32);
    impl IntStatus {
        #[doc = "The error status, one bit per channel. The error status is set when a channel transfer encounters the following error events: - Bus error - Unaligned address - Unaligned transfer width - Reserved configuration 0x0: Channel n has no error status 0x1: Channel n has error status."]
        #[inline(always)]
        pub const fn error(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "The error status, one bit per channel. The error status is set when a channel transfer encounters the following error events: - Bus error - Unaligned address - Unaligned transfer width - Reserved configuration 0x0: Channel n has no error status 0x1: Channel n has error status."]
        #[inline(always)]
        pub fn set_error(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "The abort status of channel, one bit per channel. The abort status is set when a channel transfer is aborted. 0x0: Channel n has no abort status 0x1: Channel n has abort status."]
        #[inline(always)]
        pub const fn abort(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "The abort status of channel, one bit per channel. The abort status is set when a channel transfer is aborted. 0x0: Channel n has no abort status 0x1: Channel n has abort status."]
        #[inline(always)]
        pub fn set_abort(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "The terminal count status, one bit per channel. The terminal count status is set when a channel transfer finishes without the abort or error event. 0x0: Channel n has no terminal count status 0x1: Channel n has terminal count status."]
        #[inline(always)]
        pub const fn tc(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "The terminal count status, one bit per channel. The terminal count status is set when a channel transfer finishes without the abort or error event. 0x0: Channel n has no terminal count status 0x1: Channel n has terminal count status."]
        #[inline(always)]
        pub fn set_tc(&mut self, n: usize, val: bool) {
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
    #[doc = "Channel n Linked List Pointer Low Part Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Llpointer(pub u32);
    impl Llpointer {
        #[doc = "Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0."]
        #[inline(always)]
        pub const fn lldbusinfidx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0."]
        #[inline(always)]
        pub fn set_lldbusinfidx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
        #[inline(always)]
        pub const fn llpointerl(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
        #[inline(always)]
        pub fn set_llpointerl(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
        }
    }
    impl Default for Llpointer {
        #[inline(always)]
        fn default() -> Llpointer {
            Llpointer(0)
        }
    }
    #[doc = "Channel n Linked List Pointer High Part Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LlpointerH(pub u32);
    impl LlpointerH {
        #[doc = "High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
        #[inline(always)]
        pub const fn llpointerh(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
        #[inline(always)]
        pub fn set_llpointerh(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LlpointerH {
        #[inline(always)]
        fn default() -> LlpointerH {
            LlpointerH(0)
        }
    }
    #[doc = "Channel n Transfer Size Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TranSize(pub u32);
    impl TranSize {
        #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
        #[inline(always)]
        pub const fn transize(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
        #[inline(always)]
        pub fn set_transize(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TranSize {
        #[inline(always)]
        fn default() -> TranSize {
            TranSize(0)
        }
    }
}
pub mod vals {
    #[doc = "Source address control."]
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
