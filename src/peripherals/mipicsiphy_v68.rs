#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "MIPI_CSI_PHY0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MipiCsiPhy {
    ptr: *mut u8,
}
unsafe impl Send for MipiCsiPhy {}
unsafe impl Sync for MipiCsiPhy {}
impl MipiCsiPhy {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "soft reset control."]
    #[inline(always)]
    pub const fn soft_rst(self) -> crate::common::Reg<regs::SoftRst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "dphy resistor calibration."]
    #[inline(always)]
    pub const fn phy_rcal(self) -> crate::common::Reg<regs::PhyRcal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "enable lprx and ulprx."]
    #[inline(always)]
    pub const fn ulp_rx_en(self) -> crate::common::Reg<regs::UlpRxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "hs-rx dc-offset auto-calibration results."]
    #[inline(always)]
    pub const fn voffcal_out(self) -> crate::common::Reg<regs::VoffcalOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "dphy hardcore control."]
    #[inline(always)]
    pub const fn csi_ctl01(self) -> crate::common::Reg<regs::CsiCtl01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "dphy hardcore control."]
    #[inline(always)]
    pub const fn csi_ctl23(self) -> crate::common::Reg<regs::CsiCtl23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "ulp lp-rx input threshold voltage trimming for data lane."]
    #[inline(always)]
    pub const fn csi_vinit(self) -> crate::common::Reg<regs::CsiVinit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "clock lane parameter."]
    #[inline(always)]
    pub const fn clane_para(self) -> crate::common::Reg<regs::ClanePara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "t-termen of all datalane."]
    #[inline(always)]
    pub const fn t_hs_termen(self) -> crate::common::Reg<regs::THsTermen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "t-settle of all data lanes."]
    #[inline(always)]
    pub const fn t_hs_settle(self) -> crate::common::Reg<regs::THsSettle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "t-init of clock lane."]
    #[inline(always)]
    pub const fn t_clane_init(self) -> crate::common::Reg<regs::TClaneInit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "t-init of data lane0."]
    #[inline(always)]
    pub const fn t_lane_init0(self) -> crate::common::Reg<regs::TLaneInit0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "t-init of data lane1."]
    #[inline(always)]
    pub const fn t_lane_init1(self) -> crate::common::Reg<regs::TLaneInit1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "the time of tlpx_ctrl of all lane."]
    #[inline(always)]
    pub const fn tlpx_ctrl(self) -> crate::common::Reg<regs::TlpxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "lane swap and dp/dn swap select."]
    #[inline(always)]
    pub const fn ne_swap(self) -> crate::common::Reg<regs::NeSwap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "misc info of dphyrx_pcs control."]
    #[inline(always)]
    pub const fn misc_info(self) -> crate::common::Reg<regs::MiscInfo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "bist test control."]
    #[inline(always)]
    pub const fn bist_test0(self) -> crate::common::Reg<regs::BistTest0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "bist test control."]
    #[inline(always)]
    pub const fn bist_test1(self) -> crate::common::Reg<regs::BistTest1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "bist test control."]
    #[inline(always)]
    pub const fn bist_test2(self) -> crate::common::Reg<regs::BistTest2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "bist test control."]
    #[inline(always)]
    pub const fn bist_test3(self) -> crate::common::Reg<regs::BistTest3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "burn-in test control."]
    #[inline(always)]
    pub const fn burn_in_test0(self) -> crate::common::Reg<regs::BurnInTest0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "burn-in test control."]
    #[inline(always)]
    pub const fn burn_in_test1(self) -> crate::common::Reg<regs::BurnInTest1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "bist test control."]
    #[inline(always)]
    pub const fn burn_in_test2(self) -> crate::common::Reg<regs::BurnInTest2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "bist test control."]
    #[inline(always)]
    pub const fn burn_in_test4(self) -> crate::common::Reg<regs::BurnInTest4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "burn-in test control."]
    #[inline(always)]
    pub const fn burn_in_test5(self) -> crate::common::Reg<regs::BurnInTest5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "burn-in test control."]
    #[inline(always)]
    pub const fn burn_in_test6(self) -> crate::common::Reg<regs::BurnInTest6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "burn-in test control."]
    #[inline(always)]
    pub const fn burn_in_test9(self) -> crate::common::Reg<regs::BurnInTest9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "debug data control."]
    #[inline(always)]
    pub const fn debug_info(self) -> crate::common::Reg<regs::DebugInfo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "the hardcore interface control in debug mode."]
    #[inline(always)]
    pub const fn debug_cfg_reg0(self) -> crate::common::Reg<regs::DebugCfgReg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "the hardcore interface control in debug mode."]
    #[inline(always)]
    pub const fn debug_cfg_reg1(self) -> crate::common::Reg<regs::DebugCfgReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "the hardcore interface control in debug mode."]
    #[inline(always)]
    pub const fn debug_cfg_reg2(self) -> crate::common::Reg<regs::DebugCfgReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d12usize) as _) }
    }
    #[doc = "the hardcore interface control in debug mode."]
    #[inline(always)]
    pub const fn debug_cfg_reg3(self) -> crate::common::Reg<regs::DebugCfgReg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d16usize) as _) }
    }
    #[doc = "the hardcore interface control in debug mode."]
    #[inline(always)]
    pub const fn debug_cfg_reg4(self) -> crate::common::Reg<regs::DebugCfgReg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d20usize) as _) }
    }
    #[doc = "the hardcore interface control in debug mode."]
    #[inline(always)]
    pub const fn debug_cfg_reg5(self) -> crate::common::Reg<regs::DebugCfgReg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d24usize) as _) }
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
    #[doc = "bist test control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BistTest0(pub u32);
    impl BistTest0 {
        #[doc = "enable prbs bist test."]
        #[must_use]
        #[inline(always)]
        pub const fn bist_en_soft(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable prbs bist test."]
        #[inline(always)]
        pub const fn set_bist_en_soft(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the source of bist_en sel."]
        #[must_use]
        #[inline(always)]
        pub const fn bist_en_sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the source of bist_en sel."]
        #[inline(always)]
        pub const fn set_bist_en_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "bist_ok of lane0."]
        #[must_use]
        #[inline(always)]
        pub const fn bist_ok_lane0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "bist_ok of lane0."]
        #[inline(always)]
        pub const fn set_bist_ok_lane0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "bist_ok of lane1."]
        #[must_use]
        #[inline(always)]
        pub const fn bist_ok_lane1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "bist_ok of lane1."]
        #[inline(always)]
        pub const fn set_bist_ok_lane1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "bist_done of lane0."]
        #[must_use]
        #[inline(always)]
        pub const fn bist_done_lan0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "bist_done of lane0."]
        #[inline(always)]
        pub const fn set_bist_done_lan0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "bist_done of lane1."]
        #[must_use]
        #[inline(always)]
        pub const fn bist_done_lan1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "bist_done of lane1."]
        #[inline(always)]
        pub const fn set_bist_done_lan1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for BistTest0 {
        #[inline(always)]
        fn default() -> BistTest0 {
            BistTest0(0)
        }
    }
    impl core::fmt::Debug for BistTest0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BistTest0")
                .field("bist_en_soft", &self.bist_en_soft())
                .field("bist_en_sel", &self.bist_en_sel())
                .field("bist_ok_lane0", &self.bist_ok_lane0())
                .field("bist_ok_lane1", &self.bist_ok_lane1())
                .field("bist_done_lan0", &self.bist_done_lan0())
                .field("bist_done_lan1", &self.bist_done_lan1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BistTest0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BistTest0 {{ bist_en_soft: {=bool:?}, bist_en_sel: {=bool:?}, bist_ok_lane0: {=bool:?}, bist_ok_lane1: {=bool:?}, bist_done_lan0: {=bool:?}, bist_done_lan1: {=bool:?} }}" , self . bist_en_soft () , self . bist_en_sel () , self . bist_ok_lane0 () , self . bist_ok_lane1 () , self . bist_done_lan0 () , self . bist_done_lan1 ())
        }
    }
    #[doc = "bist test control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BistTest1(pub u32);
    impl BistTest1 {
        #[doc = "the byte num of prbs bist check num."]
        #[must_use]
        #[inline(always)]
        pub const fn prbs_check_num(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the byte num of prbs bist check num."]
        #[inline(always)]
        pub const fn set_prbs_check_num(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BistTest1 {
        #[inline(always)]
        fn default() -> BistTest1 {
            BistTest1(0)
        }
    }
    impl core::fmt::Debug for BistTest1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BistTest1")
                .field("prbs_check_num", &self.prbs_check_num())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BistTest1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BistTest1 {{ prbs_check_num: {=u32:?} }}",
                self.prbs_check_num()
            )
        }
    }
    #[doc = "bist test control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BistTest2(pub u32);
    impl BistTest2 {
        #[doc = "the threshold of prbs bist error."]
        #[must_use]
        #[inline(always)]
        pub const fn prbs_err_threshold(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the threshold of prbs bist error."]
        #[inline(always)]
        pub const fn set_prbs_err_threshold(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "the seed of prbs7."]
        #[must_use]
        #[inline(always)]
        pub const fn prbs_seed(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the seed of prbs7."]
        #[inline(always)]
        pub const fn set_prbs_seed(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for BistTest2 {
        #[inline(always)]
        fn default() -> BistTest2 {
            BistTest2(0)
        }
    }
    impl core::fmt::Debug for BistTest2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BistTest2")
                .field("prbs_err_threshold", &self.prbs_err_threshold())
                .field("prbs_seed", &self.prbs_seed())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BistTest2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BistTest2 {{ prbs_err_threshold: {=u16:?}, prbs_seed: {=u8:?} }}",
                self.prbs_err_threshold(),
                self.prbs_seed()
            )
        }
    }
    #[doc = "bist test control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BistTest3(pub u32);
    impl BistTest3 {
        #[doc = "the byte num of mismatch data of data lane0 in bist mode."]
        #[must_use]
        #[inline(always)]
        pub const fn prbs_err_num_lan0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the byte num of mismatch data of data lane0 in bist mode."]
        #[inline(always)]
        pub const fn set_prbs_err_num_lan0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "the byte num of mismatch data of data lane1 in bist mode."]
        #[must_use]
        #[inline(always)]
        pub const fn prbs_err_num_lan1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "the byte num of mismatch data of data lane1 in bist mode."]
        #[inline(always)]
        pub const fn set_prbs_err_num_lan1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for BistTest3 {
        #[inline(always)]
        fn default() -> BistTest3 {
            BistTest3(0)
        }
    }
    impl core::fmt::Debug for BistTest3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BistTest3")
                .field("prbs_err_num_lan0", &self.prbs_err_num_lan0())
                .field("prbs_err_num_lan1", &self.prbs_err_num_lan1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BistTest3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BistTest3 {{ prbs_err_num_lan0: {=u16:?}, prbs_err_num_lan1: {=u16:?} }}",
                self.prbs_err_num_lan0(),
                self.prbs_err_num_lan1()
            )
        }
    }
    #[doc = "burn-in test control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BurnInTest0(pub u32);
    impl BurnInTest0 {
        #[doc = "enable prbs burn_in test."]
        #[must_use]
        #[inline(always)]
        pub const fn burn_in_en_soft(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable prbs burn_in test."]
        #[inline(always)]
        pub const fn set_burn_in_en_soft(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the source of prbs burn_in_en sel."]
        #[must_use]
        #[inline(always)]
        pub const fn burn_in_en_sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the source of prbs burn_in_en sel."]
        #[inline(always)]
        pub const fn set_burn_in_en_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "burn_in_ok of lane0."]
        #[must_use]
        #[inline(always)]
        pub const fn burn_in_ok_lan0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "burn_in_ok of lane0."]
        #[inline(always)]
        pub const fn set_burn_in_ok_lan0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "burn_in_ok of lane1."]
        #[must_use]
        #[inline(always)]
        pub const fn burn_in_ok_lan1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "burn_in_ok of lane1."]
        #[inline(always)]
        pub const fn set_burn_in_ok_lan1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "burn_in_ok of clock lane."]
        #[must_use]
        #[inline(always)]
        pub const fn burn_in_ok_clan(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "burn_in_ok of clock lane."]
        #[inline(always)]
        pub const fn set_burn_in_ok_clan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for BurnInTest0 {
        #[inline(always)]
        fn default() -> BurnInTest0 {
            BurnInTest0(0)
        }
    }
    impl core::fmt::Debug for BurnInTest0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BurnInTest0")
                .field("burn_in_en_soft", &self.burn_in_en_soft())
                .field("burn_in_en_sel", &self.burn_in_en_sel())
                .field("burn_in_ok_lan0", &self.burn_in_ok_lan0())
                .field("burn_in_ok_lan1", &self.burn_in_ok_lan1())
                .field("burn_in_ok_clan", &self.burn_in_ok_clan())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BurnInTest0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "BurnInTest0 {{ burn_in_en_soft: {=bool:?}, burn_in_en_sel: {=bool:?}, burn_in_ok_lan0: {=bool:?}, burn_in_ok_lan1: {=bool:?}, burn_in_ok_clan: {=bool:?} }}" , self . burn_in_en_soft () , self . burn_in_en_sel () , self . burn_in_ok_lan0 () , self . burn_in_ok_lan1 () , self . burn_in_ok_clan ())
        }
    }
    #[doc = "burn-in test control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BurnInTest1(pub u32);
    impl BurnInTest1 {
        #[doc = "the seed of prbs7 for brun-in test."]
        #[must_use]
        #[inline(always)]
        pub const fn burn_in_seed(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the seed of prbs7 for brun-in test."]
        #[inline(always)]
        pub const fn set_burn_in_seed(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for BurnInTest1 {
        #[inline(always)]
        fn default() -> BurnInTest1 {
            BurnInTest1(0)
        }
    }
    impl core::fmt::Debug for BurnInTest1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BurnInTest1")
                .field("burn_in_seed", &self.burn_in_seed())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BurnInTest1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BurnInTest1 {{ burn_in_seed: {=u8:?} }}",
                self.burn_in_seed()
            )
        }
    }
    #[doc = "bist test control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BurnInTest2(pub u32);
    impl BurnInTest2 {
        #[doc = "the bit num of mismatch data on data lan0 in burn-in mode."]
        #[must_use]
        #[inline(always)]
        pub const fn burn_in_err_num_lan0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the bit num of mismatch data on data lan0 in burn-in mode."]
        #[inline(always)]
        pub const fn set_burn_in_err_num_lan0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "the bit num of mismatch data on data lan1 in burn-in mode."]
        #[must_use]
        #[inline(always)]
        pub const fn burn_in_err_num_lan1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "the bit num of mismatch data on data lan1 in burn-in mode."]
        #[inline(always)]
        pub const fn set_burn_in_err_num_lan1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for BurnInTest2 {
        #[inline(always)]
        fn default() -> BurnInTest2 {
            BurnInTest2(0)
        }
    }
    impl core::fmt::Debug for BurnInTest2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BurnInTest2")
                .field("burn_in_err_num_lan0", &self.burn_in_err_num_lan0())
                .field("burn_in_err_num_lan1", &self.burn_in_err_num_lan1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BurnInTest2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BurnInTest2 {{ burn_in_err_num_lan0: {=u16:?}, burn_in_err_num_lan1: {=u16:?} }}",
                self.burn_in_err_num_lan0(),
                self.burn_in_err_num_lan1()
            )
        }
    }
    #[doc = "bist test control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BurnInTest4(pub u32);
    impl BurnInTest4 {
        #[doc = "the bit num of mismatch data on clock lane in burn-in mode."]
        #[must_use]
        #[inline(always)]
        pub const fn burn_in_err_num_clan(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the bit num of mismatch data on clock lane in burn-in mode."]
        #[inline(always)]
        pub const fn set_burn_in_err_num_clan(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for BurnInTest4 {
        #[inline(always)]
        fn default() -> BurnInTest4 {
            BurnInTest4(0)
        }
    }
    impl core::fmt::Debug for BurnInTest4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BurnInTest4")
                .field("burn_in_err_num_clan", &self.burn_in_err_num_clan())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BurnInTest4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BurnInTest4 {{ burn_in_err_num_clan: {=u16:?} }}",
                self.burn_in_err_num_clan()
            )
        }
    }
    #[doc = "burn-in test control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BurnInTest5(pub u32);
    impl BurnInTest5 {
        #[doc = "the checked bit num of lane0."]
        #[must_use]
        #[inline(always)]
        pub const fn burn_in_check_num_lan0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the checked bit num of lane0."]
        #[inline(always)]
        pub const fn set_burn_in_check_num_lan0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BurnInTest5 {
        #[inline(always)]
        fn default() -> BurnInTest5 {
            BurnInTest5(0)
        }
    }
    impl core::fmt::Debug for BurnInTest5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BurnInTest5")
                .field("burn_in_check_num_lan0", &self.burn_in_check_num_lan0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BurnInTest5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BurnInTest5 {{ burn_in_check_num_lan0: {=u32:?} }}",
                self.burn_in_check_num_lan0()
            )
        }
    }
    #[doc = "burn-in test control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BurnInTest6(pub u32);
    impl BurnInTest6 {
        #[doc = "the checked bit num of lane1."]
        #[must_use]
        #[inline(always)]
        pub const fn burn_in_checked_num_lan1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the checked bit num of lane1."]
        #[inline(always)]
        pub const fn set_burn_in_checked_num_lan1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BurnInTest6 {
        #[inline(always)]
        fn default() -> BurnInTest6 {
            BurnInTest6(0)
        }
    }
    impl core::fmt::Debug for BurnInTest6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BurnInTest6")
                .field("burn_in_checked_num_lan1", &self.burn_in_checked_num_lan1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BurnInTest6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BurnInTest6 {{ burn_in_checked_num_lan1: {=u32:?} }}",
                self.burn_in_checked_num_lan1()
            )
        }
    }
    #[doc = "burn-in test control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BurnInTest9(pub u32);
    impl BurnInTest9 {
        #[doc = "the checked bit num of clock lane."]
        #[must_use]
        #[inline(always)]
        pub const fn burn_in_check_num_clan(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the checked bit num of clock lane."]
        #[inline(always)]
        pub const fn set_burn_in_check_num_clan(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BurnInTest9 {
        #[inline(always)]
        fn default() -> BurnInTest9 {
            BurnInTest9(0)
        }
    }
    impl core::fmt::Debug for BurnInTest9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BurnInTest9")
                .field("burn_in_check_num_clan", &self.burn_in_check_num_clan())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BurnInTest9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BurnInTest9 {{ burn_in_check_num_clan: {=u32:?} }}",
                self.burn_in_check_num_clan()
            )
        }
    }
    #[doc = "clock lane parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClanePara(pub u32);
    impl ClanePara {
        #[doc = "the value of tclk-settle of clklane."]
        #[must_use]
        #[inline(always)]
        pub const fn t_clk_settle(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the value of tclk-settle of clklane."]
        #[inline(always)]
        pub const fn set_t_clk_settle(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "time for the clock lane receiver to enable the HS line termination."]
        #[must_use]
        #[inline(always)]
        pub const fn t_clk_termen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "time for the clock lane receiver to enable the HS line termination."]
        #[inline(always)]
        pub const fn set_t_clk_termen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for ClanePara {
        #[inline(always)]
        fn default() -> ClanePara {
            ClanePara(0)
        }
    }
    impl core::fmt::Debug for ClanePara {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClanePara")
                .field("t_clk_settle", &self.t_clk_settle())
                .field("t_clk_termen", &self.t_clk_termen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ClanePara {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ClanePara {{ t_clk_settle: {=u8:?}, t_clk_termen: {=u8:?} }}",
                self.t_clk_settle(),
                self.t_clk_termen()
            )
        }
    }
    #[doc = "dphy hardcore control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CsiCtl01(pub u32);
    impl CsiCtl01 {
        #[doc = "force data lane-n and clock lane hs-rx to be normal operation."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl0_0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "force data lane-n and clock lane hs-rx to be normal operation."]
        #[inline(always)]
        pub const fn set_csi_ctl0_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "force data lane-n and clock lane lp/ulprx to be normal operation."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl0_1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force data lane-n and clock lane lp/ulprx to be normal operation."]
        #[inline(always)]
        pub const fn set_csi_ctl0_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ulprx_lpen."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl0_2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ulprx_lpen."]
        #[inline(always)]
        pub const fn set_csi_ctl0_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "hs_rx_voffcal_trim_polar."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl0_3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "hs_rx_voffcal_trim_polar."]
        #[inline(always)]
        pub const fn set_csi_ctl0_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "bypass hs_rx_voffcal_en."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl0_4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "bypass hs_rx_voffcal_en."]
        #[inline(always)]
        pub const fn set_csi_ctl0_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ulprx_vref_trim."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl0_5(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "ulprx_vref_trim."]
        #[inline(always)]
        pub const fn set_csi_ctl0_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "clock lane hs-rx dc-offset trimming control."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl0_6(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "clock lane hs-rx dc-offset trimming control."]
        #[inline(always)]
        pub const fn set_csi_ctl0_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "clock lane hs-rx dc-offset auto-calibration enable."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl0_7(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "clock lane hs-rx dc-offset auto-calibration enable."]
        #[inline(always)]
        pub const fn set_csi_ctl0_7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "force data lane-n and clock lane hs-rx to be normal operation."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl1_0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "force data lane-n and clock lane hs-rx to be normal operation."]
        #[inline(always)]
        pub const fn set_csi_ctl1_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "force data lane-n and clock lane lp/ulprx to be normal operation."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl1_1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "force data lane-n and clock lane lp/ulprx to be normal operation."]
        #[inline(always)]
        pub const fn set_csi_ctl1_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ulprx_lpen."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl1_2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "ulprx_lpen."]
        #[inline(always)]
        pub const fn set_csi_ctl1_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "hs_rx_voffcal_trim_polar."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl1_3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "hs_rx_voffcal_trim_polar."]
        #[inline(always)]
        pub const fn set_csi_ctl1_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "bypass hs_rx_voffcal_en."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl1_4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "bypass hs_rx_voffcal_en."]
        #[inline(always)]
        pub const fn set_csi_ctl1_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ulprx_vref_trim."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl1_5(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "ulprx_vref_trim."]
        #[inline(always)]
        pub const fn set_csi_ctl1_5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "clock lane hs-rx dc-offset trimming control."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl1_6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "clock lane hs-rx dc-offset trimming control."]
        #[inline(always)]
        pub const fn set_csi_ctl1_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "clock lane hs-rx dc-offset auto-calibration enable."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl1_7(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "clock lane hs-rx dc-offset auto-calibration enable."]
        #[inline(always)]
        pub const fn set_csi_ctl1_7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for CsiCtl01 {
        #[inline(always)]
        fn default() -> CsiCtl01 {
            CsiCtl01(0)
        }
    }
    impl core::fmt::Debug for CsiCtl01 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CsiCtl01")
                .field("csi_ctl0_0", &self.csi_ctl0_0())
                .field("csi_ctl0_1", &self.csi_ctl0_1())
                .field("csi_ctl0_2", &self.csi_ctl0_2())
                .field("csi_ctl0_3", &self.csi_ctl0_3())
                .field("csi_ctl0_4", &self.csi_ctl0_4())
                .field("csi_ctl0_5", &self.csi_ctl0_5())
                .field("csi_ctl0_6", &self.csi_ctl0_6())
                .field("csi_ctl0_7", &self.csi_ctl0_7())
                .field("csi_ctl1_0", &self.csi_ctl1_0())
                .field("csi_ctl1_1", &self.csi_ctl1_1())
                .field("csi_ctl1_2", &self.csi_ctl1_2())
                .field("csi_ctl1_3", &self.csi_ctl1_3())
                .field("csi_ctl1_4", &self.csi_ctl1_4())
                .field("csi_ctl1_5", &self.csi_ctl1_5())
                .field("csi_ctl1_6", &self.csi_ctl1_6())
                .field("csi_ctl1_7", &self.csi_ctl1_7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CsiCtl01 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CsiCtl01 {{ csi_ctl0_0: {=bool:?}, csi_ctl0_1: {=bool:?}, csi_ctl0_2: {=bool:?}, csi_ctl0_3: {=bool:?}, csi_ctl0_4: {=bool:?}, csi_ctl0_5: {=u8:?}, csi_ctl0_6: {=u8:?}, csi_ctl0_7: {=bool:?}, csi_ctl1_0: {=bool:?}, csi_ctl1_1: {=bool:?}, csi_ctl1_2: {=bool:?}, csi_ctl1_3: {=bool:?}, csi_ctl1_4: {=bool:?}, csi_ctl1_5: {=u8:?}, csi_ctl1_6: {=u8:?}, csi_ctl1_7: {=bool:?} }}" , self . csi_ctl0_0 () , self . csi_ctl0_1 () , self . csi_ctl0_2 () , self . csi_ctl0_3 () , self . csi_ctl0_4 () , self . csi_ctl0_5 () , self . csi_ctl0_6 () , self . csi_ctl0_7 () , self . csi_ctl1_0 () , self . csi_ctl1_1 () , self . csi_ctl1_2 () , self . csi_ctl1_3 () , self . csi_ctl1_4 () , self . csi_ctl1_5 () , self . csi_ctl1_6 () , self . csi_ctl1_7 ())
        }
    }
    #[doc = "dphy hardcore control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CsiCtl23(pub u32);
    impl CsiCtl23 {
        #[doc = "data lane-0 hs-rx skew adjust with binary code."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl3_0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "data lane-0 hs-rx skew adjust with binary code."]
        #[inline(always)]
        pub const fn set_csi_ctl3_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "data lane-0 skew trimming enable."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl3_1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "data lane-0 skew trimming enable."]
        #[inline(always)]
        pub const fn set_csi_ctl3_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "data lane-1 hs-rx skew adjust with binary code."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl3_2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "data lane-1 hs-rx skew adjust with binary code."]
        #[inline(always)]
        pub const fn set_csi_ctl3_2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "data lane-1 skew trimming enable."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_ctl3_3(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "data lane-1 skew trimming enable."]
        #[inline(always)]
        pub const fn set_csi_ctl3_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for CsiCtl23 {
        #[inline(always)]
        fn default() -> CsiCtl23 {
            CsiCtl23(0)
        }
    }
    impl core::fmt::Debug for CsiCtl23 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CsiCtl23")
                .field("csi_ctl3_0", &self.csi_ctl3_0())
                .field("csi_ctl3_1", &self.csi_ctl3_1())
                .field("csi_ctl3_2", &self.csi_ctl3_2())
                .field("csi_ctl3_3", &self.csi_ctl3_3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CsiCtl23 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CsiCtl23 {{ csi_ctl3_0: {=u8:?}, csi_ctl3_1: {=bool:?}, csi_ctl3_2: {=u8:?}, csi_ctl3_3: {=bool:?} }}" , self . csi_ctl3_0 () , self . csi_ctl3_1 () , self . csi_ctl3_2 () , self . csi_ctl3_3 ())
        }
    }
    #[doc = "ulp lp-rx input threshold voltage trimming for data lane."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CsiVinit(pub u32);
    impl CsiVinit {
        #[doc = "pt ft indicator in csi lane-0."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_0_lprx_vinit(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "pt ft indicator in csi lane-0."]
        #[inline(always)]
        pub const fn set_csi_0_lprx_vinit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "pt ft indicator in csi lane-1."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_1_lprx_vinit(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "pt ft indicator in csi lane-1."]
        #[inline(always)]
        pub const fn set_csi_1_lprx_vinit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "pt ft indicator in csi clk lane."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_clk_lprx_vint(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "pt ft indicator in csi clk lane."]
        #[inline(always)]
        pub const fn set_csi_clk_lprx_vint(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "pt ft indicator in csi clk data lane."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_lprx_vref_trim(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "pt ft indicator in csi clk data lane."]
        #[inline(always)]
        pub const fn set_csi_lprx_vref_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for CsiVinit {
        #[inline(always)]
        fn default() -> CsiVinit {
            CsiVinit(0)
        }
    }
    impl core::fmt::Debug for CsiVinit {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CsiVinit")
                .field("csi_0_lprx_vinit", &self.csi_0_lprx_vinit())
                .field("csi_1_lprx_vinit", &self.csi_1_lprx_vinit())
                .field("csi_clk_lprx_vint", &self.csi_clk_lprx_vint())
                .field("csi_lprx_vref_trim", &self.csi_lprx_vref_trim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CsiVinit {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CsiVinit {{ csi_0_lprx_vinit: {=u8:?}, csi_1_lprx_vinit: {=u8:?}, csi_clk_lprx_vint: {=u8:?}, csi_lprx_vref_trim: {=u8:?} }}" , self . csi_0_lprx_vinit () , self . csi_1_lprx_vinit () , self . csi_clk_lprx_vint () , self . csi_lprx_vref_trim ())
        }
    }
    #[doc = "the hardcore interface control in debug mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DebugCfgReg0(pub u32);
    impl DebugCfgReg0 {
        #[doc = "debug config register0."]
        #[must_use]
        #[inline(always)]
        pub const fn debug_cfg_reg0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "debug config register0."]
        #[inline(always)]
        pub const fn set_debug_cfg_reg0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DebugCfgReg0 {
        #[inline(always)]
        fn default() -> DebugCfgReg0 {
            DebugCfgReg0(0)
        }
    }
    impl core::fmt::Debug for DebugCfgReg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DebugCfgReg0")
                .field("debug_cfg_reg0", &self.debug_cfg_reg0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DebugCfgReg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DebugCfgReg0 {{ debug_cfg_reg0: {=u32:?} }}",
                self.debug_cfg_reg0()
            )
        }
    }
    #[doc = "the hardcore interface control in debug mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DebugCfgReg1(pub u32);
    impl DebugCfgReg1 {
        #[doc = "debug config register1."]
        #[must_use]
        #[inline(always)]
        pub const fn debug_cfg_reg1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "debug config register1."]
        #[inline(always)]
        pub const fn set_debug_cfg_reg1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DebugCfgReg1 {
        #[inline(always)]
        fn default() -> DebugCfgReg1 {
            DebugCfgReg1(0)
        }
    }
    impl core::fmt::Debug for DebugCfgReg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DebugCfgReg1")
                .field("debug_cfg_reg1", &self.debug_cfg_reg1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DebugCfgReg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DebugCfgReg1 {{ debug_cfg_reg1: {=u32:?} }}",
                self.debug_cfg_reg1()
            )
        }
    }
    #[doc = "the hardcore interface control in debug mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DebugCfgReg2(pub u32);
    impl DebugCfgReg2 {
        #[doc = "debug config register2."]
        #[must_use]
        #[inline(always)]
        pub const fn debug_cfg_reg2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "debug config register2."]
        #[inline(always)]
        pub const fn set_debug_cfg_reg2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DebugCfgReg2 {
        #[inline(always)]
        fn default() -> DebugCfgReg2 {
            DebugCfgReg2(0)
        }
    }
    impl core::fmt::Debug for DebugCfgReg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DebugCfgReg2")
                .field("debug_cfg_reg2", &self.debug_cfg_reg2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DebugCfgReg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DebugCfgReg2 {{ debug_cfg_reg2: {=u32:?} }}",
                self.debug_cfg_reg2()
            )
        }
    }
    #[doc = "the hardcore interface control in debug mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DebugCfgReg3(pub u32);
    impl DebugCfgReg3 {
        #[doc = "debug config register3."]
        #[must_use]
        #[inline(always)]
        pub const fn debug_cfg_reg3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "debug config register3."]
        #[inline(always)]
        pub const fn set_debug_cfg_reg3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DebugCfgReg3 {
        #[inline(always)]
        fn default() -> DebugCfgReg3 {
            DebugCfgReg3(0)
        }
    }
    impl core::fmt::Debug for DebugCfgReg3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DebugCfgReg3")
                .field("debug_cfg_reg3", &self.debug_cfg_reg3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DebugCfgReg3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DebugCfgReg3 {{ debug_cfg_reg3: {=u32:?} }}",
                self.debug_cfg_reg3()
            )
        }
    }
    #[doc = "the hardcore interface control in debug mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DebugCfgReg4(pub u32);
    impl DebugCfgReg4 {
        #[doc = "debug config register4."]
        #[must_use]
        #[inline(always)]
        pub const fn debug_cfg_reg4(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "debug config register4."]
        #[inline(always)]
        pub const fn set_debug_cfg_reg4(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DebugCfgReg4 {
        #[inline(always)]
        fn default() -> DebugCfgReg4 {
            DebugCfgReg4(0)
        }
    }
    impl core::fmt::Debug for DebugCfgReg4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DebugCfgReg4")
                .field("debug_cfg_reg4", &self.debug_cfg_reg4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DebugCfgReg4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DebugCfgReg4 {{ debug_cfg_reg4: {=u32:?} }}",
                self.debug_cfg_reg4()
            )
        }
    }
    #[doc = "the hardcore interface control in debug mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DebugCfgReg5(pub u32);
    impl DebugCfgReg5 {
        #[doc = "debug config register5."]
        #[must_use]
        #[inline(always)]
        pub const fn debug_cfg_reg5(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "debug config register5."]
        #[inline(always)]
        pub const fn set_debug_cfg_reg5(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DebugCfgReg5 {
        #[inline(always)]
        fn default() -> DebugCfgReg5 {
            DebugCfgReg5(0)
        }
    }
    impl core::fmt::Debug for DebugCfgReg5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DebugCfgReg5")
                .field("debug_cfg_reg5", &self.debug_cfg_reg5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DebugCfgReg5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DebugCfgReg5 {{ debug_cfg_reg5: {=u32:?} }}",
                self.debug_cfg_reg5()
            )
        }
    }
    #[doc = "debug data control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DebugInfo(pub u32);
    impl DebugInfo {
        #[doc = "the debug bus sel."]
        #[must_use]
        #[inline(always)]
        pub const fn debug_mode_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "the debug bus sel."]
        #[inline(always)]
        pub const fn set_debug_mode_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for DebugInfo {
        #[inline(always)]
        fn default() -> DebugInfo {
            DebugInfo(0)
        }
    }
    impl core::fmt::Debug for DebugInfo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DebugInfo")
                .field("debug_mode_sel", &self.debug_mode_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DebugInfo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DebugInfo {{ debug_mode_sel: {=u8:?} }}",
                self.debug_mode_sel()
            )
        }
    }
    #[doc = "misc info of dphyrx_pcs control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MiscInfo(pub u32);
    impl MiscInfo {
        #[doc = "at least six zero is checked before sot swquence \"00011101\"."]
        #[must_use]
        #[inline(always)]
        pub const fn long_sotsync_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "at least six zero is checked before sot swquence \"00011101\"."]
        #[inline(always)]
        pub const fn set_long_sotsync_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the lp10 select signal in ulps_exit state."]
        #[must_use]
        #[inline(always)]
        pub const fn ulps_lp10_sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the lp10 select signal in ulps_exit state."]
        #[inline(always)]
        pub const fn set_ulps_lp10_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for MiscInfo {
        #[inline(always)]
        fn default() -> MiscInfo {
            MiscInfo(0)
        }
    }
    impl core::fmt::Debug for MiscInfo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MiscInfo")
                .field("long_sotsync_en", &self.long_sotsync_en())
                .field("ulps_lp10_sel", &self.ulps_lp10_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MiscInfo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MiscInfo {{ long_sotsync_en: {=bool:?}, ulps_lp10_sel: {=bool:?} }}",
                self.long_sotsync_en(),
                self.ulps_lp10_sel()
            )
        }
    }
    #[doc = "lane swap and dp/dn swap select."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NeSwap(pub u32);
    impl NeSwap {
        #[doc = "data lane0 swap."]
        #[must_use]
        #[inline(always)]
        pub const fn lane_swap_lane0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "data lane0 swap."]
        #[inline(always)]
        pub const fn set_lane_swap_lane0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "data lane1 swap."]
        #[must_use]
        #[inline(always)]
        pub const fn lane_swap_lan1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "data lane1 swap."]
        #[inline(always)]
        pub const fn set_lane_swap_lan1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "datalane0 dpdn swap."]
        #[must_use]
        #[inline(always)]
        pub const fn dpdn_swap_lan0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "datalane0 dpdn swap."]
        #[inline(always)]
        pub const fn set_dpdn_swap_lan0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "datalane1 dpdn swap."]
        #[must_use]
        #[inline(always)]
        pub const fn dpdn_swap_lane1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "datalane1 dpdn swap."]
        #[inline(always)]
        pub const fn set_dpdn_swap_lane1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for NeSwap {
        #[inline(always)]
        fn default() -> NeSwap {
            NeSwap(0)
        }
    }
    impl core::fmt::Debug for NeSwap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NeSwap")
                .field("lane_swap_lane0", &self.lane_swap_lane0())
                .field("lane_swap_lan1", &self.lane_swap_lan1())
                .field("dpdn_swap_lan0", &self.dpdn_swap_lan0())
                .field("dpdn_swap_lane1", &self.dpdn_swap_lane1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NeSwap {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "NeSwap {{ lane_swap_lane0: {=u8:?}, lane_swap_lan1: {=u8:?}, dpdn_swap_lan0: {=bool:?}, dpdn_swap_lane1: {=bool:?} }}" , self . lane_swap_lane0 () , self . lane_swap_lan1 () , self . dpdn_swap_lan0 () , self . dpdn_swap_lane1 ())
        }
    }
    #[doc = "dphy resistor calibration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyRcal(pub u32);
    impl PhyRcal {
        #[doc = "enable hs-rx terminal trimming."]
        #[must_use]
        #[inline(always)]
        pub const fn rcal_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable hs-rx terminal trimming."]
        #[inline(always)]
        pub const fn set_rcal_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "default value of HS-RX terminal configure."]
        #[must_use]
        #[inline(always)]
        pub const fn rcal_trim(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x0f;
            val as u8
        }
        #[doc = "default value of HS-RX terminal configure."]
        #[inline(always)]
        pub const fn set_rcal_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
        }
        #[doc = "rcal function control."]
        #[must_use]
        #[inline(always)]
        pub const fn rcal_ctl(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0xff;
            val as u8
        }
        #[doc = "rcal function control."]
        #[inline(always)]
        pub const fn set_rcal_ctl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
        }
        #[doc = "hs-rx terminal trimming results."]
        #[must_use]
        #[inline(always)]
        pub const fn rcal_out(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "hs-rx terminal trimming results."]
        #[inline(always)]
        pub const fn set_rcal_out(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
        #[doc = "hs-rx terminal trimming done indicator signal."]
        #[must_use]
        #[inline(always)]
        pub const fn rcal_done(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "hs-rx terminal trimming done indicator signal."]
        #[inline(always)]
        pub const fn set_rcal_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for PhyRcal {
        #[inline(always)]
        fn default() -> PhyRcal {
            PhyRcal(0)
        }
    }
    impl core::fmt::Debug for PhyRcal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyRcal")
                .field("rcal_en", &self.rcal_en())
                .field("rcal_trim", &self.rcal_trim())
                .field("rcal_ctl", &self.rcal_ctl())
                .field("rcal_out", &self.rcal_out())
                .field("rcal_done", &self.rcal_done())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyRcal {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PhyRcal {{ rcal_en: {=bool:?}, rcal_trim: {=u8:?}, rcal_ctl: {=u8:?}, rcal_out: {=u8:?}, rcal_done: {=bool:?} }}" , self . rcal_en () , self . rcal_trim () , self . rcal_ctl () , self . rcal_out () , self . rcal_done ())
        }
    }
    #[doc = "soft reset control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SoftRst(pub u32);
    impl SoftRst {
        #[doc = "the soft reset of clk_cfg domain."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_clk_soft_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "the soft reset of clk_cfg domain."]
        #[inline(always)]
        pub const fn set_cfg_clk_soft_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the soft reset of clk_hs domain."]
        #[must_use]
        #[inline(always)]
        pub const fn hs_clk_soft_rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the soft reset of clk_hs domain."]
        #[inline(always)]
        pub const fn set_hs_clk_soft_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SoftRst {
        #[inline(always)]
        fn default() -> SoftRst {
            SoftRst(0)
        }
    }
    impl core::fmt::Debug for SoftRst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SoftRst")
                .field("cfg_clk_soft_rst", &self.cfg_clk_soft_rst())
                .field("hs_clk_soft_rst", &self.hs_clk_soft_rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SoftRst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SoftRst {{ cfg_clk_soft_rst: {=bool:?}, hs_clk_soft_rst: {=bool:?} }}",
                self.cfg_clk_soft_rst(),
                self.hs_clk_soft_rst()
            )
        }
    }
    #[doc = "t-init of clock lane."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TClaneInit(pub u32);
    impl TClaneInit {
        #[doc = "initialization time of lock lane."]
        #[must_use]
        #[inline(always)]
        pub const fn t_clk_init(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "initialization time of lock lane."]
        #[inline(always)]
        pub const fn set_t_clk_init(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for TClaneInit {
        #[inline(always)]
        fn default() -> TClaneInit {
            TClaneInit(0)
        }
    }
    impl core::fmt::Debug for TClaneInit {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TClaneInit")
                .field("t_clk_init", &self.t_clk_init())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TClaneInit {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TClaneInit {{ t_clk_init: {=u32:?} }}",
                self.t_clk_init()
            )
        }
    }
    #[doc = "t-settle of all data lanes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct THsSettle(pub u32);
    impl THsSettle {
        #[doc = "the value of ths-settle of data lane0."]
        #[must_use]
        #[inline(always)]
        pub const fn t_d0_settle(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the value of ths-settle of data lane0."]
        #[inline(always)]
        pub const fn set_t_d0_settle(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the value of ths-settle of data lane1."]
        #[must_use]
        #[inline(always)]
        pub const fn t_d1_settle(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the value of ths-settle of data lane1."]
        #[inline(always)]
        pub const fn set_t_d1_settle(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for THsSettle {
        #[inline(always)]
        fn default() -> THsSettle {
            THsSettle(0)
        }
    }
    impl core::fmt::Debug for THsSettle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("THsSettle")
                .field("t_d0_settle", &self.t_d0_settle())
                .field("t_d1_settle", &self.t_d1_settle())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for THsSettle {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "THsSettle {{ t_d0_settle: {=u8:?}, t_d1_settle: {=u8:?} }}",
                self.t_d0_settle(),
                self.t_d1_settle()
            )
        }
    }
    #[doc = "t-termen of all datalane."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct THsTermen(pub u32);
    impl THsTermen {
        #[doc = "the value of ths-termen of datalane0."]
        #[must_use]
        #[inline(always)]
        pub const fn t_d0_termen(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the value of ths-termen of datalane0."]
        #[inline(always)]
        pub const fn set_t_d0_termen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the value of ths-termen of datalane1."]
        #[must_use]
        #[inline(always)]
        pub const fn t_d1_termen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the value of ths-termen of datalane1."]
        #[inline(always)]
        pub const fn set_t_d1_termen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for THsTermen {
        #[inline(always)]
        fn default() -> THsTermen {
            THsTermen(0)
        }
    }
    impl core::fmt::Debug for THsTermen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("THsTermen")
                .field("t_d0_termen", &self.t_d0_termen())
                .field("t_d1_termen", &self.t_d1_termen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for THsTermen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "THsTermen {{ t_d0_termen: {=u8:?}, t_d1_termen: {=u8:?} }}",
                self.t_d0_termen(),
                self.t_d1_termen()
            )
        }
    }
    #[doc = "t-init of data lane0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TLaneInit0(pub u32);
    impl TLaneInit0 {
        #[doc = "initialization time of data lane."]
        #[must_use]
        #[inline(always)]
        pub const fn t_d0_init(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "initialization time of data lane."]
        #[inline(always)]
        pub const fn set_t_d0_init(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for TLaneInit0 {
        #[inline(always)]
        fn default() -> TLaneInit0 {
            TLaneInit0(0)
        }
    }
    impl core::fmt::Debug for TLaneInit0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TLaneInit0")
                .field("t_d0_init", &self.t_d0_init())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TLaneInit0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TLaneInit0 {{ t_d0_init: {=u32:?} }}", self.t_d0_init())
        }
    }
    #[doc = "t-init of data lane1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TLaneInit1(pub u32);
    impl TLaneInit1 {
        #[doc = "initialization time of data lane."]
        #[must_use]
        #[inline(always)]
        pub const fn t_d1_init(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "initialization time of data lane."]
        #[inline(always)]
        pub const fn set_t_d1_init(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for TLaneInit1 {
        #[inline(always)]
        fn default() -> TLaneInit1 {
            TLaneInit1(0)
        }
    }
    impl core::fmt::Debug for TLaneInit1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TLaneInit1")
                .field("t_d1_init", &self.t_d1_init())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TLaneInit1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TLaneInit1 {{ t_d1_init: {=u32:?} }}", self.t_d1_init())
        }
    }
    #[doc = "the time of tlpx_ctrl of all lane."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TlpxCtrl(pub u32);
    impl TlpxCtrl {
        #[doc = "the width of tlpx."]
        #[must_use]
        #[inline(always)]
        pub const fn tlpx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the width of tlpx."]
        #[inline(always)]
        pub const fn set_tlpx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "enable the tlpx width check."]
        #[must_use]
        #[inline(always)]
        pub const fn en_tlpx_check(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "enable the tlpx width check."]
        #[inline(always)]
        pub const fn set_en_tlpx_check(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for TlpxCtrl {
        #[inline(always)]
        fn default() -> TlpxCtrl {
            TlpxCtrl(0)
        }
    }
    impl core::fmt::Debug for TlpxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TlpxCtrl")
                .field("tlpx", &self.tlpx())
                .field("en_tlpx_check", &self.en_tlpx_check())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TlpxCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TlpxCtrl {{ tlpx: {=u8:?}, en_tlpx_check: {=bool:?} }}",
                self.tlpx(),
                self.en_tlpx_check()
            )
        }
    }
    #[doc = "enable lprx and ulprx."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UlpRxEn(pub u32);
    impl UlpRxEn {
        #[doc = "clock lane lp=rx receiver enable control."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_clk_lprx_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "clock lane lp=rx receiver enable control."]
        #[inline(always)]
        pub const fn set_csi_clk_lprx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "data lane1 lp-rx receiver enable control."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_1_lprx_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "data lane1 lp-rx receiver enable control."]
        #[inline(always)]
        pub const fn set_csi_1_lprx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "clock lane ulp-rx receiver enable control."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_clk_ulprx_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "clock lane ulp-rx receiver enable control."]
        #[inline(always)]
        pub const fn set_csi_clk_ulprx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "data lane0 ulp-rx receiver enable control."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_0_ulprx_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "data lane0 ulp-rx receiver enable control."]
        #[inline(always)]
        pub const fn set_csi_0_ulprx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "data lane1 ulp-rx receiver enable control."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_1_ulprx_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "data lane1 ulp-rx receiver enable control."]
        #[inline(always)]
        pub const fn set_csi_1_ulprx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for UlpRxEn {
        #[inline(always)]
        fn default() -> UlpRxEn {
            UlpRxEn(0)
        }
    }
    impl core::fmt::Debug for UlpRxEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UlpRxEn")
                .field("csi_clk_lprx_en", &self.csi_clk_lprx_en())
                .field("csi_1_lprx_en", &self.csi_1_lprx_en())
                .field("csi_clk_ulprx_en", &self.csi_clk_ulprx_en())
                .field("csi_0_ulprx_en", &self.csi_0_ulprx_en())
                .field("csi_1_ulprx_en", &self.csi_1_ulprx_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UlpRxEn {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "UlpRxEn {{ csi_clk_lprx_en: {=bool:?}, csi_1_lprx_en: {=bool:?}, csi_clk_ulprx_en: {=bool:?}, csi_0_ulprx_en: {=bool:?}, csi_1_ulprx_en: {=bool:?} }}" , self . csi_clk_lprx_en () , self . csi_1_lprx_en () , self . csi_clk_ulprx_en () , self . csi_0_ulprx_en () , self . csi_1_ulprx_en ())
        }
    }
    #[doc = "hs-rx dc-offset auto-calibration results."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VoffcalOut(pub u32);
    impl VoffcalOut {
        #[doc = "data lane1 hs-rx dc-offset auto-calibration result."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_1_voffcal_out(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x1f;
            val as u8
        }
        #[doc = "data lane1 hs-rx dc-offset auto-calibration result."]
        #[inline(always)]
        pub const fn set_csi_1_voffcal_out(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
        }
        #[doc = "data lane1 hs-rx dc-offset auto-calibration done."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_1_voffcal_done(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "data lane1 hs-rx dc-offset auto-calibration done."]
        #[inline(always)]
        pub const fn set_csi_1_voffcal_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "data lane0 hs-rx dc-offset auto-calibration result."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_o_voffcal_out(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x1f;
            val as u8
        }
        #[doc = "data lane0 hs-rx dc-offset auto-calibration result."]
        #[inline(always)]
        pub const fn set_csi_o_voffcal_out(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
        }
        #[doc = "data lane0 hs-rx dc-offset auto-calibration done."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_0_voffcal_done(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "data lane0 hs-rx dc-offset auto-calibration done."]
        #[inline(always)]
        pub const fn set_csi_0_voffcal_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "clock lane hs-rx dc-offset auto-calibration results."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_clk_voffcal_out(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "clock lane hs-rx dc-offset auto-calibration results."]
        #[inline(always)]
        pub const fn set_csi_clk_voffcal_out(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "clock lane hs-rx dc-offset auto-calibration done."]
        #[must_use]
        #[inline(always)]
        pub const fn csi_clk_voffcal_done(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "clock lane hs-rx dc-offset auto-calibration done."]
        #[inline(always)]
        pub const fn set_csi_clk_voffcal_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for VoffcalOut {
        #[inline(always)]
        fn default() -> VoffcalOut {
            VoffcalOut(0)
        }
    }
    impl core::fmt::Debug for VoffcalOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("VoffcalOut")
                .field("csi_1_voffcal_out", &self.csi_1_voffcal_out())
                .field("csi_1_voffcal_done", &self.csi_1_voffcal_done())
                .field("csi_o_voffcal_out", &self.csi_o_voffcal_out())
                .field("csi_0_voffcal_done", &self.csi_0_voffcal_done())
                .field("csi_clk_voffcal_out", &self.csi_clk_voffcal_out())
                .field("csi_clk_voffcal_done", &self.csi_clk_voffcal_done())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for VoffcalOut {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "VoffcalOut {{ csi_1_voffcal_out: {=u8:?}, csi_1_voffcal_done: {=bool:?}, csi_o_voffcal_out: {=u8:?}, csi_0_voffcal_done: {=bool:?}, csi_clk_voffcal_out: {=u8:?}, csi_clk_voffcal_done: {=bool:?} }}" , self . csi_1_voffcal_out () , self . csi_1_voffcal_done () , self . csi_o_voffcal_out () , self . csi_0_voffcal_done () , self . csi_clk_voffcal_out () , self . csi_clk_voffcal_done ())
        }
    }
}
