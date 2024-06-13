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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Channel Control Register."]
    #[inline(always)]
    pub const fn ch_ctrl(self) -> crate::common::Reg<regs::ChCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn st(self) -> crate::common::Reg<regs::St, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Channel Configuration Register."]
    #[inline(always)]
    pub const fn ch_cfg(self) -> crate::common::Reg<regs::ChCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "CIC configuration register."]
    #[inline(always)]
    pub const fn cic_cfg(self) -> crate::common::Reg<regs::CicCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "In Buf Control Register."]
    #[inline(always)]
    pub const fn ctrl_inbuf(self) -> crate::common::Reg<regs::CtrlInbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Filter 0 Control Register."]
    #[inline(always)]
    pub const fn ctrl_filt0(self) -> crate::common::Reg<regs::CtrlFilt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Filter 1 Control Register."]
    #[inline(always)]
    pub const fn ctrl_filt1(self) -> crate::common::Reg<regs::CtrlFilt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Run Register."]
    #[inline(always)]
    pub const fn run(self) -> crate::common::Reg<regs::Run, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Memory Access Address."]
    #[inline(always)]
    pub const fn memaddr(self) -> crate::common::Reg<regs::Memaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Memory Access Data."]
    #[inline(always)]
    pub const fn memdata(self) -> crate::common::Reg<regs::Memdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "HPF A Coef Register."]
    #[inline(always)]
    pub const fn hpf_ma(self) -> crate::common::Reg<regs::HpfMa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "HPF B Coef Register."]
    #[inline(always)]
    pub const fn hpf_b(self) -> crate::common::Reg<regs::HpfB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
}
pub mod regs {
    #[doc = "Channel Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChCfg(pub u32);
    impl ChCfg {
        #[doc = "Type of Channel 0 2'b00: dec-by-3 wiith filter type0 (CIC Compenstation+norm filter) 2'b01: dec-by-3 with filter type 1 (No CIC compenstation, only norm filter)."]
        #[inline(always)]
        pub const fn ch0_type(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Type of Channel 0 2'b00: dec-by-3 wiith filter type0 (CIC Compenstation+norm filter) 2'b01: dec-by-3 with filter type 1 (No CIC compenstation, only norm filter)."]
        #[inline(always)]
        pub fn set_ch0_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ch1_type(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ch1_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ch2_type(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ch2_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ch3_type(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ch3_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ch4_type(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ch4_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ch5_type(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ch5_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ch6_type(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ch6_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ch7_type(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ch7_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ch8_type(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ch8_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn ch9_type(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub fn set_ch9_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
    }
    impl Default for ChCfg {
        #[inline(always)]
        fn default() -> ChCfg {
            ChCfg(0)
        }
    }
    #[doc = "Channel Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChCtrl(pub u32);
    impl ChCtrl {
        #[doc = "Asserted to enable the channel. Ch8 & 9 are refs. Ch0-7 are pdm mics."]
        #[inline(always)]
        pub const fn ch_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Asserted to enable the channel. Ch8 & 9 are refs. Ch0-7 are pdm mics."]
        #[inline(always)]
        pub fn set_ch_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
        #[inline(always)]
        pub const fn ch_pol(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
        #[inline(always)]
        pub fn set_ch_pol(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ChCtrl {
        #[inline(always)]
        fn default() -> ChCtrl {
            ChCtrl(0)
        }
    }
    #[doc = "CIC configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CicCfg(pub u32);
    impl CicCfg {
        #[doc = "CIC decimation factor."]
        #[inline(always)]
        pub const fn cic_dec_ratio(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "CIC decimation factor."]
        #[inline(always)]
        pub fn set_cic_dec_ratio(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Sigma_delta_order\\[1:0\\]
2'b00: 7 2'b01: 6 2'b10: 5 Others: unused."]
        #[inline(always)]
        pub const fn sgd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Sigma_delta_order\\[1:0\\]
2'b00: 7 2'b01: 6 2'b10: 5 Others: unused."]
        #[inline(always)]
        pub fn set_sgd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
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
    #[doc = "Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "pdm high pass filter enable. This order-1 HPF only applies to the PDM mic data."]
        #[inline(always)]
        pub const fn hpf_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "pdm high pass filter enable. This order-1 HPF only applies to the PDM mic data."]
        #[inline(always)]
        pub fn set_hpf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "pdm_clk_output_en."]
        #[inline(always)]
        pub const fn pdm_clk_oe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "pdm_clk_output_en."]
        #[inline(always)]
        pub fn set_pdm_clk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "asserted to bypass the pdm clock divider."]
        #[inline(always)]
        pub const fn pdm_clk_div_bypass(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to bypass the pdm clock divider."]
        #[inline(always)]
        pub fn set_pdm_clk_div_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)."]
        #[inline(always)]
        pub const fn pdm_clk_hfdiv(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[doc = "The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)."]
        #[inline(always)]
        pub fn set_pdm_clk_hfdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[doc = "Capture cycle delay>=0, should be less than PDM_CLK_HFDIV."]
        #[inline(always)]
        pub const fn capt_dly(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[doc = "Capture cycle delay>=0, should be less than PDM_CLK_HFDIV."]
        #[inline(always)]
        pub fn set_capt_dly(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
        }
        #[doc = "decimation rate after CIC. Now it is forced to be 3."]
        #[inline(always)]
        pub const fn dec_aft_cic(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "decimation rate after CIC. Now it is forced to be 3."]
        #[inline(always)]
        pub fn set_dec_aft_cic(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Error interrupt enable This bit controls the generation of an interrupt when an error condition (CIC saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled."]
        #[inline(always)]
        pub const fn cic_sat_err_ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable This bit controls the generation of an interrupt when an error condition (CIC saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled."]
        #[inline(always)]
        pub fn set_cic_sat_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CIC overload error interrupt enable."]
        #[inline(always)]
        pub const fn cic_ovld_err_ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CIC overload error interrupt enable."]
        #[inline(always)]
        pub fn set_cic_ovld_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "output fifo overflow error interrupt enable."]
        #[inline(always)]
        pub const fn ofifo_ovfl_err_ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "output fifo overflow error interrupt enable."]
        #[inline(always)]
        pub fn set_ofifo_ovfl_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "data accessed out of boundary error interruput enable. The error happens when the module cannot calculate the enough number of data in time."]
        #[inline(always)]
        pub const fn filt_crx_err_ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "data accessed out of boundary error interruput enable. The error happens when the module cannot calculate the enough number of data in time."]
        #[inline(always)]
        pub fn set_filt_crx_err_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Asserted to use Coef RAM instead of Coef ROM."]
        #[inline(always)]
        pub const fn use_coef_ram(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to use Coef RAM instead of Coef ROM."]
        #[inline(always)]
        pub fn set_use_coef_ram(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "asserted if the falling edge of the ref fclk from DAO is the start of a new frame. This is used to to align DAO feedback signal."]
        #[inline(always)]
        pub const fn sof_fedge(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "asserted if the falling edge of the ref fclk from DAO is the start of a new frame. This is used to to align DAO feedback signal."]
        #[inline(always)]
        pub fn set_sof_fedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "software reset the module. Self-clear."]
        #[inline(always)]
        pub const fn sftrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "software reset the module. Self-clear."]
        #[inline(always)]
        pub fn set_sftrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "Filter 0 Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlFilt0(pub u32);
    impl CtrlFilt0 {
        #[doc = "Starting address of Coef of filter type 2'b00 in coef memory."]
        #[inline(always)]
        pub const fn coef_start_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Starting address of Coef of filter type 2'b00 in coef memory."]
        #[inline(always)]
        pub fn set_coef_start_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Coef length of filter type 2'b00 in coef memory."]
        #[inline(always)]
        pub const fn coef_len_m0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Coef length of filter type 2'b00 in coef memory."]
        #[inline(always)]
        pub fn set_coef_len_m0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for CtrlFilt0 {
        #[inline(always)]
        fn default() -> CtrlFilt0 {
            CtrlFilt0(0)
        }
    }
    #[doc = "Filter 1 Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlFilt1(pub u32);
    impl CtrlFilt1 {
        #[doc = "Starting address of Coef of filter type 2'b01 in coef memory."]
        #[inline(always)]
        pub const fn coef_start_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Starting address of Coef of filter type 2'b01 in coef memory."]
        #[inline(always)]
        pub fn set_coef_start_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Coef length of filter type 2'b01 in coef memory."]
        #[inline(always)]
        pub const fn coef_len_m1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Coef length of filter type 2'b01 in coef memory."]
        #[inline(always)]
        pub fn set_coef_len_m1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for CtrlFilt1 {
        #[inline(always)]
        fn default() -> CtrlFilt1 {
            CtrlFilt1(0)
        }
    }
    #[doc = "In Buf Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlInbuf(pub u32);
    impl CtrlInbuf {
        #[doc = "The starting address of channel 0 in filter data buffer."]
        #[inline(always)]
        pub const fn start_addr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "The starting address of channel 0 in filter data buffer."]
        #[inline(always)]
        pub fn set_start_addr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "The spacing between starting address of adjacent channels."]
        #[inline(always)]
        pub const fn pitch(&self) -> u16 {
            let val = (self.0 >> 11usize) & 0x07ff;
            val as u16
        }
        #[doc = "The spacing between starting address of adjacent channels."]
        #[inline(always)]
        pub fn set_pitch(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 11usize)) | (((val as u32) & 0x07ff) << 11usize);
        }
        #[doc = "The buf size-1 for each channel."]
        #[inline(always)]
        pub const fn max_ptr(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0xff;
            val as u8
        }
        #[doc = "The buf size-1 for each channel."]
        #[inline(always)]
        pub fn set_max_ptr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 22usize)) | (((val as u32) & 0xff) << 22usize);
        }
    }
    impl Default for CtrlInbuf {
        #[inline(always)]
        fn default() -> CtrlInbuf {
            CtrlInbuf(0)
        }
    }
    #[doc = "HPF B Coef Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpfB(pub u32);
    impl HpfB {
        #[doc = "coef B of the Order-1 HPF."]
        #[inline(always)]
        pub const fn coef(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "coef B of the Order-1 HPF."]
        #[inline(always)]
        pub fn set_coef(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HpfB {
        #[inline(always)]
        fn default() -> HpfB {
            HpfB(0)
        }
    }
    #[doc = "HPF A Coef Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpfMa(pub u32);
    impl HpfMa {
        #[doc = "Composite value of coef A of the Order-1 HPF."]
        #[inline(always)]
        pub const fn coef(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Composite value of coef A of the Order-1 HPF."]
        #[inline(always)]
        pub fn set_coef(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HpfMa {
        #[inline(always)]
        fn default() -> HpfMa {
            HpfMa(0)
        }
    }
    #[doc = "Memory Access Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Memaddr(pub u32);
    impl Memaddr {
        #[doc = "0--0x0FFFFFFF: COEF_RAM 0x10000000--0x1FFFFFFF: DATA_RAM."]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "0--0x0FFFFFFF: COEF_RAM 0x10000000--0x1FFFFFFF: DATA_RAM."]
        #[inline(always)]
        pub fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Memaddr {
        #[inline(always)]
        fn default() -> Memaddr {
            Memaddr(0)
        }
    }
    #[doc = "Memory Access Data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Memdata(pub u32);
    impl Memdata {
        #[doc = "The data write-to/read-from buffer."]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The data write-to/read-from buffer."]
        #[inline(always)]
        pub fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Memdata {
        #[inline(always)]
        fn default() -> Memdata {
            Memdata(0)
        }
    }
    #[doc = "Run Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Run(pub u32);
    impl Run {
        #[doc = "Asserted to enable the module."]
        #[inline(always)]
        pub const fn pdm_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enable the module."]
        #[inline(always)]
        pub fn set_pdm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Run {
        #[inline(always)]
        fn default() -> Run {
            Run(0)
        }
    }
    #[doc = "Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct St(pub u32);
    impl St {
        #[doc = "CIC saturation. Write 1 clear."]
        #[inline(always)]
        pub const fn cic_sat_err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CIC saturation. Write 1 clear."]
        #[inline(always)]
        pub fn set_cic_sat_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CIC overload error. write 1 clear."]
        #[inline(always)]
        pub const fn cic_ovld_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CIC overload error. write 1 clear."]
        #[inline(always)]
        pub fn set_cic_ovld_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "output fifo overflow error. The reason may be sampling frequency mismatch, either fast or slow."]
        #[inline(always)]
        pub const fn ofifo_ovfl_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "output fifo overflow error. The reason may be sampling frequency mismatch, either fast or slow."]
        #[inline(always)]
        pub fn set_ofifo_ovfl_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "data accessed out of boundary error."]
        #[inline(always)]
        pub const fn filt_crx_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "data accessed out of boundary error."]
        #[inline(always)]
        pub fn set_filt_crx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for St {
        #[inline(always)]
        fn default() -> St {
            St(0)
        }
    }
}
