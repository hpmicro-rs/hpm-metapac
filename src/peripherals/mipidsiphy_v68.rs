#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "MIPI_DSI_PHY0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MipiDsiPhy {
    ptr: *mut u8,
}
unsafe impl Send for MipiDsiPhy {}
unsafe impl Sync for MipiDsiPhy {}
impl MipiDsiPhy {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "timer counter about clock lane parameter."]
    #[inline(always)]
    pub const fn clane_para0(self) -> crate::common::Reg<regs::ClanePara0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[inline(always)]
    pub const fn clane_para1(self) -> crate::common::Reg<regs::ClanePara1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[inline(always)]
    pub const fn clane_para2(self) -> crate::common::Reg<regs::ClanePara2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[inline(always)]
    pub const fn clane_para3(self) -> crate::common::Reg<regs::ClanePara3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[inline(always)]
    pub const fn dlane0_para0(self) -> crate::common::Reg<regs::Dlane0Para0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[inline(always)]
    pub const fn dlane0_para1(self) -> crate::common::Reg<regs::Dlane0Para1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[inline(always)]
    pub const fn dlane0_para2(self) -> crate::common::Reg<regs::Dlane0Para2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[inline(always)]
    pub const fn dlane0_para3(self) -> crate::common::Reg<regs::Dlane0Para3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[inline(always)]
    pub const fn dlane0_para4(self) -> crate::common::Reg<regs::Dlane0Para4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[inline(always)]
    pub const fn dlane1_para0(self) -> crate::common::Reg<regs::Dlane1Para0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[inline(always)]
    pub const fn dlane1_para1(self) -> crate::common::Reg<regs::Dlane1Para1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[inline(always)]
    pub const fn dlane1_para2(self) -> crate::common::Reg<regs::Dlane1Para2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[inline(always)]
    pub const fn dlane1_para3(self) -> crate::common::Reg<regs::Dlane1Para3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[inline(always)]
    pub const fn dlane2_para0(self) -> crate::common::Reg<regs::Dlane2Para0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[inline(always)]
    pub const fn dlane2_para1(self) -> crate::common::Reg<regs::Dlane2Para1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[inline(always)]
    pub const fn dlane2_para2(self) -> crate::common::Reg<regs::Dlane2Para2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[inline(always)]
    pub const fn dlane2_para3(self) -> crate::common::Reg<regs::Dlane2Para3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[inline(always)]
    pub const fn dlane3_para0(self) -> crate::common::Reg<regs::Dlane3Para0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[inline(always)]
    pub const fn dlane3_para1(self) -> crate::common::Reg<regs::Dlane3Para1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[inline(always)]
    pub const fn dlane3_para2(self) -> crate::common::Reg<regs::Dlane3Para2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[inline(always)]
    pub const fn dlane3_para3(self) -> crate::common::Reg<regs::Dlane3Para3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "timing parameter for all lanes."]
    #[inline(always)]
    pub const fn common_para0(self) -> crate::common::Reg<regs::CommonPara0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "dphy control parameter."]
    #[inline(always)]
    pub const fn ctrl_para0(self) -> crate::common::Reg<regs::CtrlPara0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "dphy pll control parameter."]
    #[inline(always)]
    pub const fn pll_ctrl_para0(self) -> crate::common::Reg<regs::PllCtrlPara0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "dphy calibration control parameter."]
    #[inline(always)]
    pub const fn rcal_ctrl(self) -> crate::common::Reg<regs::RcalCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "dphy trimming parameter."]
    #[inline(always)]
    pub const fn trim_para(self) -> crate::common::Reg<regs::TrimPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "dphy test control parameter."]
    #[inline(always)]
    pub const fn test_para0(self) -> crate::common::Reg<regs::TestPara0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "dphy bist test control parameter."]
    #[inline(always)]
    pub const fn test_para1(self) -> crate::common::Reg<regs::TestPara1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "dphy control parameter."]
    #[inline(always)]
    pub const fn misc_para(self) -> crate::common::Reg<regs::MiscPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "dphy clock lane control parameter."]
    #[inline(always)]
    pub const fn clane_para4(self) -> crate::common::Reg<regs::ClanePara4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "dphy clock lane control parameter."]
    #[inline(always)]
    pub const fn interface_para(
        self,
    ) -> crate::common::Reg<regs::InterfacePara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "reserved the pins for pcs."]
    #[inline(always)]
    pub const fn pcs_reserved_pin_para(
        self,
    ) -> crate::common::Reg<regs::PcsReservedPinPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "parallel data about clock lane parameter."]
    #[inline(always)]
    pub const fn clane_data_para(
        self,
    ) -> crate::common::Reg<regs::ClaneDataPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "pma about clock lane select parameter."]
    #[inline(always)]
    pub const fn pma_lane_sel_para(
        self,
    ) -> crate::common::Reg<regs::PmaLaneSelPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
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
    #[doc = "parallel data about clock lane parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClaneDataPara(pub u32);
    impl ClaneDataPara {
        #[doc = "the parallel data about clock lane."]
        #[must_use]
        #[inline(always)]
        pub const fn clane_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the parallel data about clock lane."]
        #[inline(always)]
        pub const fn set_clane_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "select the data about clock lane."]
        #[must_use]
        #[inline(always)]
        pub const fn clane_data_sel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "select the data about clock lane."]
        #[inline(always)]
        pub const fn set_clane_data_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for ClaneDataPara {
        #[inline(always)]
        fn default() -> ClaneDataPara {
            ClaneDataPara(0)
        }
    }
    impl core::fmt::Debug for ClaneDataPara {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClaneDataPara")
                .field("clane_data", &self.clane_data())
                .field("clane_data_sel", &self.clane_data_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ClaneDataPara {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ClaneDataPara {{ clane_data: {=u8:?}, clane_data_sel: {=bool:?} }}",
                self.clane_data(),
                self.clane_data_sel()
            )
        }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClanePara0(pub u32);
    impl ClanePara0 {
        #[doc = "the soft reset of clk_cfg domain."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rst2enlptx_c(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the soft reset of clk_cfg domain."]
        #[inline(always)]
        pub const fn set_t_rst2enlptx_c(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ClanePara0 {
        #[inline(always)]
        fn default() -> ClanePara0 {
            ClanePara0(0)
        }
    }
    impl core::fmt::Debug for ClanePara0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClanePara0")
                .field("t_rst2enlptx_c", &self.t_rst2enlptx_c())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ClanePara0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ClanePara0 {{ t_rst2enlptx_c: {=u16:?} }}",
                self.t_rst2enlptx_c()
            )
        }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClanePara1(pub u32);
    impl ClanePara1 {
        #[doc = "the number of byteclk cycles that clklane drive LP-11 during initialization period."]
        #[must_use]
        #[inline(always)]
        pub const fn t_inittime_c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles that clklane drive LP-11 during initialization period."]
        #[inline(always)]
        pub const fn set_t_inittime_c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ClanePara1 {
        #[inline(always)]
        fn default() -> ClanePara1 {
            ClanePara1(0)
        }
    }
    impl core::fmt::Debug for ClanePara1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClanePara1")
                .field("t_inittime_c", &self.t_inittime_c())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ClanePara1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ClanePara1 {{ t_inittime_c: {=u32:?} }}",
                self.t_inittime_c()
            )
        }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClanePara2(pub u32);
    impl ClanePara2 {
        #[doc = "the number of byteclk cycles that hs clock shall be driven prior to data lane beginning the transition from lp to hs mode."]
        #[must_use]
        #[inline(always)]
        pub const fn t_clkpre_c(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that hs clock shall be driven prior to data lane beginning the transition from lp to hs mode."]
        #[inline(always)]
        pub const fn set_t_clkpre_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that clock lane clkp/n lines are at the hs-zero state hs-0 during a hs clock transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_clkzero_c(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that clock lane clkp/n lines are at the hs-zero state hs-0 during a hs clock transmission."]
        #[inline(always)]
        pub const fn set_t_clkzero_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that clock lane clkp/n lines are at the hs prepare state lp-00 during a hs clock transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_clkprepare_c(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that clock lane clkp/n lines are at the hs prepare state lp-00 during a hs clock transmission."]
        #[inline(always)]
        pub const fn set_t_clkprepare_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ClanePara2 {
        #[inline(always)]
        fn default() -> ClanePara2 {
            ClanePara2(0)
        }
    }
    impl core::fmt::Debug for ClanePara2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClanePara2")
                .field("t_clkpre_c", &self.t_clkpre_c())
                .field("t_clkzero_c", &self.t_clkzero_c())
                .field("t_clkprepare_c", &self.t_clkprepare_c())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ClanePara2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ClanePara2 {{ t_clkpre_c: {=u8:?}, t_clkzero_c: {=u8:?}, t_clkprepare_c: {=u8:?} }}" , self . t_clkpre_c () , self . t_clkzero_c () , self . t_clkprepare_c ())
        }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClanePara3(pub u32);
    impl ClanePara3 {
        #[doc = "the number of byteclk cycles that the clock lane clkp/n lines are at hs-exit state after a hs clock transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hsexit_c(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the clock lane clkp/n lines are at hs-exit state after a hs clock transmission."]
        #[inline(always)]
        pub const fn set_t_hsexit_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that the clock lane clkp/n lines are at state hs-tail sate hs-0 during a hs clock transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_clktrial_c(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the clock lane clkp/n lines are at state hs-tail sate hs-0 during a hs clock transmission."]
        #[inline(always)]
        pub const fn set_t_clktrial_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that the clock lane should keep sending the hs-clock after the last associated data lane has transitioned to LP mode."]
        #[must_use]
        #[inline(always)]
        pub const fn t_clkpost_c(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the clock lane should keep sending the hs-clock after the last associated data lane has transitioned to LP mode."]
        #[inline(always)]
        pub const fn set_t_clkpost_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ClanePara3 {
        #[inline(always)]
        fn default() -> ClanePara3 {
            ClanePara3(0)
        }
    }
    impl core::fmt::Debug for ClanePara3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClanePara3")
                .field("t_hsexit_c", &self.t_hsexit_c())
                .field("t_clktrial_c", &self.t_clktrial_c())
                .field("t_clkpost_c", &self.t_clkpost_c())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ClanePara3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ClanePara3 {{ t_hsexit_c: {=u8:?}, t_clktrial_c: {=u8:?}, t_clkpost_c: {=u8:?} }}",
                self.t_hsexit_c(),
                self.t_clktrial_c(),
                self.t_clkpost_c()
            )
        }
    }
    #[doc = "dphy clock lane control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClanePara4(pub u32);
    impl ClanePara4 {
        #[doc = "the number of byteclk cycles from exiting ultra low power state to enabling the low-power driver."]
        #[must_use]
        #[inline(always)]
        pub const fn t_wakeup_c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles from exiting ultra low power state to enabling the low-power driver."]
        #[inline(always)]
        pub const fn set_t_wakeup_c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ClanePara4 {
        #[inline(always)]
        fn default() -> ClanePara4 {
            ClanePara4(0)
        }
    }
    impl core::fmt::Debug for ClanePara4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClanePara4")
                .field("t_wakeup_c", &self.t_wakeup_c())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ClanePara4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ClanePara4 {{ t_wakeup_c: {=u32:?} }}",
                self.t_wakeup_c()
            )
        }
    }
    #[doc = "timing parameter for all lanes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CommonPara0(pub u32);
    impl CommonPara0 {
        #[doc = "the number of byteclk cycles of transmitted length of any low-power state period."]
        #[must_use]
        #[inline(always)]
        pub const fn t_lpx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles of transmitted length of any low-power state period."]
        #[inline(always)]
        pub const fn set_t_lpx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CommonPara0 {
        #[inline(always)]
        fn default() -> CommonPara0 {
            CommonPara0(0)
        }
    }
    impl core::fmt::Debug for CommonPara0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CommonPara0")
                .field("t_lpx", &self.t_lpx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CommonPara0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CommonPara0 {{ t_lpx: {=u8:?} }}", self.t_lpx())
        }
    }
    #[doc = "dphy control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlPara0(pub u32);
    impl CtrlPara0 {
        #[doc = "power down all modules inside su includes ivref, r-calibration and pll, high effective."]
        #[must_use]
        #[inline(always)]
        pub const fn su_iddq_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "power down all modules inside su includes ivref, r-calibration and pll, high effective."]
        #[inline(always)]
        pub const fn set_su_iddq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "power on all dsi lane."]
        #[must_use]
        #[inline(always)]
        pub const fn pwon_dsi(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "power on all dsi lane."]
        #[inline(always)]
        pub const fn set_pwon_dsi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "power on pll high active."]
        #[must_use]
        #[inline(always)]
        pub const fn pwon_pll(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "power on pll high active."]
        #[inline(always)]
        pub const fn set_pwon_pll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "select the cource of PMA power on control signals."]
        #[must_use]
        #[inline(always)]
        pub const fn pwon_sel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "select the cource of PMA power on control signals."]
        #[inline(always)]
        pub const fn set_pwon_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "lp-cd enable for lane0."]
        #[must_use]
        #[inline(always)]
        pub const fn en_lpcd_d0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "lp-cd enable for lane0."]
        #[inline(always)]
        pub const fn set_en_lpcd_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "lp-rx enable for lane0."]
        #[must_use]
        #[inline(always)]
        pub const fn en_lprx_d0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "lp-rx enable for lane0."]
        #[inline(always)]
        pub const fn set_en_lprx_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ulp-rx enable for lane0."]
        #[must_use]
        #[inline(always)]
        pub const fn en_ulprx_d0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ulp-rx enable for lane0."]
        #[inline(always)]
        pub const fn set_en_ulprx_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "the indicator signal of reference generator is ready."]
        #[must_use]
        #[inline(always)]
        pub const fn vbg_rdy(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "the indicator signal of reference generator is ready."]
        #[inline(always)]
        pub const fn set_vbg_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for CtrlPara0 {
        #[inline(always)]
        fn default() -> CtrlPara0 {
            CtrlPara0(0)
        }
    }
    impl core::fmt::Debug for CtrlPara0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CtrlPara0")
                .field("su_iddq_en", &self.su_iddq_en())
                .field("pwon_dsi", &self.pwon_dsi())
                .field("pwon_pll", &self.pwon_pll())
                .field("pwon_sel", &self.pwon_sel())
                .field("en_lpcd_d0", &self.en_lpcd_d0())
                .field("en_lprx_d0", &self.en_lprx_d0())
                .field("en_ulprx_d0", &self.en_ulprx_d0())
                .field("vbg_rdy", &self.vbg_rdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CtrlPara0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CtrlPara0 {{ su_iddq_en: {=bool:?}, pwon_dsi: {=bool:?}, pwon_pll: {=bool:?}, pwon_sel: {=bool:?}, en_lpcd_d0: {=bool:?}, en_lprx_d0: {=bool:?}, en_ulprx_d0: {=bool:?}, vbg_rdy: {=bool:?} }}" , self . su_iddq_en () , self . pwon_dsi () , self . pwon_pll () , self . pwon_sel () , self . en_lpcd_d0 () , self . en_lprx_d0 () , self . en_ulprx_d0 () , self . vbg_rdy ())
        }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane0Para0(pub u32);
    impl Dlane0Para0 {
        #[doc = "the number of byteclk cycles that datalane0 wait to enable lptx_en after reset release."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rst2enlptx_d0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the number of byteclk cycles that datalane0 wait to enable lptx_en after reset release."]
        #[inline(always)]
        pub const fn set_t_rst2enlptx_d0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dlane0Para0 {
        #[inline(always)]
        fn default() -> Dlane0Para0 {
            Dlane0Para0(0)
        }
    }
    impl core::fmt::Debug for Dlane0Para0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane0Para0")
                .field("t_rst2enlptx_d0", &self.t_rst2enlptx_d0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane0Para0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlane0Para0 {{ t_rst2enlptx_d0: {=u16:?} }}",
                self.t_rst2enlptx_d0()
            )
        }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane0Para1(pub u32);
    impl Dlane0Para1 {
        #[doc = "the number of byteclk cycles that datalane0 drive lp-11 during initiaalization period."]
        #[must_use]
        #[inline(always)]
        pub const fn t_inittime_d0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles that datalane0 drive lp-11 during initiaalization period."]
        #[inline(always)]
        pub const fn set_t_inittime_d0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane0Para1 {
        #[inline(always)]
        fn default() -> Dlane0Para1 {
            Dlane0Para1(0)
        }
    }
    impl core::fmt::Debug for Dlane0Para1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane0Para1")
                .field("t_inittime_d0", &self.t_inittime_d0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane0Para1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlane0Para1 {{ t_inittime_d0: {=u32:?} }}",
                self.t_inittime_d0()
            )
        }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane0Para2(pub u32);
    impl Dlane0Para2 {
        #[doc = "the number of byteclk cycles that the datalane0 stay at state hs-exit sate after a hs clock transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hsexit_d0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at state hs-exit sate after a hs clock transmission."]
        #[inline(always)]
        pub const fn set_t_hsexit_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at hs-trail state during a hs clock transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hstrail_d0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at hs-trail state during a hs clock transmission."]
        #[inline(always)]
        pub const fn set_t_hstrail_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at hs-zero sate during a hs transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hszero_d0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at hs-zero sate during a hs transmission."]
        #[inline(always)]
        pub const fn set_t_hszero_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at hs prepare state lp-00 during a hs transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hsprepare_d0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at hs prepare state lp-00 during a hs transmission."]
        #[inline(always)]
        pub const fn set_t_hsprepare_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dlane0Para2 {
        #[inline(always)]
        fn default() -> Dlane0Para2 {
            Dlane0Para2(0)
        }
    }
    impl core::fmt::Debug for Dlane0Para2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane0Para2")
                .field("t_hsexit_d0", &self.t_hsexit_d0())
                .field("t_hstrail_d0", &self.t_hstrail_d0())
                .field("t_hszero_d0", &self.t_hszero_d0())
                .field("t_hsprepare_d0", &self.t_hsprepare_d0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane0Para2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dlane0Para2 {{ t_hsexit_d0: {=u8:?}, t_hstrail_d0: {=u8:?}, t_hszero_d0: {=u8:?}, t_hsprepare_d0: {=u8:?} }}" , self . t_hsexit_d0 () , self . t_hstrail_d0 () , self . t_hszero_d0 () , self . t_hsprepare_d0 ())
        }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane0Para3(pub u32);
    impl Dlane0Para3 {
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[must_use]
        #[inline(always)]
        pub const fn t_wakeup_d0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[inline(always)]
        pub const fn set_t_wakeup_d0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane0Para3 {
        #[inline(always)]
        fn default() -> Dlane0Para3 {
            Dlane0Para3(0)
        }
    }
    impl core::fmt::Debug for Dlane0Para3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane0Para3")
                .field("t_wakeup_d0", &self.t_wakeup_d0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane0Para3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlane0Para3 {{ t_wakeup_d0: {=u32:?} }}",
                self.t_wakeup_d0()
            )
        }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane0Para4(pub u32);
    impl Dlane0Para4 {
        #[doc = "the number of byteclk cycles that the new transmitter drivers the bridge state after accepting control during bta."]
        #[must_use]
        #[inline(always)]
        pub const fn t_taget_d0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the new transmitter drivers the bridge state after accepting control during bta."]
        #[inline(always)]
        pub const fn set_t_taget_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that the rx waits after a bridge state has been detected during a turnaround procedure."]
        #[must_use]
        #[inline(always)]
        pub const fn t_tasure_d0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the rx waits after a bridge state has been detected during a turnaround procedure."]
        #[inline(always)]
        pub const fn set_t_tasure_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that the tx drives the bridge state during a turnaroud procedure."]
        #[must_use]
        #[inline(always)]
        pub const fn t_tago_d0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the tx drives the bridge state during a turnaroud procedure."]
        #[inline(always)]
        pub const fn set_t_tago_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Dlane0Para4 {
        #[inline(always)]
        fn default() -> Dlane0Para4 {
            Dlane0Para4(0)
        }
    }
    impl core::fmt::Debug for Dlane0Para4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane0Para4")
                .field("t_taget_d0", &self.t_taget_d0())
                .field("t_tasure_d0", &self.t_tasure_d0())
                .field("t_tago_d0", &self.t_tago_d0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane0Para4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlane0Para4 {{ t_taget_d0: {=u8:?}, t_tasure_d0: {=u8:?}, t_tago_d0: {=u8:?} }}",
                self.t_taget_d0(),
                self.t_tasure_d0(),
                self.t_tago_d0()
            )
        }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane1Para0(pub u32);
    impl Dlane1Para0 {
        #[doc = "the number of byteclk cycles that datalane1 wait to enable lptx_en after reset release."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rst2enlptx_d1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the number of byteclk cycles that datalane1 wait to enable lptx_en after reset release."]
        #[inline(always)]
        pub const fn set_t_rst2enlptx_d1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dlane1Para0 {
        #[inline(always)]
        fn default() -> Dlane1Para0 {
            Dlane1Para0(0)
        }
    }
    impl core::fmt::Debug for Dlane1Para0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane1Para0")
                .field("t_rst2enlptx_d1", &self.t_rst2enlptx_d1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane1Para0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlane1Para0 {{ t_rst2enlptx_d1: {=u16:?} }}",
                self.t_rst2enlptx_d1()
            )
        }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane1Para1(pub u32);
    impl Dlane1Para1 {
        #[doc = "the number of byteclk cycles that datalane1 drive lp-11 during initiaalization period."]
        #[must_use]
        #[inline(always)]
        pub const fn t_inittime_d1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles that datalane1 drive lp-11 during initiaalization period."]
        #[inline(always)]
        pub const fn set_t_inittime_d1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane1Para1 {
        #[inline(always)]
        fn default() -> Dlane1Para1 {
            Dlane1Para1(0)
        }
    }
    impl core::fmt::Debug for Dlane1Para1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane1Para1")
                .field("t_inittime_d1", &self.t_inittime_d1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane1Para1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlane1Para1 {{ t_inittime_d1: {=u32:?} }}",
                self.t_inittime_d1()
            )
        }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane1Para2(pub u32);
    impl Dlane1Para2 {
        #[doc = "the number of byteclk cycles that the datalane1 stay at state hs-exit sate after a hs clock transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hsexit_d1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at state hs-exit sate after a hs clock transmission."]
        #[inline(always)]
        pub const fn set_t_hsexit_d1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at hs-trail state during a hs clock transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hstrail_d1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at hs-trail state during a hs clock transmission."]
        #[inline(always)]
        pub const fn set_t_hstrail_d1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at hs-zero sate during a hs transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hszero_d1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at hs-zero sate during a hs transmission."]
        #[inline(always)]
        pub const fn set_t_hszero_d1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at hs prepare state lp-00 during a hs transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hsprepare_d1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at hs prepare state lp-00 during a hs transmission."]
        #[inline(always)]
        pub const fn set_t_hsprepare_d1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dlane1Para2 {
        #[inline(always)]
        fn default() -> Dlane1Para2 {
            Dlane1Para2(0)
        }
    }
    impl core::fmt::Debug for Dlane1Para2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane1Para2")
                .field("t_hsexit_d1", &self.t_hsexit_d1())
                .field("t_hstrail_d1", &self.t_hstrail_d1())
                .field("t_hszero_d1", &self.t_hszero_d1())
                .field("t_hsprepare_d1", &self.t_hsprepare_d1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane1Para2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dlane1Para2 {{ t_hsexit_d1: {=u8:?}, t_hstrail_d1: {=u8:?}, t_hszero_d1: {=u8:?}, t_hsprepare_d1: {=u8:?} }}" , self . t_hsexit_d1 () , self . t_hstrail_d1 () , self . t_hszero_d1 () , self . t_hsprepare_d1 ())
        }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane1Para3(pub u32);
    impl Dlane1Para3 {
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[must_use]
        #[inline(always)]
        pub const fn t_wakeup_d1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[inline(always)]
        pub const fn set_t_wakeup_d1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane1Para3 {
        #[inline(always)]
        fn default() -> Dlane1Para3 {
            Dlane1Para3(0)
        }
    }
    impl core::fmt::Debug for Dlane1Para3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane1Para3")
                .field("t_wakeup_d1", &self.t_wakeup_d1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane1Para3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlane1Para3 {{ t_wakeup_d1: {=u32:?} }}",
                self.t_wakeup_d1()
            )
        }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane2Para0(pub u32);
    impl Dlane2Para0 {
        #[doc = "the number of byteclk cycles that datalane2 wait to enable lptx_en after reset release."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rst2enlptx_d2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the number of byteclk cycles that datalane2 wait to enable lptx_en after reset release."]
        #[inline(always)]
        pub const fn set_t_rst2enlptx_d2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dlane2Para0 {
        #[inline(always)]
        fn default() -> Dlane2Para0 {
            Dlane2Para0(0)
        }
    }
    impl core::fmt::Debug for Dlane2Para0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane2Para0")
                .field("t_rst2enlptx_d2", &self.t_rst2enlptx_d2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane2Para0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlane2Para0 {{ t_rst2enlptx_d2: {=u16:?} }}",
                self.t_rst2enlptx_d2()
            )
        }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane2Para1(pub u32);
    impl Dlane2Para1 {
        #[doc = "the number of byteclk cycles that datalane2 drive lp-11 during initiaalization period."]
        #[must_use]
        #[inline(always)]
        pub const fn t_inittime_d2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles that datalane2 drive lp-11 during initiaalization period."]
        #[inline(always)]
        pub const fn set_t_inittime_d2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane2Para1 {
        #[inline(always)]
        fn default() -> Dlane2Para1 {
            Dlane2Para1(0)
        }
    }
    impl core::fmt::Debug for Dlane2Para1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane2Para1")
                .field("t_inittime_d2", &self.t_inittime_d2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane2Para1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlane2Para1 {{ t_inittime_d2: {=u32:?} }}",
                self.t_inittime_d2()
            )
        }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane2Para2(pub u32);
    impl Dlane2Para2 {
        #[doc = "the number of byteclk cycles that the datalane2 stay at state hs-exit sate after a hs clock transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hsexit_d2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at state hs-exit sate after a hs clock transmission."]
        #[inline(always)]
        pub const fn set_t_hsexit_d2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at hs-trail state during a hs clock transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hstrail_d2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at hs-trail state during a hs clock transmission."]
        #[inline(always)]
        pub const fn set_t_hstrail_d2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at hs-zero sate during a hs transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hszero_d2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at hs-zero sate during a hs transmission."]
        #[inline(always)]
        pub const fn set_t_hszero_d2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at hs prepare state lp-00 during a hs transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hsprepare_d2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at hs prepare state lp-00 during a hs transmission."]
        #[inline(always)]
        pub const fn set_t_hsprepare_d2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dlane2Para2 {
        #[inline(always)]
        fn default() -> Dlane2Para2 {
            Dlane2Para2(0)
        }
    }
    impl core::fmt::Debug for Dlane2Para2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane2Para2")
                .field("t_hsexit_d2", &self.t_hsexit_d2())
                .field("t_hstrail_d2", &self.t_hstrail_d2())
                .field("t_hszero_d2", &self.t_hszero_d2())
                .field("t_hsprepare_d2", &self.t_hsprepare_d2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane2Para2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dlane2Para2 {{ t_hsexit_d2: {=u8:?}, t_hstrail_d2: {=u8:?}, t_hszero_d2: {=u8:?}, t_hsprepare_d2: {=u8:?} }}" , self . t_hsexit_d2 () , self . t_hstrail_d2 () , self . t_hszero_d2 () , self . t_hsprepare_d2 ())
        }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane2Para3(pub u32);
    impl Dlane2Para3 {
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[must_use]
        #[inline(always)]
        pub const fn t_wakeup_d2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[inline(always)]
        pub const fn set_t_wakeup_d2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane2Para3 {
        #[inline(always)]
        fn default() -> Dlane2Para3 {
            Dlane2Para3(0)
        }
    }
    impl core::fmt::Debug for Dlane2Para3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane2Para3")
                .field("t_wakeup_d2", &self.t_wakeup_d2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane2Para3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlane2Para3 {{ t_wakeup_d2: {=u32:?} }}",
                self.t_wakeup_d2()
            )
        }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane3Para0(pub u32);
    impl Dlane3Para0 {
        #[doc = "the number of byteclk cycles that datalane3 wait to enable lptx_en after reset release."]
        #[must_use]
        #[inline(always)]
        pub const fn t_rst2enlptx_d3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the number of byteclk cycles that datalane3 wait to enable lptx_en after reset release."]
        #[inline(always)]
        pub const fn set_t_rst2enlptx_d3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dlane3Para0 {
        #[inline(always)]
        fn default() -> Dlane3Para0 {
            Dlane3Para0(0)
        }
    }
    impl core::fmt::Debug for Dlane3Para0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane3Para0")
                .field("t_rst2enlptx_d3", &self.t_rst2enlptx_d3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane3Para0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlane3Para0 {{ t_rst2enlptx_d3: {=u16:?} }}",
                self.t_rst2enlptx_d3()
            )
        }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane3Para1(pub u32);
    impl Dlane3Para1 {
        #[doc = "the number of byteclk cycles that datalane3 drive lp-11 during initiaalization period."]
        #[must_use]
        #[inline(always)]
        pub const fn t_inittime_d3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles that datalane3 drive lp-11 during initiaalization period."]
        #[inline(always)]
        pub const fn set_t_inittime_d3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane3Para1 {
        #[inline(always)]
        fn default() -> Dlane3Para1 {
            Dlane3Para1(0)
        }
    }
    impl core::fmt::Debug for Dlane3Para1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane3Para1")
                .field("t_inittime_d3", &self.t_inittime_d3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane3Para1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlane3Para1 {{ t_inittime_d3: {=u32:?} }}",
                self.t_inittime_d3()
            )
        }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane3Para2(pub u32);
    impl Dlane3Para2 {
        #[doc = "the number of byteclk cycles that the datalane3 stay at state hs-exit sate after a hs clock transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hsexit_d3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at state hs-exit sate after a hs clock transmission."]
        #[inline(always)]
        pub const fn set_t_hsexit_d3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at hs-trail state during a hs clock transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hstrail_d3(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at hs-trail state during a hs clock transmission."]
        #[inline(always)]
        pub const fn set_t_hstrail_d3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at hs-zero sate during a hs transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hszero_d3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at hs-zero sate during a hs transmission."]
        #[inline(always)]
        pub const fn set_t_hszero_d3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at hs prepare state lp-00 during a hs transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn t_hsprepare_d3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at hs prepare state lp-00 during a hs transmission."]
        #[inline(always)]
        pub const fn set_t_hsprepare_d3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dlane3Para2 {
        #[inline(always)]
        fn default() -> Dlane3Para2 {
            Dlane3Para2(0)
        }
    }
    impl core::fmt::Debug for Dlane3Para2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane3Para2")
                .field("t_hsexit_d3", &self.t_hsexit_d3())
                .field("t_hstrail_d3", &self.t_hstrail_d3())
                .field("t_hszero_d3", &self.t_hszero_d3())
                .field("t_hsprepare_d3", &self.t_hsprepare_d3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane3Para2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dlane3Para2 {{ t_hsexit_d3: {=u8:?}, t_hstrail_d3: {=u8:?}, t_hszero_d3: {=u8:?}, t_hsprepare_d3: {=u8:?} }}" , self . t_hsexit_d3 () , self . t_hstrail_d3 () , self . t_hszero_d3 () , self . t_hsprepare_d3 ())
        }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane3Para3(pub u32);
    impl Dlane3Para3 {
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[must_use]
        #[inline(always)]
        pub const fn t_wakeup_d3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[inline(always)]
        pub const fn set_t_wakeup_d3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane3Para3 {
        #[inline(always)]
        fn default() -> Dlane3Para3 {
            Dlane3Para3(0)
        }
    }
    impl core::fmt::Debug for Dlane3Para3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlane3Para3")
                .field("t_wakeup_d3", &self.t_wakeup_d3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlane3Para3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dlane3Para3 {{ t_wakeup_d3: {=u32:?} }}",
                self.t_wakeup_d3()
            )
        }
    }
    #[doc = "dphy clock lane control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InterfacePara(pub u32);
    impl InterfacePara {
        #[doc = "the extend length of rxvalidesc."]
        #[must_use]
        #[inline(always)]
        pub const fn rxvalidesc_extend_vld(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the extend length of rxvalidesc."]
        #[inline(always)]
        pub const fn set_rxvalidesc_extend_vld(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the extend length of txreadyesc."]
        #[must_use]
        #[inline(always)]
        pub const fn txreadyesc_extend_vld(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the extend length of txreadyesc."]
        #[inline(always)]
        pub const fn set_txreadyesc_extend_vld(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for InterfacePara {
        #[inline(always)]
        fn default() -> InterfacePara {
            InterfacePara(0)
        }
    }
    impl core::fmt::Debug for InterfacePara {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("InterfacePara")
                .field("rxvalidesc_extend_vld", &self.rxvalidesc_extend_vld())
                .field("txreadyesc_extend_vld", &self.txreadyesc_extend_vld())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for InterfacePara {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "InterfacePara {{ rxvalidesc_extend_vld: {=u8:?}, txreadyesc_extend_vld: {=u8:?} }}" , self . rxvalidesc_extend_vld () , self . txreadyesc_extend_vld ())
        }
    }
    #[doc = "dphy control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MiscPara(pub u32);
    impl MiscPara {
        #[doc = "mask the phy error."]
        #[must_use]
        #[inline(always)]
        pub const fn phyerr_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "mask the phy error."]
        #[inline(always)]
        pub const fn set_phyerr_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "the number of active data lanes."]
        #[must_use]
        #[inline(always)]
        pub const fn lane_num(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "the number of active data lanes."]
        #[inline(always)]
        pub const fn set_lane_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "the phase select of clk_rxesc."]
        #[must_use]
        #[inline(always)]
        pub const fn dll_sel(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[doc = "the phase select of clk_rxesc."]
        #[inline(always)]
        pub const fn set_dll_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
        }
    }
    impl Default for MiscPara {
        #[inline(always)]
        fn default() -> MiscPara {
            MiscPara(0)
        }
    }
    impl core::fmt::Debug for MiscPara {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MiscPara")
                .field("phyerr_mask", &self.phyerr_mask())
                .field("lane_num", &self.lane_num())
                .field("dll_sel", &self.dll_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MiscPara {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MiscPara {{ phyerr_mask: {=u8:?}, lane_num: {=u8:?}, dll_sel: {=u8:?} }}",
                self.phyerr_mask(),
                self.lane_num(),
                self.dll_sel()
            )
        }
    }
    #[doc = "reserved the pins for pcs."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PcsReservedPinPara(pub u32);
    impl PcsReservedPinPara {
        #[doc = "pma clock dsi_rclk_i inverter signal."]
        #[must_use]
        #[inline(always)]
        pub const fn inv_dsi_rclk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "pma clock dsi_rclk_i inverter signal."]
        #[inline(always)]
        pub const fn set_inv_dsi_rclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "pclk inverter signal."]
        #[must_use]
        #[inline(always)]
        pub const fn inv_pclk(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "pclk inverter signal."]
        #[inline(always)]
        pub const fn set_inv_pclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "clk_txesc inverter signal."]
        #[must_use]
        #[inline(always)]
        pub const fn inv_clk_txesc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clk_txesc inverter signal."]
        #[inline(always)]
        pub const fn set_inv_clk_txesc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "clk_txhs inverter signal."]
        #[must_use]
        #[inline(always)]
        pub const fn inv_clk_txhs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "clk_txhs inverter signal."]
        #[inline(always)]
        pub const fn set_inv_clk_txhs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "select the clock source of clk_txhs in pcs."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_txhs_sel_inner(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "select the clock source of clk_txhs in pcs."]
        #[inline(always)]
        pub const fn set_clk_txhs_sel_inner(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for PcsReservedPinPara {
        #[inline(always)]
        fn default() -> PcsReservedPinPara {
            PcsReservedPinPara(0)
        }
    }
    impl core::fmt::Debug for PcsReservedPinPara {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PcsReservedPinPara")
                .field("inv_dsi_rclk", &self.inv_dsi_rclk())
                .field("inv_pclk", &self.inv_pclk())
                .field("inv_clk_txesc", &self.inv_clk_txesc())
                .field("inv_clk_txhs", &self.inv_clk_txhs())
                .field("clk_txhs_sel_inner", &self.clk_txhs_sel_inner())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PcsReservedPinPara {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PcsReservedPinPara {{ inv_dsi_rclk: {=bool:?}, inv_pclk: {=bool:?}, inv_clk_txesc: {=bool:?}, inv_clk_txhs: {=bool:?}, clk_txhs_sel_inner: {=bool:?} }}" , self . inv_dsi_rclk () , self . inv_pclk () , self . inv_clk_txesc () , self . inv_clk_txhs () , self . clk_txhs_sel_inner ())
        }
    }
    #[doc = "dphy pll control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PllCtrlPara0(pub u32);
    impl PllCtrlPara0 {
        #[doc = "pixell clock divided from pll output."]
        #[must_use]
        #[inline(always)]
        pub const fn dsi_pixelclk_div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "pixell clock divided from pll output."]
        #[inline(always)]
        pub const fn set_dsi_pixelclk_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "pll loop divider ratio control."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_div(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x7fff;
            val as u16
        }
        #[doc = "pll loop divider ratio control."]
        #[inline(always)]
        pub const fn set_pll_div(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 4usize)) | (((val as u32) & 0x7fff) << 4usize);
        }
        #[doc = "input reference clock divider ratio control."]
        #[must_use]
        #[inline(always)]
        pub const fn refclk_div(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x1f;
            val as u8
        }
        #[doc = "input reference clock divider ratio control."]
        #[inline(always)]
        pub const fn set_refclk_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
        }
        #[doc = "data reate control signal."]
        #[must_use]
        #[inline(always)]
        pub const fn rate(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "data reate control signal."]
        #[inline(always)]
        pub const fn set_rate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "pll lock indication."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_lock(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "pll lock indication."]
        #[inline(always)]
        pub const fn set_pll_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for PllCtrlPara0 {
        #[inline(always)]
        fn default() -> PllCtrlPara0 {
            PllCtrlPara0(0)
        }
    }
    impl core::fmt::Debug for PllCtrlPara0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PllCtrlPara0")
                .field("dsi_pixelclk_div", &self.dsi_pixelclk_div())
                .field("pll_div", &self.pll_div())
                .field("refclk_div", &self.refclk_div())
                .field("rate", &self.rate())
                .field("pll_lock", &self.pll_lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PllCtrlPara0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PllCtrlPara0 {{ dsi_pixelclk_div: {=u8:?}, pll_div: {=u16:?}, refclk_div: {=u8:?}, rate: {=u8:?}, pll_lock: {=bool:?} }}" , self . dsi_pixelclk_div () , self . pll_div () , self . refclk_div () , self . rate () , self . pll_lock ())
        }
    }
    #[doc = "pma about clock lane select parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PmaLaneSelPara(pub u32);
    impl PmaLaneSelPara {
        #[doc = "select the channel 1 as the data lane."]
        #[must_use]
        #[inline(always)]
        pub const fn pma_dlane1_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "select the channel 1 as the data lane."]
        #[inline(always)]
        pub const fn set_pma_dlane1_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "select the channel 2 as the data lane."]
        #[must_use]
        #[inline(always)]
        pub const fn pma_dlane2_sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "select the channel 2 as the data lane."]
        #[inline(always)]
        pub const fn set_pma_dlane2_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "select the channel 3 as the data lane."]
        #[must_use]
        #[inline(always)]
        pub const fn pma_dlane3_sel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "select the channel 3 as the data lane."]
        #[inline(always)]
        pub const fn set_pma_dlane3_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "select the channel 4 as the data lane."]
        #[must_use]
        #[inline(always)]
        pub const fn pma_dlane4_sel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "select the channel 4 as the data lane."]
        #[inline(always)]
        pub const fn set_pma_dlane4_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for PmaLaneSelPara {
        #[inline(always)]
        fn default() -> PmaLaneSelPara {
            PmaLaneSelPara(0)
        }
    }
    impl core::fmt::Debug for PmaLaneSelPara {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PmaLaneSelPara")
                .field("pma_dlane1_sel", &self.pma_dlane1_sel())
                .field("pma_dlane2_sel", &self.pma_dlane2_sel())
                .field("pma_dlane3_sel", &self.pma_dlane3_sel())
                .field("pma_dlane4_sel", &self.pma_dlane4_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PmaLaneSelPara {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PmaLaneSelPara {{ pma_dlane1_sel: {=bool:?}, pma_dlane2_sel: {=bool:?}, pma_dlane3_sel: {=bool:?}, pma_dlane4_sel: {=bool:?} }}" , self . pma_dlane1_sel () , self . pma_dlane2_sel () , self . pma_dlane3_sel () , self . pma_dlane4_sel ())
        }
    }
    #[doc = "dphy calibration control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RcalCtrl(pub u32);
    impl RcalCtrl {
        #[doc = "hs-tx output impedance trimming done indicator signal."]
        #[must_use]
        #[inline(always)]
        pub const fn rcal_done(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "hs-tx output impedance trimming done indicator signal."]
        #[inline(always)]
        pub const fn set_rcal_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "resistor calibration control, reserved for test."]
        #[must_use]
        #[inline(always)]
        pub const fn rcal_ctrl(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0xff;
            val as u8
        }
        #[doc = "resistor calibration control, reserved for test."]
        #[inline(always)]
        pub const fn set_rcal_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
        }
        #[doc = "default value of hs-tx output resistance configure."]
        #[must_use]
        #[inline(always)]
        pub const fn rcal_trim(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "default value of hs-tx output resistance configure."]
        #[inline(always)]
        pub const fn set_rcal_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "enable hs-tx output impedance trimming."]
        #[must_use]
        #[inline(always)]
        pub const fn rcal_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "enable hs-tx output impedance trimming."]
        #[inline(always)]
        pub const fn set_rcal_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for RcalCtrl {
        #[inline(always)]
        fn default() -> RcalCtrl {
            RcalCtrl(0)
        }
    }
    impl core::fmt::Debug for RcalCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RcalCtrl")
                .field("rcal_done", &self.rcal_done())
                .field("rcal_ctrl", &self.rcal_ctrl())
                .field("rcal_trim", &self.rcal_trim())
                .field("rcal_en", &self.rcal_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RcalCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "RcalCtrl {{ rcal_done: {=bool:?}, rcal_ctrl: {=u8:?}, rcal_trim: {=u8:?}, rcal_en: {=bool:?} }}" , self . rcal_done () , self . rcal_ctrl () , self . rcal_trim () , self . rcal_en ())
        }
    }
    #[doc = "dphy test control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TestPara0(pub u32);
    impl TestPara0 {
        #[doc = "pt/ft test mode select."]
        #[must_use]
        #[inline(always)]
        pub const fn ft_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "pt/ft test mode select."]
        #[inline(always)]
        pub const fn set_ft_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "enable fast transmission between lp-tx and hs-tx."]
        #[must_use]
        #[inline(always)]
        pub const fn fset_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable fast transmission between lp-tx and hs-tx."]
        #[inline(always)]
        pub const fn set_fset_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "analog test signal select."]
        #[must_use]
        #[inline(always)]
        pub const fn atest_sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "analog test signal select."]
        #[inline(always)]
        pub const fn set_atest_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "analog test signal enable."]
        #[must_use]
        #[inline(always)]
        pub const fn atest_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "analog test signal enable."]
        #[inline(always)]
        pub const fn set_atest_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "indicate prbs7 bist test is ok."]
        #[must_use]
        #[inline(always)]
        pub const fn bist_n_ok(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x1f;
            val as u8
        }
        #[doc = "indicate prbs7 bist test is ok."]
        #[inline(always)]
        pub const fn set_bist_n_ok(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
        }
        #[doc = "indicate prbs7 bist test is done."]
        #[must_use]
        #[inline(always)]
        pub const fn bist_n_done(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x1f;
            val as u8
        }
        #[doc = "indicate prbs7 bist test is done."]
        #[inline(always)]
        pub const fn set_bist_n_done(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
        }
        #[doc = "the byte num of mismatch data of lane in bist mode."]
        #[must_use]
        #[inline(always)]
        pub const fn error_num(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x3f;
            val as u8
        }
        #[doc = "the byte num of mismatch data of lane in bist mode."]
        #[inline(always)]
        pub const fn set_error_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
        }
    }
    impl Default for TestPara0 {
        #[inline(always)]
        fn default() -> TestPara0 {
            TestPara0(0)
        }
    }
    impl core::fmt::Debug for TestPara0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TestPara0")
                .field("ft_sel", &self.ft_sel())
                .field("fset_en", &self.fset_en())
                .field("atest_sel", &self.atest_sel())
                .field("atest_en", &self.atest_en())
                .field("bist_n_ok", &self.bist_n_ok())
                .field("bist_n_done", &self.bist_n_done())
                .field("error_num", &self.error_num())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TestPara0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TestPara0 {{ ft_sel: {=u8:?}, fset_en: {=bool:?}, atest_sel: {=u8:?}, atest_en: {=bool:?}, bist_n_ok: {=u8:?}, bist_n_done: {=u8:?}, error_num: {=u8:?} }}" , self . ft_sel () , self . fset_en () , self . atest_sel () , self . atest_en () , self . bist_n_ok () , self . bist_n_done () , self . error_num ())
        }
    }
    #[doc = "dphy bist test control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TestPara1(pub u32);
    impl TestPara1 {
        #[doc = "prbs generator and checker pattern select signal."]
        #[must_use]
        #[inline(always)]
        pub const fn prbs_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "prbs generator and checker pattern select signal."]
        #[inline(always)]
        pub const fn set_prbs_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "bist mode select."]
        #[must_use]
        #[inline(always)]
        pub const fn bist_sel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "bist mode select."]
        #[inline(always)]
        pub const fn set_bist_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "bist enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bist_en(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "bist enable."]
        #[inline(always)]
        pub const fn set_bist_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "enable insert error in bist test pattern."]
        #[must_use]
        #[inline(always)]
        pub const fn bist_bit_error(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable insert error in bist test pattern."]
        #[inline(always)]
        pub const fn set_bist_bit_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "the threshold of prbs bit error."]
        #[must_use]
        #[inline(always)]
        pub const fn err_threshold(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[doc = "the threshold of prbs bit error."]
        #[inline(always)]
        pub const fn set_err_threshold(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[doc = "the byte num of prbs bist check num."]
        #[must_use]
        #[inline(always)]
        pub const fn check_num(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "the byte num of prbs bist check num."]
        #[inline(always)]
        pub const fn set_check_num(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
        }
    }
    impl Default for TestPara1 {
        #[inline(always)]
        fn default() -> TestPara1 {
            TestPara1(0)
        }
    }
    impl core::fmt::Debug for TestPara1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TestPara1")
                .field("prbs_sel", &self.prbs_sel())
                .field("bist_sel", &self.bist_sel())
                .field("bist_en", &self.bist_en())
                .field("bist_bit_error", &self.bist_bit_error())
                .field("err_threshold", &self.err_threshold())
                .field("check_num", &self.check_num())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TestPara1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TestPara1 {{ prbs_sel: {=u8:?}, bist_sel: {=bool:?}, bist_en: {=u8:?}, bist_bit_error: {=bool:?}, err_threshold: {=u8:?}, check_num: {=u32:?} }}" , self . prbs_sel () , self . bist_sel () , self . bist_en () , self . bist_bit_error () , self . err_threshold () , self . check_num ())
        }
    }
    #[doc = "dphy trimming parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrimPara(pub u32);
    impl TrimPara {
        #[doc = "lp-cd input threshold voltage trimming for lane0."]
        #[must_use]
        #[inline(always)]
        pub const fn lpcd_vref_trim(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "lp-cd input threshold voltage trimming for lane0."]
        #[inline(always)]
        pub const fn set_lpcd_vref_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "lp-rx input threshold voltage trimming for lane0."]
        #[must_use]
        #[inline(always)]
        pub const fn lprx_vref_trim(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "lp-rx input threshold voltage trimming for lane0."]
        #[inline(always)]
        pub const fn set_lprx_vref_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "lp-tx output slew-rate trimming for lane0~4."]
        #[must_use]
        #[inline(always)]
        pub const fn lptx_sr_trim(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "lp-tx output slew-rate trimming for lane0~4."]
        #[inline(always)]
        pub const fn set_lptx_sr_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "hs-tx output vod trimming for lane-0~4."]
        #[must_use]
        #[inline(always)]
        pub const fn hstx_amp_trim(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[doc = "hs-tx output vod trimming for lane-0~4."]
        #[inline(always)]
        pub const fn set_hstx_amp_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
    }
    impl Default for TrimPara {
        #[inline(always)]
        fn default() -> TrimPara {
            TrimPara(0)
        }
    }
    impl core::fmt::Debug for TrimPara {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrimPara")
                .field("lpcd_vref_trim", &self.lpcd_vref_trim())
                .field("lprx_vref_trim", &self.lprx_vref_trim())
                .field("lptx_sr_trim", &self.lptx_sr_trim())
                .field("hstx_amp_trim", &self.hstx_amp_trim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrimPara {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TrimPara {{ lpcd_vref_trim: {=u8:?}, lprx_vref_trim: {=u8:?}, lptx_sr_trim: {=u8:?}, hstx_amp_trim: {=u8:?} }}" , self . lpcd_vref_trim () , self . lprx_vref_trim () , self . lptx_sr_trim () , self . hstx_amp_trim ())
        }
    }
}
