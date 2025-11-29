#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ESC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esc {
    ptr: *mut u8,
}
unsafe impl Send for Esc {}
unsafe impl Sync for Esc {}
impl Esc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Type of EtherCAT controller."]
    #[inline(always)]
    pub const fn type_(self) -> crate::common::Reg<regs::EscType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Revision of EtherCAT controller."]
    #[inline(always)]
    pub const fn revision(self) -> crate::common::Reg<regs::Revision, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "Build of EtherCAT controller."]
    #[inline(always)]
    pub const fn build(self) -> crate::common::Reg<regs::Build, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "FMMU supported."]
    #[inline(always)]
    pub const fn fmmu_num(self) -> crate::common::Reg<regs::FmmuNum, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SyncManagers supported."]
    #[inline(always)]
    pub const fn syncm_num(self) -> crate::common::Reg<regs::SyncmNum, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "RAM Size."]
    #[inline(always)]
    pub const fn ram_size(self) -> crate::common::Reg<regs::RamSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Port Descriptor."]
    #[inline(always)]
    pub const fn port_desc(self) -> crate::common::Reg<regs::PortDesc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "ESC Feature supported."]
    #[inline(always)]
    pub const fn feature(self) -> crate::common::Reg<regs::Feature, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Configured Station Address."]
    #[inline(always)]
    pub const fn station_addr(self) -> crate::common::Reg<regs::StationAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Configured Station Alias."]
    #[inline(always)]
    pub const fn station_als(self) -> crate::common::Reg<regs::StationAls, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Register Write Enable."]
    #[inline(always)]
    pub const fn reg_wen(self) -> crate::common::Reg<regs::RegWen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Register Write Protection."]
    #[inline(always)]
    pub const fn reg_wp(self) -> crate::common::Reg<regs::RegWp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21usize) as _) }
    }
    #[doc = "ESC Write Enable."]
    #[inline(always)]
    pub const fn esc_wen(self) -> crate::common::Reg<regs::EscWen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "ESC Write Protection."]
    #[inline(always)]
    pub const fn esc_wp(self) -> crate::common::Reg<regs::EscWp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x31usize) as _) }
    }
    #[doc = "ESC Reset ECAT."]
    #[inline(always)]
    pub const fn esc_rst_ecat(self) -> crate::common::Reg<regs::EscRstEcat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "ESC Reset PDI."]
    #[inline(always)]
    pub const fn esc_rst_pdi(self) -> crate::common::Reg<regs::EscRstPdi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x41usize) as _) }
    }
    #[doc = "ESC DL Control."]
    #[inline(always)]
    pub const fn esc_dl_ctrl(self) -> crate::common::Reg<regs::EscDlCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Physical Read/Write Offset."]
    #[inline(always)]
    pub const fn physical_rw_offset(
        self,
    ) -> crate::common::Reg<regs::PhysicalRwOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "ESC DL Status."]
    #[inline(always)]
    pub const fn esc_dl_stat(self) -> crate::common::Reg<regs::EscDlStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "AL Control."]
    #[inline(always)]
    pub const fn al_ctrl(self) -> crate::common::Reg<regs::AlCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "AL Status."]
    #[inline(always)]
    pub const fn al_stat(self) -> crate::common::Reg<regs::AlStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "AL Status Code."]
    #[inline(always)]
    pub const fn al_stat_code(self) -> crate::common::Reg<regs::AlStatCode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "RUN LED Override."]
    #[inline(always)]
    pub const fn run_led_ovrd(self) -> crate::common::Reg<regs::RunLedOvrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "ERR LED Override."]
    #[inline(always)]
    pub const fn err_led_ovrd(self) -> crate::common::Reg<regs::ErrLedOvrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0139usize) as _) }
    }
    #[doc = "PDI Control."]
    #[inline(always)]
    pub const fn pdi_ctrl(self) -> crate::common::Reg<regs::EscPdiCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "ESC Configuration."]
    #[inline(always)]
    pub const fn esc_cfg(self) -> crate::common::Reg<regs::EscCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0141usize) as _) }
    }
    #[doc = "PDI Information."]
    #[inline(always)]
    pub const fn pdi_info(self) -> crate::common::Reg<regs::PdiInfo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014eusize) as _) }
    }
    #[doc = "PDI Configuration."]
    #[inline(always)]
    pub const fn pdi_cfg(self) -> crate::common::Reg<regs::PdiCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "PDI Sync/Latch\\[1:0\\]
