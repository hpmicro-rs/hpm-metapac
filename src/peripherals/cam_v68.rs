#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "CAM0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cam {
    ptr: *mut u8,
}
unsafe impl Send for Cam {}
unsafe impl Sync for Cam {}
impl Cam {
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
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Control 2 Register."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sta(self) -> crate::common::Reg<regs::Sta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Pixel DMA Frame Buffer 1 Address."]
    #[inline(always)]
    pub const fn dmasa_fb1(self) -> crate::common::Reg<regs::DmasaFb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Pixel DMA Frame Buffer 2 Address."]
    #[inline(always)]
    pub const fn dmasa_fb2(self) -> crate::common::Reg<regs::DmasaFb2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Buffer Parameters Register."]
    #[inline(always)]
    pub const fn buf_para(self) -> crate::common::Reg<regs::BufPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Ideal Image Size Register."]
    #[inline(always)]
    pub const fn ideal_wn_size(self) -> crate::common::Reg<regs::IdealWnSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Control CR18 Register."]
    #[inline(always)]
    pub const fn cr18(self) -> crate::common::Reg<regs::Cr18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Pixel UV DMA Frame Buffer 1 Address."]
    #[inline(always)]
    pub const fn dmasa_uv1(self) -> crate::common::Reg<regs::DmasaUv1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Pixel UV DMA Frame Buffer 2 Address."]
    #[inline(always)]
    pub const fn dmasa_uv2(self) -> crate::common::Reg<regs::DmasaUv2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Control CR20 Register."]
    #[inline(always)]
    pub const fn cr20(self) -> crate::common::Reg<regs::Cr20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Color Space Conversion Config Register 0."]
    #[inline(always)]
    pub const fn csc_coef0(self) -> crate::common::Reg<regs::CscCoef0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Color Space Conversion Config Register 1."]
    #[inline(always)]
    pub const fn csc_coef1(self) -> crate::common::Reg<regs::CscCoef1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Color Space Conversion Config Register 2."]
    #[inline(always)]
    pub const fn csc_coef2(self) -> crate::common::Reg<regs::CscCoef2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "Low Color Key Register."]
    #[inline(always)]
    pub const fn clrkey_low(self) -> crate::common::Reg<regs::ClrkeyLow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "High Color Key Register."]
    #[inline(always)]
    pub const fn clrkey_high(self) -> crate::common::Reg<regs::ClrkeyHigh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn histogram_fifo(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::HistogramFifo, crate::common::RW> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize + n * 4usize) as _) }
    }
    #[doc = "Roi Width Config Register."]
    #[inline(always)]
    pub const fn roi_width(self) -> crate::common::Reg<regs::RoiWidth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0490usize) as _) }
    }
    #[doc = "Roi Width Config Register."]
    #[inline(always)]
    pub const fn roi_height(self) -> crate::common::Reg<regs::RoiHeight, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0494usize) as _) }
    }
    #[doc = "Pro Config Register."]
    #[inline(always)]
    pub const fn pro_ctrl(self) -> crate::common::Reg<regs::ProCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0498usize) as _) }
    }
    #[doc = "actual size."]
    #[inline(always)]
    pub const fn act_size(self) -> crate::common::Reg<regs::ActSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x049cusize) as _) }
    }
    #[doc = "vsync valid counter."]
    #[inline(always)]
    pub const fn vsync_valid_cnt(
        self,
    ) -> crate::common::Reg<regs::VsyncValidCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a0usize) as _) }
    }
    #[doc = "hsync valid counter."]
    #[inline(always)]
    pub const fn hsync_valid_cnt(
        self,
    ) -> crate::common::Reg<regs::HsyncValidCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a4usize) as _) }
    }
    #[doc = "valid margin."]
    #[inline(always)]
    pub const fn valid_margin(self) -> crate::common::Reg<regs::ValidMargin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a8usize) as _) }
    }
    #[doc = "alarm set."]
    #[inline(always)]
    pub const fn alarm_set(self) -> crate::common::Reg<regs::AlarmSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04acusize) as _) }
    }
}
pub mod regs {
    #[doc = "actual size."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ActSize(pub u32);
    impl ActSize {
        #[doc = "actual width after scale and/or roi."]
        #[inline(always)]
        pub const fn act_width(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "actual width after scale and/or roi."]
        #[inline(always)]
        pub fn set_act_width(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "actual height after scale and/or roi."]
        #[inline(always)]
        pub const fn act_height(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "actual height after scale and/or roi."]
        #[inline(always)]
        pub fn set_act_height(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for ActSize {
        #[inline(always)]
        fn default() -> ActSize {
            ActSize(0)
        }
    }
    #[doc = "alarm set."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlarmSet(pub u32);
    impl AlarmSet {
        #[doc = "frequency division."]
        #[inline(always)]
        pub const fn pre_div(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "frequency division."]
        #[inline(always)]
        pub fn set_pre_div(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "define signal duty cycles(base clock) 0x0: disable signal 0x1: high 1, low 15 0x2: high 2, low 14 …... 0xF: high 15, low 1."]
        #[inline(always)]
        pub const fn fatal_normal(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "define signal duty cycles(base clock) 0x0: disable signal 0x1: high 1, low 15 0x2: high 2, low 14 …... 0xF: high 15, low 1."]
        #[inline(always)]
        pub fn set_fatal_normal(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "define signal duty cycles(base clock) 0x0: disable signal 0x1: high 1, low 15 0x2: high 2, low 14 …... 0xF: high 15, low 1."]
        #[inline(always)]
        pub const fn sig_normal(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "define signal duty cycles(base clock) 0x0: disable signal 0x1: high 1, low 15 0x2: high 2, low 14 …... 0xF: high 15, low 1."]
        #[inline(always)]
        pub fn set_sig_normal(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for AlarmSet {
        #[inline(always)]
        fn default() -> AlarmSet {
            AlarmSet(0)
        }
    }
    #[doc = "Buffer Parameters Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BufPara(pub u32);
    impl BufPara {
        #[doc = "Line Blank Space Stride. Indicates the space between the end of line image storage and the start of a new line storage in the frame buffer. The width of the line storage in frame buffer(in double words) minus the width of the image(in double words) is the stride. The stride should be double words aligned. The embedded DMA controller will skip the stride before starting to write the next row of the image."]
        #[inline(always)]
        pub const fn linebsp_stride(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Line Blank Space Stride. Indicates the space between the end of line image storage and the start of a new line storage in the frame buffer. The width of the line storage in frame buffer(in double words) minus the width of the image(in double words) is the stride. The stride should be double words aligned. The embedded DMA controller will skip the stride before starting to write the next row of the image."]
        #[inline(always)]
        pub fn set_linebsp_stride(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for BufPara {
        #[inline(always)]
        fn default() -> BufPara {
            BufPara(0)
        }
    }
    #[doc = "High Color Key Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClrkeyHigh(pub u32);
    impl ClrkeyHigh {
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
    impl Default for ClrkeyHigh {
        #[inline(always)]
        fn default() -> ClrkeyHigh {
            ClrkeyHigh(0)
        }
    }
    #[doc = "Low Color Key Register."]
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
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "the bit width of the sensor 0: 8 bits 1: 10 bits 3:24bits Others: Undefined."]
        #[inline(always)]
        pub const fn sensor_bit_width(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "the bit width of the sensor 0: 8 bits 1: 10 bits 3:24bits Others: Undefined."]
        #[inline(always)]
        pub fn set_sensor_bit_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "input color formats: 0010b:24bit:RGB888 0011b:24bit:RGB666 0100b:16bit:RGB565 0101b:16bit:RGB444 0110b:16bit:RGB555 0111b: 16bit: YCbCr422 (Y0 Cb Y1 Cr, each 8-bit) YUV YCrCb Note: YUV420 is not supported. 1000b: 24bit: YUV444."]
        #[inline(always)]
        pub const fn color_formats(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[doc = "input color formats: 0010b:24bit:RGB888 0011b:24bit:RGB666 0100b:16bit:RGB565 0101b:16bit:RGB444 0110b:16bit:RGB555 0111b: 16bit: YCbCr422 (Y0 Cb Y1 Cr, each 8-bit) YUV YCrCb Note: YUV420 is not supported. 1000b: 24bit: YUV444."]
        #[inline(always)]
        pub fn set_color_formats(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[doc = "00: Normal Mode (one plane mode) 01: Two Plane Mode (Y, UV plane) 10: Y-only Mode, byte sequence as Y0,Y1,Y2,Y3 11: Binary Mode, bit sequence is from LSB to MSB when CR20\\[BIG_END\\]=0."]
        #[inline(always)]
        pub const fn storage_mode(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "00: Normal Mode (one plane mode) 01: Two Plane Mode (Y, UV plane) 10: Y-only Mode, byte sequence as Y0,Y1,Y2,Y3 11: Binary Mode, bit sequence is from LSB to MSB when CR20\\[BIG_END\\]=0."]
        #[inline(always)]
        pub fn set_storage_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Invert Data Input. This bit enables or disables internal inverters on the data lines. 0 CAM_D data lines are directly applied to internal circuitry 1 CAM_D data lines are inverted before applied to internal circuitry."]
        #[inline(always)]
        pub const fn inv_data(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Invert Data Input. This bit enables or disables internal inverters on the data lines. 0 CAM_D data lines are directly applied to internal circuitry 1 CAM_D data lines are inverted before applied to internal circuitry."]
        #[inline(always)]
        pub fn set_inv_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt. 0 SOF interrupt is generated on SOF falling edge 1 SOF interrupt is generated on SOF rising edge."]
        #[inline(always)]
        pub const fn sof_int_pol(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt. 0 SOF interrupt is generated on SOF falling edge 1 SOF interrupt is generated on SOF rising edge."]
        #[inline(always)]
        pub fn set_sof_int_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Synchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO on every SOF."]
        #[inline(always)]
        pub const fn sync_rxfifo_clr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO on every SOF."]
        #[inline(always)]
        pub fn set_sync_rxfifo_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ASynchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO immediately. It will be auto-cleared."]
        #[inline(always)]
        pub const fn async_rxfifo_clr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "ASynchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO immediately. It will be auto-cleared."]
        #[inline(always)]
        pub fn set_async_rxfifo_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "force to restart the bus pointer at the every end of the sof period, and at the same time, clr the fifo pointer."]
        #[inline(always)]
        pub const fn restart_busptr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "force to restart the bus pointer at the every end of the sof period, and at the same time, clr the fifo pointer."]
        #[inline(always)]
        pub fn set_restart_busptr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Data Packing Direction. This bit Controls how 8-bit/10-bit image data is packed into 32-bit RX FIFO. 0 Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. 1 Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO."]
        #[inline(always)]
        pub const fn pack_dir(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Data Packing Direction. This bit Controls how 8-bit/10-bit image data is packed into 32-bit RX FIFO. 0 Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. 1 Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO."]
        #[inline(always)]
        pub fn set_pack_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SWAP 16-Bit Enable. This bit enables the swapping of 16-bit data. Data is packed from 8-bit or 10-bit to 32-bit first (according to the setting of PACK_DIR) and then swapped as 16-bit words before being put into the RX FIFO. The action of the bit only affects the RX FIFO. NOTE: Example of swapping enabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x 33441122 NOTE: Example of swapping disabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x11223344 0 Disable swapping 1 Enable swapping."]
        #[inline(always)]
        pub const fn swap16_en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SWAP 16-Bit Enable. This bit enables the swapping of 16-bit data. Data is packed from 8-bit or 10-bit to 32-bit first (according to the setting of PACK_DIR) and then swapped as 16-bit words before being put into the RX FIFO. The action of the bit only affects the RX FIFO. NOTE: Example of swapping enabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x 33441122 NOTE: Example of swapping disabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x11223344 0 Disable swapping 1 Enable swapping."]
        #[inline(always)]
        pub fn set_swap16_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "invert vsync pad input before it is used."]
        #[inline(always)]
        pub const fn inv_vsync(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "invert vsync pad input before it is used."]
        #[inline(always)]
        pub fn set_inv_vsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "invert hsync pad input before it is used."]
        #[inline(always)]
        pub const fn inv_hsync(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "invert hsync pad input before it is used."]
        #[inline(always)]
        pub fn set_inv_hsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "invert pixclk pad input before it is used."]
        #[inline(always)]
        pub const fn inv_pixclk(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "invert pixclk pad input before it is used."]
        #[inline(always)]
        pub fn set_inv_pixclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "If asserted, will change the output color to ARGB8888 mode. Used by input color as RGB565, RGB888, YUV888, etc. The byte sequence is B,G,R,A. Depends on correct CR2\\[ClrBitFormat\\]
configuration."]
        #[inline(always)]
        pub const fn color_ext(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "If asserted, will change the output color to ARGB8888 mode. Used by input color as RGB565, RGB888, YUV888, etc. The byte sequence is B,G,R,A. Depends on correct CR2\\[ClrBitFormat\\]
configuration."]
        #[inline(always)]
        pub fn set_color_ext(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "invert den pad input before it is used."]
        #[inline(always)]
        pub const fn inv_den(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "invert den pad input before it is used."]
        #[inline(always)]
        pub fn set_inv_den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    #[doc = "Control CR18 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr18(pub u32);
    impl Cr18 {
        #[doc = "AWQOS for bus fabric arbitration."]
        #[inline(always)]
        pub const fn awqos(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[doc = "AWQOS for bus fabric arbitration."]
        #[inline(always)]
        pub fn set_awqos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
        }
    }
    impl Default for Cr18 {
        #[inline(always)]
        fn default() -> Cr18 {
            Cr18(0)
        }
    }
    #[doc = "Control 2 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Input Byte & bit sequence same as OV5640, except for Raw mode. Used only for internal ARGB conversion."]
        #[inline(always)]
        pub const fn clrbitformat(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Input Byte & bit sequence same as OV5640, except for Raw mode. Used only for internal ARGB conversion."]
        #[inline(always)]
        pub fn set_clrbitformat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "DMA Request Enable for RxFIFO. This bit enables the dma request from RxFIFO to the embedded DMA controller. 0 Disable the dma request 1 Enable the dma request. The UV Rx FIFO is only enabled to filling data in 2 plane mode."]
        #[inline(always)]
        pub const fn dma_req_en_rff(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Request Enable for RxFIFO. This bit enables the dma request from RxFIFO to the embedded DMA controller. 0 Disable the dma request 1 Enable the dma request. The UV Rx FIFO is only enabled to filling data in 2 plane mode."]
        #[inline(always)]
        pub fn set_dma_req_en_rff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RxFIFO Full Level. When the number of data in RxFIFO reaches this level, a RxFIFO full interrupt is generated, or an RXFIFO DMA request is sent. 000 4 Double words 001 8 Double words 010 16 Double words 011 24 Double words 100 32 Double words 101 48 Double words 110 64 Double words 111 96 Double words."]
        #[inline(always)]
        pub const fn rxff_level(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "RxFIFO Full Level. When the number of data in RxFIFO reaches this level, a RxFIFO full interrupt is generated, or an RXFIFO DMA request is sent. 000 4 Double words 001 8 Double words 010 16 Double words 011 24 Double words 100 32 Double words 101 48 Double words 110 64 Double words 111 96 Double words."]
        #[inline(always)]
        pub fn set_rxff_level(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "Frame Count Reset. Resets the Frame Counter. 0 Do not reset 1 Reset frame counter immediately."]
        #[inline(always)]
        pub const fn frmcnt_rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Frame Count Reset. Resets the Frame Counter. 0 Do not reset 1 Reset frame counter immediately."]
        #[inline(always)]
        pub fn set_frmcnt_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Frame Counter. This is a 16-bit Frame Counter (Wraps around automatically after reaching the maximum)."]
        #[inline(always)]
        pub const fn frmcnt_15_0(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Frame Counter. This is a 16-bit Frame Counter (Wraps around automatically after reaching the maximum)."]
        #[inline(always)]
        pub fn set_frmcnt_15_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    #[doc = "Control CR20 Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr20(pub u32);
    impl Cr20 {
        #[doc = "Threshold to generate binary color. Bin 1 is output if the pixel is greater than the threshold."]
        #[inline(always)]
        pub const fn threshold(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Threshold to generate binary color. Bin 1 is output if the pixel is greater than the threshold."]
        #[inline(always)]
        pub fn set_threshold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Asserted when binary output is in big-endian type, which mean the right most data is at the LSBs. Take function only inside the 32-bit word."]
        #[inline(always)]
        pub const fn big_end(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted when binary output is in big-endian type, which mean the right most data is at the LSBs. Take function only inside the 32-bit word."]
        #[inline(always)]
        pub fn set_big_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "histogarm enable."]
        #[inline(always)]
        pub const fn histogram_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "histogarm enable."]
        #[inline(always)]
        pub fn set_histogram_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "binary picture output enable."]
        #[inline(always)]
        pub const fn binary_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "binary picture output enable."]
        #[inline(always)]
        pub fn set_binary_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr20 {
        #[inline(always)]
        fn default() -> Cr20 {
            Cr20(0)
        }
    }
    #[doc = "Color Space Conversion Config Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CscCoef0(pub u32);
    impl CscCoef0 {
        #[doc = "Two's compliment amplitude offset implicit in the Y data. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
        #[inline(always)]
        pub const fn y_offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Two's compliment amplitude offset implicit in the Y data. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
        #[inline(always)]
        pub fn set_y_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "Two's compliment phase offset implicit for CbCr data. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
        #[inline(always)]
        pub const fn uv_offset(&self) -> u16 {
            let val = (self.0 >> 9usize) & 0x01ff;
            val as u16
        }
        #[doc = "Two's compliment phase offset implicit for CbCr data. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
        #[inline(always)]
        pub fn set_uv_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 9usize)) | (((val as u32) & 0x01ff) << 9usize);
        }
        #[doc = "Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)."]
        #[inline(always)]
        pub const fn c0(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)."]
        #[inline(always)]
        pub fn set_c0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
        }
        #[doc = "Enable the CSC unit 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data."]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the CSC unit 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data."]
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
    #[doc = "Color Space Conversion Config Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CscCoef1(pub u32);
    impl CscCoef1 {
        #[doc = "Two's compliment Blue U/Cb multiplier coefficient. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
        #[inline(always)]
        pub const fn c4(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Blue U/Cb multiplier coefficient. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
        #[inline(always)]
        pub fn set_c4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Two's compliment Red V/Cr multiplier coefficient. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
        #[inline(always)]
        pub const fn c1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Red V/Cr multiplier coefficient. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
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
    #[doc = "Color Space Conversion Config Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CscCoef2(pub u32);
    impl CscCoef2 {
        #[doc = "Two's compliment Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
        #[inline(always)]
        pub const fn c3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
        #[inline(always)]
        pub fn set_c3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Two's compliment Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
        #[inline(always)]
        pub const fn c2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Two's compliment Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
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
    #[doc = "Pixel DMA Frame Buffer 1 Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmasaFb1(pub u32);
    impl DmasaFb1 {
        #[doc = "DMA Start Address in Frame Buffer1. Indicates the start address to write data. The embedded DMA controller will read data from RxFIFO and write it from this address through AHB bus. The address should be double words aligned. In Two-Plane Mode, Y buffer1."]
        #[inline(always)]
        pub const fn ptr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "DMA Start Address in Frame Buffer1. Indicates the start address to write data. The embedded DMA controller will read data from RxFIFO and write it from this address through AHB bus. The address should be double words aligned. In Two-Plane Mode, Y buffer1."]
        #[inline(always)]
        pub fn set_ptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for DmasaFb1 {
        #[inline(always)]
        fn default() -> DmasaFb1 {
            DmasaFb1(0)
        }
    }
    #[doc = "Pixel DMA Frame Buffer 2 Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmasaFb2(pub u32);
    impl DmasaFb2 {
        #[doc = "DMA Start Address in Frame Buffer2. Indicates the start address to write data. The embedded DMA controller will read data from RxFIFO and write it from this address through AHB bus. The address should be double words aligned. In Two-Plane Mode, Y buffer2."]
        #[inline(always)]
        pub const fn ptr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "DMA Start Address in Frame Buffer2. Indicates the start address to write data. The embedded DMA controller will read data from RxFIFO and write it from this address through AHB bus. The address should be double words aligned. In Two-Plane Mode, Y buffer2."]
        #[inline(always)]
        pub fn set_ptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for DmasaFb2 {
        #[inline(always)]
        fn default() -> DmasaFb2 {
            DmasaFb2(0)
        }
    }
    #[doc = "Pixel UV DMA Frame Buffer 1 Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmasaUv1(pub u32);
    impl DmasaUv1 {
        #[doc = "Two Plane UV Buffer Start Address 1."]
        #[inline(always)]
        pub const fn ptr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Two Plane UV Buffer Start Address 1."]
        #[inline(always)]
        pub fn set_ptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for DmasaUv1 {
        #[inline(always)]
        fn default() -> DmasaUv1 {
            DmasaUv1(0)
        }
    }
    #[doc = "Pixel UV DMA Frame Buffer 2 Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmasaUv2(pub u32);
    impl DmasaUv2 {
        #[doc = "Two Plane UV Buffer Start Address 2."]
        #[inline(always)]
        pub const fn ptr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Two Plane UV Buffer Start Address 2."]
        #[inline(always)]
        pub fn set_ptr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for DmasaUv2 {
        #[inline(always)]
        fn default() -> DmasaUv2 {
            DmasaUv2(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HistogramFifo(pub u32);
    impl HistogramFifo {
        #[doc = "the appearance of bin x (x=(address-DATA0)/4)."]
        #[inline(always)]
        pub const fn hist_y(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "the appearance of bin x (x=(address-DATA0)/4)."]
        #[inline(always)]
        pub fn set_hist_y(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for HistogramFifo {
        #[inline(always)]
        fn default() -> HistogramFifo {
            HistogramFifo(0)
        }
    }
    #[doc = "hsync valid counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HsyncValidCnt(pub u32);
    impl HsyncValidCnt {
        #[doc = "hsync valid counter."]
        #[inline(always)]
        pub const fn hsync_valid_cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "hsync valid counter."]
        #[inline(always)]
        pub fn set_hsync_valid_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HsyncValidCnt {
        #[inline(always)]
        fn default() -> HsyncValidCnt {
            HsyncValidCnt(0)
        }
    }
    #[doc = "Ideal Image Size Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IdealWnSize(pub u32);
    impl IdealWnSize {
        #[doc = "Image Width. Indicates how many active pixels in a line of the image from the sensor. The number of bytes to be transferred is re-calculated automatically in hardware based on cr1\\[color_ext\\]
and cr1\\[store_mode\\]. Default value is 2*pixel number. As the input data from the sensor is 8-bit/pixel format, the IMAGE_WIDTH should be a multiple of 8 pixels."]
        #[inline(always)]
        pub const fn width(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Image Width. Indicates how many active pixels in a line of the image from the sensor. The number of bytes to be transferred is re-calculated automatically in hardware based on cr1\\[color_ext\\]
and cr1\\[store_mode\\]. Default value is 2*pixel number. As the input data from the sensor is 8-bit/pixel format, the IMAGE_WIDTH should be a multiple of 8 pixels."]
        #[inline(always)]
        pub fn set_width(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Image Height. Indicates how many active pixels in a column of the image from the sensor."]
        #[inline(always)]
        pub const fn height(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Image Height. Indicates how many active pixels in a column of the image from the sensor."]
        #[inline(always)]
        pub fn set_height(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for IdealWnSize {
        #[inline(always)]
        fn default() -> IdealWnSize {
            IdealWnSize(0)
        }
    }
    #[doc = "Interrupt Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt. 0 SOF interrupt disable 1 SOF interrupt enable."]
        #[inline(always)]
        pub const fn sof_int_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt. 0 SOF interrupt disable 1 SOF interrupt enable."]
        #[inline(always)]
        pub fn set_sof_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Frame Buffer1 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer1 DMA transfer done. 0 Frame Buffer1 DMA Transfer Done interrupt disable 1 Frame Buffer1 DMA Transfer Done interrupt enable."]
        #[inline(always)]
        pub const fn fb1_dma_done_inten(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Frame Buffer1 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer1 DMA transfer done. 0 Frame Buffer1 DMA Transfer Done interrupt disable 1 Frame Buffer1 DMA Transfer Done interrupt enable."]
        #[inline(always)]
        pub fn set_fb1_dma_done_inten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Frame Buffer2 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer2 DMA transfer done. 0 Frame Buffer2 DMA Transfer Done interrupt disable 1 Frame Buffer2 DMA Transfer Done interrupt enable."]
        #[inline(always)]
        pub const fn fb2_dma_done_inten(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Frame Buffer2 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer2 DMA transfer done. 0 Frame Buffer2 DMA Transfer Done interrupt disable 1 Frame Buffer2 DMA Transfer Done interrupt enable."]
        #[inline(always)]
        pub fn set_fb2_dma_done_inten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt. 0 RxFIFO overrun interrupt is disabled 1 RxFIFO overrun interrupt is enabled."]
        #[inline(always)]
        pub const fn rf_or_inten(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt. 0 RxFIFO overrun interrupt is disabled 1 RxFIFO overrun interrupt is enabled."]
        #[inline(always)]
        pub fn set_rf_or_inten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt. 0 EOF interrupt is disabled. 1 EOF interrupt is generated when RX count value is reached."]
        #[inline(always)]
        pub const fn eof_int_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt. 0 EOF interrupt is disabled. 1 EOF interrupt is generated when RX count value is reached."]
        #[inline(always)]
        pub fn set_eof_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Hresponse Error Enable. This bit enables the hresponse error interrupt. 0 Disable hresponse error interrupt 1 Enable hresponse error interrupt."]
        #[inline(always)]
        pub const fn hresp_err_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Hresponse Error Enable. This bit enables the hresponse error interrupt. 0 Disable hresponse error interrupt 1 Enable hresponse error interrupt."]
        #[inline(always)]
        pub fn set_hresp_err_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable hist done int."]
        #[inline(always)]
        pub const fn hist_done_int_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable hist done int."]
        #[inline(always)]
        pub fn set_hist_done_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation interrupt enable."]
        #[inline(always)]
        pub const fn err_cl_bwid_cfg_int_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation interrupt enable."]
        #[inline(always)]
        pub fn set_err_cl_bwid_cfg_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    #[doc = "Pro Config Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProCtrl(pub u32);
    impl ProCtrl {
        #[doc = "000 keep all pixel for width 001 keep 1 for every 2 pixel for width 010 keep 1 for every 3 pixel for width 011 keep 1 for every 4 pixel for width 100 keep 1 for every 5 pixel for width 101 keep 1 for every 6 pixel for width 110 keep 1 for every 7 pixel for width 111 keep 1 for every 8 pixel for width."]
        #[inline(always)]
        pub const fn scale_width_select(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "000 keep all pixel for width 001 keep 1 for every 2 pixel for width 010 keep 1 for every 3 pixel for width 011 keep 1 for every 4 pixel for width 100 keep 1 for every 5 pixel for width 101 keep 1 for every 6 pixel for width 110 keep 1 for every 7 pixel for width 111 keep 1 for every 8 pixel for width."]
        #[inline(always)]
        pub fn set_scale_width_select(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "000 keep all pixel for height 001 keep 1 for every 2 pixel for height 010 keep 1 for every 3 pixel for height 011 keep 1 for every 4 pixel for height 100 keep 1 for every 5 pixel for height 101 keep 1 for every 6 pixel for height 110 keep 1 for every 7 pixel for height 111 keep 1 for every 8 pixel for height."]
        #[inline(always)]
        pub const fn scale_height_select(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "000 keep all pixel for height 001 keep 1 for every 2 pixel for height 010 keep 1 for every 3 pixel for height 011 keep 1 for every 4 pixel for height 100 keep 1 for every 5 pixel for height 101 keep 1 for every 6 pixel for height 110 keep 1 for every 7 pixel for height 111 keep 1 for every 8 pixel for height."]
        #[inline(always)]
        pub fn set_scale_height_select(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "scale configration update."]
        #[inline(always)]
        pub const fn scale_update(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "scale configration update."]
        #[inline(always)]
        pub fn set_scale_update(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "roi configration update."]
        #[inline(always)]
        pub const fn roi_update(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "roi configration update."]
        #[inline(always)]
        pub fn set_roi_update(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "0 generate alarm in normal mode 1 force to generate fatal alarm."]
        #[inline(always)]
        pub const fn err_inject(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "0 generate alarm in normal mode 1 force to generate fatal alarm."]
        #[inline(always)]
        pub fn set_err_inject(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for ProCtrl {
        #[inline(always)]
        fn default() -> ProCtrl {
            ProCtrl(0)
        }
    }
    #[doc = "Roi Width Config Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RoiHeight(pub u32);
    impl RoiHeight {
        #[doc = "start address of height for roi."]
        #[inline(always)]
        pub const fn roi_height_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "start address of height for roi."]
        #[inline(always)]
        pub fn set_roi_height_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "end address of height for roi."]
        #[inline(always)]
        pub const fn roi_height_end(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "end address of height for roi."]
        #[inline(always)]
        pub fn set_roi_height_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for RoiHeight {
        #[inline(always)]
        fn default() -> RoiHeight {
            RoiHeight(0)
        }
    }
    #[doc = "Roi Width Config Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RoiWidth(pub u32);
    impl RoiWidth {
        #[doc = "start address of width for roi."]
        #[inline(always)]
        pub const fn roi_width_start(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "start address of width for roi."]
        #[inline(always)]
        pub fn set_roi_width_start(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "end address of width for roi."]
        #[inline(always)]
        pub const fn roi_width_end(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "end address of width for roi."]
        #[inline(always)]
        pub fn set_roi_width_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for RoiWidth {
        #[inline(always)]
        fn default() -> RoiWidth {
            RoiWidth(0)
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sta(pub u32);
    impl Sta {
        #[doc = "Hresponse Error Interrupt Status. Indicates that a hresponse error has been detected. (Cleared by writing 1) 0 No hresponse error. 1 Hresponse error is detected."]
        #[inline(always)]
        pub const fn hresp_err_int(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Hresponse Error Interrupt Status. Indicates that a hresponse error has been detected. (Cleared by writing 1) 0 No hresponse error. 1 Hresponse error is detected."]
        #[inline(always)]
        pub fn set_hresp_err_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1) 0 SOF is not detected. 1 SOF is detected."]
        #[inline(always)]
        pub const fn sof_int(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1) 0 SOF is not detected. 1 SOF is detected."]
        #[inline(always)]
        pub fn set_sof_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1) 0 EOF is not detected. 1 EOF is detected."]
        #[inline(always)]
        pub const fn eof_int(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1) 0 EOF is not detected. 1 EOF is detected."]
        #[inline(always)]
        pub fn set_eof_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DMA Transfer Done in Frame Buffer1. Indicates that the DMA transfer from RxFIFO to Frame Buffer1 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writing 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
        #[inline(always)]
        pub const fn dma_tsf_done_fb1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Transfer Done in Frame Buffer1. Indicates that the DMA transfer from RxFIFO to Frame Buffer1 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writing 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
        #[inline(always)]
        pub fn set_dma_tsf_done_fb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "DMA Transfer Done in Frame Buffer2. Indicates that the DMA transfer from RxFIFO to Frame Buffer2 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writing 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
        #[inline(always)]
        pub const fn dma_tsf_done_fb2(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DMA Transfer Done in Frame Buffer2. Indicates that the DMA transfer from RxFIFO to Frame Buffer2 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writing 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
        #[inline(always)]
        pub fn set_dma_tsf_done_fb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "RxFIFO Overrun Interrupt Status. Indicates the overflow status of the RxFIFO register. (Cleared by writing 1) 0 RXFIFO has not overflowed. 1 RXFIFO has overflowed."]
        #[inline(always)]
        pub const fn rf_or_int(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "RxFIFO Overrun Interrupt Status. Indicates the overflow status of the RxFIFO register. (Cleared by writing 1) 0 RXFIFO has not overflowed. 1 RXFIFO has overflowed."]
        #[inline(always)]
        pub fn set_rf_or_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "hist cal done."]
        #[inline(always)]
        pub const fn hist_done(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "hist cal done."]
        #[inline(always)]
        pub fn set_hist_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation found."]
        #[inline(always)]
        pub const fn err_cl_bwid_cfg(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation found."]
        #[inline(always)]
        pub fn set_err_cl_bwid_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Sta {
        #[inline(always)]
        fn default() -> Sta {
            Sta(0)
        }
    }
    #[doc = "valid margin."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ValidMargin(pub u32);
    impl ValidMargin {
        #[doc = "vsync valid margin."]
        #[inline(always)]
        pub const fn vsync_valid_margin(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "vsync valid margin."]
        #[inline(always)]
        pub fn set_vsync_valid_margin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "hsync valid margin."]
        #[inline(always)]
        pub const fn hsync_valid_margin(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "hsync valid margin."]
        #[inline(always)]
        pub fn set_hsync_valid_margin(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for ValidMargin {
        #[inline(always)]
        fn default() -> ValidMargin {
            ValidMargin(0)
        }
    }
    #[doc = "vsync valid counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VsyncValidCnt(pub u32);
    impl VsyncValidCnt {
        #[doc = "vsync valid counter."]
        #[inline(always)]
        pub const fn vsync_valid_cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "vsync valid counter."]
        #[inline(always)]
        pub fn set_vsync_valid_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for VsyncValidCnt {
        #[inline(always)]
        fn default() -> VsyncValidCnt {
            VsyncValidCnt(0)
        }
    }
}
