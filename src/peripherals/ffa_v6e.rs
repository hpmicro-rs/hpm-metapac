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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn fp_ctrl(self) -> crate::common::Reg<regs::FpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn fp_st(self) -> crate::common::Reg<regs::FpSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_ctrl(self) -> crate::common::Reg<regs::OpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_cmd(self) -> crate::common::Reg<regs::OpCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fft_misc(self) -> crate::common::Reg<regs::OpFftMisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fir_misc(self) -> crate::common::Reg<regs::OpFirMisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg0(self) -> crate::common::Reg<regs::OpReg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fir_misc1(self) -> crate::common::Reg<regs::OpFirMisc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg1(self) -> crate::common::Reg<regs::OpReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fft_inrbuf(self) -> crate::common::Reg<regs::OpFftInrbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg2(self) -> crate::common::Reg<regs::OpReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fir_inbuf(self) -> crate::common::Reg<regs::OpFirInbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg3(self) -> crate::common::Reg<regs::OpReg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fft_outrbuf(self) -> crate::common::Reg<regs::OpFftOutrbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fir_coefbuf(self) -> crate::common::Reg<regs::OpFirCoefbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg4(self) -> crate::common::Reg<regs::OpReg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_fir_outbuf(self) -> crate::common::Reg<regs::OpFirOutbuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg5(self) -> crate::common::Reg<regs::OpReg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg6(self) -> crate::common::Reg<regs::OpReg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn op_reg7(self) -> crate::common::Reg<regs::OpReg7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
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
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Asserted to enable the module."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to enable the module."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "software reset the module if asserted to be 1. EN is only active after this bit is zero."]
        #[must_use]
        #[inline(always)]
        pub const fn sftrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "software reset the module if asserted to be 1. EN is only active after this bit is zero."]
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
                .field("en", &self.en())
                .field("sftrst", &self.sftrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ctrl {{ en: {=bool:?}, sftrst: {=bool:?} }}",
                self.en(),
                self.sftrst()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FpCtrl(pub u32);
    impl FpCtrl {
        #[doc = "The input max exp for float. When used as float input, this field must be configured. The absolute value of input data should be smalller than pow(2, (IN_MAX+1)). So this suggested value is (ceil(log2(fabs(in\\[\\])))-1)."]
        #[must_use]
        #[inline(always)]
        pub const fn in_max(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The input max exp for float. When used as float input, this field must be configured. The absolute value of input data should be smalller than pow(2, (IN_MAX+1)). So this suggested value is (ceil(log2(fabs(in\\[\\])))-1)."]
        #[inline(always)]
        pub const fn set_in_max(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "The output max exp for float. When used as float output, this field must be configured. The absolute value of output data should be smalller than pow(2, (OUT_MAX+1)). So this suggested value is (ceil(log2(fabs(out\\[\\])))-1)."]
        #[must_use]
        #[inline(always)]
        pub const fn out_max(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "The output max exp for float. When used as float output, this field must be configured. The absolute value of output data should be smalller than pow(2, (OUT_MAX+1)). So this suggested value is (ceil(log2(fabs(out\\[\\])))-1)."]
        #[inline(always)]
        pub const fn set_out_max(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "The coef max exp for float. When used as float input, this field must be configured. The absolute value of coefficients should be smalller than pow(2, (COEF_MAX+1)). So this suggested value is (ceil(log2(fabs(coef\\[\\])))-1)."]
        #[must_use]
        #[inline(always)]
        pub const fn coef_max(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "The coef max exp for float. When used as float input, this field must be configured. The absolute value of coefficients should be smalller than pow(2, (COEF_MAX+1)). So this suggested value is (ceil(log2(fabs(coef\\[\\])))-1)."]
        #[inline(always)]
        pub const fn set_coef_max(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Asserted to use biased exp as exp input and exp output."]
        #[must_use]
        #[inline(always)]
        pub const fn opt_bias_exp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to use biased exp as exp input and exp output."]
        #[inline(always)]
        pub const fn set_opt_bias_exp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "2'b00: exp for input data 2'b01: exp for output data 2'b10: exp for coef data."]
        #[must_use]
        #[inline(always)]
        pub const fn exp_st_sel(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "2'b00: exp for input data 2'b01: exp for output data 2'b10: exp for coef data."]
        #[inline(always)]
        pub const fn set_exp_st_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "COEF_NAN interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn coef_nan_ie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "COEF_NAN interrupt enable."]
        #[inline(always)]
        pub const fn set_coef_nan_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "IN_NAN interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn in_nan_ie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "IN_NAN interrupt enable."]
        #[inline(always)]
        pub const fn set_in_nan_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "COEF_SAT interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn coef_sat_ie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "COEF_SAT interrupt enable."]
        #[inline(always)]
        pub const fn set_coef_sat_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "IN_SAT interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn in_sat_ie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "IN_SAT interrupt enable."]
        #[inline(always)]
        pub const fn set_in_sat_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FpCtrl {
        #[inline(always)]
        fn default() -> FpCtrl {
            FpCtrl(0)
        }
    }
    impl core::fmt::Debug for FpCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FpCtrl")
                .field("in_max", &self.in_max())
                .field("out_max", &self.out_max())
                .field("coef_max", &self.coef_max())
                .field("opt_bias_exp", &self.opt_bias_exp())
                .field("exp_st_sel", &self.exp_st_sel())
                .field("coef_nan_ie", &self.coef_nan_ie())
                .field("in_nan_ie", &self.in_nan_ie())
                .field("coef_sat_ie", &self.coef_sat_ie())
                .field("in_sat_ie", &self.in_sat_ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FpCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FpCtrl {{ in_max: {=u8:?}, out_max: {=u8:?}, coef_max: {=u8:?}, opt_bias_exp: {=bool:?}, exp_st_sel: {=u8:?}, coef_nan_ie: {=bool:?}, in_nan_ie: {=bool:?}, coef_sat_ie: {=bool:?}, in_sat_ie: {=bool:?} }}" , self . in_max () , self . out_max () , self . coef_max () , self . opt_bias_exp () , self . exp_st_sel () , self . coef_nan_ie () , self . in_nan_ie () , self . coef_sat_ie () , self . in_sat_ie ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FpSt(pub u32);
    impl FpSt {
        #[doc = "The min exp for float."]
        #[must_use]
        #[inline(always)]
        pub const fn exp_min(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The min exp for float."]
        #[inline(always)]
        pub const fn set_exp_min(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "The max exp for float."]
        #[must_use]
        #[inline(always)]
        pub const fn exp_max(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "The max exp for float."]
        #[inline(always)]
        pub const fn set_exp_max(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "COEF_NAN found."]
        #[must_use]
        #[inline(always)]
        pub const fn coef_nan(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "COEF_NAN found."]
        #[inline(always)]
        pub const fn set_coef_nan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "IN_NAN found."]
        #[must_use]
        #[inline(always)]
        pub const fn in_nan(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "IN_NAN found."]
        #[inline(always)]
        pub const fn set_in_nan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "the float coef is saturated when converted from float to fix due to small FLT_CTRL\\[COEF_MAX\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn coef_sat(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "the float coef is saturated when converted from float to fix due to small FLT_CTRL\\[COEF_MAX\\]."]
        #[inline(always)]
        pub const fn set_coef_sat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "the float input is saturated when converted from float to fix due to small FLT_CTRL\\[IN_MAX\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn in_sat(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "the float input is saturated when converted from float to fix due to small FLT_CTRL\\[IN_MAX\\]."]
        #[inline(always)]
        pub const fn set_in_sat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FpSt {
        #[inline(always)]
        fn default() -> FpSt {
            FpSt(0)
        }
    }
    impl core::fmt::Debug for FpSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FpSt")
                .field("exp_min", &self.exp_min())
                .field("exp_max", &self.exp_max())
                .field("coef_nan", &self.coef_nan())
                .field("in_nan", &self.in_nan())
                .field("coef_sat", &self.coef_sat())
                .field("in_sat", &self.in_sat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FpSt {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FpSt {{ exp_min: {=u8:?}, exp_max: {=u8:?}, coef_nan: {=bool:?}, in_nan: {=bool:?}, coef_sat: {=bool:?}, in_sat: {=bool:?} }}" , self . exp_min () , self . exp_max () , self . coef_nan () , self . in_nan () , self . coef_sat () , self . in_sat ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u32);
    impl IntEn {
        #[doc = "Indicate that operation cmd is done, and data are available in system memory."]
        #[must_use]
        #[inline(always)]
        pub const fn op_cmd_done(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Indicate that operation cmd is done, and data are available in system memory."]
        #[inline(always)]
        pub const fn set_op_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Indicate that next command sequence is already read into the module."]
        #[must_use]
        #[inline(always)]
        pub const fn nxt_cmd_rd_done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Indicate that next command sequence is already read into the module."]
        #[inline(always)]
        pub const fn set_nxt_cmd_rd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable Data Read Error interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Data Read Error interrupt."]
        #[inline(always)]
        pub const fn set_rd_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable Read Bus Error for NXT DATA interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_nxt_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Read Bus Error for NXT DATA interrupt."]
        #[inline(always)]
        pub const fn set_rd_nxt_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable Data Write Error interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Data Write Error interrupt."]
        #[inline(always)]
        pub const fn set_wr_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FFT Overflow Err."]
        #[must_use]
        #[inline(always)]
        pub const fn fft_ov(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FFT Overflow Err."]
        #[inline(always)]
        pub const fn set_fft_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FIR Overflow err."]
        #[must_use]
        #[inline(always)]
        pub const fn fir_ov(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIR Overflow err."]
        #[inline(always)]
        pub const fn set_fir_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn wrsv1(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Reserved."]
        #[inline(always)]
        pub const fn set_wrsv1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    impl core::fmt::Debug for IntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntEn")
                .field("op_cmd_done", &self.op_cmd_done())
                .field("nxt_cmd_rd_done", &self.nxt_cmd_rd_done())
                .field("rd_err", &self.rd_err())
                .field("rd_nxt_err", &self.rd_nxt_err())
                .field("wr_err", &self.wr_err())
                .field("fft_ov", &self.fft_ov())
                .field("fir_ov", &self.fir_ov())
                .field("wrsv1", &self.wrsv1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntEn {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IntEn {{ op_cmd_done: {=bool:?}, nxt_cmd_rd_done: {=bool:?}, rd_err: {=bool:?}, rd_nxt_err: {=bool:?}, wr_err: {=bool:?}, fft_ov: {=bool:?}, fir_ov: {=bool:?}, wrsv1: {=u32:?} }}" , self . op_cmd_done () , self . nxt_cmd_rd_done () , self . rd_err () , self . rd_nxt_err () , self . wr_err () , self . fft_ov () , self . fir_ov () , self . wrsv1 ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpCmd(pub u32);
    impl OpCmd {
        #[doc = "The length of nxt commands in 32-bit words."]
        #[must_use]
        #[inline(always)]
        pub const fn nxt_cmd_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "The length of nxt commands in 32-bit words."]
        #[inline(always)]
        pub const fn set_nxt_cmd_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Input data type: 0:Real Q31, 1:Real Q15, 2:Complex Q31, 3:Complex Q15 4:complex sp float 5: real sp float."]
        #[must_use]
        #[inline(always)]
        pub const fn ind_type(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "Input data type: 0:Real Q31, 1:Real Q15, 2:Complex Q31, 3:Complex Q15 4:complex sp float 5: real sp float."]
        #[inline(always)]
        pub const fn set_ind_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "Coef data type (used for FIR): 0:Real Q31, 1:Real Q15, 2:Complex Q31, 3:Complex Q15 4:complex sp float 5: real sp float."]
        #[must_use]
        #[inline(always)]
        pub const fn coef_type(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Coef data type (used for FIR): 0:Real Q31, 1:Real Q15, 2:Complex Q31, 3:Complex Q15 4:complex sp float 5: real sp float."]
        #[inline(always)]
        pub const fn set_coef_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Output data type: 0:Real Q31, 1:Real Q15, 2:Complex Q31, 3:Complex Q15 4:complex sp float 5: real sp float."]
        #[must_use]
        #[inline(always)]
        pub const fn outd_type(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[doc = "Output data type: 0:Real Q31, 1:Real Q15, 2:Complex Q31, 3:Complex Q15 4:complex sp float 5: real sp float."]
        #[inline(always)]
        pub const fn set_outd_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[doc = "The Command Used: 0: FIR 2: FFT Others: Reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x3f;
            val as u8
        }
        #[doc = "The Command Used: 0: FIR 2: FFT Others: Reserved."]
        #[inline(always)]
        pub const fn set_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
        }
        #[doc = "asserted to have conjuate value for coefs in computation."]
        #[must_use]
        #[inline(always)]
        pub const fn conj_c(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "asserted to have conjuate value for coefs in computation."]
        #[inline(always)]
        pub const fn set_conj_c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for OpCmd {
        #[inline(always)]
        fn default() -> OpCmd {
            OpCmd(0)
        }
    }
    impl core::fmt::Debug for OpCmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpCmd")
                .field("nxt_cmd_len", &self.nxt_cmd_len())
                .field("ind_type", &self.ind_type())
                .field("coef_type", &self.coef_type())
                .field("outd_type", &self.outd_type())
                .field("cmd", &self.cmd())
                .field("conj_c", &self.conj_c())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpCmd {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "OpCmd {{ nxt_cmd_len: {=u8:?}, ind_type: {=u8:?}, coef_type: {=u8:?}, outd_type: {=u8:?}, cmd: {=u8:?}, conj_c: {=bool:?} }}" , self . nxt_cmd_len () , self . ind_type () , self . coef_type () , self . outd_type () , self . cmd () , self . conj_c ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpCtrl(pub u32);
    impl OpCtrl {
        #[doc = "Whether CUR_CMD is enabled. Asserted to enable the CUR_CMD."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Whether CUR_CMD is enabled. Asserted to enable the CUR_CMD."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Whether NXT_CMD is enabled. Asserted to enable the NXT_CMD when CUR_CMD is done, or CUR_CMD is not enabled.."]
        #[must_use]
        #[inline(always)]
        pub const fn nxt_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Whether NXT_CMD is enabled. Asserted to enable the NXT_CMD when CUR_CMD is done, or CUR_CMD is not enabled.."]
        #[inline(always)]
        pub const fn set_nxt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "The address for the next command. It will be processed after CUR_CMD is executed and done.."]
        #[must_use]
        #[inline(always)]
        pub const fn nxt_addr(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "The address for the next command. It will be processed after CUR_CMD is executed and done.."]
        #[inline(always)]
        pub const fn set_nxt_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for OpCtrl {
        #[inline(always)]
        fn default() -> OpCtrl {
            OpCtrl(0)
        }
    }
    impl core::fmt::Debug for OpCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpCtrl")
                .field("en", &self.en())
                .field("nxt_en", &self.nxt_en())
                .field("nxt_addr", &self.nxt_addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OpCtrl {{ en: {=bool:?}, nxt_en: {=bool:?}, nxt_addr: {=u32:?} }}",
                self.en(),
                self.nxt_en(),
                self.nxt_addr()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFftInrbuf(pub u32);
    impl OpFftInrbuf {
        #[doc = "The input (real) data buffer pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn loc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The input (real) data buffer pointer."]
        #[inline(always)]
        pub const fn set_loc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpFftInrbuf {
        #[inline(always)]
        fn default() -> OpFftInrbuf {
            OpFftInrbuf(0)
        }
    }
    impl core::fmt::Debug for OpFftInrbuf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpFftInrbuf")
                .field("loc", &self.loc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpFftInrbuf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OpFftInrbuf {{ loc: {=u32:?} }}", self.loc())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFftMisc(pub u32);
    impl OpFftMisc {
        #[doc = "Memory block for indata. Should be assigned as 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ind_blk(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Memory block for indata. Should be assigned as 0."]
        #[inline(always)]
        pub const fn set_ind_blk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Memory block for indata. Should be assigned as 1."]
        #[must_use]
        #[inline(always)]
        pub const fn tmp_blk(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Memory block for indata. Should be assigned as 1."]
        #[inline(always)]
        pub const fn set_tmp_blk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Asserted to indicate IFFT."]
        #[must_use]
        #[inline(always)]
        pub const fn ifft(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Asserted to indicate IFFT."]
        #[inline(always)]
        pub const fn set_ifft(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FFT length 0:8, ..., n:2^(3+n)."]
        #[must_use]
        #[inline(always)]
        pub const fn fft_len(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[doc = "FFT length 0:8, ..., n:2^(3+n)."]
        #[inline(always)]
        pub const fn set_fft_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
        }
    }
    impl Default for OpFftMisc {
        #[inline(always)]
        fn default() -> OpFftMisc {
            OpFftMisc(0)
        }
    }
    impl core::fmt::Debug for OpFftMisc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpFftMisc")
                .field("ind_blk", &self.ind_blk())
                .field("tmp_blk", &self.tmp_blk())
                .field("ifft", &self.ifft())
                .field("fft_len", &self.fft_len())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpFftMisc {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "OpFftMisc {{ ind_blk: {=u8:?}, tmp_blk: {=u8:?}, ifft: {=bool:?}, fft_len: {=u8:?} }}" , self . ind_blk () , self . tmp_blk () , self . ifft () , self . fft_len ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFftOutrbuf(pub u32);
    impl OpFftOutrbuf {
        #[doc = "The output (real) data buffer pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn loc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The output (real) data buffer pointer."]
        #[inline(always)]
        pub const fn set_loc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpFftOutrbuf {
        #[inline(always)]
        fn default() -> OpFftOutrbuf {
            OpFftOutrbuf(0)
        }
    }
    impl core::fmt::Debug for OpFftOutrbuf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpFftOutrbuf")
                .field("loc", &self.loc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpFftOutrbuf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OpFftOutrbuf {{ loc: {=u32:?} }}", self.loc())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFirCoefbuf(pub u32);
    impl OpFirCoefbuf {
        #[doc = "The coef buf pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn loc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The coef buf pointer."]
        #[inline(always)]
        pub const fn set_loc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpFirCoefbuf {
        #[inline(always)]
        fn default() -> OpFirCoefbuf {
            OpFirCoefbuf(0)
        }
    }
    impl core::fmt::Debug for OpFirCoefbuf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpFirCoefbuf")
                .field("loc", &self.loc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpFirCoefbuf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OpFirCoefbuf {{ loc: {=u32:?} }}", self.loc())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFirInbuf(pub u32);
    impl OpFirInbuf {
        #[doc = "The input data buffer pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn loc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The input data buffer pointer."]
        #[inline(always)]
        pub const fn set_loc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpFirInbuf {
        #[inline(always)]
        fn default() -> OpFirInbuf {
            OpFirInbuf(0)
        }
    }
    impl core::fmt::Debug for OpFirInbuf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpFirInbuf")
                .field("loc", &self.loc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpFirInbuf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OpFirInbuf {{ loc: {=u32:?} }}", self.loc())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFirMisc(pub u32);
    impl OpFirMisc {
        #[doc = "Length of FIR coefs （max 256）."]
        #[must_use]
        #[inline(always)]
        pub const fn fir_coef_taps(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Length of FIR coefs （max 256）."]
        #[inline(always)]
        pub const fn set_fir_coef_taps(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for OpFirMisc {
        #[inline(always)]
        fn default() -> OpFirMisc {
            OpFirMisc(0)
        }
    }
    impl core::fmt::Debug for OpFirMisc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpFirMisc")
                .field("fir_coef_taps", &self.fir_coef_taps())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpFirMisc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OpFirMisc {{ fir_coef_taps: {=u16:?} }}",
                self.fir_coef_taps()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFirMisc1(pub u32);
    impl OpFirMisc1 {
        #[doc = "The input data data length."]
        #[must_use]
        #[inline(always)]
        pub const fn fir_data_taps(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "The input data data length."]
        #[inline(always)]
        pub const fn set_fir_data_taps(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Should be assigned as 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ind_mem_blk(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Should be assigned as 2."]
        #[inline(always)]
        pub const fn set_ind_mem_blk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Should be assigned as 1."]
        #[must_use]
        #[inline(always)]
        pub const fn coef_mem_blk(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Should be assigned as 1."]
        #[inline(always)]
        pub const fn set_coef_mem_blk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "Should be assigned as 0."]
        #[must_use]
        #[inline(always)]
        pub const fn outd_mem_blk(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Should be assigned as 0."]
        #[inline(always)]
        pub const fn set_outd_mem_blk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for OpFirMisc1 {
        #[inline(always)]
        fn default() -> OpFirMisc1 {
            OpFirMisc1(0)
        }
    }
    impl core::fmt::Debug for OpFirMisc1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpFirMisc1")
                .field("fir_data_taps", &self.fir_data_taps())
                .field("ind_mem_blk", &self.ind_mem_blk())
                .field("coef_mem_blk", &self.coef_mem_blk())
                .field("outd_mem_blk", &self.outd_mem_blk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpFirMisc1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "OpFirMisc1 {{ fir_data_taps: {=u16:?}, ind_mem_blk: {=u8:?}, coef_mem_blk: {=u8:?}, outd_mem_blk: {=u8:?} }}" , self . fir_data_taps () , self . ind_mem_blk () , self . coef_mem_blk () , self . outd_mem_blk ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpFirOutbuf(pub u32);
    impl OpFirOutbuf {
        #[doc = "The output data buffer pointer. The length of the output buffer should be （FIR_DATA_TAPS - FIR_COEF_TAPS + 1）."]
        #[must_use]
        #[inline(always)]
        pub const fn loc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The output data buffer pointer. The length of the output buffer should be （FIR_DATA_TAPS - FIR_COEF_TAPS + 1）."]
        #[inline(always)]
        pub const fn set_loc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpFirOutbuf {
        #[inline(always)]
        fn default() -> OpFirOutbuf {
            OpFirOutbuf(0)
        }
    }
    impl core::fmt::Debug for OpFirOutbuf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpFirOutbuf")
                .field("loc", &self.loc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpFirOutbuf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OpFirOutbuf {{ loc: {=u32:?} }}", self.loc())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg0(pub u32);
    impl OpReg0 {
        #[doc = "Contents."]
        #[must_use]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg0 {
        #[inline(always)]
        fn default() -> OpReg0 {
            OpReg0(0)
        }
    }
    impl core::fmt::Debug for OpReg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpReg0").field("ct", &self.ct()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpReg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OpReg0 {{ ct: {=u32:?} }}", self.ct())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg1(pub u32);
    impl OpReg1 {
        #[doc = "Contents."]
        #[must_use]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg1 {
        #[inline(always)]
        fn default() -> OpReg1 {
            OpReg1(0)
        }
    }
    impl core::fmt::Debug for OpReg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpReg1").field("ct", &self.ct()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpReg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OpReg1 {{ ct: {=u32:?} }}", self.ct())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg2(pub u32);
    impl OpReg2 {
        #[doc = "Contents."]
        #[must_use]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg2 {
        #[inline(always)]
        fn default() -> OpReg2 {
            OpReg2(0)
        }
    }
    impl core::fmt::Debug for OpReg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpReg2").field("ct", &self.ct()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpReg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OpReg2 {{ ct: {=u32:?} }}", self.ct())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg3(pub u32);
    impl OpReg3 {
        #[doc = "Contents."]
        #[must_use]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg3 {
        #[inline(always)]
        fn default() -> OpReg3 {
            OpReg3(0)
        }
    }
    impl core::fmt::Debug for OpReg3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpReg3").field("ct", &self.ct()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpReg3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OpReg3 {{ ct: {=u32:?} }}", self.ct())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg4(pub u32);
    impl OpReg4 {
        #[doc = "Contents."]
        #[must_use]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg4 {
        #[inline(always)]
        fn default() -> OpReg4 {
            OpReg4(0)
        }
    }
    impl core::fmt::Debug for OpReg4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpReg4").field("ct", &self.ct()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpReg4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OpReg4 {{ ct: {=u32:?} }}", self.ct())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg5(pub u32);
    impl OpReg5 {
        #[doc = "Contents."]
        #[must_use]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg5 {
        #[inline(always)]
        fn default() -> OpReg5 {
            OpReg5(0)
        }
    }
    impl core::fmt::Debug for OpReg5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpReg5").field("ct", &self.ct()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpReg5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OpReg5 {{ ct: {=u32:?} }}", self.ct())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg6(pub u32);
    impl OpReg6 {
        #[doc = "Contents."]
        #[must_use]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg6 {
        #[inline(always)]
        fn default() -> OpReg6 {
            OpReg6(0)
        }
    }
    impl core::fmt::Debug for OpReg6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpReg6").field("ct", &self.ct()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpReg6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OpReg6 {{ ct: {=u32:?} }}", self.ct())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OpReg7(pub u32);
    impl OpReg7 {
        #[doc = "Contents."]
        #[must_use]
        #[inline(always)]
        pub const fn ct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Contents."]
        #[inline(always)]
        pub const fn set_ct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for OpReg7 {
        #[inline(always)]
        fn default() -> OpReg7 {
            OpReg7(0)
        }
    }
    impl core::fmt::Debug for OpReg7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OpReg7").field("ct", &self.ct()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OpReg7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OpReg7 {{ ct: {=u32:?} }}", self.ct())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "Indicate that operation cmd is done, and data are available in system memory."]
        #[must_use]
        #[inline(always)]
        pub const fn op_cmd_done(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Indicate that operation cmd is done, and data are available in system memory."]
        #[inline(always)]
        pub const fn set_op_cmd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Indicate that next command sequence is already read into the module."]
        #[must_use]
        #[inline(always)]
        pub const fn nxt_cmd_rd_done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Indicate that next command sequence is already read into the module."]
        #[inline(always)]
        pub const fn set_nxt_cmd_rd_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AXI Data Read Error."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Data Read Error."]
        #[inline(always)]
        pub const fn set_rd_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "AXI Read Bus Error for NXT DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_nxt_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Read Bus Error for NXT DATA."]
        #[inline(always)]
        pub const fn set_rd_nxt_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AXI Data Write Error."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "AXI Data Write Error."]
        #[inline(always)]
        pub const fn set_wr_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FFT Overflow Err."]
        #[must_use]
        #[inline(always)]
        pub const fn fft_ov(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FFT Overflow Err."]
        #[inline(always)]
        pub const fn set_fft_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FIR Overflow err."]
        #[must_use]
        #[inline(always)]
        pub const fn fir_ov(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIR Overflow err."]
        #[inline(always)]
        pub const fn set_fir_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Ored together by ( FP_ST\\[IN_SAT\\]
& FP_CTRL\\[IN_SAT_IE\\]
) | ( FP_ST\\[COEF_SAT\\]
& FP_CTRL\\[COEF_SAT_IE\\]
)."]
        #[must_use]
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
        pub const fn set_fp_sat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Ored together by ( FP_ST\\[IN_NAN\\]
& FP_CTRL\\[IN_NAN_IE\\]
) | ( FP_ST\\[COEF_NAN\\]
& FP_CTRL\\[COEF_NAN_IE\\]
)."]
        #[must_use]
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
        pub const fn set_fp_nan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    impl core::fmt::Debug for Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Status")
                .field("op_cmd_done", &self.op_cmd_done())
                .field("nxt_cmd_rd_done", &self.nxt_cmd_rd_done())
                .field("rd_err", &self.rd_err())
                .field("rd_nxt_err", &self.rd_nxt_err())
                .field("wr_err", &self.wr_err())
                .field("fft_ov", &self.fft_ov())
                .field("fir_ov", &self.fir_ov())
                .field("fp_sat", &self.fp_sat())
                .field("fp_nan", &self.fp_nan())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Status {{ op_cmd_done: {=bool:?}, nxt_cmd_rd_done: {=bool:?}, rd_err: {=bool:?}, rd_nxt_err: {=bool:?}, wr_err: {=bool:?}, fft_ov: {=bool:?}, fir_ov: {=bool:?}, fp_sat: {=bool:?}, fp_nan: {=bool:?} }}" , self . op_cmd_done () , self . nxt_cmd_rd_done () , self . rd_err () , self . rd_nxt_err () , self . wr_err () , self . fft_ov () , self . fir_ov () , self . fp_sat () , self . fp_nan ())
        }
    }
}
