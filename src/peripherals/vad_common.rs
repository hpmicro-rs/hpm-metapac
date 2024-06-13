#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "VAD."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vad {
    ptr: *mut u8,
}
unsafe impl Send for Vad {}
unsafe impl Sync for Vad {}
impl Vad {
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
    #[doc = "Filter Control Register."]
    #[inline(always)]
    pub const fn filtctrl(self) -> crate::common::Reg<regs::Filtctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Decision Control Register 0."]
    #[inline(always)]
    pub const fn dec_ctrl0(self) -> crate::common::Reg<regs::DecCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Decision Control Register 1."]
    #[inline(always)]
    pub const fn dec_ctrl1(self) -> crate::common::Reg<regs::DecCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Decision Control Register 2."]
    #[inline(always)]
    pub const fn dec_ctrl2(self) -> crate::common::Reg<regs::DecCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn st(self) -> crate::common::Reg<regs::St, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Out FIFO."]
    #[inline(always)]
    pub const fn ofifo(self) -> crate::common::Reg<regs::Ofifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Run Command Register."]
    #[inline(always)]
    pub const fn run(self) -> crate::common::Reg<regs::Run, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Out FIFO Control Register."]
    #[inline(always)]
    pub const fn ofifo_ctrl(self) -> crate::common::Reg<regs::OfifoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "CIC Configuration Register."]
    #[inline(always)]
    pub const fn cic_cfg(self) -> crate::common::Reg<regs::CicCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn coef(self, n: usize) -> crate::common::Reg<regs::Coef, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "CIC Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CicCfg(pub u32);
    impl CicCfg {
        #[doc = "the shift value after CIC results."]
        #[inline(always)]
        pub const fn post_scale(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x3f;
            val as u8
        }
        #[doc = "the shift value after CIC results."]
        #[inline(always)]
        pub fn set_post_scale(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
        }
    }
    impl Default for CicCfg {
        #[inline(always)]
        fn default() -> CicCfg {
            CicCfg(0)
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Coef(pub u32);
    impl Coef {
        #[doc = "The current detected short time energy."]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The current detected short time energy."]
        #[inline(always)]
        pub fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Coef {
        #[inline(always)]
        fn default() -> Coef {
            Coef(0)
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "the number of channels to be stored in buffer. Asserted to enable 2 channels."]
        #[inline(always)]
        pub const fn chnum(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "the number of channels to be stored in buffer. Asserted to enable 2 channels."]
        #[inline(always)]
        pub fn set_chnum(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
        #[inline(always)]
        pub const fn ch_pol(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
        #[inline(always)]
        pub fn set_ch_pol(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "pdm_clk_output_en."]
        #[inline(always)]
        pub const fn pdm_clk_oe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "pdm_clk_output_en."]
        #[inline(always)]
        pub fn set_pdm_clk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "asserted to bypass the pdm clock divider."]
        #[inline(always)]
        pub const fn pdm_clk_div_bypass(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to bypass the pdm clock divider."]
        #[inline(always)]
        pub fn set_pdm_clk_div_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OFIFO threshold to generate ofifo_av (when fillings >= threshold) (fifo size: max 16 items, 16*32bits)."]
        #[inline(always)]
        pub const fn fifo_thrsh(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "OFIFO threshold to generate ofifo_av (when fillings >= threshold) (fifo size: max 16 items, 16*32bits)."]
        #[inline(always)]
        pub fn set_fifo_thrsh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "asserted to disable membuf."]
        #[inline(always)]
        pub const fn membuf_disable(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to disable membuf."]
        #[inline(always)]
        pub fn set_membuf_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CIC saturation Interrupt Enable."]
        #[inline(always)]
        pub const fn cic_sat_err_ie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CIC saturation Interrupt Enable."]
        #[inline(always)]
        pub fn set_cic_sat_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CIC overload Interrupt Enable."]
        #[inline(always)]
        pub const fn cic_ovld_err_ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CIC overload Interrupt Enable."]
        #[inline(always)]
        pub fn set_cic_ovld_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "IIR overflow error interrupt enable."]
        #[inline(always)]
        pub const fn iir_ovfl_err_ie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "IIR overflow error interrupt enable."]
        #[inline(always)]
        pub fn set_iir_ovfl_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "IIR overload error interrupt enable."]
        #[inline(always)]
        pub const fn iir_ovld_err_ie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "IIR overload error interrupt enable."]
        #[inline(always)]
        pub fn set_iir_ovld_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OFIFO overflow error interrupt enable."]
        #[inline(always)]
        pub const fn ofifo_ovfl_err_ie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "OFIFO overflow error interrupt enable."]
        #[inline(always)]
        pub fn set_ofifo_ovfl_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Buf empty interrupt enable."]
        #[inline(always)]
        pub const fn membuf_empty_ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Buf empty interrupt enable."]
        #[inline(always)]
        pub fn set_membuf_empty_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OFIFO data available interrupt enable."]
        #[inline(always)]
        pub const fn ofifo_av_ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "OFIFO data available interrupt enable."]
        #[inline(always)]
        pub fn set_ofifo_av_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "VAD event interrupt enable."]
        #[inline(always)]
        pub const fn vad_ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "VAD event interrupt enable."]
        #[inline(always)]
        pub fn set_vad_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)."]
        #[inline(always)]
        pub const fn pdm_clk_hfdiv(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)."]
        #[inline(always)]
        pub fn set_pdm_clk_hfdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Capture cycle delay>=0, should be less than PDM_CLK_HFDIV."]
        #[inline(always)]
        pub const fn capt_dly(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Capture cycle delay>=0, should be less than PDM_CLK_HFDIV."]
        #[inline(always)]
        pub fn set_capt_dly(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "Decision Control Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DecCtrl0(pub u32);
    impl DecCtrl0 {
        #[doc = "length of sub-block."]
        #[inline(always)]
        pub const fn subblk_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "length of sub-block."]
        #[inline(always)]
        pub fn set_subblk_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "asserted to have 3 sub-blocks, otherwise to have 2 sub-blocks."]
        #[inline(always)]
        pub const fn blk_cfg(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to have 3 sub-blocks, otherwise to have 2 sub-blocks."]
        #[inline(always)]
        pub fn set_blk_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "the value of amplitude for noise determination when calculationg ZCR."]
        #[inline(always)]
        pub const fn noise_tol(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "the value of amplitude for noise determination when calculationg ZCR."]
        #[inline(always)]
        pub fn set_noise_tol(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DecCtrl0 {
        #[inline(always)]
        fn default() -> DecCtrl0 {
            DecCtrl0(0)
        }
    }
    #[doc = "Decision Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DecCtrl1(pub u32);
    impl DecCtrl1 {
        #[doc = "ZCR low limit."]
        #[inline(always)]
        pub const fn zcr_low(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "ZCR low limit."]
        #[inline(always)]
        pub fn set_zcr_low(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "ZCR high limit."]
        #[inline(always)]
        pub const fn zcr_high(&self) -> u16 {
            let val = (self.0 >> 11usize) & 0x07ff;
            val as u16
        }
        #[doc = "ZCR high limit."]
        #[inline(always)]
        pub fn set_zcr_high(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 11usize)) | (((val as u32) & 0x07ff) << 11usize);
        }
    }
    impl Default for DecCtrl1 {
        #[inline(always)]
        fn default() -> DecCtrl1 {
            DecCtrl1(0)
        }
    }
    #[doc = "Decision Control Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DecCtrl2(pub u32);
    impl DecCtrl2 {
        #[doc = "amplitude low limit."]
        #[inline(always)]
        pub const fn amp_low(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "amplitude low limit."]
        #[inline(always)]
        pub fn set_amp_low(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "amplitude high limit."]
        #[inline(always)]
        pub const fn amp_high(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "amplitude high limit."]
        #[inline(always)]
        pub fn set_amp_high(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DecCtrl2 {
        #[inline(always)]
        fn default() -> DecCtrl2 {
            DecCtrl2(0)
        }
    }
    #[doc = "Filter Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Filtctrl(pub u32);
    impl Filtctrl {
        #[doc = "IIR slot enable."]
        #[inline(always)]
        pub const fn iir_slot_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "IIR slot enable."]
        #[inline(always)]
        pub fn set_iir_slot_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the decimation ratio of iir after CIC -1 2: means dec-by-3."]
        #[inline(always)]
        pub const fn decratio(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "the decimation ratio of iir after CIC -1 2: means dec-by-3."]
        #[inline(always)]
        pub fn set_decratio(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
    }
    impl Default for Filtctrl {
        #[inline(always)]
        fn default() -> Filtctrl {
            Filtctrl(0)
        }
    }
    #[doc = "Out FIFO."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ofifo(pub u32);
    impl Ofifo {
        #[doc = "The PCM data. When there is only one channel, the samples are from Ch0, and the 2 samples in the 32-bits are: bit \\[31:16\\]: the samples earlier in time (\\[T-1\\]). Bit \\[15:0\\]: the samples later in time (\\[T\\]). When there is two channels, the samples in the 32-bits are: bit \\[31:16\\]: the samples belong to Ch 1 (when ch_pol\\[1:0\\]==2, the data is captured at the positive part of the pdm clk). bit \\[15:0\\]: the samples belong to Ch 0 (when ch_pol\\[1:0\\]==2, the data is captured at the negtive part of the pdm clk)."]
        #[inline(always)]
        pub const fn d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The PCM data. When there is only one channel, the samples are from Ch0, and the 2 samples in the 32-bits are: bit \\[31:16\\]: the samples earlier in time (\\[T-1\\]). Bit \\[15:0\\]: the samples later in time (\\[T\\]). When there is two channels, the samples in the 32-bits are: bit \\[31:16\\]: the samples belong to Ch 1 (when ch_pol\\[1:0\\]==2, the data is captured at the positive part of the pdm clk). bit \\[15:0\\]: the samples belong to Ch 0 (when ch_pol\\[1:0\\]==2, the data is captured at the negtive part of the pdm clk)."]
        #[inline(always)]
        pub fn set_d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ofifo {
        #[inline(always)]
        fn default() -> Ofifo {
            Ofifo(0)
        }
    }
    #[doc = "Out FIFO Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OfifoCtrl(pub u32);
    impl OfifoCtrl {
        #[doc = "Asserted to enable OFIFO."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enable OFIFO."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for OfifoCtrl {
        #[inline(always)]
        fn default() -> OfifoCtrl {
            OfifoCtrl(0)
        }
    }
    #[doc = "Run Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Run(pub u32);
    impl Run {
        #[doc = "module enable."]
        #[inline(always)]
        pub const fn vad_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "module enable."]
        #[inline(always)]
        pub fn set_vad_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "software reset. Self-clear."]
        #[inline(always)]
        pub const fn sftrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "software reset. Self-clear."]
        #[inline(always)]
        pub fn set_sftrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Run {
        #[inline(always)]
        fn default() -> Run {
            Run(0)
        }
    }
    #[doc = "Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct St(pub u32);
    impl St {
        #[doc = "CIC saturation."]
        #[inline(always)]
        pub const fn cic_sat_err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CIC saturation."]
        #[inline(always)]
        pub fn set_cic_sat_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CIC overload."]
        #[inline(always)]
        pub const fn cic_ovld_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CIC overload."]
        #[inline(always)]
        pub fn set_cic_ovld_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IIR oberflow."]
        #[inline(always)]
        pub const fn iir_ovfl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IIR oberflow."]
        #[inline(always)]
        pub fn set_iir_ovfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IIR overloading."]
        #[inline(always)]
        pub const fn iir_ovld(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IIR overloading."]
        #[inline(always)]
        pub fn set_iir_ovld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OFIFO overflow."]
        #[inline(always)]
        pub const fn ofifo_ovfl(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OFIFO overflow."]
        #[inline(always)]
        pub fn set_ofifo_ovfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Buf empty."]
        #[inline(always)]
        pub const fn membuf_empty(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Buf empty."]
        #[inline(always)]
        pub fn set_membuf_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "OFIFO data available."]
        #[inline(always)]
        pub const fn ofifo_av(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "OFIFO data available."]
        #[inline(always)]
        pub fn set_ofifo_av(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "VAD event found."]
        #[inline(always)]
        pub const fn vad(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "VAD event found."]
        #[inline(always)]
        pub fn set_vad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for St {
        #[inline(always)]
        fn default() -> St {
            St(0)
        }
    }
}
