#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdCmdTable {
    ptr: *mut u8,
}
unsafe impl Send for CmdCmdTable {}
unsafe impl Sync for CmdCmdTable {}
impl CmdCmdTable {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "command start value."]
    #[inline(always)]
    pub const fn min(self) -> crate::common::Reg<regs::Min, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "command end value."]
    #[inline(always)]
    pub const fn max(self) -> crate::common::Reg<regs::Max, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "command compare bit enable."]
    #[inline(always)]
    pub const fn msk(self) -> crate::common::Reg<regs::Msk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "command pointer 0 - 3."]
    #[inline(always)]
    pub const fn pta(self) -> crate::common::Reg<regs::Pta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "command pointer 4 - 7."]
    #[inline(always)]
    pub const fn ptb(self) -> crate::common::Reg<regs::Ptb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdLatch {
    ptr: *mut u8,
}
unsafe impl Send for CmdLatch {}
unsafe impl Sync for CmdLatch {}
impl CmdLatch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn tran(self, n: usize) -> crate::common::Reg<regs::Tran, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Latch configuration."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Latch time."]
    #[inline(always)]
    pub const fn time(self) -> crate::common::Reg<regs::Time, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Latch status."]
    #[inline(always)]
    pub const fn sts(self) -> crate::common::Reg<regs::CmdLatchSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl {
    ptr: *mut u8,
}
unsafe impl Send for Ctrl {}
unsafe impl Sync for Ctrl {}
impl Ctrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Engine control register."]
    #[inline(always)]
    pub const fn engine_ctrl(self) -> crate::common::Reg<regs::EngineCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Pointer configuration register."]
    #[inline(always)]
    pub const fn engine_ptr_cfg(self) -> crate::common::Reg<regs::EnginePtrCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Watch dog configuration register."]
    #[inline(always)]
    pub const fn engine_wdg_cfg(self) -> crate::common::Reg<regs::EngineWdgCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Execution status."]
    #[inline(always)]
    pub const fn engine_exe_sta(self) -> crate::common::Reg<regs::EngineExeSta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Execution pointer."]
    #[inline(always)]
    pub const fn engine_exe_ptr(self) -> crate::common::Reg<regs::EngineExePtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Execution instruction."]
    #[inline(always)]
    pub const fn engine_exe_inst(
        self,
    ) -> crate::common::Reg<regs::EngineExeInst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Watch dog status."]
    #[inline(always)]
    pub const fn engine_wdg_sta(self) -> crate::common::Reg<regs::EngineWdgSta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Transceiver control register."]
    #[inline(always)]
    pub const fn xcvr_ctrl(self) -> crate::common::Reg<regs::XcvrCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Transceiver configuration register."]
    #[inline(always)]
    pub const fn xcvr_type_cfg(self) -> crate::common::Reg<regs::XcvrTypeCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Transceiver baud rate register."]
    #[inline(always)]
    pub const fn xcvr_baud_cfg(self) -> crate::common::Reg<regs::XcvrBaudCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Transceiver data timing configuration."]
    #[inline(always)]
    pub const fn xcvr_data_cfg(self) -> crate::common::Reg<regs::XcvrDataCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Transceiver clock timing configuration."]
    #[inline(always)]
    pub const fn xcvr_clk_cfg(self) -> crate::common::Reg<regs::XcvrClkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Transceiver pin status."]
    #[inline(always)]
    pub const fn xcvr_pin(self) -> crate::common::Reg<regs::XcvrPin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "FSM of asynchronous."]
    #[inline(always)]
    pub const fn xcvr_state(self) -> crate::common::Reg<regs::XcvrState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Trigger input configuration."]
    #[inline(always)]
    pub const fn trg_in_cfg(self) -> crate::common::Reg<regs::TrgInCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Software trigger."]
    #[inline(always)]
    pub const fn trg_sw(self) -> crate::common::Reg<regs::TrgSw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Period trigger configuration."]
    #[inline(always)]
    pub const fn trg_prd_cfg(self) -> crate::common::Reg<regs::TrgPrdCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Trigger period."]
    #[inline(always)]
    pub const fn trg_prd(self) -> crate::common::Reg<regs::TrgPrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Trigger output configuration."]
    #[inline(always)]
    pub const fn trg_out_cfg(self) -> crate::common::Reg<regs::TrgOutCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Period trigger status."]
    #[inline(always)]
    pub const fn trg_prd_sts(self) -> crate::common::Reg<regs::TrgPrdSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Period trigger counter."]
    #[inline(always)]
    pub const fn trg_prd_cnt(self) -> crate::common::Reg<regs::TrgPrdCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn trg_table_cmd(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::TrgTableCmd, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn trg_table_time(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::TrgTableTime, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "command register mode."]
    #[inline(always)]
    pub const fn cmd_mode(self) -> crate::common::Reg<regs::CmdMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "command register configuration."]
    #[inline(always)]
    pub const fn cmd_idx(self) -> crate::common::Reg<regs::CmdIdx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "command."]
    #[inline(always)]
    pub const fn cmd_cmd(self) -> crate::common::Reg<regs::CmdCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "command bit set register."]
    #[inline(always)]
    pub const fn cmd_set(self) -> crate::common::Reg<regs::CmdSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "command bit clear register."]
    #[inline(always)]
    pub const fn cmd_clr(self) -> crate::common::Reg<regs::CmdClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "command bit invert register."]
    #[inline(always)]
    pub const fn cmd_inv(self) -> crate::common::Reg<regs::CmdInv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Commad input."]
    #[inline(always)]
    pub const fn cmd_in(self) -> crate::common::Reg<regs::CmdIn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Command output."]
    #[inline(always)]
    pub const fn cmd_out(self) -> crate::common::Reg<regs::CmdOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Command status."]
    #[inline(always)]
    pub const fn cmd_sts(self) -> crate::common::Reg<regs::CmdSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmd_cmd_table(self, n: usize) -> CmdCmdTable {
        assert!(n < 8usize);
        unsafe { CmdCmdTable::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 32usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn cmd_latch(self, n: usize) -> CmdLatch {
        assert!(n < 4usize);
        unsafe { CmdLatch::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 32usize) as _) }
    }
    #[doc = "Sample selection register."]
    #[inline(always)]
    pub const fn pos_smp_en(self) -> crate::common::Reg<regs::PosSmpEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Sample configuration."]
    #[inline(always)]
    pub const fn pos_smp_cfg(self) -> crate::common::Reg<regs::PosSmpCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Sample data."]
    #[inline(always)]
    pub const fn pos_smp_dat(self) -> crate::common::Reg<regs::PosSmpDat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "Sample override position."]
    #[inline(always)]
    pub const fn pos_smp_pos(self) -> crate::common::Reg<regs::PosSmpPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "Sample override revolution."]
    #[inline(always)]
    pub const fn pos_smp_rev(self) -> crate::common::Reg<regs::PosSmpRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "Sample override speed."]
    #[inline(always)]
    pub const fn pos_smp_spd(self) -> crate::common::Reg<regs::PosSmpSpd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "Sample override accelerate."]
    #[inline(always)]
    pub const fn pos_smp_acc(self) -> crate::common::Reg<regs::PosSmpAcc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "Update configuration."]
    #[inline(always)]
    pub const fn pos_upd_en(self) -> crate::common::Reg<regs::PosUpdEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "Update configuration."]
    #[inline(always)]
    pub const fn pos_upd_cfg(self) -> crate::common::Reg<regs::PosUpdCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "Update data."]
    #[inline(always)]
    pub const fn pos_upd_dat(self) -> crate::common::Reg<regs::PosUpdDat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Update overide time."]
    #[inline(always)]
    pub const fn pos_upd_time(self) -> crate::common::Reg<regs::PosUpdTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "Update override position."]
    #[inline(always)]
    pub const fn pos_upd_pos(self) -> crate::common::Reg<regs::PosUpdPos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Update override revolution."]
    #[inline(always)]
    pub const fn pos_upd_rev(self) -> crate::common::Reg<regs::PosUpdRev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Update override speed."]
    #[inline(always)]
    pub const fn pos_upd_spd(self) -> crate::common::Reg<regs::PosUpdSpd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Update override accelerate."]
    #[inline(always)]
    pub const fn pos_upd_acc(self) -> crate::common::Reg<regs::PosUpdAcc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "Sample valid."]
    #[inline(always)]
    pub const fn pos_smp_val(self) -> crate::common::Reg<regs::PosSmpVal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Sample status."]
    #[inline(always)]
    pub const fn pos_smp_sts(self) -> crate::common::Reg<regs::PosSmpSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "input time."]
    #[inline(always)]
    pub const fn pos_time_in(self) -> crate::common::Reg<regs::PosTimeIn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "Input position."]
    #[inline(always)]
    pub const fn pos_pos_in(self) -> crate::common::Reg<regs::PosPosIn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "Input revolution."]
    #[inline(always)]
    pub const fn pos_rev_in(self) -> crate::common::Reg<regs::PosRevIn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[doc = "Input speed."]
    #[inline(always)]
    pub const fn pos_spd_in(self) -> crate::common::Reg<regs::PosSpdIn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[doc = "Input accelerate."]
    #[inline(always)]
    pub const fn pos_acc_in(self) -> crate::common::Reg<regs::PosAccIn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02dcusize) as _) }
    }
    #[doc = "Update status."]
    #[inline(always)]
    pub const fn pos_upd_sts(self) -> crate::common::Reg<regs::PosUpdSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e4usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn irq_int_en(self) -> crate::common::Reg<regs::IrqIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Interrupt flag."]
    #[inline(always)]
    pub const fn irq_int_flag(self) -> crate::common::Reg<regs::IrqIntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Interrupt status."]
    #[inline(always)]
    pub const fn irq_int_sts(self) -> crate::common::Reg<regs::IrqIntSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "Match pointer 0."]
    #[inline(always)]
    pub const fn irq_pointer0(self) -> crate::common::Reg<regs::IrqPointer0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "Match pointer 1."]
    #[inline(always)]
    pub const fn irq_pointer1(self) -> crate::common::Reg<regs::IrqPointer1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "Match instruction 0."]
    #[inline(always)]
    pub const fn irq_instr0(self) -> crate::common::Reg<regs::IrqInstr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "Match instruction 1."]
    #[inline(always)]
    pub const fn irq_instr1(self) -> crate::common::Reg<regs::IrqInstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dat {
    ptr: *mut u8,
}
unsafe impl Send for Dat {}
unsafe impl Sync for Dat {}
impl Dat {
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
    pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Data register bit index."]
    #[inline(always)]
    pub const fn idx(self) -> crate::common::Reg<regs::Idx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Gold data for data check."]
    #[inline(always)]
    pub const fn gold(self) -> crate::common::Reg<regs::Gold, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CRC calculation initial vector."]
    #[inline(always)]
    pub const fn crcinit(self) -> crate::common::Reg<regs::Crcinit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CRC calculation polynomial."]
    #[inline(always)]
    pub const fn crcpoly(self) -> crate::common::Reg<regs::Crcpoly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Data value."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Data bit set."]
    #[inline(always)]
    pub const fn set(self) -> crate::common::Reg<regs::Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Data bit clear."]
    #[inline(always)]
    pub const fn clr(self) -> crate::common::Reg<regs::Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Data bit invert."]
    #[inline(always)]
    pub const fn inv(self) -> crate::common::Reg<regs::Inv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Data input."]
    #[inline(always)]
    pub const fn in_(self) -> crate::common::Reg<regs::In, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Data output."]
    #[inline(always)]
    pub const fn out(self) -> crate::common::Reg<regs::Out, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Data status."]
    #[inline(always)]
    pub const fn sts(self) -> crate::common::Reg<regs::DatSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
}
#[doc = "SEI."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sei {
    ptr: *mut u8,
}
unsafe impl Send for Sei {}
unsafe impl Sync for Sei {}
impl Sei {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ctrl(self, n: usize) -> Ctrl {
        assert!(n < 2usize);
        unsafe { Ctrl::from_ptr(self.ptr.wrapping_add(0x0usize + n * 1024usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn instr(self, n: usize) -> crate::common::Reg<regs::Instr, crate::common::RW> {
        assert!(n < 64usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3400usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn dat(self, n: usize) -> Dat {
        assert!(n < 10usize);
        unsafe { Dat::from_ptr(self.ptr.wrapping_add(0x3800usize + n * 64usize) as _) }
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
    #[doc = "Latch configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "Delay in system clock cycle, for state transition."]
        #[must_use]
        #[inline(always)]
        pub const fn delay(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Delay in system clock cycle, for state transition."]
        #[inline(always)]
        pub const fn set_delay(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Output select 0: state0-state1 1: state1-state2 2: state2-state3 3: state3-state0."]
        #[must_use]
        #[inline(always)]
        pub const fn select(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Output select 0: state0-state1 1: state1-state2 2: state2-state3 3: state3-state0."]
        #[inline(always)]
        pub const fn set_select(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Enable latch 0: disable 1: enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable latch 0: disable 1: enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    impl core::fmt::Debug for Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg")
                .field("delay", &self.delay())
                .field("select", &self.select())
                .field("en", &self.en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg {{ delay: {=u16:?}, select: {=u8:?}, en: {=bool:?} }}",
                self.delay(),
                self.select(),
                self.en()
            )
        }
    }
    #[doc = "Data bit clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clr(pub u32);
    impl Clr {
        #[doc = "DATA bit clear."]
        #[must_use]
        #[inline(always)]
        pub const fn data_clr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DATA bit clear."]
        #[inline(always)]
        pub const fn set_data_clr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Clr {
        #[inline(always)]
        fn default() -> Clr {
            Clr(0)
        }
    }
    impl core::fmt::Debug for Clr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clr")
                .field("data_clr", &self.data_clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Clr {{ data_clr: {=u32:?} }}", self.data_clr())
        }
    }
    #[doc = "command bit clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdClr(pub u32);
    impl CmdClr {
        #[doc = "DATA bit clear."]
        #[must_use]
        #[inline(always)]
        pub const fn data_clr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DATA bit clear."]
        #[inline(always)]
        pub const fn set_data_clr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CmdClr {
        #[inline(always)]
        fn default() -> CmdClr {
            CmdClr(0)
        }
    }
    impl core::fmt::Debug for CmdClr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmdClr")
                .field("data_clr", &self.data_clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmdClr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CmdClr {{ data_clr: {=u32:?} }}", self.data_clr())
        }
    }
    #[doc = "command."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdCmd(pub u32);
    impl CmdCmd {
        #[doc = "DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DATA."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CmdCmd {
        #[inline(always)]
        fn default() -> CmdCmd {
            CmdCmd(0)
        }
    }
    impl core::fmt::Debug for CmdCmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmdCmd")
                .field("data", &self.data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmdCmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CmdCmd {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "command register configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdIdx(pub u32);
    impl CmdIdx {
        #[doc = "Lowest bit index."]
        #[must_use]
        #[inline(always)]
        pub const fn min_bit(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Lowest bit index."]
        #[inline(always)]
        pub const fn set_min_bit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Highest bit index."]
        #[must_use]
        #[inline(always)]
        pub const fn max_bit(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Highest bit index."]
        #[inline(always)]
        pub const fn set_max_bit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "First bit index for tranceive."]
        #[must_use]
        #[inline(always)]
        pub const fn first_bit(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "First bit index for tranceive."]
        #[inline(always)]
        pub const fn set_first_bit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Last bit index for tranceive."]
        #[must_use]
        #[inline(always)]
        pub const fn last_bit(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Last bit index for tranceive."]
        #[inline(always)]
        pub const fn set_last_bit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for CmdIdx {
        #[inline(always)]
        fn default() -> CmdIdx {
            CmdIdx(0)
        }
    }
    impl core::fmt::Debug for CmdIdx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmdIdx")
                .field("min_bit", &self.min_bit())
                .field("max_bit", &self.max_bit())
                .field("first_bit", &self.first_bit())
                .field("last_bit", &self.last_bit())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmdIdx {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CmdIdx {{ min_bit: {=u8:?}, max_bit: {=u8:?}, first_bit: {=u8:?}, last_bit: {=u8:?} }}" , self . min_bit () , self . max_bit () , self . first_bit () , self . last_bit ())
        }
    }
    #[doc = "Commad input."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdIn(pub u32);
    impl CmdIn {
        #[doc = "Commad input."]
        #[must_use]
        #[inline(always)]
        pub const fn data_in(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Commad input."]
        #[inline(always)]
        pub const fn set_data_in(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CmdIn {
        #[inline(always)]
        fn default() -> CmdIn {
            CmdIn(0)
        }
    }
    impl core::fmt::Debug for CmdIn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmdIn")
                .field("data_in", &self.data_in())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmdIn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CmdIn {{ data_in: {=u32:?} }}", self.data_in())
        }
    }
    #[doc = "command bit invert register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdInv(pub u32);
    impl CmdInv {
        #[doc = "DATA bit toggle."]
        #[must_use]
        #[inline(always)]
        pub const fn data_tgl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DATA bit toggle."]
        #[inline(always)]
        pub const fn set_data_tgl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CmdInv {
        #[inline(always)]
        fn default() -> CmdInv {
            CmdInv(0)
        }
    }
    impl core::fmt::Debug for CmdInv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmdInv")
                .field("data_tgl", &self.data_tgl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmdInv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CmdInv {{ data_tgl: {=u32:?} }}", self.data_tgl())
        }
    }
    #[doc = "Latch status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdLatchSts(pub u32);
    impl CmdLatchSts {
        #[doc = "Latch counter."]
        #[must_use]
        #[inline(always)]
        pub const fn lat_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Latch counter."]
        #[inline(always)]
        pub const fn set_lat_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "State."]
        #[must_use]
        #[inline(always)]
        pub const fn state(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "State."]
        #[inline(always)]
        pub const fn set_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for CmdLatchSts {
        #[inline(always)]
        fn default() -> CmdLatchSts {
            CmdLatchSts(0)
        }
    }
    impl core::fmt::Debug for CmdLatchSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmdLatchSts")
                .field("lat_cnt", &self.lat_cnt())
                .field("state", &self.state())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmdLatchSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CmdLatchSts {{ lat_cnt: {=u16:?}, state: {=u8:?} }}",
                self.lat_cnt(),
                self.state()
            )
        }
    }
    #[doc = "command register mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdMode(pub u32);
    impl CmdMode {
        #[doc = "Data mode(CMD register only support data mode) 0: data mode 1: check mode 2: CRC mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Data mode(CMD register only support data mode) 0: data mode 1: check mode 2: CRC mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Write 1 to rewind read/write pointer, this is a self clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rewind(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 to rewind read/write pointer, this is a self clear bit."]
        #[inline(always)]
        pub const fn set_rewind(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Signed 0: unsigned value 1: signed value."]
        #[must_use]
        #[inline(always)]
        pub const fn signed(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Signed 0: unsigned value 1: signed value."]
        #[inline(always)]
        pub const fn set_signed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "bit order 0: LSB first 1: MSB first."]
        #[must_use]
        #[inline(always)]
        pub const fn border(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "bit order 0: LSB first 1: MSB first."]
        #[inline(always)]
        pub const fn set_border(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "word order 0: sample as bit order 1: different from bit order."]
        #[must_use]
        #[inline(always)]
        pub const fn worder(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "word order 0: sample as bit order 1: different from bit order."]
        #[inline(always)]
        pub const fn set_worder(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "word length 0: 1 bit 1: 2 bit ... 31: 32 bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wlen(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "word length 0: 1 bit 1: 2 bit ... 31: 32 bit."]
        #[inline(always)]
        pub const fn set_wlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for CmdMode {
        #[inline(always)]
        fn default() -> CmdMode {
            CmdMode(0)
        }
    }
    impl core::fmt::Debug for CmdMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmdMode")
                .field("mode", &self.mode())
                .field("rewind", &self.rewind())
                .field("signed", &self.signed())
                .field("border", &self.border())
                .field("worder", &self.worder())
                .field("wlen", &self.wlen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmdMode {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CmdMode {{ mode: {=u8:?}, rewind: {=bool:?}, signed: {=bool:?}, border: {=bool:?}, worder: {=bool:?}, wlen: {=u8:?} }}" , self . mode () , self . rewind () , self . signed () , self . border () , self . worder () , self . wlen ())
        }
    }
    #[doc = "Command output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdOut(pub u32);
    impl CmdOut {
        #[doc = "Command output."]
        #[must_use]
        #[inline(always)]
        pub const fn data_out(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Command output."]
        #[inline(always)]
        pub const fn set_data_out(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CmdOut {
        #[inline(always)]
        fn default() -> CmdOut {
            CmdOut(0)
        }
    }
    impl core::fmt::Debug for CmdOut {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmdOut")
                .field("data_out", &self.data_out())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmdOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CmdOut {{ data_out: {=u32:?} }}", self.data_out())
        }
    }
    #[doc = "command bit set register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdSet(pub u32);
    impl CmdSet {
        #[doc = "DATA bit set."]
        #[must_use]
        #[inline(always)]
        pub const fn data_set(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DATA bit set."]
        #[inline(always)]
        pub const fn set_data_set(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CmdSet {
        #[inline(always)]
        fn default() -> CmdSet {
            CmdSet(0)
        }
    }
    impl core::fmt::Debug for CmdSet {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmdSet")
                .field("data_set", &self.data_set())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmdSet {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CmdSet {{ data_set: {=u32:?} }}", self.data_set())
        }
    }
    #[doc = "Command status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdSts(pub u32);
    impl CmdSts {
        #[doc = "Bit index."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_idx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Bit index."]
        #[inline(always)]
        pub const fn set_bit_idx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Word counter."]
        #[must_use]
        #[inline(always)]
        pub const fn word_cnt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Word counter."]
        #[inline(always)]
        pub const fn set_word_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Word index."]
        #[must_use]
        #[inline(always)]
        pub const fn word_idx(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Word index."]
        #[inline(always)]
        pub const fn set_word_idx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for CmdSts {
        #[inline(always)]
        fn default() -> CmdSts {
            CmdSts(0)
        }
    }
    impl core::fmt::Debug for CmdSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmdSts")
                .field("bit_idx", &self.bit_idx())
                .field("word_cnt", &self.word_cnt())
                .field("word_idx", &self.word_idx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CmdSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CmdSts {{ bit_idx: {=u8:?}, word_cnt: {=u8:?}, word_idx: {=u8:?} }}",
                self.bit_idx(),
                self.word_cnt(),
                self.word_idx()
            )
        }
    }
    #[doc = "CRC calculation initial vector."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcinit(pub u32);
    impl Crcinit {
        #[doc = "CRC initial value."]
        #[must_use]
        #[inline(always)]
        pub const fn crc_init(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CRC initial value."]
        #[inline(always)]
        pub const fn set_crc_init(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Crcinit {
        #[inline(always)]
        fn default() -> Crcinit {
            Crcinit(0)
        }
    }
    impl core::fmt::Debug for Crcinit {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crcinit")
                .field("crc_init", &self.crc_init())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crcinit {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Crcinit {{ crc_init: {=u32:?} }}", self.crc_init())
        }
    }
    #[doc = "CRC calculation polynomial."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcpoly(pub u32);
    impl Crcpoly {
        #[doc = "CRC polymonial."]
        #[must_use]
        #[inline(always)]
        pub const fn crc_poly(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CRC polymonial."]
        #[inline(always)]
        pub const fn set_crc_poly(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Crcpoly {
        #[inline(always)]
        fn default() -> Crcpoly {
            Crcpoly(0)
        }
    }
    impl core::fmt::Debug for Crcpoly {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crcpoly")
                .field("crc_poly", &self.crc_poly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crcpoly {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Crcpoly {{ crc_poly: {=u32:?} }}", self.crc_poly())
        }
    }
    #[doc = "Data status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DatSts(pub u32);
    impl DatSts {
        #[doc = "Bit index."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_idx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Bit index."]
        #[inline(always)]
        pub const fn set_bit_idx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Word counter."]
        #[must_use]
        #[inline(always)]
        pub const fn word_cnt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Word counter."]
        #[inline(always)]
        pub const fn set_word_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Word index."]
        #[must_use]
        #[inline(always)]
        pub const fn word_idx(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Word index."]
        #[inline(always)]
        pub const fn set_word_idx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "CRC index."]
        #[must_use]
        #[inline(always)]
        pub const fn crc_idx(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "CRC index."]
        #[inline(always)]
        pub const fn set_crc_idx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for DatSts {
        #[inline(always)]
        fn default() -> DatSts {
            DatSts(0)
        }
    }
    impl core::fmt::Debug for DatSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DatSts")
                .field("bit_idx", &self.bit_idx())
                .field("word_cnt", &self.word_cnt())
                .field("word_idx", &self.word_idx())
                .field("crc_idx", &self.crc_idx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DatSts {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DatSts {{ bit_idx: {=u8:?}, word_cnt: {=u8:?}, word_idx: {=u8:?}, crc_idx: {=u8:?} }}" , self . bit_idx () , self . word_cnt () , self . word_idx () , self . crc_idx ())
        }
    }
    #[doc = "Data value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data(pub u32);
    impl Data {
        #[doc = "DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DATA."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Data {
        #[inline(always)]
        fn default() -> Data {
            Data(0)
        }
    }
    impl core::fmt::Debug for Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Data {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Data {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "Engine control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EngineCtrl(pub u32);
    impl EngineCtrl {
        #[doc = "Enable 0: disable 1: enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable 0: disable 1: enable."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Rewind execution pointer 0: run 1: clean status and rewind."]
        #[must_use]
        #[inline(always)]
        pub const fn rewind(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Rewind execution pointer 0: run 1: clean status and rewind."]
        #[inline(always)]
        pub const fn set_rewind(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Explain timout as exception 0: when timeout, pointer move to next instruction 1: when timeout, pointer jump to timeout vector."]
        #[must_use]
        #[inline(always)]
        pub const fn except(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Explain timout as exception 0: when timeout, pointer move to next instruction 1: when timeout, pointer jump to timeout vector."]
        #[inline(always)]
        pub const fn set_except(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wait for trigger before excuting 0: Execute on enable 1: Wait trigger before exection after enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn arming(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Wait for trigger before excuting 0: Execute on enable 1: Wait trigger before exection after enabled."]
        #[inline(always)]
        pub const fn set_arming(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Enable watch dog 0: Watch dog disabled 1: Watch dog enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn watch(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enable watch dog 0: Watch dog disabled 1: Watch dog enabled."]
        #[inline(always)]
        pub const fn set_watch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for EngineCtrl {
        #[inline(always)]
        fn default() -> EngineCtrl {
            EngineCtrl(0)
        }
    }
    impl core::fmt::Debug for EngineCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EngineCtrl")
                .field("enable", &self.enable())
                .field("rewind", &self.rewind())
                .field("except", &self.except())
                .field("arming", &self.arming())
                .field("watch", &self.watch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EngineCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EngineCtrl {{ enable: {=bool:?}, rewind: {=bool:?}, except: {=bool:?}, arming: {=bool:?}, watch: {=bool:?} }}" , self . enable () , self . rewind () , self . except () , self . arming () , self . watch ())
        }
    }
    #[doc = "Execution instruction."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EngineExeInst(pub u32);
    impl EngineExeInst {
        #[doc = "Current instruction."]
        #[must_use]
        #[inline(always)]
        pub const fn inst(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Current instruction."]
        #[inline(always)]
        pub const fn set_inst(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EngineExeInst {
        #[inline(always)]
        fn default() -> EngineExeInst {
            EngineExeInst(0)
        }
    }
    impl core::fmt::Debug for EngineExeInst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EngineExeInst")
                .field("inst", &self.inst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EngineExeInst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EngineExeInst {{ inst: {=u32:?} }}", self.inst())
        }
    }
    #[doc = "Execution pointer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EngineExePtr(pub u32);
    impl EngineExePtr {
        #[doc = "Current program pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn pointer(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Current program pointer."]
        #[inline(always)]
        pub const fn set_pointer(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Bit count in send and receive instruction execution."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_cnt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Bit count in send and receive instruction execution."]
        #[inline(always)]
        pub const fn set_bit_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Halt count in halt instrution."]
        #[must_use]
        #[inline(always)]
        pub const fn halt_cnt(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Halt count in halt instrution."]
        #[inline(always)]
        pub const fn set_halt_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for EngineExePtr {
        #[inline(always)]
        fn default() -> EngineExePtr {
            EngineExePtr(0)
        }
    }
    impl core::fmt::Debug for EngineExePtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EngineExePtr")
                .field("pointer", &self.pointer())
                .field("bit_cnt", &self.bit_cnt())
                .field("halt_cnt", &self.halt_cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EngineExePtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EngineExePtr {{ pointer: {=u8:?}, bit_cnt: {=u8:?}, halt_cnt: {=u8:?} }}",
                self.pointer(),
                self.bit_cnt(),
                self.halt_cnt()
            )
        }
    }
    #[doc = "Execution status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EngineExeSta(pub u32);
    impl EngineExeSta {
        #[doc = "Program finished 0: Program is executing 1: Program finished."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Program finished 0: Program is executing 1: Program finished."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Watchdog timer expired 0: Not expired 1: Expired."]
        #[must_use]
        #[inline(always)]
        pub const fn expire(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog timer expired 0: Not expired 1: Expired."]
        #[inline(always)]
        pub const fn set_expire(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Waiting for trigger for execution 0: Not in waiting status 1: In waiting status."]
        #[must_use]
        #[inline(always)]
        pub const fn armed(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Waiting for trigger for execution 0: Not in waiting status 1: In waiting status."]
        #[inline(always)]
        pub const fn set_armed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Execution has been triggered 0: Execution not triggered 1: Execution triggered."]
        #[must_use]
        #[inline(always)]
        pub const fn trigered(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Execution has been triggered 0: Execution not triggered 1: Execution triggered."]
        #[inline(always)]
        pub const fn set_trigered(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for EngineExeSta {
        #[inline(always)]
        fn default() -> EngineExeSta {
            EngineExeSta(0)
        }
    }
    impl core::fmt::Debug for EngineExeSta {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EngineExeSta")
                .field("stall", &self.stall())
                .field("expire", &self.expire())
                .field("armed", &self.armed())
                .field("trigered", &self.trigered())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EngineExeSta {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EngineExeSta {{ stall: {=bool:?}, expire: {=bool:?}, armed: {=bool:?}, trigered: {=bool:?} }}" , self . stall () , self . expire () , self . armed () , self . trigered ())
        }
    }
    #[doc = "Pointer configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EnginePtrCfg(pub u32);
    impl EnginePtrCfg {
        #[doc = "Initial execute pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn pointer_init(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Initial execute pointer."]
        #[inline(always)]
        pub const fn set_pointer_init(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Pointer to the instruction that the program starts executing after the instruction timeout. The timeout is WDOG_TIME."]
        #[must_use]
        #[inline(always)]
        pub const fn pointer_wdog(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Pointer to the instruction that the program starts executing after the instruction timeout. The timeout is WDOG_TIME."]
        #[inline(always)]
        pub const fn set_pointer_wdog(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Bias for data register access, if calculated index bigger than 32, index will wrap around 0: real data index 1: access index is 1 greater than instruction address 2: access index is 2 greater than instruction address ... 31: access index is 31 greater than instruction address."]
        #[must_use]
        #[inline(always)]
        pub const fn dat_base(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Bias for data register access, if calculated index bigger than 32, index will wrap around 0: real data index 1: access index is 1 greater than instruction address 2: access index is 2 greater than instruction address ... 31: access index is 31 greater than instruction address."]
        #[inline(always)]
        pub const fn set_dat_base(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Select DATA register to receive CDM bit in BiSSC slave mode 0: ignore 1: command 2: data register 2 3: data register 3 ... 29:data register 29 30: value 0 when send, ignore in receive 31: value1 when send, ignore in receive."]
        #[must_use]
        #[inline(always)]
        pub const fn dat_cdm(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Select DATA register to receive CDM bit in BiSSC slave mode 0: ignore 1: command 2: data register 2 3: data register 3 ... 29:data register 29 30: value 0 when send, ignore in receive 31: value1 when send, ignore in receive."]
        #[inline(always)]
        pub const fn set_dat_cdm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for EnginePtrCfg {
        #[inline(always)]
        fn default() -> EnginePtrCfg {
            EnginePtrCfg(0)
        }
    }
    impl core::fmt::Debug for EnginePtrCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EnginePtrCfg")
                .field("pointer_init", &self.pointer_init())
                .field("pointer_wdog", &self.pointer_wdog())
                .field("dat_base", &self.dat_base())
                .field("dat_cdm", &self.dat_cdm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EnginePtrCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "EnginePtrCfg {{ pointer_init: {=u8:?}, pointer_wdog: {=u8:?}, dat_base: {=u8:?}, dat_cdm: {=u8:?} }}" , self . pointer_init () , self . pointer_wdog () , self . dat_base () , self . dat_cdm ())
        }
    }
    #[doc = "Watch dog configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EngineWdgCfg(pub u32);
    impl EngineWdgCfg {
        #[doc = "Time out count for each instruction, counter in bit time."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog_time(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Time out count for each instruction, counter in bit time."]
        #[inline(always)]
        pub const fn set_wdog_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for EngineWdgCfg {
        #[inline(always)]
        fn default() -> EngineWdgCfg {
            EngineWdgCfg(0)
        }
    }
    impl core::fmt::Debug for EngineWdgCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EngineWdgCfg")
                .field("wdog_time", &self.wdog_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EngineWdgCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EngineWdgCfg {{ wdog_time: {=u16:?} }}",
                self.wdog_time()
            )
        }
    }
    #[doc = "Watch dog status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EngineWdgSta(pub u32);
    impl EngineWdgSta {
        #[doc = "Current watch dog counter value."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Current watch dog counter value."]
        #[inline(always)]
        pub const fn set_wdog_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for EngineWdgSta {
        #[inline(always)]
        fn default() -> EngineWdgSta {
            EngineWdgSta(0)
        }
    }
    impl core::fmt::Debug for EngineWdgSta {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EngineWdgSta")
                .field("wdog_cnt", &self.wdog_cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EngineWdgSta {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EngineWdgSta {{ wdog_cnt: {=u16:?} }}", self.wdog_cnt())
        }
    }
    #[doc = "Gold data for data check."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gold(pub u32);
    impl Gold {
        #[doc = "Gold value for check mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gold_value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Gold value for check mode."]
        #[inline(always)]
        pub const fn set_gold_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Gold {
        #[inline(always)]
        fn default() -> Gold {
            Gold(0)
        }
    }
    impl core::fmt::Debug for Gold {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gold")
                .field("gold_value", &self.gold_value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gold {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gold {{ gold_value: {=u32:?} }}", self.gold_value())
        }
    }
    #[doc = "Data register bit index."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idx(pub u32);
    impl Idx {
        #[doc = "Lowest bit index."]
        #[must_use]
        #[inline(always)]
        pub const fn min_bit(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Lowest bit index."]
        #[inline(always)]
        pub const fn set_min_bit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Highest bit index."]
        #[must_use]
        #[inline(always)]
        pub const fn max_bit(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Highest bit index."]
        #[inline(always)]
        pub const fn set_max_bit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "First bit index for tranceive."]
        #[must_use]
        #[inline(always)]
        pub const fn first_bit(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "First bit index for tranceive."]
        #[inline(always)]
        pub const fn set_first_bit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Last bit index for tranceive."]
        #[must_use]
        #[inline(always)]
        pub const fn last_bit(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Last bit index for tranceive."]
        #[inline(always)]
        pub const fn set_last_bit(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Idx {
        #[inline(always)]
        fn default() -> Idx {
            Idx(0)
        }
    }
    impl core::fmt::Debug for Idx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idx")
                .field("min_bit", &self.min_bit())
                .field("max_bit", &self.max_bit())
                .field("first_bit", &self.first_bit())
                .field("last_bit", &self.last_bit())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idx {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Idx {{ min_bit: {=u8:?}, max_bit: {=u8:?}, first_bit: {=u8:?}, last_bit: {=u8:?} }}" , self . min_bit () , self . max_bit () , self . first_bit () , self . last_bit ())
        }
    }
    #[doc = "Data input."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct In(pub u32);
    impl In {
        #[doc = "Data input."]
        #[must_use]
        #[inline(always)]
        pub const fn data_in(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data input."]
        #[inline(always)]
        pub const fn set_data_in(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for In {
        #[inline(always)]
        fn default() -> In {
            In(0)
        }
    }
    impl core::fmt::Debug for In {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("In")
                .field("data_in", &self.data_in())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for In {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "In {{ data_in: {=u32:?} }}", self.data_in())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Instr(pub u32);
    impl Instr {
        #[doc = "1\\]
When OP is 0, this area is the halt time in baudrate, 0 represents infinite time. \\[2\\]
When OP is 1, this area is the the pointer to the command table. OPR\\[4\\]=1, OPR\\[3:0\\]
value is CMD_TABLE instruct pointer; OPR\\[4\\]=0, OPR\\[3:0\\]=0 is INIT_POINTER; OPR\\[4\\]=0, OPR\\[3:0\\]=1 is WDG_POINTER. \\[3\\]
When OP is 2-7, this area is the data length as fellow: 0: 1 bit 1: 2 bit ... 31: 32 bit."]
        #[must_use]
        #[inline(always)]
        pub const fn opr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "1\\]
When OP is 0, this area is the halt time in baudrate, 0 represents infinite time. \\[2\\]
When OP is 1, this area is the the pointer to the command table. OPR\\[4\\]=1, OPR\\[3:0\\]
value is CMD_TABLE instruct pointer; OPR\\[4\\]=0, OPR\\[3:0\\]=0 is INIT_POINTER; OPR\\[4\\]=0, OPR\\[3:0\\]=1 is WDG_POINTER. \\[3\\]
When OP is 2-7, this area is the data length as fellow: 0: 1 bit 1: 2 bit ... 31: 32 bit."]
        #[inline(always)]
        pub const fn set_opr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "DATA register 0: ignore data 1: command 2: data register 2 3: data register 3 ... 29: data register 29 30: value 0 when send, wait 0 in receive 31: value1 when send, wait 1 in receive."]
        #[must_use]
        #[inline(always)]
        pub const fn dat(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "DATA register 0: ignore data 1: command 2: data register 2 3: data register 3 ... 29: data register 29 30: value 0 when send, wait 0 in receive 31: value1 when send, wait 1 in receive."]
        #[inline(always)]
        pub const fn set_dat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "CRC register 0: don't calculate CRC 1: do not set this value 2: data register 2 3: data register 3 ... 29: data register 29 30: value 0 when send, wait 0 in receive 31: value1 when send, wait 1 in receive."]
        #[must_use]
        #[inline(always)]
        pub const fn crc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "CRC register 0: don't calculate CRC 1: do not set this value 2: data register 2 3: data register 3 ... 29: data register 29 30: value 0 when send, wait 0 in receive 31: value1 when send, wait 1 in receive."]
        #[inline(always)]
        pub const fn set_crc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "clock 0: low 1: rise-fall 2: fall-rise 3: high."]
        #[must_use]
        #[inline(always)]
        pub const fn ck(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "clock 0: low 1: rise-fall 2: fall-rise 3: high."]
        #[inline(always)]
        pub const fn set_ck(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "operation 0: halt 1: jump 2: send with timeout check 3: send without timout check 4: wait with timeout check 5: wait without timout check 6: receive with timeout check 7: receive without timout check."]
        #[must_use]
        #[inline(always)]
        pub const fn op(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x07;
            val as u8
        }
        #[doc = "operation 0: halt 1: jump 2: send with timeout check 3: send without timout check 4: wait with timeout check 5: wait without timout check 6: receive with timeout check 7: receive without timout check."]
        #[inline(always)]
        pub const fn set_op(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
        }
    }
    impl Default for Instr {
        #[inline(always)]
        fn default() -> Instr {
            Instr(0)
        }
    }
    impl core::fmt::Debug for Instr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Instr")
                .field("opr", &self.opr())
                .field("dat", &self.dat())
                .field("crc", &self.crc())
                .field("ck", &self.ck())
                .field("op", &self.op())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Instr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Instr {{ opr: {=u8:?}, dat: {=u8:?}, crc: {=u8:?}, ck: {=u8:?}, op: {=u8:?} }}",
                self.opr(),
                self.dat(),
                self.crc(),
                self.ck(),
                self.op()
            )
        }
    }
    #[doc = "Data bit invert."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Inv(pub u32);
    impl Inv {
        #[doc = "DATA bit toggle."]
        #[must_use]
        #[inline(always)]
        pub const fn data_inv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DATA bit toggle."]
        #[inline(always)]
        pub const fn set_data_inv(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Inv {
        #[inline(always)]
        fn default() -> Inv {
            Inv(0)
        }
    }
    impl core::fmt::Debug for Inv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Inv")
                .field("data_inv", &self.data_inv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Inv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Inv {{ data_inv: {=u32:?} }}", self.data_inv())
        }
    }
    #[doc = "Match instruction 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqInstr0(pub u32);
    impl IrqInstr0 {
        #[doc = "Match instruction 0."]
        #[must_use]
        #[inline(always)]
        pub const fn instr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Match instruction 0."]
        #[inline(always)]
        pub const fn set_instr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IrqInstr0 {
        #[inline(always)]
        fn default() -> IrqInstr0 {
            IrqInstr0(0)
        }
    }
    impl core::fmt::Debug for IrqInstr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqInstr0")
                .field("instr", &self.instr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqInstr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqInstr0 {{ instr: {=u32:?} }}", self.instr())
        }
    }
    #[doc = "Match instruction 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqInstr1(pub u32);
    impl IrqInstr1 {
        #[doc = "Match instruction 1."]
        #[must_use]
        #[inline(always)]
        pub const fn instr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Match instruction 1."]
        #[inline(always)]
        pub const fn set_instr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IrqInstr1 {
        #[inline(always)]
        fn default() -> IrqInstr1 {
            IrqInstr1(0)
        }
    }
    impl core::fmt::Debug for IrqInstr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqInstr1")
                .field("instr", &self.instr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqInstr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqInstr1 {{ instr: {=u32:?} }}", self.instr())
        }
    }
    #[doc = "Interrupt Enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqIntEn(pub u32);
    impl IrqIntEn {
        #[doc = "Stall."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Stall."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Exception."]
        #[must_use]
        #[inline(always)]
        pub const fn except(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Exception."]
        #[inline(always)]
        pub const fn set_except(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Watch dog."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Watch dog."]
        #[inline(always)]
        pub const fn set_wdog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Pointer 0 start."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr0_st(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Pointer 0 start."]
        #[inline(always)]
        pub const fn set_ptr0_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Pointer 1 start."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr1_st(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Pointer 1 start."]
        #[inline(always)]
        pub const fn set_ptr1_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Instruction 0 start."]
        #[must_use]
        #[inline(always)]
        pub const fn instr0_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction 0 start."]
        #[inline(always)]
        pub const fn set_instr0_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Instruction 1 start."]
        #[must_use]
        #[inline(always)]
        pub const fn instr1_st(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction 1 start."]
        #[inline(always)]
        pub const fn set_instr1_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Pointer 0 end."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr0_end(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Pointer 0 end."]
        #[inline(always)]
        pub const fn set_ptr0_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Pointer 1 end."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr1_end(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Pointer 1 end."]
        #[inline(always)]
        pub const fn set_ptr1_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Instruction 0 end."]
        #[must_use]
        #[inline(always)]
        pub const fn instr0_end(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction 0 end."]
        #[inline(always)]
        pub const fn set_instr0_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Instruction 1 end."]
        #[must_use]
        #[inline(always)]
        pub const fn instr1_end(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction 1 end."]
        #[inline(always)]
        pub const fn set_instr1_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Transfer error."]
        #[must_use]
        #[inline(always)]
        pub const fn trx_err(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error."]
        #[inline(always)]
        pub const fn set_trx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn timeout(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout."]
        #[inline(always)]
        pub const fn set_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Latch0."]
        #[must_use]
        #[inline(always)]
        pub const fn latch0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Latch0."]
        #[inline(always)]
        pub const fn set_latch0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Latch1."]
        #[must_use]
        #[inline(always)]
        pub const fn latch1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Latch1."]
        #[inline(always)]
        pub const fn set_latch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Latch2."]
        #[must_use]
        #[inline(always)]
        pub const fn latch2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Latch2."]
        #[inline(always)]
        pub const fn set_latch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Latch3."]
        #[must_use]
        #[inline(always)]
        pub const fn latch3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Latch3."]
        #[inline(always)]
        pub const fn set_latch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Sample error."]
        #[must_use]
        #[inline(always)]
        pub const fn smp_err(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Sample error."]
        #[inline(always)]
        pub const fn set_smp_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Trigger0."]
        #[must_use]
        #[inline(always)]
        pub const fn triger0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger0."]
        #[inline(always)]
        pub const fn set_triger0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Trigger1."]
        #[must_use]
        #[inline(always)]
        pub const fn triger1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger1."]
        #[inline(always)]
        pub const fn set_triger1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Trigger2."]
        #[must_use]
        #[inline(always)]
        pub const fn triger2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger2."]
        #[inline(always)]
        pub const fn set_triger2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Trigger3."]
        #[must_use]
        #[inline(always)]
        pub const fn triger3(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger3."]
        #[inline(always)]
        pub const fn set_triger3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Trigger0 failed."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_err0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger0 failed."]
        #[inline(always)]
        pub const fn set_trg_err0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Trigger1 failed."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_err1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger1 failed."]
        #[inline(always)]
        pub const fn set_trg_err1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Trigger2 failed."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_err2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger2 failed."]
        #[inline(always)]
        pub const fn set_trg_err2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Trigger3 failed."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_err3(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger3 failed."]
        #[inline(always)]
        pub const fn set_trg_err3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IrqIntEn {
        #[inline(always)]
        fn default() -> IrqIntEn {
            IrqIntEn(0)
        }
    }
    impl core::fmt::Debug for IrqIntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqIntEn")
                .field("stall", &self.stall())
                .field("except", &self.except())
                .field("wdog", &self.wdog())
                .field("ptr0_st", &self.ptr0_st())
                .field("ptr1_st", &self.ptr1_st())
                .field("instr0_st", &self.instr0_st())
                .field("instr1_st", &self.instr1_st())
                .field("ptr0_end", &self.ptr0_end())
                .field("ptr1_end", &self.ptr1_end())
                .field("instr0_end", &self.instr0_end())
                .field("instr1_end", &self.instr1_end())
                .field("trx_err", &self.trx_err())
                .field("timeout", &self.timeout())
                .field("latch0", &self.latch0())
                .field("latch1", &self.latch1())
                .field("latch2", &self.latch2())
                .field("latch3", &self.latch3())
                .field("smp_err", &self.smp_err())
                .field("triger0", &self.triger0())
                .field("triger1", &self.triger1())
                .field("triger2", &self.triger2())
                .field("triger3", &self.triger3())
                .field("trg_err0", &self.trg_err0())
                .field("trg_err1", &self.trg_err1())
                .field("trg_err2", &self.trg_err2())
                .field("trg_err3", &self.trg_err3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqIntEn {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IrqIntEn {{ stall: {=bool:?}, except: {=bool:?}, wdog: {=bool:?}, ptr0_st: {=bool:?}, ptr1_st: {=bool:?}, instr0_st: {=bool:?}, instr1_st: {=bool:?}, ptr0_end: {=bool:?}, ptr1_end: {=bool:?}, instr0_end: {=bool:?}, instr1_end: {=bool:?}, trx_err: {=bool:?}, timeout: {=bool:?}, latch0: {=bool:?}, latch1: {=bool:?}, latch2: {=bool:?}, latch3: {=bool:?}, smp_err: {=bool:?}, triger0: {=bool:?}, triger1: {=bool:?}, triger2: {=bool:?}, triger3: {=bool:?}, trg_err0: {=bool:?}, trg_err1: {=bool:?}, trg_err2: {=bool:?}, trg_err3: {=bool:?} }}" , self . stall () , self . except () , self . wdog () , self . ptr0_st () , self . ptr1_st () , self . instr0_st () , self . instr1_st () , self . ptr0_end () , self . ptr1_end () , self . instr0_end () , self . instr1_end () , self . trx_err () , self . timeout () , self . latch0 () , self . latch1 () , self . latch2 () , self . latch3 () , self . smp_err () , self . triger0 () , self . triger1 () , self . triger2 () , self . triger3 () , self . trg_err0 () , self . trg_err1 () , self . trg_err2 () , self . trg_err3 ())
        }
    }
    #[doc = "Interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqIntFlag(pub u32);
    impl IrqIntFlag {
        #[doc = "Stall."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Stall."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Exception."]
        #[must_use]
        #[inline(always)]
        pub const fn except(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Exception."]
        #[inline(always)]
        pub const fn set_except(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Watch dog."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Watch dog."]
        #[inline(always)]
        pub const fn set_wdog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Pointer 0 start."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr0_st(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Pointer 0 start."]
        #[inline(always)]
        pub const fn set_ptr0_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Pointer 1 start."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr1_st(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Pointer 1 start."]
        #[inline(always)]
        pub const fn set_ptr1_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Instruction 0 start."]
        #[must_use]
        #[inline(always)]
        pub const fn instr0_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction 0 start."]
        #[inline(always)]
        pub const fn set_instr0_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Instruction 1 start."]
        #[must_use]
        #[inline(always)]
        pub const fn instr1_st(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction 1 start."]
        #[inline(always)]
        pub const fn set_instr1_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Pointer 0 end."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr0_end(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Pointer 0 end."]
        #[inline(always)]
        pub const fn set_ptr0_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Pointer 1 end."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr1_end(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Pointer 1 end."]
        #[inline(always)]
        pub const fn set_ptr1_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Instruction 0 end."]
        #[must_use]
        #[inline(always)]
        pub const fn instr0_end(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction 0 end."]
        #[inline(always)]
        pub const fn set_instr0_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Instruction 1 end."]
        #[must_use]
        #[inline(always)]
        pub const fn instr1_end(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction 1 end."]
        #[inline(always)]
        pub const fn set_instr1_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Transfer error."]
        #[must_use]
        #[inline(always)]
        pub const fn trx_err(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error."]
        #[inline(always)]
        pub const fn set_trx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn timeout(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout."]
        #[inline(always)]
        pub const fn set_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Latch0."]
        #[must_use]
        #[inline(always)]
        pub const fn latch0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Latch0."]
        #[inline(always)]
        pub const fn set_latch0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Latch1."]
        #[must_use]
        #[inline(always)]
        pub const fn latch1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Latch1."]
        #[inline(always)]
        pub const fn set_latch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Latch2."]
        #[must_use]
        #[inline(always)]
        pub const fn latch2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Latch2."]
        #[inline(always)]
        pub const fn set_latch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Latch3."]
        #[must_use]
        #[inline(always)]
        pub const fn latch3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Latch3."]
        #[inline(always)]
        pub const fn set_latch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Sample error."]
        #[must_use]
        #[inline(always)]
        pub const fn smp_err(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Sample error."]
        #[inline(always)]
        pub const fn set_smp_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Trigger0."]
        #[must_use]
        #[inline(always)]
        pub const fn triger0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger0."]
        #[inline(always)]
        pub const fn set_triger0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Trigger1."]
        #[must_use]
        #[inline(always)]
        pub const fn triger1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger1."]
        #[inline(always)]
        pub const fn set_triger1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Trigger2."]
        #[must_use]
        #[inline(always)]
        pub const fn triger2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger2."]
        #[inline(always)]
        pub const fn set_triger2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Trigger3."]
        #[must_use]
        #[inline(always)]
        pub const fn triger3(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger3."]
        #[inline(always)]
        pub const fn set_triger3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Trigger0 failed."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_err0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger0 failed."]
        #[inline(always)]
        pub const fn set_trg_err0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Trigger1 failed."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_err1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger1 failed."]
        #[inline(always)]
        pub const fn set_trg_err1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Trigger2 failed."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_err2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger2 failed."]
        #[inline(always)]
        pub const fn set_trg_err2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Trigger3 failed."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_err3(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger3 failed."]
        #[inline(always)]
        pub const fn set_trg_err3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IrqIntFlag {
        #[inline(always)]
        fn default() -> IrqIntFlag {
            IrqIntFlag(0)
        }
    }
    impl core::fmt::Debug for IrqIntFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqIntFlag")
                .field("stall", &self.stall())
                .field("except", &self.except())
                .field("wdog", &self.wdog())
                .field("ptr0_st", &self.ptr0_st())
                .field("ptr1_st", &self.ptr1_st())
                .field("instr0_st", &self.instr0_st())
                .field("instr1_st", &self.instr1_st())
                .field("ptr0_end", &self.ptr0_end())
                .field("ptr1_end", &self.ptr1_end())
                .field("instr0_end", &self.instr0_end())
                .field("instr1_end", &self.instr1_end())
                .field("trx_err", &self.trx_err())
                .field("timeout", &self.timeout())
                .field("latch0", &self.latch0())
                .field("latch1", &self.latch1())
                .field("latch2", &self.latch2())
                .field("latch3", &self.latch3())
                .field("smp_err", &self.smp_err())
                .field("triger0", &self.triger0())
                .field("triger1", &self.triger1())
                .field("triger2", &self.triger2())
                .field("triger3", &self.triger3())
                .field("trg_err0", &self.trg_err0())
                .field("trg_err1", &self.trg_err1())
                .field("trg_err2", &self.trg_err2())
                .field("trg_err3", &self.trg_err3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqIntFlag {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IrqIntFlag {{ stall: {=bool:?}, except: {=bool:?}, wdog: {=bool:?}, ptr0_st: {=bool:?}, ptr1_st: {=bool:?}, instr0_st: {=bool:?}, instr1_st: {=bool:?}, ptr0_end: {=bool:?}, ptr1_end: {=bool:?}, instr0_end: {=bool:?}, instr1_end: {=bool:?}, trx_err: {=bool:?}, timeout: {=bool:?}, latch0: {=bool:?}, latch1: {=bool:?}, latch2: {=bool:?}, latch3: {=bool:?}, smp_err: {=bool:?}, triger0: {=bool:?}, triger1: {=bool:?}, triger2: {=bool:?}, triger3: {=bool:?}, trg_err0: {=bool:?}, trg_err1: {=bool:?}, trg_err2: {=bool:?}, trg_err3: {=bool:?} }}" , self . stall () , self . except () , self . wdog () , self . ptr0_st () , self . ptr1_st () , self . instr0_st () , self . instr1_st () , self . ptr0_end () , self . ptr1_end () , self . instr0_end () , self . instr1_end () , self . trx_err () , self . timeout () , self . latch0 () , self . latch1 () , self . latch2 () , self . latch3 () , self . smp_err () , self . triger0 () , self . triger1 () , self . triger2 () , self . triger3 () , self . trg_err0 () , self . trg_err1 () , self . trg_err2 () , self . trg_err3 ())
        }
    }
    #[doc = "Interrupt status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqIntSts(pub u32);
    impl IrqIntSts {
        #[doc = "Stall."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Stall."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Exception."]
        #[must_use]
        #[inline(always)]
        pub const fn except(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Exception."]
        #[inline(always)]
        pub const fn set_except(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Watch dog."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Watch dog."]
        #[inline(always)]
        pub const fn set_wdog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Pointer 0 start."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr0_st(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Pointer 0 start."]
        #[inline(always)]
        pub const fn set_ptr0_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Pointer 1 start."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr1_st(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Pointer 1 start."]
        #[inline(always)]
        pub const fn set_ptr1_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Instruction 0 start."]
        #[must_use]
        #[inline(always)]
        pub const fn instr0_st(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction 0 start."]
        #[inline(always)]
        pub const fn set_instr0_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Instruction 1 start."]
        #[must_use]
        #[inline(always)]
        pub const fn instr1_st(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction 1 start."]
        #[inline(always)]
        pub const fn set_instr1_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Pointer 0 end."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr0_end(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Pointer 0 end."]
        #[inline(always)]
        pub const fn set_ptr0_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Pointer 1 end."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr1_end(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Pointer 1 end."]
        #[inline(always)]
        pub const fn set_ptr1_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Instruction 0 end."]
        #[must_use]
        #[inline(always)]
        pub const fn instr0_end(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction 0 end."]
        #[inline(always)]
        pub const fn set_instr0_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Instruction 1 end."]
        #[must_use]
        #[inline(always)]
        pub const fn instr1_end(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction 1 end."]
        #[inline(always)]
        pub const fn set_instr1_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Transfer error."]
        #[must_use]
        #[inline(always)]
        pub const fn trx_err(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer error."]
        #[inline(always)]
        pub const fn set_trx_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn timeout(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout."]
        #[inline(always)]
        pub const fn set_timeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Latch0."]
        #[must_use]
        #[inline(always)]
        pub const fn latch0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Latch0."]
        #[inline(always)]
        pub const fn set_latch0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Latch1."]
        #[must_use]
        #[inline(always)]
        pub const fn latch1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Latch1."]
        #[inline(always)]
        pub const fn set_latch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Latch2."]
        #[must_use]
        #[inline(always)]
        pub const fn latch2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Latch2."]
        #[inline(always)]
        pub const fn set_latch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Latch3."]
        #[must_use]
        #[inline(always)]
        pub const fn latch3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Latch3."]
        #[inline(always)]
        pub const fn set_latch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Sample error."]
        #[must_use]
        #[inline(always)]
        pub const fn smp_err(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Sample error."]
        #[inline(always)]
        pub const fn set_smp_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Trigger0."]
        #[must_use]
        #[inline(always)]
        pub const fn triger0(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger0."]
        #[inline(always)]
        pub const fn set_triger0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Trigger1."]
        #[must_use]
        #[inline(always)]
        pub const fn triger1(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger1."]
        #[inline(always)]
        pub const fn set_triger1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Trigger2."]
        #[must_use]
        #[inline(always)]
        pub const fn triger2(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger2."]
        #[inline(always)]
        pub const fn set_triger2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Trigger3."]
        #[must_use]
        #[inline(always)]
        pub const fn triger3(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger3."]
        #[inline(always)]
        pub const fn set_triger3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Trigger0 failed."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_err0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger0 failed."]
        #[inline(always)]
        pub const fn set_trg_err0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Trigger1 failed."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_err1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger1 failed."]
        #[inline(always)]
        pub const fn set_trg_err1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Trigger2 failed."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_err2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger2 failed."]
        #[inline(always)]
        pub const fn set_trg_err2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Trigger3 failed."]
        #[must_use]
        #[inline(always)]
        pub const fn trg_err3(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger3 failed."]
        #[inline(always)]
        pub const fn set_trg_err3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IrqIntSts {
        #[inline(always)]
        fn default() -> IrqIntSts {
            IrqIntSts(0)
        }
    }
    impl core::fmt::Debug for IrqIntSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqIntSts")
                .field("stall", &self.stall())
                .field("except", &self.except())
                .field("wdog", &self.wdog())
                .field("ptr0_st", &self.ptr0_st())
                .field("ptr1_st", &self.ptr1_st())
                .field("instr0_st", &self.instr0_st())
                .field("instr1_st", &self.instr1_st())
                .field("ptr0_end", &self.ptr0_end())
                .field("ptr1_end", &self.ptr1_end())
                .field("instr0_end", &self.instr0_end())
                .field("instr1_end", &self.instr1_end())
                .field("trx_err", &self.trx_err())
                .field("timeout", &self.timeout())
                .field("latch0", &self.latch0())
                .field("latch1", &self.latch1())
                .field("latch2", &self.latch2())
                .field("latch3", &self.latch3())
                .field("smp_err", &self.smp_err())
                .field("triger0", &self.triger0())
                .field("triger1", &self.triger1())
                .field("triger2", &self.triger2())
                .field("triger3", &self.triger3())
                .field("trg_err0", &self.trg_err0())
                .field("trg_err1", &self.trg_err1())
                .field("trg_err2", &self.trg_err2())
                .field("trg_err3", &self.trg_err3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqIntSts {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IrqIntSts {{ stall: {=bool:?}, except: {=bool:?}, wdog: {=bool:?}, ptr0_st: {=bool:?}, ptr1_st: {=bool:?}, instr0_st: {=bool:?}, instr1_st: {=bool:?}, ptr0_end: {=bool:?}, ptr1_end: {=bool:?}, instr0_end: {=bool:?}, instr1_end: {=bool:?}, trx_err: {=bool:?}, timeout: {=bool:?}, latch0: {=bool:?}, latch1: {=bool:?}, latch2: {=bool:?}, latch3: {=bool:?}, smp_err: {=bool:?}, triger0: {=bool:?}, triger1: {=bool:?}, triger2: {=bool:?}, triger3: {=bool:?}, trg_err0: {=bool:?}, trg_err1: {=bool:?}, trg_err2: {=bool:?}, trg_err3: {=bool:?} }}" , self . stall () , self . except () , self . wdog () , self . ptr0_st () , self . ptr1_st () , self . instr0_st () , self . instr1_st () , self . ptr0_end () , self . ptr1_end () , self . instr0_end () , self . instr1_end () , self . trx_err () , self . timeout () , self . latch0 () , self . latch1 () , self . latch2 () , self . latch3 () , self . smp_err () , self . triger0 () , self . triger1 () , self . triger2 () , self . triger3 () , self . trg_err0 () , self . trg_err1 () , self . trg_err2 () , self . trg_err3 ())
        }
    }
    #[doc = "Match pointer 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqPointer0(pub u32);
    impl IrqPointer0 {
        #[doc = "Match pointer 0."]
        #[must_use]
        #[inline(always)]
        pub const fn pointer(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Match pointer 0."]
        #[inline(always)]
        pub const fn set_pointer(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for IrqPointer0 {
        #[inline(always)]
        fn default() -> IrqPointer0 {
            IrqPointer0(0)
        }
    }
    impl core::fmt::Debug for IrqPointer0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqPointer0")
                .field("pointer", &self.pointer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqPointer0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqPointer0 {{ pointer: {=u8:?} }}", self.pointer())
        }
    }
    #[doc = "Match pointer 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrqPointer1(pub u32);
    impl IrqPointer1 {
        #[doc = "Match pointer 1."]
        #[must_use]
        #[inline(always)]
        pub const fn pointer(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Match pointer 1."]
        #[inline(always)]
        pub const fn set_pointer(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for IrqPointer1 {
        #[inline(always)]
        fn default() -> IrqPointer1 {
            IrqPointer1(0)
        }
    }
    impl core::fmt::Debug for IrqPointer1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IrqPointer1")
                .field("pointer", &self.pointer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IrqPointer1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IrqPointer1 {{ pointer: {=u8:?} }}", self.pointer())
        }
    }
    #[doc = "command end value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Max(pub u32);
    impl Max {
        #[doc = "maximum command value."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd_max(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "maximum command value."]
        #[inline(always)]
        pub const fn set_cmd_max(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Max {
        #[inline(always)]
        fn default() -> Max {
            Max(0)
        }
    }
    impl core::fmt::Debug for Max {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Max")
                .field("cmd_max", &self.cmd_max())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Max {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Max {{ cmd_max: {=u32:?} }}", self.cmd_max())
        }
    }
    #[doc = "command start value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Min(pub u32);
    impl Min {
        #[doc = "minimum command value."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd_min(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "minimum command value."]
        #[inline(always)]
        pub const fn set_cmd_min(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Min {
        #[inline(always)]
        fn default() -> Min {
            Min(0)
        }
    }
    impl core::fmt::Debug for Min {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Min")
                .field("cmd_min", &self.cmd_min())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Min {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Min {{ cmd_min: {=u32:?} }}", self.cmd_min())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mode(pub u32);
    impl Mode {
        #[doc = "Data mode 0: data mode 1: check mode 2: CRC mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Data mode 0: data mode 1: check mode 2: CRC mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Write 1 to rewind read/write pointer, this is a self clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rewind(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 to rewind read/write pointer, this is a self clear bit."]
        #[inline(always)]
        pub const fn set_rewind(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Signed 0: unsigned value 1: signed value."]
        #[must_use]
        #[inline(always)]
        pub const fn signed(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Signed 0: unsigned value 1: signed value."]
        #[inline(always)]
        pub const fn set_signed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "bit order 0: LSB first 1: MSB first."]
        #[must_use]
        #[inline(always)]
        pub const fn border(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "bit order 0: LSB first 1: MSB first."]
        #[inline(always)]
        pub const fn set_border(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "word order 0: sample as bit order 1: different from bit order."]
        #[must_use]
        #[inline(always)]
        pub const fn worder(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "word order 0: sample as bit order 1: different from bit order."]
        #[inline(always)]
        pub const fn set_worder(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CRC invert 0: use CRC 1: use inverted CRC."]
        #[must_use]
        #[inline(always)]
        pub const fn crc_inv(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC invert 0: use CRC 1: use inverted CRC."]
        #[inline(always)]
        pub const fn set_crc_inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CRC shift mode, this mode is used to perform repeat code check 0: CRC 1: shift mode."]
        #[must_use]
        #[inline(always)]
        pub const fn crc_shift(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CRC shift mode, this mode is used to perform repeat code check 0: CRC 1: shift mode."]
        #[inline(always)]
        pub const fn set_crc_shift(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "word length 0: 1 bit 1: 2 bit ... 31: 32 bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wlen(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "word length 0: 1 bit 1: 2 bit ... 31: 32 bit."]
        #[inline(always)]
        pub const fn set_wlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "CRC length 0: 1 bit 1: 2 bit ... 31: 32 bit."]
        #[must_use]
        #[inline(always)]
        pub const fn crc_len(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "CRC length 0: 1 bit 1: 2 bit ... 31: 32 bit."]
        #[inline(always)]
        pub const fn set_crc_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Mode {
        #[inline(always)]
        fn default() -> Mode {
            Mode(0)
        }
    }
    impl core::fmt::Debug for Mode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mode")
                .field("mode", &self.mode())
                .field("rewind", &self.rewind())
                .field("signed", &self.signed())
                .field("border", &self.border())
                .field("worder", &self.worder())
                .field("crc_inv", &self.crc_inv())
                .field("crc_shift", &self.crc_shift())
                .field("wlen", &self.wlen())
                .field("crc_len", &self.crc_len())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mode {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mode {{ mode: {=u8:?}, rewind: {=bool:?}, signed: {=bool:?}, border: {=bool:?}, worder: {=bool:?}, crc_inv: {=bool:?}, crc_shift: {=bool:?}, wlen: {=u8:?}, crc_len: {=u8:?} }}" , self . mode () , self . rewind () , self . signed () , self . border () , self . worder () , self . crc_inv () , self . crc_shift () , self . wlen () , self . crc_len ())
        }
    }
    #[doc = "command compare bit enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msk(pub u32);
    impl Msk {
        #[doc = "compare mask."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd_mask(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "compare mask."]
        #[inline(always)]
        pub const fn set_cmd_mask(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Msk {
        #[inline(always)]
        fn default() -> Msk {
            Msk(0)
        }
    }
    impl core::fmt::Debug for Msk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Msk")
                .field("cmd_mask", &self.cmd_mask())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Msk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Msk {{ cmd_mask: {=u32:?} }}", self.cmd_mask())
        }
    }
    #[doc = "Data output."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Out(pub u32);
    impl Out {
        #[doc = "Data output."]
        #[must_use]
        #[inline(always)]
        pub const fn data_out(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data output."]
        #[inline(always)]
        pub const fn set_data_out(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Out {
        #[inline(always)]
        fn default() -> Out {
            Out(0)
        }
    }
    impl core::fmt::Debug for Out {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Out")
                .field("data_out", &self.data_out())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Out {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Out {{ data_out: {=u32:?} }}", self.data_out())
        }
    }
    #[doc = "Input accelerate."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosAccIn(pub u32);
    impl PosAccIn {
        #[doc = "Input accelerate."]
        #[must_use]
        #[inline(always)]
        pub const fn acc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Input accelerate."]
        #[inline(always)]
        pub const fn set_acc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosAccIn {
        #[inline(always)]
        fn default() -> PosAccIn {
            PosAccIn(0)
        }
    }
    impl core::fmt::Debug for PosAccIn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosAccIn")
                .field("acc", &self.acc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosAccIn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosAccIn {{ acc: {=u32:?} }}", self.acc())
        }
    }
    #[doc = "Input position."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosPosIn(pub u32);
    impl PosPosIn {
        #[doc = "Input position."]
        #[must_use]
        #[inline(always)]
        pub const fn pos(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Input position."]
        #[inline(always)]
        pub const fn set_pos(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosPosIn {
        #[inline(always)]
        fn default() -> PosPosIn {
            PosPosIn(0)
        }
    }
    impl core::fmt::Debug for PosPosIn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosPosIn")
                .field("pos", &self.pos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosPosIn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosPosIn {{ pos: {=u32:?} }}", self.pos())
        }
    }
    #[doc = "Input revolution."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosRevIn(pub u32);
    impl PosRevIn {
        #[doc = "Input revolution."]
        #[must_use]
        #[inline(always)]
        pub const fn rev(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Input revolution."]
        #[inline(always)]
        pub const fn set_rev(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosRevIn {
        #[inline(always)]
        fn default() -> PosRevIn {
            PosRevIn(0)
        }
    }
    impl core::fmt::Debug for PosRevIn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosRevIn")
                .field("rev", &self.rev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosRevIn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosRevIn {{ rev: {=u32:?} }}", self.rev())
        }
    }
    #[doc = "Sample override accelerate."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosSmpAcc(pub u32);
    impl PosSmpAcc {
        #[doc = "Sample override accelerate."]
        #[must_use]
        #[inline(always)]
        pub const fn acc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Sample override accelerate."]
        #[inline(always)]
        pub const fn set_acc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosSmpAcc {
        #[inline(always)]
        fn default() -> PosSmpAcc {
            PosSmpAcc(0)
        }
    }
    impl core::fmt::Debug for PosSmpAcc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosSmpAcc")
                .field("acc", &self.acc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosSmpAcc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosSmpAcc {{ acc: {=u32:?} }}", self.acc())
        }
    }
    #[doc = "Sample configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosSmpCfg(pub u32);
    impl PosSmpCfg {
        #[doc = "Sample window, in clock cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn window(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Sample window, in clock cycle."]
        #[inline(always)]
        pub const fn set_window(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Latch selection 0: latch 0 1: latch 1 2: latch 2 3: latch 3."]
        #[must_use]
        #[inline(always)]
        pub const fn lat_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Latch selection 0: latch 0 1: latch 1 2: latch 2 3: latch 3."]
        #[inline(always)]
        pub const fn set_lat_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Sample one time 0: Sample during windows time 1: Close sample window after first sample."]
        #[must_use]
        #[inline(always)]
        pub const fn once(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Sample one time 0: Sample during windows time 1: Close sample window after first sample."]
        #[inline(always)]
        pub const fn set_once(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for PosSmpCfg {
        #[inline(always)]
        fn default() -> PosSmpCfg {
            PosSmpCfg(0)
        }
    }
    impl core::fmt::Debug for PosSmpCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosSmpCfg")
                .field("window", &self.window())
                .field("lat_sel", &self.lat_sel())
                .field("once", &self.once())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosSmpCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PosSmpCfg {{ window: {=u16:?}, lat_sel: {=u8:?}, once: {=bool:?} }}",
                self.window(),
                self.lat_sel(),
                self.once()
            )
        }
    }
    #[doc = "Sample data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosSmpDat(pub u32);
    impl PosSmpDat {
        #[doc = "Data register sampled, each bit represent a data register."]
        #[must_use]
        #[inline(always)]
        pub const fn dat_sel(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data register sampled, each bit represent a data register."]
        #[inline(always)]
        pub const fn set_dat_sel(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosSmpDat {
        #[inline(always)]
        fn default() -> PosSmpDat {
            PosSmpDat(0)
        }
    }
    impl core::fmt::Debug for PosSmpDat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosSmpDat")
                .field("dat_sel", &self.dat_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosSmpDat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosSmpDat {{ dat_sel: {=u32:?} }}", self.dat_sel())
        }
    }
    #[doc = "Sample selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosSmpEn(pub u32);
    impl PosSmpEn {
        #[doc = "Data register for position transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Data register for position transfer."]
        #[inline(always)]
        pub const fn set_pos_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Position include position."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Position include position."]
        #[inline(always)]
        pub const fn set_pos_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data register for revolution transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn rev_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data register for revolution transfer."]
        #[inline(always)]
        pub const fn set_rev_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Position include revolution."]
        #[must_use]
        #[inline(always)]
        pub const fn rev_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Position include revolution."]
        #[inline(always)]
        pub const fn set_rev_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Data register for speed transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn spd_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Data register for speed transfer."]
        #[inline(always)]
        pub const fn set_spd_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Position include speed."]
        #[must_use]
        #[inline(always)]
        pub const fn spd_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Position include speed."]
        #[inline(always)]
        pub const fn set_spd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Data register for acceleration transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data register for acceleration transfer."]
        #[inline(always)]
        pub const fn set_acc_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "Position include acceleration."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Position include acceleration."]
        #[inline(always)]
        pub const fn set_acc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PosSmpEn {
        #[inline(always)]
        fn default() -> PosSmpEn {
            PosSmpEn(0)
        }
    }
    impl core::fmt::Debug for PosSmpEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosSmpEn")
                .field("pos_sel", &self.pos_sel())
                .field("pos_en", &self.pos_en())
                .field("rev_sel", &self.rev_sel())
                .field("rev_en", &self.rev_en())
                .field("spd_sel", &self.spd_sel())
                .field("spd_en", &self.spd_en())
                .field("acc_sel", &self.acc_sel())
                .field("acc_en", &self.acc_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosSmpEn {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PosSmpEn {{ pos_sel: {=u8:?}, pos_en: {=bool:?}, rev_sel: {=u8:?}, rev_en: {=bool:?}, spd_sel: {=u8:?}, spd_en: {=bool:?}, acc_sel: {=u8:?}, acc_en: {=bool:?} }}" , self . pos_sel () , self . pos_en () , self . rev_sel () , self . rev_en () , self . spd_sel () , self . spd_en () , self . acc_sel () , self . acc_en ())
        }
    }
    #[doc = "Sample override position."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosSmpPos(pub u32);
    impl PosSmpPos {
        #[doc = "Sample override position."]
        #[must_use]
        #[inline(always)]
        pub const fn pos(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Sample override position."]
        #[inline(always)]
        pub const fn set_pos(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosSmpPos {
        #[inline(always)]
        fn default() -> PosSmpPos {
            PosSmpPos(0)
        }
    }
    impl core::fmt::Debug for PosSmpPos {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosSmpPos")
                .field("pos", &self.pos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosSmpPos {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosSmpPos {{ pos: {=u32:?} }}", self.pos())
        }
    }
    #[doc = "Sample override revolution."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosSmpRev(pub u32);
    impl PosSmpRev {
        #[doc = "Sample override revolution."]
        #[must_use]
        #[inline(always)]
        pub const fn rev(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Sample override revolution."]
        #[inline(always)]
        pub const fn set_rev(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosSmpRev {
        #[inline(always)]
        fn default() -> PosSmpRev {
            PosSmpRev(0)
        }
    }
    impl core::fmt::Debug for PosSmpRev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosSmpRev")
                .field("rev", &self.rev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosSmpRev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosSmpRev {{ rev: {=u32:?} }}", self.rev())
        }
    }
    #[doc = "Sample override speed."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosSmpSpd(pub u32);
    impl PosSmpSpd {
        #[doc = "Sample override speed."]
        #[must_use]
        #[inline(always)]
        pub const fn spd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Sample override speed."]
        #[inline(always)]
        pub const fn set_spd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosSmpSpd {
        #[inline(always)]
        fn default() -> PosSmpSpd {
            PosSmpSpd(0)
        }
    }
    impl core::fmt::Debug for PosSmpSpd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosSmpSpd")
                .field("spd", &self.spd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosSmpSpd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosSmpSpd {{ spd: {=u32:?} }}", self.spd())
        }
    }
    #[doc = "Sample status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosSmpSts(pub u32);
    impl PosSmpSts {
        #[doc = "Sample window counter."]
        #[must_use]
        #[inline(always)]
        pub const fn win_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Sample window counter."]
        #[inline(always)]
        pub const fn set_win_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Sample occured 0: Sample not happened 1: Sample occured."]
        #[must_use]
        #[inline(always)]
        pub const fn occur(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Sample occured 0: Sample not happened 1: Sample occured."]
        #[inline(always)]
        pub const fn set_occur(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for PosSmpSts {
        #[inline(always)]
        fn default() -> PosSmpSts {
            PosSmpSts(0)
        }
    }
    impl core::fmt::Debug for PosSmpSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosSmpSts")
                .field("win_cnt", &self.win_cnt())
                .field("occur", &self.occur())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosSmpSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PosSmpSts {{ win_cnt: {=u16:?}, occur: {=bool:?} }}",
                self.win_cnt(),
                self.occur()
            )
        }
    }
    #[doc = "Sample valid."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosSmpVal(pub u32);
    impl PosSmpVal {
        #[doc = "Position include position."]
        #[must_use]
        #[inline(always)]
        pub const fn pos(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Position include position."]
        #[inline(always)]
        pub const fn set_pos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Position include revolution."]
        #[must_use]
        #[inline(always)]
        pub const fn rev(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Position include revolution."]
        #[inline(always)]
        pub const fn set_rev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Position include speed."]
        #[must_use]
        #[inline(always)]
        pub const fn spd(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Position include speed."]
        #[inline(always)]
        pub const fn set_spd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Position include acceleration."]
        #[must_use]
        #[inline(always)]
        pub const fn acc(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Position include acceleration."]
        #[inline(always)]
        pub const fn set_acc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PosSmpVal {
        #[inline(always)]
        fn default() -> PosSmpVal {
            PosSmpVal(0)
        }
    }
    impl core::fmt::Debug for PosSmpVal {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosSmpVal")
                .field("pos", &self.pos())
                .field("rev", &self.rev())
                .field("spd", &self.spd())
                .field("acc", &self.acc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosSmpVal {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PosSmpVal {{ pos: {=bool:?}, rev: {=bool:?}, spd: {=bool:?}, acc: {=bool:?} }}",
                self.pos(),
                self.rev(),
                self.spd(),
                self.acc()
            )
        }
    }
    #[doc = "Input speed."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosSpdIn(pub u32);
    impl PosSpdIn {
        #[doc = "Input speed."]
        #[must_use]
        #[inline(always)]
        pub const fn spd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Input speed."]
        #[inline(always)]
        pub const fn set_spd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosSpdIn {
        #[inline(always)]
        fn default() -> PosSpdIn {
            PosSpdIn(0)
        }
    }
    impl core::fmt::Debug for PosSpdIn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosSpdIn")
                .field("spd", &self.spd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosSpdIn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosSpdIn {{ spd: {=u32:?} }}", self.spd())
        }
    }
    #[doc = "input time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosTimeIn(pub u32);
    impl PosTimeIn {
        #[doc = "input time."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "input time."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosTimeIn {
        #[inline(always)]
        fn default() -> PosTimeIn {
            PosTimeIn(0)
        }
    }
    impl core::fmt::Debug for PosTimeIn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosTimeIn")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosTimeIn {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosTimeIn {{ time: {=u32:?} }}", self.time())
        }
    }
    #[doc = "Update override accelerate."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosUpdAcc(pub u32);
    impl PosUpdAcc {
        #[doc = "Update override accelerate."]
        #[must_use]
        #[inline(always)]
        pub const fn acc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Update override accelerate."]
        #[inline(always)]
        pub const fn set_acc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosUpdAcc {
        #[inline(always)]
        fn default() -> PosUpdAcc {
            PosUpdAcc(0)
        }
    }
    impl core::fmt::Debug for PosUpdAcc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosUpdAcc")
                .field("acc", &self.acc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosUpdAcc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosUpdAcc {{ acc: {=u32:?} }}", self.acc())
        }
    }
    #[doc = "Update configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosUpdCfg(pub u32);
    impl PosUpdCfg {
        #[doc = "Latch selection 0: latch 0 1: latch 1 2: latch 2 3: latch 3."]
        #[must_use]
        #[inline(always)]
        pub const fn lat_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Latch selection 0: latch 0 1: latch 1 2: latch 2 3: latch 3."]
        #[inline(always)]
        pub const fn set_lat_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Sample one time 0: Sample during windows time 1: Close sample window after first sample."]
        #[must_use]
        #[inline(always)]
        pub const fn onerr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Sample one time 0: Sample during windows time 1: Close sample window after first sample."]
        #[inline(always)]
        pub const fn set_onerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Use override time 0: use time sample from motor group 1: use override time."]
        #[must_use]
        #[inline(always)]
        pub const fn time_ovrd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Use override time 0: use time sample from motor group 1: use override time."]
        #[inline(always)]
        pub const fn set_time_ovrd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PosUpdCfg {
        #[inline(always)]
        fn default() -> PosUpdCfg {
            PosUpdCfg(0)
        }
    }
    impl core::fmt::Debug for PosUpdCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosUpdCfg")
                .field("lat_sel", &self.lat_sel())
                .field("onerr", &self.onerr())
                .field("time_ovrd", &self.time_ovrd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosUpdCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PosUpdCfg {{ lat_sel: {=u8:?}, onerr: {=bool:?}, time_ovrd: {=bool:?} }}",
                self.lat_sel(),
                self.onerr(),
                self.time_ovrd()
            )
        }
    }
    #[doc = "Update data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosUpdDat(pub u32);
    impl PosUpdDat {
        #[doc = "Data register sampled, each bit represent a data register."]
        #[must_use]
        #[inline(always)]
        pub const fn dat_sel(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data register sampled, each bit represent a data register."]
        #[inline(always)]
        pub const fn set_dat_sel(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosUpdDat {
        #[inline(always)]
        fn default() -> PosUpdDat {
            PosUpdDat(0)
        }
    }
    impl core::fmt::Debug for PosUpdDat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosUpdDat")
                .field("dat_sel", &self.dat_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosUpdDat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosUpdDat {{ dat_sel: {=u32:?} }}", self.dat_sel())
        }
    }
    #[doc = "Update configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosUpdEn(pub u32);
    impl PosUpdEn {
        #[doc = "Data register for position transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Data register for position transfer."]
        #[inline(always)]
        pub const fn set_pos_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Position include position."]
        #[must_use]
        #[inline(always)]
        pub const fn pos_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Position include position."]
        #[inline(always)]
        pub const fn set_pos_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data register for revolution transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn rev_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data register for revolution transfer."]
        #[inline(always)]
        pub const fn set_rev_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Position include revolution."]
        #[must_use]
        #[inline(always)]
        pub const fn rev_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Position include revolution."]
        #[inline(always)]
        pub const fn set_rev_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Data register for speed transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn spd_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Data register for speed transfer."]
        #[inline(always)]
        pub const fn set_spd_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Position include speed."]
        #[must_use]
        #[inline(always)]
        pub const fn spd_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Position include speed."]
        #[inline(always)]
        pub const fn set_spd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Data register for acceleration transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data register for acceleration transfer."]
        #[inline(always)]
        pub const fn set_acc_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "Position include acceleration."]
        #[must_use]
        #[inline(always)]
        pub const fn acc_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Position include acceleration."]
        #[inline(always)]
        pub const fn set_acc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PosUpdEn {
        #[inline(always)]
        fn default() -> PosUpdEn {
            PosUpdEn(0)
        }
    }
    impl core::fmt::Debug for PosUpdEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosUpdEn")
                .field("pos_sel", &self.pos_sel())
                .field("pos_en", &self.pos_en())
                .field("rev_sel", &self.rev_sel())
                .field("rev_en", &self.rev_en())
                .field("spd_sel", &self.spd_sel())
                .field("spd_en", &self.spd_en())
                .field("acc_sel", &self.acc_sel())
                .field("acc_en", &self.acc_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosUpdEn {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PosUpdEn {{ pos_sel: {=u8:?}, pos_en: {=bool:?}, rev_sel: {=u8:?}, rev_en: {=bool:?}, spd_sel: {=u8:?}, spd_en: {=bool:?}, acc_sel: {=u8:?}, acc_en: {=bool:?} }}" , self . pos_sel () , self . pos_en () , self . rev_sel () , self . rev_en () , self . spd_sel () , self . spd_en () , self . acc_sel () , self . acc_en ())
        }
    }
    #[doc = "Update override position."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosUpdPos(pub u32);
    impl PosUpdPos {
        #[doc = "Update override position."]
        #[must_use]
        #[inline(always)]
        pub const fn pos(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Update override position."]
        #[inline(always)]
        pub const fn set_pos(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosUpdPos {
        #[inline(always)]
        fn default() -> PosUpdPos {
            PosUpdPos(0)
        }
    }
    impl core::fmt::Debug for PosUpdPos {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosUpdPos")
                .field("pos", &self.pos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosUpdPos {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosUpdPos {{ pos: {=u32:?} }}", self.pos())
        }
    }
    #[doc = "Update override revolution."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosUpdRev(pub u32);
    impl PosUpdRev {
        #[doc = "Update override revolution."]
        #[must_use]
        #[inline(always)]
        pub const fn rev(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Update override revolution."]
        #[inline(always)]
        pub const fn set_rev(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosUpdRev {
        #[inline(always)]
        fn default() -> PosUpdRev {
            PosUpdRev(0)
        }
    }
    impl core::fmt::Debug for PosUpdRev {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosUpdRev")
                .field("rev", &self.rev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosUpdRev {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosUpdRev {{ rev: {=u32:?} }}", self.rev())
        }
    }
    #[doc = "Update override speed."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosUpdSpd(pub u32);
    impl PosUpdSpd {
        #[doc = "Update override speed."]
        #[must_use]
        #[inline(always)]
        pub const fn spd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Update override speed."]
        #[inline(always)]
        pub const fn set_spd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosUpdSpd {
        #[inline(always)]
        fn default() -> PosUpdSpd {
            PosUpdSpd(0)
        }
    }
    impl core::fmt::Debug for PosUpdSpd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosUpdSpd")
                .field("spd", &self.spd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosUpdSpd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosUpdSpd {{ spd: {=u32:?} }}", self.spd())
        }
    }
    #[doc = "Update status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosUpdSts(pub u32);
    impl PosUpdSts {
        #[doc = "Update error 0: data receive normally 1: data receive error."]
        #[must_use]
        #[inline(always)]
        pub const fn upd_err(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Update error 0: data receive normally 1: data receive error."]
        #[inline(always)]
        pub const fn set_upd_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for PosUpdSts {
        #[inline(always)]
        fn default() -> PosUpdSts {
            PosUpdSts(0)
        }
    }
    impl core::fmt::Debug for PosUpdSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosUpdSts")
                .field("upd_err", &self.upd_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosUpdSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosUpdSts {{ upd_err: {=bool:?} }}", self.upd_err())
        }
    }
    #[doc = "Update overide time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PosUpdTime(pub u32);
    impl PosUpdTime {
        #[doc = "Update override time."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Update override time."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PosUpdTime {
        #[inline(always)]
        fn default() -> PosUpdTime {
            PosUpdTime(0)
        }
    }
    impl core::fmt::Debug for PosUpdTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PosUpdTime")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PosUpdTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PosUpdTime {{ time: {=u32:?} }}", self.time())
        }
    }
    #[doc = "command pointer 0 - 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pta(pub u32);
    impl Pta {
        #[doc = "pointer0."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "pointer0."]
        #[inline(always)]
        pub const fn set_ptr0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "pointer1."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "pointer1."]
        #[inline(always)]
        pub const fn set_ptr1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "pointer2."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "pointer2."]
        #[inline(always)]
        pub const fn set_ptr2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "pointer3."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "pointer3."]
        #[inline(always)]
        pub const fn set_ptr3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Pta {
        #[inline(always)]
        fn default() -> Pta {
            Pta(0)
        }
    }
    impl core::fmt::Debug for Pta {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pta")
                .field("ptr0", &self.ptr0())
                .field("ptr1", &self.ptr1())
                .field("ptr2", &self.ptr2())
                .field("ptr3", &self.ptr3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pta {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pta {{ ptr0: {=u8:?}, ptr1: {=u8:?}, ptr2: {=u8:?}, ptr3: {=u8:?} }}",
                self.ptr0(),
                self.ptr1(),
                self.ptr2(),
                self.ptr3()
            )
        }
    }
    #[doc = "command pointer 4 - 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptb(pub u32);
    impl Ptb {
        #[doc = "pointer4."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "pointer4."]
        #[inline(always)]
        pub const fn set_ptr4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "pointer5."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr5(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "pointer5."]
        #[inline(always)]
        pub const fn set_ptr5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "pointer6."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr6(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "pointer6."]
        #[inline(always)]
        pub const fn set_ptr6(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "pointer7."]
        #[must_use]
        #[inline(always)]
        pub const fn ptr7(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "pointer7."]
        #[inline(always)]
        pub const fn set_ptr7(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Ptb {
        #[inline(always)]
        fn default() -> Ptb {
            Ptb(0)
        }
    }
    impl core::fmt::Debug for Ptb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptb")
                .field("ptr4", &self.ptr4())
                .field("ptr5", &self.ptr5())
                .field("ptr6", &self.ptr6())
                .field("ptr7", &self.ptr7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ptb {{ ptr4: {=u8:?}, ptr5: {=u8:?}, ptr6: {=u8:?}, ptr7: {=u8:?} }}",
                self.ptr4(),
                self.ptr5(),
                self.ptr6(),
                self.ptr7()
            )
        }
    }
    #[doc = "Data bit set."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Set(pub u32);
    impl Set {
        #[doc = "DATA bit set."]
        #[must_use]
        #[inline(always)]
        pub const fn data_set(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DATA bit set."]
        #[inline(always)]
        pub const fn set_data_set(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Set {
        #[inline(always)]
        fn default() -> Set {
            Set(0)
        }
    }
    impl core::fmt::Debug for Set {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Set")
                .field("data_set", &self.data_set())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Set {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Set {{ data_set: {=u32:?} }}", self.data_set())
        }
    }
    #[doc = "Latch time."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Time(pub u32);
    impl Time {
        #[doc = "Latch time."]
        #[must_use]
        #[inline(always)]
        pub const fn lat_time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Latch time."]
        #[inline(always)]
        pub const fn set_lat_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Time {
        #[inline(always)]
        fn default() -> Time {
            Time(0)
        }
    }
    impl core::fmt::Debug for Time {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Time")
                .field("lat_time", &self.lat_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Time {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Time {{ lat_time: {=u32:?} }}", self.lat_time())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tran(pub u32);
    impl Tran {
        #[doc = "override pointer check."]
        #[must_use]
        #[inline(always)]
        pub const fn ov_ptr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "override pointer check."]
        #[inline(always)]
        pub const fn set_ov_ptr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "override clock check."]
        #[must_use]
        #[inline(always)]
        pub const fn ov_clk(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "override clock check."]
        #[inline(always)]
        pub const fn set_ov_clk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "override TX data check."]
        #[must_use]
        #[inline(always)]
        pub const fn ov_txd(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "override TX data check."]
        #[inline(always)]
        pub const fn set_ov_txd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "override timeout check."]
        #[must_use]
        #[inline(always)]
        pub const fn ov_tm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "override timeout check."]
        #[inline(always)]
        pub const fn set_ov_tm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "pointer 0: match 1: not match 2:entry 3:leave."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_ptr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "pointer 0: match 1: not match 2:entry 3:leave."]
        #[inline(always)]
        pub const fn set_cfg_ptr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "clock 0: high 1: low 2: rise 3: fall."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_clk(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "clock 0: high 1: low 2: rise 3: fall."]
        #[inline(always)]
        pub const fn set_cfg_clk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "data send 0: high 1: low 2: rise 3: fall."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_txd(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "data send 0: high 1: low 2: rise 3: fall."]
        #[inline(always)]
        pub const fn set_cfg_txd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "timeout 0: high 1: low 2: rise 3: fall."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_tm(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "timeout 0: high 1: low 2: rise 3: fall."]
        #[inline(always)]
        pub const fn set_cfg_tm(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "pointer."]
        #[must_use]
        #[inline(always)]
        pub const fn pointer(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "pointer."]
        #[inline(always)]
        pub const fn set_pointer(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Tran {
        #[inline(always)]
        fn default() -> Tran {
            Tran(0)
        }
    }
    impl core::fmt::Debug for Tran {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tran")
                .field("ov_ptr", &self.ov_ptr())
                .field("ov_clk", &self.ov_clk())
                .field("ov_txd", &self.ov_txd())
                .field("ov_tm", &self.ov_tm())
                .field("cfg_ptr", &self.cfg_ptr())
                .field("cfg_clk", &self.cfg_clk())
                .field("cfg_txd", &self.cfg_txd())
                .field("cfg_tm", &self.cfg_tm())
                .field("pointer", &self.pointer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tran {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Tran {{ ov_ptr: {=bool:?}, ov_clk: {=bool:?}, ov_txd: {=bool:?}, ov_tm: {=bool:?}, cfg_ptr: {=u8:?}, cfg_clk: {=u8:?}, cfg_txd: {=u8:?}, cfg_tm: {=u8:?}, pointer: {=u8:?} }}" , self . ov_ptr () , self . ov_clk () , self . ov_txd () , self . ov_tm () , self . cfg_ptr () , self . cfg_clk () , self . cfg_txd () , self . cfg_tm () , self . pointer ())
        }
    }
    #[doc = "Trigger input configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgInCfg(pub u32);
    impl TrgInCfg {
        #[doc = "Trigger 0 sigal selection 0: trigger in 0 1: trigger in 1 ... 7: trigger in 7."]
        #[must_use]
        #[inline(always)]
        pub const fn in0_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Trigger 0 sigal selection 0: trigger in 0 1: trigger in 1 ... 7: trigger in 7."]
        #[inline(always)]
        pub const fn set_in0_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Enable trigger 0 0: disable trigger 1 1: enable trigger 1."]
        #[must_use]
        #[inline(always)]
        pub const fn in0_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger 0 0: disable trigger 1 1: enable trigger 1."]
        #[inline(always)]
        pub const fn set_in0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Trigger 1 sigal selection 0: trigger in 0 1: trigger in 1 ... 7: trigger in 7."]
        #[must_use]
        #[inline(always)]
        pub const fn in1_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Trigger 1 sigal selection 0: trigger in 0 1: trigger in 1 ... 7: trigger in 7."]
        #[inline(always)]
        pub const fn set_in1_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Enable trigger 1 0: disable trigger 1 1: enable trigger 1."]
        #[must_use]
        #[inline(always)]
        pub const fn in1_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger 1 0: disable trigger 1 1: enable trigger 1."]
        #[inline(always)]
        pub const fn set_in1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Synchronize sigal selection (tigger 2) 0: trigger in 0 1: trigger in 1 ... 7: trigger in 7."]
        #[must_use]
        #[inline(always)]
        pub const fn sync_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Synchronize sigal selection (tigger 2) 0: trigger in 0 1: trigger in 1 ... 7: trigger in 7."]
        #[inline(always)]
        pub const fn set_sync_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Enable period trigger (tigger 2) 0: periodical trigger disabled 1: periodical trigger enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn prd_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Enable period trigger (tigger 2) 0: periodical trigger disabled 1: periodical trigger enabled."]
        #[inline(always)]
        pub const fn set_prd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for TrgInCfg {
        #[inline(always)]
        fn default() -> TrgInCfg {
            TrgInCfg(0)
        }
    }
    impl core::fmt::Debug for TrgInCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrgInCfg")
                .field("in0_sel", &self.in0_sel())
                .field("in0_en", &self.in0_en())
                .field("in1_sel", &self.in1_sel())
                .field("in1_en", &self.in1_en())
                .field("sync_sel", &self.sync_sel())
                .field("prd_en", &self.prd_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrgInCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TrgInCfg {{ in0_sel: {=u8:?}, in0_en: {=bool:?}, in1_sel: {=u8:?}, in1_en: {=bool:?}, sync_sel: {=u8:?}, prd_en: {=bool:?} }}" , self . in0_sel () , self . in0_en () , self . in1_sel () , self . in1_en () , self . sync_sel () , self . prd_en ())
        }
    }
    #[doc = "Trigger output configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgOutCfg(pub u32);
    impl TrgOutCfg {
        #[doc = "Trigger 0 sigal selection 0: trigger out 0 1: trigger out 1 ... 7: trigger out 7."]
        #[must_use]
        #[inline(always)]
        pub const fn out0_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Trigger 0 sigal selection 0: trigger out 0 1: trigger out 1 ... 7: trigger out 7."]
        #[inline(always)]
        pub const fn set_out0_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Enable trigger 0 0: disable trigger 1 1: enable trigger 1."]
        #[must_use]
        #[inline(always)]
        pub const fn out0_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger 0 0: disable trigger 1 1: enable trigger 1."]
        #[inline(always)]
        pub const fn set_out0_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Trigger 1 sigal selection 0: trigger out 0 1: trigger out 1 ... 7: trigger out 7."]
        #[must_use]
        #[inline(always)]
        pub const fn out1_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Trigger 1 sigal selection 0: trigger out 0 1: trigger out 1 ... 7: trigger out 7."]
        #[inline(always)]
        pub const fn set_out1_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Enable trigger 1 0: disable trigger 1 1: enable trigger 1."]
        #[must_use]
        #[inline(always)]
        pub const fn out1_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger 1 0: disable trigger 1 1: enable trigger 1."]
        #[inline(always)]
        pub const fn set_out1_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Trigger 2 sigal selection 0: trigger out 0 1: trigger out 1 ... 7: trigger out 7."]
        #[must_use]
        #[inline(always)]
        pub const fn out2_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Trigger 2 sigal selection 0: trigger out 0 1: trigger out 1 ... 7: trigger out 7."]
        #[inline(always)]
        pub const fn set_out2_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Enable trigger 2 0: disable trigger 2 1: enable trigger 2."]
        #[must_use]
        #[inline(always)]
        pub const fn out2_en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger 2 0: disable trigger 2 1: enable trigger 2."]
        #[inline(always)]
        pub const fn set_out2_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Trigger 3 sigal selection 0: trigger out 0 1: trigger out 1 ... 7: trigger out 7."]
        #[must_use]
        #[inline(always)]
        pub const fn out3_sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Trigger 3 sigal selection 0: trigger out 0 1: trigger out 1 ... 7: trigger out 7."]
        #[inline(always)]
        pub const fn set_out3_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Enable trigger 3 0: disable trigger 3 1: enable trigger 3."]
        #[must_use]
        #[inline(always)]
        pub const fn out3_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable trigger 3 0: disable trigger 3 1: enable trigger 3."]
        #[inline(always)]
        pub const fn set_out3_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for TrgOutCfg {
        #[inline(always)]
        fn default() -> TrgOutCfg {
            TrgOutCfg(0)
        }
    }
    impl core::fmt::Debug for TrgOutCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrgOutCfg")
                .field("out0_sel", &self.out0_sel())
                .field("out0_en", &self.out0_en())
                .field("out1_sel", &self.out1_sel())
                .field("out1_en", &self.out1_en())
                .field("out2_sel", &self.out2_sel())
                .field("out2_en", &self.out2_en())
                .field("out3_sel", &self.out3_sel())
                .field("out3_en", &self.out3_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrgOutCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TrgOutCfg {{ out0_sel: {=u8:?}, out0_en: {=bool:?}, out1_sel: {=u8:?}, out1_en: {=bool:?}, out2_sel: {=u8:?}, out2_en: {=bool:?}, out3_sel: {=u8:?}, out3_en: {=bool:?} }}" , self . out0_sel () , self . out0_en () , self . out1_sel () , self . out1_en () , self . out2_sel () , self . out2_en () , self . out3_sel () , self . out3_en ())
        }
    }
    #[doc = "Trigger period."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgPrd(pub u32);
    impl TrgPrd {
        #[doc = "Trigger period."]
        #[must_use]
        #[inline(always)]
        pub const fn period(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Trigger period."]
        #[inline(always)]
        pub const fn set_period(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TrgPrd {
        #[inline(always)]
        fn default() -> TrgPrd {
            TrgPrd(0)
        }
    }
    impl core::fmt::Debug for TrgPrd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrgPrd")
                .field("period", &self.period())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrgPrd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TrgPrd {{ period: {=u32:?} }}", self.period())
        }
    }
    #[doc = "Period trigger configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgPrdCfg(pub u32);
    impl TrgPrdCfg {
        #[doc = "Synchronous 0: Not synchronous 1: Synchronous every trigger source."]
        #[must_use]
        #[inline(always)]
        pub const fn sync(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronous 0: Not synchronous 1: Synchronous every trigger source."]
        #[inline(always)]
        pub const fn set_sync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Wait for trigger synchronous before trigger 0: Trigger directly 1: Wait trigger source before period trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn arming(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Wait for trigger synchronous before trigger 0: Trigger directly 1: Wait trigger source before period trigger."]
        #[inline(always)]
        pub const fn set_arming(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for TrgPrdCfg {
        #[inline(always)]
        fn default() -> TrgPrdCfg {
            TrgPrdCfg(0)
        }
    }
    impl core::fmt::Debug for TrgPrdCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrgPrdCfg")
                .field("sync", &self.sync())
                .field("arming", &self.arming())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrgPrdCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TrgPrdCfg {{ sync: {=bool:?}, arming: {=bool:?} }}",
                self.sync(),
                self.arming()
            )
        }
    }
    #[doc = "Period trigger counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgPrdCnt(pub u32);
    impl TrgPrdCnt {
        #[doc = "Trigger period counter."]
        #[must_use]
        #[inline(always)]
        pub const fn period_cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Trigger period counter."]
        #[inline(always)]
        pub const fn set_period_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TrgPrdCnt {
        #[inline(always)]
        fn default() -> TrgPrdCnt {
            TrgPrdCnt(0)
        }
    }
    impl core::fmt::Debug for TrgPrdCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrgPrdCnt")
                .field("period_cnt", &self.period_cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrgPrdCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TrgPrdCnt {{ period_cnt: {=u32:?} }}", self.period_cnt())
        }
    }
    #[doc = "Period trigger status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgPrdSts(pub u32);
    impl TrgPrdSts {
        #[doc = "Waiting for trigger 0: Not in waiting status 1: In waiting status."]
        #[must_use]
        #[inline(always)]
        pub const fn armed(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Waiting for trigger 0: Not in waiting status 1: In waiting status."]
        #[inline(always)]
        pub const fn set_armed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Period has been triggered 0: Not triggered 1: Triggered."]
        #[must_use]
        #[inline(always)]
        pub const fn trigered(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Period has been triggered 0: Not triggered 1: Triggered."]
        #[inline(always)]
        pub const fn set_trigered(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for TrgPrdSts {
        #[inline(always)]
        fn default() -> TrgPrdSts {
            TrgPrdSts(0)
        }
    }
    impl core::fmt::Debug for TrgPrdSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrgPrdSts")
                .field("armed", &self.armed())
                .field("trigered", &self.trigered())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrgPrdSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TrgPrdSts {{ armed: {=bool:?}, trigered: {=bool:?} }}",
                self.armed(),
                self.trigered()
            )
        }
    }
    #[doc = "Software trigger."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgSw(pub u32);
    impl TrgSw {
        #[doc = "Software trigger (tigger 3). this bit is self-clear 0: trigger source disabled 1: trigger source enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn soft(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software trigger (tigger 3). this bit is self-clear 0: trigger source disabled 1: trigger source enabled."]
        #[inline(always)]
        pub const fn set_soft(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for TrgSw {
        #[inline(always)]
        fn default() -> TrgSw {
            TrgSw(0)
        }
    }
    impl core::fmt::Debug for TrgSw {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrgSw").field("soft", &self.soft()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrgSw {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TrgSw {{ soft: {=bool:?} }}", self.soft())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgTableCmd(pub u32);
    impl TrgTableCmd {
        #[doc = "Trigger command."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd_trigger0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Trigger command."]
        #[inline(always)]
        pub const fn set_cmd_trigger0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TrgTableCmd {
        #[inline(always)]
        fn default() -> TrgTableCmd {
            TrgTableCmd(0)
        }
    }
    impl core::fmt::Debug for TrgTableCmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrgTableCmd")
                .field("cmd_trigger0", &self.cmd_trigger0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrgTableCmd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TrgTableCmd {{ cmd_trigger0: {=u32:?} }}",
                self.cmd_trigger0()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrgTableTime(pub u32);
    impl TrgTableTime {
        #[doc = "Trigger time."]
        #[must_use]
        #[inline(always)]
        pub const fn trigger0_time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Trigger time."]
        #[inline(always)]
        pub const fn set_trigger0_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TrgTableTime {
        #[inline(always)]
        fn default() -> TrgTableTime {
            TrgTableTime(0)
        }
    }
    impl core::fmt::Debug for TrgTableTime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TrgTableTime")
                .field("trigger0_time", &self.trigger0_time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TrgTableTime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TrgTableTime {{ trigger0_time: {=u32:?} }}",
                self.trigger0_time()
            )
        }
    }
    #[doc = "Transceiver baud rate register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XcvrBaudCfg(pub u32);
    impl XcvrBaudCfg {
        #[doc = "Baud rate, bit time in system clock cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn baud_div(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Baud rate, bit time in system clock cycle."]
        #[inline(always)]
        pub const fn set_baud_div(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Baud synchronous time, minmum bit time."]
        #[must_use]
        #[inline(always)]
        pub const fn sync_point(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Baud synchronous time, minmum bit time."]
        #[inline(always)]
        pub const fn set_sync_point(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for XcvrBaudCfg {
        #[inline(always)]
        fn default() -> XcvrBaudCfg {
            XcvrBaudCfg(0)
        }
    }
    impl core::fmt::Debug for XcvrBaudCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("XcvrBaudCfg")
                .field("baud_div", &self.baud_div())
                .field("sync_point", &self.sync_point())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for XcvrBaudCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "XcvrBaudCfg {{ baud_div: {=u16:?}, sync_point: {=u16:?} }}",
                self.baud_div(),
                self.sync_point()
            )
        }
    }
    #[doc = "Transceiver clock timing configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XcvrClkCfg(pub u32);
    impl XcvrClkCfg {
        #[doc = "clock point 0 in system clcok cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn ck0_point(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "clock point 0 in system clcok cycle."]
        #[inline(always)]
        pub const fn set_ck0_point(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "clock point 1 in system clcok cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn ck1_point(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "clock point 1 in system clcok cycle."]
        #[inline(always)]
        pub const fn set_ck1_point(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for XcvrClkCfg {
        #[inline(always)]
        fn default() -> XcvrClkCfg {
            XcvrClkCfg(0)
        }
    }
    impl core::fmt::Debug for XcvrClkCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("XcvrClkCfg")
                .field("ck0_point", &self.ck0_point())
                .field("ck1_point", &self.ck1_point())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for XcvrClkCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "XcvrClkCfg {{ ck0_point: {=u16:?}, ck1_point: {=u16:?} }}",
                self.ck0_point(),
                self.ck1_point()
            )
        }
    }
    #[doc = "Transceiver control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XcvrCtrl(pub u32);
    impl XcvrCtrl {
        #[doc = "Tranceiver mode 0: synchronous maaster 1: synchronous slave 2: asynchronous mode 3: asynchronous mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Tranceiver mode 0: synchronous maaster 1: synchronous slave 2: asynchronous mode 3: asynchronous mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Restart tranceiver, this is a self clear bit 0: no effect 1: reset tranceiver."]
        #[must_use]
        #[inline(always)]
        pub const fn restart(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Restart tranceiver, this is a self clear bit 0: no effect 1: reset tranceiver."]
        #[inline(always)]
        pub const fn set_restart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear parity error, this is a self clear bit 0: no effect 1: clear parity error."]
        #[must_use]
        #[inline(always)]
        pub const fn par_clr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear parity error, this is a self clear bit 0: no effect 1: clear parity error."]
        #[inline(always)]
        pub const fn set_par_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Tipple sampe 0: sample 1 time for data transition 1: sample 3 times in receive and result in 2oo3."]
        #[must_use]
        #[inline(always)]
        pub const fn trismp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tipple sampe 0: sample 1 time for data transition 1: sample 3 times in receive and result in 2oo3."]
        #[inline(always)]
        pub const fn set_trismp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for XcvrCtrl {
        #[inline(always)]
        fn default() -> XcvrCtrl {
            XcvrCtrl(0)
        }
    }
    impl core::fmt::Debug for XcvrCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("XcvrCtrl")
                .field("mode", &self.mode())
                .field("restart", &self.restart())
                .field("par_clr", &self.par_clr())
                .field("trismp", &self.trismp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for XcvrCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "XcvrCtrl {{ mode: {=u8:?}, restart: {=bool:?}, par_clr: {=bool:?}, trismp: {=bool:?} }}" , self . mode () , self . restart () , self . par_clr () , self . trismp ())
        }
    }
    #[doc = "Transceiver data timing configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XcvrDataCfg(pub u32);
    impl XcvrDataCfg {
        #[doc = "data receive point in system clcok cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn rxd_point(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "data receive point in system clcok cycle."]
        #[inline(always)]
        pub const fn set_rxd_point(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "data transmit point in system clcok cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn txd_point(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "data transmit point in system clcok cycle."]
        #[inline(always)]
        pub const fn set_txd_point(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for XcvrDataCfg {
        #[inline(always)]
        fn default() -> XcvrDataCfg {
            XcvrDataCfg(0)
        }
    }
    impl core::fmt::Debug for XcvrDataCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("XcvrDataCfg")
                .field("rxd_point", &self.rxd_point())
                .field("txd_point", &self.txd_point())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for XcvrDataCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "XcvrDataCfg {{ rxd_point: {=u16:?}, txd_point: {=u16:?} }}",
                self.rxd_point(),
                self.txd_point()
            )
        }
    }
    #[doc = "Transceiver pin status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XcvrPin(pub u32);
    impl XcvrPin {
        #[doc = "TX output 0: data 0 1: data 1."]
        #[must_use]
        #[inline(always)]
        pub const fn do_tx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TX output 0: data 0 1: data 1."]
        #[inline(always)]
        pub const fn set_do_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TX state 0: data 0 1: data 1."]
        #[must_use]
        #[inline(always)]
        pub const fn di_tx(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX state 0: data 0 1: data 1."]
        #[inline(always)]
        pub const fn set_di_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TX drive state 0: input 1: output."]
        #[must_use]
        #[inline(always)]
        pub const fn oe_tx(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TX drive state 0: input 1: output."]
        #[inline(always)]
        pub const fn set_oe_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DE output 0: data 0 1: data 1."]
        #[must_use]
        #[inline(always)]
        pub const fn do_de(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DE output 0: data 0 1: data 1."]
        #[inline(always)]
        pub const fn set_do_de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DE state 0: data 0 1: data 1."]
        #[must_use]
        #[inline(always)]
        pub const fn di_de(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DE state 0: data 0 1: data 1."]
        #[inline(always)]
        pub const fn set_di_de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "DE drive state 0: input 1: output."]
        #[must_use]
        #[inline(always)]
        pub const fn oe_de(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DE drive state 0: input 1: output."]
        #[inline(always)]
        pub const fn set_oe_de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "RX output 0: data 0 1: data 1."]
        #[must_use]
        #[inline(always)]
        pub const fn do_rx(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RX output 0: data 0 1: data 1."]
        #[inline(always)]
        pub const fn set_do_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RX state 0: data 0 1: data 1."]
        #[must_use]
        #[inline(always)]
        pub const fn di_rx(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RX state 0: data 0 1: data 1."]
        #[inline(always)]
        pub const fn set_di_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RX drive state 0: input 1: output."]
        #[must_use]
        #[inline(always)]
        pub const fn oe_rx(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RX drive state 0: input 1: output."]
        #[inline(always)]
        pub const fn set_oe_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "CK output 0: data 0 1: data 1."]
        #[must_use]
        #[inline(always)]
        pub const fn do_ck(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CK output 0: data 0 1: data 1."]
        #[inline(always)]
        pub const fn set_do_ck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CK state 0: data 0 1: data 1."]
        #[must_use]
        #[inline(always)]
        pub const fn di_ck(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CK state 0: data 0 1: data 1."]
        #[inline(always)]
        pub const fn set_di_ck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CK drive state 0: input 1: output."]
        #[must_use]
        #[inline(always)]
        pub const fn oe_ck(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CK drive state 0: input 1: output."]
        #[inline(always)]
        pub const fn set_oe_ck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for XcvrPin {
        #[inline(always)]
        fn default() -> XcvrPin {
            XcvrPin(0)
        }
    }
    impl core::fmt::Debug for XcvrPin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("XcvrPin")
                .field("do_tx", &self.do_tx())
                .field("di_tx", &self.di_tx())
                .field("oe_tx", &self.oe_tx())
                .field("do_de", &self.do_de())
                .field("di_de", &self.di_de())
                .field("oe_de", &self.oe_de())
                .field("do_rx", &self.do_rx())
                .field("di_rx", &self.di_rx())
                .field("oe_rx", &self.oe_rx())
                .field("do_ck", &self.do_ck())
                .field("di_ck", &self.di_ck())
                .field("oe_ck", &self.oe_ck())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for XcvrPin {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "XcvrPin {{ do_tx: {=bool:?}, di_tx: {=bool:?}, oe_tx: {=bool:?}, do_de: {=bool:?}, di_de: {=bool:?}, oe_de: {=bool:?}, do_rx: {=bool:?}, di_rx: {=bool:?}, oe_rx: {=bool:?}, do_ck: {=bool:?}, di_ck: {=bool:?}, oe_ck: {=bool:?} }}" , self . do_tx () , self . di_tx () , self . oe_tx () , self . do_de () , self . di_de () , self . oe_de () , self . do_rx () , self . di_rx () , self . oe_rx () , self . do_ck () , self . di_ck () , self . oe_ck ())
        }
    }
    #[doc = "FSM of asynchronous."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XcvrState(pub u32);
    impl XcvrState {
        #[doc = "FSM of asynchronous transmit."]
        #[must_use]
        #[inline(always)]
        pub const fn send_state(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "FSM of asynchronous transmit."]
        #[inline(always)]
        pub const fn set_send_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "FSM of asynchronous receive."]
        #[must_use]
        #[inline(always)]
        pub const fn recv_state(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "FSM of asynchronous receive."]
        #[inline(always)]
        pub const fn set_recv_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for XcvrState {
        #[inline(always)]
        fn default() -> XcvrState {
            XcvrState(0)
        }
    }
    impl core::fmt::Debug for XcvrState {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("XcvrState")
                .field("send_state", &self.send_state())
                .field("recv_state", &self.recv_state())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for XcvrState {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "XcvrState {{ send_state: {=u8:?}, recv_state: {=u8:?} }}",
                self.send_state(),
                self.recv_state()
            )
        }
    }
    #[doc = "Transceiver configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XcvrTypeCfg(pub u32);
    impl XcvrTypeCfg {
        #[doc = "Idle state value of clock line 0: data'0' 1: data'1'."]
        #[must_use]
        #[inline(always)]
        pub const fn ck_idlev(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Idle state value of clock line 0: data'0' 1: data'1'."]
        #[inline(always)]
        pub const fn set_ck_idlev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Idle state value of data line 0: data'0' 1: data'1'."]
        #[must_use]
        #[inline(always)]
        pub const fn da_idlev(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Idle state value of data line 0: data'0' 1: data'1'."]
        #[inline(always)]
        pub const fn set_da_idlev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Idle state driver of clock line 0: output 1: high-Z."]
        #[must_use]
        #[inline(always)]
        pub const fn ck_idlez(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Idle state driver of clock line 0: output 1: high-Z."]
        #[inline(always)]
        pub const fn set_ck_idlez(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Idle state driver of data line 0: output 1: high-Z."]
        #[must_use]
        #[inline(always)]
        pub const fn da_idlez(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Idle state driver of data line 0: output 1: high-Z."]
        #[inline(always)]
        pub const fn set_da_idlez(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "enable parity check for asynchronous mode 0: disable 1: enable."]
        #[must_use]
        #[inline(always)]
        pub const fn par_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "enable parity check for asynchronous mode 0: disable 1: enable."]
        #[inline(always)]
        pub const fn set_par_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Polarity of parity for asynchronous mode 0: even 1: odd."]
        #[must_use]
        #[inline(always)]
        pub const fn par_pol(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Polarity of parity for asynchronous mode 0: even 1: odd."]
        #[inline(always)]
        pub const fn set_par_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Number of data bit for asynchronous mode 0: 1 bit 1: 2 bit ... 31: 32 bit."]
        #[must_use]
        #[inline(always)]
        pub const fn data_len(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of data bit for asynchronous mode 0: 1 bit 1: 2 bit ... 31: 32 bit."]
        #[inline(always)]
        pub const fn set_data_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Number of extra stop bit for asynchronous mode 0: 1 bit 1: 2 bit ... 255: 256 bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wait_len(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Number of extra stop bit for asynchronous mode 0: 1 bit 1: 2 bit ... 255: 256 bit."]
        #[inline(always)]
        pub const fn set_wait_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for XcvrTypeCfg {
        #[inline(always)]
        fn default() -> XcvrTypeCfg {
            XcvrTypeCfg(0)
        }
    }
    impl core::fmt::Debug for XcvrTypeCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("XcvrTypeCfg")
                .field("ck_idlev", &self.ck_idlev())
                .field("da_idlev", &self.da_idlev())
                .field("ck_idlez", &self.ck_idlez())
                .field("da_idlez", &self.da_idlez())
                .field("par_en", &self.par_en())
                .field("par_pol", &self.par_pol())
                .field("data_len", &self.data_len())
                .field("wait_len", &self.wait_len())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for XcvrTypeCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "XcvrTypeCfg {{ ck_idlev: {=bool:?}, da_idlev: {=bool:?}, ck_idlez: {=bool:?}, da_idlez: {=bool:?}, par_en: {=bool:?}, par_pol: {=bool:?}, data_len: {=u8:?}, wait_len: {=u8:?} }}" , self . ck_idlev () , self . da_idlev () , self . ck_idlez () , self . da_idlez () , self . par_en () , self . par_pol () , self . data_len () , self . wait_len ())
        }
    }
}
