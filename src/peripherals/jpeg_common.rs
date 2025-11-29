#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "JPEG."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jpeg {
    ptr: *mut u8,
}
unsafe impl Send for Jpeg {}
unsafe impl Sync for Jpeg {}
impl Jpeg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "In DMA Misc Control Register."]
    #[inline(always)]
    pub const fn in_dma_misc(self) -> crate::common::Reg<regs::InDmaMisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "In DMA Buf Address."]
    #[inline(always)]
    pub const fn in_dmabase(self) -> crate::common::Reg<regs::InDmabase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "In DMA Buf Control 0 Register."]
    #[inline(always)]
    pub const fn in_dma_ctrl0(self) -> crate::common::Reg<regs::InDmaCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "In DMA Buf Control 1 Register."]
    #[inline(always)]
    pub const fn in_dma_ctrl1(self) -> crate::common::Reg<regs::InDmaCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "In DMA Next Command Register."]
    #[inline(always)]
    pub const fn inxt_cmd(self) -> crate::common::Reg<regs::InxtCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Out DMA Misc Control Register."]
    #[inline(always)]
    pub const fn out_dma_misc(self) -> crate::common::Reg<regs::OutDmaMisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Out DMA Buf Address."]
    #[inline(always)]
    pub const fn out_dmabase(self) -> crate::common::Reg<regs::OutDmabase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Out DMA Buf Control 0 Register."]
    #[inline(always)]
    pub const fn out_dma_ctrl0(self) -> crate::common::Reg<regs::OutDmaCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Out DMA Buf Control 1 Register."]
    #[inline(always)]
    pub const fn out_dma_ctrl1(self) -> crate::common::Reg<regs::OutDmaCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Out DMA Next Command Register."]
    #[inline(always)]
    pub const fn onxt_cmd(self) -> crate::common::Reg<regs::OnxtCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Image width register."]
    #[inline(always)]
    pub const fn width(self) -> crate::common::Reg<regs::Width, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Image height register."]
    #[inline(always)]
    pub const fn height(self) -> crate::common::Reg<regs::Height, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Buf Access Addr."]
    #[inline(always)]
    pub const fn buf_addr(self) -> crate::common::Reg<regs::BufAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Buf Access Data."]
    #[inline(always)]
    pub const fn buf_data(self) -> crate::common::Reg<regs::BufData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Out DMA Bytes Counter."]
    #[inline(always)]
    pub const fn out_dmacnt(self) -> crate::common::Reg<regs::OutDmacnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "YUV2RGB coefficients Register 0."]
    #[inline(always)]
    pub const fn csc_coef0(self) -> crate::common::Reg<regs::CscCoef0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "YUV2RGB coefficients Register 1."]
    #[inline(always)]
    pub const fn csc_coef1(self) -> crate::common::Reg<regs::CscCoef1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "YUV2RGB coefficients Register 2."]
    #[inline(always)]
    pub const fn csc_coef2(self) -> crate::common::Reg<regs::CscCoef2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "RGB2YUV coefficients Register 0."]
    #[inline(always)]
    pub const fn rgb2yuv_coef0(self) -> crate::common::Reg<regs::Rgb2yuvCoef0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "RGB2YUV coefficients Register 1."]
    #[inline(always)]
    pub const fn rgb2yuv_coef1(self) -> crate::common::Reg<regs::Rgb2yuvCoef1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "RGB2YUV coefficients Register 2."]
    #[inline(always)]
    pub const fn rgb2yuv_coef2(self) -> crate::common::Reg<regs::Rgb2yuvCoef2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "RGB2YUV coefficients Register 3."]
    #[inline(always)]
    pub const fn rgb2yuv_coef3(self) -> crate::common::Reg<regs::Rgb2yuvCoef3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "RGB2YUV coefficients Register 4."]
    #[inline(always)]
    pub const fn rgb2yuv_coef4(self) -> crate::common::Reg<regs::Rgb2yuvCoef4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Image Control Register 1."]
    #[inline(always)]
    pub const fn img_reg1(self) -> crate::common::Reg<regs::ImgReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Image Control Register 2."]
    #[inline(always)]
    pub const fn img_reg2(self) -> crate::common::Reg<regs::ImgReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Image Control Register 3."]
    #[inline(always)]
    pub const fn img_reg3(self) -> crate::common::Reg<regs::ImgReg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn imgreg(self, n: usize) -> crate::common::Reg<regs::Imgreg, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize + n * 4usize) as _) }
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
    #[doc = "Buf Access Addr."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BufAddr(pub u32);
    impl BufAddr {
        #[doc = "ADDR\\[31:28\\]
denotes the buffer type: 0x2: Qmem 0x3: HuffEnc 0x4: HuffMin 0x5: HuffBase 0x6: HuffSymb ADDR\\[27:0\\]
is the address inside the buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ADDR\\[31:28\\]
denotes the buffer type: 0x2: Qmem 0x3: HuffEnc 0x4: HuffMin 0x5: HuffBase 0x6: HuffSymb ADDR\\[27:0\\]
is the address inside the buffer."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BufAddr {
        #[inline(always)]
        fn default() -> BufAddr {
            BufAddr(0)
        }
    }
    impl core::fmt::Debug for BufAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BufAddr")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BufAddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BufAddr {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Buf Access Data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BufData(pub u32);
    impl BufData {
        #[doc = "The data write-to/read-from buffer. The n-th address read will be actually the data written for n-1 th address, and the actual stored location is n-1 th address."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The data write-to/read-from buffer. The n-th address read will be actually the data written for n-1 th address, and the actual stored location is n-1 th address."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BufData {
        #[inline(always)]
        fn default() -> BufData {
            BufData(0)
        }
    }
    impl core::fmt::Debug for BufData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BufData")
                .field("data", &self.data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BufData {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BufData {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "1b - Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn jpeg_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1b - Enabled."]
        #[inline(always)]
        pub const fn set_jpeg_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "1: decoder, 0:encoder."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "1: decoder, 0:encoder."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Asserted if to start a new encoder/decoder conversion. It will at first stop the inner JPEG module, then reset it, and then re-run it. It is a different mode from DMA phase mode. It cannot be configured in the DMA chain descriptor. It should be configured by the core processor. Auto clear."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted if to start a new encoder/decoder conversion. It will at first stop the inner JPEG module, then reset it, and then re-run it. It is a different mode from DMA phase mode. It cannot be configured in the DMA chain descriptor. It should be configured by the core processor. Auto clear."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Software Reset."]
        #[must_use]
        #[inline(always)]
        pub const fn jpeg_sftrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Software Reset."]
        #[inline(always)]
        pub const fn set_jpeg_sftrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "3'b000: for 420, hy=2, vy=2, hc=1, vc=1 // 6 sub-blocks per MCU 3'b001: for 422h, hy=2, vy=1, hc=1, vc=1 // 4 sub-blocks per MCU 3'b010: for 422v, hy=1, vy=2, hc=1, vc=1 // 4 sub-blocks per MCU 3'b011: for 444, hy=1, vy=1, hc=1, vc=1 // 3 sub-blocks per MCU 3'b100: for 400, hy=2, vy=2, hc=0, vc=0 // 4 sub-blocks per MCU Others: Undefined."]
        #[must_use]
        #[inline(always)]
        pub const fn jdata_format(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "3'b000: for 420, hy=2, vy=2, hc=1, vc=1 // 6 sub-blocks per MCU 3'b001: for 422h, hy=2, vy=1, hc=1, vc=1 // 4 sub-blocks per MCU 3'b010: for 422v, hy=1, vy=2, hc=1, vc=1 // 4 sub-blocks per MCU 3'b011: for 444, hy=1, vy=1, hc=1, vc=1 // 3 sub-blocks per MCU 3'b100: for 400, hy=2, vy=2, hc=0, vc=0 // 4 sub-blocks per MCU Others: Undefined."]
        #[inline(always)]
        pub const fn set_jdata_format(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as R,B 2'b11: YUV422H1P, byte sequence as Y0,U0,Y1,V0."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_opath_sel(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as R,B 2'b11: YUV422H1P, byte sequence as Y0,U0,Y1,V0."]
        #[inline(always)]
        pub const fn set_cfg_opath_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on."]
        #[must_use]
        #[inline(always)]
        pub const fn clkgate(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on."]
        #[inline(always)]
        pub const fn set_clkgate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "asserted to use APB clock, so that the memory contents could be read out through APB interface."]
        #[must_use]
        #[inline(always)]
        pub const fn mem_debug_clk_sel(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to use APB clock, so that the memory contents could be read out through APB interface."]
        #[inline(always)]
        pub const fn set_mem_debug_clk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "The jpg endec restart error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn codec_restart_err_irq_en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "The jpg endec restart error interrupt enable."]
        #[inline(always)]
        pub const fn set_codec_restart_err_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "The jpg endec process done interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn codec_over_irq_en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "The jpg endec process done interrupt enable."]
        #[inline(always)]
        pub const fn set_codec_over_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as B,R 2'b11: YUV422H, byte sequence as Y0,U0,Y1,V0."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_ipath_sel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as B,R 2'b11: YUV422H, byte sequence as Y0,U0,Y1,V0."]
        #[inline(always)]
        pub const fn set_cfg_ipath_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Normally the default CbCr sequence is that Cb macro block coming before Cr macro blk. If Cr macro block is first, set this bit to 1'b1. This bit only impact the color space conversion from/to RGB."]
        #[must_use]
        #[inline(always)]
        pub const fn jd_uvswap(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Normally the default CbCr sequence is that Cb macro block coming before Cr macro blk. If Cr macro block is first, set this bit to 1'b1. This bit only impact the color space conversion from/to RGB."]
        #[inline(always)]
        pub const fn set_jd_uvswap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    impl core::fmt::Debug for Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg")
                .field("jpeg_en", &self.jpeg_en())
                .field("mode", &self.mode())
                .field("start", &self.start())
                .field("jpeg_sftrst", &self.jpeg_sftrst())
                .field("jdata_format", &self.jdata_format())
                .field("cfg_opath_sel", &self.cfg_opath_sel())
                .field("clkgate", &self.clkgate())
                .field("mem_debug_clk_sel", &self.mem_debug_clk_sel())
                .field("codec_restart_err_irq_en", &self.codec_restart_err_irq_en())
                .field("codec_over_irq_en", &self.codec_over_irq_en())
                .field("cfg_ipath_sel", &self.cfg_ipath_sel())
                .field("jd_uvswap", &self.jd_uvswap())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cfg {{ jpeg_en: {=bool:?}, mode: {=bool:?}, start: {=bool:?}, jpeg_sftrst: {=bool:?}, jdata_format: {=u8:?}, cfg_opath_sel: {=u8:?}, clkgate: {=bool:?}, mem_debug_clk_sel: {=bool:?}, codec_restart_err_irq_en: {=bool:?}, codec_over_irq_en: {=bool:?}, cfg_ipath_sel: {=u8:?}, jd_uvswap: {=bool:?} }}" , self . jpeg_en () , self . mode () , self . start () , self . jpeg_sftrst () , self . jdata_format () , self . cfg_opath_sel () , self . clkgate () , self . mem_debug_clk_sel () , self . codec_restart_err_irq_en () , self . codec_over_irq_en () , self . cfg_ipath_sel () , self . jd_uvswap ())
        }
    }
    #[doc = "YUV2RGB coefficients Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CscCoef0(pub u32);
    impl CscCoef0 {
        #[doc = "Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
        #[must_use]
        #[inline(always)]
        pub const fn y_offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
        #[inline(always)]
        pub const fn set_y_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
        #[must_use]
        #[inline(always)]
        pub const fn uv_offset(&self) -> u16 {
            let val = (self.0 >> 9usize) & 0x01ff;
            val as u16
        }
        #[doc = "Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
        #[inline(always)]
        pub const fn set_uv_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 9usize)) | (((val as u32) & 0x01ff) << 9usize);
        }
        #[doc = "Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)."]
        #[must_use]
        #[inline(always)]
        pub const fn c0(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)."]
        #[inline(always)]
        pub const fn set_c0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
        }
        #[doc = "Enable the CSC unit. 0b - The CSC is bypassed 1b - The CSC is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the CSC unit. 0b - The CSC is bypassed 1b - The CSC is enabled."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "This bit changes the behavior when performing U/V converting. 0b - Converting YUV to RGB data 1b - Converting YCbCr to RGB data."]
        #[must_use]
        #[inline(always)]
        pub const fn ycbcr_mode(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "This bit changes the behavior when performing U/V converting. 0b - Converting YUV to RGB data 1b - Converting YCbCr to RGB data."]
        #[inline(always)]
        pub const fn set_ycbcr_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CscCoef0 {
        #[inline(always)]
        fn default() -> CscCoef0 {
            CscCoef0(0)
        }
    }
    impl core::fmt::Debug for CscCoef0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CscCoef0")
                .field("y_offset", &self.y_offset())
                .field("uv_offset", &self.uv_offset())
                .field("c0", &self.c0())
                .field("enable", &self.enable())
                .field("ycbcr_mode", &self.ycbcr_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CscCoef0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CscCoef0 {{ y_offset: {=u16:?}, uv_offset: {=u16:?}, c0: {=u16:?}, enable: {=bool:?}, ycbcr_mode: {=bool:?} }}" , self . y_offset () , self . uv_offset () , self . c0 () , self . enable () , self . ycbcr_mode ())
        }
    }
    #[doc = "YUV2RGB coefficients Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CscCoef1(pub u32);
    impl CscCoef1 {
        #[doc = "Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
        #[must_use]
        #[inline(always)]
        pub const fn c4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
        #[inline(always)]
        pub const fn set_c4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
        #[must_use]
        #[inline(always)]
        pub const fn c1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
        #[inline(always)]
        pub const fn set_c1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for CscCoef1 {
        #[inline(always)]
        fn default() -> CscCoef1 {
            CscCoef1(0)
        }
    }
    impl core::fmt::Debug for CscCoef1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CscCoef1")
                .field("c4", &self.c4())
                .field("c1", &self.c1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CscCoef1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CscCoef1 {{ c4: {=u16:?}, c1: {=u16:?} }}",
                self.c4(),
                self.c1()
            )
        }
    }
    #[doc = "YUV2RGB coefficients Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CscCoef2(pub u32);
    impl CscCoef2 {
        #[doc = "Two's compliment Green U/Cb multiplier coefficient C3. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
        #[must_use]
        #[inline(always)]
        pub const fn c3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Green U/Cb multiplier coefficient C3. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
        #[inline(always)]
        pub const fn set_c3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Two's compliment Green V/Cr multiplier coefficient C2. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
        #[must_use]
        #[inline(always)]
        pub const fn c2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Green V/Cr multiplier coefficient C2. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
        #[inline(always)]
        pub const fn set_c2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for CscCoef2 {
        #[inline(always)]
        fn default() -> CscCoef2 {
            CscCoef2(0)
        }
    }
    impl core::fmt::Debug for CscCoef2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CscCoef2")
                .field("c3", &self.c3())
                .field("c2", &self.c2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CscCoef2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CscCoef2 {{ c3: {=u16:?}, c2: {=u16:?} }}",
                self.c3(),
                self.c2()
            )
        }
    }
    #[doc = "Image height register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Height(pub u32);
    impl Height {
        #[doc = "Image Height (it is the max index of pixel counting from 0, assuming the top left pixel is indexed as \\[0,0\\])."]
        #[must_use]
        #[inline(always)]
        pub const fn img(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Image Height (it is the max index of pixel counting from 0, assuming the top left pixel is indexed as \\[0,0\\])."]
        #[inline(always)]
        pub const fn set_img(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Height {
        #[inline(always)]
        fn default() -> Height {
            Height(0)
        }
    }
    impl core::fmt::Debug for Height {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Height").field("img", &self.img()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Height {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Height {{ img: {=u16:?} }}", self.img())
        }
    }
    #[doc = "Image Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ImgReg1(pub u32);
    impl ImgReg1 {
        #[doc = "Ncol is the number of color components in the image data to process minus 1. For example, for a grayscale image Ncol=0, for an RGB image, Ncol=2."]
        #[must_use]
        #[inline(always)]
        pub const fn ncol(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Ncol is the number of color components in the image data to process minus 1. For example, for a grayscale image Ncol=0, for an RGB image, Ncol=2."]
        #[inline(always)]
        pub const fn set_ncol(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Encoder Use only. Asseted to enable the Restart Marker processing. A Restart Marker is inserted in the outputted ECS (Entropy Coded Segment) every NRST+1 MCUs."]
        #[must_use]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Encoder Use only. Asseted to enable the Restart Marker processing. A Restart Marker is inserted in the outputted ECS (Entropy Coded Segment) every NRST+1 MCUs."]
        #[inline(always)]
        pub const fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for ImgReg1 {
        #[inline(always)]
        fn default() -> ImgReg1 {
            ImgReg1(0)
        }
    }
    impl core::fmt::Debug for ImgReg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ImgReg1")
                .field("ncol", &self.ncol())
                .field("re", &self.re())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ImgReg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ImgReg1 {{ ncol: {=u8:?}, re: {=bool:?} }}",
                self.ncol(),
                self.re()
            )
        }
    }
    #[doc = "Image Control Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ImgReg2(pub u32);
    impl ImgReg2 {
        #[doc = "Encoder Use only. The number of NMCU to be generated in encoder mode."]
        #[must_use]
        #[inline(always)]
        pub const fn nmcu(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "Encoder Use only. The number of NMCU to be generated in encoder mode."]
        #[inline(always)]
        pub const fn set_nmcu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for ImgReg2 {
        #[inline(always)]
        fn default() -> ImgReg2 {
            ImgReg2(0)
        }
    }
    impl core::fmt::Debug for ImgReg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ImgReg2")
                .field("nmcu", &self.nmcu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ImgReg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ImgReg2 {{ nmcu: {=u32:?} }}", self.nmcu())
        }
    }
    #[doc = "Image Control Register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ImgReg3(pub u32);
    impl ImgReg3 {
        #[doc = "Encoder use only. It is the number of MCUs between two Restart Markers (if enabled) minus 1. The content of this register is ignored if the Re bit inregister 1 is not set."]
        #[must_use]
        #[inline(always)]
        pub const fn nrst(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Encoder use only. It is the number of MCUs between two Restart Markers (if enabled) minus 1. The content of this register is ignored if the Re bit inregister 1 is not set."]
        #[inline(always)]
        pub const fn set_nrst(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ImgReg3 {
        #[inline(always)]
        fn default() -> ImgReg3 {
            ImgReg3(0)
        }
    }
    impl core::fmt::Debug for ImgReg3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ImgReg3")
                .field("nrst", &self.nrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ImgReg3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ImgReg3 {{ nrst: {=u16:?} }}", self.nrst())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imgreg(pub u32);
    impl Imgreg {
        #[doc = "Encoder use only. The selection of the Huffman table for the encoding of the DC coefficients in the data units belonging to the color component."]
        #[must_use]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Encoder use only. The selection of the Huffman table for the encoding of the DC coefficients in the data units belonging to the color component."]
        #[inline(always)]
        pub const fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Encoder use only. The selection of the Huffman table for the encoding of the AC coefficients in the data units belonging to the color component."]
        #[must_use]
        #[inline(always)]
        pub const fn ha(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Encoder use only. The selection of the Huffman table for the encoding of the AC coefficients in the data units belonging to the color component."]
        #[inline(always)]
        pub const fn set_ha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Encoder use only. The selection of the quantization table."]
        #[must_use]
        #[inline(always)]
        pub const fn qt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Encoder use only. The selection of the quantization table."]
        #[inline(always)]
        pub const fn set_qt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Encoder use only. The number of data units (8x8 blocks of data) of the color componet contained in the MCU minus 1."]
        #[must_use]
        #[inline(always)]
        pub const fn nblock(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Encoder use only. The number of data units (8x8 blocks of data) of the color componet contained in the MCU minus 1."]
        #[inline(always)]
        pub const fn set_nblock(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Imgreg {
        #[inline(always)]
        fn default() -> Imgreg {
            Imgreg(0)
        }
    }
    impl core::fmt::Debug for Imgreg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Imgreg")
                .field("hd", &self.hd())
                .field("ha", &self.ha())
                .field("qt", &self.qt())
                .field("nblock", &self.nblock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Imgreg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Imgreg {{ hd: {=bool:?}, ha: {=bool:?}, qt: {=u8:?}, nblock: {=u8:?} }}",
                self.hd(),
                self.ha(),
                self.qt(),
                self.nblock()
            )
        }
    }
    #[doc = "In DMA Buf Control 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InDmaCtrl0(pub u32);
    impl InDmaCtrl0 {
        #[doc = "Pitch between the starting point of Rows. Only active when In_DMA_ID=Pixel.."]
        #[must_use]
        #[inline(always)]
        pub const fn pitch(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Pitch between the starting point of Rows. Only active when In_DMA_ID=Pixel.."]
        #[inline(always)]
        pub const fn set_pitch(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Total length (Low 16 bits) in Bytes -1 for transfer when In_DMA_ID!=Pixel."]
        #[must_use]
        #[inline(always)]
        pub const fn ttlen(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Total length (Low 16 bits) in Bytes -1 for transfer when In_DMA_ID!=Pixel."]
        #[inline(always)]
        pub const fn set_ttlen(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for InDmaCtrl0 {
        #[inline(always)]
        fn default() -> InDmaCtrl0 {
            InDmaCtrl0(0)
        }
    }
    impl core::fmt::Debug for InDmaCtrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("InDmaCtrl0")
                .field("pitch", &self.pitch())
                .field("ttlen", &self.ttlen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for InDmaCtrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "InDmaCtrl0 {{ pitch: {=u16:?}, ttlen: {=u16:?} }}",
                self.pitch(),
                self.ttlen()
            )
        }
    }
    #[doc = "In DMA Buf Control 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InDmaCtrl1(pub u32);
    impl InDmaCtrl1 {
        #[doc = "Total length (High 16 bits) in Bytes -1 for transfer. See reference in InDMA_Ctrl0\\[TTLEN\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn rowlen(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Total length (High 16 bits) in Bytes -1 for transfer. See reference in InDMA_Ctrl0\\[TTLEN\\]."]
        #[inline(always)]
        pub const fn set_rowlen(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for InDmaCtrl1 {
        #[inline(always)]
        fn default() -> InDmaCtrl1 {
            InDmaCtrl1(0)
        }
    }
    impl core::fmt::Debug for InDmaCtrl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("InDmaCtrl1")
                .field("rowlen", &self.rowlen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for InDmaCtrl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "InDmaCtrl1 {{ rowlen: {=u16:?} }}", self.rowlen())
        }
    }
    #[doc = "In DMA Misc Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InDmaMisc(pub u32);
    impl InDmaMisc {
        #[doc = "Asserted if In_DMA_ID=Pixel."]
        #[must_use]
        #[inline(always)]
        pub const fn indma2d(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted if In_DMA_ID=Pixel."]
        #[inline(always)]
        pub const fn set_indma2d(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Asserted to request DMA. Automatically clear after DMA is done."]
        #[must_use]
        #[inline(always)]
        pub const fn in_dma_req(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to request DMA. Automatically clear after DMA is done."]
        #[inline(always)]
        pub const fn set_in_dma_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "0: Pixel (In) 1: ECS (In) 2: Qmem 3: HuffEnc 4: HuffMin 5: HuffBase 6: HuffSymb."]
        #[must_use]
        #[inline(always)]
        pub const fn in_dma_id(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "0: Pixel (In) 1: ECS (In) 2: Qmem 3: HuffEnc 4: HuffMin 5: HuffBase 6: HuffSymb."]
        #[inline(always)]
        pub const fn set_in_dma_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "interrupt enable for all interrupt sources of In DMA module."]
        #[must_use]
        #[inline(always)]
        pub const fn irq_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable for all interrupt sources of In DMA module."]
        #[inline(always)]
        pub const fn set_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "In DMA axi bus error inetrrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn axi_err_irq_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "In DMA axi bus error inetrrupt enable."]
        #[inline(always)]
        pub const fn set_axi_err_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "In DMA Done enable."]
        #[must_use]
        #[inline(always)]
        pub const fn in_dma_done_irq_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "In DMA Done enable."]
        #[inline(always)]
        pub const fn set_in_dma_done_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "In DMA Next Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn nxt_irq_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "In DMA Next Interrupt Enable."]
        #[inline(always)]
        pub const fn set_nxt_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Renew In DMA. Default is to continue the write address counter when a new DMA request comes. Asserted to reset the write address counter."]
        #[must_use]
        #[inline(always)]
        pub const fn indma_renew(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Renew In DMA. Default is to continue the write address counter when a new DMA request comes. Asserted to reset the write address counter."]
        #[inline(always)]
        pub const fn set_indma_renew(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. Only work for pixel data. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}."]
        #[must_use]
        #[inline(always)]
        pub const fn pack_dir(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. Only work for pixel data. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}."]
        #[inline(always)]
        pub const fn set_pack_dir(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Swap bit\\[31:24\\]
and bit \\[15:8\\]
before pack dir operation. Only work for pixel data."]
        #[must_use]
        #[inline(always)]
        pub const fn inb13_swap(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Swap bit\\[31:24\\]
and bit \\[15:8\\]
before pack dir operation. Only work for pixel data."]
        #[inline(always)]
        pub const fn set_inb13_swap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "max_ot when input are RGB pixels. For 16 bits per pixel, it can be set as 4. For 32 bits per pixel, it will be set as 2."]
        #[must_use]
        #[inline(always)]
        pub const fn max_ot(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x0f;
            val as u8
        }
        #[doc = "max_ot when input are RGB pixels. For 16 bits per pixel, it can be set as 4. For 32 bits per pixel, it will be set as 2."]
        #[inline(always)]
        pub const fn set_max_ot(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 15usize)) | (((val as u32) & 0x0f) << 15usize);
        }
        #[doc = "QoS for AXI read channel."]
        #[must_use]
        #[inline(always)]
        pub const fn arqos(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x0f;
            val as u8
        }
        #[doc = "QoS for AXI read channel."]
        #[inline(always)]
        pub const fn set_arqos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
        }
    }
    impl Default for InDmaMisc {
        #[inline(always)]
        fn default() -> InDmaMisc {
            InDmaMisc(0)
        }
    }
    impl core::fmt::Debug for InDmaMisc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("InDmaMisc")
                .field("indma2d", &self.indma2d())
                .field("in_dma_req", &self.in_dma_req())
                .field("in_dma_id", &self.in_dma_id())
                .field("irq_en", &self.irq_en())
                .field("axi_err_irq_en", &self.axi_err_irq_en())
                .field("in_dma_done_irq_en", &self.in_dma_done_irq_en())
                .field("nxt_irq_en", &self.nxt_irq_en())
                .field("indma_renew", &self.indma_renew())
                .field("pack_dir", &self.pack_dir())
                .field("inb13_swap", &self.inb13_swap())
                .field("max_ot", &self.max_ot())
                .field("arqos", &self.arqos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for InDmaMisc {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "InDmaMisc {{ indma2d: {=bool:?}, in_dma_req: {=bool:?}, in_dma_id: {=u8:?}, irq_en: {=bool:?}, axi_err_irq_en: {=bool:?}, in_dma_done_irq_en: {=bool:?}, nxt_irq_en: {=bool:?}, indma_renew: {=bool:?}, pack_dir: {=u8:?}, inb13_swap: {=bool:?}, max_ot: {=u8:?}, arqos: {=u8:?} }}" , self . indma2d () , self . in_dma_req () , self . in_dma_id () , self . irq_en () , self . axi_err_irq_en () , self . in_dma_done_irq_en () , self . nxt_irq_en () , self . indma_renew () , self . pack_dir () , self . inb13_swap () , self . max_ot () , self . arqos ())
        }
    }
    #[doc = "In DMA Buf Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InDmabase(pub u32);
    impl InDmabase {
        #[doc = "Y plane (or Encoded Bit Plane)."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Y plane (or Encoded Bit Plane)."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for InDmabase {
        #[inline(always)]
        fn default() -> InDmabase {
            InDmabase(0)
        }
    }
    impl core::fmt::Debug for InDmabase {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("InDmabase")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for InDmabase {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "InDmabase {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "In DMA Next Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InxtCmd(pub u32);
    impl InxtCmd {
        #[doc = "NXTCMD phase Enable Bit."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "NXTCMD phase Enable Bit."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "asserted if there is either a DATA DMA phase or NXTCMD phase. Automatically cleared. Will trigger the InDMA transfer if CFG\\[JPEG_EN\\]
is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn op_valid(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "asserted if there is either a DATA DMA phase or NXTCMD phase. Automatically cleared. Will trigger the InDMA transfer if CFG\\[JPEG_EN\\]
is 1."]
        #[inline(always)]
        pub const fn set_op_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "The address pointing to the next command."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "The address pointing to the next command."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for InxtCmd {
        #[inline(always)]
        fn default() -> InxtCmd {
            InxtCmd(0)
        }
    }
    impl core::fmt::Debug for InxtCmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("InxtCmd")
                .field("en", &self.en())
                .field("op_valid", &self.op_valid())
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for InxtCmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "InxtCmd {{ en: {=bool:?}, op_valid: {=bool:?}, addr: {=u32:?} }}",
                self.en(),
                self.op_valid(),
                self.addr()
            )
        }
    }
    #[doc = "Out DMA Next Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OnxtCmd(pub u32);
    impl OnxtCmd {
        #[doc = "NXTCMD phase Enable Bit."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "NXTCMD phase Enable Bit."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "asserted if there is either a DATA DMA phase or NXTCMD phase. Automatically cleared. Will trigger the OutDMA and NXTCMD phase transfer if CFG\\[JPEG_EN\\]
is 1."]
        #[must_use]
        #[inline(always)]
        pub const fn op_valid(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "asserted if there is either a DATA DMA phase or NXTCMD phase. Automatically cleared. Will trigger the OutDMA and NXTCMD phase transfer if CFG\\[JPEG_EN\\]
is 1."]
        #[inline(always)]
        pub const fn set_op_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "The address pointing to the next command."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "The address pointing to the next command."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for OnxtCmd {
        #[inline(always)]
        fn default() -> OnxtCmd {
            OnxtCmd(0)
        }
    }
    impl core::fmt::Debug for OnxtCmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OnxtCmd")
                .field("en", &self.en())
                .field("op_valid", &self.op_valid())
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OnxtCmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OnxtCmd {{ en: {=bool:?}, op_valid: {=bool:?}, addr: {=u32:?} }}",
                self.en(),
                self.op_valid(),
                self.addr()
            )
        }
    }
    #[doc = "Out DMA Buf Control 0 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutDmaCtrl0(pub u32);
    impl OutDmaCtrl0 {
        #[doc = "Pitch between the starting point of Rows when Out_DMA_ID==Pixel."]
        #[must_use]
        #[inline(always)]
        pub const fn pitch(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Pitch between the starting point of Rows when Out_DMA_ID==Pixel."]
        #[inline(always)]
        pub const fn set_pitch(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Total length (Low 16 bits) in Bytes -1 for transfer when Out_DMA_ID!=Pixel. If Out_DMA_ID=ECS, it can be any value greater than the length of the ECS, for example, the number of encoded bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn ttlen(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Total length (Low 16 bits) in Bytes -1 for transfer when Out_DMA_ID!=Pixel. If Out_DMA_ID=ECS, it can be any value greater than the length of the ECS, for example, the number of encoded bytes."]
        #[inline(always)]
        pub const fn set_ttlen(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for OutDmaCtrl0 {
        #[inline(always)]
        fn default() -> OutDmaCtrl0 {
            OutDmaCtrl0(0)
        }
    }
    impl core::fmt::Debug for OutDmaCtrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OutDmaCtrl0")
                .field("pitch", &self.pitch())
                .field("ttlen", &self.ttlen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OutDmaCtrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OutDmaCtrl0 {{ pitch: {=u16:?}, ttlen: {=u16:?} }}",
                self.pitch(),
                self.ttlen()
            )
        }
    }
    #[doc = "Out DMA Buf Control 1 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutDmaCtrl1(pub u32);
    impl OutDmaCtrl1 {
        #[doc = "Total length (High 16 bits) in Bytes -1 for transfer. See reference in OutDMA_Ctrl0\\[TTLEN\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn rowlen(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Total length (High 16 bits) in Bytes -1 for transfer. See reference in OutDMA_Ctrl0\\[TTLEN\\]."]
        #[inline(always)]
        pub const fn set_rowlen(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for OutDmaCtrl1 {
        #[inline(always)]
        fn default() -> OutDmaCtrl1 {
            OutDmaCtrl1(0)
        }
    }
    impl core::fmt::Debug for OutDmaCtrl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OutDmaCtrl1")
                .field("rowlen", &self.rowlen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OutDmaCtrl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OutDmaCtrl1 {{ rowlen: {=u16:?} }}", self.rowlen())
        }
    }
    #[doc = "Out DMA Misc Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutDmaMisc(pub u32);
    impl OutDmaMisc {
        #[doc = "Asserted if Out_DMA_ID==Pixel."]
        #[must_use]
        #[inline(always)]
        pub const fn outdma2d(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted if Out_DMA_ID==Pixel."]
        #[inline(always)]
        pub const fn set_outdma2d(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Asserted to enable Out DMA request."]
        #[must_use]
        #[inline(always)]
        pub const fn out_dma_req(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enable Out DMA request."]
        #[inline(always)]
        pub const fn set_out_dma_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "0: Pixel (Out) 1: ECS (Out)."]
        #[must_use]
        #[inline(always)]
        pub const fn out_dma_id(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "0: Pixel (Out) 1: ECS (Out)."]
        #[inline(always)]
        pub const fn set_out_dma_id(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "interrupt enable for all interrupt sources of Out DMA module."]
        #[must_use]
        #[inline(always)]
        pub const fn irq_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable for all interrupt sources of Out DMA module."]
        #[inline(always)]
        pub const fn set_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Out DMA axi bus error inetrrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn axi_err_irq_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Out DMA axi bus error inetrrupt enable."]
        #[inline(always)]
        pub const fn set_axi_err_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Out DMA Done interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn out_dma_done_irq_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Out DMA Done interrupt Enable."]
        #[inline(always)]
        pub const fn set_out_dma_done_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Out DMA Next Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn nxt_irq_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Out DMA Next Interrupt Enable."]
        #[inline(always)]
        pub const fn set_nxt_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Add 0xFFD9 to the ending of the odma stream when all original image pixels are processed by the encoder module."]
        #[must_use]
        #[inline(always)]
        pub const fn add_odma_endings(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Add 0xFFD9 to the ending of the odma stream when all original image pixels are processed by the encoder module."]
        #[inline(always)]
        pub const fn set_add_odma_endings(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Asserted to ini output counter."]
        #[must_use]
        #[inline(always)]
        pub const fn ini_outcnt(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to ini output counter."]
        #[inline(always)]
        pub const fn set_ini_outcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable output counter (unit as bytes)."]
        #[must_use]
        #[inline(always)]
        pub const fn en_outcnt(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable output counter (unit as bytes)."]
        #[inline(always)]
        pub const fn set_en_outcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. All outdma data are impacted. 2'b00: no change {A3, A2, A1, A0} (This is used for ecs stream) 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}."]
        #[must_use]
        #[inline(always)]
        pub const fn pack_dir(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. All outdma data are impacted. 2'b00: no change {A3, A2, A1, A0} (This is used for ecs stream) 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}."]
        #[inline(always)]
        pub const fn set_pack_dir(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn awqos(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_awqos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
        }
    }
    impl Default for OutDmaMisc {
        #[inline(always)]
        fn default() -> OutDmaMisc {
            OutDmaMisc(0)
        }
    }
    impl core::fmt::Debug for OutDmaMisc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OutDmaMisc")
                .field("outdma2d", &self.outdma2d())
                .field("out_dma_req", &self.out_dma_req())
                .field("out_dma_id", &self.out_dma_id())
                .field("irq_en", &self.irq_en())
                .field("axi_err_irq_en", &self.axi_err_irq_en())
                .field("out_dma_done_irq_en", &self.out_dma_done_irq_en())
                .field("nxt_irq_en", &self.nxt_irq_en())
                .field("add_odma_endings", &self.add_odma_endings())
                .field("ini_outcnt", &self.ini_outcnt())
                .field("en_outcnt", &self.en_outcnt())
                .field("pack_dir", &self.pack_dir())
                .field("awqos", &self.awqos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OutDmaMisc {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "OutDmaMisc {{ outdma2d: {=bool:?}, out_dma_req: {=bool:?}, out_dma_id: {=bool:?}, irq_en: {=bool:?}, axi_err_irq_en: {=bool:?}, out_dma_done_irq_en: {=bool:?}, nxt_irq_en: {=bool:?}, add_odma_endings: {=bool:?}, ini_outcnt: {=bool:?}, en_outcnt: {=bool:?}, pack_dir: {=u8:?}, awqos: {=u8:?} }}" , self . outdma2d () , self . out_dma_req () , self . out_dma_id () , self . irq_en () , self . axi_err_irq_en () , self . out_dma_done_irq_en () , self . nxt_irq_en () , self . add_odma_endings () , self . ini_outcnt () , self . en_outcnt () , self . pack_dir () , self . awqos ())
        }
    }
    #[doc = "Out DMA Buf Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutDmabase(pub u32);
    impl OutDmabase {
        #[doc = "Y plane (or Encoded Bit Plane)."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Y plane (or Encoded Bit Plane)."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OutDmabase {
        #[inline(always)]
        fn default() -> OutDmabase {
            OutDmabase(0)
        }
    }
    impl core::fmt::Debug for OutDmabase {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OutDmabase")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OutDmabase {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OutDmabase {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Out DMA Bytes Counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutDmacnt(pub u32);
    impl OutDmacnt {
        #[doc = "The out DMA counter."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The out DMA counter."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OutDmacnt {
        #[inline(always)]
        fn default() -> OutDmacnt {
            OutDmacnt(0)
        }
    }
    impl core::fmt::Debug for OutDmacnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OutDmacnt")
                .field("val", &self.val())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OutDmacnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OutDmacnt {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "RGB2YUV coefficients Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgb2yuvCoef0(pub u32);
    impl Rgb2yuvCoef0 {
        #[doc = "CSC parameters Y_OFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn y_offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "CSC parameters Y_OFFSET."]
        #[inline(always)]
        pub const fn set_y_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "CSC parameters UV_OFFSET."]
        #[must_use]
        #[inline(always)]
        pub const fn uv_offset(&self) -> u16 {
            let val = (self.0 >> 9usize) & 0x01ff;
            val as u16
        }
        #[doc = "CSC parameters UV_OFFSET."]
        #[inline(always)]
        pub const fn set_uv_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 9usize)) | (((val as u32) & 0x01ff) << 9usize);
        }
        #[doc = "CSC parameters C0."]
        #[must_use]
        #[inline(always)]
        pub const fn c0(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C0."]
        #[inline(always)]
        pub const fn set_c0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
        }
        #[doc = "Asserted to enable this RGB2YCbCr CSC stage."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enable this RGB2YCbCr CSC stage."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Asserted to use YCrCb mode. Must be assigned as 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ycbcr_mode(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to use YCrCb mode. Must be assigned as 1."]
        #[inline(always)]
        pub const fn set_ycbcr_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rgb2yuvCoef0 {
        #[inline(always)]
        fn default() -> Rgb2yuvCoef0 {
            Rgb2yuvCoef0(0)
        }
    }
    impl core::fmt::Debug for Rgb2yuvCoef0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgb2yuvCoef0")
                .field("y_offset", &self.y_offset())
                .field("uv_offset", &self.uv_offset())
                .field("c0", &self.c0())
                .field("enable", &self.enable())
                .field("ycbcr_mode", &self.ycbcr_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rgb2yuvCoef0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rgb2yuvCoef0 {{ y_offset: {=u16:?}, uv_offset: {=u16:?}, c0: {=u16:?}, enable: {=bool:?}, ycbcr_mode: {=bool:?} }}" , self . y_offset () , self . uv_offset () , self . c0 () , self . enable () , self . ycbcr_mode ())
        }
    }
    #[doc = "RGB2YUV coefficients Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgb2yuvCoef1(pub u32);
    impl Rgb2yuvCoef1 {
        #[doc = "CSC parameters C4."]
        #[must_use]
        #[inline(always)]
        pub const fn c4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C4."]
        #[inline(always)]
        pub const fn set_c4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "CSC parameters C1."]
        #[must_use]
        #[inline(always)]
        pub const fn c1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C1."]
        #[inline(always)]
        pub const fn set_c1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for Rgb2yuvCoef1 {
        #[inline(always)]
        fn default() -> Rgb2yuvCoef1 {
            Rgb2yuvCoef1(0)
        }
    }
    impl core::fmt::Debug for Rgb2yuvCoef1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgb2yuvCoef1")
                .field("c4", &self.c4())
                .field("c1", &self.c1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rgb2yuvCoef1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rgb2yuvCoef1 {{ c4: {=u16:?}, c1: {=u16:?} }}",
                self.c4(),
                self.c1()
            )
        }
    }
    #[doc = "RGB2YUV coefficients Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgb2yuvCoef2(pub u32);
    impl Rgb2yuvCoef2 {
        #[doc = "CSC parameters C3."]
        #[must_use]
        #[inline(always)]
        pub const fn c3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C3."]
        #[inline(always)]
        pub const fn set_c3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "CSC parameters C2."]
        #[must_use]
        #[inline(always)]
        pub const fn c2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C2."]
        #[inline(always)]
        pub const fn set_c2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for Rgb2yuvCoef2 {
        #[inline(always)]
        fn default() -> Rgb2yuvCoef2 {
            Rgb2yuvCoef2(0)
        }
    }
    impl core::fmt::Debug for Rgb2yuvCoef2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgb2yuvCoef2")
                .field("c3", &self.c3())
                .field("c2", &self.c2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rgb2yuvCoef2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rgb2yuvCoef2 {{ c3: {=u16:?}, c2: {=u16:?} }}",
                self.c3(),
                self.c2()
            )
        }
    }
    #[doc = "RGB2YUV coefficients Register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgb2yuvCoef3(pub u32);
    impl Rgb2yuvCoef3 {
        #[doc = "CSC parameters C5."]
        #[must_use]
        #[inline(always)]
        pub const fn c5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C5."]
        #[inline(always)]
        pub const fn set_c5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "CSC parameters C6."]
        #[must_use]
        #[inline(always)]
        pub const fn c6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C6."]
        #[inline(always)]
        pub const fn set_c6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for Rgb2yuvCoef3 {
        #[inline(always)]
        fn default() -> Rgb2yuvCoef3 {
            Rgb2yuvCoef3(0)
        }
    }
    impl core::fmt::Debug for Rgb2yuvCoef3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgb2yuvCoef3")
                .field("c5", &self.c5())
                .field("c6", &self.c6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rgb2yuvCoef3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rgb2yuvCoef3 {{ c5: {=u16:?}, c6: {=u16:?} }}",
                self.c5(),
                self.c6()
            )
        }
    }
    #[doc = "RGB2YUV coefficients Register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgb2yuvCoef4(pub u32);
    impl Rgb2yuvCoef4 {
        #[doc = "CSC parameters C7."]
        #[must_use]
        #[inline(always)]
        pub const fn c7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C7."]
        #[inline(always)]
        pub const fn set_c7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "CSC parameters C8."]
        #[must_use]
        #[inline(always)]
        pub const fn c8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C8."]
        #[inline(always)]
        pub const fn set_c8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for Rgb2yuvCoef4 {
        #[inline(always)]
        fn default() -> Rgb2yuvCoef4 {
            Rgb2yuvCoef4(0)
        }
    }
    impl core::fmt::Debug for Rgb2yuvCoef4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rgb2yuvCoef4")
                .field("c7", &self.c7())
                .field("c8", &self.c8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rgb2yuvCoef4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rgb2yuvCoef4 {{ c7: {=u16:?}, c8: {=u16:?} }}",
                self.c7(),
                self.c8()
            )
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stat(pub u32);
    impl Stat {
        #[doc = "codec restart marker error interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn restart_marker_error(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "codec restart marker error interrupt."]
        #[inline(always)]
        pub const fn set_restart_marker_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Coding or decoding process is over. DMA is not included. The module is completely not busy only when in_dma_transfer_done and out_dma_transfer_done, and codec_over are all asserted."]
        #[must_use]
        #[inline(always)]
        pub const fn codec_over(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Coding or decoding process is over. DMA is not included. The module is completely not busy only when in_dma_transfer_done and out_dma_transfer_done, and codec_over are all asserted."]
        #[inline(always)]
        pub const fn set_codec_over(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "InDMA process done."]
        #[must_use]
        #[inline(always)]
        pub const fn in_dma_transfer_done(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "InDMA process done."]
        #[inline(always)]
        pub const fn set_in_dma_transfer_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OutDMA process done."]
        #[must_use]
        #[inline(always)]
        pub const fn out_dma_transfer_done(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OutDMA process done."]
        #[inline(always)]
        pub const fn set_out_dma_transfer_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "InDMA next interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn inxt_irq(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "InDMA next interrupt."]
        #[inline(always)]
        pub const fn set_inxt_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "OutDMA next interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn onxt_irq(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "OutDMA next interrupt."]
        #[inline(always)]
        pub const fn set_onxt_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "axi bus error."]
        #[must_use]
        #[inline(always)]
        pub const fn axi_err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "axi bus error."]
        #[inline(always)]
        pub const fn set_axi_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "out-dma axi bus error."]
        #[must_use]
        #[inline(always)]
        pub const fn axi_write_err(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "out-dma axi bus error."]
        #[inline(always)]
        pub const fn set_axi_write_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "in-dma axi bus error."]
        #[must_use]
        #[inline(always)]
        pub const fn axi_read_err(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "in-dma axi bus error."]
        #[inline(always)]
        pub const fn set_axi_read_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "the axi err id."]
        #[must_use]
        #[inline(always)]
        pub const fn axi_err_id(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x0f;
            val as u8
        }
        #[doc = "the axi err id."]
        #[inline(always)]
        pub const fn set_axi_err_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
        }
        #[doc = "When 1 means that the module is busy doing conversion and data transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "When 1 means that the module is busy doing conversion and data transfer."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Stat {
        #[inline(always)]
        fn default() -> Stat {
            Stat(0)
        }
    }
    impl core::fmt::Debug for Stat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Stat")
                .field("restart_marker_error", &self.restart_marker_error())
                .field("codec_over", &self.codec_over())
                .field("in_dma_transfer_done", &self.in_dma_transfer_done())
                .field("out_dma_transfer_done", &self.out_dma_transfer_done())
                .field("inxt_irq", &self.inxt_irq())
                .field("onxt_irq", &self.onxt_irq())
                .field("axi_err", &self.axi_err())
                .field("axi_write_err", &self.axi_write_err())
                .field("axi_read_err", &self.axi_read_err())
                .field("axi_err_id", &self.axi_err_id())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Stat {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Stat {{ restart_marker_error: {=bool:?}, codec_over: {=bool:?}, in_dma_transfer_done: {=bool:?}, out_dma_transfer_done: {=bool:?}, inxt_irq: {=bool:?}, onxt_irq: {=bool:?}, axi_err: {=bool:?}, axi_write_err: {=bool:?}, axi_read_err: {=bool:?}, axi_err_id: {=u8:?}, busy: {=bool:?} }}" , self . restart_marker_error () , self . codec_over () , self . in_dma_transfer_done () , self . out_dma_transfer_done () , self . inxt_irq () , self . onxt_irq () , self . axi_err () , self . axi_write_err () , self . axi_read_err () , self . axi_err_id () , self . busy ())
        }
    }
    #[doc = "Image width register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Width(pub u32);
    impl Width {
        #[doc = "Image Width (it is the max index of pixel counting from 0, assuming the top left pixel is indexed as \\[0,0\\])."]
        #[must_use]
        #[inline(always)]
        pub const fn img(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Image Width (it is the max index of pixel counting from 0, assuming the top left pixel is indexed as \\[0,0\\])."]
        #[inline(always)]
        pub const fn set_img(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Width {
        #[inline(always)]
        fn default() -> Width {
            Width(0)
        }
    }
    impl core::fmt::Debug for Width {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Width").field("img", &self.img()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Width {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Width {{ img: {=u16:?} }}", self.img())
        }
    }
}
