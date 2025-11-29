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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Filter Control Register."]
    #[inline(always)]
    pub const fn filtctrl(self) -> crate::common::Reg<regs::Filtctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Decision Control Register 0."]
    #[inline(always)]
    pub const fn dec_ctrl0(self) -> crate::common::Reg<regs::DecCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Decision Control Register 1."]
    #[inline(always)]
    pub const fn dec_ctrl1(self) -> crate::common::Reg<regs::DecCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Decision Control Register 2."]
    #[inline(always)]
    pub const fn dec_ctrl2(self) -> crate::common::Reg<regs::DecCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn st(self) -> crate::common::Reg<regs::St, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Out FIFO."]
    #[inline(always)]
    pub const fn ofifo(self) -> crate::common::Reg<regs::Ofifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Run Command Register."]
    #[inline(always)]
    pub const fn run(self) -> crate::common::Reg<regs::Run, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Out FIFO Control Register."]
    #[inline(always)]
    pub const fn ofifo_ctrl(self) -> crate::common::Reg<regs::OfifoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "CIC Configuration Register."]
    #[inline(always)]
    pub const fn cic_cfg(self) -> crate::common::Reg<regs::CicCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn coef(self, n: usize) -> crate::common::Reg<regs::Coef, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
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
    #[doc = "CIC Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CicCfg(pub u32);
    impl CicCfg {
        #[doc = "the shift value after CIC results."]
        #[must_use]
        #[inline(always)]
        pub const fn post_scale(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x3f;
            val as u8
        }
        #[doc = "the shift value after CIC results."]
        #[inline(always)]
        pub const fn set_post_scale(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
        }
    }
    impl Default for CicCfg {
        #[inline(always)]
        fn default() -> CicCfg {
            CicCfg(0)
        }
    }
    impl core::fmt::Debug for CicCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CicCfg")
                .field("post_scale", &self.post_scale())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CicCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CicCfg {{ post_scale: {=u8:?} }}", self.post_scale())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Coef(pub u32);
    impl Coef {
        #[doc = "The current detected short time energy."]
        #[must_use]
        #[inline(always)]
        pub const fn val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The current detected short time energy."]
        #[inline(always)]
        pub const fn set_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Coef {
        #[inline(always)]
        fn default() -> Coef {
            Coef(0)
        }
    }
    impl core::fmt::Debug for Coef {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Coef").field("val", &self.val()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Coef {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Coef {{ val: {=u32:?} }}", self.val())
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "the number of channels to be stored in buffer. Asserted to enable 2 channels."]
        #[must_use]
        #[inline(always)]
        pub const fn chnum(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "the number of channels to be stored in buffer. Asserted to enable 2 channels."]
        #[inline(always)]
        pub const fn set_chnum(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
        #[must_use]
        #[inline(always)]
        pub const fn ch_pol(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
        #[inline(always)]
        pub const fn set_ch_pol(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "pdm_clk_output_en."]
        #[must_use]
        #[inline(always)]
        pub const fn pdm_clk_oe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "pdm_clk_output_en."]
        #[inline(always)]
        pub const fn set_pdm_clk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "asserted to bypass the pdm clock divider."]
        #[must_use]
        #[inline(always)]
        pub const fn pdm_clk_div_bypass(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to bypass the pdm clock divider."]
        #[inline(always)]
        pub const fn set_pdm_clk_div_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OFIFO threshold to generate ofifo_av (when fillings >= threshold) (fifo size: max 16 items, 16*32bits)."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_thrsh(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "OFIFO threshold to generate ofifo_av (when fillings >= threshold) (fifo size: max 16 items, 16*32bits)."]
        #[inline(always)]
        pub const fn set_fifo_thrsh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "asserted to disable membuf."]
        #[must_use]
        #[inline(always)]
        pub const fn membuf_disable(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to disable membuf."]
        #[inline(always)]
        pub const fn set_membuf_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CIC saturation Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cic_sat_err_ie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CIC saturation Interrupt Enable."]
        #[inline(always)]
        pub const fn set_cic_sat_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CIC overload Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cic_ovld_err_ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CIC overload Interrupt Enable."]
        #[inline(always)]
        pub const fn set_cic_ovld_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "IIR overflow error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_ovfl_err_ie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "IIR overflow error interrupt enable."]
        #[inline(always)]
        pub const fn set_iir_ovfl_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "IIR overload error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_ovld_err_ie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "IIR overload error interrupt enable."]
        #[inline(always)]
        pub const fn set_iir_ovld_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OFIFO overflow error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ofifo_ovfl_err_ie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "OFIFO overflow error interrupt enable."]
        #[inline(always)]
        pub const fn set_ofifo_ovfl_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Buf empty interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn membuf_empty_ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Buf empty interrupt enable."]
        #[inline(always)]
        pub const fn set_membuf_empty_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OFIFO data available interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ofifo_av_ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "OFIFO data available interrupt enable."]
        #[inline(always)]
        pub const fn set_ofifo_av_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "VAD event interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vad_ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "VAD event interrupt enable."]
        #[inline(always)]
        pub const fn set_vad_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)."]
        #[must_use]
        #[inline(always)]
        pub const fn pdm_clk_hfdiv(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)."]
        #[inline(always)]
        pub const fn set_pdm_clk_hfdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Capture cycle delay>=0, should be less than PDM_CLK_HFDIV."]
        #[must_use]
        #[inline(always)]
        pub const fn capt_dly(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Capture cycle delay>=0, should be less than PDM_CLK_HFDIV."]
        #[inline(always)]
        pub const fn set_capt_dly(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
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
                .field("chnum", &self.chnum())
                .field("ch_pol", &self.ch_pol())
                .field("pdm_clk_oe", &self.pdm_clk_oe())
                .field("pdm_clk_div_bypass", &self.pdm_clk_div_bypass())
                .field("fifo_thrsh", &self.fifo_thrsh())
                .field("membuf_disable", &self.membuf_disable())
                .field("cic_sat_err_ie", &self.cic_sat_err_ie())
                .field("cic_ovld_err_ie", &self.cic_ovld_err_ie())
                .field("iir_ovfl_err_ie", &self.iir_ovfl_err_ie())
                .field("iir_ovld_err_ie", &self.iir_ovld_err_ie())
                .field("ofifo_ovfl_err_ie", &self.ofifo_ovfl_err_ie())
                .field("membuf_empty_ie", &self.membuf_empty_ie())
                .field("ofifo_av_ie", &self.ofifo_av_ie())
                .field("vad_ie", &self.vad_ie())
                .field("pdm_clk_hfdiv", &self.pdm_clk_hfdiv())
                .field("capt_dly", &self.capt_dly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl {{ chnum: {=bool:?}, ch_pol: {=u8:?}, pdm_clk_oe: {=bool:?}, pdm_clk_div_bypass: {=bool:?}, fifo_thrsh: {=u8:?}, membuf_disable: {=bool:?}, cic_sat_err_ie: {=bool:?}, cic_ovld_err_ie: {=bool:?}, iir_ovfl_err_ie: {=bool:?}, iir_ovld_err_ie: {=bool:?}, ofifo_ovfl_err_ie: {=bool:?}, membuf_empty_ie: {=bool:?}, ofifo_av_ie: {=bool:?}, vad_ie: {=bool:?}, pdm_clk_hfdiv: {=u8:?}, capt_dly: {=u8:?} }}" , self . chnum () , self . ch_pol () , self . pdm_clk_oe () , self . pdm_clk_div_bypass () , self . fifo_thrsh () , self . membuf_disable () , self . cic_sat_err_ie () , self . cic_ovld_err_ie () , self . iir_ovfl_err_ie () , self . iir_ovld_err_ie () , self . ofifo_ovfl_err_ie () , self . membuf_empty_ie () , self . ofifo_av_ie () , self . vad_ie () , self . pdm_clk_hfdiv () , self . capt_dly ())
        }
    }
    #[doc = "Decision Control Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DecCtrl0(pub u32);
    impl DecCtrl0 {
        #[doc = "length of sub-block."]
        #[must_use]
        #[inline(always)]
        pub const fn subblk_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "length of sub-block."]
        #[inline(always)]
        pub const fn set_subblk_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "asserted to have 3 sub-blocks, otherwise to have 2 sub-blocks."]
        #[must_use]
        #[inline(always)]
        pub const fn blk_cfg(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to have 3 sub-blocks, otherwise to have 2 sub-blocks."]
        #[inline(always)]
        pub const fn set_blk_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "the value of amplitude for noise determination when calculationg ZCR."]
        #[must_use]
        #[inline(always)]
        pub const fn noise_tol(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "the value of amplitude for noise determination when calculationg ZCR."]
        #[inline(always)]
        pub const fn set_noise_tol(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DecCtrl0 {
        #[inline(always)]
        fn default() -> DecCtrl0 {
            DecCtrl0(0)
        }
    }
    impl core::fmt::Debug for DecCtrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DecCtrl0")
                .field("subblk_len", &self.subblk_len())
                .field("blk_cfg", &self.blk_cfg())
                .field("noise_tol", &self.noise_tol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DecCtrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DecCtrl0 {{ subblk_len: {=u16:?}, blk_cfg: {=bool:?}, noise_tol: {=u16:?} }}",
                self.subblk_len(),
                self.blk_cfg(),
                self.noise_tol()
            )
        }
    }
    #[doc = "Decision Control Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DecCtrl1(pub u32);
    impl DecCtrl1 {
        #[doc = "ZCR low limit."]
        #[must_use]
        #[inline(always)]
        pub const fn zcr_low(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "ZCR low limit."]
        #[inline(always)]
        pub const fn set_zcr_low(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "ZCR high limit."]
        #[must_use]
        #[inline(always)]
        pub const fn zcr_high(&self) -> u16 {
            let val = (self.0 >> 11usize) & 0x07ff;
            val as u16
        }
        #[doc = "ZCR high limit."]
        #[inline(always)]
        pub const fn set_zcr_high(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 11usize)) | (((val as u32) & 0x07ff) << 11usize);
        }
    }
    impl Default for DecCtrl1 {
        #[inline(always)]
        fn default() -> DecCtrl1 {
            DecCtrl1(0)
        }
    }
    impl core::fmt::Debug for DecCtrl1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DecCtrl1")
                .field("zcr_low", &self.zcr_low())
                .field("zcr_high", &self.zcr_high())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DecCtrl1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DecCtrl1 {{ zcr_low: {=u16:?}, zcr_high: {=u16:?} }}",
                self.zcr_low(),
                self.zcr_high()
            )
        }
    }
    #[doc = "Decision Control Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DecCtrl2(pub u32);
    impl DecCtrl2 {
        #[doc = "amplitude low limit."]
        #[must_use]
        #[inline(always)]
        pub const fn amp_low(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "amplitude low limit."]
        #[inline(always)]
        pub const fn set_amp_low(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "amplitude high limit."]
        #[must_use]
        #[inline(always)]
        pub const fn amp_high(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "amplitude high limit."]
        #[inline(always)]
        pub const fn set_amp_high(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DecCtrl2 {
        #[inline(always)]
        fn default() -> DecCtrl2 {
            DecCtrl2(0)
        }
    }
    impl core::fmt::Debug for DecCtrl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DecCtrl2")
                .field("amp_low", &self.amp_low())
                .field("amp_high", &self.amp_high())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DecCtrl2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DecCtrl2 {{ amp_low: {=u16:?}, amp_high: {=u16:?} }}",
                self.amp_low(),
                self.amp_high()
            )
        }
    }
    #[doc = "Filter Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Filtctrl(pub u32);
    impl Filtctrl {
        #[doc = "IIR slot enable."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_slot_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "IIR slot enable."]
        #[inline(always)]
        pub const fn set_iir_slot_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the decimation ratio of iir after CIC -1 2: means dec-by-3."]
        #[must_use]
        #[inline(always)]
        pub const fn decratio(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "the decimation ratio of iir after CIC -1 2: means dec-by-3."]
        #[inline(always)]
        pub const fn set_decratio(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
    }
    impl Default for Filtctrl {
        #[inline(always)]
        fn default() -> Filtctrl {
            Filtctrl(0)
        }
    }
    impl core::fmt::Debug for Filtctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Filtctrl")
                .field("iir_slot_en", &self.iir_slot_en())
                .field("decratio", &self.decratio())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Filtctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Filtctrl {{ iir_slot_en: {=u8:?}, decratio: {=u8:?} }}",
                self.iir_slot_en(),
                self.decratio()
            )
        }
    }
    #[doc = "Out FIFO."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ofifo(pub u32);
    impl Ofifo {
        #[doc = "The PCM data. When there is only one channel, the samples are from Ch0, and the 2 samples in the 32-bits are: bit \\[31:16\\]: the samples earlier in time (\\[T-1\\]). Bit \\[15:0\\]: the samples later in time (\\[T\\]). When there is two channels, the samples in the 32-bits are: bit \\[31:16\\]: the samples belong to Ch 1 (when ch_pol\\[1:0\\]==2, the data is captured at the positive part of the pdm clk). bit \\[15:0\\]: the samples belong to Ch 0 (when ch_pol\\[1:0\\]==2, the data is captured at the negtive part of the pdm clk)."]
        #[must_use]
        #[inline(always)]
        pub const fn d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The PCM data. When there is only one channel, the samples are from Ch0, and the 2 samples in the 32-bits are: bit \\[31:16\\]: the samples earlier in time (\\[T-1\\]). Bit \\[15:0\\]: the samples later in time (\\[T\\]). When there is two channels, the samples in the 32-bits are: bit \\[31:16\\]: the samples belong to Ch 1 (when ch_pol\\[1:0\\]==2, the data is captured at the positive part of the pdm clk). bit \\[15:0\\]: the samples belong to Ch 0 (when ch_pol\\[1:0\\]==2, the data is captured at the negtive part of the pdm clk)."]
        #[inline(always)]
        pub const fn set_d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ofifo {
        #[inline(always)]
        fn default() -> Ofifo {
            Ofifo(0)
        }
    }
    impl core::fmt::Debug for Ofifo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ofifo").field("d", &self.d()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ofifo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ofifo {{ d: {=u32:?} }}", self.d())
        }
    }
    #[doc = "Out FIFO Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OfifoCtrl(pub u32);
    impl OfifoCtrl {
        #[doc = "Asserted to enable OFIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enable OFIFO."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for OfifoCtrl {
        #[inline(always)]
        fn default() -> OfifoCtrl {
            OfifoCtrl(0)
        }
    }
    impl core::fmt::Debug for OfifoCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OfifoCtrl").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OfifoCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OfifoCtrl {{ en: {=bool:?} }}", self.en())
        }
    }
    #[doc = "Run Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Run(pub u32);
    impl Run {
        #[doc = "module enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vad_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "module enable."]
        #[inline(always)]
        pub const fn set_vad_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "software reset. Self-clear."]
        #[must_use]
        #[inline(always)]
        pub const fn sftrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "software reset. Self-clear."]
        #[inline(always)]
        pub const fn set_sftrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Run {
        #[inline(always)]
        fn default() -> Run {
            Run(0)
        }
    }
    impl core::fmt::Debug for Run {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Run")
                .field("vad_en", &self.vad_en())
                .field("sftrst", &self.sftrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Run {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Run {{ vad_en: {=bool:?}, sftrst: {=bool:?} }}",
                self.vad_en(),
                self.sftrst()
            )
        }
    }
    #[doc = "Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct St(pub u32);
    impl St {
        #[doc = "CIC saturation."]
        #[must_use]
        #[inline(always)]
        pub const fn cic_sat_err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CIC saturation."]
        #[inline(always)]
        pub const fn set_cic_sat_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CIC overload."]
        #[must_use]
        #[inline(always)]
        pub const fn cic_ovld_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CIC overload."]
        #[inline(always)]
        pub const fn set_cic_ovld_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IIR oberflow."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_ovfl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IIR oberflow."]
        #[inline(always)]
        pub const fn set_iir_ovfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IIR overloading."]
        #[must_use]
        #[inline(always)]
        pub const fn iir_ovld(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IIR overloading."]
        #[inline(always)]
        pub const fn set_iir_ovld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OFIFO overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn ofifo_ovfl(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OFIFO overflow."]
        #[inline(always)]
        pub const fn set_ofifo_ovfl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Buf empty."]
        #[must_use]
        #[inline(always)]
        pub const fn membuf_empty(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Buf empty."]
        #[inline(always)]
        pub const fn set_membuf_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "OFIFO data available."]
        #[must_use]
        #[inline(always)]
        pub const fn ofifo_av(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "OFIFO data available."]
        #[inline(always)]
        pub const fn set_ofifo_av(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "VAD event found."]
        #[must_use]
        #[inline(always)]
        pub const fn vad(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "VAD event found."]
        #[inline(always)]
        pub const fn set_vad(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for St {
        #[inline(always)]
        fn default() -> St {
            St(0)
        }
    }
    impl core::fmt::Debug for St {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("St")
                .field("cic_sat_err", &self.cic_sat_err())
                .field("cic_ovld_err", &self.cic_ovld_err())
                .field("iir_ovfl", &self.iir_ovfl())
                .field("iir_ovld", &self.iir_ovld())
                .field("ofifo_ovfl", &self.ofifo_ovfl())
                .field("membuf_empty", &self.membuf_empty())
                .field("ofifo_av", &self.ofifo_av())
                .field("vad", &self.vad())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for St {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "St {{ cic_sat_err: {=bool:?}, cic_ovld_err: {=bool:?}, iir_ovfl: {=bool:?}, iir_ovld: {=bool:?}, ofifo_ovfl: {=bool:?}, membuf_empty: {=bool:?}, ofifo_av: {=bool:?}, vad: {=bool:?} }}" , self . cic_sat_err () , self . cic_ovld_err () , self . iir_ovfl () , self . iir_ovld () , self . ofifo_ovfl () , self . membuf_empty () , self . ofifo_av () , self . vad ())
        }
    }
}
