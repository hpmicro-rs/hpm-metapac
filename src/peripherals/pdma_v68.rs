#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutPs {
    ptr: *mut u8,
}
unsafe impl Send for OutPs {}
unsafe impl Sync for OutPs {}
impl OutPs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Layer Upper Left Corner Register."]
    #[inline(always)]
    pub const fn ulc(self) -> crate::common::Reg<regs::Ulc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Layer Lower Right Corner Register."]
    #[inline(always)]
    pub const fn lrc(self) -> crate::common::Reg<regs::Lrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "PDMA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdma {
    ptr: *mut u8,
}
unsafe impl Send for Pdma {}
unsafe impl Sync for Pdma {}
impl Pdma {
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
    pub const fn ctrl(self) -> crate::common::Reg<regs::PdmaCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Out Layer Control Register."]
    #[inline(always)]
    pub const fn out_ctrl(self) -> crate::common::Reg<regs::OutCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Output buffer address."]
    #[inline(always)]
    pub const fn out_buf(self) -> crate::common::Reg<regs::OutBuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Outlayer Pitch Register."]
    #[inline(always)]
    pub const fn out_pitch(self) -> crate::common::Reg<regs::OutPitch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Output Lower Right Corner Register."]
    #[inline(always)]
    pub const fn out_lrc(self) -> crate::common::Reg<regs::OutLrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn out_ps(self, n: usize) -> OutPs {
        assert!(n < 2usize);
        unsafe { OutPs::from_ptr(self.ptr.add(0x1cusize + n * 8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ps(self, n: usize) -> Ps {
        assert!(n < 2usize);
        unsafe { Ps::from_ptr(self.ptr.add(0x30usize + n * 48usize) as _) }
    }
    #[doc = "YUV2RGB coefficients register 0."]
    #[inline(always)]
    pub const fn yuv2rgb_coef0(self) -> crate::common::Reg<regs::Yuv2rgbCoef0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "YUV2RGB coefficients register 1."]
    #[inline(always)]
    pub const fn yuv2rgb_coef1(self) -> crate::common::Reg<regs::Yuv2rgbCoef1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "YUV2RGB coefficients register 2."]
    #[inline(always)]
    pub const fn yuv2rgb_coef2(self) -> crate::common::Reg<regs::Yuv2rgbCoef2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "RGB2YUV coefficients register 0."]
    #[inline(always)]
    pub const fn rgb2yuv_coef0(self) -> crate::common::Reg<regs::Rgb2yuvCoef0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "RGB2YUV coefficients register 1."]
    #[inline(always)]
    pub const fn rgb2yuv_coef1(self) -> crate::common::Reg<regs::Rgb2yuvCoef1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "RGB2YUV coefficients register 2."]
    #[inline(always)]
    pub const fn rgb2yuv_coef2(self) -> crate::common::Reg<regs::Rgb2yuvCoef2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "RGB2YUV coefficients register 3."]
    #[inline(always)]
    pub const fn rgb2yuv_coef3(self) -> crate::common::Reg<regs::Rgb2yuvCoef3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "RGB2YUV coefficients register 4."]
    #[inline(always)]
    pub const fn rgb2yuv_coef4(self) -> crate::common::Reg<regs::Rgb2yuvCoef4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ps {
    ptr: *mut u8,
}
unsafe impl Send for Ps {}
unsafe impl Sync for Ps {}
impl Ps {
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
    pub const fn ctrl(self) -> crate::common::Reg<regs::PsCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Layer data buffer address."]
    #[inline(always)]
    pub const fn buf(self) -> crate::common::Reg<regs::Buf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Layer data pitch register."]
    #[inline(always)]
    pub const fn pitch(self) -> crate::common::Reg<regs::Pitch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Layer background color register."]
    #[inline(always)]
    pub const fn bkgd(self) -> crate::common::Reg<regs::Bkgd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Layer scale register."]
    #[inline(always)]
    pub const fn scale(self) -> crate::common::Reg<regs::Scale, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Layer offset register."]
    #[inline(always)]
    pub const fn offset(self) -> crate::common::Reg<regs::Offset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Layer low color key register."]
    #[inline(always)]
    pub const fn clrkey_low(self) -> crate::common::Reg<regs::ClrkeyLow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Layer high color key register."]
    #[inline(always)]
    pub const fn clrkey_high(self) -> crate::common::Reg<regs::ClrkeyHigh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Layer original size register."]
    #[inline(always)]
    pub const fn org(self) -> crate::common::Reg<regs::Org, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "Layer background color register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bkgd(pub u32);
    impl Bkgd {
        #[doc = "Background color (in 32bpp format) for any pixels not within the scaled range of the picture, but within the buffer range specified by the PS ULC/LRC. The top 8-bit is the alpha channel."]
        #[inline(always)]
        pub const fn color(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Background color (in 32bpp format) for any pixels not within the scaled range of the picture, but within the buffer range specified by the PS ULC/LRC. The top 8-bit is the alpha channel."]
        #[inline(always)]
        pub fn set_color(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bkgd {
        #[inline(always)]
        fn default() -> Bkgd {
            Bkgd(0)
        }
    }
    #[doc = "Layer data buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buf(pub u32);
    impl Buf {
        #[doc = "Address pointer for the PS RGB or Y (luma) input buffer."]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Address pointer for the PS RGB or Y (luma) input buffer."]
        #[inline(always)]
        pub fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Buf {
        #[inline(always)]
        fn default() -> Buf {
            Buf(0)
        }
    }
    #[doc = "Layer high color key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClrkeyHigh(pub u32);
    impl ClrkeyHigh {
        #[doc = "High range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
        #[inline(always)]
        pub const fn limit(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "High range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
        #[inline(always)]
        pub fn set_limit(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for ClrkeyHigh {
        #[inline(always)]
        fn default() -> ClrkeyHigh {
            ClrkeyHigh(0)
        }
    }
    #[doc = "Layer low color key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClrkeyLow(pub u32);
    impl ClrkeyLow {
        #[doc = "Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
        #[inline(always)]
        pub const fn limit(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
        #[inline(always)]
        pub fn set_limit(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for ClrkeyLow {
        #[inline(always)]
        fn default() -> ClrkeyLow {
            ClrkeyLow(0)
        }
    }
    #[doc = "Layer Lower Right Corner Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lrc(pub u32);
    impl Lrc {
        #[doc = "This field indicates the lower right X-coordinate (in pixels) of the processed surface in the output frame buffer."]
        #[inline(always)]
        pub const fn x(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "This field indicates the lower right X-coordinate (in pixels) of the processed surface in the output frame buffer."]
        #[inline(always)]
        pub fn set_x(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "This field indicates the lower right Y-coordinate (in pixels) of the processed surface in the output frame buffer."]
        #[inline(always)]
        pub const fn y(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[doc = "This field indicates the lower right Y-coordinate (in pixels) of the processed surface in the output frame buffer."]
        #[inline(always)]
        pub fn set_y(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for Lrc {
        #[inline(always)]
        fn default() -> Lrc {
            Lrc(0)
        }
    }
    #[doc = "Layer offset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Offset(pub u32);
    impl Offset {
        #[doc = "This is a 12 bit fractional representation (0.####_####_####) of the X scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
        #[inline(always)]
        pub const fn x(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "This is a 12 bit fractional representation (0.####_####_####) of the X scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
        #[inline(always)]
        pub fn set_x(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "This is a 12 bit fractional representation (0.####_####_####) of the Y scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
        #[inline(always)]
        pub const fn y(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "This is a 12 bit fractional representation (0.####_####_####) of the Y scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
        #[inline(always)]
        pub fn set_y(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Offset {
        #[inline(always)]
        fn default() -> Offset {
            Offset(0)
        }
    }
    #[doc = "Layer original size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Org(pub u32);
    impl Org {
        #[doc = "The number of horizontal pixels of the original frame (not -1)."]
        #[inline(always)]
        pub const fn width(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "The number of horizontal pixels of the original frame (not -1)."]
        #[inline(always)]
        pub fn set_width(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "The number of vertical pixels of the original frame (not -1)."]
        #[inline(always)]
        pub const fn hight(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[doc = "The number of vertical pixels of the original frame (not -1)."]
        #[inline(always)]
        pub fn set_hight(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for Org {
        #[inline(always)]
        fn default() -> Org {
            Org(0)
        }
    }
    #[doc = "Output buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutBuf(pub u32);
    impl OutBuf {
        #[doc = "Current address pointer for the output frame buffer. The address can have any byte alignment. 64B alignment is recommended for optimal performance."]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Current address pointer for the output frame buffer. The address can have any byte alignment. 64B alignment is recommended for optimal performance."]
        #[inline(always)]
        pub fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OutBuf {
        #[inline(always)]
        fn default() -> OutBuf {
            OutBuf(0)
        }
    }
    #[doc = "Out Layer Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutCtrl(pub u32);
    impl OutCtrl {
        #[doc = "Output buffer format. 0x0 ARGB8888 - 32-bit pixles, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x12 UYVY1P422 - 16-bit pixels (1-plane , byte sequence as U0,Y0,V0,Y1)."]
        #[inline(always)]
        pub const fn format(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Output buffer format. 0x0 ARGB8888 - 32-bit pixles, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x12 UYVY1P422 - 16-bit pixels (1-plane , byte sequence as U0,Y0,V0,Y1)."]
        #[inline(always)]
        pub fn set_format(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Asserted to normalize the output color channels with alpha channels."]
        #[inline(always)]
        pub const fn norm_out(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to normalize the output color channels with alpha channels."]
        #[inline(always)]
        pub fn set_norm_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Alpha Blending Mode 0: SKBlendMode_Clear (If PS1_CTRL\\[BKGNDCL4CLR\\]
is asserted, use PS1_BKGRND color to fill the range determined by PS1, else fill the range determined by PS1 with zero); 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional belding mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
        #[inline(always)]
        pub const fn ablend_mode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Alpha Blending Mode 0: SKBlendMode_Clear (If PS1_CTRL\\[BKGNDCL4CLR\\]
is asserted, use PS1_BKGRND color to fill the range determined by PS1, else fill the range determined by PS1 with zero); 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional belding mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
        #[inline(always)]
        pub fn set_ablend_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "The usage of the SRCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the SRCALPHA\\[7:0\\]
is invalid, use the alpha value embedded in the stream 1: the SRCALPHA\\[7:0\\]
is used to override the alpha value embedded in the stream . (useful when the corresponding data stream has no alpha info) 2: the SRCALPHA\\[7:0\\]
is used to scale the alpha value embedded in the stream 3: don't multiply the color data with any alpha values for blender inputs."]
        #[inline(always)]
        pub const fn srcalpha_op(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "The usage of the SRCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the SRCALPHA\\[7:0\\]
is invalid, use the alpha value embedded in the stream 1: the SRCALPHA\\[7:0\\]
is used to override the alpha value embedded in the stream . (useful when the corresponding data stream has no alpha info) 2: the SRCALPHA\\[7:0\\]
is used to scale the alpha value embedded in the stream 3: don't multiply the color data with any alpha values for blender inputs."]
        #[inline(always)]
        pub fn set_srcalpha_op(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "The usage of the DSTALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the DSTALPHA\\[7:0\\]
is invalid, use the alpha value embedded in the stream 1: the DSTALPHA\\[7:0\\]
is used to override the alpha value embedded in the stream. (useful when the corresponding data stream has no alpha info) 2: the DSTALPHA\\[7:0\\]
is used to scale the alpha value embedded in the stream 3: don't multiply the color data with any alpha values for blender inputs."]
        #[inline(always)]
        pub const fn dstalpha_op(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "The usage of the DSTALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the DSTALPHA\\[7:0\\]
is invalid, use the alpha value embedded in the stream 1: the DSTALPHA\\[7:0\\]
is used to override the alpha value embedded in the stream. (useful when the corresponding data stream has no alpha info) 2: the DSTALPHA\\[7:0\\]
is used to scale the alpha value embedded in the stream 3: don't multiply the color data with any alpha values for blender inputs."]
        #[inline(always)]
        pub fn set_dstalpha_op(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "The source (P0) system ALPHA value."]
        #[inline(always)]
        pub const fn srcalpha(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "The source (P0) system ALPHA value."]
        #[inline(always)]
        pub fn set_srcalpha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "The destination (P1) system ALPHA value."]
        #[inline(always)]
        pub const fn dstalpha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "The destination (P1) system ALPHA value."]
        #[inline(always)]
        pub fn set_dstalpha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for OutCtrl {
        #[inline(always)]
        fn default() -> OutCtrl {
            OutCtrl(0)
        }
    }
    #[doc = "Output Lower Right Corner Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutLrc(pub u32);
    impl OutLrc {
        #[doc = "This field indicates the lower right X-coordinate (in pixels) of the output frame buffer. Should be the width of the output image size."]
        #[inline(always)]
        pub const fn x(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "This field indicates the lower right X-coordinate (in pixels) of the output frame buffer. Should be the width of the output image size."]
        #[inline(always)]
        pub fn set_x(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "This field indicates the lower right Y-coordinate (in pixels) of the output frame buffer. The value is the height of the output image size."]
        #[inline(always)]
        pub const fn y(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[doc = "This field indicates the lower right Y-coordinate (in pixels) of the output frame buffer. The value is the height of the output image size."]
        #[inline(always)]
        pub fn set_y(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for OutLrc {
        #[inline(always)]
        fn default() -> OutLrc {
            OutLrc(0)
        }
    }
    #[doc = "Outlayer Pitch Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutPitch(pub u32);
    impl OutPitch {
        #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
        #[inline(always)]
        pub const fn bytelen(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
        #[inline(always)]
        pub fn set_bytelen(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for OutPitch {
        #[inline(always)]
        fn default() -> OutPitch {
            OutPitch(0)
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdmaCtrl(pub u32);
    impl PdmaCtrl {
        #[doc = "1b - Enabled."]
        #[inline(always)]
        pub const fn pdma_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1b - Enabled."]
        #[inline(always)]
        pub fn set_pdma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Software Reset. Write 1 to clear PDMA internal logic. Write 0 to exit software reset mode."]
        #[inline(always)]
        pub const fn pdma_sftrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Software Reset. Write 1 to clear PDMA internal logic. Write 0 to exit software reset mode."]
        #[inline(always)]
        pub fn set_pdma_sftrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Plane 0 Enable."]
        #[inline(always)]
        pub const fn p0_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Plane 0 Enable."]
        #[inline(always)]
        pub fn set_p0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Plane 1 Enable."]
        #[inline(always)]
        pub const fn p1_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Plane 1 Enable."]
        #[inline(always)]
        pub fn set_p1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Asserted when the Block Size is 16x16, else 8x8."]
        #[inline(always)]
        pub const fn bs16(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted when the Block Size is 16x16, else 8x8."]
        #[inline(always)]
        pub fn set_bs16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable normal interrupt."]
        #[inline(always)]
        pub const fn irq_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable normal interrupt."]
        #[inline(always)]
        pub fn set_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on."]
        #[inline(always)]
        pub const fn clkgate(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on."]
        #[inline(always)]
        pub fn set_clkgate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable interrupt of PDMA_DONE."]
        #[inline(always)]
        pub const fn pdma_done_irq_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable interrupt of PDMA_DONE."]
        #[inline(always)]
        pub fn set_pdma_done_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable interrupt of AXI bus error."]
        #[inline(always)]
        pub const fn axierr_irq_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable interrupt of AXI bus error."]
        #[inline(always)]
        pub fn set_axierr_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Decide the byte sequence of the 32-bit output word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}."]
        #[inline(always)]
        pub const fn pack_dir(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Decide the byte sequence of the 32-bit output word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}."]
        #[inline(always)]
        pub fn set_pack_dir(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "QoS for AXI write bus."]
        #[inline(always)]
        pub const fn awqos(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x0f;
            val as u8
        }
        #[doc = "QoS for AXI write bus."]
        #[inline(always)]
        pub fn set_awqos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 15usize)) | (((val as u32) & 0x0f) << 15usize);
        }
        #[doc = "QoS for AXI read bus."]
        #[inline(always)]
        pub const fn arqos(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x0f;
            val as u8
        }
        #[doc = "QoS for AXI read bus."]
        #[inline(always)]
        pub fn set_arqos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
        }
    }
    impl Default for PdmaCtrl {
        #[inline(always)]
        fn default() -> PdmaCtrl {
            PdmaCtrl(0)
        }
    }
    #[doc = "Layer data pitch register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pitch(pub u32);
    impl Pitch {
        #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
        #[inline(always)]
        pub const fn bytelen(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
        #[inline(always)]
        pub fn set_bytelen(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Pitch {
        #[inline(always)]
        fn default() -> Pitch {
            Pitch(0)
        }
    }
    #[doc = "Layer Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PsCtrl(pub u32);
    impl PsCtrl {
        #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 16 of this register. 0x0 ARGB888 - 32-bit pixels, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x13 YUYV1P422 - 16-bit pixels (1-plane byte sequence Y0,U0,Y1,V0 interleaved bytes)."]
        #[inline(always)]
        pub const fn format(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 16 of this register. 0x0 ARGB888 - 32-bit pixels, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x13 YUYV1P422 - 16-bit pixels (1-plane byte sequence Y0,U0,Y1,V0 interleaved bytes)."]
        #[inline(always)]
        pub fn set_format(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Swap bytes in half-words. For each 16 bit half-word, the two bytes will be swapped."]
        #[inline(always)]
        pub const fn hw_byte_swap(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Swap bytes in half-words. For each 16 bit half-word, the two bytes will be swapped."]
        #[inline(always)]
        pub fn set_hw_byte_swap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Horizontal pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECX2 - Decimate PS by 2. 0x2 DECX4 - Decimate PS by 4. 0x3 DECX8 - Decimate PS by 8."]
        #[inline(always)]
        pub const fn decx(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x03;
            val as u8
        }
        #[doc = "Horizontal pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECX2 - Decimate PS by 2. 0x2 DECX4 - Decimate PS by 4. 0x3 DECX8 - Decimate PS by 8."]
        #[inline(always)]
        pub fn set_decx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
        }
        #[doc = "Verticle pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECY2 - Decimate PS by 2. 0x2 DECY4 - Decimate PS by 4. 0x3 DECY8 - Decimate PS by 8."]
        #[inline(always)]
        pub const fn decy(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "Verticle pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECY2 - Decimate PS by 2. 0x2 DECY4 - Decimate PS by 4. 0x3 DECY8 - Decimate PS by 8."]
        #[inline(always)]
        pub fn set_decy(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "Indicates the clockwise rotation to be applied at the input buffer. The rotation effect is defined as occurring after the FLIP_X and FLIP_Y permutation. 0x0 ROT_0 0x1 ROT_90 0x2 ROT_180 0x3 ROT_270."]
        #[inline(always)]
        pub const fn rotate(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "Indicates the clockwise rotation to be applied at the input buffer. The rotation effect is defined as occurring after the FLIP_X and FLIP_Y permutation. 0x0 ROT_0 0x1 ROT_90 0x2 ROT_180 0x3 ROT_270."]
        #[inline(always)]
        pub fn set_rotate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "Indicates that the input should be flipped horizontally (effect applied before rotation)."]
        #[inline(always)]
        pub const fn hflip(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the input should be flipped horizontally (effect applied before rotation)."]
        #[inline(always)]
        pub fn set_hflip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Indicates that the input should be flipped vertically (effect applied before rotation)."]
        #[inline(always)]
        pub const fn vflip(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Indicates that the input should be flipped vertically (effect applied before rotation)."]
        #[inline(always)]
        pub fn set_vflip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Asserted to bypass the CSC stage."]
        #[inline(always)]
        pub const fn bypass(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to bypass the CSC stage."]
        #[inline(always)]
        pub fn set_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "YCbCr mode or YUV mode."]
        #[inline(always)]
        pub const fn ycbcr_mode(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "YCbCr mode or YUV mode."]
        #[inline(always)]
        pub fn set_ycbcr_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Enable to use background color for clear area."]
        #[inline(always)]
        pub const fn bkgcl4clr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Enable to use background color for clear area."]
        #[inline(always)]
        pub fn set_bkgcl4clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}."]
        #[inline(always)]
        pub const fn pack_dir(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}."]
        #[inline(always)]
        pub fn set_pack_dir(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "Swap bit\\[31:24\\]
and bit \\[15:8\\]
before pack_dir operation."]
        #[inline(always)]
        pub const fn inb13_swap(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Swap bit\\[31:24\\]
and bit \\[15:8\\]
before pack_dir operation."]
        #[inline(always)]
        pub fn set_inb13_swap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "1: For those pixels that are this plane-only, use the colcor values and alpha values directly as blender output for un-normalized outputs configurations. 0: For those pixels that are this plane-only, the operations are determined by other operation configurations."]
        #[inline(always)]
        pub const fn pl_only_blendop(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "1: For those pixels that are this plane-only, use the colcor values and alpha values directly as blender output for un-normalized outputs configurations. 0: For those pixels that are this plane-only, the operations are determined by other operation configurations."]
        #[inline(always)]
        pub fn set_pl_only_blendop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for PsCtrl {
        #[inline(always)]
        fn default() -> PsCtrl {
            PsCtrl(0)
        }
    }
    #[doc = "RGB2YUV coefficients register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgb2yuvCoef0(pub u32);
    impl Rgb2yuvCoef0 {
        #[doc = "CSC parameters Y_OFFSET."]
        #[inline(always)]
        pub const fn y_offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "CSC parameters Y_OFFSET."]
        #[inline(always)]
        pub fn set_y_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "CSC parameters UV_OFFSET."]
        #[inline(always)]
        pub const fn uv_offset(&self) -> u16 {
            let val = (self.0 >> 9usize) & 0x01ff;
            val as u16
        }
        #[doc = "CSC parameters UV_OFFSET."]
        #[inline(always)]
        pub fn set_uv_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 9usize)) | (((val as u32) & 0x01ff) << 9usize);
        }
        #[doc = "CSC parameters C0."]
        #[inline(always)]
        pub const fn c0(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C0."]
        #[inline(always)]
        pub fn set_c0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
        }
        #[doc = "Asserted to enable this RGB2YUV CSC stage."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enable this RGB2YUV CSC stage."]
        #[inline(always)]
        pub fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Asserted to use YCrCb mode."]
        #[inline(always)]
        pub const fn ycbcr_mode(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to use YCrCb mode."]
        #[inline(always)]
        pub fn set_ycbcr_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rgb2yuvCoef0 {
        #[inline(always)]
        fn default() -> Rgb2yuvCoef0 {
            Rgb2yuvCoef0(0)
        }
    }
    #[doc = "RGB2YUV coefficients register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgb2yuvCoef1(pub u32);
    impl Rgb2yuvCoef1 {
        #[doc = "CSC parameters C4."]
        #[inline(always)]
        pub const fn c4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C4."]
        #[inline(always)]
        pub fn set_c4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "CSC parameters C1."]
        #[inline(always)]
        pub const fn c1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C1."]
        #[inline(always)]
        pub fn set_c1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for Rgb2yuvCoef1 {
        #[inline(always)]
        fn default() -> Rgb2yuvCoef1 {
            Rgb2yuvCoef1(0)
        }
    }
    #[doc = "RGB2YUV coefficients register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgb2yuvCoef2(pub u32);
    impl Rgb2yuvCoef2 {
        #[doc = "CSC parameters C3."]
        #[inline(always)]
        pub const fn c3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C3."]
        #[inline(always)]
        pub fn set_c3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "CSC parameters C2."]
        #[inline(always)]
        pub const fn c2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C2."]
        #[inline(always)]
        pub fn set_c2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for Rgb2yuvCoef2 {
        #[inline(always)]
        fn default() -> Rgb2yuvCoef2 {
            Rgb2yuvCoef2(0)
        }
    }
    #[doc = "RGB2YUV coefficients register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgb2yuvCoef3(pub u32);
    impl Rgb2yuvCoef3 {
        #[doc = "CSC parameters C5."]
        #[inline(always)]
        pub const fn c5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C5."]
        #[inline(always)]
        pub fn set_c5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "CSC parameters C6."]
        #[inline(always)]
        pub const fn c6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C6."]
        #[inline(always)]
        pub fn set_c6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for Rgb2yuvCoef3 {
        #[inline(always)]
        fn default() -> Rgb2yuvCoef3 {
            Rgb2yuvCoef3(0)
        }
    }
    #[doc = "RGB2YUV coefficients register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rgb2yuvCoef4(pub u32);
    impl Rgb2yuvCoef4 {
        #[doc = "CSC parameters C7."]
        #[inline(always)]
        pub const fn c7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C7."]
        #[inline(always)]
        pub fn set_c7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "CSC parameters C8."]
        #[inline(always)]
        pub const fn c8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "CSC parameters C8."]
        #[inline(always)]
        pub fn set_c8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for Rgb2yuvCoef4 {
        #[inline(always)]
        fn default() -> Rgb2yuvCoef4 {
            Rgb2yuvCoef4(0)
        }
    }
    #[doc = "Layer scale register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scale(pub u32);
    impl Scale {
        #[doc = "This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the Y scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
        #[inline(always)]
        pub const fn x(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the Y scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
        #[inline(always)]
        pub fn set_x(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
        #[doc = "This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the X scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
        #[inline(always)]
        pub const fn y(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the X scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
        #[inline(always)]
        pub fn set_y(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
    }
    impl Default for Scale {
        #[inline(always)]
        fn default() -> Scale {
            Scale(0)
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stat(pub u32);
    impl Stat {
        #[doc = "Asserted to indicate a IRQ event."]
        #[inline(always)]
        pub const fn irq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to indicate a IRQ event."]
        #[inline(always)]
        pub fn set_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AXI0 read err."]
        #[inline(always)]
        pub const fn axi_0_read_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "AXI0 read err."]
        #[inline(always)]
        pub fn set_axi_0_read_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "AXI1 read err."]
        #[inline(always)]
        pub const fn axi_1_read_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AXI1 read err."]
        #[inline(always)]
        pub fn set_axi_1_read_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "AXI0 write err."]
        #[inline(always)]
        pub const fn axi_0_write_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AXI0 write err."]
        #[inline(always)]
        pub fn set_axi_0_write_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AXI error ID."]
        #[inline(always)]
        pub const fn axi_err_id(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "AXI error ID."]
        #[inline(always)]
        pub fn set_axi_err_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "PDMA one image done."]
        #[inline(always)]
        pub const fn pdma_done(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PDMA one image done."]
        #[inline(always)]
        pub fn set_pdma_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "X block that is processing."]
        #[inline(always)]
        pub const fn blockx(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "X block that is processing."]
        #[inline(always)]
        pub fn set_blockx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Y block that is processing."]
        #[inline(always)]
        pub const fn blocky(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Y block that is processing."]
        #[inline(always)]
        pub fn set_blocky(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Stat {
        #[inline(always)]
        fn default() -> Stat {
            Stat(0)
        }
    }
    #[doc = "Layer Upper Left Corner Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ulc(pub u32);
    impl Ulc {
        #[doc = "This field indicates the upper left X-coordinate (in pixels) of the processed surface in the output frame buffer."]
        #[inline(always)]
        pub const fn x(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "This field indicates the upper left X-coordinate (in pixels) of the processed surface in the output frame buffer."]
        #[inline(always)]
        pub fn set_x(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[doc = "This field indicates the upper left Y-coordinate (in pixels) of the processed surface in the output frame buffer."]
        #[inline(always)]
        pub const fn y(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[doc = "This field indicates the upper left Y-coordinate (in pixels) of the processed surface in the output frame buffer."]
        #[inline(always)]
        pub fn set_y(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for Ulc {
        #[inline(always)]
        fn default() -> Ulc {
            Ulc(0)
        }
    }
    #[doc = "YUV2RGB coefficients register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Yuv2rgbCoef0(pub u32);
    impl Yuv2rgbCoef0 {
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
    }
    impl Default for Yuv2rgbCoef0 {
        #[inline(always)]
        fn default() -> Yuv2rgbCoef0 {
            Yuv2rgbCoef0(0)
        }
    }
    #[doc = "YUV2RGB coefficients register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Yuv2rgbCoef1(pub u32);
    impl Yuv2rgbCoef1 {
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
    impl Default for Yuv2rgbCoef1 {
        #[inline(always)]
        fn default() -> Yuv2rgbCoef1 {
            Yuv2rgbCoef1(0)
        }
    }
    #[doc = "YUV2RGB coefficients register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Yuv2rgbCoef2(pub u32);
    impl Yuv2rgbCoef2 {
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
    impl Default for Yuv2rgbCoef2 {
        #[inline(always)]
        fn default() -> Yuv2rgbCoef2 {
            Yuv2rgbCoef2(0)
        }
    }
}
