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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[inline(always)]
    pub const fn clane_para1(self) -> crate::common::Reg<regs::ClanePara1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[inline(always)]
    pub const fn clane_para2(self) -> crate::common::Reg<regs::ClanePara2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[inline(always)]
    pub const fn clane_para3(self) -> crate::common::Reg<regs::ClanePara3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[inline(always)]
    pub const fn dlane0_para0(self) -> crate::common::Reg<regs::Dlane0Para0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[inline(always)]
    pub const fn dlane0_para1(self) -> crate::common::Reg<regs::Dlane0Para1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[inline(always)]
    pub const fn dlane0_para2(self) -> crate::common::Reg<regs::Dlane0Para2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[inline(always)]
    pub const fn dlane0_para3(self) -> crate::common::Reg<regs::Dlane0Para3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[inline(always)]
    pub const fn dlane0_para4(self) -> crate::common::Reg<regs::Dlane0Para4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[inline(always)]
    pub const fn dlane1_para0(self) -> crate::common::Reg<regs::Dlane1Para0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[inline(always)]
    pub const fn dlane1_para1(self) -> crate::common::Reg<regs::Dlane1Para1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[inline(always)]
    pub const fn dlane1_para2(self) -> crate::common::Reg<regs::Dlane1Para2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[inline(always)]
    pub const fn dlane1_para3(self) -> crate::common::Reg<regs::Dlane1Para3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[inline(always)]
    pub const fn dlane2_para0(self) -> crate::common::Reg<regs::Dlane2Para0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[inline(always)]
    pub const fn dlane2_para1(self) -> crate::common::Reg<regs::Dlane2Para1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[inline(always)]
    pub const fn dlane2_para2(self) -> crate::common::Reg<regs::Dlane2Para2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[inline(always)]
    pub const fn dlane2_para3(self) -> crate::common::Reg<regs::Dlane2Para3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[inline(always)]
    pub const fn dlane3_para0(self) -> crate::common::Reg<regs::Dlane3Para0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[inline(always)]
    pub const fn dlane3_para1(self) -> crate::common::Reg<regs::Dlane3Para1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[inline(always)]
    pub const fn dlane3_para2(self) -> crate::common::Reg<regs::Dlane3Para2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[inline(always)]
    pub const fn dlane3_para3(self) -> crate::common::Reg<regs::Dlane3Para3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "timing parameter for all lanes."]
    #[inline(always)]
    pub const fn common_para0(self) -> crate::common::Reg<regs::CommonPara0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "dphy control parameter."]
    #[inline(always)]
    pub const fn ctrl_para0(self) -> crate::common::Reg<regs::CtrlPara0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "dphy pll control parameter."]
    #[inline(always)]
    pub const fn pll_ctrl_para0(self) -> crate::common::Reg<regs::PllCtrlPara0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "dphy calibration control parameter."]
    #[inline(always)]
    pub const fn rcal_ctrl(self) -> crate::common::Reg<regs::RcalCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "dphy trimming parameter."]
    #[inline(always)]
    pub const fn trim_para(self) -> crate::common::Reg<regs::TrimPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "dphy test control parameter."]
    #[inline(always)]
    pub const fn test_para0(self) -> crate::common::Reg<regs::TestPara0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "dphy bist test control parameter."]
    #[inline(always)]
    pub const fn test_para1(self) -> crate::common::Reg<regs::TestPara1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "dphy control parameter."]
    #[inline(always)]
    pub const fn misc_para(self) -> crate::common::Reg<regs::MiscPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "dphy clock lane control parameter."]
    #[inline(always)]
    pub const fn clane_para4(self) -> crate::common::Reg<regs::ClanePara4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "dphy clock lane control parameter."]
    #[inline(always)]
    pub const fn interface_para(
        self,
    ) -> crate::common::Reg<regs::InterfacePara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "reserved the pins for pcs."]
    #[inline(always)]
    pub const fn pcs_reserved_pin_para(
        self,
    ) -> crate::common::Reg<regs::PcsReservedPinPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "parallel data about clock lane parameter."]
    #[inline(always)]
    pub const fn clane_data_para(
        self,
    ) -> crate::common::Reg<regs::ClaneDataPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "pma about clock lane select parameter."]
    #[inline(always)]
    pub const fn pma_lane_sel_para(
        self,
    ) -> crate::common::Reg<regs::PmaLaneSelPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
}
pub mod regs {
    #[doc = "parallel data about clock lane parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClaneDataPara(pub u32);
    impl ClaneDataPara {
        #[doc = "the parallel data about clock lane."]
        #[inline(always)]
        pub const fn clane_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the parallel data about clock lane."]
        #[inline(always)]
        pub fn set_clane_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "select the data about clock lane."]
        #[inline(always)]
        pub const fn clane_data_sel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "select the data about clock lane."]
        #[inline(always)]
        pub fn set_clane_data_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for ClaneDataPara {
        #[inline(always)]
        fn default() -> ClaneDataPara {
            ClaneDataPara(0)
        }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClanePara0(pub u32);
    impl ClanePara0 {
        #[doc = "the soft reset of clk_cfg domain."]
        #[inline(always)]
        pub const fn t_rst2enlptx_c(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the soft reset of clk_cfg domain."]
        #[inline(always)]
        pub fn set_t_rst2enlptx_c(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ClanePara0 {
        #[inline(always)]
        fn default() -> ClanePara0 {
            ClanePara0(0)
        }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClanePara1(pub u32);
    impl ClanePara1 {
        #[doc = "the number of byteclk cycles that clklane drive LP-11 during initialization period."]
        #[inline(always)]
        pub const fn t_inittime_c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles that clklane drive LP-11 during initialization period."]
        #[inline(always)]
        pub fn set_t_inittime_c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ClanePara1 {
        #[inline(always)]
        fn default() -> ClanePara1 {
            ClanePara1(0)
        }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClanePara2(pub u32);
    impl ClanePara2 {
        #[doc = "the number of byteclk cycles that hs clock shall be driven prior to data lane beginning the transition from lp to hs mode."]
        #[inline(always)]
        pub const fn t_clkpre_c(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that hs clock shall be driven prior to data lane beginning the transition from lp to hs mode."]
        #[inline(always)]
        pub fn set_t_clkpre_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that clock lane clkp/n lines are at the hs-zero state hs-0 during a hs clock transmission."]
        #[inline(always)]
        pub const fn t_clkzero_c(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that clock lane clkp/n lines are at the hs-zero state hs-0 during a hs clock transmission."]
        #[inline(always)]
        pub fn set_t_clkzero_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that clock lane clkp/n lines are at the hs prepare state lp-00 during a hs clock transmission."]
        #[inline(always)]
        pub const fn t_clkprepare_c(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that clock lane clkp/n lines are at the hs prepare state lp-00 during a hs clock transmission."]
        #[inline(always)]
        pub fn set_t_clkprepare_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ClanePara2 {
        #[inline(always)]
        fn default() -> ClanePara2 {
            ClanePara2(0)
        }
    }
    #[doc = "timer counter about clock lane parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClanePara3(pub u32);
    impl ClanePara3 {
        #[doc = "the number of byteclk cycles that the clock lane clkp/n lines are at hs-exit state after a hs clock transmission."]
        #[inline(always)]
        pub const fn t_hsexit_c(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the clock lane clkp/n lines are at hs-exit state after a hs clock transmission."]
        #[inline(always)]
        pub fn set_t_hsexit_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that the clock lane clkp/n lines are at state hs-tail sate hs-0 during a hs clock transmission."]
        #[inline(always)]
        pub const fn t_clktrial_c(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the clock lane clkp/n lines are at state hs-tail sate hs-0 during a hs clock transmission."]
        #[inline(always)]
        pub fn set_t_clktrial_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that the clock lane should keep sending the hs-clock after the last associated data lane has transitioned to LP mode."]
        #[inline(always)]
        pub const fn t_clkpost_c(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the clock lane should keep sending the hs-clock after the last associated data lane has transitioned to LP mode."]
        #[inline(always)]
        pub fn set_t_clkpost_c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ClanePara3 {
        #[inline(always)]
        fn default() -> ClanePara3 {
            ClanePara3(0)
        }
    }
    #[doc = "dphy clock lane control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClanePara4(pub u32);
    impl ClanePara4 {
        #[doc = "the number of byteclk cycles from exiting ultra low power state to enabling the low-power driver."]
        #[inline(always)]
        pub const fn t_wakeup_c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles from exiting ultra low power state to enabling the low-power driver."]
        #[inline(always)]
        pub fn set_t_wakeup_c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ClanePara4 {
        #[inline(always)]
        fn default() -> ClanePara4 {
            ClanePara4(0)
        }
    }
    #[doc = "timing parameter for all lanes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CommonPara0(pub u32);
    impl CommonPara0 {
        #[doc = "the number of byteclk cycles of transmitted length of any low-power state period."]
        #[inline(always)]
        pub const fn t_lpx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles of transmitted length of any low-power state period."]
        #[inline(always)]
        pub fn set_t_lpx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CommonPara0 {
        #[inline(always)]
        fn default() -> CommonPara0 {
            CommonPara0(0)
        }
    }
    #[doc = "dphy control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlPara0(pub u32);
    impl CtrlPara0 {
        #[doc = "power down all modules inside su includes ivref, r-calibration and pll, high effective."]
        #[inline(always)]
        pub const fn su_iddq_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "power down all modules inside su includes ivref, r-calibration and pll, high effective."]
        #[inline(always)]
        pub fn set_su_iddq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "power on all dsi lane."]
        #[inline(always)]
        pub const fn pwon_dsi(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "power on all dsi lane."]
        #[inline(always)]
        pub fn set_pwon_dsi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "power on pll high active."]
        #[inline(always)]
        pub const fn pwon_pll(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "power on pll high active."]
        #[inline(always)]
        pub fn set_pwon_pll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "select the cource of PMA power on control signals."]
        #[inline(always)]
        pub const fn pwon_sel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "select the cource of PMA power on control signals."]
        #[inline(always)]
        pub fn set_pwon_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "lp-cd enable for lane0."]
        #[inline(always)]
        pub const fn en_lpcd_d0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "lp-cd enable for lane0."]
        #[inline(always)]
        pub fn set_en_lpcd_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "lp-rx enable for lane0."]
        #[inline(always)]
        pub const fn en_lprx_d0(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "lp-rx enable for lane0."]
        #[inline(always)]
        pub fn set_en_lprx_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ulp-rx enable for lane0."]
        #[inline(always)]
        pub const fn en_ulprx_d0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ulp-rx enable for lane0."]
        #[inline(always)]
        pub fn set_en_ulprx_d0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "the indicator signal of reference generator is ready."]
        #[inline(always)]
        pub const fn vbg_rdy(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "the indicator signal of reference generator is ready."]
        #[inline(always)]
        pub fn set_vbg_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for CtrlPara0 {
        #[inline(always)]
        fn default() -> CtrlPara0 {
            CtrlPara0(0)
        }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane0Para0(pub u32);
    impl Dlane0Para0 {
        #[doc = "the number of byteclk cycles that datalane0 wait to enable lptx_en after reset release."]
        #[inline(always)]
        pub const fn t_rst2enlptx_d0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the number of byteclk cycles that datalane0 wait to enable lptx_en after reset release."]
        #[inline(always)]
        pub fn set_t_rst2enlptx_d0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dlane0Para0 {
        #[inline(always)]
        fn default() -> Dlane0Para0 {
            Dlane0Para0(0)
        }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane0Para1(pub u32);
    impl Dlane0Para1 {
        #[doc = "the number of byteclk cycles that datalane0 drive lp-11 during initiaalization period."]
        #[inline(always)]
        pub const fn t_inittime_d0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles that datalane0 drive lp-11 during initiaalization period."]
        #[inline(always)]
        pub fn set_t_inittime_d0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane0Para1 {
        #[inline(always)]
        fn default() -> Dlane0Para1 {
            Dlane0Para1(0)
        }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane0Para2(pub u32);
    impl Dlane0Para2 {
        #[doc = "the number of byteclk cycles that the datalane0 stay at state hs-exit sate after a hs clock transmission."]
        #[inline(always)]
        pub const fn t_hsexit_d0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at state hs-exit sate after a hs clock transmission."]
        #[inline(always)]
        pub fn set_t_hsexit_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at hs-trail state during a hs clock transmission."]
        #[inline(always)]
        pub const fn t_hstrail_d0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at hs-trail state during a hs clock transmission."]
        #[inline(always)]
        pub fn set_t_hstrail_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at hs-zero sate during a hs transmission."]
        #[inline(always)]
        pub const fn t_hszero_d0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at hs-zero sate during a hs transmission."]
        #[inline(always)]
        pub fn set_t_hszero_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at hs prepare state lp-00 during a hs transmission."]
        #[inline(always)]
        pub const fn t_hsprepare_d0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane0 stay at hs prepare state lp-00 during a hs transmission."]
        #[inline(always)]
        pub fn set_t_hsprepare_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dlane0Para2 {
        #[inline(always)]
        fn default() -> Dlane0Para2 {
            Dlane0Para2(0)
        }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane0Para3(pub u32);
    impl Dlane0Para3 {
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[inline(always)]
        pub const fn t_wakeup_d0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[inline(always)]
        pub fn set_t_wakeup_d0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane0Para3 {
        #[inline(always)]
        fn default() -> Dlane0Para3 {
            Dlane0Para3(0)
        }
    }
    #[doc = "timer counter about datalane0 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane0Para4(pub u32);
    impl Dlane0Para4 {
        #[doc = "the number of byteclk cycles that the new transmitter drivers the bridge state after accepting control during bta."]
        #[inline(always)]
        pub const fn t_taget_d0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the new transmitter drivers the bridge state after accepting control during bta."]
        #[inline(always)]
        pub fn set_t_taget_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that the rx waits after a bridge state has been detected during a turnaround procedure."]
        #[inline(always)]
        pub const fn t_tasure_d0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the rx waits after a bridge state has been detected during a turnaround procedure."]
        #[inline(always)]
        pub fn set_t_tasure_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that the tx drives the bridge state during a turnaroud procedure."]
        #[inline(always)]
        pub const fn t_tago_d0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the tx drives the bridge state during a turnaroud procedure."]
        #[inline(always)]
        pub fn set_t_tago_d0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Dlane0Para4 {
        #[inline(always)]
        fn default() -> Dlane0Para4 {
            Dlane0Para4(0)
        }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane1Para0(pub u32);
    impl Dlane1Para0 {
        #[doc = "the number of byteclk cycles that datalane1 wait to enable lptx_en after reset release."]
        #[inline(always)]
        pub const fn t_rst2enlptx_d1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the number of byteclk cycles that datalane1 wait to enable lptx_en after reset release."]
        #[inline(always)]
        pub fn set_t_rst2enlptx_d1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dlane1Para0 {
        #[inline(always)]
        fn default() -> Dlane1Para0 {
            Dlane1Para0(0)
        }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane1Para1(pub u32);
    impl Dlane1Para1 {
        #[doc = "the number of byteclk cycles that datalane1 drive lp-11 during initiaalization period."]
        #[inline(always)]
        pub const fn t_inittime_d1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles that datalane1 drive lp-11 during initiaalization period."]
        #[inline(always)]
        pub fn set_t_inittime_d1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane1Para1 {
        #[inline(always)]
        fn default() -> Dlane1Para1 {
            Dlane1Para1(0)
        }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane1Para2(pub u32);
    impl Dlane1Para2 {
        #[doc = "the number of byteclk cycles that the datalane1 stay at state hs-exit sate after a hs clock transmission."]
        #[inline(always)]
        pub const fn t_hsexit_d1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at state hs-exit sate after a hs clock transmission."]
        #[inline(always)]
        pub fn set_t_hsexit_d1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at hs-trail state during a hs clock transmission."]
        #[inline(always)]
        pub const fn t_hstrail_d1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at hs-trail state during a hs clock transmission."]
        #[inline(always)]
        pub fn set_t_hstrail_d1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at hs-zero sate during a hs transmission."]
        #[inline(always)]
        pub const fn t_hszero_d1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at hs-zero sate during a hs transmission."]
        #[inline(always)]
        pub fn set_t_hszero_d1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at hs prepare state lp-00 during a hs transmission."]
        #[inline(always)]
        pub const fn t_hsprepare_d1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane1 stay at hs prepare state lp-00 during a hs transmission."]
        #[inline(always)]
        pub fn set_t_hsprepare_d1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dlane1Para2 {
        #[inline(always)]
        fn default() -> Dlane1Para2 {
            Dlane1Para2(0)
        }
    }
    #[doc = "timer counter about datalane1 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane1Para3(pub u32);
    impl Dlane1Para3 {
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[inline(always)]
        pub const fn t_wakeup_d1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[inline(always)]
        pub fn set_t_wakeup_d1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane1Para3 {
        #[inline(always)]
        fn default() -> Dlane1Para3 {
            Dlane1Para3(0)
        }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane2Para0(pub u32);
    impl Dlane2Para0 {
        #[doc = "the number of byteclk cycles that datalane2 wait to enable lptx_en after reset release."]
        #[inline(always)]
        pub const fn t_rst2enlptx_d2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the number of byteclk cycles that datalane2 wait to enable lptx_en after reset release."]
        #[inline(always)]
        pub fn set_t_rst2enlptx_d2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dlane2Para0 {
        #[inline(always)]
        fn default() -> Dlane2Para0 {
            Dlane2Para0(0)
        }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane2Para1(pub u32);
    impl Dlane2Para1 {
        #[doc = "the number of byteclk cycles that datalane2 drive lp-11 during initiaalization period."]
        #[inline(always)]
        pub const fn t_inittime_d2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles that datalane2 drive lp-11 during initiaalization period."]
        #[inline(always)]
        pub fn set_t_inittime_d2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane2Para1 {
        #[inline(always)]
        fn default() -> Dlane2Para1 {
            Dlane2Para1(0)
        }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane2Para2(pub u32);
    impl Dlane2Para2 {
        #[doc = "the number of byteclk cycles that the datalane2 stay at state hs-exit sate after a hs clock transmission."]
        #[inline(always)]
        pub const fn t_hsexit_d2(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at state hs-exit sate after a hs clock transmission."]
        #[inline(always)]
        pub fn set_t_hsexit_d2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at hs-trail state during a hs clock transmission."]
        #[inline(always)]
        pub const fn t_hstrail_d2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at hs-trail state during a hs clock transmission."]
        #[inline(always)]
        pub fn set_t_hstrail_d2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at hs-zero sate during a hs transmission."]
        #[inline(always)]
        pub const fn t_hszero_d2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at hs-zero sate during a hs transmission."]
        #[inline(always)]
        pub fn set_t_hszero_d2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at hs prepare state lp-00 during a hs transmission."]
        #[inline(always)]
        pub const fn t_hsprepare_d2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane2 stay at hs prepare state lp-00 during a hs transmission."]
        #[inline(always)]
        pub fn set_t_hsprepare_d2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dlane2Para2 {
        #[inline(always)]
        fn default() -> Dlane2Para2 {
            Dlane2Para2(0)
        }
    }
    #[doc = "timer counter about datalane2 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane2Para3(pub u32);
    impl Dlane2Para3 {
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[inline(always)]
        pub const fn t_wakeup_d2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[inline(always)]
        pub fn set_t_wakeup_d2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane2Para3 {
        #[inline(always)]
        fn default() -> Dlane2Para3 {
            Dlane2Para3(0)
        }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane3Para0(pub u32);
    impl Dlane3Para0 {
        #[doc = "the number of byteclk cycles that datalane3 wait to enable lptx_en after reset release."]
        #[inline(always)]
        pub const fn t_rst2enlptx_d3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the number of byteclk cycles that datalane3 wait to enable lptx_en after reset release."]
        #[inline(always)]
        pub fn set_t_rst2enlptx_d3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dlane3Para0 {
        #[inline(always)]
        fn default() -> Dlane3Para0 {
            Dlane3Para0(0)
        }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane3Para1(pub u32);
    impl Dlane3Para1 {
        #[doc = "the number of byteclk cycles that datalane3 drive lp-11 during initiaalization period."]
        #[inline(always)]
        pub const fn t_inittime_d3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles that datalane3 drive lp-11 during initiaalization period."]
        #[inline(always)]
        pub fn set_t_inittime_d3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane3Para1 {
        #[inline(always)]
        fn default() -> Dlane3Para1 {
            Dlane3Para1(0)
        }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane3Para2(pub u32);
    impl Dlane3Para2 {
        #[doc = "the number of byteclk cycles that the datalane3 stay at state hs-exit sate after a hs clock transmission."]
        #[inline(always)]
        pub const fn t_hsexit_d3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at state hs-exit sate after a hs clock transmission."]
        #[inline(always)]
        pub fn set_t_hsexit_d3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at hs-trail state during a hs clock transmission."]
        #[inline(always)]
        pub const fn t_hstrail_d3(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at hs-trail state during a hs clock transmission."]
        #[inline(always)]
        pub fn set_t_hstrail_d3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at hs-zero sate during a hs transmission."]
        #[inline(always)]
        pub const fn t_hszero_d3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at hs-zero sate during a hs transmission."]
        #[inline(always)]
        pub fn set_t_hszero_d3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at hs prepare state lp-00 during a hs transmission."]
        #[inline(always)]
        pub const fn t_hsprepare_d3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "the number of byteclk cycles that the datalane3 stay at hs prepare state lp-00 during a hs transmission."]
        #[inline(always)]
        pub fn set_t_hsprepare_d3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dlane3Para2 {
        #[inline(always)]
        fn default() -> Dlane3Para2 {
            Dlane3Para2(0)
        }
    }
    #[doc = "timer counter about datalane3 parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlane3Para3(pub u32);
    impl Dlane3Para3 {
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[inline(always)]
        pub const fn t_wakeup_d3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "the number of byteclk cycles from exiting ultra low power sate to enabling the low-power driver."]
        #[inline(always)]
        pub fn set_t_wakeup_d3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dlane3Para3 {
        #[inline(always)]
        fn default() -> Dlane3Para3 {
            Dlane3Para3(0)
        }
    }
    #[doc = "dphy clock lane control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InterfacePara(pub u32);
    impl InterfacePara {
        #[doc = "the extend length of rxvalidesc."]
        #[inline(always)]
        pub const fn rxvalidesc_extend_vld(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the extend length of rxvalidesc."]
        #[inline(always)]
        pub fn set_rxvalidesc_extend_vld(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the extend length of txreadyesc."]
        #[inline(always)]
        pub const fn txreadyesc_extend_vld(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the extend length of txreadyesc."]
        #[inline(always)]
        pub fn set_txreadyesc_extend_vld(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for InterfacePara {
        #[inline(always)]
        fn default() -> InterfacePara {
            InterfacePara(0)
        }
    }
    #[doc = "dphy control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MiscPara(pub u32);
    impl MiscPara {
        #[doc = "mask the phy error."]
        #[inline(always)]
        pub const fn phyerr_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "mask the phy error."]
        #[inline(always)]
        pub fn set_phyerr_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "the number of active data lanes."]
        #[inline(always)]
        pub const fn lane_num(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "the number of active data lanes."]
        #[inline(always)]
        pub fn set_lane_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "the phase select of clk_rxesc."]
        #[inline(always)]
        pub const fn dll_sel(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[doc = "the phase select of clk_rxesc."]
        #[inline(always)]
        pub fn set_dll_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
        }
    }
    impl Default for MiscPara {
        #[inline(always)]
        fn default() -> MiscPara {
            MiscPara(0)
        }
    }
    #[doc = "reserved the pins for pcs."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PcsReservedPinPara(pub u32);
    impl PcsReservedPinPara {
        #[doc = "pma clock dsi_rclk_i inverter signal."]
        #[inline(always)]
        pub const fn inv_dsi_rclk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "pma clock dsi_rclk_i inverter signal."]
        #[inline(always)]
        pub fn set_inv_dsi_rclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "pclk inverter signal."]
        #[inline(always)]
        pub const fn inv_pclk(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "pclk inverter signal."]
        #[inline(always)]
        pub fn set_inv_pclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "clk_txesc inverter signal."]
        #[inline(always)]
        pub const fn inv_clk_txesc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clk_txesc inverter signal."]
        #[inline(always)]
        pub fn set_inv_clk_txesc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "clk_txhs inverter signal."]
        #[inline(always)]
        pub const fn inv_clk_txhs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "clk_txhs inverter signal."]
        #[inline(always)]
        pub fn set_inv_clk_txhs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "select the clock source of clk_txhs in pcs."]
        #[inline(always)]
        pub const fn clk_txhs_sel_inner(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "select the clock source of clk_txhs in pcs."]
        #[inline(always)]
        pub fn set_clk_txhs_sel_inner(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for PcsReservedPinPara {
        #[inline(always)]
        fn default() -> PcsReservedPinPara {
            PcsReservedPinPara(0)
        }
    }
    #[doc = "dphy pll control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PllCtrlPara0(pub u32);
    impl PllCtrlPara0 {
        #[doc = "pixell clock divided from pll output."]
        #[inline(always)]
        pub const fn dsi_pixelclk_div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "pixell clock divided from pll output."]
        #[inline(always)]
        pub fn set_dsi_pixelclk_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "pll loop divider ratio control."]
        #[inline(always)]
        pub const fn pll_div(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x7fff;
            val as u16
        }
        #[doc = "pll loop divider ratio control."]
        #[inline(always)]
        pub fn set_pll_div(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 4usize)) | (((val as u32) & 0x7fff) << 4usize);
        }
        #[doc = "input reference clock divider ratio control."]
        #[inline(always)]
        pub const fn refclk_div(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x1f;
            val as u8
        }
        #[doc = "input reference clock divider ratio control."]
        #[inline(always)]
        pub fn set_refclk_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
        }
        #[doc = "data reate control signal."]
        #[inline(always)]
        pub const fn rate(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "data reate control signal."]
        #[inline(always)]
        pub fn set_rate(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "pll lock indication."]
        #[inline(always)]
        pub const fn pll_lock(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "pll lock indication."]
        #[inline(always)]
        pub fn set_pll_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for PllCtrlPara0 {
        #[inline(always)]
        fn default() -> PllCtrlPara0 {
            PllCtrlPara0(0)
        }
    }
    #[doc = "pma about clock lane select parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PmaLaneSelPara(pub u32);
    impl PmaLaneSelPara {
        #[doc = "select the channel 1 as the data lane."]
        #[inline(always)]
        pub const fn pma_dlane1_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "select the channel 1 as the data lane."]
        #[inline(always)]
        pub fn set_pma_dlane1_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "select the channel 2 as the data lane."]
        #[inline(always)]
        pub const fn pma_dlane2_sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "select the channel 2 as the data lane."]
        #[inline(always)]
        pub fn set_pma_dlane2_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "select the channel 3 as the data lane."]
        #[inline(always)]
        pub const fn pma_dlane3_sel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "select the channel 3 as the data lane."]
        #[inline(always)]
        pub fn set_pma_dlane3_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "select the channel 4 as the data lane."]
        #[inline(always)]
        pub const fn pma_dlane4_sel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "select the channel 4 as the data lane."]
        #[inline(always)]
        pub fn set_pma_dlane4_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for PmaLaneSelPara {
        #[inline(always)]
        fn default() -> PmaLaneSelPara {
            PmaLaneSelPara(0)
        }
    }
    #[doc = "dphy calibration control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RcalCtrl(pub u32);
    impl RcalCtrl {
        #[doc = "hs-tx output impedance trimming done indicator signal."]
        #[inline(always)]
        pub const fn rcal_done(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "hs-tx output impedance trimming done indicator signal."]
        #[inline(always)]
        pub fn set_rcal_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "resistor calibration control, reserved for test."]
        #[inline(always)]
        pub const fn rcal_ctrl(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0xff;
            val as u8
        }
        #[doc = "resistor calibration control, reserved for test."]
        #[inline(always)]
        pub fn set_rcal_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
        }
        #[doc = "default value of hs-tx output resistance configure."]
        #[inline(always)]
        pub const fn rcal_trim(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "default value of hs-tx output resistance configure."]
        #[inline(always)]
        pub fn set_rcal_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "enable hs-tx output impedance trimming."]
        #[inline(always)]
        pub const fn rcal_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "enable hs-tx output impedance trimming."]
        #[inline(always)]
        pub fn set_rcal_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for RcalCtrl {
        #[inline(always)]
        fn default() -> RcalCtrl {
            RcalCtrl(0)
        }
    }
    #[doc = "dphy test control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TestPara0(pub u32);
    impl TestPara0 {
        #[doc = "pt/ft test mode select."]
        #[inline(always)]
        pub const fn ft_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "pt/ft test mode select."]
        #[inline(always)]
        pub fn set_ft_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "enable fast transmission between lp-tx and hs-tx."]
        #[inline(always)]
        pub const fn fset_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable fast transmission between lp-tx and hs-tx."]
        #[inline(always)]
        pub fn set_fset_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "analog test signal select."]
        #[inline(always)]
        pub const fn atest_sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "analog test signal select."]
        #[inline(always)]
        pub fn set_atest_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "analog test signal enable."]
        #[inline(always)]
        pub const fn atest_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "analog test signal enable."]
        #[inline(always)]
        pub fn set_atest_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "indicate prbs7 bist test is ok."]
        #[inline(always)]
        pub const fn bist_n_ok(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x1f;
            val as u8
        }
        #[doc = "indicate prbs7 bist test is ok."]
        #[inline(always)]
        pub fn set_bist_n_ok(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
        }
        #[doc = "indicate prbs7 bist test is done."]
        #[inline(always)]
        pub const fn bist_n_done(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x1f;
            val as u8
        }
        #[doc = "indicate prbs7 bist test is done."]
        #[inline(always)]
        pub fn set_bist_n_done(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
        }
        #[doc = "the byte num of mismatch data of lane in bist mode."]
        #[inline(always)]
        pub const fn error_num(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x3f;
            val as u8
        }
        #[doc = "the byte num of mismatch data of lane in bist mode."]
        #[inline(always)]
        pub fn set_error_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
        }
    }
    impl Default for TestPara0 {
        #[inline(always)]
        fn default() -> TestPara0 {
            TestPara0(0)
        }
    }
    #[doc = "dphy bist test control parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TestPara1(pub u32);
    impl TestPara1 {
        #[doc = "prbs generator and checker pattern select signal."]
        #[inline(always)]
        pub const fn prbs_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "prbs generator and checker pattern select signal."]
        #[inline(always)]
        pub fn set_prbs_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "bist mode select."]
        #[inline(always)]
        pub const fn bist_sel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "bist mode select."]
        #[inline(always)]
        pub fn set_bist_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "bist enable."]
        #[inline(always)]
        pub const fn bist_en(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "bist enable."]
        #[inline(always)]
        pub fn set_bist_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "enable insert error in bist test pattern."]
        #[inline(always)]
        pub const fn bist_bit_error(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable insert error in bist test pattern."]
        #[inline(always)]
        pub fn set_bist_bit_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "the threshold of prbs bit error."]
        #[inline(always)]
        pub const fn err_threshold(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[doc = "the threshold of prbs bit error."]
        #[inline(always)]
        pub fn set_err_threshold(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[doc = "the byte num of prbs bist check num."]
        #[inline(always)]
        pub const fn check_num(&self) -> u32 {
            let val = (self.0 >> 10usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "the byte num of prbs bist check num."]
        #[inline(always)]
        pub fn set_check_num(&mut self, val: u32) {
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
    #[doc = "dphy trimming parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrimPara(pub u32);
    impl TrimPara {
        #[doc = "lp-cd input threshold voltage trimming for lane0."]
        #[inline(always)]
        pub const fn lpcd_vref_trim(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "lp-cd input threshold voltage trimming for lane0."]
        #[inline(always)]
        pub fn set_lpcd_vref_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "lp-rx input threshold voltage trimming for lane0."]
        #[inline(always)]
        pub const fn lprx_vref_trim(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "lp-rx input threshold voltage trimming for lane0."]
        #[inline(always)]
        pub fn set_lprx_vref_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "lp-tx output slew-rate trimming for lane0~4."]
        #[inline(always)]
        pub const fn lptx_sr_trim(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "lp-tx output slew-rate trimming for lane0~4."]
        #[inline(always)]
        pub fn set_lptx_sr_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "hs-tx output vod trimming for lane-0~4."]
        #[inline(always)]
        pub const fn hstx_amp_trim(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[doc = "hs-tx output vod trimming for lane-0~4."]
        #[inline(always)]
        pub fn set_hstx_amp_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
    }
    impl Default for TrimPara {
        #[inline(always)]
        fn default() -> TrimPara {
            TrimPara(0)
        }
    }
}
