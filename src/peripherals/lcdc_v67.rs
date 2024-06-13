#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer {
    ptr: *mut u8,
}
unsafe impl Send for Layer {}
unsafe impl Sync for Layer {}
impl Layer {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Layer Control Register."]
    #[inline(always)]
    pub const fn layctrl(self) -> crate::common::Reg<regs::Layctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Layer Alpha Register."]
    #[inline(always)]
    pub const fn alphas(self) -> crate::common::Reg<regs::Alphas, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Layer Size Register."]
    #[inline(always)]
    pub const fn laysize(self) -> crate::common::Reg<regs::Laysize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Layer Position Register."]
    #[inline(always)]
    pub const fn laypos(self) -> crate::common::Reg<regs::Laypos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Layer Buffer Pointer Register."]
    #[inline(always)]
    pub const fn start0(self) -> crate::common::Reg<regs::Start0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Layer Bus Config Register."]
    #[inline(always)]
    pub const fn linecfg(self) -> crate::common::Reg<regs::Linecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Layer Background Color Register."]
    #[inline(always)]
    pub const fn bg_cl(self) -> crate::common::Reg<regs::BgCl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Layer Color Space Conversion Config Register 0."]
    #[inline(always)]
    pub const fn csc_coef0(self) -> crate::common::Reg<regs::CscCoef0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Layer Color Space Conversion Config Register 1."]
    #[inline(always)]
    pub const fn csc_coef1(self) -> crate::common::Reg<regs::CscCoef1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Layer Color Space Conversion Config Register 2."]
    #[inline(always)]
    pub const fn csc_coef2(self) -> crate::common::Reg<regs::CscCoef2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
#[doc = "LCDC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcdc {
    ptr: *mut u8,
}
unsafe impl Send for Lcdc {}
unsafe impl Sync for Lcdc {}
impl Lcdc {
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
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Background Color Register."]
    #[inline(always)]
    pub const fn bgnd_cl(self) -> crate::common::Reg<regs::BgndCl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Display Window Size Register."]
    #[inline(always)]
    pub const fn disp_wn_size(self) -> crate::common::Reg<regs::DispWnSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "HSYNC Config Register."]
    #[inline(always)]
    pub const fn hsync_para(self) -> crate::common::Reg<regs::HsyncPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "VSYNC Config Register."]
    #[inline(always)]
    pub const fn vsync_para(self) -> crate::common::Reg<regs::VsyncPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "DMA Status Register."]
    #[inline(always)]
    pub const fn dma_st(self) -> crate::common::Reg<regs::DmaSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn st(self) -> crate::common::Reg<regs::St, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "TX FIFO Register."]
    #[inline(always)]
    pub const fn txfifo(self) -> crate::common::Reg<regs::Txfifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn layer(self, n: usize) -> Layer {
        assert!(n < 8usize);
        unsafe { Layer::from_ptr(self.ptr.add(0x0200usize + n * 64usize) as _) }
    }
    #[doc = "Clut Load Control Register."]
    #[inline(always)]
    pub const fn clut_load(self) -> crate::common::Reg<regs::ClutLoad, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
}
pub mod regs {
    #[doc = "Layer Alpha Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alphas(pub u32);
    impl Alphas {
        #[doc = "The system alpha value for the input stream from previous stage (DST)."]
        #[inline(always)]
        pub const fn ind(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The system alpha value for the input stream from previous stage (DST)."]
        #[inline(always)]
        pub fn set_ind(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "The system alpha value for the data stream of current layer stream (SRC)."]
        #[inline(always)]
        pub const fn locd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "The system alpha value for the data stream of current layer stream (SRC)."]
        #[inline(always)]
        pub fn set_locd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Alphas {
        #[inline(always)]
        fn default() -> Alphas {
            Alphas(0)
        }
    }
    #[doc = "Layer Background Color Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BgCl(pub u32);
    impl BgCl {
        #[doc = "ARGB8888. It is only useful in the last active stage in the pipeline."]
        #[inline(always)]
        pub const fn argb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ARGB8888. It is only useful in the last active stage in the pipeline."]
        #[inline(always)]
        pub fn set_argb(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BgCl {
        #[inline(always)]
        fn default() -> BgCl {
            BgCl(0)
        }
    }
    #[doc = "Background Color Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BgndCl(pub u32);
    impl BgndCl {
        #[doc = "Blue component of the default color displayed in the sectors where no layer is active."]
        #[inline(always)]
        pub const fn b(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Blue component of the default color displayed in the sectors where no layer is active."]
        #[inline(always)]
        pub fn set_b(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Green component of the default color displayed in the sectors where no layer is active."]
        #[inline(always)]
        pub const fn g(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Green component of the default color displayed in the sectors where no layer is active."]
        #[inline(always)]
        pub fn set_g(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Red component of the default color displayed in the sectors where no layer is active."]
        #[inline(always)]
        pub const fn r(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Red component of the default color displayed in the sectors where no layer is active."]
        #[inline(always)]
        pub fn set_r(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for BgndCl {
        #[inline(always)]
        fn default() -> BgndCl {
            BgndCl(0)
        }
    }
    #[doc = "Clut Load Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClutLoad(pub u32);
    impl ClutLoad {
        #[doc = "CLUT Update Enable The bit is written to 1 when software want to update the Color Look Up Tables during display. If set to 1, software update selected CLUT due to SEL_CLUT_NUM setting, the table will be copied from CLUT8 during vertical blanking period after SHADOW_LOAD_EN is set to 1. If set to 0, software can update CLUT8 directly according to the CLUT memory map. Hardware will automatically clear this bit when selected CLUT is updated according to SEL_CLUT_NUM."]
        #[inline(always)]
        pub const fn update_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CLUT Update Enable The bit is written to 1 when software want to update the Color Look Up Tables during display. If set to 1, software update selected CLUT due to SEL_CLUT_NUM setting, the table will be copied from CLUT8 during vertical blanking period after SHADOW_LOAD_EN is set to 1. If set to 0, software can update CLUT8 directly according to the CLUT memory map. Hardware will automatically clear this bit when selected CLUT is updated according to SEL_CLUT_NUM."]
        #[inline(always)]
        pub fn set_update_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Selected CLUT Number The SEL_CLUT_NUM is used to select which plane's CLUT need to be updated. The hardware can only backup one CLUT setting and load, so the SEL_CLUT_NUM can't be changed when CLUT_LOAD\\[UPDATE_EN\\]
is 1. . 3'h0 - PLANE 0 . 3'h1 - PLANE 1 . ------ . 3'h7 - PLANE 7 CLUT 8 can be modified via APB even when display is on. Currently CLUT for plane 0..7 cannot be modified via APB when display is on. Can only be updated via CLUT_LOAD\\[UPDATE_EN\\]
bit."]
        #[inline(always)]
        pub const fn sel_num(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Selected CLUT Number The SEL_CLUT_NUM is used to select which plane's CLUT need to be updated. The hardware can only backup one CLUT setting and load, so the SEL_CLUT_NUM can't be changed when CLUT_LOAD\\[UPDATE_EN\\]
is 1. . 3'h0 - PLANE 0 . 3'h1 - PLANE 1 . ------ . 3'h7 - PLANE 7 CLUT 8 can be modified via APB even when display is on. Currently CLUT for plane 0..7 cannot be modified via APB when display is on. Can only be updated via CLUT_LOAD\\[UPDATE_EN\\]
bit."]
        #[inline(always)]
        pub fn set_sel_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for ClutLoad {
        #[inline(always)]
        fn default() -> ClutLoad {
            ClutLoad(0)
        }
    }
    #[doc = "Layer Color Space Conversion Config Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CscCoef0(pub u32);
    impl CscCoef0 {
        #[doc = "Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
        #[inline(always)]
        pub const fn y_offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
        #[inline(always)]
        pub fn set_y_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
        #[inline(always)]
        pub const fn uv_offset(&self) -> u16 {
            let val = (self.0 >> 9usize) & 0x01ff;
            val as u16
        }
        #[doc = "Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
        #[inline(always)]
        pub fn set_uv_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 9usize)) | (((val as u32) & 0x01ff) << 9usize);
        }
        #[doc = "Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)."]
        #[inline(always)]
        pub const fn c0(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)."]
        #[inline(always)]
        pub fn set_c0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
        }
        #[doc = "Enable the CSC unit in the LCDC plane data path. 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data This bit will be shadowed."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the CSC unit in the LCDC plane data path. 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data This bit will be shadowed."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "This bit changes the behavior when performing U/V converting. 0b - Converting YUV to RGB data 1b - Converting YCbCr to RGB data."]
        #[inline(always)]
        pub const fn ycbcr_mode(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "This bit changes the behavior when performing U/V converting. 0b - Converting YUV to RGB data 1b - Converting YCbCr to RGB data."]
        #[inline(always)]
        pub fn set_ycbcr_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CscCoef0 {
        #[inline(always)]
        fn default() -> CscCoef0 {
            CscCoef0(0)
        }
    }
    #[doc = "Layer Color Space Conversion Config Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CscCoef1(pub u32);
    impl CscCoef1 {
        #[doc = "Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
        #[inline(always)]
        pub const fn c4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
        #[inline(always)]
        pub fn set_c4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
        #[inline(always)]
        pub const fn c1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
        #[inline(always)]
        pub fn set_c1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for CscCoef1 {
        #[inline(always)]
        fn default() -> CscCoef1 {
            CscCoef1(0)
        }
    }
    #[doc = "Layer Color Space Conversion Config Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CscCoef2(pub u32);
    impl CscCoef2 {
        #[doc = "Two's compliment Green U/Cb multiplier coefficient C3. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
        #[inline(always)]
        pub const fn c3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Green U/Cb multiplier coefficient C3. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
        #[inline(always)]
        pub fn set_c3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Two's compliment Green V/Cr multiplier coefficient C2. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
        #[inline(always)]
        pub const fn c2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Green V/Cr multiplier coefficient C2. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
        #[inline(always)]
        pub fn set_c2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for CscCoef2 {
        #[inline(always)]
        fn default() -> CscCoef2 {
            CscCoef2(0)
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Polarity of HSYNC 0b - HSYNC signal active HIGH 1b - HSYNC signal active LOW."]
        #[inline(always)]
        pub const fn inv_hsync(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Polarity of HSYNC 0b - HSYNC signal active HIGH 1b - HSYNC signal active LOW."]
        #[inline(always)]
        pub fn set_inv_hsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Polarity of VSYNC 0b - VSYNC signal active HIGH 1b - VSYNC signal active LOW."]
        #[inline(always)]
        pub const fn inv_vsync(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Polarity of VSYNC 0b - VSYNC signal active HIGH 1b - VSYNC signal active LOW."]
        #[inline(always)]
        pub fn set_inv_vsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Polarity of HREF 0b - HREF signal active HIGH, indicating active pixel data 1b - HREF signal active LOW."]
        #[inline(always)]
        pub const fn inv_href(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Polarity of HREF 0b - HREF signal active HIGH, indicating active pixel data 1b - HREF signal active LOW."]
        #[inline(always)]
        pub fn set_inv_href(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Polarity change of Pixel Clock. 0b - LCDC outputs data on the rising edge, and Display samples data on the falling edge 1b - LCDC outputs data on the falling edge, Display samples data on the rising edge."]
        #[inline(always)]
        pub const fn inv_pxclk(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Polarity change of Pixel Clock. 0b - LCDC outputs data on the rising edge, and Display samples data on the falling edge 1b - LCDC outputs data on the falling edge, Display samples data on the rising edge."]
        #[inline(always)]
        pub fn set_inv_pxclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Indicates if value at the output (pixel data output) needs to be negated. 0b - Output is to remain same as the data inside memory 1b - Output to be negated from the data inside memory."]
        #[inline(always)]
        pub const fn inv_pxdata(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates if value at the output (pixel data output) needs to be negated. 0b - Output is to remain same as the data inside memory 1b - Output to be negated from the data inside memory."]
        #[inline(always)]
        pub fn set_inv_pxdata(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ARQOS for bus fabric arbitration."]
        #[inline(always)]
        pub const fn arqos(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "ARQOS for bus fabric arbitration."]
        #[inline(always)]
        pub fn set_arqos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "background color for clear mode when the alpha channel is 0."]
        #[inline(always)]
        pub const fn bgdcl4clr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "background color for clear mode when the alpha channel is 0."]
        #[inline(always)]
        pub fn set_bgdcl4clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "LCDIF operating mode. 00b - Normal mode. Panel content controlled by layer configuration. 01b - Test Mode1.(BGND Color Display) 10b - Test Mode2.(Column Color Bar) 11b - Test Mode3.(Row Color Bar)."]
        #[inline(always)]
        pub const fn disp_mode(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "LCDIF operating mode. 00b - Normal mode. Panel content controlled by layer configuration. 01b - Test Mode1.(BGND Color Display) 10b - Test Mode2.(Column Color Bar) 11b - Test Mode3.(Row Color Bar)."]
        #[inline(always)]
        pub fn set_disp_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "LCDIF line output order. 000b - RGB. 001b - RBG. 010b - GBR. 011b - GRB. 100b - BRG. 101b - BGR."]
        #[inline(always)]
        pub const fn line_pattern(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x07;
            val as u8
        }
        #[doc = "LCDIF line output order. 000b - RGB. 001b - RBG. 010b - GBR. 011b - GRB. 100b - BRG. 101b - BGR."]
        #[inline(always)]
        pub fn set_line_pattern(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 27usize)) | (((val as u32) & 0x07) << 27usize);
        }
        #[doc = "Display panel On/Off mode. 0b - Display Off. 1b - Display On. Display can be set off at any time, but it can only be set on after VS_BLANK status is asserted. So a good procedure to stop and turn on the display is: 1) clr VS_BLANK status 2) assert software reset 3) de-assert software reset 4) set display off 5) check VS_BLANK status until it is asserted, 6)reset the module, change settings 7) set display on."]
        #[inline(always)]
        pub const fn disp_on(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Display panel On/Off mode. 0b - Display Off. 1b - Display On. Display can be set off at any time, but it can only be set on after VS_BLANK status is asserted. So a good procedure to stop and turn on the display is: 1) clr VS_BLANK status 2) assert software reset 3) de-assert software reset 4) set display off 5) check VS_BLANK status until it is asserted, 6)reset the module, change settings 7) set display on."]
        #[inline(always)]
        pub fn set_disp_on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Software reset, high active. When write 1 ,all internal logical will be reset. 0b - No action 1b - All LCDC internal registers are forced into their reset state. Interface registers are not affected."]
        #[inline(always)]
        pub const fn sw_rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset, high active. When write 1 ,all internal logical will be reset. 0b - No action 1b - All LCDC internal registers are forced into their reset state. Interface registers are not affected."]
        #[inline(always)]
        pub fn set_sw_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "Display Window Size Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DispWnSize(pub u32);
    impl DispWnSize {
        #[doc = "Sets the display size horizontal resolution in pixels."]
        #[inline(always)]
        pub const fn x(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Sets the display size horizontal resolution in pixels."]
        #[inline(always)]
        pub fn set_x(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Sets the display size vertical resolution in pixels."]
        #[inline(always)]
        pub const fn y(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Sets the display size vertical resolution in pixels."]
        #[inline(always)]
        pub fn set_y(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for DispWnSize {
        #[inline(always)]
        fn default() -> DispWnSize {
            DispWnSize(0)
        }
    }
    #[doc = "DMA Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaSt(pub u32);
    impl DmaSt {
        #[doc = "Plane n frame 0 dma done. W1C."]
        #[inline(always)]
        pub const fn dma0_done(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Plane n frame 0 dma done. W1C."]
        #[inline(always)]
        pub fn set_dma0_done(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Plane n frame 1 dma done. W1C."]
        #[inline(always)]
        pub const fn dma1_done(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Plane n frame 1 dma done. W1C."]
        #[inline(always)]
        pub fn set_dma1_done(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "plane n axi error. W1C."]
        #[inline(always)]
        pub const fn dma_err(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "plane n axi error. W1C."]
        #[inline(always)]
        pub fn set_dma_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for DmaSt {
        #[inline(always)]
        fn default() -> DmaSt {
            DmaSt(0)
        }
    }
    #[doc = "HSYNC Config Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HsyncPara(pub u32);
    impl HsyncPara {
        #[doc = "HSYNC active pulse width (in pixel clock cycles). Pulse width has a minimum value of 1."]
        #[inline(always)]
        pub const fn pw(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HSYNC active pulse width (in pixel clock cycles). Pulse width has a minimum value of 1."]
        #[inline(always)]
        pub fn set_pw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HSYNC back-porch pulse width (in pixel clock cycles). If zero, indicates no back-porch for HSYNC."]
        #[inline(always)]
        pub const fn bp(&self) -> u16 {
            let val = (self.0 >> 11usize) & 0x01ff;
            val as u16
        }
        #[doc = "HSYNC back-porch pulse width (in pixel clock cycles). If zero, indicates no back-porch for HSYNC."]
        #[inline(always)]
        pub fn set_bp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 11usize)) | (((val as u32) & 0x01ff) << 11usize);
        }
        #[doc = "HSYNC front-porch pulse width (in pixel clock cycles). If zero, indicates no front-porch for HSYNC."]
        #[inline(always)]
        pub const fn fp(&self) -> u16 {
            let val = (self.0 >> 22usize) & 0x01ff;
            val as u16
        }
        #[doc = "HSYNC front-porch pulse width (in pixel clock cycles). If zero, indicates no front-porch for HSYNC."]
        #[inline(always)]
        pub fn set_fp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 22usize)) | (((val as u32) & 0x01ff) << 22usize);
        }
    }
    impl Default for HsyncPara {
        #[inline(always)]
        fn default() -> HsyncPara {
            HsyncPara(0)
        }
    }
    #[doc = "Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "Interrupt enable for end of sof."]
        #[inline(always)]
        pub const fn vsync(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable for end of sof."]
        #[inline(always)]
        pub fn set_vsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Interrupt enable for underrun."]
        #[inline(always)]
        pub const fn underrun(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable for underrun."]
        #[inline(always)]
        pub fn set_underrun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Interrupt enable for start of sof."]
        #[inline(always)]
        pub const fn vs_blank(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable for start of sof."]
        #[inline(always)]
        pub fn set_vs_blank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Asserted when the output buffer urgent underrun condition encountered."]
        #[inline(always)]
        pub const fn urgent_underrun(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted when the output buffer urgent underrun condition encountered."]
        #[inline(always)]
        pub fn set_urgent_underrun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Interrupt enable for DMA done."]
        #[inline(always)]
        pub const fn dma_done(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Interrupt enable for DMA done."]
        #[inline(always)]
        pub fn set_dma_done(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Interrupt enable for DMA error."]
        #[inline(always)]
        pub const fn dma_err(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Interrupt enable for DMA error."]
        #[inline(always)]
        pub fn set_dma_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    #[doc = "Layer Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Layctrl(pub u32);
    impl Layctrl {
        #[doc = "Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
        #[inline(always)]
        pub const fn ab_mode(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
        #[inline(always)]
        pub fn set_ab_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\]
is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\]
is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\]
is used to scale the alpha value from previous pipeline Others: Reserved."]
        #[inline(always)]
        pub const fn inalpha_op(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\]
is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\]
is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\]
is used to scale the alpha value from previous pipeline Others: Reserved."]
        #[inline(always)]
        pub fn set_inalpha_op(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\]
is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\]
is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\]
is used to scale the alpha value from the data stream Others: Reserved."]
        #[inline(always)]
        pub const fn localpha_op(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\]
is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\]
is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\]
is used to scale the alpha value from the data stream Others: Reserved."]
        #[inline(always)]
        pub fn set_localpha_op(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), the low byte contains the full R component. 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\]
1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4."]
        #[inline(always)]
        pub const fn pixformat(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x0f;
            val as u8
        }
        #[doc = "Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), the low byte contains the full R component. 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\]
1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4."]
        #[inline(always)]
        pub fn set_pixformat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
        }
        #[doc = "The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)."]
        #[inline(always)]
        pub const fn yuv_format(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)."]
        #[inline(always)]
        pub fn set_yuv_format(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
        #[inline(always)]
        pub const fn shadow_load_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
        #[inline(always)]
        pub fn set_shadow_load_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order."]
        #[inline(always)]
        pub const fn pack_dir(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order."]
        #[inline(always)]
        pub fn set_pack_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Layctrl {
        #[inline(always)]
        fn default() -> Layctrl {
            Layctrl(0)
        }
    }
    #[doc = "Layer Position Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Laypos(pub u32);
    impl Laypos {
        #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
        #[inline(always)]
        pub const fn x(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
        #[inline(always)]
        pub fn set_x(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
        #[inline(always)]
        pub const fn y(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
        #[inline(always)]
        pub fn set_y(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Laypos {
        #[inline(always)]
        fn default() -> Laypos {
            Laypos(0)
        }
    }
    #[doc = "Layer Size Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Laysize(pub u32);
    impl Laysize {
        #[doc = "Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
        #[inline(always)]
        pub const fn width(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
        #[inline(always)]
        pub fn set_width(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Height of the layer in pixels."]
        #[inline(always)]
        pub const fn height(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Height of the layer in pixels."]
        #[inline(always)]
        pub fn set_height(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Laysize {
        #[inline(always)]
        fn default() -> Laysize {
            Laysize(0)
        }
    }
    #[doc = "Layer Bus Config Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Linecfg(pub u32);
    impl Linecfg {
        #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundary."]
        #[inline(always)]
        pub const fn pitch(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundary."]
        #[inline(always)]
        pub fn set_pitch(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "the number of outstanding axi read transactions. If zero, it means max 8."]
        #[inline(always)]
        pub const fn max_ot(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "the number of outstanding axi read transactions. If zero, it means max 8."]
        #[inline(always)]
        pub fn set_max_ot(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes."]
        #[inline(always)]
        pub const fn mpt_size(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[doc = "Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes."]
        #[inline(always)]
        pub fn set_mpt_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Linecfg {
        #[inline(always)]
        fn default() -> Linecfg {
            Linecfg(0)
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct St(pub u32);
    impl St {
        #[doc = "Asserted when in vertical blanking period. At the end of VSYNC."]
        #[inline(always)]
        pub const fn vsync(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted when in vertical blanking period. At the end of VSYNC."]
        #[inline(always)]
        pub fn set_vsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Asserted when the output buffer underrun condition encountered."]
        #[inline(always)]
        pub const fn underrun(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted when the output buffer underrun condition encountered."]
        #[inline(always)]
        pub fn set_underrun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Asserted when in vertical blanking period. At the start of VSYNC."]
        #[inline(always)]
        pub const fn vs_blank(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted when in vertical blanking period. At the start of VSYNC."]
        #[inline(always)]
        pub fn set_vs_blank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Asserted when the output buffer urgent underrun condition encountered."]
        #[inline(always)]
        pub const fn urgent_underrun(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted when the output buffer urgent underrun condition encountered."]
        #[inline(always)]
        pub fn set_urgent_underrun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for St {
        #[inline(always)]
        fn default() -> St {
            St(0)
        }
    }
    #[doc = "Layer Buffer Pointer Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Start0(pub u32);
    impl Start0 {
        #[doc = "Input buffer Start address 0."]
        #[inline(always)]
        pub const fn addr0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Input buffer Start address 0."]
        #[inline(always)]
        pub fn set_addr0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Start0 {
        #[inline(always)]
        fn default() -> Start0 {
            Start0(0)
        }
    }
    #[doc = "TX FIFO Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txfifo(pub u32);
    impl Txfifo {
        #[doc = "Threshold to start the lcd raster (0--0x7F)."]
        #[inline(always)]
        pub const fn thrsh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Threshold to start the lcd raster (0--0x7F)."]
        #[inline(always)]
        pub fn set_thrsh(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Txfifo {
        #[inline(always)]
        fn default() -> Txfifo {
            Txfifo(0)
        }
    }
    #[doc = "VSYNC Config Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VsyncPara(pub u32);
    impl VsyncPara {
        #[doc = "VSYNC active pulse width (in horizontal line cycles). Pulse width has a minimum value of 1."]
        #[inline(always)]
        pub const fn pw(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "VSYNC active pulse width (in horizontal line cycles). Pulse width has a minimum value of 1."]
        #[inline(always)]
        pub fn set_pw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "VSYNC back-porch pulse width (in horizontal line cycles). If zero, means no back-porch for VSYNC."]
        #[inline(always)]
        pub const fn bp(&self) -> u16 {
            let val = (self.0 >> 11usize) & 0x01ff;
            val as u16
        }
        #[doc = "VSYNC back-porch pulse width (in horizontal line cycles). If zero, means no back-porch for VSYNC."]
        #[inline(always)]
        pub fn set_bp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 11usize)) | (((val as u32) & 0x01ff) << 11usize);
        }
        #[doc = "VSYNC front-porch pulse width (in horizontal line cycles). If zero, means no front-porch for VSYNC."]
        #[inline(always)]
        pub const fn fp(&self) -> u16 {
            let val = (self.0 >> 22usize) & 0x01ff;
            val as u16
        }
        #[doc = "VSYNC front-porch pulse width (in horizontal line cycles). If zero, means no front-porch for VSYNC."]
        #[inline(always)]
        pub fn set_fp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 22usize)) | (((val as u32) & 0x01ff) << 22usize);
        }
    }
    impl Default for VsyncPara {
        #[inline(always)]
        fn default() -> VsyncPara {
            VsyncPara(0)
        }
    }
}
