#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PDM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdm {
    ptr: *mut u8,
}
unsafe impl Send for Pdm {}
unsafe impl Sync for Pdm {}
impl Pdm {
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
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch_ctrl(self) -> crate::common::Reg<regs::ChCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn st(self) -> crate::common::Reg<regs::St, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CIC configuration register."]
    #[inline(always)]
    pub const fn cic_cfg(self) -> crate::common::Reg<regs::CicCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Run Register."]
    #[inline(always)]
    pub const fn run(self) -> crate::common::Reg<regs::Run, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
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
    #[doc = "Channel Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChCtrl(pub u32);
    impl ChCtrl {
        #[doc = "Asserted to enable the channel. Ch8 & 9 are refs. Ch0-7 are pdm mics."]
        #[must_use]
        #[inline(always)]
        pub const fn ch_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Asserted to enable the channel. Ch8 & 9 are refs. Ch0-7 are pdm mics."]
        #[inline(always)]
        pub const fn set_ch_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
        #[must_use]
        #[inline(always)]
        pub const fn ch_pol(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
        #[inline(always)]
        pub const fn set_ch_pol(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ChCtrl {
        #[inline(always)]
        fn default() -> ChCtrl {
            ChCtrl(0)
        }
    }
    impl core::fmt::Debug for ChCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ChCtrl")
                .field("ch_en", &self.ch_en())
                .field("ch_pol", &self.ch_pol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ChCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ChCtrl {{ ch_en: {=u16:?}, ch_pol: {=u8:?} }}",
                self.ch_en(),
                self.ch_pol()
            )
        }
    }
    #[doc = "CIC configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CicCfg(pub u32);
    impl CicCfg {
        #[doc = "CIC decimation factor."]
        #[must_use]
        #[inline(always)]
        pub const fn cic_dec_ratio(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "CIC decimation factor."]
        #[inline(always)]
        pub const fn set_cic_dec_ratio(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Sigma_delta_order\\[1:0\\]
2'b00: 7 2'b01: 6 2'b10: 5 Others: unused."]
        #[must_use]
        #[inline(always)]
        pub const fn sgd(&self) -> super::vals::SigmaDeltaOrder {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::SigmaDeltaOrder::from_bits(val as u8)
        }
        #[doc = "Sigma_delta_order\\[1:0\\]
2'b00: 7 2'b01: 6 2'b10: 5 Others: unused."]
        #[inline(always)]
        pub const fn set_sgd(&mut self, val: super::vals::SigmaDeltaOrder) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
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
                .field("cic_dec_ratio", &self.cic_dec_ratio())
                .field("sgd", &self.sgd())
                .field("post_scale", &self.post_scale())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CicCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CicCfg {{ cic_dec_ratio: {=u8:?}, sgd: {:?}, post_scale: {=u8:?} }}",
                self.cic_dec_ratio(),
                self.sgd(),
                self.post_scale()
            )
        }
    }
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "pdm_clk_output_en."]
        #[must_use]
        #[inline(always)]
        pub const fn pdm_clk_oe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "pdm_clk_output_en."]
        #[inline(always)]
        pub const fn set_pdm_clk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "asserted to bypass the pdm clock divider."]
        #[must_use]
        #[inline(always)]
        pub const fn pdm_clk_div_bypass(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to bypass the pdm clock divider."]
        #[inline(always)]
        pub const fn set_pdm_clk_div_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)."]
        #[must_use]
        #[inline(always)]
        pub const fn pdm_clk_hfdiv(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[doc = "The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)."]
        #[inline(always)]
        pub const fn set_pdm_clk_hfdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[doc = "Capture cycle delay>=0, should be less than PDM_CLK_HFDIV."]
        #[must_use]
        #[inline(always)]
        pub const fn capt_dly(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[doc = "Capture cycle delay>=0, should be less than PDM_CLK_HFDIV."]
        #[inline(always)]
        pub const fn set_capt_dly(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
        }
        #[doc = "Error interrupt enable This bit controls the generation of an interrupt when an error condition (CIC saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn cic_sat_err_ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable This bit controls the generation of an interrupt when an error condition (CIC saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled."]
        #[inline(always)]
        pub const fn set_cic_sat_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CIC overload error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cic_ovld_err_ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CIC overload error interrupt enable."]
        #[inline(always)]
        pub const fn set_cic_ovld_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "output fifo overflow error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ofifo_ovfl_err_ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "output fifo overflow error interrupt enable."]
        #[inline(always)]
        pub const fn set_ofifo_ovfl_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "asserted if the falling edge of the ref fclk from DAO is the start of a new frame. This is used to to align DAO feedback signal."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_fedge(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "asserted if the falling edge of the ref fclk from DAO is the start of a new frame. This is used to to align DAO feedback signal."]
        #[inline(always)]
        pub const fn set_sof_fedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "software reset the module. Self-clear."]
        #[must_use]
        #[inline(always)]
        pub const fn sftrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "software reset the module. Self-clear."]
        #[inline(always)]
        pub const fn set_sftrst(&mut self, val: bool) {
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
                .field("pdm_clk_oe", &self.pdm_clk_oe())
                .field("pdm_clk_div_bypass", &self.pdm_clk_div_bypass())
                .field("pdm_clk_hfdiv", &self.pdm_clk_hfdiv())
                .field("capt_dly", &self.capt_dly())
                .field("cic_sat_err_ie", &self.cic_sat_err_ie())
                .field("cic_ovld_err_ie", &self.cic_ovld_err_ie())
                .field("ofifo_ovfl_err_ie", &self.ofifo_ovfl_err_ie())
                .field("sof_fedge", &self.sof_fedge())
                .field("sftrst", &self.sftrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl {{ pdm_clk_oe: {=bool:?}, pdm_clk_div_bypass: {=bool:?}, pdm_clk_hfdiv: {=u8:?}, capt_dly: {=u8:?}, cic_sat_err_ie: {=bool:?}, cic_ovld_err_ie: {=bool:?}, ofifo_ovfl_err_ie: {=bool:?}, sof_fedge: {=bool:?}, sftrst: {=bool:?} }}" , self . pdm_clk_oe () , self . pdm_clk_div_bypass () , self . pdm_clk_hfdiv () , self . capt_dly () , self . cic_sat_err_ie () , self . cic_ovld_err_ie () , self . ofifo_ovfl_err_ie () , self . sof_fedge () , self . sftrst ())
        }
    }
    #[doc = "Run Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Run(pub u32);
    impl Run {
        #[doc = "Asserted to enable the module."]
        #[must_use]
        #[inline(always)]
        pub const fn pdm_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enable the module."]
        #[inline(always)]
        pub const fn set_pdm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
                .field("pdm_en", &self.pdm_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Run {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Run {{ pdm_en: {=bool:?} }}", self.pdm_en())
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct St(pub u32);
    impl St {
        #[doc = "CIC saturation. Write 1 clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cic_sat_err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CIC saturation. Write 1 clear."]
        #[inline(always)]
        pub const fn set_cic_sat_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CIC overload error. write 1 clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cic_ovld_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CIC overload error. write 1 clear."]
        #[inline(always)]
        pub const fn set_cic_ovld_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "output fifo overflow error. The reason may be sampling frequency mismatch, either fast or slow."]
        #[must_use]
        #[inline(always)]
        pub const fn ofifo_ovfl_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "output fifo overflow error. The reason may be sampling frequency mismatch, either fast or slow."]
        #[inline(always)]
        pub const fn set_ofifo_ovfl_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("ofifo_ovfl_err", &self.ofifo_ovfl_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for St {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "St {{ cic_sat_err: {=bool:?}, cic_ovld_err: {=bool:?}, ofifo_ovfl_err: {=bool:?} }}" , self . cic_sat_err () , self . cic_ovld_err () , self . ofifo_ovfl_err ())
        }
    }
}
pub mod vals {
    #[doc = "Sigma delta order."]
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SigmaDeltaOrder {
        #[doc = "7."]
        _7 = 0x0,
        #[doc = "6."]
        _6 = 0x01,
        #[doc = "5."]
        _5 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl SigmaDeltaOrder {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SigmaDeltaOrder {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SigmaDeltaOrder {
        #[inline(always)]
        fn from(val: u8) -> SigmaDeltaOrder {
            SigmaDeltaOrder::from_bits(val)
        }
    }
    impl From<SigmaDeltaOrder> for u8 {
        #[inline(always)]
        fn from(val: SigmaDeltaOrder) -> u8 {
            SigmaDeltaOrder::to_bits(val)
        }
    }
}
