#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "MIPI_DSI0."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MipiDsi {
    ptr: *mut u8,
}
unsafe impl Send for MipiDsi {}
unsafe impl Sync for MipiDsi {}
impl MipiDsi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "version."]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "power up."]
    #[inline(always)]
    pub const fn pwr_up(self) -> crate::common::Reg<regs::PwrUp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "divide lanebyteclk for timeout."]
    #[inline(always)]
    pub const fn clkmgr_cfg(self) -> crate::common::Reg<regs::ClkmgrCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "virtual channel ID for DPI traffic."]
    #[inline(always)]
    pub const fn dpi_vcid(self) -> crate::common::Reg<regs::DpiVcid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "dpi color coding."]
    #[inline(always)]
    pub const fn dpi_color_coding(
        self,
    ) -> crate::common::Reg<regs::DpiColorCoding, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "the polarity of DPI signals."]
    #[inline(always)]
    pub const fn dpi_cfg_pol(self) -> crate::common::Reg<regs::DpiCfgPol, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "the timing for low-power commands sent while in video mode."]
    #[inline(always)]
    pub const fn dpi_lp_cmd_tim(self) -> crate::common::Reg<regs::DpiLpCmdTim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "configures how EoTp, BTA, CRC and ECC to be used."]
    #[inline(always)]
    pub const fn pckhdl_cfg(self) -> crate::common::Reg<regs::PckhdlCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "configures the virtual channel ID of read response to store and return to generic interface."]
    #[inline(always)]
    pub const fn gen_vcid(self) -> crate::common::Reg<regs::GenVcid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "configures the mode of operation between video or command mode."]
    #[inline(always)]
    pub const fn mode_cfg(self) -> crate::common::Reg<regs::ModeCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "several aspect of video mode operation."]
    #[inline(always)]
    pub const fn vid_mode_cfg(self) -> crate::common::Reg<regs::VidModeCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "configures the video packet size."]
    #[inline(always)]
    pub const fn vid_pkt_size(self) -> crate::common::Reg<regs::VidPktSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "configures the number of chunks to use."]
    #[inline(always)]
    pub const fn vid_num_chunks(self) -> crate::common::Reg<regs::VidNumChunks, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "configures the size of null packets."]
    #[inline(always)]
    pub const fn vid_null_size(self) -> crate::common::Reg<regs::VidNullSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "configures the video HAS time."]
    #[inline(always)]
    pub const fn vid_hsa_time(self) -> crate::common::Reg<regs::VidHsaTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "configure the video HBP time."]
    #[inline(always)]
    pub const fn vid_hbp_time(self) -> crate::common::Reg<regs::VidHbpTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "configures the overall time for each video line."]
    #[inline(always)]
    pub const fn vid_hline_time(self) -> crate::common::Reg<regs::VidHlineTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "configures the vsa period."]
    #[inline(always)]
    pub const fn vid_vsa_lines(self) -> crate::common::Reg<regs::VidVsaLines, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "configures the vbp period."]
    #[inline(always)]
    pub const fn vid_vbp_lines(self) -> crate::common::Reg<regs::VidVbpLines, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "configures the vfp period."]
    #[inline(always)]
    pub const fn vid_vfp_lines(self) -> crate::common::Reg<regs::VidVfpLines, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "configures the vertical resolution of video."]
    #[inline(always)]
    pub const fn vid_vactive_lines(
        self,
    ) -> crate::common::Reg<regs::VidVactiveLines, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "This register configures several aspect of command mode operation, tearing effect, acknowledge for each packet and the speed mode to transmit each Data Type related to commands."]
    #[inline(always)]
    pub const fn cmd_mode_cfg(self) -> crate::common::Reg<regs::CmdModeCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "sets the header for new packets sent using the generic interface."]
    #[inline(always)]
    pub const fn gen_hdr(self) -> crate::common::Reg<regs::GenHdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "sets the payload for packets sent using the generic interface."]
    #[inline(always)]
    pub const fn gen_pld_data(self) -> crate::common::Reg<regs::GenPldData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "information about the status of FIFOs related to DBI and Generic interface."]
    #[inline(always)]
    pub const fn cmd_pkt_status(self) -> crate::common::Reg<regs::CmdPktStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "configures the trigger timeout errors."]
    #[inline(always)]
    pub const fn to_cnt_cfg(self) -> crate::common::Reg<regs::ToCntCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "configures the peripheral response timeout after high speed read operations."]
    #[inline(always)]
    pub const fn hs_rd_to_cnt(self) -> crate::common::Reg<regs::HsRdToCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "configures the peripheral response timeout after low-power read operation."]
    #[inline(always)]
    pub const fn lp_rd_to_cnt(self) -> crate::common::Reg<regs::LpRdToCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "configures the peripheral response timeout after high speed write operations."]
    #[inline(always)]
    pub const fn hs_wr_to_cnt(self) -> crate::common::Reg<regs::HsWrToCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "configures the peripheral response timeout after low power write operations."]
    #[inline(always)]
    pub const fn lp_wr_to_cnt(self) -> crate::common::Reg<regs::LpWrToCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "configures the periphera response timeout after bus turnaround."]
    #[inline(always)]
    pub const fn bta_to_cnt(self) -> crate::common::Reg<regs::BtaToCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "sotres 3d control information for vss packets in video mode."]
    #[inline(always)]
    pub const fn sdf_3d(self) -> crate::common::Reg<regs::Sdf3d, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "configures the possibility for using non continuous clock in the clock lane."]
    #[inline(always)]
    pub const fn lpclk_ctrl(self) -> crate::common::Reg<regs::LpclkCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "sets the time that dsi host assumes in calculations for the clock lane to switch between high-speed and low-power."]
    #[inline(always)]
    pub const fn phy_tmr_lpclk_cfg(
        self,
    ) -> crate::common::Reg<regs::PhyTmrLpclkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "sets the time that dsi host assumes in calculations for data lanes to switch between hs to lp."]
    #[inline(always)]
    pub const fn phy_tmr_cfg(self) -> crate::common::Reg<regs::PhyTmrCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "controls resets and the pll of d-phy."]
    #[inline(always)]
    pub const fn phy_rstz(self) -> crate::common::Reg<regs::PhyRstz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "configures the number of active lanes."]
    #[inline(always)]
    pub const fn phy_if_cfg(self) -> crate::common::Reg<regs::PhyIfCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "configures entering and leaving ulps."]
    #[inline(always)]
    pub const fn phy_ulps_ctrl(self) -> crate::common::Reg<regs::PhyUlpsCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "configures the pins that activate triggers in the d-phy."]
    #[inline(always)]
    pub const fn phy_tx_triggers(
        self,
    ) -> crate::common::Reg<regs::PhyTxTriggers, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "contains information about the status of the d-phy."]
    #[inline(always)]
    pub const fn phy_status(self) -> crate::common::Reg<regs::PhyStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "controls clock and clear pins of the d-phy vendor specific interface."]
    #[inline(always)]
    pub const fn phy_tst_ctrl0(self) -> crate::common::Reg<regs::PhyTstCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "controls data and enable pins of the d-phy."]
    #[inline(always)]
    pub const fn phy_tst_ctrl1(self) -> crate::common::Reg<regs::PhyTstCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "controls the status of interrupt."]
    #[inline(always)]
    pub const fn int_st0(self) -> crate::common::Reg<regs::IntSt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "the interrupt source related to timeout etc."]
    #[inline(always)]
    pub const fn int_st1(self) -> crate::common::Reg<regs::IntSt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "configures masks for the sources of interrupt that affec int_st0."]
    #[inline(always)]
    pub const fn int_msk0(self) -> crate::common::Reg<regs::IntMsk0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "configures masks for int_st1."]
    #[inline(always)]
    pub const fn int_msk1(self) -> crate::common::Reg<regs::IntMsk1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "controls the skew calibration of D-phy."]
    #[inline(always)]
    pub const fn phy_cal(self) -> crate::common::Reg<regs::PhyCal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "forces that affect the int_st0 register."]
    #[inline(always)]
    pub const fn int_force0(self) -> crate::common::Reg<regs::IntForce0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "forces interrupts that affect the int_st1 register."]
    #[inline(always)]
    pub const fn int_force1(self) -> crate::common::Reg<regs::IntForce1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "configures times related to PHY to perform some operations in lane byte clock cycle."]
    #[inline(always)]
    pub const fn phy_tmr_rd(self) -> crate::common::Reg<regs::PhyTmrRd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "configures the minimum time required by phy between ulpsactivenot and ulpsexitreq for clock and data lane."]
    #[inline(always)]
    pub const fn auto_ulps_min_time(
        self,
    ) -> crate::common::Reg<regs::AutoUlpsMinTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "select phy mode."]
    #[inline(always)]
    pub const fn phy_mode(self) -> crate::common::Reg<regs::PhyMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "controls dpi shadow feature."]
    #[inline(always)]
    pub const fn vid_shadow_ctrl(
        self,
    ) -> crate::common::Reg<regs::VidShadowCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "holds the value that controller is using for DPI_VCID."]
    #[inline(always)]
    pub const fn dpi_vcid_act(self) -> crate::common::Reg<regs::DpiVcidAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "holds the value that controller is using for DPI_COLOR_CODING."]
    #[inline(always)]
    pub const fn dpi_color_coding_act(
        self,
    ) -> crate::common::Reg<regs::DpiColorCodingAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "holds value that controller is using for dpi_lp_cmd_time."]
    #[inline(always)]
    pub const fn dpi_lp_cmd_tim_act(
        self,
    ) -> crate::common::Reg<regs::DpiLpCmdTimAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "holds value that controller is using for vid_mode_cfg."]
    #[inline(always)]
    pub const fn vid_mode_cfg_act(
        self,
    ) -> crate::common::Reg<regs::VidModeCfgAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "holds value that controller is using for vid_pkt_size."]
    #[inline(always)]
    pub const fn vid_pkt_size_act(
        self,
    ) -> crate::common::Reg<regs::VidPktSizeAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "holds value that controller is using for vid_num_chunks."]
    #[inline(always)]
    pub const fn vid_num_chunks_act(
        self,
    ) -> crate::common::Reg<regs::VidNumChunksAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "holds the value that controller is using for vid_null_size."]
    #[inline(always)]
    pub const fn vid_null_size_act(
        self,
    ) -> crate::common::Reg<regs::VidNullSizeAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "the value of vid_hsa_time."]
    #[inline(always)]
    pub const fn vid_hsa_time_act(
        self,
    ) -> crate::common::Reg<regs::VidHsaTimeAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "the value that controller is using for vid_hbp_time."]
    #[inline(always)]
    pub const fn vid_hbp_time_act(
        self,
    ) -> crate::common::Reg<regs::VidHbpTimeAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "the value for vid_hline_time."]
    #[inline(always)]
    pub const fn vid_hline_time_act(
        self,
    ) -> crate::common::Reg<regs::VidHlineTimeAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "value for vid_vsa_lines."]
    #[inline(always)]
    pub const fn vid_vsa_lines_act(
        self,
    ) -> crate::common::Reg<regs::VidVsaLinesAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "value for vid_vbp_lines."]
    #[inline(always)]
    pub const fn vid_vbp_lines_act(
        self,
    ) -> crate::common::Reg<regs::VidVbpLinesAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "value for vid_vfp_lines."]
    #[inline(always)]
    pub const fn vid_vfp_lines_act(
        self,
    ) -> crate::common::Reg<regs::VidVfpLinesAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "value for vid_vactive_lines."]
    #[inline(always)]
    pub const fn vid_vactive_lines_act(
        self,
    ) -> crate::common::Reg<regs::VidVactiveLinesAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "status of fifo related to dpi."]
    #[inline(always)]
    pub const fn vid_pkt_status(self) -> crate::common::Reg<regs::VidPktStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "value for sdf_3d."]
    #[inline(always)]
    pub const fn sdf_3d_act(self) -> crate::common::Reg<regs::Sdf3dAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
}
pub mod regs {
    #[doc = "configures the minimum time required by phy between ulpsactivenot and ulpsexitreq for clock and data lane."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AutoUlpsMinTime(pub u32);
    impl AutoUlpsMinTime {
        #[doc = "configures the minimum time required by phy between ulpsactivenot and ulpsexitreq for clock and data lane."]
        #[inline(always)]
        pub const fn ulps_min_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "configures the minimum time required by phy between ulpsactivenot and ulpsexitreq for clock and data lane."]
        #[inline(always)]
        pub fn set_ulps_min_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for AutoUlpsMinTime {
        #[inline(always)]
        fn default() -> AutoUlpsMinTime {
            AutoUlpsMinTime(0)
        }
    }
    #[doc = "configures the periphera response timeout after bus turnaround."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BtaToCnt(pub u32);
    impl BtaToCnt {
        #[doc = "sets the period for which dsi host keeps the link still after completing a bus turnaround."]
        #[inline(always)]
        pub const fn bta_to_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "sets the period for which dsi host keeps the link still after completing a bus turnaround."]
        #[inline(always)]
        pub fn set_bta_to_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for BtaToCnt {
        #[inline(always)]
        fn default() -> BtaToCnt {
            BtaToCnt(0)
        }
    }
    #[doc = "divide lanebyteclk for timeout."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClkmgrCfg(pub u32);
    impl ClkmgrCfg {
        #[doc = "the division factor for the TX Escape clock source lanebyteclk."]
        #[inline(always)]
        pub const fn tx_esc_clk_division(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "the division factor for the TX Escape clock source lanebyteclk."]
        #[inline(always)]
        pub fn set_tx_esc_clk_division(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "the timeout clock division factor for HS to LP and LP to HS transition error."]
        #[inline(always)]
        pub const fn to_clk_division(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "the timeout clock division factor for HS to LP and LP to HS transition error."]
        #[inline(always)]
        pub fn set_to_clk_division(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for ClkmgrCfg {
        #[inline(always)]
        fn default() -> ClkmgrCfg {
            ClkmgrCfg(0)
        }
    }
    #[doc = "This register configures several aspect of command mode operation, tearing effect, acknowledge for each packet and the speed mode to transmit each Data Type related to commands."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdModeCfg(pub u32);
    impl CmdModeCfg {
        #[doc = "When set to 1, this bit enables the tearing effect acknowledge request."]
        #[inline(always)]
        pub const fn tear_fx_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "When set to 1, this bit enables the tearing effect acknowledge request."]
        #[inline(always)]
        pub fn set_tear_fx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "When set to 1, this bit enables the acknowledge request after each packet transmission."]
        #[inline(always)]
        pub const fn ack_rqst_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set to 1, this bit enables the acknowledge request after each packet transmission."]
        #[inline(always)]
        pub fn set_ack_rqst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub const fn gen_sw_0p_tx(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub fn set_gen_sw_0p_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub const fn gen_sw_1p_tx(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub fn set_gen_sw_1p_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub const fn gen_sw_2p_tx(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub fn set_gen_sw_2p_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub const fn gen_sr_0p_tx(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub fn set_gen_sr_0p_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub const fn gen_sr_1p_tx(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub fn set_gen_sr_1p_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub const fn gen_sr_2p_tx(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "This bit configures the Generic short read packet with two parameters command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub fn set_gen_sr_2p_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "This bit configures the Generic long write packet command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub const fn gen_lw_tx(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "This bit configures the Generic long write packet command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub fn set_gen_lw_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "This bit configures the DCS short write packet with zero parameter command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub const fn dcs_sw_0p_tx(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "This bit configures the DCS short write packet with zero parameter command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub fn set_dcs_sw_0p_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "This bit configures the DCS short write packet with one parameter command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub const fn dcs_sw_1p_tx(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "This bit configures the DCS short write packet with one parameter command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub fn set_dcs_sw_1p_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "This bit configures the DCS short read packet with zero parameter command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub const fn dcs_sr_0p_tx(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "This bit configures the DCS short read packet with zero parameter command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub fn set_dcs_sr_0p_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "This bit configures the DCS long write packet command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub const fn dcs_lw_tx(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "This bit configures the DCS long write packet command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub fn set_dcs_lw_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "This bit configures the maximum read packet size command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub const fn max_rd_pkt_size(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "This bit configures the maximum read packet size command transmission type: 0x0 (HIGHSPEED): Transition type is High Speed 0x1 (LOWPOWER): Transition type is Low Power."]
        #[inline(always)]
        pub fn set_max_rd_pkt_size(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for CmdModeCfg {
        #[inline(always)]
        fn default() -> CmdModeCfg {
            CmdModeCfg(0)
        }
    }
    #[doc = "information about the status of FIFOs related to DBI and Generic interface."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdPktStatus(pub u32);
    impl CmdPktStatus {
        #[doc = "indicates the empty status of the generic command FIFO."]
        #[inline(always)]
        pub const fn gen_cmd_empty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the empty status of the generic command FIFO."]
        #[inline(always)]
        pub fn set_gen_cmd_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "indicates the full status of the generic command FIFO."]
        #[inline(always)]
        pub const fn gen_cmd_full(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the full status of the generic command FIFO."]
        #[inline(always)]
        pub fn set_gen_cmd_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "indicates the empty status of the generic write payload FIFO."]
        #[inline(always)]
        pub const fn gen_pld_w_empty(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the empty status of the generic write payload FIFO."]
        #[inline(always)]
        pub fn set_gen_pld_w_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "indicates the full status of the generic write payload FIFO."]
        #[inline(always)]
        pub const fn gen_pld_w_full(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the full status of the generic write payload FIFO."]
        #[inline(always)]
        pub fn set_gen_pld_w_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "indicates the empty status of the generic read payload FIFO."]
        #[inline(always)]
        pub const fn gen_pld_r_empty(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the empty status of the generic read payload FIFO."]
        #[inline(always)]
        pub fn set_gen_pld_r_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "indicates the full status of the generic read payoad FIFO."]
        #[inline(always)]
        pub const fn gen_pld_r_full(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the full status of the generic read payoad FIFO."]
        #[inline(always)]
        pub fn set_gen_pld_r_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "indicates a read command is issued and the entire response is not sotred in the FIFO."]
        #[inline(always)]
        pub const fn gen_rd_cmd_busy(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "indicates a read command is issued and the entire response is not sotred in the FIFO."]
        #[inline(always)]
        pub fn set_gen_rd_cmd_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "the empty status of the generic command internal buffer."]
        #[inline(always)]
        pub const fn gen_buff_cmd_empty(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "the empty status of the generic command internal buffer."]
        #[inline(always)]
        pub fn set_gen_buff_cmd_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "the full status of the generic command internal buffer."]
        #[inline(always)]
        pub const fn gen_buff_cmd_full(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "the full status of the generic command internal buffer."]
        #[inline(always)]
        pub fn set_gen_buff_cmd_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "the empty status of the generic payload internal buffer."]
        #[inline(always)]
        pub const fn gen_buff_pld_empty(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "the empty status of the generic payload internal buffer."]
        #[inline(always)]
        pub fn set_gen_buff_pld_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "the full status of the generic payload internal buffer."]
        #[inline(always)]
        pub const fn gen_buff_pld_full(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "the full status of the generic payload internal buffer."]
        #[inline(always)]
        pub fn set_gen_buff_pld_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for CmdPktStatus {
        #[inline(always)]
        fn default() -> CmdPktStatus {
            CmdPktStatus(0)
        }
    }
    #[doc = "the polarity of DPI signals."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DpiCfgPol(pub u32);
    impl DpiCfgPol {
        #[doc = "configures the data enable pin active low."]
        #[inline(always)]
        pub const fn dataen_active_low(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "configures the data enable pin active low."]
        #[inline(always)]
        pub fn set_dataen_active_low(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "configures the vertical synchronism pin as active low."]
        #[inline(always)]
        pub const fn vsync_active_low(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "configures the vertical synchronism pin as active low."]
        #[inline(always)]
        pub fn set_vsync_active_low(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "configures the horizontal synchronism pin as active low."]
        #[inline(always)]
        pub const fn hsync_active_low(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "configures the horizontal synchronism pin as active low."]
        #[inline(always)]
        pub fn set_hsync_active_low(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "configures the shutdown pin as active low."]
        #[inline(always)]
        pub const fn shutd_active_low(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "configures the shutdown pin as active low."]
        #[inline(always)]
        pub fn set_shutd_active_low(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "configures the color mode pin as active low."]
        #[inline(always)]
        pub const fn colorm_active_low(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "configures the color mode pin as active low."]
        #[inline(always)]
        pub fn set_colorm_active_low(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for DpiCfgPol {
        #[inline(always)]
        fn default() -> DpiCfgPol {
            DpiCfgPol(0)
        }
    }
    #[doc = "dpi color coding."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DpiColorCoding(pub u32);
    impl DpiColorCoding {
        #[doc = "configures the DPI color for video mode."]
        #[inline(always)]
        pub const fn dpi_color_coding(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "configures the DPI color for video mode."]
        #[inline(always)]
        pub fn set_dpi_color_coding(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "when set to 1, this bit activates loosely packed variant to 18-bit configurations."]
        #[inline(always)]
        pub const fn loosely18_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "when set to 1, this bit activates loosely packed variant to 18-bit configurations."]
        #[inline(always)]
        pub fn set_loosely18_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for DpiColorCoding {
        #[inline(always)]
        fn default() -> DpiColorCoding {
            DpiColorCoding(0)
        }
    }
    #[doc = "holds the value that controller is using for DPI_COLOR_CODING."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DpiColorCodingAct(pub u32);
    impl DpiColorCodingAct {
        #[doc = "configures the DPI color for video mode."]
        #[inline(always)]
        pub const fn dip_color_coding(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "configures the DPI color for video mode."]
        #[inline(always)]
        pub fn set_dip_color_coding(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "avtivates loosely packed variant to 18-bit configuration."]
        #[inline(always)]
        pub const fn loosely18_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "avtivates loosely packed variant to 18-bit configuration."]
        #[inline(always)]
        pub fn set_loosely18_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for DpiColorCodingAct {
        #[inline(always)]
        fn default() -> DpiColorCodingAct {
            DpiColorCodingAct(0)
        }
    }
    #[doc = "the timing for low-power commands sent while in video mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DpiLpCmdTim(pub u32);
    impl DpiLpCmdTim {
        #[doc = "transmission of commands in low-power mode, defines the size in bytes of the largest packet that can fit in a line during the VACT region."]
        #[inline(always)]
        pub const fn invact_lpcmd_time(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "transmission of commands in low-power mode, defines the size in bytes of the largest packet that can fit in a line during the VACT region."]
        #[inline(always)]
        pub fn set_invact_lpcmd_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "transmission of commands in low-power mode, defines the size in bytes of the largest pachet that can fit in a line during the VSA VBP and VFP;."]
        #[inline(always)]
        pub const fn outvact_lpcmd_time(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "transmission of commands in low-power mode, defines the size in bytes of the largest pachet that can fit in a line during the VSA VBP and VFP;."]
        #[inline(always)]
        pub fn set_outvact_lpcmd_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for DpiLpCmdTim {
        #[inline(always)]
        fn default() -> DpiLpCmdTim {
            DpiLpCmdTim(0)
        }
    }
    #[doc = "holds value that controller is using for dpi_lp_cmd_time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DpiLpCmdTimAct(pub u32);
    impl DpiLpCmdTimAct {
        #[doc = "transmission of commands in low-power mode, it specifies the size in bytes of the lagest packet that can fit in a line during the vact regions."]
        #[inline(always)]
        pub const fn invact_lpcmd_time(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "transmission of commands in low-power mode, it specifies the size in bytes of the lagest packet that can fit in a line during the vact regions."]
        #[inline(always)]
        pub fn set_invact_lpcmd_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "transmission of commands in low-power mode, it specifies the size in bytes of the lagest packet that can fit in a line during the VSA VBP and VFP regions."]
        #[inline(always)]
        pub const fn outvact_lpcmd_time(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "transmission of commands in low-power mode, it specifies the size in bytes of the lagest packet that can fit in a line during the VSA VBP and VFP regions."]
        #[inline(always)]
        pub fn set_outvact_lpcmd_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for DpiLpCmdTimAct {
        #[inline(always)]
        fn default() -> DpiLpCmdTimAct {
            DpiLpCmdTimAct(0)
        }
    }
    #[doc = "virtual channel ID for DPI traffic."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DpiVcid(pub u32);
    impl DpiVcid {
        #[doc = "the DPI virtual channel id to the video mode packets."]
        #[inline(always)]
        pub const fn dpi_vcid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "the DPI virtual channel id to the video mode packets."]
        #[inline(always)]
        pub fn set_dpi_vcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for DpiVcid {
        #[inline(always)]
        fn default() -> DpiVcid {
            DpiVcid(0)
        }
    }
    #[doc = "holds the value that controller is using for DPI_VCID."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DpiVcidAct(pub u32);
    impl DpiVcidAct {
        #[doc = "specifies the DPI virtual channel id that is indexed to the video mode packets."]
        #[inline(always)]
        pub const fn dpi_vcid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "specifies the DPI virtual channel id that is indexed to the video mode packets."]
        #[inline(always)]
        pub fn set_dpi_vcid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for DpiVcidAct {
        #[inline(always)]
        fn default() -> DpiVcidAct {
            DpiVcidAct(0)
        }
    }
    #[doc = "sets the header for new packets sent using the generic interface."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GenHdr(pub u32);
    impl GenHdr {
        #[doc = "configures the packet data type of the header packet."]
        #[inline(always)]
        pub const fn gen_dt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "configures the packet data type of the header packet."]
        #[inline(always)]
        pub fn set_gen_dt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "configures the virtual channel ID of the header packet."]
        #[inline(always)]
        pub const fn gen_vc(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "configures the virtual channel ID of the header packet."]
        #[inline(always)]
        pub fn set_gen_vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "configures the least significant byte of the header packet's word count for long packets or data0 for short packets."]
        #[inline(always)]
        pub const fn gen_wc_lsbyte(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "configures the least significant byte of the header packet's word count for long packets or data0 for short packets."]
        #[inline(always)]
        pub fn set_gen_wc_lsbyte(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "configures the most significant byte of the header packet's word count for long packets or data 1 for shout packets."]
        #[inline(always)]
        pub const fn gen_wc_msbyte(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "configures the most significant byte of the header packet's word count for long packets or data 1 for shout packets."]
        #[inline(always)]
        pub fn set_gen_wc_msbyte(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for GenHdr {
        #[inline(always)]
        fn default() -> GenHdr {
            GenHdr(0)
        }
    }
    #[doc = "sets the payload for packets sent using the generic interface."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GenPldData(pub u32);
    impl GenPldData {
        #[doc = "indicates byte1 of the packet payload."]
        #[inline(always)]
        pub const fn gen_pld_b1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "indicates byte1 of the packet payload."]
        #[inline(always)]
        pub fn set_gen_pld_b1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "indicates byte2 of the packet payload."]
        #[inline(always)]
        pub const fn gen_pld_b2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "indicates byte2 of the packet payload."]
        #[inline(always)]
        pub fn set_gen_pld_b2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "indicates byte3 of the packet payload."]
        #[inline(always)]
        pub const fn gen_pld_b3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "indicates byte3 of the packet payload."]
        #[inline(always)]
        pub fn set_gen_pld_b3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "indicates byte4 of the packet payload."]
        #[inline(always)]
        pub const fn gen_pld_b4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "indicates byte4 of the packet payload."]
        #[inline(always)]
        pub fn set_gen_pld_b4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for GenPldData {
        #[inline(always)]
        fn default() -> GenPldData {
            GenPldData(0)
        }
    }
    #[doc = "configures the virtual channel ID of read response to store and return to generic interface."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GenVcid(pub u32);
    impl GenVcid {
        #[doc = "indicates the generic interface read-back virtual channel identication."]
        #[inline(always)]
        pub const fn gen_vcid_rx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "indicates the generic interface read-back virtual channel identication."]
        #[inline(always)]
        pub fn set_gen_vcid_rx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "indicates the virtual channel identification for tear effect by hardware."]
        #[inline(always)]
        pub const fn gen_vcid_tear_auto(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "indicates the virtual channel identification for tear effect by hardware."]
        #[inline(always)]
        pub fn set_gen_vcid_tear_auto(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "indicates the generic interface virtual channel identification where generic packet is automatically generated and transmitted."]
        #[inline(always)]
        pub const fn gen_vcid_tx_auto(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "indicates the generic interface virtual channel identification where generic packet is automatically generated and transmitted."]
        #[inline(always)]
        pub fn set_gen_vcid_tx_auto(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for GenVcid {
        #[inline(always)]
        fn default() -> GenVcid {
            GenVcid(0)
        }
    }
    #[doc = "configures the peripheral response timeout after high speed read operations."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HsRdToCnt(pub u32);
    impl HsRdToCnt {
        #[doc = "sets a period for which DWC_mipi_dsi_host keeps the link still after sending a high speed read operation;."]
        #[inline(always)]
        pub const fn hs_rd_to_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "sets a period for which DWC_mipi_dsi_host keeps the link still after sending a high speed read operation;."]
        #[inline(always)]
        pub fn set_hs_rd_to_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for HsRdToCnt {
        #[inline(always)]
        fn default() -> HsRdToCnt {
            HsRdToCnt(0)
        }
    }
    #[doc = "configures the peripheral response timeout after high speed write operations."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HsWrToCnt(pub u32);
    impl HsWrToCnt {
        #[doc = "sets the period for which dwc_mipi_dsi_host keeps the link still after sending a high speed write operation."]
        #[inline(always)]
        pub const fn hs_wr_to_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "sets the period for which dwc_mipi_dsi_host keeps the link still after sending a high speed write operation."]
        #[inline(always)]
        pub fn set_hs_wr_to_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for HsWrToCnt {
        #[inline(always)]
        fn default() -> HsWrToCnt {
            HsWrToCnt(0)
        }
    }
    #[doc = "forces that affect the int_st0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForce0(pub u32);
    impl IntForce0 {
        #[doc = "force the SoT serror from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "force the SoT serror from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "force the SoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force the SoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "force the EoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "force the EoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "force the Escap mode entry command error from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "force the Escap mode entry command error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "force the LP transmit sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "force the LP transmit sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "force the peripheral timeout error from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "force the peripheral timeout error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "force the false control error fro the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "force the false control error fro the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "force the reserved from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "force the reserved from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "force the ecc error sigle-bit from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "force the ecc error sigle-bit from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "force the ECC error multi-bit from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err_9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "force the ECC error multi-bit from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err_9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "force the checksum error from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err_10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "force the checksum error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err_10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "force the not recongnized dsi data type from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err_11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "force the not recongnized dsi data type from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err_11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "force the dsi vc id invalid from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err_12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "force the dsi vc id invalid from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err_12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "force the invalid transmission length from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err_13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "force the invalid transmission length from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err_13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "force the reserved from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err_14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "force the reserved from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err_14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "force the DSI protocal violation from the acknowledge error report."]
        #[inline(always)]
        pub const fn force_ack_with_err_15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "force the DSI protocal violation from the acknowledge error report."]
        #[inline(always)]
        pub fn set_force_ack_with_err_15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "force ErrEsc escape entry error from lane0."]
        #[inline(always)]
        pub const fn force_dphy_errors_0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "force ErrEsc escape entry error from lane0."]
        #[inline(always)]
        pub fn set_force_dphy_errors_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "force ErrSyncEsc low-power data transmission synchronization error from lane 0."]
        #[inline(always)]
        pub const fn force_dphy_errors_1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "force ErrSyncEsc low-power data transmission synchronization error from lane 0."]
        #[inline(always)]
        pub fn set_force_dphy_errors_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "force control error ErrControl from lane0."]
        #[inline(always)]
        pub const fn force_dphy_errors_2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "force control error ErrControl from lane0."]
        #[inline(always)]
        pub fn set_force_dphy_errors_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "force LP0 contention error ErrContentionLP0 from lane0."]
        #[inline(always)]
        pub const fn force_dphy_errors_3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "force LP0 contention error ErrContentionLP0 from lane0."]
        #[inline(always)]
        pub fn set_force_dphy_errors_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "force LP1 contention error ErrContentionLP1 from lane0."]
        #[inline(always)]
        pub const fn force_dphy_errors_4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "force LP1 contention error ErrContentionLP1 from lane0."]
        #[inline(always)]
        pub fn set_force_dphy_errors_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for IntForce0 {
        #[inline(always)]
        fn default() -> IntForce0 {
            IntForce0(0)
        }
    }
    #[doc = "forces interrupts that affect the int_st1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntForce1(pub u32);
    impl IntForce1 {
        #[doc = "force that the high-speed transmission timeout counter reached the end and contention has been detected."]
        #[inline(always)]
        pub const fn force_to_hs_tx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "force that the high-speed transmission timeout counter reached the end and contention has been detected."]
        #[inline(always)]
        pub fn set_force_to_hs_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "force that the low-power reception timeout counter reached the end and contention has been detected."]
        #[inline(always)]
        pub const fn force_to_lp_tx(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force that the low-power reception timeout counter reached the end and contention has been detected."]
        #[inline(always)]
        pub fn set_force_to_lp_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "force that the ECC single error has been detected and corrected in a reveived packet."]
        #[inline(always)]
        pub const fn force_ecc_sigle_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "force that the ECC single error has been detected and corrected in a reveived packet."]
        #[inline(always)]
        pub fn set_force_ecc_sigle_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "force that the ECC multiple error has been detected in a revieved packet."]
        #[inline(always)]
        pub const fn force_ecc_multi_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "force that the ECC multiple error has been detected in a revieved packet."]
        #[inline(always)]
        pub fn set_force_ecc_multi_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "force that the CRC error has been detected in the reveived packet payload."]
        #[inline(always)]
        pub const fn force_crc_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "force that the CRC error has been detected in the reveived packet payload."]
        #[inline(always)]
        pub fn set_force_crc_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "force that the packet size error has been detected during the packet reception."]
        #[inline(always)]
        pub const fn force_pkt_size_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "force that the packet size error has been detected during the packet reception."]
        #[inline(always)]
        pub fn set_force_pkt_size_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "force that the EoTp packet has not been received at the end of the incoming peripheral transmission."]
        #[inline(always)]
        pub const fn force_eopt_err(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "force that the EoTp packet has not been received at the end of the incoming peripheral transmission."]
        #[inline(always)]
        pub fn set_force_eopt_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "force the payload FIFO is full during a DPI pixel line storage."]
        #[inline(always)]
        pub const fn force_dpi_bpld_wr_err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "force the payload FIFO is full during a DPI pixel line storage."]
        #[inline(always)]
        pub fn set_force_dpi_bpld_wr_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "force the system tried to write a command and FIFO is full."]
        #[inline(always)]
        pub const fn force_gen_cmd_wr_err(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "force the system tried to write a command and FIFO is full."]
        #[inline(always)]
        pub fn set_force_gen_cmd_wr_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "force the system tried to write a payload and FIFO is full."]
        #[inline(always)]
        pub const fn force_gen_pld_wr_err(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "force the system tried to write a payload and FIFO is full."]
        #[inline(always)]
        pub fn set_force_gen_pld_wr_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "force the payload FIFO become empty when packet build."]
        #[inline(always)]
        pub const fn force_gen_pld_send_err(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "force the payload FIFO become empty when packet build."]
        #[inline(always)]
        pub fn set_force_gen_pld_send_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "force that during a DCS read data, the payload FIFO becomes empty."]
        #[inline(always)]
        pub const fn force_gen_pld_rd_err(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "force that during a DCS read data, the payload FIFO becomes empty."]
        #[inline(always)]
        pub fn set_force_gen_pld_rd_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "force that during a generic interface packet read back, the payload FIFO full."]
        #[inline(always)]
        pub const fn force_gen_pld_recev_err(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "force that during a generic interface packet read back, the payload FIFO full."]
        #[inline(always)]
        pub fn set_force_gen_pld_recev_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "force an underflow when reading payload to build dsi packet for video mode."]
        #[inline(always)]
        pub const fn force_dpi_buff_pld_under(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "force an underflow when reading payload to build dsi packet for video mode."]
        #[inline(always)]
        pub fn set_force_dpi_buff_pld_under(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "force tear_request has occurred but tear effect is not active in dsi host and device."]
        #[inline(always)]
        pub const fn force_tear_request_err(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "force tear_request has occurred but tear effect is not active in dsi host and device."]
        #[inline(always)]
        pub fn set_force_tear_request_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for IntForce1 {
        #[inline(always)]
        fn default() -> IntForce1 {
            IntForce1(0)
        }
    }
    #[doc = "configures masks for the sources of interrupt that affec int_st0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMsk0(pub u32);
    impl IntMsk0 {
        #[doc = "disable the SoT serror from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "disable the SoT serror from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "disable the SoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "disable the SoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "disable the EoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "disable the EoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "disable the Escap mode entry command error from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "disable the Escap mode entry command error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "disable the LP transmit sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "disable the LP transmit sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "disable the peripheral timeout error from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "disable the peripheral timeout error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "disable the false control error fro the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "disable the false control error fro the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "disable the reserved from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "disable the reserved from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "disable the ecc error sigle-bit from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "disable the ecc error sigle-bit from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "disable the ECC error multi-bit from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err_9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "disable the ECC error multi-bit from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err_9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "disable the checksum error from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err_10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "disable the checksum error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err_10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "disable the not recongnized dsi data type from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err_11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "disable the not recongnized dsi data type from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err_11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "disable the dsi vc id invalid from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err_12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "disable the dsi vc id invalid from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err_12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "disable the invalid transmission length from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err_13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "disable the invalid transmission length from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err_13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "disable the reserved from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err_14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "disable the reserved from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err_14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "disable the DSI protocal violation from the acknowledge error report."]
        #[inline(always)]
        pub const fn mask_ack_with_err_15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "disable the DSI protocal violation from the acknowledge error report."]
        #[inline(always)]
        pub fn set_mask_ack_with_err_15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "disable ErrEsc escape entry error from lane0."]
        #[inline(always)]
        pub const fn mask_dphy_errors_0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "disable ErrEsc escape entry error from lane0."]
        #[inline(always)]
        pub fn set_mask_dphy_errors_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "disable ErrSyncEsc low-power data transmission synchronization error from lane 0."]
        #[inline(always)]
        pub const fn mask_dphy_errors_1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "disable ErrSyncEsc low-power data transmission synchronization error from lane 0."]
        #[inline(always)]
        pub fn set_mask_dphy_errors_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "disable control error ErrControl from lane0."]
        #[inline(always)]
        pub const fn mask_dphy_errors_2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "disable control error ErrControl from lane0."]
        #[inline(always)]
        pub fn set_mask_dphy_errors_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "disable LP0 contention error ErrContentionLP0 from lane0."]
        #[inline(always)]
        pub const fn mask_dphy_errors_3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "disable LP0 contention error ErrContentionLP0 from lane0."]
        #[inline(always)]
        pub fn set_mask_dphy_errors_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "disable LP1 contention error ErrContentionLP1 from lane0."]
        #[inline(always)]
        pub const fn mask_dphy_errors_4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "disable LP1 contention error ErrContentionLP1 from lane0."]
        #[inline(always)]
        pub fn set_mask_dphy_errors_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for IntMsk0 {
        #[inline(always)]
        fn default() -> IntMsk0 {
            IntMsk0(0)
        }
    }
    #[doc = "configures masks for int_st1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntMsk1(pub u32);
    impl IntMsk1 {
        #[doc = "disable that the high-speed transmission timeout counter reached the end and contention has been detected."]
        #[inline(always)]
        pub const fn mask_to_hs_tx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "disable that the high-speed transmission timeout counter reached the end and contention has been detected."]
        #[inline(always)]
        pub fn set_mask_to_hs_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "disable that the low-power reception timeout counter reached the end and contention has been detected."]
        #[inline(always)]
        pub const fn mask_to_lp_tx(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "disable that the low-power reception timeout counter reached the end and contention has been detected."]
        #[inline(always)]
        pub fn set_mask_to_lp_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "disable that the ECC single error has been detected and corrected in a reveived packet."]
        #[inline(always)]
        pub const fn mask_ecc_sigle_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "disable that the ECC single error has been detected and corrected in a reveived packet."]
        #[inline(always)]
        pub fn set_mask_ecc_sigle_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "disable that the ECC multiple error has been detected in a revieved packet."]
        #[inline(always)]
        pub const fn mask_ecc_multi_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "disable that the ECC multiple error has been detected in a revieved packet."]
        #[inline(always)]
        pub fn set_mask_ecc_multi_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "disable that the CRC error has been detected in the reveived packet payload."]
        #[inline(always)]
        pub const fn mask_crc_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "disable that the CRC error has been detected in the reveived packet payload."]
        #[inline(always)]
        pub fn set_mask_crc_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "disable that the packet size error has been detected during the packet reception."]
        #[inline(always)]
        pub const fn mask_pkt_size_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "disable that the packet size error has been detected during the packet reception."]
        #[inline(always)]
        pub fn set_mask_pkt_size_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "disable that the EoTp packet has not been received at the end of the incoming peripheral transmission."]
        #[inline(always)]
        pub const fn mask_eopt_err(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "disable that the EoTp packet has not been received at the end of the incoming peripheral transmission."]
        #[inline(always)]
        pub fn set_mask_eopt_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "disable the payload FIFO is full during a DPI pixel line storage."]
        #[inline(always)]
        pub const fn mask_dpi_bpld_wr_err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "disable the payload FIFO is full during a DPI pixel line storage."]
        #[inline(always)]
        pub fn set_mask_dpi_bpld_wr_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "disable the system tried to write a command and FIFO is full."]
        #[inline(always)]
        pub const fn mask_gen_cmd_wr_err(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "disable the system tried to write a command and FIFO is full."]
        #[inline(always)]
        pub fn set_mask_gen_cmd_wr_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "disable the system tried to write a payload and FIFO is full."]
        #[inline(always)]
        pub const fn mask_gen_pld_wr_err(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "disable the system tried to write a payload and FIFO is full."]
        #[inline(always)]
        pub fn set_mask_gen_pld_wr_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "disable the payload FIFO become empty when packet build."]
        #[inline(always)]
        pub const fn mask_gen_pld_send_err(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "disable the payload FIFO become empty when packet build."]
        #[inline(always)]
        pub fn set_mask_gen_pld_send_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "disable that during a DCS read data, the payload FIFO becomes empty."]
        #[inline(always)]
        pub const fn mask_gen_pld_rd_err(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "disable that during a DCS read data, the payload FIFO becomes empty."]
        #[inline(always)]
        pub fn set_mask_gen_pld_rd_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "disable that during a generic interface packet read back, the payload FIFO full."]
        #[inline(always)]
        pub const fn mask_gen_pld_recev_err(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "disable that during a generic interface packet read back, the payload FIFO full."]
        #[inline(always)]
        pub fn set_mask_gen_pld_recev_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "disable an underflow when reading payload to build dsi packet for video mode."]
        #[inline(always)]
        pub const fn mask_dpi_buff_pld_under(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "disable an underflow when reading payload to build dsi packet for video mode."]
        #[inline(always)]
        pub fn set_mask_dpi_buff_pld_under(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "disable tear_request has occurred but tear effect is not active in dsi host and device."]
        #[inline(always)]
        pub const fn mask_tear_request_err(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "disable tear_request has occurred but tear effect is not active in dsi host and device."]
        #[inline(always)]
        pub fn set_mask_tear_request_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for IntMsk1 {
        #[inline(always)]
        fn default() -> IntMsk1 {
            IntMsk1(0)
        }
    }
    #[doc = "controls the status of interrupt."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntSt0(pub u32);
    impl IntSt0 {
        #[doc = "retrives the SoT serror from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "retrives the SoT serror from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "retrives the SoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "retrives the SoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "retrives the EoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "retrives the EoT sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "retrives the Escap mode entry command error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "retrives the Escap mode entry command error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "retrives the LP transmit sync error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "retrives the LP transmit sync error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "retrives the peripheral timeout error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "retrives the peripheral timeout error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "retrieves the false control error fro the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "retrieves the false control error fro the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "retrieves the reserved from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "retrieves the reserved from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "retrives the ecc error sigle-bit from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "retrives the ecc error sigle-bit from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "retrives the ECC error multi-bit from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err_9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "retrives the ECC error multi-bit from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err_9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "retrives the checksum error from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err_10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "retrives the checksum error from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err_10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "retrives the not recongnized dsi data type from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err_11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "retrives the not recongnized dsi data type from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err_11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "retrieves the dsi vc id invalid from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err_12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "retrieves the dsi vc id invalid from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err_12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "retrives the invalid transmission length from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err_13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "retrives the invalid transmission length from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err_13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "retrives the reserved from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err_14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "retrives the reserved from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err_14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "retrives the DSI protocal violation from the acknowledge error report."]
        #[inline(always)]
        pub const fn ack_with_err_15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "retrives the DSI protocal violation from the acknowledge error report."]
        #[inline(always)]
        pub fn set_ack_with_err_15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "indicates ErrEsc escape entry error from lane0."]
        #[inline(always)]
        pub const fn dphy_errors_0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "indicates ErrEsc escape entry error from lane0."]
        #[inline(always)]
        pub fn set_dphy_errors_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "indicates ErrSyncEsc low-power data transmission synchronization error from lane 0."]
        #[inline(always)]
        pub const fn dphy_errors_1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "indicates ErrSyncEsc low-power data transmission synchronization error from lane 0."]
        #[inline(always)]
        pub fn set_dphy_errors_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "indicates control error ErrControl from lane0."]
        #[inline(always)]
        pub const fn dphy_errors_2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "indicates control error ErrControl from lane0."]
        #[inline(always)]
        pub fn set_dphy_errors_2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "indicates LP0 contention error ErrContentionLP0 from lane0."]
        #[inline(always)]
        pub const fn dphy_errors_3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "indicates LP0 contention error ErrContentionLP0 from lane0."]
        #[inline(always)]
        pub fn set_dphy_errors_3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "indicates LP1 contention error ErrContentionLP1 from lane0."]
        #[inline(always)]
        pub const fn dphy_errors_4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "indicates LP1 contention error ErrContentionLP1 from lane0."]
        #[inline(always)]
        pub fn set_dphy_errors_4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for IntSt0 {
        #[inline(always)]
        fn default() -> IntSt0 {
            IntSt0(0)
        }
    }
    #[doc = "the interrupt source related to timeout etc."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntSt1(pub u32);
    impl IntSt1 {
        #[doc = "indicates that the high-speed transmission timeout counter reached the end and contention has been detected."]
        #[inline(always)]
        pub const fn to_hs_tx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "indicates that the high-speed transmission timeout counter reached the end and contention has been detected."]
        #[inline(always)]
        pub fn set_to_hs_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "indicates that the low-power reception timeout counter reached the end and contention has been detected."]
        #[inline(always)]
        pub const fn to_lp_tx(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "indicates that the low-power reception timeout counter reached the end and contention has been detected."]
        #[inline(always)]
        pub fn set_to_lp_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "indicates that the ECC single error has been detected and corrected in a reveived packet."]
        #[inline(always)]
        pub const fn ecc_sigle_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "indicates that the ECC single error has been detected and corrected in a reveived packet."]
        #[inline(always)]
        pub fn set_ecc_sigle_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "indicates that the ECC multiple error has been detected in a revieved packet."]
        #[inline(always)]
        pub const fn ecc_multi_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "indicates that the ECC multiple error has been detected in a revieved packet."]
        #[inline(always)]
        pub fn set_ecc_multi_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "indicates that the CRC error has been detected in the reveived packet payload."]
        #[inline(always)]
        pub const fn crc_err(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "indicates that the CRC error has been detected in the reveived packet payload."]
        #[inline(always)]
        pub fn set_crc_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "indicates that the packet size error has been detected during the packet reception."]
        #[inline(always)]
        pub const fn pkt_size_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "indicates that the packet size error has been detected during the packet reception."]
        #[inline(always)]
        pub fn set_pkt_size_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "indicates that the EoTp packet has not been received at the end of the incoming peripheral transmission."]
        #[inline(always)]
        pub const fn eopt_err(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "indicates that the EoTp packet has not been received at the end of the incoming peripheral transmission."]
        #[inline(always)]
        pub fn set_eopt_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "indicates the payload FIFO is full during a DPI pixel line storage."]
        #[inline(always)]
        pub const fn dpi_bpld_wr_err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the payload FIFO is full during a DPI pixel line storage."]
        #[inline(always)]
        pub fn set_dpi_bpld_wr_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "indicates the system tried to write a command and FIFO is full."]
        #[inline(always)]
        pub const fn gen_cmd_wr_err(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the system tried to write a command and FIFO is full."]
        #[inline(always)]
        pub fn set_gen_cmd_wr_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "indicates the system tried to write a payload and FIFO is full."]
        #[inline(always)]
        pub const fn gen_pld_wr_err(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the system tried to write a payload and FIFO is full."]
        #[inline(always)]
        pub fn set_gen_pld_wr_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "indicates the payload FIFO become empty when packet build."]
        #[inline(always)]
        pub const fn gen_pld_send_err(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the payload FIFO become empty when packet build."]
        #[inline(always)]
        pub fn set_gen_pld_send_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "indicates that during a DCS read data, the payload FIFO becomes empty."]
        #[inline(always)]
        pub const fn gen_pld_rd_err(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "indicates that during a DCS read data, the payload FIFO becomes empty."]
        #[inline(always)]
        pub fn set_gen_pld_rd_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "indicates that during a generic interface packet read back, the payload FIFO full."]
        #[inline(always)]
        pub const fn gen_pld_recev_err(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "indicates that during a generic interface packet read back, the payload FIFO full."]
        #[inline(always)]
        pub fn set_gen_pld_recev_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "indicates an underflow when reading payload to build dsi packet for video mode."]
        #[inline(always)]
        pub const fn dpi_buff_pld_under(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "indicates an underflow when reading payload to build dsi packet for video mode."]
        #[inline(always)]
        pub fn set_dpi_buff_pld_under(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "indicates tear_request has occurred but tear effect is not active in dsi host and device."]
        #[inline(always)]
        pub const fn tear_request_err(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "indicates tear_request has occurred but tear effect is not active in dsi host and device."]
        #[inline(always)]
        pub fn set_tear_request_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for IntSt1 {
        #[inline(always)]
        fn default() -> IntSt1 {
            IntSt1(0)
        }
    }
    #[doc = "configures the peripheral response timeout after low-power read operation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LpRdToCnt(pub u32);
    impl LpRdToCnt {
        #[doc = "sets a period for which dwc_mipi_dsi_host keeps the link still after sending a low power read operation."]
        #[inline(always)]
        pub const fn lp_rd_to_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "sets a period for which dwc_mipi_dsi_host keeps the link still after sending a low power read operation."]
        #[inline(always)]
        pub fn set_lp_rd_to_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LpRdToCnt {
        #[inline(always)]
        fn default() -> LpRdToCnt {
            LpRdToCnt(0)
        }
    }
    #[doc = "configures the peripheral response timeout after low power write operations."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LpWrToCnt(pub u32);
    impl LpWrToCnt {
        #[doc = "sets the period for which dsi host keeps the link still after sending a low power write operation."]
        #[inline(always)]
        pub const fn lp_wr_to_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "sets the period for which dsi host keeps the link still after sending a low power write operation."]
        #[inline(always)]
        pub fn set_lp_wr_to_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for LpWrToCnt {
        #[inline(always)]
        fn default() -> LpWrToCnt {
            LpWrToCnt(0)
        }
    }
    #[doc = "configures the possibility for using non continuous clock in the clock lane."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LpclkCtrl(pub u32);
    impl LpclkCtrl {
        #[doc = "controls the D-PHY PPI txrequestclkhs signal."]
        #[inline(always)]
        pub const fn phy_txrequestclkhs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "controls the D-PHY PPI txrequestclkhs signal."]
        #[inline(always)]
        pub fn set_phy_txrequestclkhs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "enables the automatic mechanism to stop providing clock in the clock lane."]
        #[inline(always)]
        pub const fn auto_clklane_ctrl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enables the automatic mechanism to stop providing clock in the clock lane."]
        #[inline(always)]
        pub fn set_auto_clklane_ctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for LpclkCtrl {
        #[inline(always)]
        fn default() -> LpclkCtrl {
            LpclkCtrl(0)
        }
    }
    #[doc = "configures the mode of operation between video or command mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModeCfg(pub u32);
    impl ModeCfg {
        #[doc = "0x0: video mode 0x1: command mode."]
        #[inline(always)]
        pub const fn cmd_video_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "0x0: video mode 0x1: command mode."]
        #[inline(always)]
        pub fn set_cmd_video_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for ModeCfg {
        #[inline(always)]
        fn default() -> ModeCfg {
            ModeCfg(0)
        }
    }
    #[doc = "configures how EoTp, BTA, CRC and ECC to be used."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PckhdlCfg(pub u32);
    impl PckhdlCfg {
        #[doc = "enable the EoTp transmission in high-speed."]
        #[inline(always)]
        pub const fn eotp_tx_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable the EoTp transmission in high-speed."]
        #[inline(always)]
        pub fn set_eotp_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "enable the EoTp reception."]
        #[inline(always)]
        pub const fn eotp_rx_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable the EoTp reception."]
        #[inline(always)]
        pub fn set_eotp_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "enable the bus turn-around request."]
        #[inline(always)]
        pub const fn bta_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable the bus turn-around request."]
        #[inline(always)]
        pub fn set_bta_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "enable the ecc reception error correction and reporting."]
        #[inline(always)]
        pub const fn ecc_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable the ecc reception error correction and reporting."]
        #[inline(always)]
        pub fn set_ecc_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "enable the crc reception and error reporting."]
        #[inline(always)]
        pub const fn crc_rx_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable the crc reception and error reporting."]
        #[inline(always)]
        pub fn set_crc_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "enable the EoTp transmission in low-power."]
        #[inline(always)]
        pub const fn eotp_tx_lp_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable the EoTp transmission in low-power."]
        #[inline(always)]
        pub fn set_eotp_tx_lp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for PckhdlCfg {
        #[inline(always)]
        fn default() -> PckhdlCfg {
            PckhdlCfg(0)
        }
    }
    #[doc = "controls the skew calibration of D-phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyCal(pub u32);
    impl PhyCal {
        #[doc = "High-speed skew calibration is started when txskewcalhs is set high (assuming that PHY is in Stop state)."]
        #[inline(always)]
        pub const fn txskewcalhs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "High-speed skew calibration is started when txskewcalhs is set high (assuming that PHY is in Stop state)."]
        #[inline(always)]
        pub fn set_txskewcalhs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PhyCal {
        #[inline(always)]
        fn default() -> PhyCal {
            PhyCal(0)
        }
    }
    #[doc = "configures the number of active lanes."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyIfCfg(pub u32);
    impl PhyIfCfg {
        #[doc = "configures the number of active data lanes."]
        #[inline(always)]
        pub const fn n_lanes(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "configures the number of active data lanes."]
        #[inline(always)]
        pub fn set_n_lanes(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "configures the minimum time phy needs to stay in stopstate before requesting an highspeed transmission."]
        #[inline(always)]
        pub const fn phy_stop_wait_time(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "configures the minimum time phy needs to stay in stopstate before requesting an highspeed transmission."]
        #[inline(always)]
        pub fn set_phy_stop_wait_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for PhyIfCfg {
        #[inline(always)]
        fn default() -> PhyIfCfg {
            PhyIfCfg(0)
        }
    }
    #[doc = "select phy mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyMode(pub u32);
    impl PhyMode {
        #[doc = "sel DPHY or CPHY."]
        #[inline(always)]
        pub const fn phy_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "sel DPHY or CPHY."]
        #[inline(always)]
        pub fn set_phy_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PhyMode {
        #[inline(always)]
        fn default() -> PhyMode {
            PhyMode(0)
        }
    }
    #[doc = "controls resets and the pll of d-phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyRstz(pub u32);
    impl PhyRstz {
        #[doc = "places the dphy macro in power down mode when set to 0."]
        #[inline(always)]
        pub const fn phy_shutdownz(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "places the dphy macro in power down mode when set to 0."]
        #[inline(always)]
        pub fn set_phy_shutdownz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "make the dphy in reset state when set to 0."]
        #[inline(always)]
        pub const fn phy_rstz(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "make the dphy in reset state when set to 0."]
        #[inline(always)]
        pub fn set_phy_rstz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "enable dphy clock lane."]
        #[inline(always)]
        pub const fn phy_enableclk(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable dphy clock lane."]
        #[inline(always)]
        pub fn set_phy_enableclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "when the d-phy is in ulps, enable the d-phy pll."]
        #[inline(always)]
        pub const fn phy_forcepll(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "when the d-phy is in ulps, enable the d-phy pll."]
        #[inline(always)]
        pub fn set_phy_forcepll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for PhyRstz {
        #[inline(always)]
        fn default() -> PhyRstz {
            PhyRstz(0)
        }
    }
    #[doc = "contains information about the status of the d-phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyStatus(pub u32);
    impl PhyStatus {
        #[doc = "This bit indicates the status of phylock D-PHY signal."]
        #[inline(always)]
        pub const fn phy_lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the status of phylock D-PHY signal."]
        #[inline(always)]
        pub fn set_phy_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "This bit indicates the status of phydirection D-PHY signal."]
        #[inline(always)]
        pub const fn phy_direction(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the status of phydirection D-PHY signal."]
        #[inline(always)]
        pub fn set_phy_direction(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "This bit indicates the status of phystopstateclklane D-PHY signal."]
        #[inline(always)]
        pub const fn phy_stopstateclklane(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the status of phystopstateclklane D-PHY signal."]
        #[inline(always)]
        pub fn set_phy_stopstateclklane(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "This bit indicates the status of phyulpsactivenotclk D-PHY signal."]
        #[inline(always)]
        pub const fn phy_ulpsactivenotclk(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the status of phyulpsactivenotclk D-PHY signal."]
        #[inline(always)]
        pub fn set_phy_ulpsactivenotclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "This bit indicates the status of phystopstate0lane D-PHY signal."]
        #[inline(always)]
        pub const fn phy_stopstate0lane(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the status of phystopstate0lane D-PHY signal."]
        #[inline(always)]
        pub fn set_phy_stopstate0lane(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "This bit indicates the status of ulpsactivenot0lane D-PHY signal."]
        #[inline(always)]
        pub const fn phy_ulpsactivenot0lane(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the status of ulpsactivenot0lane D-PHY signal."]
        #[inline(always)]
        pub fn set_phy_ulpsactivenot0lane(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "This bit indicates the status of rxulpsesc0lane D-PHY signa."]
        #[inline(always)]
        pub const fn phy_rxulpsesc0lane(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the status of rxulpsesc0lane D-PHY signa."]
        #[inline(always)]
        pub fn set_phy_rxulpsesc0lane(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "This bit indicates the status of phystopstate1lane D-PHY signal."]
        #[inline(always)]
        pub const fn phy_stopstate1lane(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the status of phystopstate1lane D-PHY signal."]
        #[inline(always)]
        pub fn set_phy_stopstate1lane(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "This bit indicates the status of ulpsactivenot1lane D-PHY signal."]
        #[inline(always)]
        pub const fn phy_ulpsactivenot1lane(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the status of ulpsactivenot1lane D-PHY signal."]
        #[inline(always)]
        pub fn set_phy_ulpsactivenot1lane(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "This bit indicates the status of phystopstate2lane D-PHY signal."]
        #[inline(always)]
        pub const fn phy_stopstate2lane(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the status of phystopstate2lane D-PHY signal."]
        #[inline(always)]
        pub fn set_phy_stopstate2lane(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "This bit indicates the status of ulpsactivenot2lane D-PHY signa."]
        #[inline(always)]
        pub const fn phy_ulpsactivenot2lane(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the status of ulpsactivenot2lane D-PHY signa."]
        #[inline(always)]
        pub fn set_phy_ulpsactivenot2lane(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "This bit indicates the status of phystopstate3lane D-PHY signal."]
        #[inline(always)]
        pub const fn phy_stopstate3lane(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the status of phystopstate3lane D-PHY signal."]
        #[inline(always)]
        pub fn set_phy_stopstate3lane(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "indicates the status of ulpsactivenot3lane d-phy signal."]
        #[inline(always)]
        pub const fn phy_ulpsactivenot3lane(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the status of ulpsactivenot3lane d-phy signal."]
        #[inline(always)]
        pub fn set_phy_ulpsactivenot3lane(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for PhyStatus {
        #[inline(always)]
        fn default() -> PhyStatus {
            PhyStatus(0)
        }
    }
    #[doc = "sets the time that dsi host assumes in calculations for data lanes to switch between hs to lp."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyTmrCfg(pub u32);
    impl PhyTmrCfg {
        #[doc = "This field configures the maximum time that the D-PHY data lanes take to go from low-power to high-speed transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub const fn phy_lp2hs_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "This field configures the maximum time that the D-PHY data lanes take to go from low-power to high-speed transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_phy_lp2hs_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "This field configures the maximum time that the D-PHY data lanes take to go from high-speed to low-power transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub const fn phy_hs2lp_time(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "This field configures the maximum time that the D-PHY data lanes take to go from high-speed to low-power transmission measured in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_phy_hs2lp_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for PhyTmrCfg {
        #[inline(always)]
        fn default() -> PhyTmrCfg {
            PhyTmrCfg(0)
        }
    }
    #[doc = "sets the time that dsi host assumes in calculations for the clock lane to switch between high-speed and low-power."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyTmrLpclkCfg(pub u32);
    impl PhyTmrLpclkCfg {
        #[doc = "configures the maximum time that the d-phy clock lane takes to go from low-power to high-speed transmission."]
        #[inline(always)]
        pub const fn phy_clklp2hs_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "configures the maximum time that the d-phy clock lane takes to go from low-power to high-speed transmission."]
        #[inline(always)]
        pub fn set_phy_clklp2hs_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "configures the maximum time that the d-phy clock lane takes to go from high-speed to low-power transmission."]
        #[inline(always)]
        pub const fn phy_clkhs2lp_time(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "configures the maximum time that the d-phy clock lane takes to go from high-speed to low-power transmission."]
        #[inline(always)]
        pub fn set_phy_clkhs2lp_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for PhyTmrLpclkCfg {
        #[inline(always)]
        fn default() -> PhyTmrLpclkCfg {
            PhyTmrLpclkCfg(0)
        }
    }
    #[doc = "configures times related to PHY to perform some operations in lane byte clock cycle."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyTmrRd(pub u32);
    impl PhyTmrRd {
        #[doc = "the maximum time required to perform a read command in lane byte clock cycles."]
        #[inline(always)]
        pub const fn max_rd_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "the maximum time required to perform a read command in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_max_rd_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for PhyTmrRd {
        #[inline(always)]
        fn default() -> PhyTmrRd {
            PhyTmrRd(0)
        }
    }
    #[doc = "controls clock and clear pins of the d-phy vendor specific interface."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyTstCtrl0(pub u32);
    impl PhyTstCtrl0 {
        #[doc = "reserve."]
        #[inline(always)]
        pub const fn phy_testclr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "reserve."]
        #[inline(always)]
        pub fn set_phy_testclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "reserve."]
        #[inline(always)]
        pub const fn phy_testclk(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "reserve."]
        #[inline(always)]
        pub fn set_phy_testclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for PhyTstCtrl0 {
        #[inline(always)]
        fn default() -> PhyTstCtrl0 {
            PhyTstCtrl0(0)
        }
    }
    #[doc = "controls data and enable pins of the d-phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyTstCtrl1(pub u32);
    impl PhyTstCtrl1 {
        #[doc = "reserve."]
        #[inline(always)]
        pub const fn phy_testdin(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "reserve."]
        #[inline(always)]
        pub fn set_phy_testdin(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "reserve."]
        #[inline(always)]
        pub const fn phy_testdout(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "reserve."]
        #[inline(always)]
        pub fn set_phy_testdout(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "reserve."]
        #[inline(always)]
        pub const fn phy_testen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "reserve."]
        #[inline(always)]
        pub fn set_phy_testen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for PhyTstCtrl1 {
        #[inline(always)]
        fn default() -> PhyTstCtrl1 {
            PhyTstCtrl1(0)
        }
    }
    #[doc = "configures the pins that activate triggers in the d-phy."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyTxTriggers(pub u32);
    impl PhyTxTriggers {
        #[doc = "controls the trigger transmissions."]
        #[inline(always)]
        pub const fn phy_tx_triggers(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "controls the trigger transmissions."]
        #[inline(always)]
        pub fn set_phy_tx_triggers(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for PhyTxTriggers {
        #[inline(always)]
        fn default() -> PhyTxTriggers {
            PhyTxTriggers(0)
        }
    }
    #[doc = "configures entering and leaving ulps."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyUlpsCtrl(pub u32);
    impl PhyUlpsCtrl {
        #[doc = "ulps mode request on clock lane."]
        #[inline(always)]
        pub const fn phy_txrequlpsclk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ulps mode request on clock lane."]
        #[inline(always)]
        pub fn set_phy_txrequlpsclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ulps mode exit on clock lane."]
        #[inline(always)]
        pub const fn phy_txexitulpsclk(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ulps mode exit on clock lane."]
        #[inline(always)]
        pub fn set_phy_txexitulpsclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ulps mode request on all active data lanes."]
        #[inline(always)]
        pub const fn phy_txrequlpslan(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ulps mode request on all active data lanes."]
        #[inline(always)]
        pub fn set_phy_txrequlpslan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ulps mode exit on all active data lanes."]
        #[inline(always)]
        pub const fn phy_txexitulpslan(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ulps mode exit on all active data lanes."]
        #[inline(always)]
        pub fn set_phy_txexitulpslan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for PhyUlpsCtrl {
        #[inline(always)]
        fn default() -> PhyUlpsCtrl {
            PhyUlpsCtrl(0)
        }
    }
    #[doc = "power up."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PwrUp(pub u32);
    impl PwrUp {
        #[doc = "0x0: reset the core 0x1: power up the core."]
        #[inline(always)]
        pub const fn shutdownz(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "0x0: reset the core 0x1: power up the core."]
        #[inline(always)]
        pub fn set_shutdownz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for PwrUp {
        #[inline(always)]
        fn default() -> PwrUp {
            PwrUp(0)
        }
    }
    #[doc = "sotres 3d control information for vss packets in video mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdf3d(pub u32);
    impl Sdf3d {
        #[doc = "defines 3D mode on/off."]
        #[inline(always)]
        pub const fn mode_3d(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "defines 3D mode on/off."]
        #[inline(always)]
        pub fn set_mode_3d(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "defines 3D image format."]
        #[inline(always)]
        pub const fn format_3d(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "defines 3D image format."]
        #[inline(always)]
        pub fn set_format_3d(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "defines whether there is a second VSYNC pulse."]
        #[inline(always)]
        pub const fn second_vsync(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "defines whether there is a second VSYNC pulse."]
        #[inline(always)]
        pub fn set_second_vsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "0x0: left eye is sent first 0x1:right eye is sent first."]
        #[inline(always)]
        pub const fn right_first(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "0x0: left eye is sent first 0x1:right eye is sent first."]
        #[inline(always)]
        pub fn set_right_first(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "set the next vss packet to include 3d control payload in every vss packet."]
        #[inline(always)]
        pub const fn send_3d_cfg(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "set the next vss packet to include 3d control payload in every vss packet."]
        #[inline(always)]
        pub fn set_send_3d_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Sdf3d {
        #[inline(always)]
        fn default() -> Sdf3d {
            Sdf3d(0)
        }
    }
    #[doc = "value for sdf_3d."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdf3dAct(pub u32);
    impl Sdf3dAct {
        #[doc = "This field specifies 3D Mode On/Off and Display Orientation."]
        #[inline(always)]
        pub const fn mode_3d(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "This field specifies 3D Mode On/Off and Display Orientation."]
        #[inline(always)]
        pub fn set_mode_3d(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "This field specifies 3D Image Format."]
        #[inline(always)]
        pub const fn format_3d(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "This field specifies 3D Image Format."]
        #[inline(always)]
        pub fn set_format_3d(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "This field specifies whether there is a second VSYNC pulse between Left and Right Images, when 3D Image Format is Frame-based."]
        #[inline(always)]
        pub const fn second_vsync(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "This field specifies whether there is a second VSYNC pulse between Left and Right Images, when 3D Image Format is Frame-based."]
        #[inline(always)]
        pub fn set_second_vsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "This bit specifies the left/right order."]
        #[inline(always)]
        pub const fn right_first(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "This bit specifies the left/right order."]
        #[inline(always)]
        pub fn set_right_first(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "When set, causes the next VSS packet to include 3D control payload in every VSS packet."]
        #[inline(always)]
        pub const fn send_3d_cfg(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "When set, causes the next VSS packet to include 3D control payload in every VSS packet."]
        #[inline(always)]
        pub fn set_send_3d_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Sdf3dAct {
        #[inline(always)]
        fn default() -> Sdf3dAct {
            Sdf3dAct(0)
        }
    }
    #[doc = "configures the trigger timeout errors."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ToCntCfg(pub u32);
    impl ToCntCfg {
        #[doc = "configures the timeout counter that triggers a low power reception timeout contention detection."]
        #[inline(always)]
        pub const fn lprx_to_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "configures the timeout counter that triggers a low power reception timeout contention detection."]
        #[inline(always)]
        pub fn set_lprx_to_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "configures the timeout counter that triggers a high speed transmission timeout contention detection."]
        #[inline(always)]
        pub const fn hstx_to_cnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "configures the timeout counter that triggers a high speed transmission timeout contention detection."]
        #[inline(always)]
        pub fn set_hstx_to_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for ToCntCfg {
        #[inline(always)]
        fn default() -> ToCntCfg {
            ToCntCfg(0)
        }
    }
    #[doc = "version."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Version(pub u32);
    impl Version {
        #[doc = "version of DSI."]
        #[inline(always)]
        pub const fn version(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "version of DSI."]
        #[inline(always)]
        pub fn set_version(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Version {
        #[inline(always)]
        fn default() -> Version {
            Version(0)
        }
    }
    #[doc = "configure the video HBP time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidHbpTime(pub u32);
    impl VidHbpTime {
        #[doc = "configures the Horizontal back porch period in lane byte clock cycles."]
        #[inline(always)]
        pub const fn vid_hpb_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "configures the Horizontal back porch period in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_vid_hpb_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for VidHbpTime {
        #[inline(always)]
        fn default() -> VidHbpTime {
            VidHbpTime(0)
        }
    }
    #[doc = "the value that controller is using for vid_hbp_time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidHbpTimeAct(pub u32);
    impl VidHbpTimeAct {
        #[doc = "the horizontal back porch period in lane byte clock cycles."]
        #[inline(always)]
        pub const fn vid_hbp_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "the horizontal back porch period in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_vid_hbp_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for VidHbpTimeAct {
        #[inline(always)]
        fn default() -> VidHbpTimeAct {
            VidHbpTimeAct(0)
        }
    }
    #[doc = "configures the overall time for each video line."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidHlineTime(pub u32);
    impl VidHlineTime {
        #[doc = "configures the size of the total line time in lane byte clock cycles."]
        #[inline(always)]
        pub const fn vid_hline_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "configures the size of the total line time in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_vid_hline_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for VidHlineTime {
        #[inline(always)]
        fn default() -> VidHlineTime {
            VidHlineTime(0)
        }
    }
    #[doc = "the value for vid_hline_time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidHlineTimeAct(pub u32);
    impl VidHlineTimeAct {
        #[doc = "the size of total line: hsa+hbp+hact+hfp."]
        #[inline(always)]
        pub const fn vid_hline_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "the size of total line: hsa+hbp+hact+hfp."]
        #[inline(always)]
        pub fn set_vid_hline_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
        }
    }
    impl Default for VidHlineTimeAct {
        #[inline(always)]
        fn default() -> VidHlineTimeAct {
            VidHlineTimeAct(0)
        }
    }
    #[doc = "configures the video HAS time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidHsaTime(pub u32);
    impl VidHsaTime {
        #[doc = "configure the Horizontal synchronism active period in lane byte clock cycles."]
        #[inline(always)]
        pub const fn vid_hsa_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "configure the Horizontal synchronism active period in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_vid_hsa_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for VidHsaTime {
        #[inline(always)]
        fn default() -> VidHsaTime {
            VidHsaTime(0)
        }
    }
    #[doc = "the value of vid_hsa_time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidHsaTimeAct(pub u32);
    impl VidHsaTimeAct {
        #[doc = "the horizontal synchronism active period in lane byte clock cycles."]
        #[inline(always)]
        pub const fn vid_hsa_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "the horizontal synchronism active period in lane byte clock cycles."]
        #[inline(always)]
        pub fn set_vid_hsa_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for VidHsaTimeAct {
        #[inline(always)]
        fn default() -> VidHsaTimeAct {
            VidHsaTimeAct(0)
        }
    }
    #[doc = "several aspect of video mode operation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidModeCfg(pub u32);
    impl VidModeCfg {
        #[doc = "indicates the video mode transmission type."]
        #[inline(always)]
        pub const fn vid_mode_type(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "indicates the video mode transmission type."]
        #[inline(always)]
        pub fn set_vid_mode_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "enable the return to low-power inside the VSA period when timing allows."]
        #[inline(always)]
        pub const fn lp_vsa_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "enable the return to low-power inside the VSA period when timing allows."]
        #[inline(always)]
        pub fn set_lp_vsa_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "enable the return to low-power inside the VBP period when timing allows."]
        #[inline(always)]
        pub const fn lp_vbp_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "enable the return to low-power inside the VBP period when timing allows."]
        #[inline(always)]
        pub fn set_lp_vbp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "enable the return to low-power inside the VFP period when timing allows."]
        #[inline(always)]
        pub const fn lp_vfp_en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "enable the return to low-power inside the VFP period when timing allows."]
        #[inline(always)]
        pub fn set_lp_vfp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "enable the return to low-power inside the VACT period when timing allows."]
        #[inline(always)]
        pub const fn lp_vact_en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "enable the return to low-power inside the VACT period when timing allows."]
        #[inline(always)]
        pub fn set_lp_vact_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "enable the return to low-power inside the HBP period when timing allows."]
        #[inline(always)]
        pub const fn lp_hbp_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "enable the return to low-power inside the HBP period when timing allows."]
        #[inline(always)]
        pub fn set_lp_hbp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "enable the return to low-power inside the HFP period when timing allows."]
        #[inline(always)]
        pub const fn lp_hfp_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "enable the return to low-power inside the HFP period when timing allows."]
        #[inline(always)]
        pub fn set_lp_hfp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "enable the request for an acknowledge response at the end of a frame."]
        #[inline(always)]
        pub const fn frame_bta_ack_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "enable the request for an acknowledge response at the end of a frame."]
        #[inline(always)]
        pub fn set_frame_bta_ack_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "enable command transmission only in low-power mode."]
        #[inline(always)]
        pub const fn lp_cmd_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "enable command transmission only in low-power mode."]
        #[inline(always)]
        pub fn set_lp_cmd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "enable video mode pattern generator."]
        #[inline(always)]
        pub const fn vpg_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "enable video mode pattern generator."]
        #[inline(always)]
        pub fn set_vpg_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "0x0: colorbar 0x1: berpattern, vertical only."]
        #[inline(always)]
        pub const fn vpg_mode(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "0x0: colorbar 0x1: berpattern, vertical only."]
        #[inline(always)]
        pub fn set_vpg_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "indicates the color bar orientation : 0x0: vertical mode 0x1: horizontal mode."]
        #[inline(always)]
        pub const fn vpg_orientation(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "indicates the color bar orientation : 0x0: vertical mode 0x1: horizontal mode."]
        #[inline(always)]
        pub fn set_vpg_orientation(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for VidModeCfg {
        #[inline(always)]
        fn default() -> VidModeCfg {
            VidModeCfg(0)
        }
    }
    #[doc = "holds value that controller is using for vid_mode_cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidModeCfgAct(pub u32);
    impl VidModeCfgAct {
        #[doc = "specifies the video mode transmission type."]
        #[inline(always)]
        pub const fn vid_mode_type(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "specifies the video mode transmission type."]
        #[inline(always)]
        pub fn set_vid_mode_type(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "enable the returne to low-power inside the VSA period when timing allows."]
        #[inline(always)]
        pub const fn lp_vsa_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable the returne to low-power inside the VSA period when timing allows."]
        #[inline(always)]
        pub fn set_lp_vsa_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "enable the returne to low-power inside the VBP period when timing allows."]
        #[inline(always)]
        pub const fn lp_vbp_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable the returne to low-power inside the VBP period when timing allows."]
        #[inline(always)]
        pub fn set_lp_vbp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "enable the returne to low-power inside the VFP period when timing allows."]
        #[inline(always)]
        pub const fn lp_vfp_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable the returne to low-power inside the VFP period when timing allows."]
        #[inline(always)]
        pub fn set_lp_vfp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "enable the returne to low-power inside the VACT period when timing allows."]
        #[inline(always)]
        pub const fn lp_vact_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable the returne to low-power inside the VACT period when timing allows."]
        #[inline(always)]
        pub fn set_lp_vact_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "enable the returne to low-power inside the HBP period when timing allows."]
        #[inline(always)]
        pub const fn lp_hbp_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable the returne to low-power inside the HBP period when timing allows."]
        #[inline(always)]
        pub fn set_lp_hbp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "enable the returne to low-power inside the HFP period when timing allows."]
        #[inline(always)]
        pub const fn lp_hfp_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable the returne to low-power inside the HFP period when timing allows."]
        #[inline(always)]
        pub fn set_lp_hfp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "enable the request for an acknowledge response at the end of a frame."]
        #[inline(always)]
        pub const fn frame_bta_ack_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "enable the request for an acknowledge response at the end of a frame."]
        #[inline(always)]
        pub fn set_frame_bta_ack_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "enable the command transmission only in low-power mode."]
        #[inline(always)]
        pub const fn lp_cmd_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "enable the command transmission only in low-power mode."]
        #[inline(always)]
        pub fn set_lp_cmd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for VidModeCfgAct {
        #[inline(always)]
        fn default() -> VidModeCfgAct {
            VidModeCfgAct(0)
        }
    }
    #[doc = "configures the size of null packets."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidNullSize(pub u32);
    impl VidNullSize {
        #[doc = "configures the number of bytes inside a null packet."]
        #[inline(always)]
        pub const fn vid_null_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "configures the number of bytes inside a null packet."]
        #[inline(always)]
        pub fn set_vid_null_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for VidNullSize {
        #[inline(always)]
        fn default() -> VidNullSize {
            VidNullSize(0)
        }
    }
    #[doc = "holds the value that controller is using for vid_null_size."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidNullSizeAct(pub u32);
    impl VidNullSizeAct {
        #[doc = "the number of bytes in side a null packet."]
        #[inline(always)]
        pub const fn vid_null_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "the number of bytes in side a null packet."]
        #[inline(always)]
        pub fn set_vid_null_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for VidNullSizeAct {
        #[inline(always)]
        fn default() -> VidNullSizeAct {
            VidNullSizeAct(0)
        }
    }
    #[doc = "configures the number of chunks to use."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidNumChunks(pub u32);
    impl VidNumChunks {
        #[doc = "configures the number of chunks to be transmitted a line period."]
        #[inline(always)]
        pub const fn vid_num_chunks(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "configures the number of chunks to be transmitted a line period."]
        #[inline(always)]
        pub fn set_vid_num_chunks(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for VidNumChunks {
        #[inline(always)]
        fn default() -> VidNumChunks {
            VidNumChunks(0)
        }
    }
    #[doc = "holds value that controller is using for vid_num_chunks."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidNumChunksAct(pub u32);
    impl VidNumChunksAct {
        #[doc = "the number of chunks to be transmitted during a line period."]
        #[inline(always)]
        pub const fn vid_num_chunks(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "the number of chunks to be transmitted during a line period."]
        #[inline(always)]
        pub fn set_vid_num_chunks(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
    }
    impl Default for VidNumChunksAct {
        #[inline(always)]
        fn default() -> VidNumChunksAct {
            VidNumChunksAct(0)
        }
    }
    #[doc = "configures the video packet size."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidPktSize(pub u32);
    impl VidPktSize {
        #[doc = "configures the number of pixels in a single video packet."]
        #[inline(always)]
        pub const fn vid_pkt_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "configures the number of pixels in a single video packet."]
        #[inline(always)]
        pub fn set_vid_pkt_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for VidPktSize {
        #[inline(always)]
        fn default() -> VidPktSize {
            VidPktSize(0)
        }
    }
    #[doc = "holds value that controller is using for vid_pkt_size."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidPktSizeAct(pub u32);
    impl VidPktSizeAct {
        #[doc = "the number of pixels in a single video packet."]
        #[inline(always)]
        pub const fn vid_pkt_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "the number of pixels in a single video packet."]
        #[inline(always)]
        pub fn set_vid_pkt_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for VidPktSizeAct {
        #[inline(always)]
        fn default() -> VidPktSizeAct {
            VidPktSizeAct(0)
        }
    }
    #[doc = "status of fifo related to dpi."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidPktStatus(pub u32);
    impl VidPktStatus {
        #[doc = "This bit indicates the empty status of write command FIFO for video Mode. This bit is set to 0 for command Mode."]
        #[inline(always)]
        pub const fn dpi_cmd_w_empty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the empty status of write command FIFO for video Mode. This bit is set to 0 for command Mode."]
        #[inline(always)]
        pub fn set_dpi_cmd_w_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "This bit indicates the full status of write command FIFO for video Mode. This bit is set to 0 for command Mode."]
        #[inline(always)]
        pub const fn dpi_cmd_w_full(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the full status of write command FIFO for video Mode. This bit is set to 0 for command Mode."]
        #[inline(always)]
        pub fn set_dpi_cmd_w_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "This bit indicates the empty status of write payload FIFO for video Mode. This bit is set to 0 for command Mode."]
        #[inline(always)]
        pub const fn dpi_pld_w_empty(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the empty status of write payload FIFO for video Mode. This bit is set to 0 for command Mode."]
        #[inline(always)]
        pub fn set_dpi_pld_w_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "This bit indicates the full status of write payload FIFO for video Mode. This bit is set to 0 for command Mode."]
        #[inline(always)]
        pub const fn dpi_pld_w_full(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the full status of write payload FIFO for video Mode. This bit is set to 0 for command Mode."]
        #[inline(always)]
        pub fn set_dpi_pld_w_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "This bit indicates the empty status of the payload internal buffer for video Mode. This bit is set to 0 for command Mod."]
        #[inline(always)]
        pub const fn dpi_buff_pld_empty(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the empty status of the payload internal buffer for video Mode. This bit is set to 0 for command Mod."]
        #[inline(always)]
        pub fn set_dpi_buff_pld_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "This bit indicates the full status of the payload internal buffer for video Mode. This bit is set to 0 for command Mode."]
        #[inline(always)]
        pub const fn dpi_buff_pld_full(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "This bit indicates the full status of the payload internal buffer for video Mode. This bit is set to 0 for command Mode."]
        #[inline(always)]
        pub fn set_dpi_buff_pld_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for VidPktStatus {
        #[inline(always)]
        fn default() -> VidPktStatus {
            VidPktStatus(0)
        }
    }
    #[doc = "controls dpi shadow feature."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidShadowCtrl(pub u32);
    impl VidShadowCtrl {
        #[doc = "when set to 1, DPI receives the active configuration from the auxiliary register."]
        #[inline(always)]
        pub const fn vid_shadow_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "when set to 1, DPI receives the active configuration from the auxiliary register."]
        #[inline(always)]
        pub fn set_vid_shadow_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "when set to 1, request that the dpi register from regbank are copied to the auxiliary registers."]
        #[inline(always)]
        pub const fn vid_shadow_req(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "when set to 1, request that the dpi register from regbank are copied to the auxiliary registers."]
        #[inline(always)]
        pub fn set_vid_shadow_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "when set to 1, the video request is done by external pin."]
        #[inline(always)]
        pub const fn vid_shadow_pin_req(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "when set to 1, the video request is done by external pin."]
        #[inline(always)]
        pub fn set_vid_shadow_pin_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for VidShadowCtrl {
        #[inline(always)]
        fn default() -> VidShadowCtrl {
            VidShadowCtrl(0)
        }
    }
    #[doc = "configures the vertical resolution of video."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidVactiveLines(pub u32);
    impl VidVactiveLines {
        #[doc = "configures the vertical active period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn v_active_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "configures the vertical active period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_v_active_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for VidVactiveLines {
        #[inline(always)]
        fn default() -> VidVactiveLines {
            VidVactiveLines(0)
        }
    }
    #[doc = "value for vid_vactive_lines."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidVactiveLinesAct(pub u32);
    impl VidVactiveLinesAct {
        #[doc = "vertical active period."]
        #[inline(always)]
        pub const fn v_active_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "vertical active period."]
        #[inline(always)]
        pub fn set_v_active_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for VidVactiveLinesAct {
        #[inline(always)]
        fn default() -> VidVactiveLinesAct {
            VidVactiveLinesAct(0)
        }
    }
    #[doc = "configures the vbp period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidVbpLines(pub u32);
    impl VidVbpLines {
        #[doc = "configures the vertical back porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vbp_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "configures the vertical back porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_vbp_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for VidVbpLines {
        #[inline(always)]
        fn default() -> VidVbpLines {
            VidVbpLines(0)
        }
    }
    #[doc = "value for vid_vbp_lines."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidVbpLinesAct(pub u32);
    impl VidVbpLinesAct {
        #[doc = "vertical back porch period."]
        #[inline(always)]
        pub const fn vbp_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "vertical back porch period."]
        #[inline(always)]
        pub fn set_vbp_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for VidVbpLinesAct {
        #[inline(always)]
        fn default() -> VidVbpLinesAct {
            VidVbpLinesAct(0)
        }
    }
    #[doc = "configures the vfp period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidVfpLines(pub u32);
    impl VidVfpLines {
        #[doc = "configures the vertical front porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vfp_linies(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "configures the vertical front porch period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_vfp_linies(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for VidVfpLines {
        #[inline(always)]
        fn default() -> VidVfpLines {
            VidVfpLines(0)
        }
    }
    #[doc = "value for vid_vfp_lines."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidVfpLinesAct(pub u32);
    impl VidVfpLinesAct {
        #[doc = "vertical porch period."]
        #[inline(always)]
        pub const fn vfp_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "vertical porch period."]
        #[inline(always)]
        pub fn set_vfp_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for VidVfpLinesAct {
        #[inline(always)]
        fn default() -> VidVfpLinesAct {
            VidVfpLinesAct(0)
        }
    }
    #[doc = "configures the vsa period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidVsaLines(pub u32);
    impl VidVsaLines {
        #[doc = "configures the verical synchronism active period measured in number of horizontal lines."]
        #[inline(always)]
        pub const fn vsa_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "configures the verical synchronism active period measured in number of horizontal lines."]
        #[inline(always)]
        pub fn set_vsa_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for VidVsaLines {
        #[inline(always)]
        fn default() -> VidVsaLines {
            VidVsaLines(0)
        }
    }
    #[doc = "value for vid_vsa_lines."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VidVsaLinesAct(pub u32);
    impl VidVsaLinesAct {
        #[doc = "vertical synchronism active period."]
        #[inline(always)]
        pub const fn vsa_lines(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "vertical synchronism active period."]
        #[inline(always)]
        pub fn set_vsa_lines(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for VidVsaLinesAct {
        #[inline(always)]
        fn default() -> VidVsaLinesAct {
            VidVsaLinesAct(0)
        }
    }
}
