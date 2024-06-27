#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "FFA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffa {
    ptr: *mut u8,
}
unsafe impl Send for Ffa {}
unsafe impl Sync for Ffa {}
impl Ffa {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn fp_ctrl(self) -> crate::common::Reg<regs::FpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn fp_st(self) -> crate::common::Reg<regs::FpSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_ctrl(self) -> crate::common::Reg<regs::OpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_cmd(self) -> crate::common::Reg<regs::OpCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fft_misc(self) -> crate::common::Reg<regs::OpFftMisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fir_misc(self) -> crate::common::Reg<regs::OpFirMisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg0(self) -> crate::common::Reg<regs::OpReg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fir_misc1(self) -> crate::common::Reg<regs::OpFirMisc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg1(self) -> crate::common::Reg<regs::OpReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fft_inrbuf(self) -> crate::common::Reg<regs::OpFftInrbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg2(self) -> crate::common::Reg<regs::OpReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fir_inbuf(self) -> crate::common::Reg<regs::OpFirInbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg3(self) -> crate::common::Reg<regs::OpReg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fft_outrbuf(self) -> crate::common::Reg<regs::OpFftOutrbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fir_coefbuf(self) -> crate::common::Reg<regs::OpFirCoefbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg4(self) -> crate::common::Reg<regs::OpReg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fir_outbuf(self) -> crate::common::Reg<regs::OpFirOutbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg5(self) -> crate::common::Reg<regs::OpReg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg6(self) -> crate::common::Reg<regs::OpReg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg7(self) -> crate::common::Reg<regs::OpReg7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
}
pub mod regs {
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Asserted to enable the module."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enable the module."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "software reset the module if asserted to be 1. EN is only active after this bit is zero."]
        #[inline(always)]
        pub const fn sftrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "software reset the module if asserted to be 1. EN is only active after this bit is zero."]
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
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FpCtrl(pub u32);
    impl FpCtrl {
        #[doc = "The input max exp for float. When used as float input, this field must be configured. The absolute value of input data should be smalller than pow(2, (IN_MAX+1)). So this suggested value is (ceil(log2(fabs(in\\[\\])))-1)."]
        #[inline(always)]
        pub const fn in_max(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The input max exp for float. When used as float input, this field must be configured. The absolute value of input data should be smalller than pow(2, (IN_MAX+1)). So this suggested value is (ceil(log2(fabs(in\\[\\])))-1)."]
        #[inline(always)]
        pub fn set_in_max(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "The output max exp for float. When used as float output, this field must be configured. The absolute value of output data should be smalller than pow(2, (OUT_MAX+1)). So this suggested value is (ceil(log2(fabs(out\\[\\])))-1)."]
        #[inline(always)]
        pub const fn out_max(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "The output max exp for float. When used as float output, this field must be configured. The absolute value of output data should be smalller than pow(2, (OUT_MAX+1)). So this suggested value is (ceil(log2(fabs(out\\[\\])))-1)."]
        #[inline(always)]
        pub fn set_out_max(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "The coef max exp for float. When used as float input, this field must be configured. The absolute value of coefficients should be smalller than pow(2, (COEF_MAX+1)). So this suggested value is (ceil(log2(fabs(coef\\[\\])))-1)."]
        #[inline(always)]
        pub const fn coef_max(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "The coef max exp for float. When used as float input, this field must be configured. The absolute value of coefficients should be smalller than pow(2, (COEF_MAX+1)). So this suggested value is (ceil(log2(fabs(coef\\[\\])))-1)."]
        #[inline(always)]
        pub fn set_coef_max(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Asserted to use biased exp as exp input and exp output."]
        #[inline(always)]
        pub const fn opt_bias_exp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to use biased exp as exp input and exp output."]
        #[inline(always)]
        pub fn set_opt_bias_exp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "2'b00: exp for input data 2'b01: exp for output data 2'b10: exp for coef data."]
        #[inline(always)]
        pub const fn exp_st_sel(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "2'b00: exp for input data 2'b01: exp for output data 2'b10: exp for coef data."]
        #[inline(always)]
        pub fn set_exp_st_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "COEF_NAN interrupt enable."]
        #[inline(always)]
        pub const fn coef_nan_ie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "COEF_NAN interrupt enable."]
        #[inline(always)]
        pub fn set_coef_nan_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "IN_NAN interrupt enable."]
        #[inline(always)]
        pub const fn in_nan_ie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "IN_NAN interrupt enable."]
        #[inline(always)]
        pub fn set_in_nan_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "COEF_SAT interrupt enable."]
        #[inline(always)]
        pub const fn coef_sat_ie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "COEF_SAT interrupt enable."]
        #[inline(always)]
        pub fn set_coef_sat_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "IN_SAT interrupt enable."]
        #[inline(always)]
        pub const fn in_sat_ie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "IN_SAT interrupt enable."]
        #[inline(always)]
        pub fn set_in_sat_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FpCtrl {
        #[inline(always)]
        fn default() -> FpCtrl {
            FpCtrl(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FpSt(pub u32);
    impl FpSt {
        #[doc = "The min exp for float."]
        #[inline(always)]
        pub const fn exp_min(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The min exp for float."]
        #[inline(always)]
        pub fn set_exp_min(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "The max exp for float."]
        #[inline(always)]
        pub const fn exp_max(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "The max exp for float."]
        #[inline(always)]
        pub fn set_exp_max(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "COEF_NAN found."]
        #[inline(always)]
        pub const fn coef_nan(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "COEF_NAN found."]
        #[inline(always)]
        pub fn set_coef_nan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "IN_NAN found."]
        #[inline(always)]
        pub const fn in_nan(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "IN_NAN found."]
        #[inline(always)]
        pub fn set_in_nan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "the float coef is saturated when converted from float to fix due to small FLT_CTRL\\[COEF_MAX\\]."]
        #[inline(always)]
        pub const fn coef_sat(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "the float coef is saturated when converted from float to fix due to small FLT_CTRL\\[COEF_MAX\\]."]
        #[inline(always)]
        pub fn set_coef_sat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "the float input is saturated when converted from float to fix due to small FLT_CTRL\\[IN_MAX\\]."]
        #[inline(always)]
        pub const fn in_sat(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "the float input is saturated when converted from float to fix due to small FLT_CTRL\\[IN_MAX\\]."]
        #[inline(always)]
        pub fn set_in_sat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FpSt {
        #[inline(always)]
        fn default() -> FpSt {
            FpSt(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "Indicate that operation cmd is done, and data are available in system memory."]
        #[inline(always)]
        pub const fn op_cmd_done(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Indicate that operation cmd is done, and data are available in system memory."]
        #[inline(always)]
        pub fn set_op_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Indicate that next command sequence is already read into the module."]
        #[inline(always)]
        pub const fn nxt_cmd_rd_done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Indicate that next command sequence is already read into the module."]
        #[inline(always)]
        pub fn set_nxt_cmd_rd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable Data Read Error interrupt."]
        #[inline(always)]
        pub const fn rd_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Data Read Error interrupt."]
        #[inline(always)]
        pub fn set_rd_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable Read Bus Error for NXT DATA interrupt."]
        #[inline(always)]
        pub const fn rd_nxt_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Read Bus Error for NXT DATA interrupt."]
        #[inline(always)]
        pub fn set_rd_nxt_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable Data Write Error interrupt."]
        #[inline(always)]
        pub const fn wr_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Data Write Error interrupt."]
        #[inline(always)]
        pub fn set_wr_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FFT Overflow Err."]
        #[inline(always)]
        pub const fn fft_ov(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FFT Overflow Err."]
        #[inline(always)]
        pub fn set_fft_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FIR Overflow err."]
        #[inline(always)]
        pub const fn fir_ov(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIR Overflow err."]
        #[inline(always)]
        pub fn set_fir_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Reserved."]
        #[inline(always)]
        pub const fn wrsv1(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Reserved."]
        #[inline(always)]
        pub fn set_wrsv1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpCmd(pub u32);
    impl OpCmd {
        #[doc = "The length of nxt commands in 32-bit words."]
        #[inline(always)]
        pub const fn nxt_cmd_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The length of nxt commands in 32-bit words."]
        #[inline(always)]
        pub fn set_nxt_cmd_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Input data type: 0:Real Q31, 1:Real Q15, 2:Complex Q31, 3:Complex Q15 4:complex sp float 5: real sp float."]
        #[inline(always)]
        pub const fn ind_type(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "Input data type: 0:Real Q31, 1:Real Q15, 2:Complex Q31, 3:Complex Q15 4:complex sp float 5: real sp float."]
        #[inline(always)]
        pub fn set_ind_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "Coef data type (used for FIR): 0:Real Q31, 1:Real Q15, 2:Complex Q31, 3:Complex Q15 4:complex sp float 5: real sp float."]
        #[inline(always)]
        pub const fn coef_type(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Coef data type (used for FIR): 0:Real Q31, 1:Real Q15, 2:Complex Q31, 3:Complex Q15 4:complex sp float 5: real sp float."]
        #[inline(always)]
        pub fn set_coef_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Output data type: 0:Real Q31, 1:Real Q15, 2:Complex Q31, 3:Complex Q15 4:complex sp float 5: real sp float."]
        #[inline(always)]
        pub const fn outd_type(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[doc = "Output data type: 0:Real Q31, 1:Real Q15, 2:Complex Q31, 3:Complex Q15 4:complex sp float 5: real sp float."]
        #[inline(always)]
        pub fn set_outd_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[doc = "The Command Used: 0: FIR 2: FFT Others: Reserved."]
        #[inline(always)]
        pub const fn cmd(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x3f;
            val as u8
        }
        #[doc = "The Command Used: 0: FIR 2: FFT Others: Reserved."]
        #[inline(always)]
        pub fn set_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
        }
        #[doc = "asserted to have conjuate value for coefs in computation."]
        #[inline(always)]
        pub const fn conj_c(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to have conjuate value for coefs in computation."]
        #[inline(always)]
        pub fn set_conj_c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for OpCmd {
        #[inline(always)]
        fn default() -> OpCmd {
            OpCmd(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpCtrl(pub u32);
    impl OpCtrl {
        #[doc = "Whether CUR_CMD is enabled. Asserted to enable the CUR_CMD."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Whether CUR_CMD is enabled. Asserted to enable the CUR_CMD."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Whether NXT_CMD is enabled. Asserted to enable the NXT_CMD when CUR_CMD is done, or CUR_CMD is not enabled.."]
        #[inline(always)]
        pub const fn nxt_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Whether NXT_CMD is enabled. Asserted to enable the NXT_CMD when CUR_CMD is done, or CUR_CMD is not enabled.."]
        #[inline(always)]
        pub fn set_nxt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "The address for the next command. It will be processed after CUR_CMD is executed and done.."]
        #[inline(always)]
        pub const fn nxt_addr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "The address for the next command. It will be processed after CUR_CMD is executed and done.."]
        #[inline(always)]
        pub fn set_nxt_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for OpCtrl {
        #[inline(always)]
        fn default() -> OpCtrl {
            OpCtrl(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFftInrbuf(pub u32);
    impl OpFftInrbuf {
        #[doc = "The input (real) data buffer pointer."]
        #[inline(always)]
        pub const fn loc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The input (real) data buffer pointer."]
        #[inline(always)]
        pub fn set_loc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpFftInrbuf {
        #[inline(always)]
        fn default() -> OpFftInrbuf {
            OpFftInrbuf(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFftMisc(pub u32);
    impl OpFftMisc {
        #[doc = "Memory block for indata. Should be assigned as 0."]
        #[inline(always)]
        pub const fn ind_blk(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Memory block for indata. Should be assigned as 0."]
        #[inline(always)]
        pub fn set_ind_blk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Memory block for indata. Should be assigned as 1."]
        #[inline(always)]
        pub const fn tmp_blk(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Memory block for indata. Should be assigned as 1."]
        #[inline(always)]
        pub fn set_tmp_blk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Asserted to indicate IFFT."]
        #[inline(always)]
        pub const fn ifft(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to indicate IFFT."]
        #[inline(always)]
        pub fn set_ifft(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FFT length 0:8, ..., n:2^(3+n)."]
        #[inline(always)]
        pub const fn fft_len(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[doc = "FFT length 0:8, ..., n:2^(3+n)."]
        #[inline(always)]
        pub fn set_fft_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
        }
    }
    impl Default for OpFftMisc {
        #[inline(always)]
        fn default() -> OpFftMisc {
            OpFftMisc(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFftOutrbuf(pub u32);
    impl OpFftOutrbuf {
        #[doc = "The output (real) data buffer pointer."]
        #[inline(always)]
        pub const fn loc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The output (real) data buffer pointer."]
        #[inline(always)]
        pub fn set_loc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpFftOutrbuf {
        #[inline(always)]
        fn default() -> OpFftOutrbuf {
            OpFftOutrbuf(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFirCoefbuf(pub u32);
    impl OpFirCoefbuf {
        #[doc = "The coef buf pointer."]
        #[inline(always)]
        pub const fn loc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The coef buf pointer."]
        #[inline(always)]
        pub fn set_loc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpFirCoefbuf {
        #[inline(always)]
        fn default() -> OpFirCoefbuf {
            OpFirCoefbuf(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFirInbuf(pub u32);
    impl OpFirInbuf {
        #[doc = "The input data buffer pointer."]
        #[inline(always)]
        pub const fn loc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The input data buffer pointer."]
        #[inline(always)]
        pub fn set_loc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpFirInbuf {
        #[inline(always)]
        fn default() -> OpFirInbuf {
            OpFirInbuf(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFirMisc(pub u32);
    impl OpFirMisc {
        #[doc = "Length of FIR coefs （max 256）."]
        #[inline(always)]
        pub const fn fir_coef_taps(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Length of FIR coefs （max 256）."]
        #[inline(always)]
        pub fn set_fir_coef_taps(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for OpFirMisc {
        #[inline(always)]
        fn default() -> OpFirMisc {
            OpFirMisc(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFirMisc1(pub u32);
    impl OpFirMisc1 {
        #[doc = "The input data data length."]
        #[inline(always)]
        pub const fn fir_data_taps(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The input data data length."]
        #[inline(always)]
        pub fn set_fir_data_taps(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Should be assigned as 2."]
        #[inline(always)]
        pub const fn ind_mem_blk(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Should be assigned as 2."]
        #[inline(always)]
        pub fn set_ind_mem_blk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Should be assigned as 1."]
        #[inline(always)]
        pub const fn coef_mem_blk(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Should be assigned as 1."]
        #[inline(always)]
        pub fn set_coef_mem_blk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "Should be assigned as 0."]
        #[inline(always)]
        pub const fn outd_mem_blk(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Should be assigned as 0."]
        #[inline(always)]
        pub fn set_outd_mem_blk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for OpFirMisc1 {
        #[inline(always)]
        fn default() -> OpFirMisc1 {
            OpFirMisc1(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFirOutbuf(pub u32);
    impl OpFirOutbuf {
        #[doc = "The output data buffer pointer. The length of the output buffer should be （FIR_DATA_TAPS - FIR_COEF_TAPS + 1）."]
        #[inline(always)]
        pub const fn loc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The output data buffer pointer. The length of the output buffer should be （FIR_DATA_TAPS - FIR_COEF_TAPS + 1）."]
        #[inline(always)]
        pub fn set_loc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpFirOutbuf {
        #[inline(always)]
        fn default() -> OpFirOutbuf {
            OpFirOutbuf(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg0(pub u32);
    impl OpReg0 {
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg0 {
        #[inline(always)]
        fn default() -> OpReg0 {
            OpReg0(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg1(pub u32);
    impl OpReg1 {
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg1 {
        #[inline(always)]
        fn default() -> OpReg1 {
            OpReg1(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg2(pub u32);
    impl OpReg2 {
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg2 {
        #[inline(always)]
        fn default() -> OpReg2 {
            OpReg2(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg3(pub u32);
    impl OpReg3 {
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg3 {
        #[inline(always)]
        fn default() -> OpReg3 {
            OpReg3(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg4(pub u32);
    impl OpReg4 {
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg4 {
        #[inline(always)]
        fn default() -> OpReg4 {
            OpReg4(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg5(pub u32);
    impl OpReg5 {
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg5 {
        #[inline(always)]
        fn default() -> OpReg5 {
            OpReg5(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg6(pub u32);
    impl OpReg6 {
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg6 {
        #[inline(always)]
        fn default() -> OpReg6 {
            OpReg6(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg7(pub u32);
    impl OpReg7 {
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg7 {
        #[inline(always)]
        fn default() -> OpReg7 {
            OpReg7(0)
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Indicate that operation cmd is done, and data are available in system memory."]
        #[inline(always)]
        pub const fn op_cmd_done(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Indicate that operation cmd is done, and data are available in system memory."]
        #[inline(always)]
        pub fn set_op_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Indicate that next command sequence is already read into the module."]
        #[inline(always)]
        pub const fn nxt_cmd_rd_done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Indicate that next command sequence is already read into the module."]
        #[inline(always)]
        pub fn set_nxt_cmd_rd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AXI Data Read Error."]
        #[inline(always)]
        pub const fn rd_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Data Read Error."]
        #[inline(always)]
        pub fn set_rd_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "AXI Read Bus Error for NXT DATA."]
        #[inline(always)]
        pub const fn rd_nxt_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Read Bus Error for NXT DATA."]
        #[inline(always)]
        pub fn set_rd_nxt_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AXI Data Write Error."]
        #[inline(always)]
        pub const fn wr_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Data Write Error."]
        #[inline(always)]
        pub fn set_wr_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FFT Overflow Err."]
        #[inline(always)]
        pub const fn fft_ov(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FFT Overflow Err."]
        #[inline(always)]
        pub fn set_fft_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FIR Overflow err."]
        #[inline(always)]
        pub const fn fir_ov(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIR Overflow err."]
        #[inline(always)]
        pub fn set_fir_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Ored together by ( FP_ST\\[IN_SAT\\]
& FP_CTRL\\[IN_SAT_IE\\]
) | ( FP_ST\\[COEF_SAT\\]
& FP_CTRL\\[COEF_SAT_IE\\]
)."]
        #[inline(always)]
        pub const fn fp_sat(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Ored together by ( FP_ST\\[IN_SAT\\]
& FP_CTRL\\[IN_SAT_IE\\]
) | ( FP_ST\\[COEF_SAT\\]
& FP_CTRL\\[COEF_SAT_IE\\]
)."]
        #[inline(always)]
        pub fn set_fp_sat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Ored together by ( FP_ST\\[IN_NAN\\]
& FP_CTRL\\[IN_NAN_IE\\]
) | ( FP_ST\\[COEF_NAN\\]
& FP_CTRL\\[COEF_NAN_IE\\]
)."]
        #[inline(always)]
        pub const fn fp_nan(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Ored together by ( FP_ST\\[IN_NAN\\]
& FP_CTRL\\[IN_NAN_IE\\]
) | ( FP_ST\\[COEF_NAN\\]
& FP_CTRL\\[COEF_NAN_IE\\]
)."]
        #[inline(always)]
        pub fn set_fp_nan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
}