Configuration."]
    #[inline(always)]
    pub const fn pdi_sl_cfg(self) -> crate::common::Reg<regs::PdiSlCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0151usize) as _) }
    }
    #[doc = "PDI Extended Configuration."]
    #[inline(always)]
    pub const fn pdi_ext_cfg(self) -> crate::common::Reg<regs::PdiExtCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0152usize) as _) }
    }
    #[doc = "ECAT Event Mask."]
    #[inline(always)]
    pub const fn ecat_evt_msk(self) -> crate::common::Reg<regs::EcatEvtMsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "PDI AL Event Mask."]
    #[inline(always)]
    pub const fn pdi_al_evt_msk(self) -> crate::common::Reg<regs::PdiAlEvtMsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "ECAT Event Request."]
    #[inline(always)]
    pub const fn ecat_evt_req(self) -> crate::common::Reg<regs::EcatEvtReq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "AL Event Request."]
    #[inline(always)]
    pub const fn al_evt_req(self) -> crate::common::Reg<regs::AlEvtReq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn rx_err_cnt(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RxErrCnt, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 2usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn fwd_rx_err_cnt(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FwdRxErrCnt, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize + n * 1usize) as _)
        }
    }
    #[doc = "ECAT Processing Unit Error Counter."]
    #[inline(always)]
    pub const fn ecat_pu_err_cnt(
        self,
    ) -> crate::common::Reg<regs::EcatPuErrCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "PDI Error Counter."]
    #[inline(always)]
    pub const fn pdi_err_cnt(self) -> crate::common::Reg<regs::PdiErrCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030dusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn lost_link_cnt(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::LostLinkCnt, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize + n * 1usize) as _)
        }
    }
    #[doc = "Watchdog Divider."]
    #[inline(always)]
    pub const fn wdg_div(self) -> crate::common::Reg<regs::WdgDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Watchdog Time PDI."]
    #[inline(always)]
    pub const fn wdg_time_pdi(self) -> crate::common::Reg<regs::WdgTimePdi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "Watchdog Time Process Data."]
    #[inline(always)]
    pub const fn wdg_time_pdat(self) -> crate::common::Reg<regs::WdgTimePdat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "Watchdog Status Process Data."]
    #[inline(always)]
    pub const fn wdg_stat_pdat(self) -> crate::common::Reg<regs::WdgStatPdat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "Watchdog Counter Process Data."]
    #[inline(always)]
    pub const fn wdg_cnt_pdat(self) -> crate::common::Reg<regs::WdgCntPdat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0442usize) as _) }
    }
    #[doc = "Watchdog Counter PDI."]
    #[inline(always)]
    pub const fn wdg_cnt_pdi(self) -> crate::common::Reg<regs::WdgCntPdi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0443usize) as _) }
    }
    #[doc = "EEPROM Configuration."]
    #[inline(always)]
    pub const fn eeprom_cfg(self) -> crate::common::Reg<regs::EepromCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "EEPROM PDI Access State."]
    #[inline(always)]
    pub const fn eeprom_pdi_acc_stat(
        self,
    ) -> crate::common::Reg<regs::EepromPdiAccStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0501usize) as _) }
    }
    #[doc = "EEPROM Control/Status."]
    #[inline(always)]
    pub const fn eeprom_ctrl_stat(
        self,
    ) -> crate::common::Reg<regs::EepromCtrlStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0502usize) as _) }
    }
    #[doc = "EEPROM Address."]
    #[inline(always)]
    pub const fn eeprom_addr(self) -> crate::common::Reg<regs::EepromAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "EEPROM Data."]
    #[inline(always)]
    pub const fn eeprom_data(self) -> crate::common::Reg<regs::EepromData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "MII Management Control/Status."]
    #[inline(always)]
    pub const fn mii_mng_cs(self) -> crate::common::Reg<regs::MiiMngCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "PHY Address."]
    #[inline(always)]
    pub const fn phy_addr(self) -> crate::common::Reg<regs::PhyAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0512usize) as _) }
    }
    #[doc = "PHY Register Address."]
    #[inline(always)]
    pub const fn phy_reg_addr(self) -> crate::common::Reg<regs::PhyRegAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0513usize) as _) }
    }
    #[doc = "PHY Data."]
    #[inline(always)]
    pub const fn phy_data(self) -> crate::common::Reg<regs::PhyData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "MII Management ECAT Access State."]
    #[inline(always)]
    pub const fn miim_ecat_acc_stat(
        self,
    ) -> crate::common::Reg<regs::MiimEcatAccStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0516usize) as _) }
    }
    #[doc = "MII Management PDI Access State."]
    #[inline(always)]
    pub const fn miim_pdi_acc_stat(
        self,
    ) -> crate::common::Reg<regs::MiimPdiAccStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0517usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn phy_stat(self, n: usize) -> crate::common::Reg<regs::PhyStat, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize + n * 1usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn fmmu(self, n: usize) -> Fmmu {
        assert!(n < 8usize);
        unsafe { Fmmu::from_ptr(self.ptr.wrapping_add(0x0600usize + n * 16usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn syncm(self, n: usize) -> Syncm {
        assert!(n < 8usize);
        unsafe { Syncm::from_ptr(self.ptr.wrapping_add(0x0800usize + n * 8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn rcv_time(self, n: usize) -> crate::common::Reg<regs::RcvTime, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize + n * 4usize) as _)
        }
    }
    #[doc = "System Time."]
    #[inline(always)]
    pub const fn sys_time(self) -> crate::common::Reg<regs::SysTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0910usize) as _) }
    }
    #[doc = "Receive Time ECAT Processing Unit."]
    #[inline(always)]
    pub const fn rcvt_ecat_pu(self) -> crate::common::Reg<regs::RcvtEcatPu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0918usize) as _) }
    }
    #[doc = "System Time Offset."]
    #[inline(always)]
    pub const fn sys_time_offset(
        self,
    ) -> crate::common::Reg<regs::SysTimeOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0920usize) as _) }
    }
    #[doc = "System Time Delay."]
    #[inline(always)]
    pub const fn sys_time_delay(self) -> crate::common::Reg<regs::SysTimeDelay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0928usize) as _) }
    }
    #[doc = "System Time Difference."]
    #[inline(always)]
    pub const fn sys_time_diff(self) -> crate::common::Reg<regs::SysTimeDiff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x092cusize) as _) }
    }
    #[doc = "Speed Counter Start."]
    #[inline(always)]
    pub const fn spd_cnt_start(self) -> crate::common::Reg<regs::SpdCntStart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0930usize) as _) }
    }
    #[doc = "Speed Counter Diff."]
    #[inline(always)]
    pub const fn spd_cnt_diff(self) -> crate::common::Reg<regs::SpdCntDiff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0932usize) as _) }
    }
    #[doc = "System Time Difference Filter Depth."]
    #[inline(always)]
    pub const fn sys_time_diff_fd(
        self,
    ) -> crate::common::Reg<regs::SysTimeDiffFd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0934usize) as _) }
    }
    #[doc = "Speed Counter Filter Depth."]
    #[inline(always)]
    pub const fn spd_cnt_fd(self) -> crate::common::Reg<regs::SpdCntFd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0935usize) as _) }
    }
    #[doc = "Receive Time Latch Mode."]
    #[inline(always)]
    pub const fn rcv_time_lm(self) -> crate::common::Reg<regs::RcvTimeLm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0936usize) as _) }
    }
    #[doc = "Cyclic Unit Control."]
    #[inline(always)]
    pub const fn cyc_unit_ctrl(self) -> crate::common::Reg<regs::CycUnitCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0980usize) as _) }
    }
    #[doc = "SYNC Out Unit Activation."]
    #[inline(always)]
    pub const fn synco_act(self) -> crate::common::Reg<regs::SyncoAct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0981usize) as _) }
    }
    #[doc = "Pulse Length of SyncSignals."]
    #[inline(always)]
    pub const fn pulse_len(self) -> crate::common::Reg<regs::PulseLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0982usize) as _) }
    }
    #[doc = "Activation Status."]
    #[inline(always)]
    pub const fn act_stat(self) -> crate::common::Reg<regs::ActStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0984usize) as _) }
    }
    #[doc = "SYNC0 Status."]
    #[inline(always)]
    pub const fn sync0_stat(self) -> crate::common::Reg<regs::Sync0Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x098eusize) as _) }
    }
    #[doc = "SYNC1 Status."]
    #[inline(always)]
    pub const fn sync1_stat(self) -> crate::common::Reg<regs::Sync1Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x098fusize) as _) }
    }
    #[doc = "Start Time Cyclic Operation."]
    #[inline(always)]
    pub const fn start_time_co(self) -> crate::common::Reg<regs::StartTimeCo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0990usize) as _) }
    }
    #[doc = "Next SYNC1 Pulse."]
    #[inline(always)]
    pub const fn nxt_sync1_pulse(
        self,
    ) -> crate::common::Reg<regs::NxtSync1Pulse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0998usize) as _) }
    }
    #[doc = "SYNC0 Cycle Time."]
    #[inline(always)]
    pub const fn sync0_cyc_time(self) -> crate::common::Reg<regs::Sync0CycTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09a0usize) as _) }
    }
    #[doc = "SYNC1 Cycle Time."]
    #[inline(always)]
    pub const fn sync1_cyc_time(self) -> crate::common::Reg<regs::Sync1CycTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09a4usize) as _) }
    }
    #[doc = "Latch0 Control."]
    #[inline(always)]
    pub const fn latch0_ctrl(self) -> crate::common::Reg<regs::Latch0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09a8usize) as _) }
    }
    #[doc = "Latch1 Control."]
    #[inline(always)]
    pub const fn latch1_ctrl(self) -> crate::common::Reg<regs::Latch1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09a9usize) as _) }
    }
    #[doc = "Latch0 Status."]
    #[inline(always)]
    pub const fn latch0_stat(self) -> crate::common::Reg<regs::Latch0Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09aeusize) as _) }
    }
    #[doc = "Latch1 Status."]
    #[inline(always)]
    pub const fn latch1_stat(self) -> crate::common::Reg<regs::Latch1Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09afusize) as _) }
    }
    #[doc = "Latch0 Time Positive Edge."]
    #[inline(always)]
    pub const fn latch0_time_pe(self) -> crate::common::Reg<regs::Latch0TimePe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09b0usize) as _) }
    }
    #[doc = "Latch0 Time Negative Edge."]
    #[inline(always)]
    pub const fn latch0_time_ne(self) -> crate::common::Reg<regs::Latch0TimeNe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09b8usize) as _) }
    }
    #[doc = "Latch1 Time Positive Edge."]
    #[inline(always)]
    pub const fn latch1_time_pe(self) -> crate::common::Reg<regs::Latch1TimePe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09c0usize) as _) }
    }
    #[doc = "Latch1 Time Negative Edge."]
    #[inline(always)]
    pub const fn latch1_time_ne(self) -> crate::common::Reg<regs::Latch1TimeNe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09c8usize) as _) }
    }
    #[doc = "EtherCAT Buffer Change Event Time."]
    #[inline(always)]
    pub const fn ecat_buf_cet(self) -> crate::common::Reg<regs::EcatBufCet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f0usize) as _) }
    }
    #[doc = "PDI Buffer Start Event Time."]
    #[inline(always)]
    pub const fn pdi_buf_set(self) -> crate::common::Reg<regs::PdiBufSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f8usize) as _) }
    }
    #[doc = "PDI Buffer Change Event Time."]
    #[inline(always)]
    pub const fn pdi_buf_cet(self) -> crate::common::Reg<regs::PdiBufCet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09fcusize) as _) }
    }
    #[doc = "Product ID."]
    #[inline(always)]
    pub const fn pid(self) -> crate::common::Reg<regs::Pid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize) as _) }
    }
    #[doc = "Vendor ID."]
    #[inline(always)]
    pub const fn vid(self) -> crate::common::Reg<regs::Vid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e08usize) as _) }
    }
    #[doc = "Digital I/O Output Data."]
    #[inline(always)]
    pub const fn dio_out_data(self) -> crate::common::Reg<regs::DioOutData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize) as _) }
    }
    #[doc = "General Purpose Outputs."]
    #[inline(always)]
    pub const fn gpo(self) -> crate::common::Reg<regs::Gpo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f10usize) as _) }
    }
    #[doc = "General Purpose Inputs."]
    #[inline(always)]
    pub const fn gpi(self) -> crate::common::Reg<regs::Gpi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f18usize) as _) }
    }
    #[doc = "User Ram Byte 0."]
    #[inline(always)]
    pub const fn user_ram_byte0(self) -> crate::common::Reg<regs::UserRamByte0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f80usize) as _) }
    }
    #[doc = "User Ram Byte 1."]
    #[inline(always)]
    pub const fn user_ram_byte1(self) -> crate::common::Reg<regs::UserRamByte1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f81usize) as _) }
    }
    #[doc = "User Ram Byte 2."]
    #[inline(always)]
    pub const fn user_ram_byte2(self) -> crate::common::Reg<regs::UserRamByte2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f82usize) as _) }
    }
    #[doc = "User Ram Byte 3."]
    #[inline(always)]
    pub const fn user_ram_byte3(self) -> crate::common::Reg<regs::UserRamByte3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f83usize) as _) }
    }
    #[doc = "User Ram Byte 4."]
    #[inline(always)]
    pub const fn user_ram_byte4(self) -> crate::common::Reg<regs::UserRamByte4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f84usize) as _) }
    }
    #[doc = "User Ram Byte 5."]
    #[inline(always)]
    pub const fn user_ram_byte5(self) -> crate::common::Reg<regs::UserRamByte5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f85usize) as _) }
    }
    #[doc = "User Ram Byte 6."]
    #[inline(always)]
    pub const fn user_ram_byte6(self) -> crate::common::Reg<regs::UserRamByte6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f86usize) as _) }
    }
    #[doc = "User Ram Byte 7."]
    #[inline(always)]
    pub const fn user_ram_byte7(self) -> crate::common::Reg<regs::UserRamByte7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f87usize) as _) }
    }
    #[doc = "User Ram Byte 8."]
    #[inline(always)]
    pub const fn user_ram_byte8(self) -> crate::common::Reg<regs::UserRamByte8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f88usize) as _) }
    }
    #[doc = "User Ram Byte 9."]
    #[inline(always)]
    pub const fn user_ram_byte9(self) -> crate::common::Reg<regs::UserRamByte9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f89usize) as _) }
    }
    #[doc = "User Ram Byte 10."]
    #[inline(always)]
    pub const fn user_ram_byte10(
        self,
    ) -> crate::common::Reg<regs::UserRamByte10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f8ausize) as _) }
    }
    #[doc = "User Ram Byte 11."]
    #[inline(always)]
    pub const fn user_ram_byte11(
        self,
    ) -> crate::common::Reg<regs::UserRamByte11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f8busize) as _) }
    }
    #[doc = "User Ram Byte 14."]
    #[inline(always)]
    pub const fn user_ram_byte14(
        self,
    ) -> crate::common::Reg<regs::UserRamByte14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f8eusize) as _) }
    }
    #[doc = "User Ram Byte 15."]
    #[inline(always)]
    pub const fn user_ram_byte15(
        self,
    ) -> crate::common::Reg<regs::UserRamByte15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f8fusize) as _) }
    }
    #[doc = "User Ram Byte 19."]
    #[inline(always)]
    pub const fn user_ram_byte19(
        self,
    ) -> crate::common::Reg<regs::UserRamByte19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f93usize) as _) }
    }
    #[doc = "Process Data Ram."]
    #[inline(always)]
    pub const fn pdram(self) -> crate::common::Reg<regs::Pdram, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "Process Data Ram Alias."]
    #[inline(always)]
    pub const fn pdram_als(self) -> crate::common::Reg<regs::PdramAls, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0000usize) as _) }
    }
    #[doc = "General Purpose Configure 0."]
    #[inline(always)]
    pub const fn gpr_cfg0(self) -> crate::common::Reg<regs::GprCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f000usize) as _) }
    }
    #[doc = "General Purpose Configure 1."]
    #[inline(always)]
    pub const fn gpr_cfg1(self) -> crate::common::Reg<regs::GprCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f004usize) as _) }
    }
    #[doc = "General Purpose Configure 2."]
    #[inline(always)]
    pub const fn gpr_cfg2(self) -> crate::common::Reg<regs::GprCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f008usize) as _) }
    }
    #[doc = "PHY Configure 0."]
    #[inline(always)]
    pub const fn phy_cfg0(self) -> crate::common::Reg<regs::PhyCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f010usize) as _) }
    }
    #[doc = "PHY Configure 1."]
    #[inline(always)]
    pub const fn phy_cfg1(self) -> crate::common::Reg<regs::PhyCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f014usize) as _) }
    }
    #[doc = "GPIO Output Enable."]
    #[inline(always)]
    pub const fn gpio_ctrl(self) -> crate::common::Reg<regs::GpioCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f020usize) as _) }
    }
    #[doc = "GPI low word Override value."]
    #[inline(always)]
    pub const fn gpi_override0(self) -> crate::common::Reg<regs::GpiOverride0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f030usize) as _) }
    }
    #[doc = "GPI high word Override value."]
    #[inline(always)]
    pub const fn gpi_override1(self) -> crate::common::Reg<regs::GpiOverride1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f034usize) as _) }
    }
    #[doc = "GPO low word read value."]
    #[inline(always)]
    pub const fn gpo_reg0(self) -> crate::common::Reg<regs::GpoReg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f038usize) as _) }
    }
    #[doc = "GPO high word read value."]
    #[inline(always)]
    pub const fn gpo_reg1(self) -> crate::common::Reg<regs::GpoReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f03cusize) as _) }
    }
    #[doc = "GPI low word read value."]
    #[inline(always)]
    pub const fn gpi_reg0(self) -> crate::common::Reg<regs::GpiReg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f040usize) as _) }
    }
    #[doc = "GPI high word read value."]
    #[inline(always)]
    pub const fn gpi_reg1(self) -> crate::common::Reg<regs::GpiReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f044usize) as _) }
    }
    #[doc = "global status register."]
    #[inline(always)]
    pub const fn gpr_status(self) -> crate::common::Reg<regs::GprStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f060usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn io_cfg(self, n: usize) -> crate::common::Reg<regs::IoCfg, crate::common::RW> {
        assert!(n < 9usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_f080usize + n * 4usize) as _)
        }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmmu {
    ptr: *mut u8,
}
unsafe impl Send for Fmmu {}
unsafe impl Sync for Fmmu {}
impl Fmmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Logical Start Address."]
    #[inline(always)]
    pub const fn logic_start_addr(
        self,
    ) -> crate::common::Reg<regs::LogicStartAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Length."]
    #[inline(always)]
    pub const fn length(self) -> crate::common::Reg<regs::FmmuLength, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Logical Start Bit."]
    #[inline(always)]
    pub const fn logic_start_bit(
        self,
    ) -> crate::common::Reg<regs::LogicStartBit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Logical Stop Bit."]
    #[inline(always)]
    pub const fn logic_stop_bit(self) -> crate::common::Reg<regs::LogicStopBit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "Physical Start Address."]
    #[inline(always)]
    pub const fn physical_start_addr(
        self,
    ) -> crate::common::Reg<regs::FmmuPhysicalStartAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Physical Start Bit."]
    #[inline(always)]
    pub const fn physical_start_bit(
        self,
    ) -> crate::common::Reg<regs::PhysicalStartBit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Type."]
    #[inline(always)]
    pub const fn type_(self) -> crate::common::Reg<regs::FmmuType, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "Activate."]
    #[inline(always)]
    pub const fn activate(self) -> crate::common::Reg<regs::FmmuActivate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syncm {
    ptr: *mut u8,
}
unsafe impl Send for Syncm {}
unsafe impl Sync for Syncm {}
impl Syncm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Physical Start Address."]
    #[inline(always)]
    pub const fn physical_start_addr(
        self,
    ) -> crate::common::Reg<regs::SyncmPhysicalStartAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Length."]
    #[inline(always)]
    pub const fn length(self) -> crate::common::Reg<regs::SyncmLength, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "Activate."]
    #[inline(always)]
    pub const fn activate(self) -> crate::common::Reg<regs::SyncmActivate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "PDI Control."]
    #[inline(always)]
    pub const fn pdi_ctrl(self) -> crate::common::Reg<regs::SyncmPdiCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
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
    #[doc = "Activation Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ActStat(pub u8);
    impl ActStat {
        #[doc = "SYNC0 activation state: 0:First SYNC0 pulse is not pending 1:First SYNC0 pulse is pending."]
        #[must_use]
        #[inline(always)]
        pub const fn sync0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC0 activation state: 0:First SYNC0 pulse is not pending 1:First SYNC0 pulse is pending."]
        #[inline(always)]
        pub const fn set_sync0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "SYNC1 activation state: 0:First SYNC1 pulse is not pending 1:First SYNC1 pulse is pending."]
        #[must_use]
        #[inline(always)]
        pub const fn sync1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC1 activation state: 0:First SYNC1 pulse is not pending 1:First SYNC1 pulse is pending."]
        #[inline(always)]
        pub const fn set_sync1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Start Time Cyclic Operation (0x0990:0x0997) plausibility check result when Sync Out Unit was activated: 0:Start Time was within near future 1:Start Time was out of near future (0x0981\\[6\\])."]
        #[must_use]
        #[inline(always)]
        pub const fn chk_rslt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Start Time Cyclic Operation (0x0990:0x0997) plausibility check result when Sync Out Unit was activated: 0:Start Time was within near future 1:Start Time was out of near future (0x0981\\[6\\])."]
        #[inline(always)]
        pub const fn set_chk_rslt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for ActStat {
        #[inline(always)]
        fn default() -> ActStat {
            ActStat(0)
        }
    }
    impl core::fmt::Debug for ActStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ActStat")
                .field("sync0", &self.sync0())
                .field("sync1", &self.sync1())
                .field("chk_rslt", &self.chk_rslt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ActStat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ActStat {{ sync0: {=bool:?}, sync1: {=bool:?}, chk_rslt: {=bool:?} }}",
                self.sync0(),
                self.sync1(),
                self.chk_rslt()
            )
        }
    }
    #[doc = "AL Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlCtrl(pub u16);
    impl AlCtrl {
        #[doc = "Initiate State Transition of the Device State Machine: 1:Request Init State 3:Request Bootstrap State 2:Request Pre-Operational State 4:Request Safe-Operational State 8:Request Operational State."]
        #[must_use]
        #[inline(always)]
        pub const fn ist(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Initiate State Transition of the Device State Machine: 1:Request Init State 3:Request Bootstrap State 2:Request Pre-Operational State 4:Request Safe-Operational State 8:Request Operational State."]
        #[inline(always)]
        pub const fn set_ist(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[doc = "Error Ind Ack: 0:No Ack of Error Ind in AL status register 1:Ack of Error Ind in AL status register."]
        #[must_use]
        #[inline(always)]
        pub const fn eia(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Error Ind Ack: 0:No Ack of Error Ind in AL status register 1:Ack of Error Ind in AL status register."]
        #[inline(always)]
        pub const fn set_eia(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Device Identification: 0:No request 1:Device Identification request."]
        #[must_use]
        #[inline(always)]
        pub const fn di(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Device Identification: 0:No request 1:Device Identification request."]
        #[inline(always)]
        pub const fn set_di(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
    }
    impl Default for AlCtrl {
        #[inline(always)]
        fn default() -> AlCtrl {
            AlCtrl(0)
        }
    }
    impl core::fmt::Debug for AlCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AlCtrl")
                .field("ist", &self.ist())
                .field("eia", &self.eia())
                .field("di", &self.di())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AlCtrl {{ ist: {=u8:?}, eia: {=bool:?}, di: {=bool:?} }}",
                self.ist(),
                self.eia(),
                self.di()
            )
        }
    }
    #[doc = "AL Event Request."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlEvtReq(pub u32);
    impl AlEvtReq {
        #[doc = "AL Control event: 0:No AL Control Register change 1:AL Control Register has been written3 (Bit is cleared by reading AL Control register 0x0120:0x0121 from PDI)."]
        #[must_use]
        #[inline(always)]
        pub const fn alc_evt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AL Control event: 0:No AL Control Register change 1:AL Control Register has been written3 (Bit is cleared by reading AL Control register 0x0120:0x0121 from PDI)."]
        #[inline(always)]
        pub const fn set_alc_evt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DC Latch event: 0:No change on DC Latch Inputs 1:At least one change on DC Latch Inputs (Bit is cleared by reading DC Latch event times from PDI, so that Latch 0/1 Status 0x09AE:0x09AF indicates no event. Available if Latch Unit is PDI-controlled)."]
        #[must_use]
        #[inline(always)]
        pub const fn dcl_evt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DC Latch event: 0:No change on DC Latch Inputs 1:At least one change on DC Latch Inputs (Bit is cleared by reading DC Latch event times from PDI, so that Latch 0/1 Status 0x09AE:0x09AF indicates no event. Available if Latch Unit is PDI-controlled)."]
        #[inline(always)]
        pub const fn set_dcl_evt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "State of DC SYNC0 (if register 0x0151\\[3\\]=1): (Bit is cleared by reading SYNC0 status 0x098E from PDI, use only in Acknowledge mode)."]
        #[must_use]
        #[inline(always)]
        pub const fn st_dc_sync0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "State of DC SYNC0 (if register 0x0151\\[3\\]=1): (Bit is cleared by reading SYNC0 status 0x098E from PDI, use only in Acknowledge mode)."]
        #[inline(always)]
        pub const fn set_st_dc_sync0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "State of DC SYNC1 (if register 0x0151\\[7\\]=1): (Bit is cleared by reading of SYNC1 status 0x098F from PDI, use only in Acknowledge mode)."]
        #[must_use]
        #[inline(always)]
        pub const fn st_dc_sync1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "State of DC SYNC1 (if register 0x0151\\[7\\]=1): (Bit is cleared by reading of SYNC1 status 0x098F from PDI, use only in Acknowledge mode)."]
        #[inline(always)]
        pub const fn set_st_dc_sync1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SyncManager activation register (SyncManager register offset 0x6) changed: 0:No change in any SyncManager 1:At least one SyncManager changed (Bit is cleared by reading SyncManager Activation registers 0x0806 etc. from PDI)."]
        #[must_use]
        #[inline(always)]
        pub const fn sm_act(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SyncManager activation register (SyncManager register offset 0x6) changed: 0:No change in any SyncManager 1:At least one SyncManager changed (Bit is cleared by reading SyncManager Activation registers 0x0806 etc. from PDI)."]
        #[inline(always)]
        pub const fn set_sm_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "EEPROM Emulation: 0:No command pending 1:EEPROM command pending (Bit is cleared by acknowledging the command in EEPROM Control/Status register 0x0502:0x0503\\[10:8\\]
from PDI)."]
        #[must_use]
        #[inline(always)]
        pub const fn ee_emu(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EEPROM Emulation: 0:No command pending 1:EEPROM command pending (Bit is cleared by acknowledging the command in EEPROM Control/Status register 0x0502:0x0503\\[10:8\\]
from PDI)."]
        #[inline(always)]
        pub const fn set_ee_emu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Watchdog Process Data: 0:Has not expired 1:Has expired (Bit is cleared by reading Watchdog Status Process Data 0x0440 from PDI)."]
        #[must_use]
        #[inline(always)]
        pub const fn wdg_pd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Process Data: 0:Has not expired 1:Has expired (Bit is cleared by reading Watchdog Status Process Data 0x0440 from PDI)."]
        #[inline(always)]
        pub const fn set_wdg_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SyncManager interrupts (SyncManager register offset 0x5, bit \\[0\\]
or \\[1\\]): 0:No SyncManager 0 interrupt 1:SyncManager 0 interrupt pending 0:No SyncManager 1 interrupt 1:SyncManager 1 interrupt pending … 0:No SyncManager 15 interrupt 1:SyncManager 15 interrupt pending."]
        #[must_use]
        #[inline(always)]
        pub const fn sm_int(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0xffff;
            val as u16
        }
        #[doc = "SyncManager interrupts (SyncManager register offset 0x5, bit \\[0\\]
or \\[1\\]): 0:No SyncManager 0 interrupt 1:SyncManager 0 interrupt pending 0:No SyncManager 1 interrupt 1:SyncManager 1 interrupt pending … 0:No SyncManager 15 interrupt 1:SyncManager 15 interrupt pending."]
        #[inline(always)]
        pub const fn set_sm_int(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
        }
    }
    impl Default for AlEvtReq {
        #[inline(always)]
        fn default() -> AlEvtReq {
            AlEvtReq(0)
        }
    }
    impl core::fmt::Debug for AlEvtReq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AlEvtReq")
                .field("alc_evt", &self.alc_evt())
                .field("dcl_evt", &self.dcl_evt())
                .field("st_dc_sync0", &self.st_dc_sync0())
                .field("st_dc_sync1", &self.st_dc_sync1())
                .field("sm_act", &self.sm_act())
                .field("ee_emu", &self.ee_emu())
                .field("wdg_pd", &self.wdg_pd())
                .field("sm_int", &self.sm_int())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlEvtReq {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AlEvtReq {{ alc_evt: {=bool:?}, dcl_evt: {=bool:?}, st_dc_sync0: {=bool:?}, st_dc_sync1: {=bool:?}, sm_act: {=bool:?}, ee_emu: {=bool:?}, wdg_pd: {=bool:?}, sm_int: {=u16:?} }}" , self . alc_evt () , self . dcl_evt () , self . st_dc_sync0 () , self . st_dc_sync1 () , self . sm_act () , self . ee_emu () , self . wdg_pd () , self . sm_int ())
        }
    }
    #[doc = "AL Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlStat(pub u16);
    impl AlStat {
        #[doc = "Actual State of the Device State Machine: 1:Init State 3:Bootstrap State 2:Pre-Operational State 4:Safe-Operational State 8:Operational State."]
        #[must_use]
        #[inline(always)]
        pub const fn as_(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Actual State of the Device State Machine: 1:Init State 3:Bootstrap State 2:Pre-Operational State 4:Safe-Operational State 8:Operational State."]
        #[inline(always)]
        pub const fn set_as_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[doc = "Error Ind: 0:Device is in State as requested or Flag cleared by command 1:Device has not entered requested State or changed State as result of a local action."]
        #[must_use]
        #[inline(always)]
        pub const fn ei(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Error Ind: 0:Device is in State as requested or Flag cleared by command 1:Device has not entered requested State or changed State as result of a local action."]
        #[inline(always)]
        pub const fn set_ei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Device Identification: 0:Device Identification not valid 1:Device Identification loaded."]
        #[must_use]
        #[inline(always)]
        pub const fn di(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Device Identification: 0:Device Identification not valid 1:Device Identification loaded."]
        #[inline(always)]
        pub const fn set_di(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
    }
    impl Default for AlStat {
        #[inline(always)]
        fn default() -> AlStat {
            AlStat(0)
        }
    }
    impl core::fmt::Debug for AlStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AlStat")
                .field("as_", &self.as_())
                .field("ei", &self.ei())
                .field("di", &self.di())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlStat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AlStat {{ as_: {=u8:?}, ei: {=bool:?}, di: {=bool:?} }}",
                self.as_(),
                self.ei(),
                self.di()
            )
        }
    }
    #[doc = "AL Status Code."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AlStatCode(pub u16);
    impl AlStatCode {
        #[doc = "AL Status Code."]
        #[must_use]
        #[inline(always)]
        pub const fn code(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "AL Status Code."]
        #[inline(always)]
        pub const fn set_code(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for AlStatCode {
        #[inline(always)]
        fn default() -> AlStatCode {
            AlStatCode(0)
        }
    }
    impl core::fmt::Debug for AlStatCode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AlStatCode")
                .field("code", &self.code())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AlStatCode {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "AlStatCode {{ code: {=u16:?} }}", self.code())
        }
    }
    #[doc = "Build of EtherCAT controller."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Build(pub u16);
    impl Build {
        #[doc = "maintenance version Z."]
        #[must_use]
        #[inline(always)]
        pub const fn z(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "maintenance version Z."]
        #[inline(always)]
        pub const fn set_z(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[doc = "minor version Y."]
        #[must_use]
        #[inline(always)]
        pub const fn y(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "minor version Y."]
        #[inline(always)]
        pub const fn set_y(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn build(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_build(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for Build {
        #[inline(always)]
        fn default() -> Build {
            Build(0)
        }
    }
    impl core::fmt::Debug for Build {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Build")
                .field("z", &self.z())
                .field("y", &self.y())
                .field("build", &self.build())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Build {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Build {{ z: {=u8:?}, y: {=u8:?}, build: {=u8:?} }}",
                self.z(),
                self.y(),
                self.build()
            )
        }
    }
    #[doc = "Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control(pub u8);
    impl Control {
        #[doc = "Operation Mode: 00:Buffered (3 buffer mode) 01:Reserved 10:Mailbox (Single buffer mode) 11:Reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn op_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Operation Mode: 00:Buffered (3 buffer mode) 01:Reserved 10:Mailbox (Single buffer mode) 11:Reserved."]
        #[inline(always)]
        pub const fn set_op_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "Direction: 00:Read:ECAT read access, PDI write access. 01:Write:ECAT write access, PDI read access. 10:Reserved 11:Reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Direction: 00:Read:ECAT read access, PDI write access. 01:Write:ECAT write access, PDI read access. 10:Reserved 11:Reserved."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "Interrupt in ECAT Event Request Register: 0:Disabled 1:Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn int_ecat(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt in ECAT Event Request Register: 0:Disabled 1:Enabled."]
        #[inline(always)]
        pub const fn set_int_ecat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Interrupt in AL Event Request Register: 0:Disabled 1:Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn int_al(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt in AL Event Request Register: 0:Disabled 1:Enabled."]
        #[inline(always)]
        pub const fn set_int_al(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Watchdog Trigger Enable: 0:Disabled 1:Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn wdg_trg_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Trigger Enable: 0:Disabled 1:Enabled."]
        #[inline(always)]
        pub const fn set_wdg_trg_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Control {
        #[inline(always)]
        fn default() -> Control {
            Control(0)
        }
    }
    impl core::fmt::Debug for Control {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Control")
                .field("op_mode", &self.op_mode())
                .field("dir", &self.dir())
                .field("int_ecat", &self.int_ecat())
                .field("int_al", &self.int_al())
                .field("wdg_trg_en", &self.wdg_trg_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Control {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Control {{ op_mode: {=u8:?}, dir: {=u8:?}, int_ecat: {=bool:?}, int_al: {=bool:?}, wdg_trg_en: {=bool:?} }}" , self . op_mode () , self . dir () , self . int_ecat () , self . int_al () , self . wdg_trg_en ())
        }
    }
    #[doc = "Cyclic Unit Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CycUnitCtrl(pub u8);
    impl CycUnitCtrl {
        #[doc = "Cyclic Unit and SYNC0 out unit control: 0:ECAT-controlled 1:PDI-controlled."]
        #[must_use]
        #[inline(always)]
        pub const fn synco(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cyclic Unit and SYNC0 out unit control: 0:ECAT-controlled 1:PDI-controlled."]
        #[inline(always)]
        pub const fn set_synco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Latch In unit 0: 0:ECAT-controlled 1:PDI-controlled NOTE:Latch interrupt is routed to ECAT/PDI depending on this setting. Always 1 (PDI-controlled) if System Time is PDI controlled."]
        #[must_use]
        #[inline(always)]
        pub const fn latchi0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Latch In unit 0: 0:ECAT-controlled 1:PDI-controlled NOTE:Latch interrupt is routed to ECAT/PDI depending on this setting. Always 1 (PDI-controlled) if System Time is PDI controlled."]
        #[inline(always)]
        pub const fn set_latchi0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Latch In unit 1: 0:ECAT-controlled 1:PDI-controlled NOTE:Latch interrupt is routed to ECAT/PDI depending on this setting."]
        #[must_use]
        #[inline(always)]
        pub const fn latchi1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Latch In unit 1: 0:ECAT-controlled 1:PDI-controlled NOTE:Latch interrupt is routed to ECAT/PDI depending on this setting."]
        #[inline(always)]
        pub const fn set_latchi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for CycUnitCtrl {
        #[inline(always)]
        fn default() -> CycUnitCtrl {
            CycUnitCtrl(0)
        }
    }
    impl core::fmt::Debug for CycUnitCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CycUnitCtrl")
                .field("synco", &self.synco())
                .field("latchi0", &self.latchi0())
                .field("latchi1", &self.latchi1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CycUnitCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CycUnitCtrl {{ synco: {=bool:?}, latchi0: {=bool:?}, latchi1: {=bool:?} }}",
                self.synco(),
                self.latchi0(),
                self.latchi1()
            )
        }
    }
    #[doc = "Digital I/O Output Data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DioOutData(pub u32);
    impl DioOutData {
        #[doc = "Output Data."]
        #[must_use]
        #[inline(always)]
        pub const fn od(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Output Data."]
        #[inline(always)]
        pub const fn set_od(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DioOutData {
        #[inline(always)]
        fn default() -> DioOutData {
            DioOutData(0)
        }
    }
    impl core::fmt::Debug for DioOutData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DioOutData")
                .field("od", &self.od())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DioOutData {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DioOutData {{ od: {=u32:?} }}", self.od())
        }
    }
    #[doc = "EtherCAT Buffer Change Event Time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EcatBufCet(pub u32);
    impl EcatBufCet {
        #[doc = "Local time at the beginning of the frame which causes at least one SyncManager to assert an ECAT event."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Local time at the beginning of the frame which causes at least one SyncManager to assert an ECAT event."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EcatBufCet {
        #[inline(always)]
        fn default() -> EcatBufCet {
            EcatBufCet(0)
        }
    }
    impl core::fmt::Debug for EcatBufCet {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EcatBufCet")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EcatBufCet {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EcatBufCet {{ time: {=u32:?} }}", self.time())
        }
    }
    #[doc = "ECAT Event Mask."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EcatEvtMsk(pub u16);
    impl EcatEvtMsk {
        #[doc = "ECAT Event masking of the ECAT Event Request Events for mapping into ECAT event field of EtherCAT frames: 0:Corresponding ECAT Event Request register bit is not mapped 1:Corresponding ECAT Event Request register bit is mapped."]
        #[must_use]
        #[inline(always)]
        pub const fn mask(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ECAT Event masking of the ECAT Event Request Events for mapping into ECAT event field of EtherCAT frames: 0:Corresponding ECAT Event Request register bit is not mapped 1:Corresponding ECAT Event Request register bit is mapped."]
        #[inline(always)]
        pub const fn set_mask(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for EcatEvtMsk {
        #[inline(always)]
        fn default() -> EcatEvtMsk {
            EcatEvtMsk(0)
        }
    }
    impl core::fmt::Debug for EcatEvtMsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EcatEvtMsk")
                .field("mask", &self.mask())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EcatEvtMsk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EcatEvtMsk {{ mask: {=u16:?} }}", self.mask())
        }
    }
    #[doc = "ECAT Event Request."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EcatEvtReq(pub u16);
    impl EcatEvtReq {
        #[doc = "DC Latch event: 0:No change on DC Latch Inputs 1:At least one change on DC Latch Inputs (Bit is cleared by reading DC Latch event times from ECAT for ECAT-controlled Latch Units, so that Latch 0/1 Status 0x09AE:0x09AF indicates no event)."]
        #[must_use]
        #[inline(always)]
        pub const fn dcl_evt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DC Latch event: 0:No change on DC Latch Inputs 1:At least one change on DC Latch Inputs (Bit is cleared by reading DC Latch event times from ECAT for ECAT-controlled Latch Units, so that Latch 0/1 Status 0x09AE:0x09AF indicates no event)."]
        #[inline(always)]
        pub const fn set_dcl_evt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "DL Status event: 0:No change in DL Status 1:DL Status change (Bit is cleared by reading out DL Status 0x0110:0x0111 from ECAT)."]
        #[must_use]
        #[inline(always)]
        pub const fn dls_evt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DL Status event: 0:No change in DL Status 1:DL Status change (Bit is cleared by reading out DL Status 0x0110:0x0111 from ECAT)."]
        #[inline(always)]
        pub const fn set_dls_evt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "AL Status event: 0:No change in AL Status 1:AL Status change (Bit is cleared by reading out AL Status 0x0130:0x0131 from ECAT)."]
        #[must_use]
        #[inline(always)]
        pub const fn als_evt(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AL Status event: 0:No change in AL Status 1:AL Status change (Bit is cleared by reading out AL Status 0x0130:0x0131 from ECAT)."]
        #[inline(always)]
        pub const fn set_als_evt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Mirrors values of each SyncManager Status: 0:No Sync Channel 0 event 1:Sync Channel 0 event pending 0:No Sync Channel 1 event 1:Sync Channel 1 event pending … 0:No Sync Channel 7 event 1:Sync Channel 7 event pending."]
        #[must_use]
        #[inline(always)]
        pub const fn mv(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0xff;
            val as u8
        }
        #[doc = "Mirrors values of each SyncManager Status: 0:No Sync Channel 0 event 1:Sync Channel 0 event pending 0:No Sync Channel 1 event 1:Sync Channel 1 event pending … 0:No Sync Channel 7 event 1:Sync Channel 7 event pending."]
        #[inline(always)]
        pub const fn set_mv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 4usize)) | (((val as u16) & 0xff) << 4usize);
        }
    }
    impl Default for EcatEvtReq {
        #[inline(always)]
        fn default() -> EcatEvtReq {
            EcatEvtReq(0)
        }
    }
    impl core::fmt::Debug for EcatEvtReq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EcatEvtReq")
                .field("dcl_evt", &self.dcl_evt())
                .field("dls_evt", &self.dls_evt())
                .field("als_evt", &self.als_evt())
                .field("mv", &self.mv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EcatEvtReq {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EcatEvtReq {{ dcl_evt: {=bool:?}, dls_evt: {=bool:?}, als_evt: {=bool:?}, mv: {=u8:?} }}" , self . dcl_evt () , self . dls_evt () , self . als_evt () , self . mv ())
        }
    }
    #[doc = "ECAT Processing Unit Error Counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EcatPuErrCnt(pub u8);
    impl EcatPuErrCnt {
        #[doc = "ECAT Processing Unit error counter (counting is stopped when 0xFF is reached). Counts errors of frames passing the Processing Unit."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ECAT Processing Unit error counter (counting is stopped when 0xFF is reached). Counts errors of frames passing the Processing Unit."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for EcatPuErrCnt {
        #[inline(always)]
        fn default() -> EcatPuErrCnt {
            EcatPuErrCnt(0)
        }
    }
    impl core::fmt::Debug for EcatPuErrCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EcatPuErrCnt")
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EcatPuErrCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EcatPuErrCnt {{ cnt: {=u8:?} }}", self.cnt())
        }
    }
    #[doc = "EEPROM Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EepromAddr(pub u32);
    impl EepromAddr {
        #[doc = "EEPROM Address 0:First word (= 16 bit) 1:Second word … Actually used EEPROM Address bits: \\[9-0\\]
: EEPROM size up to 16 Kbit \\[17-0\\]
: EEPROM size 32 Kbit – 4 Mbit \\[31-0\\]
: EEPROM Emulation."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "EEPROM Address 0:First word (= 16 bit) 1:Second word … Actually used EEPROM Address bits: \\[9-0\\]
: EEPROM size up to 16 Kbit \\[17-0\\]
: EEPROM size 32 Kbit – 4 Mbit \\[31-0\\]
: EEPROM Emulation."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EepromAddr {
        #[inline(always)]
        fn default() -> EepromAddr {
            EepromAddr(0)
        }
    }
    impl core::fmt::Debug for EepromAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EepromAddr")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EepromAddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EepromAddr {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "EEPROM Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EepromCfg(pub u8);
    impl EepromCfg {
        #[doc = "EEPROM control is offered to PDI: 0:no 1:yes (PDI has EEPROM control)."]
        #[must_use]
        #[inline(always)]
        pub const fn pdi(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EEPROM control is offered to PDI: 0:no 1:yes (PDI has EEPROM control)."]
        #[inline(always)]
        pub const fn set_pdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Force ECAT access: 0:Do not change Bit 0x0501\\[0\\]
1:Reset Bit 0x0501\\[0\\]
to 0."]
        #[must_use]
        #[inline(always)]
        pub const fn force_ecat(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force ECAT access: 0:Do not change Bit 0x0501\\[0\\]
1:Reset Bit 0x0501\\[0\\]
to 0."]
        #[inline(always)]
        pub const fn set_force_ecat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for EepromCfg {
        #[inline(always)]
        fn default() -> EepromCfg {
            EepromCfg(0)
        }
    }
    impl core::fmt::Debug for EepromCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EepromCfg")
                .field("pdi", &self.pdi())
                .field("force_ecat", &self.force_ecat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EepromCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EepromCfg {{ pdi: {=bool:?}, force_ecat: {=bool:?} }}",
                self.pdi(),
                self.force_ecat()
            )
        }
    }
    #[doc = "EEPROM Control/Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EepromCtrlStat(pub u16);
    impl EepromCtrlStat {
        #[doc = "ECAT write enable*2 : 0:Write requests are disabled 1:Write requests are enabled This bit is always 1 if PDI has EEPROM control."]
        #[must_use]
        #[inline(always)]
        pub const fn ecat_wen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECAT write enable*2 : 0:Write requests are disabled 1:Write requests are enabled This bit is always 1 if PDI has EEPROM control."]
        #[inline(always)]
        pub const fn set_ecat_wen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "EPROM emulation: 0:Normal operation (I²C interface used) 1:PDI emulates EEPROM (I²C not used)."]
        #[must_use]
        #[inline(always)]
        pub const fn ee_emu(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EPROM emulation: 0:Normal operation (I²C interface used) 1:PDI emulates EEPROM (I²C not used)."]
        #[inline(always)]
        pub const fn set_ee_emu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "Supported number of EEPROM read bytes: 0:4 Bytes 1:8 Bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn num_rd_byte(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Supported number of EEPROM read bytes: 0:4 Bytes 1:8 Bytes."]
        #[inline(always)]
        pub const fn set_num_rd_byte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Selected EEPROM Algorithm: 0:1 address byte (1Kbit – 16Kbit EEPROMs) 1:2 address bytes (32Kbit – 4 Mbit EEPROMs)."]
        #[must_use]
        #[inline(always)]
        pub const fn ee_algm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Selected EEPROM Algorithm: 0:1 address byte (1Kbit – 16Kbit EEPROMs) 1:2 address bytes (32Kbit – 4 Mbit EEPROMs)."]
        #[inline(always)]
        pub const fn set_ee_algm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Command register*2: Write:Initiate command. Read:Currently executed command Commands: 000:No command/EEPROM idle (clear error bits) 001:Read 010:Write 100:Reload Others:Reserved/invalid commands (do not issue) EEPROM emulation only:after execution, PDI writes command value to indicate operation is ready."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Command register*2: Write:Initiate command. Read:Currently executed command Commands: 000:No command/EEPROM idle (clear error bits) 001:Read 010:Write 100:Reload Others:Reserved/invalid commands (do not issue) EEPROM emulation only:after execution, PDI writes command value to indicate operation is ready."]
        #[inline(always)]
        pub const fn set_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
        }
        #[doc = "Checksum Error in ESC Configuration Area: 0:Checksum ok 1:Checksum error EEPROM emulation for IP Core only:PDI writes 1 if a CRC failure has occurred for a reload command."]
        #[must_use]
        #[inline(always)]
        pub const fn cksm_err(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Checksum Error in ESC Configuration Area: 0:Checksum ok 1:Checksum error EEPROM emulation for IP Core only:PDI writes 1 if a CRC failure has occurred for a reload command."]
        #[inline(always)]
        pub const fn set_cksm_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "EEPROM loading status: 0:EEPROM loaded, device information ok 1:EEPROM not loaded, device information not available (EEPROM loading in progress or finished with a failure)."]
        #[must_use]
        #[inline(always)]
        pub const fn ee_lds(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "EEPROM loading status: 0:EEPROM loaded, device information ok 1:EEPROM not loaded, device information not available (EEPROM loading in progress or finished with a failure)."]
        #[inline(always)]
        pub const fn set_ee_lds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Error Acknowledge/Command*3 : 0:No error 1:Missing EEPROM acknowledge or invalid command EEPROM emulation only:PDI writes 1 if a temporary failure has occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn err_ack_cmd(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Error Acknowledge/Command*3 : 0:No error 1:Missing EEPROM acknowledge or invalid command EEPROM emulation only:PDI writes 1 if a temporary failure has occurred."]
        #[inline(always)]
        pub const fn set_err_ack_cmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "Error Write Enable*3 : 0:No error 1:Write Command without Write enable."]
        #[must_use]
        #[inline(always)]
        pub const fn err_wen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Error Write Enable*3 : 0:No error 1:Write Command without Write enable."]
        #[inline(always)]
        pub const fn set_err_wen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "Busy: 0:EEPROM Interface is idle 1:EEPROM Interface is busy."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Busy: 0:EEPROM Interface is idle 1:EEPROM Interface is busy."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for EepromCtrlStat {
        #[inline(always)]
        fn default() -> EepromCtrlStat {
            EepromCtrlStat(0)
        }
    }
    impl core::fmt::Debug for EepromCtrlStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EepromCtrlStat")
                .field("ecat_wen", &self.ecat_wen())
                .field("ee_emu", &self.ee_emu())
                .field("num_rd_byte", &self.num_rd_byte())
                .field("ee_algm", &self.ee_algm())
                .field("cmd", &self.cmd())
                .field("cksm_err", &self.cksm_err())
                .field("ee_lds", &self.ee_lds())
                .field("err_ack_cmd", &self.err_ack_cmd())
                .field("err_wen", &self.err_wen())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EepromCtrlStat {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EepromCtrlStat {{ ecat_wen: {=bool:?}, ee_emu: {=bool:?}, num_rd_byte: {=bool:?}, ee_algm: {=bool:?}, cmd: {=u8:?}, cksm_err: {=bool:?}, ee_lds: {=bool:?}, err_ack_cmd: {=bool:?}, err_wen: {=bool:?}, busy: {=bool:?} }}" , self . ecat_wen () , self . ee_emu () , self . num_rd_byte () , self . ee_algm () , self . cmd () , self . cksm_err () , self . ee_lds () , self . err_ack_cmd () , self . err_wen () , self . busy ())
        }
    }
    #[doc = "EEPROM Data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EepromData(pub u64);
    impl EepromData {
        #[doc = "EEPROM Write data (data to be written to EEPROM) or EEPROM Read data (data read from EEPROM, lower bytes)."]
        #[must_use]
        #[inline(always)]
        pub const fn lo(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "EEPROM Write data (data to be written to EEPROM) or EEPROM Read data (data read from EEPROM, lower bytes)."]
        #[inline(always)]
        pub const fn set_lo(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u64) & 0xffff) << 0usize);
        }
        #[doc = "EEPROM Read data (data read from EEPROM, higher bytes)."]
        #[must_use]
        #[inline(always)]
        pub const fn hi(&self) -> u64 {
            let val = (self.0 >> 16usize) & 0xffff_ffff_ffff;
            val as u64
        }
        #[doc = "EEPROM Read data (data read from EEPROM, higher bytes)."]
        #[inline(always)]
        pub const fn set_hi(&mut self, val: u64) {
            self.0 = (self.0 & !(0xffff_ffff_ffff << 16usize))
                | (((val as u64) & 0xffff_ffff_ffff) << 16usize);
        }
    }
    impl Default for EepromData {
        #[inline(always)]
        fn default() -> EepromData {
            EepromData(0)
        }
    }
    impl core::fmt::Debug for EepromData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EepromData")
                .field("lo", &self.lo())
                .field("hi", &self.hi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EepromData {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EepromData {{ lo: {=u16:?}, hi: {=u64:?} }}",
                self.lo(),
                self.hi()
            )
        }
    }
    #[doc = "EEPROM PDI Access State."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EepromPdiAccStat(pub u8);
    impl EepromPdiAccStat {
        #[doc = "Access to EEPROM: 0:PDI releases EEPROM access 1:PDI takes EEPROM access (PDI has EEPROM control)."]
        #[must_use]
        #[inline(always)]
        pub const fn access(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Access to EEPROM: 0:PDI releases EEPROM access 1:PDI takes EEPROM access (PDI has EEPROM control)."]
        #[inline(always)]
        pub const fn set_access(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for EepromPdiAccStat {
        #[inline(always)]
        fn default() -> EepromPdiAccStat {
            EepromPdiAccStat(0)
        }
    }
    impl core::fmt::Debug for EepromPdiAccStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EepromPdiAccStat")
                .field("access", &self.access())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EepromPdiAccStat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EepromPdiAccStat {{ access: {=bool:?} }}", self.access())
        }
    }
    #[doc = "ERR LED Override."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ErrLedOvrd(pub u8);
    impl ErrLedOvrd {
        #[doc = "LED code: 0x0:Off 0x1-0xC:Flash 1x – 12x 0xD:Blinking 0xE:Flickering 0xF:On."]
        #[must_use]
        #[inline(always)]
        pub const fn led_code(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "LED code: 0x0:Off 0x1-0xC:Flash 1x – 12x 0xD:Blinking 0xE:Flickering 0xF:On."]
        #[inline(always)]
        pub const fn set_led_code(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "Enable Override: 0:Override disabled 1:Override enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn en_ovrd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Override: 0:Override disabled 1:Override enabled."]
        #[inline(always)]
        pub const fn set_en_ovrd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for ErrLedOvrd {
        #[inline(always)]
        fn default() -> ErrLedOvrd {
            ErrLedOvrd(0)
        }
    }
    impl core::fmt::Debug for ErrLedOvrd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ErrLedOvrd")
                .field("led_code", &self.led_code())
                .field("en_ovrd", &self.en_ovrd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ErrLedOvrd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ErrLedOvrd {{ led_code: {=u8:?}, en_ovrd: {=bool:?} }}",
                self.led_code(),
                self.en_ovrd()
            )
        }
    }
    #[doc = "ESC Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EscCfg(pub u8);
    impl EscCfg {
        #[doc = "Device emulation (control of AL status): 0:AL status register has to be set by PDI 1:AL status register will be set to value written to AL control register."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_emu(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Device emulation (control of AL status): 0:AL status register has to be set by PDI 1:AL status register will be set to value written to AL control register."]
        #[inline(always)]
        pub const fn set_dev_emu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Enhanced Link detection all ports: 0:disabled (if bits \\[7:4\\]=0) 1:enabled at all ports (overrides bits \\[7:4\\])."]
        #[must_use]
        #[inline(always)]
        pub const fn eldap(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enhanced Link detection all ports: 0:disabled (if bits \\[7:4\\]=0) 1:enabled at all ports (overrides bits \\[7:4\\])."]
        #[inline(always)]
        pub const fn set_eldap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Distributed Clocks SYNC Out Unit: 0:disabled (power saving) 1:enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn dcsou(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Distributed Clocks SYNC Out Unit: 0:disabled (power saving) 1:enabled."]
        #[inline(always)]
        pub const fn set_dcsou(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Distributed Clocks Latch In Unit: 0:disabled (power saving) 1:enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn cdliu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Distributed Clocks Latch In Unit: 0:disabled (power saving) 1:enabled."]
        #[inline(always)]
        pub const fn set_cdliu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Enhanced Link port 0: 0:disabled (if bit 1=0) 1:enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn elp0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enhanced Link port 0: 0:disabled (if bit 1=0) 1:enabled."]
        #[inline(always)]
        pub const fn set_elp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Enhanced Link port 1: 0:disabled (if bit 1=0) 1:enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn elp1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enhanced Link port 1: 0:disabled (if bit 1=0) 1:enabled."]
        #[inline(always)]
        pub const fn set_elp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Enhanced Link port 2: 0:disabled (if bit 1=0) 1:enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn elp2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enhanced Link port 2: 0:disabled (if bit 1=0) 1:enabled."]
        #[inline(always)]
        pub const fn set_elp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Enhanced Link port 3: 0:disabled (if bit 1=0) 1:enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn elp3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enhanced Link port 3: 0:disabled (if bit 1=0) 1:enabled."]
        #[inline(always)]
        pub const fn set_elp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for EscCfg {
        #[inline(always)]
        fn default() -> EscCfg {
            EscCfg(0)
        }
    }
    impl core::fmt::Debug for EscCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EscCfg")
                .field("dev_emu", &self.dev_emu())
                .field("eldap", &self.eldap())
                .field("dcsou", &self.dcsou())
                .field("cdliu", &self.cdliu())
                .field("elp0", &self.elp0())
                .field("elp1", &self.elp1())
                .field("elp2", &self.elp2())
                .field("elp3", &self.elp3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EscCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EscCfg {{ dev_emu: {=bool:?}, eldap: {=bool:?}, dcsou: {=bool:?}, cdliu: {=bool:?}, elp0: {=bool:?}, elp1: {=bool:?}, elp2: {=bool:?}, elp3: {=bool:?} }}" , self . dev_emu () , self . eldap () , self . dcsou () , self . cdliu () , self . elp0 () , self . elp1 () , self . elp2 () , self . elp3 ())
        }
    }
    #[doc = "ESC DL Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EscDlCtrl(pub u32);
    impl EscDlCtrl {
        #[doc = "Forwarding rule: 0:Forward non-EtherCAT frames: EtherCAT frames are processed, non-EtherCAT frames are forwarded without processing or modification. The source MAC address is not changed for any frame. 1:Destroy non-EtherCAT frames: EtherCAT frames are processed, non-EtherCAT frames are destroyed. The source MAC address is changed by the Processing Unit for every frame (SOURCE_MAC\\[1\\]
is set."]
        #[must_use]
        #[inline(always)]
        pub const fn fr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Forwarding rule: 0:Forward non-EtherCAT frames: EtherCAT frames are processed, non-EtherCAT frames are forwarded without processing or modification. The source MAC address is not changed for any frame. 1:Destroy non-EtherCAT frames: EtherCAT frames are processed, non-EtherCAT frames are destroyed. The source MAC address is changed by the Processing Unit for every frame (SOURCE_MAC\\[1\\]
is set."]
        #[inline(always)]
        pub const fn set_fr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Temporary use of settings in 0x0100:0x0103\\[8:15\\]: 0:permanent use 1:use for about 1 second, then revert to previous settings."]
        #[must_use]
        #[inline(always)]
        pub const fn tu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Temporary use of settings in 0x0100:0x0103\\[8:15\\]: 0:permanent use 1:use for about 1 second, then revert to previous settings."]
        #[inline(always)]
        pub const fn set_tu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Loop Port 0: 00:Auto 01:Auto Close 10:Open 11:Closed NOTE: Loop open means sending/receiving over this port is enabled, loop closed means sending/receiving is disabled and frames are forwarded to the next open port internally. Auto:loop closed at link down, opened at link up Auto Close:loop closed at link down, opened with writing 01 again after link up (or receiving a valid Ethernet frame at the closed port) Open:loop open regardless of link state Closed:loop closed regardless of link state."]
        #[must_use]
        #[inline(always)]
        pub const fn lp0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Loop Port 0: 00:Auto 01:Auto Close 10:Open 11:Closed NOTE: Loop open means sending/receiving over this port is enabled, loop closed means sending/receiving is disabled and frames are forwarded to the next open port internally. Auto:loop closed at link down, opened at link up Auto Close:loop closed at link down, opened with writing 01 again after link up (or receiving a valid Ethernet frame at the closed port) Open:loop open regardless of link state Closed:loop closed regardless of link state."]
        #[inline(always)]
        pub const fn set_lp0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Loop Port 1: 00:Auto 01:Auto Close 10:Open 11:Closed."]
        #[must_use]
        #[inline(always)]
        pub const fn lp1(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Loop Port 1: 00:Auto 01:Auto Close 10:Open 11:Closed."]
        #[inline(always)]
        pub const fn set_lp1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Loop Port 2: 00:Auto 01:Auto Close 10:Open 11:Closed."]
        #[must_use]
        #[inline(always)]
        pub const fn lp2(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Loop Port 2: 00:Auto 01:Auto Close 10:Open 11:Closed."]
        #[inline(always)]
        pub const fn set_lp2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Loop Port 3: 00:Auto 01:Auto Close 10:Open 11:Closed."]
        #[must_use]
        #[inline(always)]
        pub const fn lp3(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Loop Port 3: 00:Auto 01:Auto Close 10:Open 11:Closed."]
        #[inline(always)]
        pub const fn set_lp3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "RX FIFO Size (ESC delays start of forwarding until FIFO is at least half full). RX FIFO Size/RX delay reduction** : Value:EBUS:MII: 0:-50 ns -40 ns (-80 ns***) 1:-40 ns -40 ns (-80 ns***) 2:-30 ns -40 ns 3:-20 ns -40 ns 4:-10 ns no change 5:no change no change 6:no change no change 7:default default NOTE:EEPROM value is only taken over at first EEPROM load after power-on or reset."]
        #[must_use]
        #[inline(always)]
        pub const fn rfs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "RX FIFO Size (ESC delays start of forwarding until FIFO is at least half full). RX FIFO Size/RX delay reduction** : Value:EBUS:MII: 0:-50 ns -40 ns (-80 ns***) 1:-40 ns -40 ns (-80 ns***) 2:-30 ns -40 ns 3:-20 ns -40 ns 4:-10 ns no change 5:no change no change 6:no change no change 7:default default NOTE:EEPROM value is only taken over at first EEPROM load after power-on or reset."]
        #[inline(always)]
        pub const fn set_rfs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Station alias: 0:Ignore Station Alias 1:Alias can be used for all configured address comm."]
        #[must_use]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Station alias: 0:Ignore Station Alias 1:Alias can be used for all configured address comm."]
        #[inline(always)]
        pub const fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for EscDlCtrl {
        #[inline(always)]
        fn default() -> EscDlCtrl {
            EscDlCtrl(0)
        }
    }
    impl core::fmt::Debug for EscDlCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EscDlCtrl")
                .field("fr", &self.fr())
                .field("tu", &self.tu())
                .field("lp0", &self.lp0())
                .field("lp1", &self.lp1())
                .field("lp2", &self.lp2())
                .field("lp3", &self.lp3())
                .field("rfs", &self.rfs())
                .field("sa", &self.sa())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EscDlCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EscDlCtrl {{ fr: {=bool:?}, tu: {=bool:?}, lp0: {=u8:?}, lp1: {=u8:?}, lp2: {=u8:?}, lp3: {=u8:?}, rfs: {=u8:?}, sa: {=bool:?} }}" , self . fr () , self . tu () , self . lp0 () , self . lp1 () , self . lp2 () , self . lp3 () , self . rfs () , self . sa ())
        }
    }
    #[doc = "ESC DL Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EscDlStat(pub u16);
    impl EscDlStat {
        #[doc = "PDI operational/EEPROM loaded correctly: 0:EEPROM not loaded, PDI not operational (no access to Process Data RAM) 1:EEPROM loaded correctly, PDI operational (access to Process Data RAM)."]
        #[must_use]
        #[inline(always)]
        pub const fn eplc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PDI operational/EEPROM loaded correctly: 0:EEPROM not loaded, PDI not operational (no access to Process Data RAM) 1:EEPROM loaded correctly, PDI operational (access to Process Data RAM)."]
        #[inline(always)]
        pub const fn set_eplc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "PDI Watchdog Status: 0:Watchdog expired 1:Watchdog reloaded."]
        #[must_use]
        #[inline(always)]
        pub const fn wds(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PDI Watchdog Status: 0:Watchdog expired 1:Watchdog reloaded."]
        #[inline(always)]
        pub const fn set_wds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Enhanced Link detection: 0:Deactivated for all ports 1:Activated for at least one port NOTE:EEPROM value is only transferred into this register at first EEPROM load after power-on or reset."]
        #[must_use]
        #[inline(always)]
        pub const fn eld(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enhanced Link detection: 0:Deactivated for all ports 1:Activated for at least one port NOTE:EEPROM value is only transferred into this register at first EEPROM load after power-on or reset."]
        #[inline(always)]
        pub const fn set_eld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Physical link on Port 0: 0:No link 1:Link detected."]
        #[must_use]
        #[inline(always)]
        pub const fn plp0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Physical link on Port 0: 0:No link 1:Link detected."]
        #[inline(always)]
        pub const fn set_plp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Physical link on Port 1: 0:No link 1:Link detected."]
        #[must_use]
        #[inline(always)]
        pub const fn plp1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Physical link on Port 1: 0:No link 1:Link detected."]
        #[inline(always)]
        pub const fn set_plp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "Physical link on Port 2: 0:No link 1:Link detected."]
        #[must_use]
        #[inline(always)]
        pub const fn plp2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Physical link on Port 2: 0:No link 1:Link detected."]
        #[inline(always)]
        pub const fn set_plp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Physical link on Port 3: 0:No link 1:Link detected."]
        #[must_use]
        #[inline(always)]
        pub const fn plp3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Physical link on Port 3: 0:No link 1:Link detected."]
        #[inline(always)]
        pub const fn set_plp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Loop Port 0: 0:Open 1:Closed."]
        #[must_use]
        #[inline(always)]
        pub const fn lp0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Port 0: 0:Open 1:Closed."]
        #[inline(always)]
        pub const fn set_lp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "Communication on Port 0: 0:No stable communication 1:Communication established."]
        #[must_use]
        #[inline(always)]
        pub const fn cp0(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Communication on Port 0: 0:No stable communication 1:Communication established."]
        #[inline(always)]
        pub const fn set_cp0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Loop Port 1: 0:Open 1:Closed."]
        #[must_use]
        #[inline(always)]
        pub const fn lp1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Port 1: 0:Open 1:Closed."]
        #[inline(always)]
        pub const fn set_lp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Communication on Port 1: 0:No stable communication 1:Communication established."]
        #[must_use]
        #[inline(always)]
        pub const fn cp1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Communication on Port 1: 0:No stable communication 1:Communication established."]
        #[inline(always)]
        pub const fn set_cp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "Loop Port 2: 0:Open 1:Closed."]
        #[must_use]
        #[inline(always)]
        pub const fn lp2(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Port 2: 0:Open 1:Closed."]
        #[inline(always)]
        pub const fn set_lp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "Communication on Port 2: 0:No stable communication 1:Communication established."]
        #[must_use]
        #[inline(always)]
        pub const fn cp2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Communication on Port 2: 0:No stable communication 1:Communication established."]
        #[inline(always)]
        pub const fn set_cp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "Loop Port 3: 0:Open 1:Closed."]
        #[must_use]
        #[inline(always)]
        pub const fn lp3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Port 3: 0:Open 1:Closed."]
        #[inline(always)]
        pub const fn set_lp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "Communication on Port 3: 0:No stable communication 1:Communication established."]
        #[must_use]
        #[inline(always)]
        pub const fn cp3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Communication on Port 3: 0:No stable communication 1:Communication established."]
        #[inline(always)]
        pub const fn set_cp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for EscDlStat {
        #[inline(always)]
        fn default() -> EscDlStat {
            EscDlStat(0)
        }
    }
    impl core::fmt::Debug for EscDlStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EscDlStat")
                .field("eplc", &self.eplc())
                .field("wds", &self.wds())
                .field("eld", &self.eld())
                .field("plp0", &self.plp0())
                .field("plp1", &self.plp1())
                .field("plp2", &self.plp2())
                .field("plp3", &self.plp3())
                .field("lp0", &self.lp0())
                .field("cp0", &self.cp0())
                .field("lp1", &self.lp1())
                .field("cp1", &self.cp1())
                .field("lp2", &self.lp2())
                .field("cp2", &self.cp2())
                .field("lp3", &self.lp3())
                .field("cp3", &self.cp3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EscDlStat {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EscDlStat {{ eplc: {=bool:?}, wds: {=bool:?}, eld: {=bool:?}, plp0: {=bool:?}, plp1: {=bool:?}, plp2: {=bool:?}, plp3: {=bool:?}, lp0: {=bool:?}, cp0: {=bool:?}, lp1: {=bool:?}, cp1: {=bool:?}, lp2: {=bool:?}, cp2: {=bool:?}, lp3: {=bool:?}, cp3: {=bool:?} }}" , self . eplc () , self . wds () , self . eld () , self . plp0 () , self . plp1 () , self . plp2 () , self . plp3 () , self . lp0 () , self . cp0 () , self . lp1 () , self . cp1 () , self . lp2 () , self . cp2 () , self . lp3 () , self . cp3 ())
        }
    }
    #[doc = "PDI Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EscPdiCtrl(pub u8);
    impl EscPdiCtrl {
        #[doc = "Process data interface: 0x00:Interface deactivated (no PDI) 0x01:4 Digital Input 0x02:4 Digital Output 0x03:2 Digital Input and 2 Digital Output 0x04:Digital I/O 0x05:SPI Slave 0x06:Oversampling I/O 0x07:EtherCAT Bridge (port 3) 0x08:16 Bit asynchronous Microcontroller interface 0x09:8 Bit asynchronous Microcontroller interface 0x0A:16 Bit synchronous Microcontroller interface 0x0B:8 Bit synchronous Microcontroller interface 0x10:32 Digital Input and 0 Digital Output 0x11:24 Digital Input and 8 Digital Output 0x12:16 Digital Input and 16 Digital Output 0x13:8 Digital Input and 24 Digital Output 0x14:0 Digital Input and 32 Digital Output 0x80:On-chip bus Others:Reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn pdi(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Process data interface: 0x00:Interface deactivated (no PDI) 0x01:4 Digital Input 0x02:4 Digital Output 0x03:2 Digital Input and 2 Digital Output 0x04:Digital I/O 0x05:SPI Slave 0x06:Oversampling I/O 0x07:EtherCAT Bridge (port 3) 0x08:16 Bit asynchronous Microcontroller interface 0x09:8 Bit asynchronous Microcontroller interface 0x0A:16 Bit synchronous Microcontroller interface 0x0B:8 Bit synchronous Microcontroller interface 0x10:32 Digital Input and 0 Digital Output 0x11:24 Digital Input and 8 Digital Output 0x12:16 Digital Input and 16 Digital Output 0x13:8 Digital Input and 24 Digital Output 0x14:0 Digital Input and 32 Digital Output 0x80:On-chip bus Others:Reserved."]
        #[inline(always)]
        pub const fn set_pdi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for EscPdiCtrl {
        #[inline(always)]
        fn default() -> EscPdiCtrl {
            EscPdiCtrl(0)
        }
    }
    impl core::fmt::Debug for EscPdiCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EscPdiCtrl")
                .field("pdi", &self.pdi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EscPdiCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EscPdiCtrl {{ pdi: {=u8:?} }}", self.pdi())
        }
    }
    #[doc = "ESC Reset ECAT."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EscRstEcat(pub u8);
    impl EscRstEcat {
        #[doc = "Progress of the reset procedure: 00:initial/reset state 01:after writing 0x52 ('R'), when previous state was 00 10:after writing 0x45 ('E'), when previous state was 01 11:after writing 0x53 ('S'), when previous state was 10. This value must not be observed because the ESC enters reset when this state is reached, resulting in state 00."]
        #[must_use]
        #[inline(always)]
        pub const fn pr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Progress of the reset procedure: 00:initial/reset state 01:after writing 0x52 ('R'), when previous state was 00 10:after writing 0x45 ('E'), when previous state was 01 11:after writing 0x53 ('S'), when previous state was 10. This value must not be observed because the ESC enters reset when this state is reached, resulting in state 00."]
        #[inline(always)]
        pub const fn set_pr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
    }
    impl Default for EscRstEcat {
        #[inline(always)]
        fn default() -> EscRstEcat {
            EscRstEcat(0)
        }
    }
    impl core::fmt::Debug for EscRstEcat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EscRstEcat")
                .field("pr", &self.pr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EscRstEcat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EscRstEcat {{ pr: {=u8:?} }}", self.pr())
        }
    }
    #[doc = "ESC Reset PDI."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EscRstPdi(pub u8);
    impl EscRstPdi {
        #[doc = "A reset is asserted after writing the reset sequence 0x52 ('R'), 0x45 ('E') and 0x53 ('S') in this register with 3 consecutive commands. Any other command which does not continue the sequence by writing the next expected value will cancel the reset procedure."]
        #[must_use]
        #[inline(always)]
        pub const fn rst(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "A reset is asserted after writing the reset sequence 0x52 ('R'), 0x45 ('E') and 0x53 ('S') in this register with 3 consecutive commands. Any other command which does not continue the sequence by writing the next expected value will cancel the reset procedure."]
        #[inline(always)]
        pub const fn set_rst(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for EscRstPdi {
        #[inline(always)]
        fn default() -> EscRstPdi {
            EscRstPdi(0)
        }
    }
    impl core::fmt::Debug for EscRstPdi {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EscRstPdi")
                .field("rst", &self.rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EscRstPdi {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EscRstPdi {{ rst: {=u8:?} }}", self.rst())
        }
    }
    #[doc = "Type of EtherCAT controller."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EscType(pub u8);
    impl EscType {
        #[doc = "Controller type."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Controller type."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for EscType {
        #[inline(always)]
        fn default() -> EscType {
            EscType(0)
        }
    }
    impl core::fmt::Debug for EscType {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EscType")
                .field("type_", &self.type_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EscType {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EscType {{ type_: {=u8:?} }}", self.type_())
        }
    }
    #[doc = "ESC Write Enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EscWen(pub u8);
    impl EscWen {
        #[doc = "If ESC write protection is enabled, this register has to be written in the same Ethernet frame (value does not matter) before other writes to this station are allowed. This bit is self-clearing at the beginning of the next frame (SOF), or if ESC Write Protection is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "If ESC write protection is enabled, this register has to be written in the same Ethernet frame (value does not matter) before other writes to this station are allowed. This bit is self-clearing at the beginning of the next frame (SOF), or if ESC Write Protection is disabled."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for EscWen {
        #[inline(always)]
        fn default() -> EscWen {
            EscWen(0)
        }
    }
    impl core::fmt::Debug for EscWen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EscWen").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EscWen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EscWen {{ en: {=bool:?} }}", self.en())
        }
    }
    #[doc = "ESC Write Protection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EscWp(pub u8);
    impl EscWp {
        #[doc = "Write protect: 0:Protection disabled 1:Protection enabled All areas are write-protected, except for 0x0030."]
        #[must_use]
        #[inline(always)]
        pub const fn wp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Write protect: 0:Protection disabled 1:Protection enabled All areas are write-protected, except for 0x0030."]
        #[inline(always)]
        pub const fn set_wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for EscWp {
        #[inline(always)]
        fn default() -> EscWp {
            EscWp(0)
        }
    }
    impl core::fmt::Debug for EscWp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EscWp").field("wp", &self.wp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EscWp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EscWp {{ wp: {=bool:?} }}", self.wp())
        }
    }
    #[doc = "ESC Feature supported."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Feature(pub u16);
    impl Feature {
        #[doc = "FMMU Operation: 0:Bit oriented 1:Byte oriented."]
        #[must_use]
        #[inline(always)]
        pub const fn fmmu(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FMMU Operation: 0:Bit oriented 1:Byte oriented."]
        #[inline(always)]
        pub const fn set_fmmu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Distributed Clocks: 0:Not available 1:Available."]
        #[must_use]
        #[inline(always)]
        pub const fn dc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Distributed Clocks: 0:Not available 1:Available."]
        #[inline(always)]
        pub const fn set_dc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Distributed Clocks width: 0:32 bit 1:64 bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dcw(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Distributed Clocks width: 0:32 bit 1:64 bit."]
        #[inline(always)]
        pub const fn set_dcw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Enhanced Link Detection MII: 0:Not available 1:Available."]
        #[must_use]
        #[inline(always)]
        pub const fn eldm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enhanced Link Detection MII: 0:Not available 1:Available."]
        #[inline(always)]
        pub const fn set_eldm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Seperate Handling of FCS Errors: 0:Not supported 1:Supported, frames with wrong FCS and additional nibble will be counted separately in Forwarded RX Error Counter."]
        #[must_use]
        #[inline(always)]
        pub const fn shfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Seperate Handling of FCS Errors: 0:Not supported 1:Supported, frames with wrong FCS and additional nibble will be counted separately in Forwarded RX Error Counter."]
        #[inline(always)]
        pub const fn set_shfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Enhanced DC SYNC Activation: 0:Not available 1:Available Note:This feature refers to registers 0x981\\[7:3\\]
and 0x0984."]
        #[must_use]
        #[inline(always)]
        pub const fn edsa(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enhanced DC SYNC Activation: 0:Not available 1:Available Note:This feature refers to registers 0x981\\[7:3\\]
and 0x0984."]
        #[inline(always)]
        pub const fn set_edsa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "EtherCAT LRW command support: 0:Supported 1:Not supported."]
        #[must_use]
        #[inline(always)]
        pub const fn lrw(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "EtherCAT LRW command support: 0:Supported 1:Not supported."]
        #[inline(always)]
        pub const fn set_lrw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "EtherCAT read/write command support(BRW,APRW,FPRW): 0:Supported 1:Not supported."]
        #[must_use]
        #[inline(always)]
        pub const fn rwc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "EtherCAT read/write command support(BRW,APRW,FPRW): 0:Supported 1:Not supported."]
        #[inline(always)]
        pub const fn set_rwc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Fixed FMMU/SyncManager configuration: 0:Variable configuration 1:Fixed configuration (refer to documentation of supporting ESCs)."]
        #[must_use]
        #[inline(always)]
        pub const fn ffsc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Fixed FMMU/SyncManager configuration: 0:Variable configuration 1:Fixed configuration (refer to documentation of supporting ESCs)."]
        #[inline(always)]
        pub const fn set_ffsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
    }
    impl Default for Feature {
        #[inline(always)]
        fn default() -> Feature {
            Feature(0)
        }
    }
    impl core::fmt::Debug for Feature {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Feature")
                .field("fmmu", &self.fmmu())
                .field("dc", &self.dc())
                .field("dcw", &self.dcw())
                .field("eldm", &self.eldm())
                .field("shfe", &self.shfe())
                .field("edsa", &self.edsa())
                .field("lrw", &self.lrw())
                .field("rwc", &self.rwc())
                .field("ffsc", &self.ffsc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Feature {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Feature {{ fmmu: {=bool:?}, dc: {=bool:?}, dcw: {=bool:?}, eldm: {=bool:?}, shfe: {=bool:?}, edsa: {=bool:?}, lrw: {=bool:?}, rwc: {=bool:?}, ffsc: {=bool:?} }}" , self . fmmu () , self . dc () , self . dcw () , self . eldm () , self . shfe () , self . edsa () , self . lrw () , self . rwc () , self . ffsc ())
        }
    }
    #[doc = "Activate."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmmuActivate(pub u8);
    impl FmmuActivate {
        #[doc = "0:FMMU deactivated 1:FMMU activated. FMMU checks logically addressed blocks to be mapped according to configured mapping."]
        #[must_use]
        #[inline(always)]
        pub const fn act(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "0:FMMU deactivated 1:FMMU activated. FMMU checks logically addressed blocks to be mapped according to configured mapping."]
        #[inline(always)]
        pub const fn set_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for FmmuActivate {
        #[inline(always)]
        fn default() -> FmmuActivate {
            FmmuActivate(0)
        }
    }
    impl core::fmt::Debug for FmmuActivate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmmuActivate")
                .field("act", &self.act())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmmuActivate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FmmuActivate {{ act: {=bool:?} }}", self.act())
        }
    }
    #[doc = "Length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmmuLength(pub u16);
    impl FmmuLength {
        #[doc = "Offset from the first logical FMMU byte to the last FMMU byte + 1 (e.g., if two bytes are used, then this parameter shall contain 2)."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Offset from the first logical FMMU byte to the last FMMU byte + 1 (e.g., if two bytes are used, then this parameter shall contain 2)."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for FmmuLength {
        #[inline(always)]
        fn default() -> FmmuLength {
            FmmuLength(0)
        }
    }
    impl core::fmt::Debug for FmmuLength {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmmuLength")
                .field("offset", &self.offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmmuLength {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FmmuLength {{ offset: {=u16:?} }}", self.offset())
        }
    }
    #[doc = "FMMU supported."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmmuNum(pub u8);
    impl FmmuNum {
        #[doc = "Number of supported FMMU channels (or entities)."]
        #[must_use]
        #[inline(always)]
        pub const fn num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Number of supported FMMU channels (or entities)."]
        #[inline(always)]
        pub const fn set_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for FmmuNum {
        #[inline(always)]
        fn default() -> FmmuNum {
            FmmuNum(0)
        }
    }
    impl core::fmt::Debug for FmmuNum {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmmuNum").field("num", &self.num()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmmuNum {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FmmuNum {{ num: {=u8:?} }}", self.num())
        }
    }
    #[doc = "Physical Start Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmmuPhysicalStartAddr(pub u16);
    impl FmmuPhysicalStartAddr {
        #[doc = "Physical Start Address (mapped to logical Start address)."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Physical Start Address (mapped to logical Start address)."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for FmmuPhysicalStartAddr {
        #[inline(always)]
        fn default() -> FmmuPhysicalStartAddr {
            FmmuPhysicalStartAddr(0)
        }
    }
    impl core::fmt::Debug for FmmuPhysicalStartAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmmuPhysicalStartAddr")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmmuPhysicalStartAddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FmmuPhysicalStartAddr {{ addr: {=u16:?} }}", self.addr())
        }
    }
    #[doc = "Type."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FmmuType(pub u8);
    impl FmmuType {
        #[doc = "0:Ignore mapping for read accesses 1:Use mapping for read accesses."]
        #[must_use]
        #[inline(always)]
        pub const fn map_rd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "0:Ignore mapping for read accesses 1:Use mapping for read accesses."]
        #[inline(always)]
        pub const fn set_map_rd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "0:Ignore mapping for write accesses 1:Use mapping for write accesses."]
        #[must_use]
        #[inline(always)]
        pub const fn map_wr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0:Ignore mapping for write accesses 1:Use mapping for write accesses."]
        #[inline(always)]
        pub const fn set_map_wr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for FmmuType {
        #[inline(always)]
        fn default() -> FmmuType {
            FmmuType(0)
        }
    }
    impl core::fmt::Debug for FmmuType {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FmmuType")
                .field("map_rd", &self.map_rd())
                .field("map_wr", &self.map_wr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FmmuType {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FmmuType {{ map_rd: {=bool:?}, map_wr: {=bool:?} }}",
                self.map_rd(),
                self.map_wr()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FwdRxErrCnt(pub u8);
    impl FwdRxErrCnt {
        #[doc = "Forwarded error counter of Port y (counting is stopped when 0xFF is reached)."]
        #[must_use]
        #[inline(always)]
        pub const fn err_cnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Forwarded error counter of Port y (counting is stopped when 0xFF is reached)."]
        #[inline(always)]
        pub const fn set_err_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for FwdRxErrCnt {
        #[inline(always)]
        fn default() -> FwdRxErrCnt {
            FwdRxErrCnt(0)
        }
    }
    impl core::fmt::Debug for FwdRxErrCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FwdRxErrCnt")
                .field("err_cnt", &self.err_cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FwdRxErrCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "FwdRxErrCnt {{ err_cnt: {=u8:?} }}", self.err_cnt())
        }
    }
    #[doc = "General Purpose Inputs."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpi(pub u64);
    impl Gpi {
        #[doc = "General Purpose Input Data."]
        #[must_use]
        #[inline(always)]
        pub const fn gpid(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "General Purpose Input Data."]
        #[inline(always)]
        pub const fn set_gpid(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for Gpi {
        #[inline(always)]
        fn default() -> Gpi {
            Gpi(0)
        }
    }
    impl core::fmt::Debug for Gpi {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gpi").field("gpid", &self.gpid()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpi {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpi {{ gpid: {=u64:?} }}", self.gpid())
        }
    }
    #[doc = "GPI low word Override value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpiOverride0(pub u32);
    impl GpiOverride0 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn gpr_override_low(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_gpr_override_low(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GpiOverride0 {
        #[inline(always)]
        fn default() -> GpiOverride0 {
            GpiOverride0(0)
        }
    }
    impl core::fmt::Debug for GpiOverride0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpiOverride0")
                .field("gpr_override_low", &self.gpr_override_low())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GpiOverride0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GpiOverride0 {{ gpr_override_low: {=u32:?} }}",
                self.gpr_override_low()
            )
        }
    }
    #[doc = "GPI high word Override value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpiOverride1(pub u32);
    impl GpiOverride1 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn gpr_override_high(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_gpr_override_high(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GpiOverride1 {
        #[inline(always)]
        fn default() -> GpiOverride1 {
            GpiOverride1(0)
        }
    }
    impl core::fmt::Debug for GpiOverride1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpiOverride1")
                .field("gpr_override_high", &self.gpr_override_high())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GpiOverride1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GpiOverride1 {{ gpr_override_high: {=u32:?} }}",
                self.gpr_override_high()
            )
        }
    }
    #[doc = "GPI low word read value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpiReg0(pub u32);
    impl GpiReg0 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GpiReg0 {
        #[inline(always)]
        fn default() -> GpiReg0 {
            GpiReg0(0)
        }
    }
    impl core::fmt::Debug for GpiReg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpiReg0")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GpiReg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GpiReg0 {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "GPI high word read value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpiReg1(pub u32);
    impl GpiReg1 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GpiReg1 {
        #[inline(always)]
        fn default() -> GpiReg1 {
            GpiReg1(0)
        }
    }
    impl core::fmt::Debug for GpiReg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpiReg1")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GpiReg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GpiReg1 {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "GPIO Output Enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpioCtrl(pub u32);
    impl GpioCtrl {
        #[doc = "select the trigger signal to latch GPO. 0000: SOF; 0001: EOF; 0010: pos of SYNC0; 0011: pos of SYNC1; 0100: pos of LATCH0; 0101: pos of LATCH1; 0110: neg of LATCH0; 0111: neg of LATCH1 1000: wdog trigger; 1001: sw set gpio_ctrl\\[30\\]; others no trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn gpo_trig_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "select the trigger signal to latch GPO. 0000: SOF; 0001: EOF; 0010: pos of SYNC0; 0011: pos of SYNC1; 0100: pos of LATCH0; 0101: pos of LATCH1; 0110: neg of LATCH0; 0111: neg of LATCH1 1000: wdog trigger; 1001: sw set gpio_ctrl\\[30\\]; others no trigger."]
        #[inline(always)]
        pub const fn set_gpo_trig_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "use gpo_trig_sel can select the trigger event to latch GPO signal(from core) set to use triggered signal; clr to use GPO signals direclty(from reg or pad)."]
        #[must_use]
        #[inline(always)]
        pub const fn gpo_trig_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "use gpo_trig_sel can select the trigger event to latch GPO signal(from core) set to use triggered signal; clr to use GPO signals direclty(from reg or pad)."]
        #[inline(always)]
        pub const fn set_gpo_trig_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "select the trigger signal to latch GPI. 0000: SOF; 0001: EOF; 0010: pos of SYNC0; 0011: pos of SYNC1; 0100: pos of LATCH0; 0101: pos of LATCH1; 0110: neg of LATCH0; 0111: neg of LATCH1 1000: wdog trigger; 1001: sw set gpio_ctrl\\[31\\]; others no trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn gpi_trig_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "select the trigger signal to latch GPI. 0000: SOF; 0001: EOF; 0010: pos of SYNC0; 0011: pos of SYNC1; 0100: pos of LATCH0; 0101: pos of LATCH1; 0110: neg of LATCH0; 0111: neg of LATCH1 1000: wdog trigger; 1001: sw set gpio_ctrl\\[31\\]; others no trigger."]
        #[inline(always)]
        pub const fn set_gpi_trig_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "use gpi_trig_sel can select the trigger event to latch GPI signal(from reg or pad) set to use triggered signal; clr to use signals direclty(from reg or pad) assign pdi_gpi = gpi_trig_en ? gpi_reg : (gpi_override_en ? gpi_override :pad_di_ecat_gpi);."]
        #[must_use]
        #[inline(always)]
        pub const fn gpi_trig_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "use gpi_trig_sel can select the trigger event to latch GPI signal(from reg or pad) set to use triggered signal; clr to use signals direclty(from reg or pad) assign pdi_gpi = gpi_trig_en ? gpi_reg : (gpi_override_en ? gpi_override :pad_di_ecat_gpi);."]
        #[inline(always)]
        pub const fn set_gpi_trig_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "set this bit will use GPI from the software register gpi_override0/1 clr to use GPI from pad directly."]
        #[must_use]
        #[inline(always)]
        pub const fn gpi_override_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "set this bit will use GPI from the software register gpi_override0/1 clr to use GPI from pad directly."]
        #[inline(always)]
        pub const fn set_gpi_override_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "if gpo_trig_sel is set to 4'b1001, setting this bit will latch GPO to gpo_reg0/1."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_latch_gpo(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "if gpo_trig_sel is set to 4'b1001, setting this bit will latch GPO to gpo_reg0/1."]
        #[inline(always)]
        pub const fn set_sw_latch_gpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "if gpi_trig_sel is set to 4'b1001, setting this bit will latch GPI to gpi_reg0/1."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_latch_gpi(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "if gpi_trig_sel is set to 4'b1001, setting this bit will latch GPI to gpi_reg0/1."]
        #[inline(always)]
        pub const fn set_sw_latch_gpi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for GpioCtrl {
        #[inline(always)]
        fn default() -> GpioCtrl {
            GpioCtrl(0)
        }
    }
    impl core::fmt::Debug for GpioCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpioCtrl")
                .field("gpo_trig_sel", &self.gpo_trig_sel())
                .field("gpo_trig_en", &self.gpo_trig_en())
                .field("gpi_trig_sel", &self.gpi_trig_sel())
                .field("gpi_trig_en", &self.gpi_trig_en())
                .field("gpi_override_en", &self.gpi_override_en())
                .field("sw_latch_gpo", &self.sw_latch_gpo())
                .field("sw_latch_gpi", &self.sw_latch_gpi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GpioCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GpioCtrl {{ gpo_trig_sel: {=u8:?}, gpo_trig_en: {=bool:?}, gpi_trig_sel: {=u8:?}, gpi_trig_en: {=bool:?}, gpi_override_en: {=bool:?}, sw_latch_gpo: {=bool:?}, sw_latch_gpi: {=bool:?} }}" , self . gpo_trig_sel () , self . gpo_trig_en () , self . gpi_trig_sel () , self . gpi_trig_en () , self . gpi_override_en () , self . sw_latch_gpo () , self . sw_latch_gpi ())
        }
    }
    #[doc = "General Purpose Outputs."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpo(pub u64);
    impl Gpo {
        #[doc = "General Purpose Output Data."]
        #[must_use]
        #[inline(always)]
        pub const fn gpod(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "General Purpose Output Data."]
        #[inline(always)]
        pub const fn set_gpod(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for Gpo {
        #[inline(always)]
        fn default() -> Gpo {
            Gpo(0)
        }
    }
    impl core::fmt::Debug for Gpo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gpo").field("gpod", &self.gpod()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gpo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gpo {{ gpod: {=u64:?} }}", self.gpod())
        }
    }
    #[doc = "GPO low word read value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpoReg0(pub u32);
    impl GpoReg0 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GpoReg0 {
        #[inline(always)]
        fn default() -> GpoReg0 {
            GpoReg0(0)
        }
    }
    impl core::fmt::Debug for GpoReg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpoReg0")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GpoReg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GpoReg0 {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "GPO high word read value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpoReg1(pub u32);
    impl GpoReg1 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for GpoReg1 {
        #[inline(always)]
        fn default() -> GpoReg1 {
            GpoReg1(0)
        }
    }
    impl core::fmt::Debug for GpoReg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GpoReg1")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GpoReg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GpoReg1 {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "General Purpose Configure 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprCfg0(pub u32);
    impl GprCfg0 {
        #[doc = "Sets EEPROM size: 0:up to 16 kbit EEPROM 1:32 kbit-4Mbit EEPROM."]
        #[must_use]
        #[inline(always)]
        pub const fn prom_size(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sets EEPROM size: 0:up to 16 kbit EEPROM 1:32 kbit-4Mbit EEPROM."]
        #[inline(always)]
        pub const fn set_prom_size(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c_sclk_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_i2c_sclk_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "1 is EEPROM emulation mode (default)."]
        #[must_use]
        #[inline(always)]
        pub const fn eeprom_emu(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "1 is EEPROM emulation mode (default)."]
        #[inline(always)]
        pub const fn set_eeprom_emu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn clk100_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_clk100_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for GprCfg0 {
        #[inline(always)]
        fn default() -> GprCfg0 {
            GprCfg0(0)
        }
    }
    impl core::fmt::Debug for GprCfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GprCfg0")
                .field("prom_size", &self.prom_size())
                .field("i2c_sclk_en", &self.i2c_sclk_en())
                .field("eeprom_emu", &self.eeprom_emu())
                .field("clk100_en", &self.clk100_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GprCfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GprCfg0 {{ prom_size: {=bool:?}, i2c_sclk_en: {=bool:?}, eeprom_emu: {=bool:?}, clk100_en: {=bool:?} }}" , self . prom_size () , self . i2c_sclk_en () , self . eeprom_emu () , self . clk100_en ())
        }
    }
    #[doc = "General Purpose Configure 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprCfg1(pub u32);
    impl GprCfg1 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn rsto_ovrd_enj(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_rsto_ovrd_enj(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn rsto_ovrd(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_rsto_ovrd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "0:from TRIGGER_MUX."]
        #[must_use]
        #[inline(always)]
        pub const fn latch0_from_io(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "0:from TRIGGER_MUX."]
        #[inline(always)]
        pub const fn set_latch0_from_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "0:from NTM."]
        #[must_use]
        #[inline(always)]
        pub const fn latch1_from_io(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "0:from NTM."]
        #[inline(always)]
        pub const fn set_latch1_from_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sync0_dma_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sync0_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sync1_dma_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sync1_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn rsto_irq_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_rsto_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sync0_irq_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sync0_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sync1_irq_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sync1_irq_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for GprCfg1 {
        #[inline(always)]
        fn default() -> GprCfg1 {
            GprCfg1(0)
        }
    }
    impl core::fmt::Debug for GprCfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GprCfg1")
                .field("rsto_ovrd_enj", &self.rsto_ovrd_enj())
                .field("rsto_ovrd", &self.rsto_ovrd())
                .field("latch0_from_io", &self.latch0_from_io())
                .field("latch1_from_io", &self.latch1_from_io())
                .field("sync0_dma_en", &self.sync0_dma_en())
                .field("sync1_dma_en", &self.sync1_dma_en())
                .field("rsto_irq_en", &self.rsto_irq_en())
                .field("sync0_irq_en", &self.sync0_irq_en())
                .field("sync1_irq_en", &self.sync1_irq_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GprCfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GprCfg1 {{ rsto_ovrd_enj: {=bool:?}, rsto_ovrd: {=bool:?}, latch0_from_io: {=bool:?}, latch1_from_io: {=bool:?}, sync0_dma_en: {=bool:?}, sync1_dma_en: {=bool:?}, rsto_irq_en: {=bool:?}, sync0_irq_en: {=bool:?}, sync1_irq_en: {=bool:?} }}" , self . rsto_ovrd_enj () , self . rsto_ovrd () , self . latch0_from_io () , self . latch1_from_io () , self . sync0_dma_en () , self . sync1_dma_en () , self . rsto_irq_en () , self . sync0_irq_en () , self . sync1_irq_en ())
        }
    }
    #[doc = "General Purpose Configure 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprCfg2(pub u32);
    impl GprCfg2 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn nmii_link0_gpr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_nmii_link0_gpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn nmii_link0_from_io(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_nmii_link0_from_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn nmii_link1_gpr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_nmii_link1_gpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn nmii_link1_from_io(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_nmii_link1_from_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn nmii_link2_gpr(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_nmii_link2_gpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn nmii_link2_from_io(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_nmii_link2_from_io(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for GprCfg2 {
        #[inline(always)]
        fn default() -> GprCfg2 {
            GprCfg2(0)
        }
    }
    impl core::fmt::Debug for GprCfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GprCfg2")
                .field("nmii_link0_gpr", &self.nmii_link0_gpr())
                .field("nmii_link0_from_io", &self.nmii_link0_from_io())
                .field("nmii_link1_gpr", &self.nmii_link1_gpr())
                .field("nmii_link1_from_io", &self.nmii_link1_from_io())
                .field("nmii_link2_gpr", &self.nmii_link2_gpr())
                .field("nmii_link2_from_io", &self.nmii_link2_from_io())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GprCfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GprCfg2 {{ nmii_link0_gpr: {=bool:?}, nmii_link0_from_io: {=bool:?}, nmii_link1_gpr: {=bool:?}, nmii_link1_from_io: {=bool:?}, nmii_link2_gpr: {=bool:?}, nmii_link2_from_io: {=bool:?} }}" , self . nmii_link0_gpr () , self . nmii_link0_from_io () , self . nmii_link1_gpr () , self . nmii_link1_from_io () , self . nmii_link2_gpr () , self . nmii_link2_from_io ())
        }
    }
    #[doc = "global status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprStatus(pub u32);
    impl GprStatus {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn link_act(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_link_act(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_state(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_dev_state(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn led_run(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_led_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn led_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_led_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn led_state_run(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_led_state_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sync_out0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sync_out0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn sync_out1(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_sync_out1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pdi_wd_state(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pdi_wd_state(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pdi_wd_trigger(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pdi_wd_trigger(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pdi_eof(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pdi_eof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn pdi_sof(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_pdi_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn nlink0_padsel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_nlink0_padsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn nlink1_padsel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_nlink1_padsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn nlink2_padsel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_nlink2_padsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for GprStatus {
        #[inline(always)]
        fn default() -> GprStatus {
            GprStatus(0)
        }
    }
    impl core::fmt::Debug for GprStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GprStatus")
                .field("link_act", &self.link_act())
                .field("dev_state", &self.dev_state())
                .field("led_run", &self.led_run())
                .field("led_err", &self.led_err())
                .field("led_state_run", &self.led_state_run())
                .field("sync_out0", &self.sync_out0())
                .field("sync_out1", &self.sync_out1())
                .field("pdi_wd_state", &self.pdi_wd_state())
                .field("pdi_wd_trigger", &self.pdi_wd_trigger())
                .field("pdi_eof", &self.pdi_eof())
                .field("pdi_sof", &self.pdi_sof())
                .field("nlink0_padsel", &self.nlink0_padsel())
                .field("nlink1_padsel", &self.nlink1_padsel())
                .field("nlink2_padsel", &self.nlink2_padsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GprStatus {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GprStatus {{ link_act: {=u8:?}, dev_state: {=bool:?}, led_run: {=bool:?}, led_err: {=bool:?}, led_state_run: {=bool:?}, sync_out0: {=bool:?}, sync_out1: {=bool:?}, pdi_wd_state: {=bool:?}, pdi_wd_trigger: {=bool:?}, pdi_eof: {=bool:?}, pdi_sof: {=bool:?}, nlink0_padsel: {=u8:?}, nlink1_padsel: {=u8:?}, nlink2_padsel: {=u8:?} }}" , self . link_act () , self . dev_state () , self . led_run () , self . led_err () , self . led_state_run () , self . sync_out0 () , self . sync_out1 () , self . pdi_wd_state () , self . pdi_wd_trigger () , self . pdi_eof () , self . pdi_sof () , self . nlink0_padsel () , self . nlink1_padsel () , self . nlink2_padsel ())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IoCfg(pub u32);
    impl IoCfg {
        #[doc = "IO usage: 0:NMII_LINK0 1:NMII_LINK1 2:NMII_LINK2 3:LINK_ACT0 4:LINK_ACT1 5:LINK_ACT2 6:LED_RUN 7:LED_ERR 8:RESET_OUT."]
        #[must_use]
        #[inline(always)]
        pub const fn func_alt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "IO usage: 0:NMII_LINK0 1:NMII_LINK1 2:NMII_LINK2 3:LINK_ACT0 4:LINK_ACT1 5:LINK_ACT2 6:LED_RUN 7:LED_ERR 8:RESET_OUT."]
        #[inline(always)]
        pub const fn set_func_alt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "1:invert the IO."]
        #[must_use]
        #[inline(always)]
        pub const fn invert(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "1:invert the IO."]
        #[inline(always)]
        pub const fn set_invert(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for IoCfg {
        #[inline(always)]
        fn default() -> IoCfg {
            IoCfg(0)
        }
    }
    impl core::fmt::Debug for IoCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IoCfg")
                .field("func_alt", &self.func_alt())
                .field("invert", &self.invert())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IoCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IoCfg {{ func_alt: {=u8:?}, invert: {=bool:?} }}",
                self.func_alt(),
                self.invert()
            )
        }
    }
    #[doc = "Latch0 Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Latch0Ctrl(pub u8);
    impl Latch0Ctrl {
        #[doc = "Latch0 positive edge: 0:Continuous Latch active 1:Single event (only first event active)."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_edge(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Latch0 positive edge: 0:Continuous Latch active 1:Single event (only first event active)."]
        #[inline(always)]
        pub const fn set_pos_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Latch0 negative edge: 0:Continuous Latch active 1:Single event (only first event active)."]
        #[must_use]
        #[inline(always)]
        pub const fn neg_edge(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Latch0 negative edge: 0:Continuous Latch active 1:Single event (only first event active)."]
        #[inline(always)]
        pub const fn set_neg_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for Latch0Ctrl {
        #[inline(always)]
        fn default() -> Latch0Ctrl {
            Latch0Ctrl(0)
        }
    }
    impl core::fmt::Debug for Latch0Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Latch0Ctrl")
                .field("pos_edge", &self.pos_edge())
                .field("neg_edge", &self.neg_edge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Latch0Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Latch0Ctrl {{ pos_edge: {=bool:?}, neg_edge: {=bool:?} }}",
                self.pos_edge(),
                self.neg_edge()
            )
        }
    }
    #[doc = "Latch0 Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Latch0Stat(pub u8);
    impl Latch0Stat {
        #[doc = "Event Latch0 positive edge. 0:Positive edge not detected or continuous mode 1:Positive edge detected in single event mode only. Flag cleared by reading out Latch0 Time Positive Edge."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_edge(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Event Latch0 positive edge. 0:Positive edge not detected or continuous mode 1:Positive edge detected in single event mode only. Flag cleared by reading out Latch0 Time Positive Edge."]
        #[inline(always)]
        pub const fn set_pos_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Event Latch0 negative edge. 0:Negative edge not detected or continuous mode 1:Negative edge detected in single event mode only. Flag cleared by reading out Latch0 Time Negative Edge."]
        #[must_use]
        #[inline(always)]
        pub const fn neg_edge(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Event Latch0 negative edge. 0:Negative edge not detected or continuous mode 1:Negative edge detected in single event mode only. Flag cleared by reading out Latch0 Time Negative Edge."]
        #[inline(always)]
        pub const fn set_neg_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Latch0 pin state."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_stat(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Latch0 pin state."]
        #[inline(always)]
        pub const fn set_pin_stat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for Latch0Stat {
        #[inline(always)]
        fn default() -> Latch0Stat {
            Latch0Stat(0)
        }
    }
    impl core::fmt::Debug for Latch0Stat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Latch0Stat")
                .field("pos_edge", &self.pos_edge())
                .field("neg_edge", &self.neg_edge())
                .field("pin_stat", &self.pin_stat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Latch0Stat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Latch0Stat {{ pos_edge: {=bool:?}, neg_edge: {=bool:?}, pin_stat: {=bool:?} }}",
                self.pos_edge(),
                self.neg_edge(),
                self.pin_stat()
            )
        }
    }
    #[doc = "Latch0 Time Negative Edge."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Latch0TimeNe(pub u64);
    impl Latch0TimeNe {
        #[doc = "System time at the negative edge of the Latch0 signal."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "System time at the negative edge of the Latch0 signal."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for Latch0TimeNe {
        #[inline(always)]
        fn default() -> Latch0TimeNe {
            Latch0TimeNe(0)
        }
    }
    impl core::fmt::Debug for Latch0TimeNe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Latch0TimeNe")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Latch0TimeNe {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Latch0TimeNe {{ time: {=u64:?} }}", self.time())
        }
    }
    #[doc = "Latch0 Time Positive Edge."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Latch0TimePe(pub u64);
    impl Latch0TimePe {
        #[doc = "System time at the positive edge of the Latch0 signal."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "System time at the positive edge of the Latch0 signal."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for Latch0TimePe {
        #[inline(always)]
        fn default() -> Latch0TimePe {
            Latch0TimePe(0)
        }
    }
    impl core::fmt::Debug for Latch0TimePe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Latch0TimePe")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Latch0TimePe {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Latch0TimePe {{ time: {=u64:?} }}", self.time())
        }
    }
    #[doc = "Latch1 Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Latch1Ctrl(pub u8);
    impl Latch1Ctrl {
        #[doc = "Latch1 positive edge: 0:Continuous Latch active 1:Single event (only first event active)."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_edge(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Latch1 positive edge: 0:Continuous Latch active 1:Single event (only first event active)."]
        #[inline(always)]
        pub const fn set_pos_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Latch1 negative edge: 0:Continuous Latch active 1:Single event (only first event active)."]
        #[must_use]
        #[inline(always)]
        pub const fn neg_edge(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Latch1 negative edge: 0:Continuous Latch active 1:Single event (only first event active)."]
        #[inline(always)]
        pub const fn set_neg_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for Latch1Ctrl {
        #[inline(always)]
        fn default() -> Latch1Ctrl {
            Latch1Ctrl(0)
        }
    }
    impl core::fmt::Debug for Latch1Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Latch1Ctrl")
                .field("pos_edge", &self.pos_edge())
                .field("neg_edge", &self.neg_edge())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Latch1Ctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Latch1Ctrl {{ pos_edge: {=bool:?}, neg_edge: {=bool:?} }}",
                self.pos_edge(),
                self.neg_edge()
            )
        }
    }
    #[doc = "Latch1 Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Latch1Stat(pub u8);
    impl Latch1Stat {
        #[doc = "Event Latch1 positive edge. 0:Positive edge not detected or continuous mode 1:Positive edge detected in single event mode only. Flag cleared by reading out Latch1 Time Positive Edge."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_edge(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Event Latch1 positive edge. 0:Positive edge not detected or continuous mode 1:Positive edge detected in single event mode only. Flag cleared by reading out Latch1 Time Positive Edge."]
        #[inline(always)]
        pub const fn set_pos_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Event Latch1 negative edge. 0:Negative edge not detected or continuous mode 1:Negative edge detected in single event mode only. Flag cleared by reading out Latch1 Time Negative Edge."]
        #[must_use]
        #[inline(always)]
        pub const fn neg_edge(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Event Latch1 negative edge. 0:Negative edge not detected or continuous mode 1:Negative edge detected in single event mode only. Flag cleared by reading out Latch1 Time Negative Edge."]
        #[inline(always)]
        pub const fn set_neg_edge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Latch1 pin state."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_stat(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Latch1 pin state."]
        #[inline(always)]
        pub const fn set_pin_stat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for Latch1Stat {
        #[inline(always)]
        fn default() -> Latch1Stat {
            Latch1Stat(0)
        }
    }
    impl core::fmt::Debug for Latch1Stat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Latch1Stat")
                .field("pos_edge", &self.pos_edge())
                .field("neg_edge", &self.neg_edge())
                .field("pin_stat", &self.pin_stat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Latch1Stat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Latch1Stat {{ pos_edge: {=bool:?}, neg_edge: {=bool:?}, pin_stat: {=bool:?} }}",
                self.pos_edge(),
                self.neg_edge(),
                self.pin_stat()
            )
        }
    }
    #[doc = "Latch1 Time Negative Edge."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Latch1TimeNe(pub u64);
    impl Latch1TimeNe {
        #[doc = "System time at the negative edge of the Latch1 signal."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "System time at the negative edge of the Latch1 signal."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for Latch1TimeNe {
        #[inline(always)]
        fn default() -> Latch1TimeNe {
            Latch1TimeNe(0)
        }
    }
    impl core::fmt::Debug for Latch1TimeNe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Latch1TimeNe")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Latch1TimeNe {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Latch1TimeNe {{ time: {=u64:?} }}", self.time())
        }
    }
    #[doc = "Latch1 Time Positive Edge."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Latch1TimePe(pub u64);
    impl Latch1TimePe {
        #[doc = "System time at the positive edge of the Latch1 signal."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "System time at the positive edge of the Latch1 signal."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for Latch1TimePe {
        #[inline(always)]
        fn default() -> Latch1TimePe {
            Latch1TimePe(0)
        }
    }
    impl core::fmt::Debug for Latch1TimePe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Latch1TimePe")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Latch1TimePe {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Latch1TimePe {{ time: {=u64:?} }}", self.time())
        }
    }
    #[doc = "Logical Start Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LogicStartAddr(pub u32);
    impl LogicStartAddr {
        #[doc = "Logical start address within the EtherCAT Address Space."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Logical start address within the EtherCAT Address Space."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LogicStartAddr {
        #[inline(always)]
        fn default() -> LogicStartAddr {
            LogicStartAddr(0)
        }
    }
    impl core::fmt::Debug for LogicStartAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LogicStartAddr")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LogicStartAddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LogicStartAddr {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Logical Start Bit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LogicStartBit(pub u8);
    impl LogicStartBit {
        #[doc = "Logical starting bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7)."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Logical starting bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7)."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
    }
    impl Default for LogicStartBit {
        #[inline(always)]
        fn default() -> LogicStartBit {
            LogicStartBit(0)
        }
    }
    impl core::fmt::Debug for LogicStartBit {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LogicStartBit")
                .field("start", &self.start())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LogicStartBit {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LogicStartBit {{ start: {=u8:?} }}", self.start())
        }
    }
    #[doc = "Logical Stop Bit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LogicStopBit(pub u8);
    impl LogicStopBit {
        #[doc = "Last logical bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7)."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Last logical bit that shall be mapped (bits are counted from least significant bit 0 to most significant bit 7)."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
    }
    impl Default for LogicStopBit {
        #[inline(always)]
        fn default() -> LogicStopBit {
            LogicStopBit(0)
        }
    }
    impl core::fmt::Debug for LogicStopBit {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LogicStopBit")
                .field("stop", &self.stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LogicStopBit {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LogicStopBit {{ stop: {=u8:?} }}", self.stop())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LostLinkCnt(pub u8);
    impl LostLinkCnt {
        #[doc = "Lost Link counter of Port y (counting is stopped when 0xff is reached). Counts only if port is open and loop is Auto."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Lost Link counter of Port y (counting is stopped when 0xff is reached). Counts only if port is open and loop is Auto."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for LostLinkCnt {
        #[inline(always)]
        fn default() -> LostLinkCnt {
            LostLinkCnt(0)
        }
    }
    impl core::fmt::Debug for LostLinkCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LostLinkCnt")
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LostLinkCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LostLinkCnt {{ cnt: {=u8:?} }}", self.cnt())
        }
    }
    #[doc = "MII Management Control/Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MiiMngCs(pub u16);
    impl MiiMngCs {
        #[doc = "Write enable*: 0:Write disabled 1:Write enabled This bit is always 1 if PDI has MI control. ET1100-0000/-0001 exception: Bit is not always 1 if PDI has MI control, and bit is writable by PDI."]
        #[must_use]
        #[inline(always)]
        pub const fn wen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Write enable*: 0:Write disabled 1:Write enabled This bit is always 1 if PDI has MI control. ET1100-0000/-0001 exception: Bit is not always 1 if PDI has MI control, and bit is writable by PDI."]
        #[inline(always)]
        pub const fn set_wen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Management Interface can be controlled by PDI (registers 0x0516-0x0517): 0:Only ECAT control 1:PDI control possible."]
        #[must_use]
        #[inline(always)]
        pub const fn pdi(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Management Interface can be controlled by PDI (registers 0x0516-0x0517): 0:Only ECAT control 1:PDI control possible."]
        #[inline(always)]
        pub const fn set_pdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "MI link detection and configuration: 0:Disabled for all ports 1:Enabled for at least one MII port, refer to PHY Port Status (0x0518 ff.) for details."]
        #[must_use]
        #[inline(always)]
        pub const fn link_dc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MI link detection and configuration: 0:Disabled for all ports 1:Enabled for at least one MII port, refer to PHY Port Status (0x0518 ff.) for details."]
        #[inline(always)]
        pub const fn set_link_dc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "PHY address of port 0 (this is equal to the PHY address offset, if the PHY addresses are consecutive) IP Core since V3.0.0/3.00c: Translation 0x0512\\[7\\]=0: Register 0x0510\\[7:3\\]
shows PHY address of port 0 Translation 0x0512\\[7\\]=1: Register 0x0510\\[7:3\\]
shows the PHY address which will be used for port 0-3 as requested by 0x0512\\[4:0\\]
(valid values 0-3)."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_addr(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "PHY address of port 0 (this is equal to the PHY address offset, if the PHY addresses are consecutive) IP Core since V3.0.0/3.00c: Translation 0x0512\\[7\\]=0: Register 0x0510\\[7:3\\]
shows PHY address of port 0 Translation 0x0512\\[7\\]=1: Register 0x0510\\[7:3\\]
shows the PHY address which will be used for port 0-3 as requested by 0x0512\\[4:0\\]
(valid values 0-3)."]
        #[inline(always)]
        pub const fn set_phy_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
        }
        #[doc = "Command register*: Write:Initiate command. Read:Currently executed command 00:No command/MI idle (clear error bits) 01:Read 10:Write Others:Reserved/invalid command (do not issue)."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Command register*: Write:Initiate command. Read:Currently executed command 00:No command/MI idle (clear error bits) 01:Read 10:Write Others:Reserved/invalid command (do not issue)."]
        #[inline(always)]
        pub const fn set_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[doc = "Read error: 0:No read error 1:Read error occurred (PHY or register not available) Cleared by writing to register 0x0511."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_err(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Read error: 0:No read error 1:Read error occurred (PHY or register not available) Cleared by writing to register 0x0511."]
        #[inline(always)]
        pub const fn set_rd_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "Command error: 0:Last Command was successful 1:Invalid command or write command without Write Enable Cleared by executing a valid command or by writing “00” to Command register bits \\[9:8\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd_err(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Command error: 0:Last Command was successful 1:Invalid command or write command without Write Enable Cleared by executing a valid command or by writing “00” to Command register bits \\[9:8\\]."]
        #[inline(always)]
        pub const fn set_cmd_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "Busy: 0:MII Management Interface is idle 1:MII Management Interface is busy."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Busy: 0:MII Management Interface is idle 1:MII Management Interface is busy."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for MiiMngCs {
        #[inline(always)]
        fn default() -> MiiMngCs {
            MiiMngCs(0)
        }
    }
    impl core::fmt::Debug for MiiMngCs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MiiMngCs")
                .field("wen", &self.wen())
                .field("pdi", &self.pdi())
                .field("link_dc", &self.link_dc())
                .field("phy_addr", &self.phy_addr())
                .field("cmd", &self.cmd())
                .field("rd_err", &self.rd_err())
                .field("cmd_err", &self.cmd_err())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MiiMngCs {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MiiMngCs {{ wen: {=bool:?}, pdi: {=bool:?}, link_dc: {=bool:?}, phy_addr: {=u8:?}, cmd: {=u8:?}, rd_err: {=bool:?}, cmd_err: {=bool:?}, busy: {=bool:?} }}" , self . wen () , self . pdi () , self . link_dc () , self . phy_addr () , self . cmd () , self . rd_err () , self . cmd_err () , self . busy ())
        }
    }
    #[doc = "MII Management ECAT Access State."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MiimEcatAccStat(pub u8);
    impl MiimEcatAccStat {
        #[doc = "Access to MII management: 0:ECAT enables PDI takeover of MII management interface 1:ECAT claims exclusive access to MII management interface."]
        #[must_use]
        #[inline(always)]
        pub const fn acc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Access to MII management: 0:ECAT enables PDI takeover of MII management interface 1:ECAT claims exclusive access to MII management interface."]
        #[inline(always)]
        pub const fn set_acc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for MiimEcatAccStat {
        #[inline(always)]
        fn default() -> MiimEcatAccStat {
            MiimEcatAccStat(0)
        }
    }
    impl core::fmt::Debug for MiimEcatAccStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MiimEcatAccStat")
                .field("acc", &self.acc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MiimEcatAccStat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MiimEcatAccStat {{ acc: {=bool:?} }}", self.acc())
        }
    }
    #[doc = "MII Management PDI Access State."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MiimPdiAccStat(pub u8);
    impl MiimPdiAccStat {
        #[doc = "Access to MII management: 0:ECAT has access to MII management 1:PDI has access to MII management."]
        #[must_use]
        #[inline(always)]
        pub const fn acc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Access to MII management: 0:ECAT has access to MII management 1:PDI has access to MII management."]
        #[inline(always)]
        pub const fn set_acc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Force PDI Access State: 0:Do not change Bit 0x0517\\[0\\]
1:Reset Bit 0x0517\\[0\\]
to 0."]
        #[must_use]
        #[inline(always)]
        pub const fn force(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force PDI Access State: 0:Do not change Bit 0x0517\\[0\\]
1:Reset Bit 0x0517\\[0\\]
to 0."]
        #[inline(always)]
        pub const fn set_force(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for MiimPdiAccStat {
        #[inline(always)]
        fn default() -> MiimPdiAccStat {
            MiimPdiAccStat(0)
        }
    }
    impl core::fmt::Debug for MiimPdiAccStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MiimPdiAccStat")
                .field("acc", &self.acc())
                .field("force", &self.force())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MiimPdiAccStat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MiimPdiAccStat {{ acc: {=bool:?}, force: {=bool:?} }}",
                self.acc(),
                self.force()
            )
        }
    }
    #[doc = "Next SYNC1 Pulse."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NxtSync1Pulse(pub u64);
    impl NxtSync1Pulse {
        #[doc = "System time of next SYNC1 pulse in ns."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "System time of next SYNC1 pulse in ns."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for NxtSync1Pulse {
        #[inline(always)]
        fn default() -> NxtSync1Pulse {
            NxtSync1Pulse(0)
        }
    }
    impl core::fmt::Debug for NxtSync1Pulse {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("NxtSync1Pulse")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for NxtSync1Pulse {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "NxtSync1Pulse {{ time: {=u64:?} }}", self.time())
        }
    }
    #[doc = "PDI AL Event Mask."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdiAlEvtMsk(pub u32);
    impl PdiAlEvtMsk {
        #[doc = "AL Event masking of the AL Event Request register Events for mapping to PDI IRQ signal: 0:Corresponding AL Event Request register bit is not mapped 1:Corresponding AL Event Request register bit is mapped."]
        #[must_use]
        #[inline(always)]
        pub const fn mask(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "AL Event masking of the AL Event Request register Events for mapping to PDI IRQ signal: 0:Corresponding AL Event Request register bit is not mapped 1:Corresponding AL Event Request register bit is mapped."]
        #[inline(always)]
        pub const fn set_mask(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PdiAlEvtMsk {
        #[inline(always)]
        fn default() -> PdiAlEvtMsk {
            PdiAlEvtMsk(0)
        }
    }
    impl core::fmt::Debug for PdiAlEvtMsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdiAlEvtMsk")
                .field("mask", &self.mask())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PdiAlEvtMsk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PdiAlEvtMsk {{ mask: {=u32:?} }}", self.mask())
        }
    }
    #[doc = "PDI Buffer Change Event Time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdiBufCet(pub u32);
    impl PdiBufCet {
        #[doc = "Local time when at least one SyncManager asserts a PDI buffer change event."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Local time when at least one SyncManager asserts a PDI buffer change event."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PdiBufCet {
        #[inline(always)]
        fn default() -> PdiBufCet {
            PdiBufCet(0)
        }
    }
    impl core::fmt::Debug for PdiBufCet {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdiBufCet")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PdiBufCet {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PdiBufCet {{ time: {=u32:?} }}", self.time())
        }
    }
    #[doc = "PDI Buffer Start Event Time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdiBufSet(pub u32);
    impl PdiBufSet {
        #[doc = "Local time when at least one SyncManager asserts a PDI buffer start event."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Local time when at least one SyncManager asserts a PDI buffer start event."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PdiBufSet {
        #[inline(always)]
        fn default() -> PdiBufSet {
            PdiBufSet(0)
        }
    }
    impl core::fmt::Debug for PdiBufSet {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdiBufSet")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PdiBufSet {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PdiBufSet {{ time: {=u32:?} }}", self.time())
        }
    }
    #[doc = "PDI Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdiCfg(pub u8);
    impl PdiCfg {
        #[doc = "On-chip bus clock: 0:asynchronous 1-31:synchronous multiplication factor (N * 25 MHz)."]
        #[must_use]
        #[inline(always)]
        pub const fn clk(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "On-chip bus clock: 0:asynchronous 1-31:synchronous multiplication factor (N * 25 MHz)."]
        #[inline(always)]
        pub const fn set_clk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "On-chip bus: 000:Intel® Avalon® 001:AXI® 010:Xilinx® PLB v4.6 100:Xilinx OPB others:reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn bus(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "On-chip bus: 000:Intel® Avalon® 001:AXI® 010:Xilinx® PLB v4.6 100:Xilinx OPB others:reserved."]
        #[inline(always)]
        pub const fn set_bus(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u8) & 0x07) << 5usize);
        }
    }
    impl Default for PdiCfg {
        #[inline(always)]
        fn default() -> PdiCfg {
            PdiCfg(0)
        }
    }
    impl core::fmt::Debug for PdiCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdiCfg")
                .field("clk", &self.clk())
                .field("bus", &self.bus())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PdiCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PdiCfg {{ clk: {=u8:?}, bus: {=u8:?} }}",
                self.clk(),
                self.bus()
            )
        }
    }
    #[doc = "PDI Error Counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdiErrCnt(pub u8);
    impl PdiErrCnt {
        #[doc = "PDI Error counter (counting is stopped when 0xFF is reached). Counts if a PDI access has an interface error."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "PDI Error counter (counting is stopped when 0xFF is reached). Counts if a PDI access has an interface error."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for PdiErrCnt {
        #[inline(always)]
        fn default() -> PdiErrCnt {
            PdiErrCnt(0)
        }
    }
    impl core::fmt::Debug for PdiErrCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdiErrCnt")
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PdiErrCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PdiErrCnt {{ cnt: {=u8:?} }}", self.cnt())
        }
    }
    #[doc = "PDI Extended Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdiExtCfg(pub u16);
    impl PdiExtCfg {
        #[doc = "Read prefetch size (in cycles of PDI width): 0:4 cycles 1:1 cycle (typical) 2:2 cycles 3:Reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn rps(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Read prefetch size (in cycles of PDI width): 0:4 cycles 1:1 cycle (typical) 2:2 cycles 3:Reserved."]
        #[inline(always)]
        pub const fn set_rps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
        }
        #[doc = "On-chip bus sub-type for AXI: 000:AXI3 001:AXI4 010:AXI4 LITE others:reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn ocbst(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "On-chip bus sub-type for AXI: 000:AXI3 001:AXI4 010:AXI4 LITE others:reserved."]
        #[inline(always)]
        pub const fn set_ocbst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
        }
    }
    impl Default for PdiExtCfg {
        #[inline(always)]
        fn default() -> PdiExtCfg {
            PdiExtCfg(0)
        }
    }
    impl core::fmt::Debug for PdiExtCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdiExtCfg")
                .field("rps", &self.rps())
                .field("ocbst", &self.ocbst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PdiExtCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PdiExtCfg {{ rps: {=u8:?}, ocbst: {=u8:?} }}",
                self.rps(),
                self.ocbst()
            )
        }
    }
    #[doc = "PDI Information."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdiInfo(pub u16);
    impl PdiInfo {
        #[doc = "DI function acknowledge by write: 0:Disabled 1:Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn pfabw(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DI function acknowledge by write: 0:Disabled 1:Enabled."]
        #[inline(always)]
        pub const fn set_pfabw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "ESC configuration area loaded from EEPROM: 0:not loaded 1:loaded."]
        #[must_use]
        #[inline(always)]
        pub const fn eclfe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ESC configuration area loaded from EEPROM: 0:not loaded 1:loaded."]
        #[inline(always)]
        pub const fn set_eclfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "PDI active: 0:PDI not active 1:PDI active."]
        #[must_use]
        #[inline(always)]
        pub const fn pdia(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PDI active: 0:PDI not active 1:PDI active."]
        #[inline(always)]
        pub const fn set_pdia(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "PDI configuration invalid: 0:PDI configuration ok 1:PDI configuration invalid."]
        #[must_use]
        #[inline(always)]
        pub const fn pdicn(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PDI configuration invalid: 0:PDI configuration ok 1:PDI configuration invalid."]
        #[inline(always)]
        pub const fn set_pdicn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
    }
    impl Default for PdiInfo {
        #[inline(always)]
        fn default() -> PdiInfo {
            PdiInfo(0)
        }
    }
    impl core::fmt::Debug for PdiInfo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdiInfo")
                .field("pfabw", &self.pfabw())
                .field("eclfe", &self.eclfe())
                .field("pdia", &self.pdia())
                .field("pdicn", &self.pdicn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PdiInfo {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PdiInfo {{ pfabw: {=bool:?}, eclfe: {=bool:?}, pdia: {=bool:?}, pdicn: {=bool:?} }}" , self . pfabw () , self . eclfe () , self . pdia () , self . pdicn ())
        }
    }
    #[doc = "PDI Sync/Latch\\[1:0\\]
Configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdiSlCfg(pub u8);
    impl PdiSlCfg {
        #[doc = "SYNC0 output driver/polarity: 00:Push-Pull active low 01:Open Drain (active low) 10:Push-Pull active high 11:Open Source (active high)."]
        #[must_use]
        #[inline(always)]
        pub const fn sync0_odp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SYNC0 output driver/polarity: 00:Push-Pull active low 01:Open Drain (active low) 10:Push-Pull active high 11:Open Source (active high)."]
        #[inline(always)]
        pub const fn set_sync0_odp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "SYNC0/LATCH0 configuration*: 0:LATCH0 Input 1:SYNC0 Output."]
        #[must_use]
        #[inline(always)]
        pub const fn sync0_cfg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC0/LATCH0 configuration*: 0:LATCH0 Input 1:SYNC0 Output."]
        #[inline(always)]
        pub const fn set_sync0_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "SYNC0 mapped to AL Event Request register 0x0220\\[2\\]: 0:Disabled 1:Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn sync0_maer(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC0 mapped to AL Event Request register 0x0220\\[2\\]: 0:Disabled 1:Enabled."]
        #[inline(always)]
        pub const fn set_sync0_maer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "SYNC1 output driver/polarity: 00:Push-Pull active low 01:Open Drain (active low) 10:Push-Pull active high 11:Open Source (active high)."]
        #[must_use]
        #[inline(always)]
        pub const fn sync1_odp(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "SYNC1 output driver/polarity: 00:Push-Pull active low 01:Open Drain (active low) 10:Push-Pull active high 11:Open Source (active high)."]
        #[inline(always)]
        pub const fn set_sync1_odp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "SYNC1/LATCH1 configuration*: 0:LATCH1 input 1:SYNC1 output."]
        #[must_use]
        #[inline(always)]
        pub const fn sync1_cfg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC1/LATCH1 configuration*: 0:LATCH1 input 1:SYNC1 output."]
        #[inline(always)]
        pub const fn set_sync1_cfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "SYNC1 mapped to AL Event Request register 0x0220\\[3\\]: 0:Disabled 1:Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn sync1_maer(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC1 mapped to AL Event Request register 0x0220\\[3\\]: 0:Disabled 1:Enabled."]
        #[inline(always)]
        pub const fn set_sync1_maer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for PdiSlCfg {
        #[inline(always)]
        fn default() -> PdiSlCfg {
            PdiSlCfg(0)
        }
    }
    impl core::fmt::Debug for PdiSlCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdiSlCfg")
                .field("sync0_odp", &self.sync0_odp())
                .field("sync0_cfg", &self.sync0_cfg())
                .field("sync0_maer", &self.sync0_maer())
                .field("sync1_odp", &self.sync1_odp())
                .field("sync1_cfg", &self.sync1_cfg())
                .field("sync1_maer", &self.sync1_maer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PdiSlCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PdiSlCfg {{ sync0_odp: {=u8:?}, sync0_cfg: {=bool:?}, sync0_maer: {=bool:?}, sync1_odp: {=u8:?}, sync1_cfg: {=bool:?}, sync1_maer: {=bool:?} }}" , self . sync0_odp () , self . sync0_cfg () , self . sync0_maer () , self . sync1_odp () , self . sync1_cfg () , self . sync1_maer ())
        }
    }
    #[doc = "Process Data Ram."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdram(pub u32);
    impl Pdram {
        #[doc = "Input Data."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Input Data."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pdram {
        #[inline(always)]
        fn default() -> Pdram {
            Pdram(0)
        }
    }
    impl core::fmt::Debug for Pdram {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdram").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdram {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pdram {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "Process Data Ram Alias."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PdramAls(pub u32);
    impl PdramAls {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PdramAls {
        #[inline(always)]
        fn default() -> PdramAls {
            PdramAls(0)
        }
    }
    impl core::fmt::Debug for PdramAls {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PdramAls")
                .field("data", &self.data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PdramAls {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PdramAls {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "PHY Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyAddr(pub u8);
    impl PhyAddr {
        #[doc = "Target PHY Address Translation 0x0512\\[7\\]=0: 0-3:Target PHY Addresses 0-3 are used to access the PHYs at port 0-3, when the PHY addresses are properly configured 4-31:The configured PHY address of port 0 (PHY address offset) is added to the Target PHY Address values 4-31 when accessing a PHY Translation 0x0512\\[7\\]=1: 0-31:Target PHY Addresses is used when accessing a PHY without translation."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Target PHY Address Translation 0x0512\\[7\\]=0: 0-3:Target PHY Addresses 0-3 are used to access the PHYs at port 0-3, when the PHY addresses are properly configured 4-31:The configured PHY address of port 0 (PHY address offset) is added to the Target PHY Address values 4-31 when accessing a PHY Translation 0x0512\\[7\\]=1: 0-31:Target PHY Addresses is used when accessing a PHY without translation."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "Target PHY Address translation: 0:Enabled 1:Disabled Refer to 0x0512\\[4:0\\]
and 0x0510\\[7:3\\]
for details."]
        #[must_use]
        #[inline(always)]
        pub const fn show(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Target PHY Address translation: 0:Enabled 1:Disabled Refer to 0x0512\\[4:0\\]
and 0x0510\\[7:3\\]
for details."]
        #[inline(always)]
        pub const fn set_show(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for PhyAddr {
        #[inline(always)]
        fn default() -> PhyAddr {
            PhyAddr(0)
        }
    }
    impl core::fmt::Debug for PhyAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyAddr")
                .field("addr", &self.addr())
                .field("show", &self.show())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyAddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PhyAddr {{ addr: {=u8:?}, show: {=bool:?} }}",
                self.addr(),
                self.show()
            )
        }
    }
    #[doc = "PHY Configure 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyCfg0(pub u32);
    impl PhyCfg0 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn port0_rmii_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_port0_rmii_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn port1_rmii_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_port1_rmii_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn port2_rmii_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_port2_rmii_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "1:100M."]
        #[must_use]
        #[inline(always)]
        pub const fn mac_speed(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "1:100M."]
        #[inline(always)]
        pub const fn set_mac_speed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for PhyCfg0 {
        #[inline(always)]
        fn default() -> PhyCfg0 {
            PhyCfg0(0)
        }
    }
    impl core::fmt::Debug for PhyCfg0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyCfg0")
                .field("port0_rmii_en", &self.port0_rmii_en())
                .field("port1_rmii_en", &self.port1_rmii_en())
                .field("port2_rmii_en", &self.port2_rmii_en())
                .field("mac_speed", &self.mac_speed())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyCfg0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PhyCfg0 {{ port0_rmii_en: {=bool:?}, port1_rmii_en: {=bool:?}, port2_rmii_en: {=bool:?}, mac_speed: {=bool:?} }}" , self . port0_rmii_en () , self . port1_rmii_en () , self . port2_rmii_en () , self . mac_speed ())
        }
    }
    #[doc = "PHY Configure 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyCfg1(pub u32);
    impl PhyCfg1 {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn rmii_p0_txck_refclk_oe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_rmii_p0_txck_refclk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn rmii_p1_txck_refclk_oe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_rmii_p1_txck_refclk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn rmii_p2_txck_refclk_oe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_rmii_p2_txck_refclk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn refck_25m_oe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_refck_25m_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn rmii_p0_rxck_refclk_oe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_rmii_p0_rxck_refclk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn rmii_p1_rxck_refclk_oe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_rmii_p1_rxck_refclk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn rmii_p2_rxck_refclk_oe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_rmii_p2_rxck_refclk_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn refck_25m_inv(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_refck_25m_inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "0:use RXCK as 50M refclk. 1:use TXCK as 50M refclk."]
        #[must_use]
        #[inline(always)]
        pub const fn rmii_refclk_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "0:use RXCK as 50M refclk. 1:use TXCK as 50M refclk."]
        #[inline(always)]
        pub const fn set_rmii_refclk_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
    }
    impl Default for PhyCfg1 {
        #[inline(always)]
        fn default() -> PhyCfg1 {
            PhyCfg1(0)
        }
    }
    impl core::fmt::Debug for PhyCfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyCfg1")
                .field("rmii_p0_txck_refclk_oe", &self.rmii_p0_txck_refclk_oe())
                .field("rmii_p1_txck_refclk_oe", &self.rmii_p1_txck_refclk_oe())
                .field("rmii_p2_txck_refclk_oe", &self.rmii_p2_txck_refclk_oe())
                .field("refck_25m_oe", &self.refck_25m_oe())
                .field("rmii_p0_rxck_refclk_oe", &self.rmii_p0_rxck_refclk_oe())
                .field("rmii_p1_rxck_refclk_oe", &self.rmii_p1_rxck_refclk_oe())
                .field("rmii_p2_rxck_refclk_oe", &self.rmii_p2_rxck_refclk_oe())
                .field("refck_25m_inv", &self.refck_25m_inv())
                .field("rmii_refclk_sel", &self.rmii_refclk_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyCfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PhyCfg1 {{ rmii_p0_txck_refclk_oe: {=bool:?}, rmii_p1_txck_refclk_oe: {=bool:?}, rmii_p2_txck_refclk_oe: {=bool:?}, refck_25m_oe: {=bool:?}, rmii_p0_rxck_refclk_oe: {=bool:?}, rmii_p1_rxck_refclk_oe: {=bool:?}, rmii_p2_rxck_refclk_oe: {=bool:?}, refck_25m_inv: {=bool:?}, rmii_refclk_sel: {=u8:?} }}" , self . rmii_p0_txck_refclk_oe () , self . rmii_p1_txck_refclk_oe () , self . rmii_p2_txck_refclk_oe () , self . refck_25m_oe () , self . rmii_p0_rxck_refclk_oe () , self . rmii_p1_rxck_refclk_oe () , self . rmii_p2_rxck_refclk_oe () , self . refck_25m_inv () , self . rmii_refclk_sel ())
        }
    }
    #[doc = "PHY Data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyData(pub u16);
    impl PhyData {
        #[doc = "PHY Read/Write Data."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "PHY Read/Write Data."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PhyData {
        #[inline(always)]
        fn default() -> PhyData {
            PhyData(0)
        }
    }
    impl core::fmt::Debug for PhyData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyData")
                .field("data", &self.data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyData {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PhyData {{ data: {=u16:?} }}", self.data())
        }
    }
    #[doc = "PHY Register Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyRegAddr(pub u8);
    impl PhyRegAddr {
        #[doc = "Address of PHY Register that shall be read/written."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Address of PHY Register that shall be read/written."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for PhyRegAddr {
        #[inline(always)]
        fn default() -> PhyRegAddr {
            PhyRegAddr(0)
        }
    }
    impl core::fmt::Debug for PhyRegAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyRegAddr")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyRegAddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PhyRegAddr {{ addr: {=u8:?} }}", self.addr())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyStat(pub u8);
    impl PhyStat {
        #[doc = "Physical link status (PHY status register 1.2): 0:No physical link 1:Physical link detected."]
        #[must_use]
        #[inline(always)]
        pub const fn pls(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Physical link status (PHY status register 1.2): 0:No physical link 1:Physical link detected."]
        #[inline(always)]
        pub const fn set_pls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Link status (100 Mbit/s, Full Duplex, Auto negotiation): 0:No link 1:Link detected."]
        #[must_use]
        #[inline(always)]
        pub const fn ls(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link status (100 Mbit/s, Full Duplex, Auto negotiation): 0:No link 1:Link detected."]
        #[inline(always)]
        pub const fn set_ls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Link status error: 0:No error 1:Link error, link inhibited."]
        #[must_use]
        #[inline(always)]
        pub const fn lse(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Link status error: 0:No error 1:Link error, link inhibited."]
        #[inline(always)]
        pub const fn set_lse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Read error: 0:No read error occurred 1:A read error has occurred Cleared by writing any value to at least one of the PHY Port y Status registers."]
        #[must_use]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Read error: 0:No read error occurred 1:A read error has occurred Cleared by writing any value to at least one of the PHY Port y Status registers."]
        #[inline(always)]
        pub const fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Link partner error: 0:No error detected 1:Link partner error."]
        #[must_use]
        #[inline(always)]
        pub const fn lpe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Link partner error: 0:No error detected 1:Link partner error."]
        #[inline(always)]
        pub const fn set_lpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "PHY configuration updated: 0:No update 1:PHY configuration was updated Cleared by writing any value to at least one of the PHY Port y Status registers."]
        #[must_use]
        #[inline(always)]
        pub const fn pcu(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PHY configuration updated: 0:No update 1:PHY configuration was updated Cleared by writing any value to at least one of the PHY Port y Status registers."]
        #[inline(always)]
        pub const fn set_pcu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for PhyStat {
        #[inline(always)]
        fn default() -> PhyStat {
            PhyStat(0)
        }
    }
    impl core::fmt::Debug for PhyStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyStat")
                .field("pls", &self.pls())
                .field("ls", &self.ls())
                .field("lse", &self.lse())
                .field("re", &self.re())
                .field("lpe", &self.lpe())
                .field("pcu", &self.pcu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhyStat {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PhyStat {{ pls: {=bool:?}, ls: {=bool:?}, lse: {=bool:?}, re: {=bool:?}, lpe: {=bool:?}, pcu: {=bool:?} }}" , self . pls () , self . ls () , self . lse () , self . re () , self . lpe () , self . pcu ())
        }
    }
    #[doc = "Physical Read/Write Offset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhysicalRwOffset(pub u16);
    impl PhysicalRwOffset {
        #[doc = "This register is used for ReadWrite commands in Device Addressing mode (FPRW, APRW, BRW). The internal read address is directly taken from the offset address field of the EtherCAT datagram header, while the internal write address is calculated by adding the Physical Read/Write Offset value to the offset address field. Internal read address = ADR, internal write address = ADR + R/W-Offset."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "This register is used for ReadWrite commands in Device Addressing mode (FPRW, APRW, BRW). The internal read address is directly taken from the offset address field of the EtherCAT datagram header, while the internal write address is calculated by adding the Physical Read/Write Offset value to the offset address field. Internal read address = ADR, internal write address = ADR + R/W-Offset."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PhysicalRwOffset {
        #[inline(always)]
        fn default() -> PhysicalRwOffset {
            PhysicalRwOffset(0)
        }
    }
    impl core::fmt::Debug for PhysicalRwOffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhysicalRwOffset")
                .field("offset", &self.offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhysicalRwOffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PhysicalRwOffset {{ offset: {=u16:?} }}", self.offset())
        }
    }
    #[doc = "Physical Start Bit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhysicalStartBit(pub u8);
    impl PhysicalStartBit {
        #[doc = "Physical starting bit as target of logical start bit mapping (bits are counted from least significant bit 0 to most significant bit 7)."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Physical starting bit as target of logical start bit mapping (bits are counted from least significant bit 0 to most significant bit 7)."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
    }
    impl Default for PhysicalStartBit {
        #[inline(always)]
        fn default() -> PhysicalStartBit {
            PhysicalStartBit(0)
        }
    }
    impl core::fmt::Debug for PhysicalStartBit {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhysicalStartBit")
                .field("start", &self.start())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PhysicalStartBit {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PhysicalStartBit {{ start: {=u8:?} }}", self.start())
        }
    }
    #[doc = "Product ID."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pid(pub u64);
    impl Pid {
        #[doc = "Product ID."]
        #[must_use]
        #[inline(always)]
        pub const fn pid(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "Product ID."]
        #[inline(always)]
        pub const fn set_pid(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for Pid {
        #[inline(always)]
        fn default() -> Pid {
            Pid(0)
        }
    }
    impl core::fmt::Debug for Pid {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pid").field("pid", &self.pid()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pid {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pid {{ pid: {=u64:?} }}", self.pid())
        }
    }
    #[doc = "Port Descriptor."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortDesc(pub u8);
    impl PortDesc {
        #[doc = "Port configuration: 00:Not implemented 01:Not configured (SII EEPROM) 10:EBUS 11:MII/RMII/RGMII."]
        #[must_use]
        #[inline(always)]
        pub const fn port0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Port configuration: 00:Not implemented 01:Not configured (SII EEPROM) 10:EBUS 11:MII/RMII/RGMII."]
        #[inline(always)]
        pub const fn set_port0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "Port configuration: 00:Not implemented 01:Not configured (SII EEPROM) 10:EBUS 11:MII/RMII/RGMII."]
        #[must_use]
        #[inline(always)]
        pub const fn port1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Port configuration: 00:Not implemented 01:Not configured (SII EEPROM) 10:EBUS 11:MII/RMII/RGMII."]
        #[inline(always)]
        pub const fn set_port1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "Port configuration: 00:Not implemented 01:Not configured (SII EEPROM) 10:EBUS 11:MII/RMII/RGMII."]
        #[must_use]
        #[inline(always)]
        pub const fn port2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Port configuration: 00:Not implemented 01:Not configured (SII EEPROM) 10:EBUS 11:MII/RMII/RGMII."]
        #[inline(always)]
        pub const fn set_port2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "Port configuration: 00:Not implemented 01:Not configured (SII EEPROM) 10:EBUS 11:MII/RMII/RGMII."]
        #[must_use]
        #[inline(always)]
        pub const fn port3(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Port configuration: 00:Not implemented 01:Not configured (SII EEPROM) 10:EBUS 11:MII/RMII/RGMII."]
        #[inline(always)]
        pub const fn set_port3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for PortDesc {
        #[inline(always)]
        fn default() -> PortDesc {
            PortDesc(0)
        }
    }
    impl core::fmt::Debug for PortDesc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortDesc")
                .field("port0", &self.port0())
                .field("port1", &self.port1())
                .field("port2", &self.port2())
                .field("port3", &self.port3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PortDesc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PortDesc {{ port0: {=u8:?}, port1: {=u8:?}, port2: {=u8:?}, port3: {=u8:?} }}",
                self.port0(),
                self.port1(),
                self.port2(),
                self.port3()
            )
        }
    }
    #[doc = "Pulse Length of SyncSignals."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PulseLen(pub u16);
    impl PulseLen {
        #[doc = "Pulse length of SyncSignals (in Units of 10ns) 0:Acknowledge mode:SyncSignal will be cleared by reading SYNC\\[1:0\\]
Status register."]
        #[must_use]
        #[inline(always)]
        pub const fn len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Pulse length of SyncSignals (in Units of 10ns) 0:Acknowledge mode:SyncSignal will be cleared by reading SYNC\\[1:0\\]
Status register."]
        #[inline(always)]
        pub const fn set_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for PulseLen {
        #[inline(always)]
        fn default() -> PulseLen {
            PulseLen(0)
        }
    }
    impl core::fmt::Debug for PulseLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PulseLen")
                .field("len", &self.len())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PulseLen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PulseLen {{ len: {=u16:?} }}", self.len())
        }
    }
    #[doc = "RAM Size."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RamSize(pub u8);
    impl RamSize {
        #[doc = "Process Data RAM size supported in KByte."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Process Data RAM size supported in KByte."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for RamSize {
        #[inline(always)]
        fn default() -> RamSize {
            RamSize(0)
        }
    }
    impl core::fmt::Debug for RamSize {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RamSize")
                .field("size", &self.size())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RamSize {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RamSize {{ size: {=u8:?} }}", self.size())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RcvTime(pub u32);
    impl RcvTime {
        #[doc = "Write: A write access to register 0x0900 with BWR or FPWR latches the local time at the beginning of the receive frame (start first bit of preamble) at each port. Write (ESC20, ET1200 exception): A write access latches the local time at the beginning of the receive frame at port 0. It enables the time stamping at the other ports. Read: Local time at the beginning of the last receive frame containing a write access to this register. NOTE:FPWR requires an address match for accessing this register like any FPWR command. All write commands with address match will increment the working counter (e.g., APWR), but they will not trigger receive time latching."]
        #[must_use]
        #[inline(always)]
        pub const fn req(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Write: A write access to register 0x0900 with BWR or FPWR latches the local time at the beginning of the receive frame (start first bit of preamble) at each port. Write (ESC20, ET1200 exception): A write access latches the local time at the beginning of the receive frame at port 0. It enables the time stamping at the other ports. Read: Local time at the beginning of the last receive frame containing a write access to this register. NOTE:FPWR requires an address match for accessing this register like any FPWR command. All write commands with address match will increment the working counter (e.g., APWR), but they will not trigger receive time latching."]
        #[inline(always)]
        pub const fn set_req(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Local time at the beginning of the last receive frame containing a write access to register 0x0900."]
        #[must_use]
        #[inline(always)]
        pub const fn lt(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Local time at the beginning of the last receive frame containing a write access to register 0x0900."]
        #[inline(always)]
        pub const fn set_lt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for RcvTime {
        #[inline(always)]
        fn default() -> RcvTime {
            RcvTime(0)
        }
    }
    impl core::fmt::Debug for RcvTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RcvTime")
                .field("req", &self.req())
                .field("lt", &self.lt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RcvTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RcvTime {{ req: {=u8:?}, lt: {=u32:?} }}",
                self.req(),
                self.lt()
            )
        }
    }
    #[doc = "Receive Time Latch Mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RcvTimeLm(pub u8);
    impl RcvTimeLm {
        #[doc = "Receive Time Latch Mode: 0:Forwarding mode (used if frames are entering the ESC at port 0 first): Receive time stamps of ports 1-3 are enabled after the write access to 0x0900, so the following frame at ports 1-3 will be time stamped (this is typically the write frame to 0x0900 coming back from the network behind the ESC). 1:Reverse mode (used if frames are entering ESC at port 1-3 first): Receive time stamps of ports 1-3 are immediately taken over from the internal hidden time stamp registers, so the previous frame entering the ESC at ports 1-3 will be time stamped when the write frame to 0x0900 enters port 0 (the previous frame at ports 1-3 is typically the write frame to 0x0900 coming from the master, which will enable time stamp."]
        #[must_use]
        #[inline(always)]
        pub const fn latch_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Receive Time Latch Mode: 0:Forwarding mode (used if frames are entering the ESC at port 0 first): Receive time stamps of ports 1-3 are enabled after the write access to 0x0900, so the following frame at ports 1-3 will be time stamped (this is typically the write frame to 0x0900 coming back from the network behind the ESC). 1:Reverse mode (used if frames are entering ESC at port 1-3 first): Receive time stamps of ports 1-3 are immediately taken over from the internal hidden time stamp registers, so the previous frame entering the ESC at ports 1-3 will be time stamped when the write frame to 0x0900 enters port 0 (the previous frame at ports 1-3 is typically the write frame to 0x0900 coming from the master, which will enable time stamp."]
        #[inline(always)]
        pub const fn set_latch_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for RcvTimeLm {
        #[inline(always)]
        fn default() -> RcvTimeLm {
            RcvTimeLm(0)
        }
    }
    impl core::fmt::Debug for RcvTimeLm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RcvTimeLm")
                .field("latch_mode", &self.latch_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RcvTimeLm {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RcvTimeLm {{ latch_mode: {=bool:?} }}",
                self.latch_mode()
            )
        }
    }
    #[doc = "Receive Time ECAT Processing Unit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RcvtEcatPu(pub u64);
    impl RcvtEcatPu {
        #[doc = "Local time at the beginning of a frame (start first bit of preamble) received at the ECAT Processing Unit containing a write access to register 0x0900 NOTE:E.g., if port 0 is open, this register reflects the Receive Time Port 0 as a 64 Bit value. Any valid EtherCAT write access to register 0x0900 triggers latching, not only BWR/FPWR commands as with register 0x0900."]
        #[must_use]
        #[inline(always)]
        pub const fn lt(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "Local time at the beginning of a frame (start first bit of preamble) received at the ECAT Processing Unit containing a write access to register 0x0900 NOTE:E.g., if port 0 is open, this register reflects the Receive Time Port 0 as a 64 Bit value. Any valid EtherCAT write access to register 0x0900 triggers latching, not only BWR/FPWR commands as with register 0x0900."]
        #[inline(always)]
        pub const fn set_lt(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for RcvtEcatPu {
        #[inline(always)]
        fn default() -> RcvtEcatPu {
            RcvtEcatPu(0)
        }
    }
    impl core::fmt::Debug for RcvtEcatPu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RcvtEcatPu")
                .field("lt", &self.lt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RcvtEcatPu {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RcvtEcatPu {{ lt: {=u64:?} }}", self.lt())
        }
    }
    #[doc = "Register Write Enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RegWen(pub u8);
    impl RegWen {
        #[doc = "If register write protection is enabled, this register has to be written in the same Ethernet frame (value does not matter) before other writes to this station are allowed. This bit is self-clearing at the beginning of the next frame (SOF), or if Register Write Protection is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "If register write protection is enabled, this register has to be written in the same Ethernet frame (value does not matter) before other writes to this station are allowed. This bit is self-clearing at the beginning of the next frame (SOF), or if Register Write Protection is disabled."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for RegWen {
        #[inline(always)]
        fn default() -> RegWen {
            RegWen(0)
        }
    }
    impl core::fmt::Debug for RegWen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RegWen").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RegWen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RegWen {{ en: {=bool:?} }}", self.en())
        }
    }
    #[doc = "Register Write Protection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RegWp(pub u8);
    impl RegWp {
        #[doc = "Register write protection: 0:Protection disabled 1:Protection enabled Registers 0x0000:0x0F7F are write-protected, except for 0x0020 and 0x0030."]
        #[must_use]
        #[inline(always)]
        pub const fn wp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Register write protection: 0:Protection disabled 1:Protection enabled Registers 0x0000:0x0F7F are write-protected, except for 0x0020 and 0x0030."]
        #[inline(always)]
        pub const fn set_wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for RegWp {
        #[inline(always)]
        fn default() -> RegWp {
            RegWp(0)
        }
    }
    impl core::fmt::Debug for RegWp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RegWp").field("wp", &self.wp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RegWp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RegWp {{ wp: {=bool:?} }}", self.wp())
        }
    }
    #[doc = "Revision of EtherCAT controller."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Revision(pub u8);
    impl Revision {
        #[doc = "major version X."]
        #[must_use]
        #[inline(always)]
        pub const fn x(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "major version X."]
        #[inline(always)]
        pub const fn set_x(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Revision {
        #[inline(always)]
        fn default() -> Revision {
            Revision(0)
        }
    }
    impl core::fmt::Debug for Revision {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Revision").field("x", &self.x()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Revision {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Revision {{ x: {=u8:?} }}", self.x())
        }
    }
    #[doc = "RUN LED Override."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RunLedOvrd(pub u8);
    impl RunLedOvrd {
        #[doc = "LED code: 0x0:Off 0x1:Flash 1x 0x2-0xC:Flash 2x – 12x 0xD:Blinking 0xE:Flickering 0xF:On."]
        #[must_use]
        #[inline(always)]
        pub const fn led_code(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "LED code: 0x0:Off 0x1:Flash 1x 0x2-0xC:Flash 2x – 12x 0xD:Blinking 0xE:Flickering 0xF:On."]
        #[inline(always)]
        pub const fn set_led_code(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "Enable Override: 0:Override disabled 1:Override enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn en_ovrd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Override: 0:Override disabled 1:Override enabled."]
        #[inline(always)]
        pub const fn set_en_ovrd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for RunLedOvrd {
        #[inline(always)]
        fn default() -> RunLedOvrd {
            RunLedOvrd(0)
        }
    }
    impl core::fmt::Debug for RunLedOvrd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RunLedOvrd")
                .field("led_code", &self.led_code())
                .field("en_ovrd", &self.en_ovrd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RunLedOvrd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RunLedOvrd {{ led_code: {=u8:?}, en_ovrd: {=bool:?} }}",
                self.led_code(),
                self.en_ovrd()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxErrCnt(pub u16);
    impl RxErrCnt {
        #[doc = "Invalid frame counter of Port y (counting is stopped when 0xFF is reached)."]
        #[must_use]
        #[inline(always)]
        pub const fn ivd_frm(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Invalid frame counter of Port y (counting is stopped when 0xFF is reached)."]
        #[inline(always)]
        pub const fn set_ivd_frm(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[doc = "RX Error counter of Port y (counting is stopped when 0xFF is reached)."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_err(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "RX Error counter of Port y (counting is stopped when 0xFF is reached)."]
        #[inline(always)]
        pub const fn set_rx_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for RxErrCnt {
        #[inline(always)]
        fn default() -> RxErrCnt {
            RxErrCnt(0)
        }
    }
    impl core::fmt::Debug for RxErrCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxErrCnt")
                .field("ivd_frm", &self.ivd_frm())
                .field("rx_err", &self.rx_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RxErrCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RxErrCnt {{ ivd_frm: {=u8:?}, rx_err: {=u8:?} }}",
                self.ivd_frm(),
                self.rx_err()
            )
        }
    }
    #[doc = "Speed Counter Diff."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpdCntDiff(pub u16);
    impl SpdCntDiff {
        #[doc = "Representation of the deviation between local clock period and Reference Clock's clock period (representation:two's complement) Range:±(Speed Counter Start – 0x7F)."]
        #[must_use]
        #[inline(always)]
        pub const fn diff(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Representation of the deviation between local clock period and Reference Clock's clock period (representation:two's complement) Range:±(Speed Counter Start – 0x7F)."]
        #[inline(always)]
        pub const fn set_diff(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for SpdCntDiff {
        #[inline(always)]
        fn default() -> SpdCntDiff {
            SpdCntDiff(0)
        }
    }
    impl core::fmt::Debug for SpdCntDiff {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SpdCntDiff")
                .field("diff", &self.diff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SpdCntDiff {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SpdCntDiff {{ diff: {=u16:?} }}", self.diff())
        }
    }
    #[doc = "Speed Counter Filter Depth."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpdCntFd(pub u8);
    impl SpdCntFd {
        #[doc = "Filter depth for averaging the clock period deviation IP Core since V2.2.0/V2.02a: A write access resets the internal speed counter filter."]
        #[must_use]
        #[inline(always)]
        pub const fn depth(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Filter depth for averaging the clock period deviation IP Core since V2.2.0/V2.02a: A write access resets the internal speed counter filter."]
        #[inline(always)]
        pub const fn set_depth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for SpdCntFd {
        #[inline(always)]
        fn default() -> SpdCntFd {
            SpdCntFd(0)
        }
    }
    impl core::fmt::Debug for SpdCntFd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SpdCntFd")
                .field("depth", &self.depth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SpdCntFd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SpdCntFd {{ depth: {=u8:?} }}", self.depth())
        }
    }
    #[doc = "Speed Counter Start."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpdCntStart(pub u16);
    impl SpdCntStart {
        #[doc = "Bandwidth for adjustment of local copy of System Time (larger values → smaller bandwidth and smoother adjustment) A write access resets System Time Difference (0x092C:0x092F) and Speed Counter Diff (0x0932:0x0933). Valid values:0x0080 to 0x3FFF."]
        #[must_use]
        #[inline(always)]
        pub const fn bw(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x7fff;
            val as u16
        }
        #[doc = "Bandwidth for adjustment of local copy of System Time (larger values → smaller bandwidth and smoother adjustment) A write access resets System Time Difference (0x092C:0x092F) and Speed Counter Diff (0x0932:0x0933). Valid values:0x0080 to 0x3FFF."]
        #[inline(always)]
        pub const fn set_bw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
        }
    }
    impl Default for SpdCntStart {
        #[inline(always)]
        fn default() -> SpdCntStart {
            SpdCntStart(0)
        }
    }
    impl core::fmt::Debug for SpdCntStart {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SpdCntStart")
                .field("bw", &self.bw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SpdCntStart {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SpdCntStart {{ bw: {=u16:?} }}", self.bw())
        }
    }
    #[doc = "Start Time Cyclic Operation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StartTimeCo(pub u64);
    impl StartTimeCo {
        #[doc = "Write:Start time (System time) of cyclic operation in ns Read:System time of next SYNC0 pulse in ns."]
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "Write:Start time (System time) of cyclic operation in ns Read:System time of next SYNC0 pulse in ns."]
        #[inline(always)]
        pub const fn set_st(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for StartTimeCo {
        #[inline(always)]
        fn default() -> StartTimeCo {
            StartTimeCo(0)
        }
    }
    impl core::fmt::Debug for StartTimeCo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("StartTimeCo")
                .field("st", &self.st())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for StartTimeCo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "StartTimeCo {{ st: {=u64:?} }}", self.st())
        }
    }
    #[doc = "Configured Station Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StationAddr(pub u16);
    impl StationAddr {
        #[doc = "Address used for node addressing (FPRD/FPWR/FPRW/FRMW commands)."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Address used for node addressing (FPRD/FPWR/FPRW/FRMW commands)."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for StationAddr {
        #[inline(always)]
        fn default() -> StationAddr {
            StationAddr(0)
        }
    }
    impl core::fmt::Debug for StationAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("StationAddr")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for StationAddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "StationAddr {{ addr: {=u16:?} }}", self.addr())
        }
    }
    #[doc = "Configured Station Alias."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StationAls(pub u16);
    impl StationAls {
        #[doc = "Alias Address used for node addressing (FPRD/FPWR/FPRW/FRMW commands). The use of this alias is activated by Register DL Control Bit 0x0100\\[24\\]. NOTE:EEPROM value is only transferred into this register at first EEPROM load after power-on or reset. ESC20 exception:EEPROM value is transferred into this register after each EEPROM reload command."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Alias Address used for node addressing (FPRD/FPWR/FPRW/FRMW commands). The use of this alias is activated by Register DL Control Bit 0x0100\\[24\\]. NOTE:EEPROM value is only transferred into this register at first EEPROM load after power-on or reset. ESC20 exception:EEPROM value is transferred into this register after each EEPROM reload command."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for StationAls {
        #[inline(always)]
        fn default() -> StationAls {
            StationAls(0)
        }
    }
    impl core::fmt::Debug for StationAls {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("StationAls")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for StationAls {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "StationAls {{ addr: {=u16:?} }}", self.addr())
        }
    }
    #[doc = "Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u8);
    impl Status {
        #[doc = "Interrupt Write: 1:Interrupt after buffer was completely and successfully written 0:Interrupt cleared after first byte of buffer was read NOTE:This interrupt is signalled to the reading side if enabled in the SM Control register."]
        #[must_use]
        #[inline(always)]
        pub const fn int_wr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Write: 1:Interrupt after buffer was completely and successfully written 0:Interrupt cleared after first byte of buffer was read NOTE:This interrupt is signalled to the reading side if enabled in the SM Control register."]
        #[inline(always)]
        pub const fn set_int_wr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Interrupt Read: 1:Interrupt after buffer was completely and successfully read 0:Interrupt cleared after first byte of buffer was written NOTE:This interrupt is signalled to the writing side if enabled in the SM Control register."]
        #[must_use]
        #[inline(always)]
        pub const fn int_rd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Read: 1:Interrupt after buffer was completely and successfully read 0:Interrupt cleared after first byte of buffer was written NOTE:This interrupt is signalled to the writing side if enabled in the SM Control register."]
        #[inline(always)]
        pub const fn set_int_rd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Mailbox mode:mailbox status: 0:Mailbox empty 1:Mailbox full Buffered mode:reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn mbx_mode(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Mailbox mode:mailbox status: 0:Mailbox empty 1:Mailbox full Buffered mode:reserved."]
        #[inline(always)]
        pub const fn set_mbx_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Buffered mode:buffer status (last written buffer): 00:1 st buffer 01:2 nd buffer 10:3 rd buffer 11:(no buffer written) Mailbox mode:reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn buf_mode(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Buffered mode:buffer status (last written buffer): 00:1 st buffer 01:2 nd buffer 10:3 rd buffer 11:(no buffer written) Mailbox mode:reserved."]
        #[inline(always)]
        pub const fn set_buf_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "Read buffer in use (opened)."]
        #[must_use]
        #[inline(always)]
        pub const fn rb_inuse(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Read buffer in use (opened)."]
        #[inline(always)]
        pub const fn set_rb_inuse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Write buffer in use (opened)."]
        #[must_use]
        #[inline(always)]
        pub const fn wb_inuse(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Write buffer in use (opened)."]
        #[inline(always)]
        pub const fn set_wb_inuse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("int_wr", &self.int_wr())
                .field("int_rd", &self.int_rd())
                .field("mbx_mode", &self.mbx_mode())
                .field("buf_mode", &self.buf_mode())
                .field("rb_inuse", &self.rb_inuse())
                .field("wb_inuse", &self.wb_inuse())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Status {{ int_wr: {=bool:?}, int_rd: {=bool:?}, mbx_mode: {=bool:?}, buf_mode: {=u8:?}, rb_inuse: {=bool:?}, wb_inuse: {=bool:?} }}" , self . int_wr () , self . int_rd () , self . mbx_mode () , self . buf_mode () , self . rb_inuse () , self . wb_inuse ())
        }
    }
    #[doc = "SYNC0 Cycle Time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sync0CycTime(pub u32);
    impl Sync0CycTime {
        #[doc = "Time between two consecutive SYNC0 pulses in ns. 0:Single shot mode, generate only one SYNC0 pulse."]
        #[must_use]
        #[inline(always)]
        pub const fn cyc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Time between two consecutive SYNC0 pulses in ns. 0:Single shot mode, generate only one SYNC0 pulse."]
        #[inline(always)]
        pub const fn set_cyc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sync0CycTime {
        #[inline(always)]
        fn default() -> Sync0CycTime {
            Sync0CycTime(0)
        }
    }
    impl core::fmt::Debug for Sync0CycTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sync0CycTime")
                .field("cyc", &self.cyc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sync0CycTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sync0CycTime {{ cyc: {=u32:?} }}", self.cyc())
        }
    }
    #[doc = "SYNC0 Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sync0Stat(pub u8);
    impl Sync0Stat {
        #[doc = "SYNC0 state for Acknowledge mode. SYNC0 in Acknowledge mode is cleared by reading this register from PDI, use only in Acknowledge mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC0 state for Acknowledge mode. SYNC0 in Acknowledge mode is cleared by reading this register from PDI, use only in Acknowledge mode."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Sync0Stat {
        #[inline(always)]
        fn default() -> Sync0Stat {
            Sync0Stat(0)
        }
    }
    impl core::fmt::Debug for Sync0Stat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sync0Stat")
                .field("ack", &self.ack())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sync0Stat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sync0Stat {{ ack: {=bool:?} }}", self.ack())
        }
    }
    #[doc = "SYNC1 Cycle Time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sync1CycTime(pub u32);
    impl Sync1CycTime {
        #[doc = "Time between SYNC0 pulse and SYNC1 pulse in ns."]
        #[must_use]
        #[inline(always)]
        pub const fn cyc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Time between SYNC0 pulse and SYNC1 pulse in ns."]
        #[inline(always)]
        pub const fn set_cyc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sync1CycTime {
        #[inline(always)]
        fn default() -> Sync1CycTime {
            Sync1CycTime(0)
        }
    }
    impl core::fmt::Debug for Sync1CycTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sync1CycTime")
                .field("cyc", &self.cyc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sync1CycTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sync1CycTime {{ cyc: {=u32:?} }}", self.cyc())
        }
    }
    #[doc = "SYNC1 Status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sync1Stat(pub u8);
    impl Sync1Stat {
        #[doc = "SYNC1 state for Acknowledge mode. SYNC1 in Acknowledge mode is cleared by reading this register from PDI, use only in Acknowledge mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC1 state for Acknowledge mode. SYNC1 in Acknowledge mode is cleared by reading this register from PDI, use only in Acknowledge mode."]
        #[inline(always)]
        pub const fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Sync1Stat {
        #[inline(always)]
        fn default() -> Sync1Stat {
            Sync1Stat(0)
        }
    }
    impl core::fmt::Debug for Sync1Stat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sync1Stat")
                .field("ack", &self.ack())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sync1Stat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sync1Stat {{ ack: {=bool:?} }}", self.ack())
        }
    }
    #[doc = "Activate."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncmActivate(pub u8);
    impl SyncmActivate {
        #[doc = "SyncManager Enable/Disable: 0:Disable:Access to Memory without SyncManager control 1:Enable:SyncManager is active and controls Memory area set in configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SyncManager Enable/Disable: 0:Disable:Access to Memory without SyncManager control 1:Enable:SyncManager is active and controls Memory area set in configuration."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Repeat Request: A toggle of Repeat Request means that a mailbox retry is needed (primarily used in conjunction with ECAT Read Mailbox)."]
        #[must_use]
        #[inline(always)]
        pub const fn repeat(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Repeat Request: A toggle of Repeat Request means that a mailbox retry is needed (primarily used in conjunction with ECAT Read Mailbox)."]
        #[inline(always)]
        pub const fn set_repeat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Latch Event ECAT: 0:No 1:Generate Latch event when EtherCAT master issues a buffer exchange."]
        #[must_use]
        #[inline(always)]
        pub const fn latch_ecat(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Latch Event ECAT: 0:No 1:Generate Latch event when EtherCAT master issues a buffer exchange."]
        #[inline(always)]
        pub const fn set_latch_ecat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Latch Event PDI: 0:No 1:Generate Latch events when PDI issues a buffer exchange or when PDI accesses buffer start address."]
        #[must_use]
        #[inline(always)]
        pub const fn latch_pdi(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Latch Event PDI: 0:No 1:Generate Latch events when PDI issues a buffer exchange or when PDI accesses buffer start address."]
        #[inline(always)]
        pub const fn set_latch_pdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for SyncmActivate {
        #[inline(always)]
        fn default() -> SyncmActivate {
            SyncmActivate(0)
        }
    }
    impl core::fmt::Debug for SyncmActivate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncmActivate")
                .field("en", &self.en())
                .field("repeat", &self.repeat())
                .field("latch_ecat", &self.latch_ecat())
                .field("latch_pdi", &self.latch_pdi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncmActivate {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SyncmActivate {{ en: {=bool:?}, repeat: {=bool:?}, latch_ecat: {=bool:?}, latch_pdi: {=bool:?} }}" , self . en () , self . repeat () , self . latch_ecat () , self . latch_pdi ())
        }
    }
    #[doc = "Length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncmLength(pub u16);
    impl SyncmLength {
        #[doc = "Number of bytes assigned to SyncManager (shall be greater than 1, otherwise SyncManager is not activated. If set to 1, only Watchdog Trigger is generated if configured)."]
        #[must_use]
        #[inline(always)]
        pub const fn len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of bytes assigned to SyncManager (shall be greater than 1, otherwise SyncManager is not activated. If set to 1, only Watchdog Trigger is generated if configured)."]
        #[inline(always)]
        pub const fn set_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for SyncmLength {
        #[inline(always)]
        fn default() -> SyncmLength {
            SyncmLength(0)
        }
    }
    impl core::fmt::Debug for SyncmLength {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncmLength")
                .field("len", &self.len())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncmLength {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SyncmLength {{ len: {=u16:?} }}", self.len())
        }
    }
    #[doc = "SyncManagers supported."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncmNum(pub u8);
    impl SyncmNum {
        #[doc = "Number of supported SyncManager channels (or entities)."]
        #[must_use]
        #[inline(always)]
        pub const fn num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Number of supported SyncManager channels (or entities)."]
        #[inline(always)]
        pub const fn set_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for SyncmNum {
        #[inline(always)]
        fn default() -> SyncmNum {
            SyncmNum(0)
        }
    }
    impl core::fmt::Debug for SyncmNum {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncmNum")
                .field("num", &self.num())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncmNum {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SyncmNum {{ num: {=u8:?} }}", self.num())
        }
    }
    #[doc = "PDI Control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncmPdiCtrl(pub u8);
    impl SyncmPdiCtrl {
        #[doc = "Deactivate SyncManager: Read: 0:Normal operation, SyncManager activated. 1:SyncManager deactivated and reset. SyncManager locks access to Memory area. Write: 0:Activate SyncManager 1:Request SyncManager deactivation NOTE:Writing 1 is delayed until the end of the frame, which is currently processed."]
        #[must_use]
        #[inline(always)]
        pub const fn deact(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Deactivate SyncManager: Read: 0:Normal operation, SyncManager activated. 1:SyncManager deactivated and reset. SyncManager locks access to Memory area. Write: 0:Activate SyncManager 1:Request SyncManager deactivation NOTE:Writing 1 is delayed until the end of the frame, which is currently processed."]
        #[inline(always)]
        pub const fn set_deact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Repeat Ack: If this is set to the same value as that set by Repeat Request, the PDI acknowledges the execution of a previous set Repeat request."]
        #[must_use]
        #[inline(always)]
        pub const fn repeat_ack(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Repeat Ack: If this is set to the same value as that set by Repeat Request, the PDI acknowledges the execution of a previous set Repeat request."]
        #[inline(always)]
        pub const fn set_repeat_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for SyncmPdiCtrl {
        #[inline(always)]
        fn default() -> SyncmPdiCtrl {
            SyncmPdiCtrl(0)
        }
    }
    impl core::fmt::Debug for SyncmPdiCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncmPdiCtrl")
                .field("deact", &self.deact())
                .field("repeat_ack", &self.repeat_ack())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncmPdiCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SyncmPdiCtrl {{ deact: {=bool:?}, repeat_ack: {=bool:?} }}",
                self.deact(),
                self.repeat_ack()
            )
        }
    }
    #[doc = "Physical Start Address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncmPhysicalStartAddr(pub u16);
    impl SyncmPhysicalStartAddr {
        #[doc = "First byte that will be handled by SyncManager."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "First byte that will be handled by SyncManager."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for SyncmPhysicalStartAddr {
        #[inline(always)]
        fn default() -> SyncmPhysicalStartAddr {
            SyncmPhysicalStartAddr(0)
        }
    }
    impl core::fmt::Debug for SyncmPhysicalStartAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncmPhysicalStartAddr")
                .field("addr", &self.addr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncmPhysicalStartAddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SyncmPhysicalStartAddr {{ addr: {=u16:?} }}",
                self.addr()
            )
        }
    }
    #[doc = "SYNC Out Unit Activation."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SyncoAct(pub u8);
    impl SyncoAct {
        #[doc = "Sync Out Unit activation: 0:Deactivated 1:Activated."]
        #[must_use]
        #[inline(always)]
        pub const fn soua(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sync Out Unit activation: 0:Deactivated 1:Activated."]
        #[inline(always)]
        pub const fn set_soua(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "SYNC0 generation: 0:Deactivated 1:SYNC0 pulse is generated."]
        #[must_use]
        #[inline(always)]
        pub const fn sync0_gen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC0 generation: 0:Deactivated 1:SYNC0 pulse is generated."]
        #[inline(always)]
        pub const fn set_sync0_gen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "SYNC1 generation: 0:Deactivated 1:SYNC1 pulse is generated."]
        #[must_use]
        #[inline(always)]
        pub const fn sync1_gen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SYNC1 generation: 0:Deactivated 1:SYNC1 pulse is generated."]
        #[inline(always)]
        pub const fn set_sync1_gen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Auto-activation by writing Start Time Cyclic Operation (0x0990:0x0997): 0:Disabled 1:Auto-activation enabled. 0x0981\\[0\\]
is set automatically after Start Time is written."]
        #[must_use]
        #[inline(always)]
        pub const fn ac(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-activation by writing Start Time Cyclic Operation (0x0990:0x0997): 0:Disabled 1:Auto-activation enabled. 0x0981\\[0\\]
is set automatically after Start Time is written."]
        #[inline(always)]
        pub const fn set_ac(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Extension of Start Time Cyclic Operation (0x0990:0x0993): 0:No extension 1:Extend 32 bit written Start Time to 64 bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ext(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Extension of Start Time Cyclic Operation (0x0990:0x0993): 0:No extension 1:Extend 32 bit written Start Time to 64 bit."]
        #[inline(always)]
        pub const fn set_ext(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Start Time plausibility check: 0:Disabled. SyncSignal generation if Start Time is reached. 1:Immediate SyncSignal generation if Start Time is outside near future (see 0x0981\\[6\\])."]
        #[must_use]
        #[inline(always)]
        pub const fn stpc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Start Time plausibility check: 0:Disabled. SyncSignal generation if Start Time is reached. 1:Immediate SyncSignal generation if Start Time is outside near future (see 0x0981\\[6\\])."]
        #[inline(always)]
        pub const fn set_stpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Near future configuration (approx.): 0:½ DC width future (231 ns or 263 ns) 1:~2.1 sec. future (231 ns)."]
        #[must_use]
        #[inline(always)]
        pub const fn nfc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Near future configuration (approx.): 0:½ DC width future (231 ns or 263 ns) 1:~2.1 sec. future (231 ns)."]
        #[inline(always)]
        pub const fn set_nfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "SyncSignal debug pulse (Vasily bit): 0:Deactivated 1:Immediately generate one ping only on SYNC0-1 according to 0x0981\\[2:1 for debugging This bit is self-clearing, always read 0. All pulses are generated at the same time, the cycle time is ignored. The configured pulse length is used."]
        #[must_use]
        #[inline(always)]
        pub const fn ssdp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SyncSignal debug pulse (Vasily bit): 0:Deactivated 1:Immediately generate one ping only on SYNC0-1 according to 0x0981\\[2:1 for debugging This bit is self-clearing, always read 0. All pulses are generated at the same time, the cycle time is ignored. The configured pulse length is used."]
        #[inline(always)]
        pub const fn set_ssdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for SyncoAct {
        #[inline(always)]
        fn default() -> SyncoAct {
            SyncoAct(0)
        }
    }
    impl core::fmt::Debug for SyncoAct {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SyncoAct")
                .field("soua", &self.soua())
                .field("sync0_gen", &self.sync0_gen())
                .field("sync1_gen", &self.sync1_gen())
                .field("ac", &self.ac())
                .field("ext", &self.ext())
                .field("stpc", &self.stpc())
                .field("nfc", &self.nfc())
                .field("ssdp", &self.ssdp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SyncoAct {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SyncoAct {{ soua: {=bool:?}, sync0_gen: {=bool:?}, sync1_gen: {=bool:?}, ac: {=bool:?}, ext: {=bool:?}, stpc: {=bool:?}, nfc: {=bool:?}, ssdp: {=bool:?} }}" , self . soua () , self . sync0_gen () , self . sync1_gen () , self . ac () , self . ext () , self . stpc () , self . nfc () , self . ssdp ())
        }
    }
    #[doc = "System Time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SysTime(pub u64);
    impl SysTime {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_st(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for SysTime {
        #[inline(always)]
        fn default() -> SysTime {
            SysTime(0)
        }
    }
    impl core::fmt::Debug for SysTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SysTime").field("st", &self.st()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SysTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SysTime {{ st: {=u64:?} }}", self.st())
        }
    }
    #[doc = "System Time Delay."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SysTimeDelay(pub u32);
    impl SysTimeDelay {
        #[doc = "Delay between Reference Clock and the ESC."]
        #[must_use]
        #[inline(always)]
        pub const fn dly(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Delay between Reference Clock and the ESC."]
        #[inline(always)]
        pub const fn set_dly(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SysTimeDelay {
        #[inline(always)]
        fn default() -> SysTimeDelay {
            SysTimeDelay(0)
        }
    }
    impl core::fmt::Debug for SysTimeDelay {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SysTimeDelay")
                .field("dly", &self.dly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SysTimeDelay {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SysTimeDelay {{ dly: {=u32:?} }}", self.dly())
        }
    }
    #[doc = "System Time Difference."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SysTimeDiff(pub u32);
    impl SysTimeDiff {
        #[doc = "Mean difference between local copy of System Time and received System Time values Difference = Received System Time – local copy of System Time."]
        #[must_use]
        #[inline(always)]
        pub const fn num(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "Mean difference between local copy of System Time and received System Time values Difference = Received System Time – local copy of System Time."]
        #[inline(always)]
        pub const fn set_num(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "0:Local copy of System Time less than received System Time 1:Local copy of System Time greater than or equal to received System Time."]
        #[must_use]
        #[inline(always)]
        pub const fn diff(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "0:Local copy of System Time less than received System Time 1:Local copy of System Time greater than or equal to received System Time."]
        #[inline(always)]
        pub const fn set_diff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for SysTimeDiff {
        #[inline(always)]
        fn default() -> SysTimeDiff {
            SysTimeDiff(0)
        }
    }
    impl core::fmt::Debug for SysTimeDiff {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SysTimeDiff")
                .field("num", &self.num())
                .field("diff", &self.diff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SysTimeDiff {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SysTimeDiff {{ num: {=u32:?}, diff: {=bool:?} }}",
                self.num(),
                self.diff()
            )
        }
    }
    #[doc = "System Time Difference Filter Depth."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SysTimeDiffFd(pub u8);
    impl SysTimeDiffFd {
        #[doc = "Filter depth for averaging the received System Time deviation IP Core since V2.2.0/V2.02a: A write access resets System Time Difference (0x092C:0x092F)."]
        #[must_use]
        #[inline(always)]
        pub const fn depth(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Filter depth for averaging the received System Time deviation IP Core since V2.2.0/V2.02a: A write access resets System Time Difference (0x092C:0x092F)."]
        #[inline(always)]
        pub const fn set_depth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for SysTimeDiffFd {
        #[inline(always)]
        fn default() -> SysTimeDiffFd {
            SysTimeDiffFd(0)
        }
    }
    impl core::fmt::Debug for SysTimeDiffFd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SysTimeDiffFd")
                .field("depth", &self.depth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SysTimeDiffFd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SysTimeDiffFd {{ depth: {=u8:?} }}", self.depth())
        }
    }
    #[doc = "System Time Offset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SysTimeOffset(pub u64);
    impl SysTimeOffset {
        #[doc = "Difference between local time and System Time. Offset is added to the local time."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u64 {
            let val = (self.0 >> 0usize) & 0x0;
            val as u64
        }
        #[doc = "Difference between local time and System Time. Offset is added to the local time."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u64) {
            self.0 = (self.0 & !(0x0 << 0usize)) | (((val as u64) & 0x0) << 0usize);
        }
    }
    impl Default for SysTimeOffset {
        #[inline(always)]
        fn default() -> SysTimeOffset {
            SysTimeOffset(0)
        }
    }
    impl core::fmt::Debug for SysTimeOffset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SysTimeOffset")
                .field("offset", &self.offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SysTimeOffset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SysTimeOffset {{ offset: {=u64:?} }}", self.offset())
        }
    }
    #[doc = "User Ram Byte 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte0(pub u8);
    impl UserRamByte0 {
        #[doc = "Number of extended feature bits."]
        #[must_use]
        #[inline(always)]
        pub const fn extf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Number of extended feature bits."]
        #[inline(always)]
        pub const fn set_extf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UserRamByte0 {
        #[inline(always)]
        fn default() -> UserRamByte0 {
            UserRamByte0(0)
        }
    }
    impl core::fmt::Debug for UserRamByte0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte0")
                .field("extf", &self.extf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "UserRamByte0 {{ extf: {=u8:?} }}", self.extf())
        }
    }
    #[doc = "User Ram Byte 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte1(pub u8);
    impl UserRamByte1 {
        #[doc = "Extended DL Control Register (0x0102:0x0103)."]
        #[must_use]
        #[inline(always)]
        pub const fn edlcr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Extended DL Control Register (0x0102:0x0103)."]
        #[inline(always)]
        pub const fn set_edlcr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "AL Status Code Register (0x0134:0x0135)."]
        #[must_use]
        #[inline(always)]
        pub const fn alscr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AL Status Code Register (0x0134:0x0135)."]
        #[inline(always)]
        pub const fn set_alscr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "ECAT Interrupt Mask (0x0200:0x0201)."]
        #[must_use]
        #[inline(always)]
        pub const fn eim(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ECAT Interrupt Mask (0x0200:0x0201)."]
        #[inline(always)]
        pub const fn set_eim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Configured Station Alias (0x0012:0x0013)."]
        #[must_use]
        #[inline(always)]
        pub const fn csa(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Configured Station Alias (0x0012:0x0013)."]
        #[inline(always)]
        pub const fn set_csa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "General Purpose Inputs (0x0F18:0x0F1F)."]
        #[must_use]
        #[inline(always)]
        pub const fn gpi(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "General Purpose Inputs (0x0F18:0x0F1F)."]
        #[inline(always)]
        pub const fn set_gpi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "General Purpose Outputs (0x0F10:0x0F17)."]
        #[must_use]
        #[inline(always)]
        pub const fn gpo(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "General Purpose Outputs (0x0F10:0x0F17)."]
        #[inline(always)]
        pub const fn set_gpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "AL Event Mask writable (0x0204:0x0207)."]
        #[must_use]
        #[inline(always)]
        pub const fn aemw(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "AL Event Mask writable (0x0204:0x0207)."]
        #[inline(always)]
        pub const fn set_aemw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Physical Read/Write Offset (0x0108:0x0109)."]
        #[must_use]
        #[inline(always)]
        pub const fn prwo(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Physical Read/Write Offset (0x0108:0x0109)."]
        #[inline(always)]
        pub const fn set_prwo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UserRamByte1 {
        #[inline(always)]
        fn default() -> UserRamByte1 {
            UserRamByte1(0)
        }
    }
    impl core::fmt::Debug for UserRamByte1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte1")
                .field("edlcr", &self.edlcr())
                .field("alscr", &self.alscr())
                .field("eim", &self.eim())
                .field("csa", &self.csa())
                .field("gpi", &self.gpi())
                .field("gpo", &self.gpo())
                .field("aemw", &self.aemw())
                .field("prwo", &self.prwo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "UserRamByte1 {{ edlcr: {=bool:?}, alscr: {=bool:?}, eim: {=bool:?}, csa: {=bool:?}, gpi: {=bool:?}, gpo: {=bool:?}, aemw: {=bool:?}, prwo: {=bool:?} }}" , self . edlcr () , self . alscr () , self . eim () , self . csa () , self . gpi () , self . gpo () , self . aemw () , self . prwo ())
        }
    }
    #[doc = "User Ram Byte 10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte10(pub u8);
    impl UserRamByte10 {
        #[doc = "DC Latch1 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn dcl1d(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DC Latch1 disable."]
        #[inline(always)]
        pub const fn set_dcl1d(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "AXI PDI."]
        #[must_use]
        #[inline(always)]
        pub const fn apdi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AXI PDI."]
        #[inline(always)]
        pub const fn set_apdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "PDI function acknowledge by PDI write."]
        #[must_use]
        #[inline(always)]
        pub const fn pdifa(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PDI function acknowledge by PDI write."]
        #[inline(always)]
        pub const fn set_pdifa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "PDI Information register (0x014E:0x014F)."]
        #[must_use]
        #[inline(always)]
        pub const fn pdiir(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PDI Information register (0x014E:0x014F)."]
        #[inline(always)]
        pub const fn set_pdiir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UserRamByte10 {
        #[inline(always)]
        fn default() -> UserRamByte10 {
            UserRamByte10(0)
        }
    }
    impl core::fmt::Debug for UserRamByte10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte10")
                .field("dcl1d", &self.dcl1d())
                .field("apdi", &self.apdi())
                .field("pdifa", &self.pdifa())
                .field("pdiir", &self.pdiir())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte10 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "UserRamByte10 {{ dcl1d: {=bool:?}, apdi: {=bool:?}, pdifa: {=bool:?}, pdiir: {=bool:?} }}" , self . dcl1d () , self . apdi () , self . pdifa () , self . pdiir ())
        }
    }
    #[doc = "User Ram Byte 11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte11(pub u8);
    impl UserRamByte11 {
        #[doc = "LED test."]
        #[must_use]
        #[inline(always)]
        pub const fn ledtst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LED test."]
        #[inline(always)]
        pub const fn set_ledtst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for UserRamByte11 {
        #[inline(always)]
        fn default() -> UserRamByte11 {
            UserRamByte11(0)
        }
    }
    impl core::fmt::Debug for UserRamByte11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte11")
                .field("ledtst", &self.ledtst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "UserRamByte11 {{ ledtst: {=bool:?} }}", self.ledtst())
        }
    }
    #[doc = "User Ram Byte 14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte14(pub u8);
    impl UserRamByte14 {
        #[doc = "Digital I/O PDI byte size."]
        #[must_use]
        #[inline(always)]
        pub const fn diobs(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Digital I/O PDI byte size."]
        #[inline(always)]
        pub const fn set_diobs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for UserRamByte14 {
        #[inline(always)]
        fn default() -> UserRamByte14 {
            UserRamByte14(0)
        }
    }
    impl core::fmt::Debug for UserRamByte14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte14")
                .field("diobs", &self.diobs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "UserRamByte14 {{ diobs: {=u8:?} }}", self.diobs())
        }
    }
    #[doc = "User Ram Byte 15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte15(pub u8);
    impl UserRamByte15 {
        #[doc = "Digital I/O PDI."]
        #[must_use]
        #[inline(always)]
        pub const fn diopdi(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Digital I/O PDI."]
        #[inline(always)]
        pub const fn set_diopdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "SPI Slave PDI."]
        #[must_use]
        #[inline(always)]
        pub const fn sspdi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SPI Slave PDI."]
        #[inline(always)]
        pub const fn set_sspdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Asynchronous µC PDI."]
        #[must_use]
        #[inline(always)]
        pub const fn aucpdi(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Asynchronous µC PDI."]
        #[inline(always)]
        pub const fn set_aucpdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for UserRamByte15 {
        #[inline(always)]
        fn default() -> UserRamByte15 {
            UserRamByte15(0)
        }
    }
    impl core::fmt::Debug for UserRamByte15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte15")
                .field("diopdi", &self.diopdi())
                .field("sspdi", &self.sspdi())
                .field("aucpdi", &self.aucpdi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "UserRamByte15 {{ diopdi: {=bool:?}, sspdi: {=bool:?}, aucpdi: {=bool:?} }}",
                self.diopdi(),
                self.sspdi(),
                self.aucpdi()
            )
        }
    }
    #[doc = "User Ram Byte 19."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte19(pub u8);
    impl UserRamByte19 {
        #[doc = "RGMII."]
        #[must_use]
        #[inline(always)]
        pub const fn rgmii(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RGMII."]
        #[inline(always)]
        pub const fn set_rgmii(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Individual PHY address read out (0x0510\\[7:3\\])."]
        #[must_use]
        #[inline(always)]
        pub const fn iparo(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Individual PHY address read out (0x0510\\[7:3\\])."]
        #[inline(always)]
        pub const fn set_iparo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "CLK_PDI_EXT is asynchronous."]
        #[must_use]
        #[inline(always)]
        pub const fn cia(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CLK_PDI_EXT is asynchronous."]
        #[inline(always)]
        pub const fn set_cia(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Use RGMII GTX_CLK phase shifted clock input."]
        #[must_use]
        #[inline(always)]
        pub const fn urgp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Use RGMII GTX_CLK phase shifted clock input."]
        #[inline(always)]
        pub const fn set_urgp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RMII."]
        #[must_use]
        #[inline(always)]
        pub const fn rmii(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RMII."]
        #[inline(always)]
        pub const fn set_rmii(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Security CPLD protection."]
        #[must_use]
        #[inline(always)]
        pub const fn scp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Security CPLD protection."]
        #[inline(always)]
        pub const fn set_scp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for UserRamByte19 {
        #[inline(always)]
        fn default() -> UserRamByte19 {
            UserRamByte19(0)
        }
    }
    impl core::fmt::Debug for UserRamByte19 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte19")
                .field("rgmii", &self.rgmii())
                .field("iparo", &self.iparo())
                .field("cia", &self.cia())
                .field("urgp", &self.urgp())
                .field("rmii", &self.rmii())
                .field("scp", &self.scp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte19 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "UserRamByte19 {{ rgmii: {=bool:?}, iparo: {=bool:?}, cia: {=bool:?}, urgp: {=bool:?}, rmii: {=bool:?}, scp: {=bool:?} }}" , self . rgmii () , self . iparo () , self . cia () , self . urgp () , self . rmii () , self . scp ())
        }
    }
    #[doc = "User Ram Byte 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte2(pub u8);
    impl UserRamByte2 {
        #[doc = "Watchdog divider writable (0x0400:0x0401) and Watchdog PDI (0x0410:0x0411)."]
        #[must_use]
        #[inline(always)]
        pub const fn wdw(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog divider writable (0x0400:0x0401) and Watchdog PDI (0x0410:0x0411)."]
        #[inline(always)]
        pub const fn set_wdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Watchdog counters (0x0442:0x0443)."]
        #[must_use]
        #[inline(always)]
        pub const fn wdgcnt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog counters (0x0442:0x0443)."]
        #[inline(always)]
        pub const fn set_wdgcnt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Write Protection (0x0020:0x0031)."]
        #[must_use]
        #[inline(always)]
        pub const fn wp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Write Protection (0x0020:0x0031)."]
        #[inline(always)]
        pub const fn set_wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Reset (0x0040:0x0041)."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Reset (0x0040:0x0041)."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "DC SyncManager Event Times (0x09F0:0x09FF)."]
        #[must_use]
        #[inline(always)]
        pub const fn dcsmet(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DC SyncManager Event Times (0x09F0:0x09FF)."]
        #[inline(always)]
        pub const fn set_dcsmet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "ECAT Processing Unit/PDI Error Counter (0x030C:0x030D)."]
        #[must_use]
        #[inline(always)]
        pub const fn epupec(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ECAT Processing Unit/PDI Error Counter (0x030C:0x030D)."]
        #[inline(always)]
        pub const fn set_epupec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "EEPROM Size configurable (0x0502\\[7\\]): 0:EEPROM Size fixed to sizes up to 16 Kbit 1:EEPROM Size configurable."]
        #[must_use]
        #[inline(always)]
        pub const fn escfg(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "EEPROM Size configurable (0x0502\\[7\\]): 0:EEPROM Size fixed to sizes up to 16 Kbit 1:EEPROM Size configurable."]
        #[inline(always)]
        pub const fn set_escfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UserRamByte2 {
        #[inline(always)]
        fn default() -> UserRamByte2 {
            UserRamByte2(0)
        }
    }
    impl core::fmt::Debug for UserRamByte2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte2")
                .field("wdw", &self.wdw())
                .field("wdgcnt", &self.wdgcnt())
                .field("wp", &self.wp())
                .field("reset", &self.reset())
                .field("dcsmet", &self.dcsmet())
                .field("epupec", &self.epupec())
                .field("escfg", &self.escfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "UserRamByte2 {{ wdw: {=bool:?}, wdgcnt: {=bool:?}, wp: {=bool:?}, reset: {=bool:?}, dcsmet: {=bool:?}, epupec: {=bool:?}, escfg: {=bool:?} }}" , self . wdw () , self . wdgcnt () , self . wp () , self . reset () , self . dcsmet () , self . epupec () , self . escfg ())
        }
    }
    #[doc = "User Ram Byte 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte3(pub u8);
    impl UserRamByte3 {
        #[doc = "Lost Link Counter (0x0310:0x0313)."]
        #[must_use]
        #[inline(always)]
        pub const fn llc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Lost Link Counter (0x0310:0x0313)."]
        #[inline(always)]
        pub const fn set_llc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "MII Management Interface (0x0510:0x0515)."]
        #[must_use]
        #[inline(always)]
        pub const fn mmi(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MII Management Interface (0x0510:0x0515)."]
        #[inline(always)]
        pub const fn set_mmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Enhanced Link Detection MII."]
        #[must_use]
        #[inline(always)]
        pub const fn eldm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enhanced Link Detection MII."]
        #[inline(always)]
        pub const fn set_eldm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Enhanced Link Detection EBUS."]
        #[must_use]
        #[inline(always)]
        pub const fn elde(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enhanced Link Detection EBUS."]
        #[inline(always)]
        pub const fn set_elde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Run LED (DEV_STATE LED)."]
        #[must_use]
        #[inline(always)]
        pub const fn rled(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Run LED (DEV_STATE LED)."]
        #[inline(always)]
        pub const fn set_rled(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UserRamByte3 {
        #[inline(always)]
        fn default() -> UserRamByte3 {
            UserRamByte3(0)
        }
    }
    impl core::fmt::Debug for UserRamByte3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte3")
                .field("llc", &self.llc())
                .field("mmi", &self.mmi())
                .field("eldm", &self.eldm())
                .field("elde", &self.elde())
                .field("rled", &self.rled())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "UserRamByte3 {{ llc: {=bool:?}, mmi: {=bool:?}, eldm: {=bool:?}, elde: {=bool:?}, rled: {=bool:?} }}" , self . llc () , self . mmi () , self . eldm () , self . elde () , self . rled ())
        }
    }
    #[doc = "User Ram Byte 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte4(pub u8);
    impl UserRamByte4 {
        #[doc = "Link/Activity LED."]
        #[must_use]
        #[inline(always)]
        pub const fn laled(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Link/Activity LED."]
        #[inline(always)]
        pub const fn set_laled(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "DC Latch In Unit."]
        #[must_use]
        #[inline(always)]
        pub const fn dliu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DC Latch In Unit."]
        #[inline(always)]
        pub const fn set_dliu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "DC Sync Out Unit."]
        #[must_use]
        #[inline(always)]
        pub const fn dsou(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DC Sync Out Unit."]
        #[inline(always)]
        pub const fn set_dsou(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "DC Time loop control assigned to PDI."]
        #[must_use]
        #[inline(always)]
        pub const fn dtlc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DC Time loop control assigned to PDI."]
        #[inline(always)]
        pub const fn set_dtlc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Link detection and configuration by MI."]
        #[must_use]
        #[inline(always)]
        pub const fn ldcm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Link detection and configuration by MI."]
        #[inline(always)]
        pub const fn set_ldcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UserRamByte4 {
        #[inline(always)]
        fn default() -> UserRamByte4 {
            UserRamByte4(0)
        }
    }
    impl core::fmt::Debug for UserRamByte4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte4")
                .field("laled", &self.laled())
                .field("dliu", &self.dliu())
                .field("dsou", &self.dsou())
                .field("dtlc", &self.dtlc())
                .field("ldcm", &self.ldcm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "UserRamByte4 {{ laled: {=bool:?}, dliu: {=bool:?}, dsou: {=bool:?}, dtlc: {=bool:?}, ldcm: {=bool:?} }}" , self . laled () , self . dliu () , self . dsou () , self . dtlc () , self . ldcm ())
        }
    }
    #[doc = "User Ram Byte 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte5(pub u8);
    impl UserRamByte5 {
        #[doc = "MI control by PDI possible."]
        #[must_use]
        #[inline(always)]
        pub const fn mcpp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MI control by PDI possible."]
        #[inline(always)]
        pub const fn set_mcpp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Automatic TX shift."]
        #[must_use]
        #[inline(always)]
        pub const fn ats(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic TX shift."]
        #[inline(always)]
        pub const fn set_ats(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "EEPROM emulation by µController."]
        #[must_use]
        #[inline(always)]
        pub const fn eeu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EEPROM emulation by µController."]
        #[inline(always)]
        pub const fn set_eeu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Disable Digital I/O register (0x0F00:0x0F03)."]
        #[must_use]
        #[inline(always)]
        pub const fn ddior(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Digital I/O register (0x0F00:0x0F03)."]
        #[inline(always)]
        pub const fn set_ddior(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for UserRamByte5 {
        #[inline(always)]
        fn default() -> UserRamByte5 {
            UserRamByte5(0)
        }
    }
    impl core::fmt::Debug for UserRamByte5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte5")
                .field("mcpp", &self.mcpp())
                .field("ats", &self.ats())
                .field("eeu", &self.eeu())
                .field("ddior", &self.ddior())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte5 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "UserRamByte5 {{ mcpp: {=bool:?}, ats: {=bool:?}, eeu: {=bool:?}, ddior: {=bool:?} }}" , self . mcpp () , self . ats () , self . eeu () , self . ddior ())
        }
    }
    #[doc = "User Ram Byte 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte6(pub u8);
    impl UserRamByte6 {
        #[doc = "RUN/ERR LED Override (0x0138:0x0139)."]
        #[must_use]
        #[inline(always)]
        pub const fn reledor(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RUN/ERR LED Override (0x0138:0x0139)."]
        #[inline(always)]
        pub const fn set_reledor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for UserRamByte6 {
        #[inline(always)]
        fn default() -> UserRamByte6 {
            UserRamByte6(0)
        }
    }
    impl core::fmt::Debug for UserRamByte6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte6")
                .field("reledor", &self.reledor())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "UserRamByte6 {{ reledor: {=bool:?} }}", self.reledor())
        }
    }
    #[doc = "User Ram Byte 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte7(pub u8);
    impl UserRamByte7 {
        #[doc = "DC Sync1 disable."]
        #[must_use]
        #[inline(always)]
        pub const fn dcs1d(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DC Sync1 disable."]
        #[inline(always)]
        pub const fn set_dcs1d(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "DC Receive Times (0x0900:0x090F)."]
        #[must_use]
        #[inline(always)]
        pub const fn dcrt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DC Receive Times (0x0900:0x090F)."]
        #[inline(always)]
        pub const fn set_dcrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "DC System Time (0x0910:0x0936)."]
        #[must_use]
        #[inline(always)]
        pub const fn dcst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DC System Time (0x0910:0x0936)."]
        #[inline(always)]
        pub const fn set_dcst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UserRamByte7 {
        #[inline(always)]
        fn default() -> UserRamByte7 {
            UserRamByte7(0)
        }
    }
    impl core::fmt::Debug for UserRamByte7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte7")
                .field("dcs1d", &self.dcs1d())
                .field("dcrt", &self.dcrt())
                .field("dcst", &self.dcst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "UserRamByte7 {{ dcs1d: {=bool:?}, dcrt: {=bool:?}, dcst: {=bool:?} }}",
                self.dcs1d(),
                self.dcrt(),
                self.dcst()
            )
        }
    }
    #[doc = "User Ram Byte 8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte8(pub u8);
    impl UserRamByte8 {
        #[doc = "DC 64 bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dc64(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DC 64 bit."]
        #[inline(always)]
        pub const fn set_dc64(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "PDI clears error counter."]
        #[must_use]
        #[inline(always)]
        pub const fn pdicec(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PDI clears error counter."]
        #[inline(always)]
        pub const fn set_pdicec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Avalon PDI."]
        #[must_use]
        #[inline(always)]
        pub const fn apdi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Avalon PDI."]
        #[inline(always)]
        pub const fn set_apdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "OPB PDI."]
        #[must_use]
        #[inline(always)]
        pub const fn opdi(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPB PDI."]
        #[inline(always)]
        pub const fn set_opdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "PLB PDI."]
        #[must_use]
        #[inline(always)]
        pub const fn ppdi(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PLB PDI."]
        #[inline(always)]
        pub const fn set_ppdi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for UserRamByte8 {
        #[inline(always)]
        fn default() -> UserRamByte8 {
            UserRamByte8(0)
        }
    }
    impl core::fmt::Debug for UserRamByte8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte8")
                .field("dc64", &self.dc64())
                .field("pdicec", &self.pdicec())
                .field("apdi", &self.apdi())
                .field("opdi", &self.opdi())
                .field("ppdi", &self.ppdi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte8 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "UserRamByte8 {{ dc64: {=bool:?}, pdicec: {=bool:?}, apdi: {=bool:?}, opdi: {=bool:?}, ppdi: {=bool:?} }}" , self . dc64 () , self . pdicec () , self . apdi () , self . opdi () , self . ppdi ())
        }
    }
    #[doc = "User Ram Byte 9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UserRamByte9(pub u8);
    impl UserRamByte9 {
        #[doc = "Direct RESET."]
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Direct RESET."]
        #[inline(always)]
        pub const fn set_dr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UserRamByte9 {
        #[inline(always)]
        fn default() -> UserRamByte9 {
            UserRamByte9(0)
        }
    }
    impl core::fmt::Debug for UserRamByte9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UserRamByte9")
                .field("dr", &self.dr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for UserRamByte9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "UserRamByte9 {{ dr: {=bool:?} }}", self.dr())
        }
    }
    #[doc = "Vendor ID."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vid(pub u64);
    impl Vid {
        #[doc = "Vendor ID: \\[23-0\\]
Company \\[31-24\\]
Department NOTE:Test Vendor IDs have \\[31:28\\]=0xE."]
        #[must_use]
        #[inline(always)]
        pub const fn vid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Vendor ID: \\[23-0\\]
Company \\[31-24\\]
Department NOTE:Test Vendor IDs have \\[31:28\\]=0xE."]
        #[inline(always)]
        pub const fn set_vid(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u64) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Vid {
        #[inline(always)]
        fn default() -> Vid {
            Vid(0)
        }
    }
    impl core::fmt::Debug for Vid {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vid").field("vid", &self.vid()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vid {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vid {{ vid: {=u32:?} }}", self.vid())
        }
    }
    #[doc = "Watchdog Counter Process Data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdgCntPdat(pub u8);
    impl WdgCntPdat {
        #[doc = "Watchdog Counter Process Data (counting is stopped when 0xFF is reached). Counts if Process Data Watchdog expires."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Watchdog Counter Process Data (counting is stopped when 0xFF is reached). Counts if Process Data Watchdog expires."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for WdgCntPdat {
        #[inline(always)]
        fn default() -> WdgCntPdat {
            WdgCntPdat(0)
        }
    }
    impl core::fmt::Debug for WdgCntPdat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdgCntPdat")
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WdgCntPdat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WdgCntPdat {{ cnt: {=u8:?} }}", self.cnt())
        }
    }
    #[doc = "Watchdog Counter PDI."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdgCntPdi(pub u8);
    impl WdgCntPdi {
        #[doc = "Watchdog PDI counter (counting is stopped when 0xFF is reached). Counts if PDI Watchdog expires."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Watchdog PDI counter (counting is stopped when 0xFF is reached). Counts if PDI Watchdog expires."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for WdgCntPdi {
        #[inline(always)]
        fn default() -> WdgCntPdi {
            WdgCntPdi(0)
        }
    }
    impl core::fmt::Debug for WdgCntPdi {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdgCntPdi")
                .field("cnt", &self.cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WdgCntPdi {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WdgCntPdi {{ cnt: {=u8:?} }}", self.cnt())
        }
    }
    #[doc = "Watchdog Divider."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdgDiv(pub u16);
    impl WdgDiv {
        #[doc = "Watchdog divider:Number of 25 MHz tics (minus 2) that represent the basic watchdog increment. (Default value is 100µs = 2498)."]
        #[must_use]
        #[inline(always)]
        pub const fn div(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Watchdog divider:Number of 25 MHz tics (minus 2) that represent the basic watchdog increment. (Default value is 100µs = 2498)."]
        #[inline(always)]
        pub const fn set_div(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for WdgDiv {
        #[inline(always)]
        fn default() -> WdgDiv {
            WdgDiv(0)
        }
    }
    impl core::fmt::Debug for WdgDiv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdgDiv").field("div", &self.div()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WdgDiv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WdgDiv {{ div: {=u16:?} }}", self.div())
        }
    }
    #[doc = "Watchdog Status Process Data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdgStatPdat(pub u16);
    impl WdgStatPdat {
        #[doc = "Watchdog Status of Process Data (triggered by SyncManagers) 0:Watchdog Process Data expired 1:Watchdog Process Data is active or disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn st(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog Status of Process Data (triggered by SyncManagers) 0:Watchdog Process Data expired 1:Watchdog Process Data is active or disabled."]
        #[inline(always)]
        pub const fn set_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
    }
    impl Default for WdgStatPdat {
        #[inline(always)]
        fn default() -> WdgStatPdat {
            WdgStatPdat(0)
        }
    }
    impl core::fmt::Debug for WdgStatPdat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdgStatPdat")
                .field("st", &self.st())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WdgStatPdat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WdgStatPdat {{ st: {=bool:?} }}", self.st())
        }
    }
    #[doc = "Watchdog Time Process Data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdgTimePdat(pub u16);
    impl WdgTimePdat {
        #[doc = "Watchdog Time Process Data:number of basic watchdog increments (Default value with Watchdog divider 100µs means 100ms Watchdog)."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Watchdog Time Process Data:number of basic watchdog increments (Default value with Watchdog divider 100µs means 100ms Watchdog)."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for WdgTimePdat {
        #[inline(always)]
        fn default() -> WdgTimePdat {
            WdgTimePdat(0)
        }
    }
    impl core::fmt::Debug for WdgTimePdat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdgTimePdat")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WdgTimePdat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WdgTimePdat {{ time: {=u16:?} }}", self.time())
        }
    }
    #[doc = "Watchdog Time PDI."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdgTimePdi(pub u16);
    impl WdgTimePdi {
        #[doc = "Watchdog Time PDI:number of basic watchdog increments (Default value with Watchdog divider 100µs means 100ms Watchdog)."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Watchdog Time PDI:number of basic watchdog increments (Default value with Watchdog divider 100µs means 100ms Watchdog)."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for WdgTimePdi {
        #[inline(always)]
        fn default() -> WdgTimePdi {
            WdgTimePdi(0)
        }
    }
    impl core::fmt::Debug for WdgTimePdi {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdgTimePdi")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for WdgTimePdi {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "WdgTimePdi {{ time: {=u16:?} }}", self.time())
        }
    }
}
