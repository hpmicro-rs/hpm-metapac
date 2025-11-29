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
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch_cfg(self) -> crate::common::Reg<regs::ChCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CIC configuration register."]
    #[inline(always)]
    pub const fn cic_cfg(self) -> crate::common::Reg<regs::CicCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "In Buf Control Register."]
    #[inline(always)]
    pub const fn ctrl_inbuf(self) -> crate::common::Reg<regs::CtrlInbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Filter 0 Control Register."]
    #[inline(always)]
    pub const fn ctrl_filt0(self) -> crate::common::Reg<regs::CtrlFilt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Filter 1 Control Register."]
    #[inline(always)]
    pub const fn ctrl_filt1(self) -> crate::common::Reg<regs::CtrlFilt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Run Register."]
    #[inline(always)]
    pub const fn run(self) -> crate::common::Reg<regs::Run, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Memory Access Address."]
    #[inline(always)]
    pub const fn memaddr(self) -> crate::common::Reg<regs::Memaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Memory Access Data."]
    #[inline(always)]
    pub const fn memdata(self) -> crate::common::Reg<regs::Memdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "HPF A Coef Register."]
    #[inline(always)]
    pub const fn hpf_ma(self) -> crate::common::Reg<regs::HpfMa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "HPF B Coef Register."]
    #[inline(always)]
    pub const fn hpf_b(self) -> crate::common::Reg<regs::HpfB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
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
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChCfg(pub u32);
    impl ChCfg {
        #[doc = "Type of Channel 0 2'b00: dec-by-3 wiith filter type0 (CIC Compenstation+norm filter) 2'b01: dec-by-3 with filter type 1 (No CIC compenstation, only norm filter)."]
        #[must_use]
        #[inline(always)]
        pub const fn ch0_type(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Type of Channel 0 2'b00: dec-by-3 wiith filter type0 (CIC Compenstation+norm filter) 2'b01: dec-by-3 with filter type 1 (No CIC compenstation, only norm filter)."]
        #[inline(always)]
        pub const fn set_ch0_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ch1_type(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ch1_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ch2_type(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ch2_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ch3_type(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ch3_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ch4_type(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ch4_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ch5_type(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ch5_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ch6_type(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ch6_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ch7_type(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ch7_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ch8_type(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ch8_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn ch9_type(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_ch9_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
    }
    impl Default for ChCfg {
        #[inline(always)]
        fn default() -> ChCfg {
            ChCfg(0)
        }
    }
    impl core::fmt::Debug for ChCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ChCfg")
                .field("ch0_type", &self.ch0_type())
                .field("ch1_type", &self.ch1_type())
                .field("ch2_type", &self.ch2_type())
                .field("ch3_type", &self.ch3_type())
                .field("ch4_type", &self.ch4_type())
                .field("ch5_type", &self.ch5_type())
                .field("ch6_type", &self.ch6_type())
                .field("ch7_type", &self.ch7_type())
                .field("ch8_type", &self.ch8_type())
                .field("ch9_type", &self.ch9_type())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ChCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ChCfg {{ ch0_type: {=u8:?}, ch1_type: {=u8:?}, ch2_type: {=u8:?}, ch3_type: {=u8:?}, ch4_type: {=u8:?}, ch5_type: {=u8:?}, ch6_type: {=u8:?}, ch7_type: {=u8:?}, ch8_type: {=u8:?}, ch9_type: {=u8:?} }}" , self . ch0_type () , self . ch1_type () , self . ch2_type () , self . ch3_type () , self . ch4_type () , self . ch5_type () , self . ch6_type () , self . ch7_type () , self . ch8_type () , self . ch9_type ())
        }
    }
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
        #[doc = "pdm high pass filter enable. This order-1 HPF only applies to the PDM mic data."]
        #[must_use]
        #[inline(always)]
        pub const fn hpf_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "pdm high pass filter enable. This order-1 HPF only applies to the PDM mic data."]
        #[inline(always)]
        pub const fn set_hpf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
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
        #[doc = "decimation rate after CIC. Now it is forced to be 3."]
        #[must_use]
        #[inline(always)]
        pub const fn dec_aft_cic(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "decimation rate after CIC. Now it is forced to be 3."]
        #[inline(always)]
        pub const fn set_dec_aft_cic(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
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
        #[doc = "data accessed out of boundary error interruput enable. The error happens when the module cannot calculate the enough number of data in time."]
        #[must_use]
        #[inline(always)]
        pub const fn filt_crx_err_ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "data accessed out of boundary error interruput enable. The error happens when the module cannot calculate the enough number of data in time."]
        #[inline(always)]
        pub const fn set_filt_crx_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Asserted to use Coef RAM instead of Coef ROM."]
        #[must_use]
        #[inline(always)]
        pub const fn use_coef_ram(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to use Coef RAM instead of Coef ROM."]
        #[inline(always)]
        pub const fn set_use_coef_ram(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
                .field("hpf_en", &self.hpf_en())
                .field("pdm_clk_oe", &self.pdm_clk_oe())
                .field("pdm_clk_div_bypass", &self.pdm_clk_div_bypass())
                .field("pdm_clk_hfdiv", &self.pdm_clk_hfdiv())
                .field("capt_dly", &self.capt_dly())
                .field("dec_aft_cic", &self.dec_aft_cic())
                .field("cic_sat_err_ie", &self.cic_sat_err_ie())
                .field("cic_ovld_err_ie", &self.cic_ovld_err_ie())
                .field("ofifo_ovfl_err_ie", &self.ofifo_ovfl_err_ie())
                .field("filt_crx_err_ie", &self.filt_crx_err_ie())
                .field("use_coef_ram", &self.use_coef_ram())
                .field("sof_fedge", &self.sof_fedge())
                .field("sftrst", &self.sftrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ctrl {{ hpf_en: {=bool:?}, pdm_clk_oe: {=bool:?}, pdm_clk_div_bypass: {=bool:?}, pdm_clk_hfdiv: {=u8:?}, capt_dly: {=u8:?}, dec_aft_cic: {=u8:?}, cic_sat_err_ie: {=bool:?}, cic_ovld_err_ie: {=bool:?}, ofifo_ovfl_err_ie: {=bool:?}, filt_crx_err_ie: {=bool:?}, use_coef_ram: {=bool:?}, sof_fedge: {=bool:?}, sftrst: {=bool:?} }}" , self . hpf_en () , self . pdm_clk_oe () , self . pdm_clk_div_bypass () , self . pdm_clk_hfdiv () , self . capt_dly () , self . dec_aft_cic () , self . cic_sat_err_ie () , self . cic_ovld_err_ie () , self . ofifo_ovfl_err_ie () , self . filt_crx_err_ie () , self . use_coef_ram () , self . sof_fedge () , self . sftrst ())
        }
    }
    #[doc = "Filter 0 Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlFilt0(pub u32);
    impl CtrlFilt0 {
        #[doc = "Starting address of Coef of filter type 2'b00 in coef memory."]
        #[must_use]
        #[inline(always)]
        pub const fn coef_start_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Starting address of Coef of filter type 2'b00 in coef memory."]
        #[inline(always)]
        pub const fn set_coef_start_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Coef length of filter type 2'b00 in coef memory."]
        #[must_use]
        #[inline(always)]
        pub const fn coef_len_m0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Coef length of filter type 2'b00 in coef memory."]
        #[inline(always)]
        pub const fn set_coef_len_m0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for CtrlFilt0 {
        #[inline(always)]
        fn default() -> CtrlFilt0 {
            CtrlFilt0(0)
        }
    }
    impl core::fmt::Debug for CtrlFilt0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CtrlFilt0")
                .field("coef_start_addr", &self.coef_start_addr())
                .field("coef_len_m0", &self.coef_len_m0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CtrlFilt0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CtrlFilt0 {{ coef_start_addr: {=u8:?}, coef_len_m0: {=u8:?} }}",
                self.coef_start_addr(),
                self.coef_len_m0()
            )
        }
    }
    #[doc = "Filter 1 Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlFilt1(pub u32);
    impl CtrlFilt1 {
        #[doc = "Starting address of Coef of filter type 2'b01 in coef memory."]
        #[must_use]
        #[inline(always)]
        pub const fn coef_start_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Starting address of Coef of filter type 2'b01 in coef memory."]
        #[inline(always)]
        pub const fn set_coef_start_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Coef length of filter type 2'b01 in coef memory."]
        #[must_use]
        #[inline(always)]
        pub const fn coef_len_m1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Coef length of filter type 2'b01 in coef memory."]
        #[inline(always)]
        pub const fn set_coef_len_m1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for CtrlFilt1 {
        #[inline(always)]
        fn default() -> CtrlFilt1 {
            CtrlFilt1(0)
        }
    }
    impl core::fmt::Debug for CtrlFilt1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CtrlFilt1")
                .field("coef_start_addr", &self.coef_start_addr())
                .field("coef_len_m1", &self.coef_len_m1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CtrlFilt1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CtrlFilt1 {{ coef_start_addr: {=u8:?}, coef_len_m1: {=u8:?} }}",
                self.coef_start_addr(),
                self.coef_len_m1()
            )
        }
    }
    #[doc = "In Buf Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlInbuf(pub u32);
    impl CtrlInbuf {
        #[doc = "The starting address of channel 0 in filter data buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn start_addr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The starting address of channel 0 in filter data buffer."]
        #[inline(always)]
        pub const fn set_start_addr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "The spacing between starting address of adjacent channels."]
        #[must_use]
        #[inline(always)]
        pub const fn pitch(&self) -> u16 {
            let val = (self.0 >> 11usize) & 0x07ff;
            val as u16
        }
        #[doc = "The spacing between starting address of adjacent channels."]
        #[inline(always)]
        pub const fn set_pitch(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 11usize)) | (((val as u32) & 0x07ff) << 11usize);
        }
        #[doc = "The buf size-1 for each channel."]
        #[must_use]
        #[inline(always)]
        pub const fn max_ptr(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0xff;
            val as u8
        }
        #[doc = "The buf size-1 for each channel."]
        #[inline(always)]
        pub const fn set_max_ptr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 22usize)) | (((val as u32) & 0xff) << 22usize);
        }
    }
    impl Default for CtrlInbuf {
        #[inline(always)]
        fn default() -> CtrlInbuf {
            CtrlInbuf(0)
        }
    }
    impl core::fmt::Debug for CtrlInbuf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CtrlInbuf")
                .field("start_addr", &self.start_addr())
                .field("pitch", &self.pitch())
                .field("max_ptr", &self.max_ptr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CtrlInbuf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CtrlInbuf {{ start_addr: {=u16:?}, pitch: {=u16:?}, max_ptr: {=u8:?} }}",
                self.start_addr(),
                self.pitch(),
                self.max_ptr()
            )
        }
    }
    #[doc = "HPF B Coef Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpfB(pub u32);
    impl HpfB {
        #[doc = "coef B of the Order-1 HPF."]
        #[must_use]
        #[inline(always)]
        pub const fn coef(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "coef B of the Order-1 HPF."]
        #[inline(always)]
        pub const fn set_coef(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HpfB {
        #[inline(always)]
        fn default() -> HpfB {
            HpfB(0)
        }
    }
    impl core::fmt::Debug for HpfB {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HpfB").field("coef", &self.coef()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HpfB {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HpfB {{ coef: {=u32:?} }}", self.coef())
        }
    }
    #[doc = "HPF A Coef Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpfMa(pub u32);
    impl HpfMa {
        #[doc = "Composite value of coef A of the Order-1 HPF."]
        #[must_use]
        #[inline(always)]
        pub const fn coef(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Composite value of coef A of the Order-1 HPF."]
        #[inline(always)]
        pub const fn set_coef(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HpfMa {
        #[inline(always)]
        fn default() -> HpfMa {
            HpfMa(0)
        }
    }
    impl core::fmt::Debug for HpfMa {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HpfMa").field("coef", &self.coef()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HpfMa {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HpfMa {{ coef: {=u32:?} }}", self.coef())
        }
    }
    #[doc = "Memory Access Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Memaddr(pub u32);
    impl Memaddr {
        #[doc = "0--0x0FFFFFFF: COEF_RAM 0x10000000--0x1FFFFFFF: DATA_RAM."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "0--0x0FFFFFFF: COEF_RAM 0x10000000--0x1FFFFFFF: DATA_RAM."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Memaddr {
        #[inline(always)]
        fn default() -> Memaddr {
            Memaddr(0)
        }
    }
    impl core::fmt::Debug for Memaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Memaddr")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Memaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Memaddr {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Memory Access Data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Memdata(pub u32);
    impl Memdata {
        #[doc = "The data write-to/read-from buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The data write-to/read-from buffer."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Memdata {
        #[inline(always)]
        fn default() -> Memdata {
            Memdata(0)
        }
    }
    impl core::fmt::Debug for Memdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Memdata")
                .field("data", &self.data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Memdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Memdata {{ data: {=u32:?} }}", self.data())
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
        #[doc = "data accessed out of boundary error."]
        #[must_use]
        #[inline(always)]
        pub const fn filt_crx_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "data accessed out of boundary error."]
        #[inline(always)]
        pub const fn set_filt_crx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("filt_crx_err", &self.filt_crx_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for St {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "St {{ cic_sat_err: {=bool:?}, cic_ovld_err: {=bool:?}, ofifo_ovfl_err: {=bool:?}, filt_crx_err: {=bool:?} }}" , self . cic_sat_err () , self . cic_ovld_err () , self . ofifo_ovfl_err () , self . filt_crx_err ())
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
