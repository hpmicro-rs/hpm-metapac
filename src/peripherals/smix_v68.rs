#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh {
    ptr: *mut u8,
}
unsafe impl Send for DmaCh {}
unsafe impl Sync for DmaCh {}
impl DmaCh {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Channel N Control Register."]
    #[inline(always)]
    pub const fn ctl(self) -> crate::common::Reg<regs::Ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Channel N Source Total Beats Register."]
    #[inline(always)]
    pub const fn burst_count(self) -> crate::common::Reg<regs::BurstCount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Channel N Source Register."]
    #[inline(always)]
    pub const fn src_addr(self) -> crate::common::Reg<regs::SrcAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Channel N Destination Register."]
    #[inline(always)]
    pub const fn dst_addr(self) -> crate::common::Reg<regs::DstAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Channel N Linked List Pointer Register."]
    #[inline(always)]
    pub const fn llp(self) -> crate::common::Reg<regs::Llp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DstCh {
    ptr: *mut u8,
}
unsafe impl Send for DstCh {}
unsafe impl Sync for DstCh {}
impl DstCh {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SMIX Dstination N Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::DstChCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SMIX Dstination N Gain Register."]
    #[inline(always)]
    pub const fn gain(self) -> crate::common::Reg<regs::DstChGain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SMIX Dstination N Max Index Register."]
    #[inline(always)]
    pub const fn bufsize(self) -> crate::common::Reg<regs::Bufsize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "SMIX Dstination N Fade-In Configuration Register."]
    #[inline(always)]
    pub const fn fadein(self) -> crate::common::Reg<regs::DstChFadein, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SMIX Dstination N Fade-Out Configuration Register."]
    #[inline(always)]
    pub const fn fadeout(self) -> crate::common::Reg<regs::DstChFadeout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SMIX Dstination N Status Register."]
    #[inline(always)]
    pub const fn st(self) -> crate::common::Reg<regs::DstChSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SMIX Dstination N Data Out Register."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::DstChData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "SMIX Dstination N Source Enable Register."]
    #[inline(always)]
    pub const fn source_en(self) -> crate::common::Reg<regs::SourceEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SMIX Dstination N Source Activation Register."]
    #[inline(always)]
    pub const fn source_act(self) -> crate::common::Reg<regs::SourceAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "SMIX Dstination N Source De-Activation Register."]
    #[inline(always)]
    pub const fn source_deact(self) -> crate::common::Reg<regs::SourceDeact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "SMIX Dstination N Source Fade-in Control Register."]
    #[inline(always)]
    pub const fn source_fadein_ctrl(
        self,
    ) -> crate::common::Reg<regs::SourceFadeinCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "SMIX Dstination N Source Deactivation Status Register."]
    #[inline(always)]
    pub const fn deact_st(self) -> crate::common::Reg<regs::DeactSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "SMIX Dstination N Source Manual Fade-out Control Register."]
    #[inline(always)]
    pub const fn source_mfadeout_ctrl(
        self,
    ) -> crate::common::Reg<regs::SourceMfadeoutCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
}
#[doc = "SMIX."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smix {
    ptr: *mut u8,
}
unsafe impl Send for Smix {}
unsafe impl Sync for Smix {}
impl Smix {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMAC_ID Register."]
    #[inline(always)]
    pub const fn dmac_id(self) -> crate::common::Reg<regs::DmacId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Transfer Complete Status."]
    #[inline(always)]
    pub const fn dmac_tc_st(self) -> crate::common::Reg<regs::DmacTcSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Transfer Abort Status."]
    #[inline(always)]
    pub const fn dmac_abrt_st(self) -> crate::common::Reg<regs::DmacAbrtSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Transfer Error Status."]
    #[inline(always)]
    pub const fn dmac_err_st(self) -> crate::common::Reg<regs::DmacErrSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn dmac_ctrl(self) -> crate::common::Reg<regs::DmacCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Abort Command Register."]
    #[inline(always)]
    pub const fn dmac_abrt_cmd(self) -> crate::common::Reg<regs::DmacAbrtCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Channel Enable Register."]
    #[inline(always)]
    pub const fn dmac_chen(self) -> crate::common::Reg<regs::DmacChen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn dma_ch(self, n: usize) -> DmaCh {
        assert!(n < 26usize);
        unsafe { DmaCh::from_ptr(self.ptr.add(0x40usize + n * 32usize) as _) }
    }
    #[doc = "SMIX Cal Saturation Status Register."]
    #[inline(always)]
    pub const fn calsat_st(self) -> crate::common::Reg<regs::CalsatSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "SMIX Fade-Out Done Status Register."]
    #[inline(always)]
    pub const fn fdot_done_st(self) -> crate::common::Reg<regs::FdotDoneSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0804usize) as _) }
    }
    #[doc = "SMIX Data Status Register."]
    #[inline(always)]
    pub const fn data_st(self) -> crate::common::Reg<regs::DataSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0808usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn dst_ch(self, n: usize) -> DstCh {
        assert!(n < 2usize);
        unsafe { DstCh::from_ptr(self.ptr.add(0x0840usize + n * 64usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn source_ch(self, n: usize) -> SourceCh {
        assert!(n < 14usize);
        unsafe { SourceCh::from_ptr(self.ptr.add(0x0900usize + n * 32usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SourceCh {
    ptr: *mut u8,
}
unsafe impl Send for SourceCh {}
unsafe impl Sync for SourceCh {}
impl SourceCh {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SMIX Source N Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::SourceChCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SMIX Source N Gain Register."]
    #[inline(always)]
    pub const fn gain(self) -> crate::common::Reg<regs::SourceChGain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SMIX Source N Fade-in Control Register."]
    #[inline(always)]
    pub const fn fadein(self) -> crate::common::Reg<regs::SourceChFadein, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "SMIX Source N Fade-out Control Register."]
    #[inline(always)]
    pub const fn fadeout(self) -> crate::common::Reg<regs::SourceChFadeout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SMIX Source N Buffer Size Register."]
    #[inline(always)]
    pub const fn buf_size(self) -> crate::common::Reg<regs::BufSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SMIX Source N Status Register."]
    #[inline(always)]
    pub const fn st(self) -> crate::common::Reg<regs::SourceChSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SMIX Source N Data Input Register."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::SourceChData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "SMIX Source N Buffer Size Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BufSize(pub u32);
    impl BufSize {
        #[doc = "unit as 16-bits per sample. Zero means no length limit. = Act Len-1. The actual length is the up_rate*(input_data_length-4). If the filter processing is down-sampling, the value of up_rate above is 1. If the filter processing is up-sampling, the value of up_rate above is the up-sampling rate."]
        #[inline(always)]
        pub const fn maxidx(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "unit as 16-bits per sample. Zero means no length limit. = Act Len-1. The actual length is the up_rate*(input_data_length-4). If the filter processing is down-sampling, the value of up_rate above is 1. If the filter processing is up-sampling, the value of up_rate above is the up-sampling rate."]
        #[inline(always)]
        pub fn set_maxidx(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BufSize {
        #[inline(always)]
        fn default() -> BufSize {
            BufSize(0)
        }
    }
    #[doc = "SMIX Dstination N Max Index Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bufsize(pub u32);
    impl Bufsize {
        #[doc = "The total length of the dst stream -1. If zero, means there is no end of the stream."]
        #[inline(always)]
        pub const fn max_idx(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The total length of the dst stream -1. If zero, means there is no end of the stream."]
        #[inline(always)]
        pub fn set_max_idx(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bufsize {
        #[inline(always)]
        fn default() -> Bufsize {
            Bufsize(0)
        }
    }
    #[doc = "Channel N Source Total Beats Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BurstCount(pub u32);
    impl BurstCount {
        #[doc = "the total number of source beats."]
        #[inline(always)]
        pub const fn num(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the total number of source beats."]
        #[inline(always)]
        pub fn set_num(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BurstCount {
        #[inline(always)]
        fn default() -> BurstCount {
            BurstCount(0)
        }
    }
    #[doc = "SMIX Cal Saturation Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CalsatSt(pub u32);
    impl CalsatSt {
        #[doc = "SRC CAL_SAT_ERR. W1C."]
        #[inline(always)]
        pub const fn src(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "SRC CAL_SAT_ERR. W1C."]
        #[inline(always)]
        pub fn set_src(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "DST CAL_SAT_ERR. W1C."]
        #[inline(always)]
        pub const fn dst(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "DST CAL_SAT_ERR. W1C."]
        #[inline(always)]
        pub fn set_dst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for CalsatSt {
        #[inline(always)]
        fn default() -> CalsatSt {
            CalsatSt(0)
        }
    }
    #[doc = "Channel N Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctl(pub u32);
    impl Ctl {
        #[doc = "channel enable bit."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "channel enable bit."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TC interrupt enable."]
        #[inline(always)]
        pub const fn tc_int_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TC interrupt enable."]
        #[inline(always)]
        pub fn set_tc_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Err interrupt enable."]
        #[inline(always)]
        pub const fn err_int_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Err interrupt enable."]
        #[inline(always)]
        pub fn set_err_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Abort interrupt enable."]
        #[inline(always)]
        pub const fn abrt_int_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Abort interrupt enable."]
        #[inline(always)]
        pub fn set_abrt_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers an error exception."]
        #[inline(always)]
        pub const fn dstaddrctrl(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers an error exception."]
        #[inline(always)]
        pub fn set_dstaddrctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers an error exception."]
        #[inline(always)]
        pub const fn srcaddrctrl(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers an error exception."]
        #[inline(always)]
        pub fn set_srcaddrctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "DMA Destination handshake mode 0x0: Normal mode 0x1: Handshake mode."]
        #[inline(always)]
        pub const fn dstmode(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Destination handshake mode 0x0: Normal mode 0x1: Handshake mode."]
        #[inline(always)]
        pub fn set_dstmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "DMA Source handshake mode 0x0: Normal mode 0x1: Handshake mode."]
        #[inline(always)]
        pub const fn srcmode(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Source handshake mode 0x0: Normal mode 0x1: Handshake mode."]
        #[inline(always)]
        pub fn set_srcmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Destination Transfer Beat Size: 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer."]
        #[inline(always)]
        pub const fn dstwidth(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "Destination Transfer Beat Size: 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer."]
        #[inline(always)]
        pub fn set_dstwidth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "Source Transfer Beat Size: 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer."]
        #[inline(always)]
        pub const fn srcwidth(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Source Transfer Beat Size: 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer."]
        #[inline(always)]
        pub fn set_srcwidth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "0x0: 1 beat per transfer 0x1: 2 beats per transfer 0x2: 4 beats per transfer 0x3: 8 beats per transfer 0x4: 16 beats per transfer 0x5: 32 beats per transfer 0x6: 64 beats per transfer 0x7: 128 beats per transfer."]
        #[inline(always)]
        pub const fn srcburstsize(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x0f;
            val as u8
        }
        #[doc = "0x0: 1 beat per transfer 0x1: 2 beats per transfer 0x2: 4 beats per transfer 0x3: 8 beats per transfer 0x4: 16 beats per transfer 0x5: 32 beats per transfer 0x6: 64 beats per transfer 0x7: 128 beats per transfer."]
        #[inline(always)]
        pub fn set_srcburstsize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 15usize)) | (((val as u32) & 0x0f) << 15usize);
        }
        #[doc = "0x0: Lower priority 0x1: Higher priority."]
        #[inline(always)]
        pub const fn priority(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "0x0: Lower priority 0x1: Higher priority."]
        #[inline(always)]
        pub fn set_priority(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
        #[inline(always)]
        pub const fn dstreqsel(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x1f;
            val as u8
        }
        #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
        #[inline(always)]
        pub fn set_dstreqsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
        }
        #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
        #[inline(always)]
        pub const fn srcreqsel(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
        #[inline(always)]
        pub fn set_srcreqsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
    }
    impl Default for Ctl {
        #[inline(always)]
        fn default() -> Ctl {
            Ctl(0)
        }
    }
    #[doc = "SMIX Data Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DataSt(pub u32);
    impl DataSt {
        #[doc = "SRC data needed."]
        #[inline(always)]
        pub const fn src_dn(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "SRC data needed."]
        #[inline(always)]
        pub fn set_src_dn(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "DST data underflow."]
        #[inline(always)]
        pub const fn dst_undl(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "DST data underflow."]
        #[inline(always)]
        pub fn set_dst_undl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "DST data available."]
        #[inline(always)]
        pub const fn dst_da(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "DST data available."]
        #[inline(always)]
        pub fn set_dst_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for DataSt {
        #[inline(always)]
        fn default() -> DataSt {
            DataSt(0)
        }
    }
    #[doc = "SMIX Dstination N Source Deactivation Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DeactSt(pub u32);
    impl DeactSt {
        #[doc = "Asserted when in de-active mode."]
        #[inline(always)]
        pub const fn src_deact_st(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Asserted when in de-active mode."]
        #[inline(always)]
        pub fn set_src_deact_st(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Asserted when in de-active mode."]
        #[inline(always)]
        pub const fn dst_deact(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted when in de-active mode."]
        #[inline(always)]
        pub fn set_dst_deact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DeactSt {
        #[inline(always)]
        fn default() -> DeactSt {
            DeactSt(0)
        }
    }
    #[doc = "Abort Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacAbrtCmd(pub u32);
    impl DmacAbrtCmd {
        #[doc = "Write 1 to force the corresponding channel into abort status."]
        #[inline(always)]
        pub const fn ch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "Write 1 to force the corresponding channel into abort status."]
        #[inline(always)]
        pub fn set_ch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for DmacAbrtCmd {
        #[inline(always)]
        fn default() -> DmacAbrtCmd {
            DmacAbrtCmd(0)
        }
    }
    #[doc = "Transfer Abort Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacAbrtSt(pub u32);
    impl DmacAbrtSt {
        #[doc = "The abort status is set when a channel transfer is aborted."]
        #[inline(always)]
        pub const fn ch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "The abort status is set when a channel transfer is aborted."]
        #[inline(always)]
        pub fn set_ch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for DmacAbrtSt {
        #[inline(always)]
        fn default() -> DmacAbrtSt {
            DmacAbrtSt(0)
        }
    }
    #[doc = "Channel Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacChen(pub u32);
    impl DmacChen {
        #[doc = "Write 1 to enable the corresponding channel."]
        #[inline(always)]
        pub const fn ch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "Write 1 to enable the corresponding channel."]
        #[inline(always)]
        pub fn set_ch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for DmacChen {
        #[inline(always)]
        fn default() -> DmacChen {
            DmacChen(0)
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacCtrl(pub u32);
    impl DmacCtrl {
        #[doc = "Software Reset."]
        #[inline(always)]
        pub const fn srst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software Reset."]
        #[inline(always)]
        pub fn set_srst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for DmacCtrl {
        #[inline(always)]
        fn default() -> DmacCtrl {
            DmacCtrl(0)
        }
    }
    #[doc = "Transfer Error Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacErrSt(pub u32);
    impl DmacErrSt {
        #[doc = "The error status is set when a channel transfer encounters the following error events: . Bus error . Unaligned address . Unaligned transfer width . Reserved configuration."]
        #[inline(always)]
        pub const fn ch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "The error status is set when a channel transfer encounters the following error events: . Bus error . Unaligned address . Unaligned transfer width . Reserved configuration."]
        #[inline(always)]
        pub fn set_ch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for DmacErrSt {
        #[inline(always)]
        fn default() -> DmacErrSt {
            DmacErrSt(0)
        }
    }
    #[doc = "DMAC_ID Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacId(pub u32);
    impl DmacId {
        #[doc = "Revision."]
        #[inline(always)]
        pub const fn rev(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "Revision."]
        #[inline(always)]
        pub fn set_rev(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
    }
    impl Default for DmacId {
        #[inline(always)]
        fn default() -> DmacId {
            DmacId(0)
        }
    }
    #[doc = "Transfer Complete Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmacTcSt(pub u32);
    impl DmacTcSt {
        #[doc = "The terminal count status is set when a channel transfer finishes without abort or error events."]
        #[inline(always)]
        pub const fn ch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "The terminal count status is set when a channel transfer finishes without abort or error events."]
        #[inline(always)]
        pub fn set_ch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for DmacTcSt {
        #[inline(always)]
        fn default() -> DmacTcSt {
            DmacTcSt(0)
        }
    }
    #[doc = "Channel N Destination Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DstAddr(pub u32);
    impl DstAddr {
        #[doc = "destination address."]
        #[inline(always)]
        pub const fn ptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "destination address."]
        #[inline(always)]
        pub fn set_ptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DstAddr {
        #[inline(always)]
        fn default() -> DstAddr {
            DstAddr(0)
        }
    }
    #[doc = "SMIX Dstination N Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DstChCtrl(pub u32);
    impl DstChCtrl {
        #[doc = "mixer function enable."]
        #[inline(always)]
        pub const fn mixer_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "mixer function enable."]
        #[inline(always)]
        pub fn set_mixer_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Soft reset."]
        #[inline(always)]
        pub const fn softrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Soft reset."]
        #[inline(always)]
        pub fn set_softrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Dst enabled. When disabled, clear the FIFO pointers."]
        #[inline(always)]
        pub const fn dst_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Dst enabled. When disabled, clear the FIFO pointers."]
        #[inline(always)]
        pub fn set_dst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FadeIn_Ctrl for destionation. Auto clear."]
        #[inline(always)]
        pub const fn dstfadin_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FadeIn_Ctrl for destionation. Auto clear."]
        #[inline(always)]
        pub fn set_dstfadin_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Automatically FadeOut_Ctrl for destionation. Only effective after DST_AFADEOUT is assigned a non-zero value."]
        #[inline(always)]
        pub const fn dstfadout_aen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Automatically FadeOut_Ctrl for destionation. Only effective after DST_AFADEOUT is assigned a non-zero value."]
        #[inline(always)]
        pub fn set_dstfadout_aen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Manual FadeOut_Ctrl for destionation. Auto clear."]
        #[inline(always)]
        pub const fn dstfadout_men(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Manual FadeOut_Ctrl for destionation. Auto clear."]
        #[inline(always)]
        pub fn set_dstfadout_men(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "activate the destination channel."]
        #[inline(always)]
        pub const fn dst_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "activate the destination channel."]
        #[inline(always)]
        pub fn set_dst_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "de-activate the destination channel."]
        #[inline(always)]
        pub const fn dst_deact(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "de-activate the destination channel."]
        #[inline(always)]
        pub fn set_dst_deact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fade-Out interrupt enable."]
        #[inline(always)]
        pub const fn fadeout_done_ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fade-Out interrupt enable."]
        #[inline(always)]
        pub fn set_fadeout_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "AutoDeactAfterFadeOut_En: Asserted to enter de-activated mode after fade-out done."]
        #[inline(always)]
        pub const fn adeactfadeout_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "AutoDeactAfterFadeOut_En: Asserted to enter de-activated mode after fade-out done."]
        #[inline(always)]
        pub fn set_adeactfadeout_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data Available IntEn."]
        #[inline(always)]
        pub const fn da_int_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data Available IntEn."]
        #[inline(always)]
        pub fn set_da_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Cal Saturation IntEn."]
        #[inline(always)]
        pub const fn calsat_int_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Cal Saturation IntEn."]
        #[inline(always)]
        pub fn set_calsat_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "FIFO threshold for DMA or Int. >= will generate req. Must be greater or equal than 8. The read burst of DMA should make the fillings in the buffer be greater than 4."]
        #[inline(always)]
        pub const fn thrsh(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0xff;
            val as u8
        }
        #[doc = "FIFO threshold for DMA or Int. >= will generate req. Must be greater or equal than 8. The read burst of DMA should make the fillings in the buffer be greater than 4."]
        #[inline(always)]
        pub fn set_thrsh(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
        }
        #[doc = "Data Underflow Error IntEn."]
        #[inline(always)]
        pub const fn data_unfl_ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Data Underflow Error IntEn."]
        #[inline(always)]
        pub fn set_data_unfl_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for DstChCtrl {
        #[inline(always)]
        fn default() -> DstChCtrl {
            DstChCtrl(0)
        }
    }
    #[doc = "SMIX Dstination N Data Out Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DstChData(pub u32);
    impl DstChData {
        #[doc = "Output data buffer."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Output data buffer."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DstChData {
        #[inline(always)]
        fn default() -> DstChData {
            DstChData(0)
        }
    }
    #[doc = "SMIX Dstination N Fade-In Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DstChFadein(pub u32);
    impl DstChFadein {
        #[doc = "Fade-in delta for linear fading in from 0 to 1 (about at most 20s for 48kHz sampled sound) (Using only top 14 bits for mul)."]
        #[inline(always)]
        pub const fn delta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Fade-in delta for linear fading in from 0 to 1 (about at most 20s for 48kHz sampled sound) (Using only top 14 bits for mul)."]
        #[inline(always)]
        pub fn set_delta(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for DstChFadein {
        #[inline(always)]
        fn default() -> DstChFadein {
            DstChFadein(0)
        }
    }
    #[doc = "SMIX Dstination N Fade-Out Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DstChFadeout(pub u32);
    impl DstChFadeout {
        #[doc = "Fade out in 2^DELTA samples. Now DELTA can be at most 14。."]
        #[inline(always)]
        pub const fn delta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Fade out in 2^DELTA samples. Now DELTA can be at most 14。."]
        #[inline(always)]
        pub fn set_delta(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for DstChFadeout {
        #[inline(always)]
        fn default() -> DstChFadeout {
            DstChFadeout(0)
        }
    }
    #[doc = "SMIX Dstination N Gain Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DstChGain(pub u32);
    impl DstChGain {
        #[doc = "Unsigned Int, with 12 fractional bits. . The top 3 bits are for shift. Same as SHFT_CTR\\[2:0\\]."]
        #[inline(always)]
        pub const fn val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Unsigned Int, with 12 fractional bits. . The top 3 bits are for shift. Same as SHFT_CTR\\[2:0\\]."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for DstChGain {
        #[inline(always)]
        fn default() -> DstChGain {
            DstChGain(0)
        }
    }
    #[doc = "SMIX Dstination N Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DstChSt(pub u32);
    impl DstChSt {
        #[doc = "The modes are: Mode 0: Disabled: after reset. Program the registers, and DSTn_CTRL \\[DST_EN\\]
to enter Mode 1. Mode 1: Enabled and not-activated. wait for DSTn_CTRL \\[DSTFADIN_EN\\]
or DSTn_CTRL \\[DST_ACT\\], jump to Mode 3 or Mode 4 based on whether Fade-in enabled. Mode 3: Enabled and activated and fade-in in progress: Can not be fade out. Will send data to DMA. Jump to Mode 4 after fadin op done. Mode 4: Enabled and activated and done fade-in, no fade-out yet: Can be fade out. Will send data to DMA. Mode 5: Enabled and activated and fade-out in progress: After faded out OP. Will send data to DMA. Will transfer to mode 6 or mode 7 depending on the DSTn_CTRL \\[ADeactFadeOut_En\\]
cfg Mode 6: Enabled and activated and faded-out: faded out is done. Will send data to DMA. Will transfer to mode 7 if manual deactivated. Mode 7: Enabled and De-activated: If configured to enter this mode, after auto or manuallly fade-out, or after manual de-activated. Won't send data to DMA. Won't gen data avail signals. Intf register can be programmed. Will change to Mode 3 or Mode 4 after manual ACT or Fade-in CMD. Will transfer to Mode 0 if DSTn_CTRL \\[DST_EN\\]
is assigned 0. To support a new stream or, to continue the old stream after a pause."]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "The modes are: Mode 0: Disabled: after reset. Program the registers, and DSTn_CTRL \\[DST_EN\\]
to enter Mode 1. Mode 1: Enabled and not-activated. wait for DSTn_CTRL \\[DSTFADIN_EN\\]
or DSTn_CTRL \\[DST_ACT\\], jump to Mode 3 or Mode 4 based on whether Fade-in enabled. Mode 3: Enabled and activated and fade-in in progress: Can not be fade out. Will send data to DMA. Jump to Mode 4 after fadin op done. Mode 4: Enabled and activated and done fade-in, no fade-out yet: Can be fade out. Will send data to DMA. Mode 5: Enabled and activated and fade-out in progress: After faded out OP. Will send data to DMA. Will transfer to mode 6 or mode 7 depending on the DSTn_CTRL \\[ADeactFadeOut_En\\]
cfg Mode 6: Enabled and activated and faded-out: faded out is done. Will send data to DMA. Will transfer to mode 7 if manual deactivated. Mode 7: Enabled and De-activated: If configured to enter this mode, after auto or manuallly fade-out, or after manual de-activated. Won't send data to DMA. Won't gen data avail signals. Intf register can be programmed. Will change to Mode 3 or Mode 4 after manual ACT or Fade-in CMD. Will transfer to Mode 0 if DSTn_CTRL \\[DST_EN\\]
is assigned 0. To support a new stream or, to continue the old stream after a pause."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Data Available."]
        #[inline(always)]
        pub const fn da(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data Available."]
        #[inline(always)]
        pub fn set_da(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Saturate Error Found. W1C."]
        #[inline(always)]
        pub const fn calsat(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Saturate Error Found. W1C."]
        #[inline(always)]
        pub fn set_calsat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Fade-Out Done. W1C."]
        #[inline(always)]
        pub const fn fdout_done(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Fade-Out Done. W1C."]
        #[inline(always)]
        pub fn set_fdout_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "destination channel output FIFO fillings."]
        #[inline(always)]
        pub const fn fifo_fillings(&self) -> u16 {
            let val = (self.0 >> 6usize) & 0x01ff;
            val as u16
        }
        #[doc = "destination channel output FIFO fillings."]
        #[inline(always)]
        pub fn set_fifo_fillings(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 6usize)) | (((val as u32) & 0x01ff) << 6usize);
        }
    }
    impl Default for DstChSt {
        #[inline(always)]
        fn default() -> DstChSt {
            DstChSt(0)
        }
    }
    #[doc = "SMIX Fade-Out Done Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FdotDoneSt(pub u32);
    impl FdotDoneSt {
        #[doc = "SRC fadeout done. W1C."]
        #[inline(always)]
        pub const fn src(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "SRC fadeout done. W1C."]
        #[inline(always)]
        pub fn set_src(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "DST fadeout done. W1C."]
        #[inline(always)]
        pub const fn dst(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "DST fadeout done. W1C."]
        #[inline(always)]
        pub fn set_dst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for FdotDoneSt {
        #[inline(always)]
        fn default() -> FdotDoneSt {
            FdotDoneSt(0)
        }
    }
    #[doc = "Channel N Linked List Pointer Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Llp(pub u32);
    impl Llp {
        #[doc = "the address pointer for the linked list descriptor."]
        #[inline(always)]
        pub const fn ptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the address pointer for the linked list descriptor."]
        #[inline(always)]
        pub fn set_ptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Llp {
        #[inline(always)]
        fn default() -> Llp {
            Llp(0)
        }
    }
    #[doc = "SMIX Dstination N Source Activation Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SourceAct(pub u32);
    impl SourceAct {
        #[doc = "Manually Activate the channel."]
        #[inline(always)]
        pub const fn val(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Manually Activate the channel."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SourceAct {
        #[inline(always)]
        fn default() -> SourceAct {
            SourceAct(0)
        }
    }
    #[doc = "SMIX Source N Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SourceChCtrl(pub u32);
    impl SourceChCtrl {
        #[doc = "0: no rate conversion 1: up-conversion x2 2: up-conversion x3 3: up-conversion x4 4: up-conversion x6 5: up-conversion x8 6: up-conversion x12 7: down-conversion /2."]
        #[inline(always)]
        pub const fn rateconv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "0: no rate conversion 1: up-conversion x2 2: up-conversion x3 3: up-conversion x4 4: up-conversion x6 5: up-conversion x8 6: up-conversion x12 7: down-conversion /2."]
        #[inline(always)]
        pub fn set_rateconv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Fade-Out interrupt enable."]
        #[inline(always)]
        pub const fn fadeout_done_ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Fade-Out interrupt enable."]
        #[inline(always)]
        pub fn set_fadeout_done_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Asserted to enter de-activated mode after fade-out done."]
        #[inline(always)]
        pub const fn autodeactafterfadeout_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enter de-activated mode after fade-out done."]
        #[inline(always)]
        pub fn set_autodeactafterfadeout_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Shift operation after FIR 0: no shift (when no upsampling or up-sampling-by-2 or up-sampling-by-3) 1: left-shift-by-1 (when up-sampling-by-4 or up-sampling-by-6) 2: left-shift-by-1 (when up-sampling-by-8 or up-sampling-by-12) 7: /2 (when rate /2) Other n: shift-left-by-n, but not suggested to be used."]
        #[inline(always)]
        pub const fn shft_ctrl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Shift operation after FIR 0: no shift (when no upsampling or up-sampling-by-2 or up-sampling-by-3) 1: left-shift-by-1 (when up-sampling-by-4 or up-sampling-by-6) 2: left-shift-by-1 (when up-sampling-by-8 or up-sampling-by-12) 7: /2 (when rate /2) Other n: shift-left-by-n, but not suggested to be used."]
        #[inline(always)]
        pub fn set_shft_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Data Needed IntEn."]
        #[inline(always)]
        pub const fn dn_int_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Data Needed IntEn."]
        #[inline(always)]
        pub fn set_dn_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Cal Saturation IntEn."]
        #[inline(always)]
        pub const fn calsat_int_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Cal Saturation IntEn."]
        #[inline(always)]
        pub fn set_calsat_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "FIFO threshold for DMA or Int. <= will generate req. Must be greater or equal than 8. This threshold is also used to trgger the internal FIR operation. To avoid the reading and writing to the same address in the memory block, the threshold should greater than 4."]
        #[inline(always)]
        pub const fn thrsh(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0xff;
            val as u8
        }
        #[doc = "FIFO threshold for DMA or Int. <= will generate req. Must be greater or equal than 8. This threshold is also used to trgger the internal FIR operation. To avoid the reading and writing to the same address in the memory block, the threshold should greater than 4."]
        #[inline(always)]
        pub fn set_thrsh(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 13usize)) | (((val as u32) & 0xff) << 13usize);
        }
        #[doc = "Asserted to reset FIFO pointer. Cleared to exit reset state."]
        #[inline(always)]
        pub const fn fifo_reset(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to reset FIFO pointer. Cleared to exit reset state."]
        #[inline(always)]
        pub fn set_fifo_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for SourceChCtrl {
        #[inline(always)]
        fn default() -> SourceChCtrl {
            SourceChCtrl(0)
        }
    }
    #[doc = "SMIX Source N Data Input Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SourceChData(pub u32);
    impl SourceChData {
        #[doc = "Data input register."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data input register."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SourceChData {
        #[inline(always)]
        fn default() -> SourceChData {
            SourceChData(0)
        }
    }
    #[doc = "SMIX Source N Fade-in Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SourceChFadein(pub u32);
    impl SourceChFadein {
        #[doc = "Fade -in confg."]
        #[inline(always)]
        pub const fn delta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Fade -in confg."]
        #[inline(always)]
        pub fn set_delta(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for SourceChFadein {
        #[inline(always)]
        fn default() -> SourceChFadein {
            SourceChFadein(0)
        }
    }
    #[doc = "SMIX Source N Fade-out Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SourceChFadeout(pub u32);
    impl SourceChFadeout {
        #[doc = "Fade out in 2^DELTA samples. Now DELTA can be at most 14。."]
        #[inline(always)]
        pub const fn delta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Fade out in 2^DELTA samples. Now DELTA can be at most 14。."]
        #[inline(always)]
        pub fn set_delta(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for SourceChFadeout {
        #[inline(always)]
        fn default() -> SourceChFadeout {
            SourceChFadeout(0)
        }
    }
    #[doc = "SMIX Source N Gain Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SourceChGain(pub u32);
    impl SourceChGain {
        #[doc = "Unsigned Int, with 12 fractional bits. The top 3 bits are for shift. Same as SHFT_CTR\\[2:0\\]."]
        #[inline(always)]
        pub const fn val(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Unsigned Int, with 12 fractional bits. The top 3 bits are for shift. Same as SHFT_CTR\\[2:0\\]."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for SourceChGain {
        #[inline(always)]
        fn default() -> SourceChGain {
            SourceChGain(0)
        }
    }
    #[doc = "SMIX Source N Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SourceChSt(pub u32);
    impl SourceChSt {
        #[doc = "The modes are: Mode 0: Disabled: after reset. Program the registers, and DSTx_SRC_EN\\[n\\]
to enter Mode 1. Mode 1: Enabled but not activated: After Enabled. Data needed signal can send out, can receive DMA data. Will enter Mode 2 after manual ACT or Fade-in CMD Mode 2: Enabled and activated and buffer feed-in in progress: Can not be fade out. Will consume data from DMA. If not enter due to Fade-in CMD, will enter Mode 4, else enter Mode 3. This mode is used to make the channel in MIX only after initial data are ready, thus will not stall mix operation due to the lackness of data of this channel omly. Mode 3: Enabled and activated and fade-in in progress: Can not be fade out. Will consume data from DMA. Mode 4: Enabled and activated and done fade-in, no fade-out yet: Can be fade out. Will consume data from DMA. Mode 5: Enabled and activated and fade-out in progress: After faded out done. Will consume data from DMA. Will transfer to mode 6 or mode 7 depending on the SRCn_CTRL\\[AutoDeactAfterFadeOut_En\\]
cfg Mode 6: Enabled and activated and faded-out: faded out is done. Will consume data from DMA. Will transfer to mode 7 if manual deactivated. Mode 7: Enabled and De-activated: If configured to enter this mode, after auto or manuallly fade-out, or after manual de-activated. Won't consume data from DMA. Won't gen data needed signals. Intf register can be programmed. Will change to Mode 2 after manual ACT or Fade-in CMD. Will transfer to Mode 0 if DSTx_SRC_EN\\[n\\]
is assigned 0. To support a new stream or, to continue the old stream after a pause."]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "The modes are: Mode 0: Disabled: after reset. Program the registers, and DSTx_SRC_EN\\[n\\]
to enter Mode 1. Mode 1: Enabled but not activated: After Enabled. Data needed signal can send out, can receive DMA data. Will enter Mode 2 after manual ACT or Fade-in CMD Mode 2: Enabled and activated and buffer feed-in in progress: Can not be fade out. Will consume data from DMA. If not enter due to Fade-in CMD, will enter Mode 4, else enter Mode 3. This mode is used to make the channel in MIX only after initial data are ready, thus will not stall mix operation due to the lackness of data of this channel omly. Mode 3: Enabled and activated and fade-in in progress: Can not be fade out. Will consume data from DMA. Mode 4: Enabled and activated and done fade-in, no fade-out yet: Can be fade out. Will consume data from DMA. Mode 5: Enabled and activated and fade-out in progress: After faded out done. Will consume data from DMA. Will transfer to mode 6 or mode 7 depending on the SRCn_CTRL\\[AutoDeactAfterFadeOut_En\\]
cfg Mode 6: Enabled and activated and faded-out: faded out is done. Will consume data from DMA. Will transfer to mode 7 if manual deactivated. Mode 7: Enabled and De-activated: If configured to enter this mode, after auto or manuallly fade-out, or after manual de-activated. Won't consume data from DMA. Won't gen data needed signals. Intf register can be programmed. Will change to Mode 2 after manual ACT or Fade-in CMD. Will transfer to Mode 0 if DSTx_SRC_EN\\[n\\]
is assigned 0. To support a new stream or, to continue the old stream after a pause."]
        #[inline(always)]
        pub fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "the poly phase counter."]
        #[inline(always)]
        pub const fn firphase(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[doc = "the poly phase counter."]
        #[inline(always)]
        pub fn set_firphase(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[doc = "Data needed flag."]
        #[inline(always)]
        pub const fn dn(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Data needed flag."]
        #[inline(always)]
        pub fn set_dn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Calculation saturation status. W1C."]
        #[inline(always)]
        pub const fn calsat(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Calculation saturation status. W1C."]
        #[inline(always)]
        pub fn set_calsat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fade-Out Done. W1C."]
        #[inline(always)]
        pub const fn fdout_done(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Fade-Out Done. W1C."]
        #[inline(always)]
        pub fn set_fdout_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "The fillings of input FIFO."]
        #[inline(always)]
        pub const fn fifo_fillings(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x01ff;
            val as u16
        }
        #[doc = "The fillings of input FIFO."]
        #[inline(always)]
        pub fn set_fifo_fillings(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 10usize)) | (((val as u32) & 0x01ff) << 10usize);
        }
    }
    impl Default for SourceChSt {
        #[inline(always)]
        fn default() -> SourceChSt {
            SourceChSt(0)
        }
    }
    #[doc = "SMIX Dstination N Source De-Activation Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SourceDeact(pub u32);
    impl SourceDeact {
        #[doc = "Manually DeActivate the channel."]
        #[inline(always)]
        pub const fn val(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Manually DeActivate the channel."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SourceDeact {
        #[inline(always)]
        fn default() -> SourceDeact {
            SourceDeact(0)
        }
    }
    #[doc = "SMIX Dstination N Source Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SourceEn(pub u32);
    impl SourceEn {
        #[doc = "After enabled, Data needed req will be asserted. DMA can feed in data. The channel will join in the sum operation of mixer operation."]
        #[inline(always)]
        pub const fn val(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "After enabled, Data needed req will be asserted. DMA can feed in data. The channel will join in the sum operation of mixer operation."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SourceEn {
        #[inline(always)]
        fn default() -> SourceEn {
            SourceEn(0)
        }
    }
    #[doc = "SMIX Dstination N Source Fade-in Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SourceFadeinCtrl(pub u32);
    impl SourceFadeinCtrl {
        #[doc = "Asserted to start fade-in operation. When the amplification factors are stable, auto clear."]
        #[inline(always)]
        pub const fn aop(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Asserted to start fade-in operation. When the amplification factors are stable, auto clear."]
        #[inline(always)]
        pub fn set_aop(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SourceFadeinCtrl {
        #[inline(always)]
        fn default() -> SourceFadeinCtrl {
            SourceFadeinCtrl(0)
        }
    }
    #[doc = "SMIX Dstination N Source Manual Fade-out Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SourceMfadeoutCtrl(pub u32);
    impl SourceMfadeoutCtrl {
        #[doc = "Asserted to start fade-out operation. When the amplification factors are stable, auto clear."]
        #[inline(always)]
        pub const fn op(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Asserted to start fade-out operation. When the amplification factors are stable, auto clear."]
        #[inline(always)]
        pub fn set_op(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SourceMfadeoutCtrl {
        #[inline(always)]
        fn default() -> SourceMfadeoutCtrl {
            SourceMfadeoutCtrl(0)
        }
    }
    #[doc = "Channel N Source Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrcAddr(pub u32);
    impl SrcAddr {
        #[doc = "source address."]
        #[inline(always)]
        pub const fn ptr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "source address."]
        #[inline(always)]
        pub fn set_ptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SrcAddr {
        #[inline(always)]
        fn default() -> SrcAddr {
            SrcAddr(0)
        }
    }
}
