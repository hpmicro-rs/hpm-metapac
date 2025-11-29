#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bin {
    ptr: *mut u8,
}
unsafe impl Send for Bin {}
unsafe impl Sync for Bin {}
impl Bin {
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
    pub const fn txdata(self, n: usize) -> crate::common::Reg<regs::Txdata, crate::common::RW> {
        assert!(n < 60usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsyn_txbuf_bin0_tque_and_tx_len(
        self,
    ) -> crate::common::Reg<regs::TsynTxbufBin0TqueAndTxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsyn_txbuf_bin0_tx_timestamp_l(
        self,
    ) -> crate::common::Reg<regs::TsynTxbufBin0TxTimestampL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsyn_txbuf_bin0_tx_timestamp_h(
        self,
    ) -> crate::common::Reg<regs::TsynTxbufBin0TxTimestampH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mac {
    ptr: *mut u8,
}
unsafe impl Send for Mac {}
unsafe impl Sync for Mac {}
impl Mac {
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
    pub const fn mac_ver(self) -> crate::common::Reg<regs::MacVer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn mac_macaddr_l(self) -> crate::common::Reg<regs::MacMacaddrL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn mac_macaddr_h(self) -> crate::common::Reg<regs::MacMacaddrH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn mac_mac_ctrl(self) -> crate::common::Reg<regs::MacMacCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn mac_tx_frames(self) -> crate::common::Reg<regs::MacTxFrames, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn mac_rx_frames(self) -> crate::common::Reg<regs::MacRxFrames, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn mac_tx_octets(self) -> crate::common::Reg<regs::MacTxOctets, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn mac_rx_octets(self) -> crate::common::Reg<regs::MacRxOctets, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn mac_mdio_cfg(self) -> crate::common::Reg<regs::MacMdioCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn mac_mdio_ctrl(self) -> crate::common::Reg<regs::MacMdioCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn mac_mdio_rd_data(
        self,
    ) -> crate::common::Reg<regs::MacMdioRdData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn mac_mdio_wr_data(
        self,
    ) -> crate::common::Reg<regs::MacMdioWrData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn mac_irq_ctrl(self) -> crate::common::Reg<regs::MacIrqCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxfifo {
    ptr: *mut u8,
}
unsafe impl Send for Rxfifo {}
unsafe impl Sync for Rxfifo {}
impl Rxfifo {
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
    pub const fn sw_ctrl_igress_rx_fdfifo_e_fdmem_cnt_byte(
        self,
    ) -> crate::common::Reg<regs::SwCtrlIgressRxFdfifoEFdmemCntByte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_igress_rx_fdfifo_e_fdmem_sts(
        self,
    ) -> crate::common::Reg<regs::SwCtrlIgressRxFdfifoEFdmemSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_igress_rx_fdfifo_e_error_flag(
        self,
    ) -> crate::common::Reg<regs::SwCtrlIgressRxFdfifoEErrorFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_igress_rx_fdfifo_e_ie_error_flag(
        self,
    ) -> crate::common::Reg<regs::SwCtrlIgressRxFdfifoEIeErrorFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_igress_rx_fdfifo_e_in_config(
        self,
    ) -> crate::common::Reg<regs::SwCtrlIgressRxFdfifoEInConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_igress_rx_fdfifo_e_out_config(
        self,
    ) -> crate::common::Reg<regs::SwCtrlIgressRxFdfifoEOutConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_igress_rx_fdfifo_e_reset(
        self,
    ) -> crate::common::Reg<regs::SwCtrlIgressRxFdfifoEReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_igress_rx_fdfifo_e_param(
        self,
    ) -> crate::common::Reg<regs::SwCtrlIgressRxFdfifoEParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_igress_rx_fdfifo_e_strfwd(
        self,
    ) -> crate::common::Reg<regs::SwCtrlIgressRxFdfifoEStrfwd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_igress_rx_fdfifo_e_portmask(
        self,
    ) -> crate::common::Reg<regs::SwCtrlIgressRxFdfifoEPortmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_igress_rx_fdfifo_e_mirror(
        self,
    ) -> crate::common::Reg<regs::SwCtrlIgressRxFdfifoEMirror, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_igress_rx_fdfifo_e_mirror_tx(
        self,
    ) -> crate::common::Reg<regs::SwCtrlIgressRxFdfifoEMirrorTx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shacl {
    ptr: *mut u8,
}
unsafe impl Send for Shacl {}
unsafe impl Sync for Shacl {}
impl Shacl {
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
    pub const fn tsn_shaper_aclist_entry0_l(
        self,
    ) -> crate::common::Reg<regs::TsnShaperAclistEntry0L, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_aclist_entry0_h(
        self,
    ) -> crate::common::Reg<regs::TsnShaperAclistEntry0H, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsnport {
    ptr: *mut u8,
}
unsafe impl Send for Tsnport {}
unsafe impl Sync for Tsnport {}
impl Tsnport {
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
    pub const fn mac(self, n: usize) -> Mac {
        assert!(n < 2usize);
        unsafe { Mac::from_ptr(self.ptr.wrapping_add(0x0usize + n * 512usize) as _) }
    }
    #[doc = "ONLY IN PORT1."]
    #[inline(always)]
    pub const fn rtc_cr(self) -> crate::common::Reg<regs::RtcCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "ONLY IN PORT1."]
    #[inline(always)]
    pub const fn rtc_sr(self) -> crate::common::Reg<regs::RtcSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "ONLY IN PORT1."]
    #[inline(always)]
    pub const fn rtc_ct_curtime_ns(
        self,
    ) -> crate::common::Reg<regs::RtcCtCurtimeNs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0810usize) as _) }
    }
    #[doc = "ONLY IN PORT1."]
    #[inline(always)]
    pub const fn rtc_ct_curtime_sec(
        self,
    ) -> crate::common::Reg<regs::RtcCtCurtimeSec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0814usize) as _) }
    }
    #[doc = "ONLY IN PORT1."]
    #[inline(always)]
    pub const fn rtc_ct_timer_incr(
        self,
    ) -> crate::common::Reg<regs::RtcCtTimerIncr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x081cusize) as _) }
    }
    #[doc = "ONLY IN PORT1."]
    #[inline(always)]
    pub const fn rtc_ofs_ns(self) -> crate::common::Reg<regs::RtcOfsNs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0820usize) as _) }
    }
    #[doc = "ONLY IN PORT1."]
    #[inline(always)]
    pub const fn rtc_ofs_sl(self) -> crate::common::Reg<regs::RtcOfsSl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0824usize) as _) }
    }
    #[doc = "ONLY IN PORT1."]
    #[inline(always)]
    pub const fn rtc_ofs_sh(self) -> crate::common::Reg<regs::RtcOfsSh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0828usize) as _) }
    }
    #[doc = "ONLY IN PORT1."]
    #[inline(always)]
    pub const fn rtc_ofs_ch(self) -> crate::common::Reg<regs::RtcOfsCh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x082cusize) as _) }
    }
    #[doc = "ONLY IN PORT1."]
    #[inline(always)]
    pub const fn rtc_alarm_ns(self) -> crate::common::Reg<regs::RtcAlarmNs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0830usize) as _) }
    }
    #[doc = "ONLY IN PORT1."]
    #[inline(always)]
    pub const fn rtc_alarm_sl(self) -> crate::common::Reg<regs::RtcAlarmSl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0834usize) as _) }
    }
    #[doc = "ONLY IN PORT1."]
    #[inline(always)]
    pub const fn rtc_alarm_sh(self) -> crate::common::Reg<regs::RtcAlarmSh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0838usize) as _) }
    }
    #[doc = "ONLY IN PORT1."]
    #[inline(always)]
    pub const fn rtc_timer_a_period(
        self,
    ) -> crate::common::Reg<regs::RtcTimerAPeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0840usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsyn_cr(self) -> crate::common::Reg<regs::TsynCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsyn_sr(self) -> crate::common::Reg<regs::TsynSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsyn_ptp_tx_sts(
        self,
    ) -> crate::common::Reg<regs::TsynPtpTxSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsyn_ptp_tx_done(
        self,
    ) -> crate::common::Reg<regs::TsynPtpTxDone, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsyn_ptp_tx_trig(
        self,
    ) -> crate::common::Reg<regs::TsynPtpTxTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsyn_ptp_rx_sts(
        self,
    ) -> crate::common::Reg<regs::TsynPtpRxSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn tsyntmr(self, n: usize) -> crate::common::Reg<regs::Tsyntmr, crate::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize + n * 4usize) as _)
        }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsyn_hclkdiv(self) -> crate::common::Reg<regs::TsynHclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsyn_rxbuf_rx_frame_length_bytes(
        self,
    ) -> crate::common::Reg<regs::TsynRxbufRxFrameLengthBytes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1600usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsyn_rxbuf_rx_time_stamp_l(
        self,
    ) -> crate::common::Reg<regs::TsynRxbufRxTimeStampL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1608usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsyn_rxbuf_rx_time_stamp_h(
        self,
    ) -> crate::common::Reg<regs::TsynRxbufRxTimeStampH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x160cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn rxdata(self, n: usize) -> crate::common::Reg<regs::Rxdata, crate::common::RW> {
        assert!(n < 60usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1610usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn bin(self, n: usize) -> Bin {
        assert!(n < 8usize);
        unsafe { Bin::from_ptr(self.ptr.wrapping_add(0x1800usize + n * 256usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_hwcfg1(
        self,
    ) -> crate::common::Reg<regs::TsnShaperHwcfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_tqav(
        self,
    ) -> crate::common::Reg<regs::TsnShaperTqav, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_tqem(
        self,
    ) -> crate::common::Reg<regs::TsnShaperTqem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_fpst(
        self,
    ) -> crate::common::Reg<regs::TsnShaperFpst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2014usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_mmct(
        self,
    ) -> crate::common::Reg<regs::TsnShaperMmct, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2018usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_holdadv(
        self,
    ) -> crate::common::Reg<regs::TsnShaperHoldadv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x201cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn mxsdu(self, n: usize) -> crate::common::Reg<regs::Mxsdu, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2100usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn txsel(self, n: usize) -> crate::common::Reg<regs::Txsel, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2120usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn idsel(self, n: usize) -> crate::common::Reg<regs::Idsel, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2140usize + n * 4usize) as _)
        }
    }
    #[doc = "qch channel0 control."]
    #[inline(always)]
    pub const fn port1_qch0_cfg(self) -> crate::common::Reg<regs::Port1Qch0Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2800usize) as _) }
    }
    #[doc = "qch channel1 control."]
    #[inline(always)]
    pub const fn port1_qch1_cfg(self) -> crate::common::Reg<regs::Port1Qch1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2804usize) as _) }
    }
    #[doc = "qch channel2 control."]
    #[inline(always)]
    pub const fn port1_qch2_cfg(self) -> crate::common::Reg<regs::Port1Qch2Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2808usize) as _) }
    }
    #[doc = "qch channel3 control."]
    #[inline(always)]
    pub const fn port1_qch3_cfg(self) -> crate::common::Reg<regs::Port1Qch3Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x280cusize) as _) }
    }
    #[doc = "qch clear."]
    #[inline(always)]
    pub const fn port1_qch_err_cfg(
        self,
    ) -> crate::common::Reg<regs::Port1QchErrCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2810usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_tas_crsr(
        self,
    ) -> crate::common::Reg<regs::TsnShaperTasCrsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3000usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_tas_acycletm(
        self,
    ) -> crate::common::Reg<regs::TsnShaperTasAcycletm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3004usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_tas_abasetm_l(
        self,
    ) -> crate::common::Reg<regs::TsnShaperTasAbasetmL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3008usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_tas_abasetm_h(
        self,
    ) -> crate::common::Reg<regs::TsnShaperTasAbasetmH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x300cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_tas_listlen(
        self,
    ) -> crate::common::Reg<regs::TsnShaperTasListlen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3010usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_tas_ocycletm(
        self,
    ) -> crate::common::Reg<regs::TsnShaperTasOcycletm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3014usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_tas_obasetm_l(
        self,
    ) -> crate::common::Reg<regs::TsnShaperTasObasetmL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3018usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_shaper_tas_obasetm_h(
        self,
    ) -> crate::common::Reg<regs::TsnShaperTasObasetmH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x301cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn mxtk(self, n: usize) -> crate::common::Reg<regs::Mxtk, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3020usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn txov(self, n: usize) -> crate::common::Reg<regs::Txov, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3040usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn shacl(self, n: usize) -> Shacl {
        assert!(n < 256usize);
        unsafe { Shacl::from_ptr(self.ptr.wrapping_add(0x3800usize + n * 8usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_ver(self) -> crate::common::Reg<regs::TsnEpVer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf000usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_ctrl(self) -> crate::common::Reg<regs::TsnEpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf004usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_txuf(self) -> crate::common::Reg<regs::TsnEpTxuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf010usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_ipcfg(self) -> crate::common::Reg<regs::TsnEpIpcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf014usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_tsf_d0(self) -> crate::common::Reg<regs::TsnEpTsfD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf020usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_tsf_d1(self) -> crate::common::Reg<regs::TsnEpTsfD1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf024usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_tsf_d2(self) -> crate::common::Reg<regs::TsnEpTsfD2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf028usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_tsf_sr(self) -> crate::common::Reg<regs::TsnEpTsfSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf02cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_mms_ctrl(
        self,
    ) -> crate::common::Reg<regs::TsnEpMmsCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf030usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_mms_sts(self) -> crate::common::Reg<regs::TsnEpMmsSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf034usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_mms_vtime(
        self,
    ) -> crate::common::Reg<regs::TsnEpMmsVtime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf038usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_mms_stat(
        self,
    ) -> crate::common::Reg<regs::TsnEpMmsStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf03cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_ptp_uptm_ns(
        self,
    ) -> crate::common::Reg<regs::TsnEpPtpUptmNs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf040usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_ptp_uptm_s(
        self,
    ) -> crate::common::Reg<regs::TsnEpPtpUptmS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf044usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn tsn_ep_ptp_sr(self) -> crate::common::Reg<regs::TsnEpPtpSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf048usize) as _) }
    }
    #[doc = "PVID Tagging Register."]
    #[inline(always)]
    pub const fn sw_ctrl_port_main_tagging(
        self,
    ) -> crate::common::Reg<regs::SwCtrlPortMainTagging, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0000usize) as _) }
    }
    #[doc = "Port Module Enable Register."]
    #[inline(always)]
    pub const fn sw_ctrl_port_main_ennable(
        self,
    ) -> crate::common::Reg<regs::SwCtrlPortMainEnnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0004usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_egress_ecsr_qdrop(
        self,
    ) -> crate::common::Reg<regs::SwCtrlEgressEcsrQdrop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2000usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn rxfifo(self, n: usize) -> Rxfifo {
        assert!(n < 2usize);
        unsafe { Rxfifo::from_ptr(self.ptr.wrapping_add(0x0001_4000usize + n * 256usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_monitor_ctrl(
        self,
    ) -> crate::common::Reg<regs::SwCtrlMonitorCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8004usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_monitor_reset(
        self,
    ) -> crate::common::Reg<regs::SwCtrlMonitorReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8008usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn sw_ctrl_monitor_param(
        self,
    ) -> crate::common::Reg<regs::SwCtrlMonitorParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_800cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_tx_counter_tx_fgood(
        self,
    ) -> crate::common::Reg<regs::MonitorTxCounterTxFgood, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8010usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_tx_counter_tx_ferror(
        self,
    ) -> crate::common::Reg<regs::MonitorTxCounterTxFerror, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8018usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_tx_counter_tx_drop_ovfl(
        self,
    ) -> crate::common::Reg<regs::MonitorTxCounterTxDropOvfl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8020usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_fgood(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxFgood, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8040usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_ferror(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxFerror, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8048usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_known(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxKnown, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8050usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_unknown(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxUnknown, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8058usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_uc(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxUc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8060usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_intern(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxIntern, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8068usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_bc(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxBc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8070usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_multi(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxMulti, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8078usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_vlan(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxVlan, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8080usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_drop_ovfl(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxDropOvfl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8088usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_drop_lu(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxDropLu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8090usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_drop_err(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxDropErr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8098usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_drop_vlan(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxDropVlan, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_80a0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn monitor_rx_counter_rx_fpe_fgood(
        self,
    ) -> crate::common::Reg<regs::MonitorRxCounterRxFpeFgood, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_80a8usize) as _) }
    }
    #[doc = "control register0."]
    #[inline(always)]
    pub const fn gpr_ctrl0(self) -> crate::common::Reg<regs::GprCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_c000usize) as _) }
    }
    #[doc = "control register2."]
    #[inline(always)]
    pub const fn gpr_ctrl2(self) -> crate::common::Reg<regs::GprCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_c008usize) as _) }
    }
}
#[doc = "TSW."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsw {
    ptr: *mut u8,
}
unsafe impl Send for Tsw {}
unsafe impl Sync for Tsw {}
impl Tsw {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "LU_MAIN control."]
    #[inline(always)]
    pub const fn lu_main_ctrl(self) -> crate::common::Reg<regs::LuMainCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "LU_MAIN hit."]
    #[inline(always)]
    pub const fn lu_main_hitmem(self) -> crate::common::Reg<regs::LuMainHitmem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "LU_MAIN parameter."]
    #[inline(always)]
    pub const fn lu_main_param(self) -> crate::common::Reg<regs::LuMainParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "LU_MAIN bypass."]
    #[inline(always)]
    pub const fn lu_main_bypass(self) -> crate::common::Reg<regs::LuMainBypass, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "LU_MAIN PCP remap."]
    #[inline(always)]
    pub const fn lu_main_pcp_remap(
        self,
    ) -> crate::common::Reg<regs::LuMainPcpRemap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "LU_MAIN version."]
    #[inline(always)]
    pub const fn lu_main_version(
        self,
    ) -> crate::common::Reg<regs::LuMainVersion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "LU_MAIN low word of action data for internal frames."]
    #[inline(always)]
    pub const fn lu_main_intf_action(
        self,
    ) -> crate::common::Reg<regs::LuMainIntfAction, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "LU_MAIN low word of action data for broadcast frames."]
    #[inline(always)]
    pub const fn lu_main_bc_action(
        self,
    ) -> crate::common::Reg<regs::LuMainBcAction, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "LU_MAIN low word of action data for unknown frames."]
    #[inline(always)]
    pub const fn lu_main_nn_action(
        self,
    ) -> crate::common::Reg<regs::LuMainNnAction, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn apb2axis_cam_sts(
        self,
    ) -> crate::common::Reg<regs::Apb2axisCamSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "request count."]
    #[inline(always)]
    pub const fn apb2axis_cam_req_cnt(
        self,
    ) -> crate::common::Reg<regs::Apb2axisCamReqCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "fill status."]
    #[inline(always)]
    pub const fn apb2axis_cam_fillsts(
        self,
    ) -> crate::common::Reg<regs::Apb2axisCamFillsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "reset."]
    #[inline(always)]
    pub const fn apb2axis_cam_reset(
        self,
    ) -> crate::common::Reg<regs::Apb2axisCamReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "parameter."]
    #[inline(always)]
    pub const fn apb2axis_cam_param(
        self,
    ) -> crate::common::Reg<regs::Apb2axisCamParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "data0."]
    #[inline(always)]
    pub const fn apb2axi_cam_reqdata_0(
        self,
    ) -> crate::common::Reg<regs::Apb2axiCamReqdata0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "data1."]
    #[inline(always)]
    pub const fn apb2axi_cam_reqdata_1(
        self,
    ) -> crate::common::Reg<regs::Apb2axiCamReqdata1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "data2."]
    #[inline(always)]
    pub const fn apb2axi_cam_reqdata_2(
        self,
    ) -> crate::common::Reg<regs::Apb2axiCamReqdata2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn apb2axis_almem_sts(
        self,
    ) -> crate::common::Reg<regs::Apb2axisAlmemSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "request count."]
    #[inline(always)]
    pub const fn apb2axis_almem_req_cnt(
        self,
    ) -> crate::common::Reg<regs::Apb2axisAlmemReqCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "fill status."]
    #[inline(always)]
    pub const fn apb2axis_almem_fillsts(
        self,
    ) -> crate::common::Reg<regs::Apb2axisAlmemFillsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "reset."]
    #[inline(always)]
    pub const fn apb2axis_almem_reset(
        self,
    ) -> crate::common::Reg<regs::Apb2axisAlmemReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "parameter."]
    #[inline(always)]
    pub const fn apb2axis_almem_param(
        self,
    ) -> crate::common::Reg<regs::Apb2axisAlmemParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "data0."]
    #[inline(always)]
    pub const fn apb2axis_almem_reqdata_0(
        self,
    ) -> crate::common::Reg<regs::Apb2axisAlmemReqdata0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "data1."]
    #[inline(always)]
    pub const fn apb2axis_almem_reqdata_1(
        self,
    ) -> crate::common::Reg<regs::Apb2axisAlmemReqdata1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn axis2apb_almem_sts(
        self,
    ) -> crate::common::Reg<regs::Axis2apbAlmemSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "response count."]
    #[inline(always)]
    pub const fn axis2apb_almem_resp_cnt(
        self,
    ) -> crate::common::Reg<regs::Axis2apbAlmemRespCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "fill status."]
    #[inline(always)]
    pub const fn axis2apb_almem_fillsts(
        self,
    ) -> crate::common::Reg<regs::Axis2apbAlmemFillsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "reset."]
    #[inline(always)]
    pub const fn axis2apb_almem_reset(
        self,
    ) -> crate::common::Reg<regs::Axis2apbAlmemReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "parameter."]
    #[inline(always)]
    pub const fn axis2apb_almem_param(
        self,
    ) -> crate::common::Reg<regs::Axis2apbAlmemParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "data0."]
    #[inline(always)]
    pub const fn axis2apb_almem_respdata_0(
        self,
    ) -> crate::common::Reg<regs::Axis2apbAlmemRespdata0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "data1."]
    #[inline(always)]
    pub const fn axis2apb_almem_respdata_1(
        self,
    ) -> crate::common::Reg<regs::Axis2apbAlmemRespdata1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn hitmem(self, n: usize) -> crate::common::Reg<regs::Hitmem, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn apb2axis_lookup_sts(
        self,
    ) -> crate::common::Reg<regs::Apb2axisLookupSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "response count."]
    #[inline(always)]
    pub const fn apb2axis_lookup_req_cnt(
        self,
    ) -> crate::common::Reg<regs::Apb2axisLookupReqCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "fill status."]
    #[inline(always)]
    pub const fn apb2axis_lookup_fillsts(
        self,
    ) -> crate::common::Reg<regs::Apb2axisLookupFillsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "reset."]
    #[inline(always)]
    pub const fn apb2axis_lookup_reset(
        self,
    ) -> crate::common::Reg<regs::Apb2axisLookupReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "parameter."]
    #[inline(always)]
    pub const fn apb2axis_lookup_param(
        self,
    ) -> crate::common::Reg<regs::Apb2axisLookupParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "LOOKUP REQUEST Register REQ_DATA_0."]
    #[inline(always)]
    pub const fn apb2axis_lookup_reqdata_0(
        self,
    ) -> crate::common::Reg<regs::Apb2axisLookupReqdata0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "LOOKUP REQUEST Register REQ_DATA_1."]
    #[inline(always)]
    pub const fn apb2axis_lookup_reqdata_1(
        self,
    ) -> crate::common::Reg<regs::Apb2axisLookupReqdata1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "LOOKUP REQUEST Register REQ_DATA_2."]
    #[inline(always)]
    pub const fn apb2axis_lookup_reqdata_3(
        self,
    ) -> crate::common::Reg<regs::Apb2axisLookupReqdata3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn axis2apb_lookup_sts(
        self,
    ) -> crate::common::Reg<regs::Axis2apbLookupSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1080usize) as _) }
    }
    #[doc = "response count."]
    #[inline(always)]
    pub const fn axis2apb_lookup_resp_cnt(
        self,
    ) -> crate::common::Reg<regs::Axis2apbLookupRespCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1090usize) as _) }
    }
    #[doc = "fill status."]
    #[inline(always)]
    pub const fn axis2apb_lookup_fillsts(
        self,
    ) -> crate::common::Reg<regs::Axis2apbLookupFillsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1094usize) as _) }
    }
    #[doc = "reset."]
    #[inline(always)]
    pub const fn axis2apb_lookup_reset(
        self,
    ) -> crate::common::Reg<regs::Axis2apbLookupReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1098usize) as _) }
    }
    #[doc = "parameter."]
    #[inline(always)]
    pub const fn axis2apb_lookup_param(
        self,
    ) -> crate::common::Reg<regs::Axis2apbLookupParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x109cusize) as _) }
    }
    #[doc = "LOOKUP RESPONSE Data Register."]
    #[inline(always)]
    pub const fn axis2apb_lookup_respdata_0(
        self,
    ) -> crate::common::Reg<regs::Axis2apbLookupRespdata0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a0usize) as _) }
    }
    #[doc = "LOOKUP RESPONSE Data Register."]
    #[inline(always)]
    pub const fn axis2apb_lookup_respdata_1(
        self,
    ) -> crate::common::Reg<regs::Axis2apbLookupRespdata1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a8usize) as _) }
    }
    #[doc = "version register."]
    #[inline(always)]
    pub const fn central_csr_version(
        self,
    ) -> crate::common::Reg<regs::CentralCsrVersion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize) as _) }
    }
    #[doc = "Parameter Register."]
    #[inline(always)]
    pub const fn central_csr_param(
        self,
    ) -> crate::common::Reg<regs::CentralCsrParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2004usize) as _) }
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn central_csr_config(
        self,
    ) -> crate::common::Reg<regs::CentralCsrConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2008usize) as _) }
    }
    #[doc = "CB Parameter Register."]
    #[inline(always)]
    pub const fn central_csr_cb_param(
        self,
    ) -> crate::common::Reg<regs::CentralCsrCbParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x200cusize) as _) }
    }
    #[doc = "QCI Control Parameter Register."]
    #[inline(always)]
    pub const fn central_csr_qci_ctrl_param(
        self,
    ) -> crate::common::Reg<regs::CentralCsrQciCtrlParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2010usize) as _) }
    }
    #[doc = "PSPF General CTRAL."]
    #[inline(always)]
    pub const fn central_qci_hwcfg(
        self,
    ) -> crate::common::Reg<regs::CentralQciHwcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2104usize) as _) }
    }
    #[doc = "Filter select index."]
    #[inline(always)]
    pub const fn central_qci_filtersel(
        self,
    ) -> crate::common::Reg<regs::CentralQciFiltersel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2110usize) as _) }
    }
    #[doc = "Flowmeter select index."]
    #[inline(always)]
    pub const fn central_qci_metersel(
        self,
    ) -> crate::common::Reg<regs::CentralQciMetersel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2114usize) as _) }
    }
    #[doc = "Gate select index."]
    #[inline(always)]
    pub const fn central_qci_gatesel(
        self,
    ) -> crate::common::Reg<regs::CentralQciGatesel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2118usize) as _) }
    }
    #[doc = "FILTER SETTING."]
    #[inline(always)]
    pub const fn central_qci_fctrl(
        self,
    ) -> crate::common::Reg<regs::CentralQciFctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2120usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_fsize(
        self,
    ) -> crate::common::Reg<regs::CentralQciFsize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2124usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn qci_cnt(self, n: usize) -> crate::common::Reg<regs::QciCnt, crate::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2140usize + n * 4usize) as _)
        }
    }
    #[doc = "Flow meter settings."]
    #[inline(always)]
    pub const fn central_qci_mctrl(
        self,
    ) -> crate::common::Reg<regs::CentralQciMctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2160usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_cir(
        self,
    ) -> crate::common::Reg<regs::CentralQciCir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2170usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_cbs(
        self,
    ) -> crate::common::Reg<regs::CentralQciCbs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2174usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_eir(
        self,
    ) -> crate::common::Reg<regs::CentralQciEir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2178usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_ebs(
        self,
    ) -> crate::common::Reg<regs::CentralQciEbs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x217cusize) as _) }
    }
    #[doc = "Gate settings."]
    #[inline(always)]
    pub const fn central_qci_gctrl(
        self,
    ) -> crate::common::Reg<regs::CentralQciGctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2180usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_gstatus(
        self,
    ) -> crate::common::Reg<regs::CentralQciGstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2184usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_glistindex(
        self,
    ) -> crate::common::Reg<regs::CentralQciGlistindex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2188usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_listlen(
        self,
    ) -> crate::common::Reg<regs::CentralQciListlen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x218cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_acycletm(
        self,
    ) -> crate::common::Reg<regs::CentralQciAcycletm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2190usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_abasetm_l(
        self,
    ) -> crate::common::Reg<regs::CentralQciAbasetmL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2194usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_abasetm_h(
        self,
    ) -> crate::common::Reg<regs::CentralQciAbasetmH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2198usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_aentry_ctrl(
        self,
    ) -> crate::common::Reg<regs::CentralQciAentryCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21a0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_aentry_aentry_ival(
        self,
    ) -> crate::common::Reg<regs::CentralQciAentryAentryIval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21a4usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_aentry_ocycletm(
        self,
    ) -> crate::common::Reg<regs::CentralQciAentryOcycletm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21a8usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_aentry_obasetm_l(
        self,
    ) -> crate::common::Reg<regs::CentralQciAentryObasetmL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21acusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn central_qci_aentry_obasetm_h(
        self,
    ) -> crate::common::Reg<regs::CentralQciAentryObasetmH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x21b0usize) as _) }
    }
    #[doc = "mm2s control register."]
    #[inline(always)]
    pub const fn mm2s_dma_cr(self) -> crate::common::Reg<regs::Mm2sDmaCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4000usize) as _) }
    }
    #[doc = "mm2s status."]
    #[inline(always)]
    pub const fn mm2s_dma_sr(self) -> crate::common::Reg<regs::Mm2sDmaSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4004usize) as _) }
    }
    #[doc = "mm2s dma fill status."]
    #[inline(always)]
    pub const fn mm2s_dma_fill(self) -> crate::common::Reg<regs::Mm2sDmaFill, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4008usize) as _) }
    }
    #[doc = "mm2s dma configure."]
    #[inline(always)]
    pub const fn mm2s_dma_cfg(self) -> crate::common::Reg<regs::Mm2sDmaCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x401cusize) as _) }
    }
    #[doc = "mm2s axi address."]
    #[inline(always)]
    pub const fn mm2s_addrlo(self) -> crate::common::Reg<regs::Mm2sAddrlo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4020usize) as _) }
    }
    #[doc = "mm2s axi length."]
    #[inline(always)]
    pub const fn mm2s_length(self) -> crate::common::Reg<regs::Mm2sLength, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4028usize) as _) }
    }
    #[doc = "mm2s command control."]
    #[inline(always)]
    pub const fn mm2s_ctrl(self) -> crate::common::Reg<regs::Mm2sCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x402cusize) as _) }
    }
    #[doc = "mm2s response buffer."]
    #[inline(always)]
    pub const fn mm2s_resp(self) -> crate::common::Reg<regs::Mm2sResp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4030usize) as _) }
    }
    #[doc = "s2mm dma control."]
    #[inline(always)]
    pub const fn s2mm_dma_cr(self) -> crate::common::Reg<regs::S2mmDmaCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4080usize) as _) }
    }
    #[doc = "s2mm state."]
    #[inline(always)]
    pub const fn s2mm_dma_sr(self) -> crate::common::Reg<regs::S2mmDmaSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4084usize) as _) }
    }
    #[doc = "s2mm buffer fill status."]
    #[inline(always)]
    pub const fn s2mm_dma_fill(self) -> crate::common::Reg<regs::S2mmDmaFill, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4088usize) as _) }
    }
    #[doc = "s2mm dma config status."]
    #[inline(always)]
    pub const fn s2mm_dma_cfg(self) -> crate::common::Reg<regs::S2mmDmaCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x409cusize) as _) }
    }
    #[doc = "s2mm axi address."]
    #[inline(always)]
    pub const fn s2mm_addrlo(self) -> crate::common::Reg<regs::S2mmAddrlo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40a0usize) as _) }
    }
    #[doc = "s2mm axi length."]
    #[inline(always)]
    pub const fn s2mm_length(self) -> crate::common::Reg<regs::S2mmLength, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40a8usize) as _) }
    }
    #[doc = "s2mm command control."]
    #[inline(always)]
    pub const fn s2mm_ctrl(self) -> crate::common::Reg<regs::S2mmCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40acusize) as _) }
    }
    #[doc = "s2mm response buffer."]
    #[inline(always)]
    pub const fn s2mm_resp(self) -> crate::common::Reg<regs::S2mmResp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40b0usize) as _) }
    }
    #[doc = "timestamp control."]
    #[inline(always)]
    pub const fn ptp_evt_ts_ctl(self) -> crate::common::Reg<regs::PtpEvtTsCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6000usize) as _) }
    }
    #[doc = "pps tod seconds."]
    #[inline(always)]
    pub const fn ptp_evt_pps_tod_sec(
        self,
    ) -> crate::common::Reg<regs::PtpEvtPpsTodSec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6008usize) as _) }
    }
    #[doc = "pps tod sun seconds."]
    #[inline(always)]
    pub const fn ptp_evt_pps_tod_ns(
        self,
    ) -> crate::common::Reg<regs::PtpEvtPpsTodNs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x600cusize) as _) }
    }
    #[doc = "target time seconds."]
    #[inline(always)]
    pub const fn ptp_evt_scp_sec0(
        self,
    ) -> crate::common::Reg<regs::PtpEvtScpSec0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x601cusize) as _) }
    }
    #[doc = "target time sub seconds."]
    #[inline(always)]
    pub const fn ptp_evt_scp_ns0(
        self,
    ) -> crate::common::Reg<regs::PtpEvtScpNs0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6020usize) as _) }
    }
    #[doc = "timer status."]
    #[inline(always)]
    pub const fn ptp_evt_tmr_sts(
        self,
    ) -> crate::common::Reg<regs::PtpEvtTmrSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6028usize) as _) }
    }
    #[doc = "pps command control."]
    #[inline(always)]
    pub const fn ptp_evt_pps_cmd(
        self,
    ) -> crate::common::Reg<regs::PtpEvtPpsCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x602cusize) as _) }
    }
    #[doc = "auxiliray read data sub seconds."]
    #[inline(always)]
    pub const fn ptp_evt_atslo(self) -> crate::common::Reg<regs::PtpEvtAtslo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6030usize) as _) }
    }
    #[doc = "auxiliray read data seconds."]
    #[inline(always)]
    pub const fn ptp_evt_atshi(self) -> crate::common::Reg<regs::PtpEvtAtshi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6034usize) as _) }
    }
    #[doc = "pps0 interval configure."]
    #[inline(always)]
    pub const fn ptp_evt_pps0_interval(
        self,
    ) -> crate::common::Reg<regs::PtpEvtPps0Interval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6060usize) as _) }
    }
    #[doc = "pps0 width configure."]
    #[inline(always)]
    pub const fn ptp_evt_pps0_width(
        self,
    ) -> crate::common::Reg<regs::PtpEvtPps0Width, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6064usize) as _) }
    }
    #[doc = "target time seconds."]
    #[inline(always)]
    pub const fn ptp_evt_scp_sec1(
        self,
    ) -> crate::common::Reg<regs::PtpEvtScpSec1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6080usize) as _) }
    }
    #[doc = "target time sub seconds."]
    #[inline(always)]
    pub const fn ptp_evt_scp_ns1(
        self,
    ) -> crate::common::Reg<regs::PtpEvtScpNs1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6084usize) as _) }
    }
    #[doc = "pps1 interval configure."]
    #[inline(always)]
    pub const fn ptp_evt_pps1_interval(
        self,
    ) -> crate::common::Reg<regs::PtpEvtPps1Interval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6088usize) as _) }
    }
    #[doc = "pps1 width configure."]
    #[inline(always)]
    pub const fn ptp_evt_pps1_width(
        self,
    ) -> crate::common::Reg<regs::PtpEvtPps1Width, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x608cusize) as _) }
    }
    #[doc = "target time seconds."]
    #[inline(always)]
    pub const fn ptp_evt_scp_sec2(
        self,
    ) -> crate::common::Reg<regs::PtpEvtScpSec2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60a0usize) as _) }
    }
    #[doc = "target time sub seconds."]
    #[inline(always)]
    pub const fn ptp_evt_scp_ns2(
        self,
    ) -> crate::common::Reg<regs::PtpEvtScpNs2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60a4usize) as _) }
    }
    #[doc = "pps2 interval configure."]
    #[inline(always)]
    pub const fn ptp_evt_pps2_interval(
        self,
    ) -> crate::common::Reg<regs::PtpEvtPps2Interval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60a8usize) as _) }
    }
    #[doc = "pps2 width configure."]
    #[inline(always)]
    pub const fn ptp_evt_pps2_width(
        self,
    ) -> crate::common::Reg<regs::PtpEvtPps2Width, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60acusize) as _) }
    }
    #[doc = "target time seconds."]
    #[inline(always)]
    pub const fn ptp_evt_scp_sec3(
        self,
    ) -> crate::common::Reg<regs::PtpEvtScpSec3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60c0usize) as _) }
    }
    #[doc = "target time sub seconds."]
    #[inline(always)]
    pub const fn ptp_evt_scp_ns3(
        self,
    ) -> crate::common::Reg<regs::PtpEvtScpNs3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60c4usize) as _) }
    }
    #[doc = "pps3 interval configure."]
    #[inline(always)]
    pub const fn ptp_evt_pps3_interval(
        self,
    ) -> crate::common::Reg<regs::PtpEvtPps3Interval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60c8usize) as _) }
    }
    #[doc = "pps3 width configure."]
    #[inline(always)]
    pub const fn ptp_evt_pps3_width(
        self,
    ) -> crate::common::Reg<regs::PtpEvtPps3Width, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60ccusize) as _) }
    }
    #[doc = "pps control 0 register."]
    #[inline(always)]
    pub const fn ptp_evt_pps_ctrl0(
        self,
    ) -> crate::common::Reg<regs::PtpEvtPpsCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60e0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn ptp_evt_pps_sel(
        self,
    ) -> crate::common::Reg<regs::PtpEvtPpsSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60e4usize) as _) }
    }
    #[doc = "softer reset control."]
    #[inline(always)]
    pub const fn soft_rst_ctrl(self) -> crate::common::Reg<regs::SoftRstCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60f0usize) as _) }
    }
    #[doc = "PVID Tagging Register."]
    #[inline(always)]
    pub const fn cpu_port_port_main_tagging(
        self,
    ) -> crate::common::Reg<regs::CpuPortPortMainTagging, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0000usize) as _) }
    }
    #[doc = "Port Module Enable Register."]
    #[inline(always)]
    pub const fn cpu_port_port_main_ennable(
        self,
    ) -> crate::common::Reg<regs::CpuPortPortMainEnnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_0004usize) as _) }
    }
    #[doc = "Stream Identification."]
    #[inline(always)]
    pub const fn cpu_port_egress_stmid_eselect(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressStmidEselect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2800usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_stmid_control(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressStmidControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2840usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_stmid_seqno(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressStmidSeqno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2844usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_stmid_matchcnt(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressStmidMatchcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2848usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_stmid_maclo(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressStmidMaclo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2850usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_stmid_machi(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressStmidMachi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2854usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_stmid_amachi(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressStmidAmachi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_285cusize) as _) }
    }
    #[doc = "Frame Replication and Elimination."]
    #[inline(always)]
    pub const fn cpu_port_egress_frer_control(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressFrerControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2a00usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_frer_sidsel(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressFrerSidsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2a04usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_frer_irfunc(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressFrerIrfunc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2a08usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_frer_srfunc(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressFrerSrfunc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2a0cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_frer_fselect(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressFrerFselect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2a10usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_frer_fctrl(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressFrerFctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2a40usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_frer_resetmsec(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressFrerResetmsec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2a44usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_frer_lat_rs_period(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressFrerLatRsPeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2a48usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_frer_lat_test_period(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressFrerLatTestPeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2a4cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_frer_lat_err_diff_alw(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressFrerLatErrDiffAlw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2a50usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_egress_frer_lat_err_cnt(
        self,
    ) -> crate::common::Reg<regs::CpuPortEgressFrerLatErrCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2a54usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn egfrcnt(self, n: usize) -> crate::common::Reg<regs::Egfrcnt, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_2a60usize + n * 4usize) as _)
        }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_rx_fdfifo_fdmem_cnt_byte(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressRxFdfifoFdmemCntByte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4000usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_rx_fdfifo_fdmem_sts(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressRxFdfifoFdmemSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4004usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_rx_fdfifo_error_flag(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressRxFdfifoErrorFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4008usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_rx_fdfifo_ie_error_flag(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressRxFdfifoIeErrorFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_400cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_rx_fdfifo_in_config(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressRxFdfifoInConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4010usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_rx_fdfifo_out_config(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressRxFdfifoOutConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4014usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_rx_fdfifo_reset(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressRxFdfifoReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4018usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_rx_fdfifo_param(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressRxFdfifoParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_401cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_rx_fdfifo_strfwd(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressRxFdfifoStrfwd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4020usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_rx_fdfifo_portmask(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressRxFdfifoPortmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4024usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_rx_fdfifo_mirror(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressRxFdfifoMirror, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4028usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_rx_fdfifo_mirror_tx(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressRxFdfifoMirrorTx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_402cusize) as _) }
    }
    #[doc = "Stream Identification."]
    #[inline(always)]
    pub const fn cpu_port_igress_stmid_eselect(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressStmidEselect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4800usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_stmid_control(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressStmidControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4840usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_stmid_seqno(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressStmidSeqno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4844usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_stmid_matchcnt(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressStmidMatchcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4848usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_stmid_maclo(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressStmidMaclo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4850usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_stmid_machi(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressStmidMachi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4854usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_stmid_amachi(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressStmidAmachi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_485cusize) as _) }
    }
    #[doc = "Frame Replication and Elimination."]
    #[inline(always)]
    pub const fn cpu_port_igress_frer_control(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressFrerControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4a00usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_frer_sidsel(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressFrerSidsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4a04usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_frer_irfunc(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressFrerIrfunc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4a08usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_frer_srfunc(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressFrerSrfunc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4a0cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_frer_fselect(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressFrerFselect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4a10usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_frer_fctrl(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressFrerFctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4a40usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_frer_resetmsec(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressFrerResetmsec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4a44usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_frer_lat_rs_period(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressFrerLatRsPeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4a48usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_frer_lat_test_period(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressFrerLatTestPeriod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4a4cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_frer_lat_err_diff_alw(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressFrerLatErrDiffAlw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4a50usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_igress_frer_lat_err_cnt(
        self,
    ) -> crate::common::Reg<regs::CpuPortIgressFrerLatErrCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4a54usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn igfrcnt(self, n: usize) -> crate::common::Reg<regs::Igfrcnt, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_4a60usize + n * 4usize) as _)
        }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_ctrl(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8004usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_reset(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8008usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_param(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorParam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_800cusize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_tx_counter_tx_fgood(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorTxCounterTxFgood, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8010usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_tx_counter_tx_ferror(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorTxCounterTxFerror, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8018usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_tx_counter_tx_drop_ovfl(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorTxCounterTxDropOvfl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8020usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_fgood(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxFgood, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8040usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_ferror(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxFerror, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8048usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_known(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxKnown, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8050usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_unknown(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxUnknown, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8058usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_uc(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxUc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8060usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_intern(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxIntern, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8068usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_bc(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxBc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8070usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_multi(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxMulti, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8078usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_vlan(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxVlan, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8080usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_drop_ovfl(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxDropOvfl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8088usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_drop_lu(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxDropLu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8090usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_drop_err(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxDropErr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_8098usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_drop_vlan(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxDropVlan, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_80a0usize) as _) }
    }
    #[doc = "No description available."]
    #[inline(always)]
    pub const fn cpu_port_monitor_rx_counter_rx_fpe_fgood(
        self,
    ) -> crate::common::Reg<regs::CpuPortMonitorRxCounterRxFpeFgood, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0001_80a8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn tsnport(self, n: usize) -> Tsnport {
        assert!(n < 3usize);
        unsafe { Tsnport::from_ptr(self.ptr.wrapping_add(0x0002_0000usize + n * 131072usize) as _) }
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
    #[doc = "data0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axiCamReqdata0(pub u32);
    impl Apb2axiCamReqdata0 {
        #[doc = "CAM APB2AXIS channel selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ch(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CAM APB2AXIS channel selection."]
        #[inline(always)]
        pub const fn set_ch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "select between set, clear or clear all."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "select between set, clear or clear all."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "entry number."]
        #[must_use]
        #[inline(always)]
        pub const fn entry_num(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "entry number."]
        #[inline(always)]
        pub const fn set_entry_num(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Apb2axiCamReqdata0 {
        #[inline(always)]
        fn default() -> Apb2axiCamReqdata0 {
            Apb2axiCamReqdata0(0)
        }
    }
    impl core::fmt::Debug for Apb2axiCamReqdata0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axiCamReqdata0")
                .field("ch", &self.ch())
                .field("type_", &self.type_())
                .field("entry_num", &self.entry_num())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axiCamReqdata0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axiCamReqdata0 {{ ch: {=bool:?}, type_: {=u8:?}, entry_num: {=u16:?} }}",
                self.ch(),
                self.type_(),
                self.entry_num()
            )
        }
    }
    #[doc = "data1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axiCamReqdata1(pub u32);
    impl Apb2axiCamReqdata1 {
        #[doc = "dest-mac\\[31:0\\]
when CH=0；PORT_VEC when CH=1."]
        #[must_use]
        #[inline(always)]
        pub const fn destmac_lo_port_vec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "dest-mac\\[31:0\\]
when CH=0；PORT_VEC when CH=1."]
        #[inline(always)]
        pub const fn set_destmac_lo_port_vec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Apb2axiCamReqdata1 {
        #[inline(always)]
        fn default() -> Apb2axiCamReqdata1 {
            Apb2axiCamReqdata1(0)
        }
    }
    impl core::fmt::Debug for Apb2axiCamReqdata1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axiCamReqdata1")
                .field("destmac_lo_port_vec", &self.destmac_lo_port_vec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axiCamReqdata1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axiCamReqdata1 {{ destmac_lo_port_vec: {=u32:?} }}",
                self.destmac_lo_port_vec()
            )
        }
    }
    #[doc = "data2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axiCamReqdata2(pub u32);
    impl Apb2axiCamReqdata2 {
        #[doc = "dest-mac\\[47:32\\]
when CH=0."]
        #[must_use]
        #[inline(always)]
        pub const fn destmac_hi(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "dest-mac\\[47:32\\]
when CH=0."]
        #[inline(always)]
        pub const fn set_destmac_hi(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "VLAN-ID value (12 bit) for the VLAN_ID table. Use the fefault VLAN-ID(VID=1), if setup an entry for non-VLAN traffic."]
        #[must_use]
        #[inline(always)]
        pub const fn vid(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "VLAN-ID value (12 bit) for the VLAN_ID table. Use the fefault VLAN-ID(VID=1), if setup an entry for non-VLAN traffic."]
        #[inline(always)]
        pub const fn set_vid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Apb2axiCamReqdata2 {
        #[inline(always)]
        fn default() -> Apb2axiCamReqdata2 {
            Apb2axiCamReqdata2(0)
        }
    }
    impl core::fmt::Debug for Apb2axiCamReqdata2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axiCamReqdata2")
                .field("destmac_hi", &self.destmac_hi())
                .field("vid", &self.vid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axiCamReqdata2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axiCamReqdata2 {{ destmac_hi: {=u16:?}, vid: {=u16:?} }}",
                self.destmac_hi(),
                self.vid()
            )
        }
    }
    #[doc = "fill status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisAlmemFillsts(pub u32);
    impl Apb2axisAlmemFillsts {
        #[doc = "FD FIFO failure, internal controller lost synchronization."]
        #[must_use]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO failure, internal controller lost synchronization."]
        #[inline(always)]
        pub const fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "frame was dropped because the internal descriptor FIFO is full."]
        #[must_use]
        #[inline(always)]
        pub const fn full(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "frame was dropped because the internal descriptor FIFO is full."]
        #[inline(always)]
        pub const fn set_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Apb2axisAlmemFillsts {
        #[inline(always)]
        fn default() -> Apb2axisAlmemFillsts {
            Apb2axisAlmemFillsts(0)
        }
    }
    impl core::fmt::Debug for Apb2axisAlmemFillsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisAlmemFillsts")
                .field("empty", &self.empty())
                .field("full", &self.full())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisAlmemFillsts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axisAlmemFillsts {{ empty: {=bool:?}, full: {=bool:?} }}",
                self.empty(),
                self.full()
            )
        }
    }
    #[doc = "parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisAlmemParam(pub u32);
    impl Apb2axisAlmemParam {
        #[doc = "number of configured 32bit words for this controller."]
        #[must_use]
        #[inline(always)]
        pub const fn wordlen_byte(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "number of configured 32bit words for this controller."]
        #[inline(always)]
        pub const fn set_wordlen_byte(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "number of configured buffer depth."]
        #[must_use]
        #[inline(always)]
        pub const fn depth(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "number of configured buffer depth."]
        #[inline(always)]
        pub const fn set_depth(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Apb2axisAlmemParam {
        #[inline(always)]
        fn default() -> Apb2axisAlmemParam {
            Apb2axisAlmemParam(0)
        }
    }
    impl core::fmt::Debug for Apb2axisAlmemParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisAlmemParam")
                .field("wordlen_byte", &self.wordlen_byte())
                .field("depth", &self.depth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisAlmemParam {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axisAlmemParam {{ wordlen_byte: {=u8:?}, depth: {=u8:?} }}",
                self.wordlen_byte(),
                self.depth()
            )
        }
    }
    #[doc = "request count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisAlmemReqCnt(pub u32);
    impl Apb2axisAlmemReqCnt {
        #[doc = "number of streams in queue."]
        #[must_use]
        #[inline(always)]
        pub const fn wrcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "number of streams in queue."]
        #[inline(always)]
        pub const fn set_wrcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Apb2axisAlmemReqCnt {
        #[inline(always)]
        fn default() -> Apb2axisAlmemReqCnt {
            Apb2axisAlmemReqCnt(0)
        }
    }
    impl core::fmt::Debug for Apb2axisAlmemReqCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisAlmemReqCnt")
                .field("wrcnt", &self.wrcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisAlmemReqCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apb2axisAlmemReqCnt {{ wrcnt: {=u8:?} }}", self.wrcnt())
        }
    }
    #[doc = "data0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisAlmemReqdata0(pub u32);
    impl Apb2axisAlmemReqdata0 {
        #[doc = "destination ports."]
        #[must_use]
        #[inline(always)]
        pub const fn dest(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "destination ports."]
        #[inline(always)]
        pub const fn set_dest(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "select the priority queue if qsel=11."]
        #[must_use]
        #[inline(always)]
        pub const fn queue(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "select the priority queue if qsel=11."]
        #[inline(always)]
        pub const fn set_queue(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "frame should dropped."]
        #[must_use]
        #[inline(always)]
        pub const fn drop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "frame should dropped."]
        #[inline(always)]
        pub const fn set_drop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "define the traffic queue selection."]
        #[must_use]
        #[inline(always)]
        pub const fn qsel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "define the traffic queue selection."]
        #[inline(always)]
        pub const fn set_qsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "user sideband information."]
        #[must_use]
        #[inline(always)]
        pub const fn utag(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x07;
            val as u8
        }
        #[doc = "user sideband information."]
        #[inline(always)]
        pub const fn set_utag(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
        }
    }
    impl Default for Apb2axisAlmemReqdata0 {
        #[inline(always)]
        fn default() -> Apb2axisAlmemReqdata0 {
            Apb2axisAlmemReqdata0(0)
        }
    }
    impl core::fmt::Debug for Apb2axisAlmemReqdata0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisAlmemReqdata0")
                .field("dest", &self.dest())
                .field("queue", &self.queue())
                .field("drop", &self.drop())
                .field("qsel", &self.qsel())
                .field("utag", &self.utag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisAlmemReqdata0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2axisAlmemReqdata0 {{ dest: {=u16:?}, queue: {=u8:?}, drop: {=bool:?}, qsel: {=u8:?}, utag: {=u8:?} }}" , self . dest () , self . queue () , self . drop () , self . qsel () , self . utag ())
        }
    }
    #[doc = "data1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisAlmemReqdata1(pub u32);
    impl Apb2axisAlmemReqdata1 {
        #[doc = "define the entry number for reading and writing."]
        #[must_use]
        #[inline(always)]
        pub const fn entry_num(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "define the entry number for reading and writing."]
        #[inline(always)]
        pub const fn set_entry_num(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "write response enable."]
        #[must_use]
        #[inline(always)]
        pub const fn resp(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "write response enable."]
        #[inline(always)]
        pub const fn set_resp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1 for write and 0 for read."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_nrd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1 for write and 0 for read."]
        #[inline(always)]
        pub const fn set_wr_nrd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb2axisAlmemReqdata1 {
        #[inline(always)]
        fn default() -> Apb2axisAlmemReqdata1 {
            Apb2axisAlmemReqdata1(0)
        }
    }
    impl core::fmt::Debug for Apb2axisAlmemReqdata1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisAlmemReqdata1")
                .field("entry_num", &self.entry_num())
                .field("resp", &self.resp())
                .field("wr_nrd", &self.wr_nrd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisAlmemReqdata1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2axisAlmemReqdata1 {{ entry_num: {=u16:?}, resp: {=bool:?}, wr_nrd: {=bool:?} }}" , self . entry_num () , self . resp () , self . wr_nrd ())
        }
    }
    #[doc = "reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisAlmemReset(pub u32);
    impl Apb2axisAlmemReset {
        #[doc = "resets controller and clears all pending stream data."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "resets controller and clears all pending stream data."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Apb2axisAlmemReset {
        #[inline(always)]
        fn default() -> Apb2axisAlmemReset {
            Apb2axisAlmemReset(0)
        }
    }
    impl core::fmt::Debug for Apb2axisAlmemReset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisAlmemReset")
                .field("reset", &self.reset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisAlmemReset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apb2axisAlmemReset {{ reset: {=bool:?} }}", self.reset())
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisAlmemSts(pub u32);
    impl Apb2axisAlmemSts {
        #[doc = "the new data is written to data register."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "the new data is written to data register."]
        #[inline(always)]
        pub const fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the controller is writing data and/or data is pending."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the controller is writing data and/or data is pending."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Apb2axisAlmemSts {
        #[inline(always)]
        fn default() -> Apb2axisAlmemSts {
            Apb2axisAlmemSts(0)
        }
    }
    impl core::fmt::Debug for Apb2axisAlmemSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisAlmemSts")
                .field("rdy", &self.rdy())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisAlmemSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axisAlmemSts {{ rdy: {=bool:?}, busy: {=bool:?} }}",
                self.rdy(),
                self.busy()
            )
        }
    }
    #[doc = "fill status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisCamFillsts(pub u32);
    impl Apb2axisCamFillsts {
        #[doc = "FD FIFO failure, internal controller lost synchronization."]
        #[must_use]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO failure, internal controller lost synchronization."]
        #[inline(always)]
        pub const fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "frame was dropped because the internal descriptor FIFO is full."]
        #[must_use]
        #[inline(always)]
        pub const fn full(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "frame was dropped because the internal descriptor FIFO is full."]
        #[inline(always)]
        pub const fn set_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Apb2axisCamFillsts {
        #[inline(always)]
        fn default() -> Apb2axisCamFillsts {
            Apb2axisCamFillsts(0)
        }
    }
    impl core::fmt::Debug for Apb2axisCamFillsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisCamFillsts")
                .field("empty", &self.empty())
                .field("full", &self.full())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisCamFillsts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axisCamFillsts {{ empty: {=bool:?}, full: {=bool:?} }}",
                self.empty(),
                self.full()
            )
        }
    }
    #[doc = "parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisCamParam(pub u32);
    impl Apb2axisCamParam {
        #[doc = "number of configured 32bit words for this controller."]
        #[must_use]
        #[inline(always)]
        pub const fn wordlen_byte(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "number of configured 32bit words for this controller."]
        #[inline(always)]
        pub const fn set_wordlen_byte(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "number of configured buffer depth."]
        #[must_use]
        #[inline(always)]
        pub const fn depth(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "number of configured buffer depth."]
        #[inline(always)]
        pub const fn set_depth(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Apb2axisCamParam {
        #[inline(always)]
        fn default() -> Apb2axisCamParam {
            Apb2axisCamParam(0)
        }
    }
    impl core::fmt::Debug for Apb2axisCamParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisCamParam")
                .field("wordlen_byte", &self.wordlen_byte())
                .field("depth", &self.depth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisCamParam {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axisCamParam {{ wordlen_byte: {=u8:?}, depth: {=u8:?} }}",
                self.wordlen_byte(),
                self.depth()
            )
        }
    }
    #[doc = "request count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisCamReqCnt(pub u32);
    impl Apb2axisCamReqCnt {
        #[doc = "number of streams in queue."]
        #[must_use]
        #[inline(always)]
        pub const fn wrcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "number of streams in queue."]
        #[inline(always)]
        pub const fn set_wrcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Apb2axisCamReqCnt {
        #[inline(always)]
        fn default() -> Apb2axisCamReqCnt {
            Apb2axisCamReqCnt(0)
        }
    }
    impl core::fmt::Debug for Apb2axisCamReqCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisCamReqCnt")
                .field("wrcnt", &self.wrcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisCamReqCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apb2axisCamReqCnt {{ wrcnt: {=u8:?} }}", self.wrcnt())
        }
    }
    #[doc = "reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisCamReset(pub u32);
    impl Apb2axisCamReset {
        #[doc = "resets controller and clears all pending stream data."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "resets controller and clears all pending stream data."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Apb2axisCamReset {
        #[inline(always)]
        fn default() -> Apb2axisCamReset {
            Apb2axisCamReset(0)
        }
    }
    impl core::fmt::Debug for Apb2axisCamReset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisCamReset")
                .field("reset", &self.reset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisCamReset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apb2axisCamReset {{ reset: {=bool:?} }}", self.reset())
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisCamSts(pub u32);
    impl Apb2axisCamSts {
        #[doc = "the new data is written to data register."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "the new data is written to data register."]
        #[inline(always)]
        pub const fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the controller is writing data and/or data is pending."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the controller is writing data and/or data is pending."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Apb2axisCamSts {
        #[inline(always)]
        fn default() -> Apb2axisCamSts {
            Apb2axisCamSts(0)
        }
    }
    impl core::fmt::Debug for Apb2axisCamSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisCamSts")
                .field("rdy", &self.rdy())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisCamSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axisCamSts {{ rdy: {=bool:?}, busy: {=bool:?} }}",
                self.rdy(),
                self.busy()
            )
        }
    }
    #[doc = "fill status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisLookupFillsts(pub u32);
    impl Apb2axisLookupFillsts {
        #[doc = "FD FIFO failure."]
        #[must_use]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO failure."]
        #[inline(always)]
        pub const fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FD FIFO full."]
        #[must_use]
        #[inline(always)]
        pub const fn full(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO full."]
        #[inline(always)]
        pub const fn set_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Apb2axisLookupFillsts {
        #[inline(always)]
        fn default() -> Apb2axisLookupFillsts {
            Apb2axisLookupFillsts(0)
        }
    }
    impl core::fmt::Debug for Apb2axisLookupFillsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisLookupFillsts")
                .field("empty", &self.empty())
                .field("full", &self.full())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisLookupFillsts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axisLookupFillsts {{ empty: {=bool:?}, full: {=bool:?} }}",
                self.empty(),
                self.full()
            )
        }
    }
    #[doc = "parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisLookupParam(pub u32);
    impl Apb2axisLookupParam {
        #[doc = "number of configured 32bit for this controller."]
        #[must_use]
        #[inline(always)]
        pub const fn wordlen_byte(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "number of configured 32bit for this controller."]
        #[inline(always)]
        pub const fn set_wordlen_byte(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "number of configured buffer depth."]
        #[must_use]
        #[inline(always)]
        pub const fn depth(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "number of configured buffer depth."]
        #[inline(always)]
        pub const fn set_depth(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Apb2axisLookupParam {
        #[inline(always)]
        fn default() -> Apb2axisLookupParam {
            Apb2axisLookupParam(0)
        }
    }
    impl core::fmt::Debug for Apb2axisLookupParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisLookupParam")
                .field("wordlen_byte", &self.wordlen_byte())
                .field("depth", &self.depth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisLookupParam {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axisLookupParam {{ wordlen_byte: {=u8:?}, depth: {=u8:?} }}",
                self.wordlen_byte(),
                self.depth()
            )
        }
    }
    #[doc = "response count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisLookupReqCnt(pub u32);
    impl Apb2axisLookupReqCnt {
        #[doc = "number of streams in queue."]
        #[must_use]
        #[inline(always)]
        pub const fn wrcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "number of streams in queue."]
        #[inline(always)]
        pub const fn set_wrcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Apb2axisLookupReqCnt {
        #[inline(always)]
        fn default() -> Apb2axisLookupReqCnt {
            Apb2axisLookupReqCnt(0)
        }
    }
    impl core::fmt::Debug for Apb2axisLookupReqCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisLookupReqCnt")
                .field("wrcnt", &self.wrcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisLookupReqCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apb2axisLookupReqCnt {{ wrcnt: {=u8:?} }}", self.wrcnt())
        }
    }
    #[doc = "LOOKUP REQUEST Register REQ_DATA_0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisLookupReqdata0(pub u32);
    impl Apb2axisLookupReqdata0 {
        #[doc = "Holding the first four bytes of requested MAC address."]
        #[must_use]
        #[inline(always)]
        pub const fn destmac(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Holding the first four bytes of requested MAC address."]
        #[inline(always)]
        pub const fn set_destmac(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Apb2axisLookupReqdata0 {
        #[inline(always)]
        fn default() -> Apb2axisLookupReqdata0 {
            Apb2axisLookupReqdata0(0)
        }
    }
    impl core::fmt::Debug for Apb2axisLookupReqdata0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisLookupReqdata0")
                .field("destmac", &self.destmac())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisLookupReqdata0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axisLookupReqdata0 {{ destmac: {=u32:?} }}",
                self.destmac()
            )
        }
    }
    #[doc = "LOOKUP REQUEST Register REQ_DATA_1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisLookupReqdata1(pub u32);
    impl Apb2axisLookupReqdata1 {
        #[doc = "Holding the last two bytes of requested MAC address."]
        #[must_use]
        #[inline(always)]
        pub const fn destmac(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Holding the last two bytes of requested MAC address."]
        #[inline(always)]
        pub const fn set_destmac(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Apb2axisLookupReqdata1 {
        #[inline(always)]
        fn default() -> Apb2axisLookupReqdata1 {
            Apb2axisLookupReqdata1(0)
        }
    }
    impl core::fmt::Debug for Apb2axisLookupReqdata1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisLookupReqdata1")
                .field("destmac", &self.destmac())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisLookupReqdata1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axisLookupReqdata1 {{ destmac: {=u16:?} }}",
                self.destmac()
            )
        }
    }
    #[doc = "LOOKUP REQUEST Register REQ_DATA_2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisLookupReqdata3(pub u32);
    impl Apb2axisLookupReqdata3 {
        #[doc = "Set the requested traffic VLAN_TCI, if IS_VLAN=1."]
        #[must_use]
        #[inline(always)]
        pub const fn vlan_tci(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Set the requested traffic VLAN_TCI, if IS_VLAN=1."]
        #[inline(always)]
        pub const fn set_vlan_tci(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Tell the LOOKUP module the requested traffic is VLAN tagged."]
        #[must_use]
        #[inline(always)]
        pub const fn is_vlan(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Tell the LOOKUP module the requested traffic is VLAN tagged."]
        #[inline(always)]
        pub const fn set_is_vlan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Apb2axisLookupReqdata3 {
        #[inline(always)]
        fn default() -> Apb2axisLookupReqdata3 {
            Apb2axisLookupReqdata3(0)
        }
    }
    impl core::fmt::Debug for Apb2axisLookupReqdata3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisLookupReqdata3")
                .field("vlan_tci", &self.vlan_tci())
                .field("is_vlan", &self.is_vlan())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisLookupReqdata3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axisLookupReqdata3 {{ vlan_tci: {=u16:?}, is_vlan: {=bool:?} }}",
                self.vlan_tci(),
                self.is_vlan()
            )
        }
    }
    #[doc = "reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisLookupReset(pub u32);
    impl Apb2axisLookupReset {
        #[doc = "Resets controller and clears all pending stream data."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Resets controller and clears all pending stream data."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Apb2axisLookupReset {
        #[inline(always)]
        fn default() -> Apb2axisLookupReset {
            Apb2axisLookupReset(0)
        }
    }
    impl core::fmt::Debug for Apb2axisLookupReset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisLookupReset")
                .field("reset", &self.reset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisLookupReset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axisLookupReset {{ reset: {=bool:?} }}",
                self.reset()
            )
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2axisLookupSts(pub u32);
    impl Apb2axisLookupSts {
        #[doc = "the new data is written to data register."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "the new data is written to data register."]
        #[inline(always)]
        pub const fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the controller is writing data and/or data is pending."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the controller is writing data and/or data is pending."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Apb2axisLookupSts {
        #[inline(always)]
        fn default() -> Apb2axisLookupSts {
            Apb2axisLookupSts(0)
        }
    }
    impl core::fmt::Debug for Apb2axisLookupSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2axisLookupSts")
                .field("rdy", &self.rdy())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2axisLookupSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2axisLookupSts {{ rdy: {=bool:?}, busy: {=bool:?} }}",
                self.rdy(),
                self.busy()
            )
        }
    }
    #[doc = "fill status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbAlmemFillsts(pub u32);
    impl Axis2apbAlmemFillsts {
        #[doc = "FD FIFO failure."]
        #[must_use]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO failure."]
        #[inline(always)]
        pub const fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FD FIFO full."]
        #[must_use]
        #[inline(always)]
        pub const fn full(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO full."]
        #[inline(always)]
        pub const fn set_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Axis2apbAlmemFillsts {
        #[inline(always)]
        fn default() -> Axis2apbAlmemFillsts {
            Axis2apbAlmemFillsts(0)
        }
    }
    impl core::fmt::Debug for Axis2apbAlmemFillsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbAlmemFillsts")
                .field("empty", &self.empty())
                .field("full", &self.full())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbAlmemFillsts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Axis2apbAlmemFillsts {{ empty: {=bool:?}, full: {=bool:?} }}",
                self.empty(),
                self.full()
            )
        }
    }
    #[doc = "parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbAlmemParam(pub u32);
    impl Axis2apbAlmemParam {
        #[doc = "number of configured 32bit for this controller."]
        #[must_use]
        #[inline(always)]
        pub const fn wordlen_byte(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "number of configured 32bit for this controller."]
        #[inline(always)]
        pub const fn set_wordlen_byte(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "number of configured buffer depth."]
        #[must_use]
        #[inline(always)]
        pub const fn depth(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "number of configured buffer depth."]
        #[inline(always)]
        pub const fn set_depth(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Axis2apbAlmemParam {
        #[inline(always)]
        fn default() -> Axis2apbAlmemParam {
            Axis2apbAlmemParam(0)
        }
    }
    impl core::fmt::Debug for Axis2apbAlmemParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbAlmemParam")
                .field("wordlen_byte", &self.wordlen_byte())
                .field("depth", &self.depth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbAlmemParam {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Axis2apbAlmemParam {{ wordlen_byte: {=u8:?}, depth: {=u8:?} }}",
                self.wordlen_byte(),
                self.depth()
            )
        }
    }
    #[doc = "reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbAlmemReset(pub u32);
    impl Axis2apbAlmemReset {
        #[doc = "Resets controller and clears all pending stream data."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Resets controller and clears all pending stream data."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Axis2apbAlmemReset {
        #[inline(always)]
        fn default() -> Axis2apbAlmemReset {
            Axis2apbAlmemReset(0)
        }
    }
    impl core::fmt::Debug for Axis2apbAlmemReset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbAlmemReset")
                .field("reset", &self.reset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbAlmemReset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Axis2apbAlmemReset {{ reset: {=bool:?} }}", self.reset())
        }
    }
    #[doc = "response count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbAlmemRespCnt(pub u32);
    impl Axis2apbAlmemRespCnt {
        #[doc = "number of streams in queue."]
        #[must_use]
        #[inline(always)]
        pub const fn rdcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "number of streams in queue."]
        #[inline(always)]
        pub const fn set_rdcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Axis2apbAlmemRespCnt {
        #[inline(always)]
        fn default() -> Axis2apbAlmemRespCnt {
            Axis2apbAlmemRespCnt(0)
        }
    }
    impl core::fmt::Debug for Axis2apbAlmemRespCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbAlmemRespCnt")
                .field("rdcnt", &self.rdcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbAlmemRespCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Axis2apbAlmemRespCnt {{ rdcnt: {=u8:?} }}", self.rdcnt())
        }
    }
    #[doc = "data0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbAlmemRespdata0(pub u32);
    impl Axis2apbAlmemRespdata0 {
        #[doc = "destination ports."]
        #[must_use]
        #[inline(always)]
        pub const fn dest(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "destination ports."]
        #[inline(always)]
        pub const fn set_dest(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "select the priority queue if qsel=11."]
        #[must_use]
        #[inline(always)]
        pub const fn queue(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "select the priority queue if qsel=11."]
        #[inline(always)]
        pub const fn set_queue(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "frame should dropped."]
        #[must_use]
        #[inline(always)]
        pub const fn drop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "frame should dropped."]
        #[inline(always)]
        pub const fn set_drop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "define the traffic queue selection."]
        #[must_use]
        #[inline(always)]
        pub const fn qsel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "define the traffic queue selection."]
        #[inline(always)]
        pub const fn set_qsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "user sideband information."]
        #[must_use]
        #[inline(always)]
        pub const fn utag(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x07;
            val as u8
        }
        #[doc = "user sideband information."]
        #[inline(always)]
        pub const fn set_utag(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
        }
    }
    impl Default for Axis2apbAlmemRespdata0 {
        #[inline(always)]
        fn default() -> Axis2apbAlmemRespdata0 {
            Axis2apbAlmemRespdata0(0)
        }
    }
    impl core::fmt::Debug for Axis2apbAlmemRespdata0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbAlmemRespdata0")
                .field("dest", &self.dest())
                .field("queue", &self.queue())
                .field("drop", &self.drop())
                .field("qsel", &self.qsel())
                .field("utag", &self.utag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbAlmemRespdata0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Axis2apbAlmemRespdata0 {{ dest: {=u16:?}, queue: {=u8:?}, drop: {=bool:?}, qsel: {=u8:?}, utag: {=u8:?} }}" , self . dest () , self . queue () , self . drop () , self . qsel () , self . utag ())
        }
    }
    #[doc = "data1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbAlmemRespdata1(pub u32);
    impl Axis2apbAlmemRespdata1 {
        #[doc = "define the entry number for reading and writing."]
        #[must_use]
        #[inline(always)]
        pub const fn entry_num(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "define the entry number for reading and writing."]
        #[inline(always)]
        pub const fn set_entry_num(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "write response enable."]
        #[must_use]
        #[inline(always)]
        pub const fn resp(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "write response enable."]
        #[inline(always)]
        pub const fn set_resp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "1 for write and 0 for read."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_nrd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "1 for write and 0 for read."]
        #[inline(always)]
        pub const fn set_wr_nrd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Axis2apbAlmemRespdata1 {
        #[inline(always)]
        fn default() -> Axis2apbAlmemRespdata1 {
            Axis2apbAlmemRespdata1(0)
        }
    }
    impl core::fmt::Debug for Axis2apbAlmemRespdata1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbAlmemRespdata1")
                .field("entry_num", &self.entry_num())
                .field("resp", &self.resp())
                .field("wr_nrd", &self.wr_nrd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbAlmemRespdata1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Axis2apbAlmemRespdata1 {{ entry_num: {=u16:?}, resp: {=bool:?}, wr_nrd: {=bool:?} }}" , self . entry_num () , self . resp () , self . wr_nrd ())
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbAlmemSts(pub u32);
    impl Axis2apbAlmemSts {
        #[doc = "the new data is written to data register."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "the new data is written to data register."]
        #[inline(always)]
        pub const fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the controller is writing data and/or data is pending."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the controller is writing data and/or data is pending."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Axis2apbAlmemSts {
        #[inline(always)]
        fn default() -> Axis2apbAlmemSts {
            Axis2apbAlmemSts(0)
        }
    }
    impl core::fmt::Debug for Axis2apbAlmemSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbAlmemSts")
                .field("rdy", &self.rdy())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbAlmemSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Axis2apbAlmemSts {{ rdy: {=bool:?}, busy: {=bool:?} }}",
                self.rdy(),
                self.busy()
            )
        }
    }
    #[doc = "fill status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbLookupFillsts(pub u32);
    impl Axis2apbLookupFillsts {
        #[doc = "FD FIFO failure."]
        #[must_use]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO failure."]
        #[inline(always)]
        pub const fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FD FIFO full."]
        #[must_use]
        #[inline(always)]
        pub const fn full(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO full."]
        #[inline(always)]
        pub const fn set_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Axis2apbLookupFillsts {
        #[inline(always)]
        fn default() -> Axis2apbLookupFillsts {
            Axis2apbLookupFillsts(0)
        }
    }
    impl core::fmt::Debug for Axis2apbLookupFillsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbLookupFillsts")
                .field("empty", &self.empty())
                .field("full", &self.full())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbLookupFillsts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Axis2apbLookupFillsts {{ empty: {=bool:?}, full: {=bool:?} }}",
                self.empty(),
                self.full()
            )
        }
    }
    #[doc = "parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbLookupParam(pub u32);
    impl Axis2apbLookupParam {
        #[doc = "number of configured 32bit for this controller."]
        #[must_use]
        #[inline(always)]
        pub const fn wordlen_byte(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "number of configured 32bit for this controller."]
        #[inline(always)]
        pub const fn set_wordlen_byte(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "number of configured buffer depth."]
        #[must_use]
        #[inline(always)]
        pub const fn depth(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "number of configured buffer depth."]
        #[inline(always)]
        pub const fn set_depth(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Axis2apbLookupParam {
        #[inline(always)]
        fn default() -> Axis2apbLookupParam {
            Axis2apbLookupParam(0)
        }
    }
    impl core::fmt::Debug for Axis2apbLookupParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbLookupParam")
                .field("wordlen_byte", &self.wordlen_byte())
                .field("depth", &self.depth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbLookupParam {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Axis2apbLookupParam {{ wordlen_byte: {=u8:?}, depth: {=u8:?} }}",
                self.wordlen_byte(),
                self.depth()
            )
        }
    }
    #[doc = "reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbLookupReset(pub u32);
    impl Axis2apbLookupReset {
        #[doc = "Resets controller and clears all pending stream data."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Resets controller and clears all pending stream data."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Axis2apbLookupReset {
        #[inline(always)]
        fn default() -> Axis2apbLookupReset {
            Axis2apbLookupReset(0)
        }
    }
    impl core::fmt::Debug for Axis2apbLookupReset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbLookupReset")
                .field("reset", &self.reset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbLookupReset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Axis2apbLookupReset {{ reset: {=bool:?} }}",
                self.reset()
            )
        }
    }
    #[doc = "response count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbLookupRespCnt(pub u32);
    impl Axis2apbLookupRespCnt {
        #[doc = "number of streams in queue."]
        #[must_use]
        #[inline(always)]
        pub const fn rdcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "number of streams in queue."]
        #[inline(always)]
        pub const fn set_rdcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Axis2apbLookupRespCnt {
        #[inline(always)]
        fn default() -> Axis2apbLookupRespCnt {
            Axis2apbLookupRespCnt(0)
        }
    }
    impl core::fmt::Debug for Axis2apbLookupRespCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbLookupRespCnt")
                .field("rdcnt", &self.rdcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbLookupRespCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Axis2apbLookupRespCnt {{ rdcnt: {=u8:?} }}",
                self.rdcnt()
            )
        }
    }
    #[doc = "LOOKUP RESPONSE Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbLookupRespdata0(pub u32);
    impl Axis2apbLookupRespdata0 {
        #[doc = "Forwarding ports from 0 to 15, Bit 0 is CPU port."]
        #[must_use]
        #[inline(always)]
        pub const fn dest(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Forwarding ports from 0 to 15, Bit 0 is CPU port."]
        #[inline(always)]
        pub const fn set_dest(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "TX traffic queue selection."]
        #[must_use]
        #[inline(always)]
        pub const fn queue(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "TX traffic queue selection."]
        #[inline(always)]
        pub const fn set_queue(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Indicate that the frame should be dropped."]
        #[must_use]
        #[inline(always)]
        pub const fn drop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Indicate that the frame should be dropped."]
        #[inline(always)]
        pub const fn set_drop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Is 1, if VID hit entry in VLAN_PORT table."]
        #[must_use]
        #[inline(always)]
        pub const fn hit_vlan(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Is 1, if VID hit entry in VLAN_PORT table."]
        #[inline(always)]
        pub const fn set_hit_vlan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "TSN user sideband information from ALMEM."]
        #[must_use]
        #[inline(always)]
        pub const fn utag(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "TSN user sideband information from ALMEM."]
        #[inline(always)]
        pub const fn set_utag(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "Is 1, if DESTMAC and VID hit an entry."]
        #[must_use]
        #[inline(always)]
        pub const fn hit(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Is 1, if DESTMAC and VID hit an entry."]
        #[inline(always)]
        pub const fn set_hit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Used for statistics. Shows that drop occurs by VLAN-ID."]
        #[must_use]
        #[inline(always)]
        pub const fn drop_vlan(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Used for statistics. Shows that drop occurs by VLAN-ID."]
        #[inline(always)]
        pub const fn set_drop_vlan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Axis2apbLookupRespdata0 {
        #[inline(always)]
        fn default() -> Axis2apbLookupRespdata0 {
            Axis2apbLookupRespdata0(0)
        }
    }
    impl core::fmt::Debug for Axis2apbLookupRespdata0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbLookupRespdata0")
                .field("dest", &self.dest())
                .field("queue", &self.queue())
                .field("drop", &self.drop())
                .field("hit_vlan", &self.hit_vlan())
                .field("utag", &self.utag())
                .field("hit", &self.hit())
                .field("drop_vlan", &self.drop_vlan())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbLookupRespdata0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Axis2apbLookupRespdata0 {{ dest: {=u16:?}, queue: {=u8:?}, drop: {=bool:?}, hit_vlan: {=bool:?}, utag: {=u8:?}, hit: {=bool:?}, drop_vlan: {=bool:?} }}" , self . dest () , self . queue () , self . drop () , self . hit_vlan () , self . utag () , self . hit () , self . drop_vlan ())
        }
    }
    #[doc = "LOOKUP RESPONSE Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbLookupRespdata1(pub u32);
    impl Axis2apbLookupRespdata1 {
        #[doc = "Entry number of ALMEM."]
        #[must_use]
        #[inline(always)]
        pub const fn entry_num(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Entry number of ALMEM."]
        #[inline(always)]
        pub const fn set_entry_num(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Axis2apbLookupRespdata1 {
        #[inline(always)]
        fn default() -> Axis2apbLookupRespdata1 {
            Axis2apbLookupRespdata1(0)
        }
    }
    impl core::fmt::Debug for Axis2apbLookupRespdata1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbLookupRespdata1")
                .field("entry_num", &self.entry_num())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbLookupRespdata1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Axis2apbLookupRespdata1 {{ entry_num: {=u16:?} }}",
                self.entry_num()
            )
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Axis2apbLookupSts(pub u32);
    impl Axis2apbLookupSts {
        #[doc = "the new data is written to data register."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "the new data is written to data register."]
        #[inline(always)]
        pub const fn set_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the controller is writing data and/or data is pending."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the controller is writing data and/or data is pending."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Axis2apbLookupSts {
        #[inline(always)]
        fn default() -> Axis2apbLookupSts {
            Axis2apbLookupSts(0)
        }
    }
    impl core::fmt::Debug for Axis2apbLookupSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Axis2apbLookupSts")
                .field("rdy", &self.rdy())
                .field("busy", &self.busy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Axis2apbLookupSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Axis2apbLookupSts {{ rdy: {=bool:?}, busy: {=bool:?} }}",
                self.rdy(),
                self.busy()
            )
        }
    }
    #[doc = "CB Parameter Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralCsrCbParam(pub u32);
    impl CentralCsrCbParam {
        #[doc = "Number of 802.1CB Recovery Function entries. 2^FRER_D entries."]
        #[must_use]
        #[inline(always)]
        pub const fn frer_d(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Number of 802.1CB Recovery Function entries. 2^FRER_D entries."]
        #[inline(always)]
        pub const fn set_frer_d(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Number of 802.1CB Stream Identification entries. 2^SID_D entries."]
        #[must_use]
        #[inline(always)]
        pub const fn sid_d(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Number of 802.1CB Stream Identification entries. 2^SID_D entries."]
        #[inline(always)]
        pub const fn set_sid_d(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for CentralCsrCbParam {
        #[inline(always)]
        fn default() -> CentralCsrCbParam {
            CentralCsrCbParam(0)
        }
    }
    impl core::fmt::Debug for CentralCsrCbParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralCsrCbParam")
                .field("frer_d", &self.frer_d())
                .field("sid_d", &self.sid_d())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralCsrCbParam {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CentralCsrCbParam {{ frer_d: {=u8:?}, sid_d: {=u8:?} }}",
                self.frer_d(),
                self.sid_d()
            )
        }
    }
    #[doc = "Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralCsrConfig(pub u32);
    impl CentralCsrConfig {
        #[doc = "Number of SYS_CLK cycles during 1 ms. It is required to calculate a correct time."]
        #[must_use]
        #[inline(always)]
        pub const fn msec_cycles(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Number of SYS_CLK cycles during 1 ms. It is required to calculate a correct time."]
        #[inline(always)]
        pub const fn set_msec_cycles(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for CentralCsrConfig {
        #[inline(always)]
        fn default() -> CentralCsrConfig {
            CentralCsrConfig(0)
        }
    }
    impl core::fmt::Debug for CentralCsrConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralCsrConfig")
                .field("msec_cycles", &self.msec_cycles())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralCsrConfig {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CentralCsrConfig {{ msec_cycles: {=u32:?} }}",
                self.msec_cycles()
            )
        }
    }
    #[doc = "Parameter Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralCsrParam(pub u32);
    impl CentralCsrParam {
        #[doc = "Number of TSN ports without counting internal CPU port. For TSN-SE, it returns always 2."]
        #[must_use]
        #[inline(always)]
        pub const fn nports(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Number of TSN ports without counting internal CPU port. For TSN-SE, it returns always 2."]
        #[inline(always)]
        pub const fn set_nports(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Specify type of switch core."]
        #[must_use]
        #[inline(always)]
        pub const fn type_(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Specify type of switch core."]
        #[inline(always)]
        pub const fn set_type_(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Shows if IP is configured in TESTMODE."]
        #[must_use]
        #[inline(always)]
        pub const fn testmode(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Shows if IP is configured in TESTMODE."]
        #[inline(always)]
        pub const fn set_testmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Shows if IP is configured with “lightweight” 802.1CB at CPU-Port."]
        #[must_use]
        #[inline(always)]
        pub const fn incl_cb0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Shows if IP is configured with “lightweight” 802.1CB at CPU-Port."]
        #[inline(always)]
        pub const fn set_incl_cb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Shows if QCI module is present."]
        #[must_use]
        #[inline(always)]
        pub const fn incl_qci(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Shows if QCI module is present."]
        #[inline(always)]
        pub const fn set_incl_qci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for CentralCsrParam {
        #[inline(always)]
        fn default() -> CentralCsrParam {
            CentralCsrParam(0)
        }
    }
    impl core::fmt::Debug for CentralCsrParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralCsrParam")
                .field("nports", &self.nports())
                .field("type_", &self.type_())
                .field("testmode", &self.testmode())
                .field("incl_cb0", &self.incl_cb0())
                .field("incl_qci", &self.incl_qci())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralCsrParam {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CentralCsrParam {{ nports: {=u8:?}, type_: {=u8:?}, testmode: {=bool:?}, incl_cb0: {=bool:?}, incl_qci: {=bool:?} }}" , self . nports () , self . type_ () , self . testmode () , self . incl_cb0 () , self . incl_qci ())
        }
    }
    #[doc = "QCI Control Parameter Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralCsrQciCtrlParam(pub u32);
    impl CentralCsrQciCtrlParam {
        #[doc = "(Log) filter table depth. 2**FTD entries."]
        #[must_use]
        #[inline(always)]
        pub const fn qci_ftd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "(Log) filter table depth. 2**FTD entries."]
        #[inline(always)]
        pub const fn set_qci_ftd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "(Log) flow meter depth. 2**FMD entries."]
        #[must_use]
        #[inline(always)]
        pub const fn qci_fmd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "(Log) flow meter depth. 2**FMD entries."]
        #[inline(always)]
        pub const fn set_qci_fmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "(Log) gate table depth. 2**GTD entries."]
        #[must_use]
        #[inline(always)]
        pub const fn qci_gtd(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "(Log) gate table depth. 2**GTD entries."]
        #[inline(always)]
        pub const fn set_qci_gtd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for CentralCsrQciCtrlParam {
        #[inline(always)]
        fn default() -> CentralCsrQciCtrlParam {
            CentralCsrQciCtrlParam(0)
        }
    }
    impl core::fmt::Debug for CentralCsrQciCtrlParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralCsrQciCtrlParam")
                .field("qci_ftd", &self.qci_ftd())
                .field("qci_fmd", &self.qci_fmd())
                .field("qci_gtd", &self.qci_gtd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralCsrQciCtrlParam {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CentralCsrQciCtrlParam {{ qci_ftd: {=u8:?}, qci_fmd: {=u8:?}, qci_gtd: {=u8:?} }}",
                self.qci_ftd(),
                self.qci_fmd(),
                self.qci_gtd()
            )
        }
    }
    #[doc = "version register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralCsrVersion(pub u32);
    impl CentralCsrVersion {
        #[doc = "Reversion number of TSN-SW core."]
        #[must_use]
        #[inline(always)]
        pub const fn ver_rev(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Reversion number of TSN-SW core."]
        #[inline(always)]
        pub const fn set_ver_rev(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Minor Version number of TSN-SW core."]
        #[must_use]
        #[inline(always)]
        pub const fn ver_lo(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Minor Version number of TSN-SW core."]
        #[inline(always)]
        pub const fn set_ver_lo(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Major Version number of TSN-SW core."]
        #[must_use]
        #[inline(always)]
        pub const fn ver_hi(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Major Version number of TSN-SW core."]
        #[inline(always)]
        pub const fn set_ver_hi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for CentralCsrVersion {
        #[inline(always)]
        fn default() -> CentralCsrVersion {
            CentralCsrVersion(0)
        }
    }
    impl core::fmt::Debug for CentralCsrVersion {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralCsrVersion")
                .field("ver_rev", &self.ver_rev())
                .field("ver_lo", &self.ver_lo())
                .field("ver_hi", &self.ver_hi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralCsrVersion {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CentralCsrVersion {{ ver_rev: {=u8:?}, ver_lo: {=u8:?}, ver_hi: {=u8:?} }}",
                self.ver_rev(),
                self.ver_lo(),
                self.ver_hi()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciAbasetmH(pub u32);
    impl CentralQciAbasetmH {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn abth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_abth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CentralQciAbasetmH {
        #[inline(always)]
        fn default() -> CentralQciAbasetmH {
            CentralQciAbasetmH(0)
        }
    }
    impl core::fmt::Debug for CentralQciAbasetmH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciAbasetmH")
                .field("abth", &self.abth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciAbasetmH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CentralQciAbasetmH {{ abth: {=u32:?} }}", self.abth())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciAbasetmL(pub u32);
    impl CentralQciAbasetmL {
        #[doc = "Administrative base time. Nanoseconds and seconds part. Cycle starts after becoming operational when time is reached by inputs <rtc_sec> and <rtc_ns>."]
        #[must_use]
        #[inline(always)]
        pub const fn abtl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Administrative base time. Nanoseconds and seconds part. Cycle starts after becoming operational when time is reached by inputs <rtc_sec> and <rtc_ns>."]
        #[inline(always)]
        pub const fn set_abtl(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for CentralQciAbasetmL {
        #[inline(always)]
        fn default() -> CentralQciAbasetmL {
            CentralQciAbasetmL(0)
        }
    }
    impl core::fmt::Debug for CentralQciAbasetmL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciAbasetmL")
                .field("abtl", &self.abtl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciAbasetmL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CentralQciAbasetmL {{ abtl: {=u32:?} }}", self.abtl())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciAcycletm(pub u32);
    impl CentralQciAcycletm {
        #[doc = "Administrative cycle time length, nanoseconds."]
        #[must_use]
        #[inline(always)]
        pub const fn act(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Administrative cycle time length, nanoseconds."]
        #[inline(always)]
        pub const fn set_act(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for CentralQciAcycletm {
        #[inline(always)]
        fn default() -> CentralQciAcycletm {
            CentralQciAcycletm(0)
        }
    }
    impl core::fmt::Debug for CentralQciAcycletm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciAcycletm")
                .field("act", &self.act())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciAcycletm {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CentralQciAcycletm {{ act: {=u32:?} }}", self.act())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciAentryAentryIval(pub u32);
    impl CentralQciAentryAentryIval {
        #[doc = "AdminList – time interval in clock ticks."]
        #[must_use]
        #[inline(always)]
        pub const fn ival(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "AdminList – time interval in clock ticks."]
        #[inline(always)]
        pub const fn set_ival(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CentralQciAentryAentryIval {
        #[inline(always)]
        fn default() -> CentralQciAentryAentryIval {
            CentralQciAentryAentryIval(0)
        }
    }
    impl core::fmt::Debug for CentralQciAentryAentryIval {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciAentryAentryIval")
                .field("ival", &self.ival())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciAentryAentryIval {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CentralQciAentryAentryIval {{ ival: {=u32:?} }}",
                self.ival()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciAentryCtrl(pub u32);
    impl CentralQciAentryCtrl {
        #[doc = "AdminList – maximum octets (0 – disabled)."]
        #[must_use]
        #[inline(always)]
        pub const fn oct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "AdminList – maximum octets (0 – disabled)."]
        #[inline(always)]
        pub const fn set_oct(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
        #[doc = "AdminList – IPV."]
        #[must_use]
        #[inline(always)]
        pub const fn ipv(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "AdminList – IPV."]
        #[inline(always)]
        pub const fn set_ipv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[doc = "AdminList – gate state (1: open)."]
        #[must_use]
        #[inline(always)]
        pub const fn state(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AdminList – gate state (1: open)."]
        #[inline(always)]
        pub const fn set_state(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CentralQciAentryCtrl {
        #[inline(always)]
        fn default() -> CentralQciAentryCtrl {
            CentralQciAentryCtrl(0)
        }
    }
    impl core::fmt::Debug for CentralQciAentryCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciAentryCtrl")
                .field("oct", &self.oct())
                .field("ipv", &self.ipv())
                .field("state", &self.state())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciAentryCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CentralQciAentryCtrl {{ oct: {=u32:?}, ipv: {=u8:?}, state: {=bool:?} }}",
                self.oct(),
                self.ipv(),
                self.state()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciAentryObasetmH(pub u32);
    impl CentralQciAentryObasetmH {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn obth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_obth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CentralQciAentryObasetmH {
        #[inline(always)]
        fn default() -> CentralQciAentryObasetmH {
            CentralQciAentryObasetmH(0)
        }
    }
    impl core::fmt::Debug for CentralQciAentryObasetmH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciAentryObasetmH")
                .field("obth", &self.obth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciAentryObasetmH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CentralQciAentryObasetmH {{ obth: {=u32:?} }}",
                self.obth()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciAentryObasetmL(pub u32);
    impl CentralQciAentryObasetmL {
        #[doc = "OperBaseTime – nanoseconds and seconds. Constantly updated – OperBaseTime + N * OperCycleTimt. Might be non-normalized."]
        #[must_use]
        #[inline(always)]
        pub const fn obtl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OperBaseTime – nanoseconds and seconds. Constantly updated – OperBaseTime + N * OperCycleTimt. Might be non-normalized."]
        #[inline(always)]
        pub const fn set_obtl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CentralQciAentryObasetmL {
        #[inline(always)]
        fn default() -> CentralQciAentryObasetmL {
            CentralQciAentryObasetmL(0)
        }
    }
    impl core::fmt::Debug for CentralQciAentryObasetmL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciAentryObasetmL")
                .field("obtl", &self.obtl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciAentryObasetmL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CentralQciAentryObasetmL {{ obtl: {=u32:?} }}",
                self.obtl()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciAentryOcycletm(pub u32);
    impl CentralQciAentryOcycletm {
        #[doc = "OperCycleTime in nanoseconds."]
        #[must_use]
        #[inline(always)]
        pub const fn oct(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OperCycleTime in nanoseconds."]
        #[inline(always)]
        pub const fn set_oct(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CentralQciAentryOcycletm {
        #[inline(always)]
        fn default() -> CentralQciAentryOcycletm {
            CentralQciAentryOcycletm(0)
        }
    }
    impl core::fmt::Debug for CentralQciAentryOcycletm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciAentryOcycletm")
                .field("oct", &self.oct())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciAentryOcycletm {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CentralQciAentryOcycletm {{ oct: {=u32:?} }}",
                self.oct()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciCbs(pub u32);
    impl CentralQciCbs {
        #[doc = "Committed burst size, in bits (not octets!) (802.1Qci – 8.6.5.1.3 (c))."]
        #[must_use]
        #[inline(always)]
        pub const fn cbs(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Committed burst size, in bits (not octets!) (802.1Qci – 8.6.5.1.3 (c))."]
        #[inline(always)]
        pub const fn set_cbs(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CentralQciCbs {
        #[inline(always)]
        fn default() -> CentralQciCbs {
            CentralQciCbs(0)
        }
    }
    impl core::fmt::Debug for CentralQciCbs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciCbs")
                .field("cbs", &self.cbs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciCbs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CentralQciCbs {{ cbs: {=u32:?} }}", self.cbs())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciCir(pub u32);
    impl CentralQciCir {
        #[doc = "Committed information rate – see Chapter 7.5.2.4. (802.1Qci – 8.6.5.1.3 (b))."]
        #[must_use]
        #[inline(always)]
        pub const fn cir(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Committed information rate – see Chapter 7.5.2.4. (802.1Qci – 8.6.5.1.3 (b))."]
        #[inline(always)]
        pub const fn set_cir(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for CentralQciCir {
        #[inline(always)]
        fn default() -> CentralQciCir {
            CentralQciCir(0)
        }
    }
    impl core::fmt::Debug for CentralQciCir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciCir")
                .field("cir", &self.cir())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciCir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CentralQciCir {{ cir: {=u32:?} }}", self.cir())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciEbs(pub u32);
    impl CentralQciEbs {
        #[doc = "Excess burst size, in bits (not octets) (802.1Qci – 8.6.5.1.3 (e))."]
        #[must_use]
        #[inline(always)]
        pub const fn ebs(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Excess burst size, in bits (not octets) (802.1Qci – 8.6.5.1.3 (e))."]
        #[inline(always)]
        pub const fn set_ebs(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CentralQciEbs {
        #[inline(always)]
        fn default() -> CentralQciEbs {
            CentralQciEbs(0)
        }
    }
    impl core::fmt::Debug for CentralQciEbs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciEbs")
                .field("ebs", &self.ebs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciEbs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CentralQciEbs {{ ebs: {=u32:?} }}", self.ebs())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciEir(pub u32);
    impl CentralQciEir {
        #[doc = "Excess information rate – see Chapter 7.5.2.4. (802.1Qci – 8.6.5.1.3 (d))."]
        #[must_use]
        #[inline(always)]
        pub const fn eir(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Excess information rate – see Chapter 7.5.2.4. (802.1Qci – 8.6.5.1.3 (d))."]
        #[inline(always)]
        pub const fn set_eir(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for CentralQciEir {
        #[inline(always)]
        fn default() -> CentralQciEir {
            CentralQciEir(0)
        }
    }
    impl core::fmt::Debug for CentralQciEir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciEir")
                .field("eir", &self.eir())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciEir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CentralQciEir {{ eir: {=u32:?} }}", self.eir())
        }
    }
    #[doc = "FILTER SETTING."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciFctrl(pub u32);
    impl CentralQciFctrl {
        #[doc = "Filter Stream ID – if enabled by ENSID."]
        #[must_use]
        #[inline(always)]
        pub const fn sid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Filter Stream ID – if enabled by ENSID."]
        #[inline(always)]
        pub const fn set_sid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Associated Gate."]
        #[must_use]
        #[inline(always)]
        pub const fn gid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Associated Gate."]
        #[inline(always)]
        pub const fn set_gid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Associated Flow Meter – if enabled by ENFID."]
        #[must_use]
        #[inline(always)]
        pub const fn fmd(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Associated Flow Meter – if enabled by ENFID."]
        #[inline(always)]
        pub const fn set_fmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Filter priority code point, if enabled by ENPCP."]
        #[must_use]
        #[inline(always)]
        pub const fn pcp(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Filter priority code point, if enabled by ENPCP."]
        #[inline(always)]
        pub const fn set_pcp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "0: Filter match any PCP value 1: Filter match PCP value (802.1Qci – 8.6.5.1.1 (c))."]
        #[must_use]
        #[inline(always)]
        pub const fn enpcp(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "0: Filter match any PCP value 1: Filter match PCP value (802.1Qci – 8.6.5.1.1 (c))."]
        #[inline(always)]
        pub const fn set_enpcp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "0: Filter match any SID value 1: Filter match SID value (802.1Qci – 8.6.5.1.1 (b))."]
        #[must_use]
        #[inline(always)]
        pub const fn ensid(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "0: Filter match any SID value 1: Filter match SID value (802.1Qci – 8.6.5.1.1 (b))."]
        #[inline(always)]
        pub const fn set_ensid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "0: No Flow Meter 1: Enable Flow Metering (802.1Qci – 8.6.5.1.1 (e.2))."]
        #[must_use]
        #[inline(always)]
        pub const fn enfid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "0: No Flow Meter 1: Enable Flow Metering (802.1Qci – 8.6.5.1.1 (e.2))."]
        #[inline(always)]
        pub const fn set_enfid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "0: No frame size check 1: Frame size checking, size defined by FSIZE.MXSZ (802.1Qci – 8.6.5.1.1 (e.1))."]
        #[must_use]
        #[inline(always)]
        pub const fn enfsz(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "0: No frame size check 1: Frame size checking, size defined by FSIZE.MXSZ (802.1Qci – 8.6.5.1.1 (e.1))."]
        #[inline(always)]
        pub const fn set_enfsz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Enable blocking of oversized frames (802.1Qci – 8.6.5.1.1 (g))."]
        #[must_use]
        #[inline(always)]
        pub const fn enblk(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Enable blocking of oversized frames (802.1Qci – 8.6.5.1.1 (g))."]
        #[inline(always)]
        pub const fn set_enblk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CentralQciFctrl {
        #[inline(always)]
        fn default() -> CentralQciFctrl {
            CentralQciFctrl(0)
        }
    }
    impl core::fmt::Debug for CentralQciFctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciFctrl")
                .field("sid", &self.sid())
                .field("gid", &self.gid())
                .field("fmd", &self.fmd())
                .field("pcp", &self.pcp())
                .field("enpcp", &self.enpcp())
                .field("ensid", &self.ensid())
                .field("enfid", &self.enfid())
                .field("enfsz", &self.enfsz())
                .field("enblk", &self.enblk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciFctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CentralQciFctrl {{ sid: {=u8:?}, gid: {=u8:?}, fmd: {=u8:?}, pcp: {=u8:?}, enpcp: {=bool:?}, ensid: {=bool:?}, enfid: {=bool:?}, enfsz: {=bool:?}, enblk: {=bool:?} }}" , self . sid () , self . gid () , self . fmd () , self . pcp () , self . enpcp () , self . ensid () , self . enfid () , self . enfsz () , self . enblk ())
        }
    }
    #[doc = "Filter select index."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciFiltersel(pub u32);
    impl CentralQciFiltersel {
        #[doc = "Filter select index Any written value larger than the maximum index (2**FTD-1) will result in a read-back value of <0>."]
        #[must_use]
        #[inline(always)]
        pub const fn index(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Filter select index Any written value larger than the maximum index (2**FTD-1) will result in a read-back value of <0>."]
        #[inline(always)]
        pub const fn set_index(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CentralQciFiltersel {
        #[inline(always)]
        fn default() -> CentralQciFiltersel {
            CentralQciFiltersel(0)
        }
    }
    impl core::fmt::Debug for CentralQciFiltersel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciFiltersel")
                .field("index", &self.index())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciFiltersel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CentralQciFiltersel {{ index: {=u8:?} }}", self.index())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciFsize(pub u32);
    impl CentralQciFsize {
        #[doc = "Maximum-SDU size in octets."]
        #[must_use]
        #[inline(always)]
        pub const fn mxsz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Maximum-SDU size in octets."]
        #[inline(always)]
        pub const fn set_mxsz(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Stream blocked due to oversize frame. Write <1> to clear. (802.1Qci – 8.6.5.1.1 (h))."]
        #[must_use]
        #[inline(always)]
        pub const fn blk(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Stream blocked due to oversize frame. Write <1> to clear. (802.1Qci – 8.6.5.1.1 (h))."]
        #[inline(always)]
        pub const fn set_blk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CentralQciFsize {
        #[inline(always)]
        fn default() -> CentralQciFsize {
            CentralQciFsize(0)
        }
    }
    impl core::fmt::Debug for CentralQciFsize {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciFsize")
                .field("mxsz", &self.mxsz())
                .field("blk", &self.blk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciFsize {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CentralQciFsize {{ mxsz: {=u16:?}, blk: {=bool:?} }}",
                self.mxsz(),
                self.blk()
            )
        }
    }
    #[doc = "Gate select index."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciGatesel(pub u32);
    impl CentralQciGatesel {
        #[doc = "Gate select index Any written value larger than the maximum index (2**GTD-1) will result in a read-back value of <0>."]
        #[must_use]
        #[inline(always)]
        pub const fn index(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Gate select index Any written value larger than the maximum index (2**GTD-1) will result in a read-back value of <0>."]
        #[inline(always)]
        pub const fn set_index(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CentralQciGatesel {
        #[inline(always)]
        fn default() -> CentralQciGatesel {
            CentralQciGatesel(0)
        }
    }
    impl core::fmt::Debug for CentralQciGatesel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciGatesel")
                .field("index", &self.index())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciGatesel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CentralQciGatesel {{ index: {=u8:?} }}", self.index())
        }
    }
    #[doc = "Gate settings."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciGctrl(pub u32);
    impl CentralQciGctrl {
        #[doc = "Gate control – enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Gate control – enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Gate – change config (self-resetting to <0>)."]
        #[must_use]
        #[inline(always)]
        pub const fn cfgch(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Gate – change config (self-resetting to <0>)."]
        #[inline(always)]
        pub const fn set_cfgch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Gate – ClosedDueToInvalidRxEnable (802.1Qci – 8.6.5.1.2 (d))."]
        #[must_use]
        #[inline(always)]
        pub const fn cdire(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Gate – ClosedDueToInvalidRxEnable (802.1Qci – 8.6.5.1.2 (d))."]
        #[inline(always)]
        pub const fn set_cdire(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Gate – ClosedDueToOctetsExceededEnable (802.1Qci – 8.6.5.1.2 (f))."]
        #[must_use]
        #[inline(always)]
        pub const fn cdoee(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Gate – ClosedDueToOctetsExceededEnable (802.1Qci – 8.6.5.1.2 (f))."]
        #[inline(always)]
        pub const fn set_cdoee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Administrative stream gate state (802.1Qci – 8.6.5.1.2 (b))."]
        #[must_use]
        #[inline(always)]
        pub const fn state(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Administrative stream gate state (802.1Qci – 8.6.5.1.2 (b))."]
        #[inline(always)]
        pub const fn set_state(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Administrative internal priority value specification (802.1Qci – 8.6.5.1.2 (c))."]
        #[must_use]
        #[inline(always)]
        pub const fn ipv(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "Administrative internal priority value specification (802.1Qci – 8.6.5.1.2 (c))."]
        #[inline(always)]
        pub const fn set_ipv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
    }
    impl Default for CentralQciGctrl {
        #[inline(always)]
        fn default() -> CentralQciGctrl {
            CentralQciGctrl(0)
        }
    }
    impl core::fmt::Debug for CentralQciGctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciGctrl")
                .field("en", &self.en())
                .field("cfgch", &self.cfgch())
                .field("cdire", &self.cdire())
                .field("cdoee", &self.cdoee())
                .field("state", &self.state())
                .field("ipv", &self.ipv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciGctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CentralQciGctrl {{ en: {=bool:?}, cfgch: {=bool:?}, cdire: {=bool:?}, cdoee: {=bool:?}, state: {=bool:?}, ipv: {=u8:?} }}" , self . en () , self . cfgch () , self . cdire () , self . cdoee () , self . state () , self . ipv ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciGlistindex(pub u32);
    impl CentralQciGlistindex {
        #[doc = "Admin list pointer, select entry 0 – 15."]
        #[must_use]
        #[inline(always)]
        pub const fn idx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Admin list pointer, select entry 0 – 15."]
        #[inline(always)]
        pub const fn set_idx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for CentralQciGlistindex {
        #[inline(always)]
        fn default() -> CentralQciGlistindex {
            CentralQciGlistindex(0)
        }
    }
    impl core::fmt::Debug for CentralQciGlistindex {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciGlistindex")
                .field("idx", &self.idx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciGlistindex {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CentralQciGlistindex {{ idx: {=u8:?} }}", self.idx())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciGstatus(pub u32);
    impl CentralQciGstatus {
        #[doc = "Configuration change error. Write <1> to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn cfgerr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration change error. Write <1> to clear."]
        #[inline(always)]
        pub const fn set_cfgerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Configuration change pending."]
        #[must_use]
        #[inline(always)]
        pub const fn cfgp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration change pending."]
        #[inline(always)]
        pub const fn set_cfgp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Gate – ClosedDueToInvalidRx. Write <1> to clear. (802.1Qci – 8.6.5.1.2 (e))."]
        #[must_use]
        #[inline(always)]
        pub const fn cdir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Gate – ClosedDueToInvalidRx. Write <1> to clear. (802.1Qci – 8.6.5.1.2 (e))."]
        #[inline(always)]
        pub const fn set_cdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Gate – ClosedDueToOctetsExceeded. Write <1> to clear. (802.1Qci – 8.6.5.1.2 (g))."]
        #[must_use]
        #[inline(always)]
        pub const fn cdoe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Gate – ClosedDueToOctetsExceeded. Write <1> to clear. (802.1Qci – 8.6.5.1.2 (g))."]
        #[inline(always)]
        pub const fn set_cdoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Operational stream gate state (802.1Qci – 8.6.5.1.2 (b))."]
        #[must_use]
        #[inline(always)]
        pub const fn state(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Operational stream gate state (802.1Qci – 8.6.5.1.2 (b))."]
        #[inline(always)]
        pub const fn set_state(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Operational internal priority value specification (802.1Qci – 8.6.5.1.2 (c))."]
        #[must_use]
        #[inline(always)]
        pub const fn ipv(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "Operational internal priority value specification (802.1Qci – 8.6.5.1.2 (c))."]
        #[inline(always)]
        pub const fn set_ipv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
    }
    impl Default for CentralQciGstatus {
        #[inline(always)]
        fn default() -> CentralQciGstatus {
            CentralQciGstatus(0)
        }
    }
    impl core::fmt::Debug for CentralQciGstatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciGstatus")
                .field("cfgerr", &self.cfgerr())
                .field("cfgp", &self.cfgp())
                .field("cdir", &self.cdir())
                .field("cdoe", &self.cdoe())
                .field("state", &self.state())
                .field("ipv", &self.ipv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciGstatus {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CentralQciGstatus {{ cfgerr: {=bool:?}, cfgp: {=bool:?}, cdir: {=bool:?}, cdoe: {=bool:?}, state: {=bool:?}, ipv: {=u8:?} }}" , self . cfgerr () , self . cfgp () , self . cdir () , self . cdoe () , self . state () , self . ipv ())
        }
    }
    #[doc = "PSPF General CTRAL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciHwcfg(pub u32);
    impl CentralQciHwcfg {
        #[doc = "FTD – parameter."]
        #[must_use]
        #[inline(always)]
        pub const fn ftd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "FTD – parameter."]
        #[inline(always)]
        pub const fn set_ftd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "GTD – parameter."]
        #[must_use]
        #[inline(always)]
        pub const fn gtd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "GTD – parameter."]
        #[inline(always)]
        pub const fn set_gtd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "FMD – parameter."]
        #[must_use]
        #[inline(always)]
        pub const fn fmd(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "FMD – parameter."]
        #[inline(always)]
        pub const fn set_fmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for CentralQciHwcfg {
        #[inline(always)]
        fn default() -> CentralQciHwcfg {
            CentralQciHwcfg(0)
        }
    }
    impl core::fmt::Debug for CentralQciHwcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciHwcfg")
                .field("ftd", &self.ftd())
                .field("gtd", &self.gtd())
                .field("fmd", &self.fmd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciHwcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CentralQciHwcfg {{ ftd: {=u8:?}, gtd: {=u8:?}, fmd: {=u8:?} }}",
                self.ftd(),
                self.gtd(),
                self.fmd()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciListlen(pub u32);
    impl CentralQciListlen {
        #[doc = "Administrative list length."]
        #[must_use]
        #[inline(always)]
        pub const fn alen(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Administrative list length."]
        #[inline(always)]
        pub const fn set_alen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Operational list length."]
        #[must_use]
        #[inline(always)]
        pub const fn olen(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Operational list length."]
        #[inline(always)]
        pub const fn set_olen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for CentralQciListlen {
        #[inline(always)]
        fn default() -> CentralQciListlen {
            CentralQciListlen(0)
        }
    }
    impl core::fmt::Debug for CentralQciListlen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciListlen")
                .field("alen", &self.alen())
                .field("olen", &self.olen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciListlen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CentralQciListlen {{ alen: {=u8:?}, olen: {=u8:?} }}",
                self.alen(),
                self.olen()
            )
        }
    }
    #[doc = "Flow meter settings."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciMctrl(pub u32);
    impl CentralQciMctrl {
        #[doc = "Coupling flag (802.1Qci – 8.6.5.1.3 (f))."]
        #[must_use]
        #[inline(always)]
        pub const fn cf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Coupling flag (802.1Qci – 8.6.5.1.3 (f))."]
        #[inline(always)]
        pub const fn set_cf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Color mode – functionally unused (802.1Qci – 8.6.5.1.3 (g))."]
        #[must_use]
        #[inline(always)]
        pub const fn cm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Color mode – functionally unused (802.1Qci – 8.6.5.1.3 (g))."]
        #[inline(always)]
        pub const fn set_cm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DropOnYellow (802.1Qci – 8.6.5.1.3 (h))."]
        #[must_use]
        #[inline(always)]
        pub const fn doy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DropOnYellow (802.1Qci – 8.6.5.1.3 (h))."]
        #[inline(always)]
        pub const fn set_doy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MarkAllFramesRedEnable (802.1Qci – 8.6.5.1.3 (i))."]
        #[must_use]
        #[inline(always)]
        pub const fn mafren(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MarkAllFramesRedEnable (802.1Qci – 8.6.5.1.3 (i))."]
        #[inline(always)]
        pub const fn set_mafren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MarkAllFramesRed – cleared by RESET (802.1Qci – 8.6.5.1.3 (j))."]
        #[must_use]
        #[inline(always)]
        pub const fn mafr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MarkAllFramesRed – cleared by RESET (802.1Qci – 8.6.5.1.3 (j))."]
        #[inline(always)]
        pub const fn set_mafr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Flow Meter reset – self-resetting to <0>."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Flow Meter reset – self-resetting to <0>."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CentralQciMctrl {
        #[inline(always)]
        fn default() -> CentralQciMctrl {
            CentralQciMctrl(0)
        }
    }
    impl core::fmt::Debug for CentralQciMctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciMctrl")
                .field("cf", &self.cf())
                .field("cm", &self.cm())
                .field("doy", &self.doy())
                .field("mafren", &self.mafren())
                .field("mafr", &self.mafr())
                .field("reset", &self.reset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciMctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CentralQciMctrl {{ cf: {=bool:?}, cm: {=bool:?}, doy: {=bool:?}, mafren: {=bool:?}, mafr: {=bool:?}, reset: {=bool:?} }}" , self . cf () , self . cm () , self . doy () , self . mafren () , self . mafr () , self . reset ())
        }
    }
    #[doc = "Flowmeter select index."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CentralQciMetersel(pub u32);
    impl CentralQciMetersel {
        #[doc = "Flowmeter select index Any written value larger than the maximum index (2**FMD-1) will result in a read-back value of <0>."]
        #[must_use]
        #[inline(always)]
        pub const fn index(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Flowmeter select index Any written value larger than the maximum index (2**FMD-1) will result in a read-back value of <0>."]
        #[inline(always)]
        pub const fn set_index(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CentralQciMetersel {
        #[inline(always)]
        fn default() -> CentralQciMetersel {
            CentralQciMetersel(0)
        }
    }
    impl core::fmt::Debug for CentralQciMetersel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CentralQciMetersel")
                .field("index", &self.index())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CentralQciMetersel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CentralQciMetersel {{ index: {=u8:?} }}", self.index())
        }
    }
    #[doc = "Frame Replication and Elimination."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressFrerControl(pub u32);
    impl CpuPortEgressFrerControl {
        #[doc = "R-TAG encoding enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rtenc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "R-TAG encoding enable."]
        #[inline(always)]
        pub const fn set_rtenc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Latent error flag – write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn later(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Latent error flag – write 1 to clear."]
        #[inline(always)]
        pub const fn set_later(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for CpuPortEgressFrerControl {
        #[inline(always)]
        fn default() -> CpuPortEgressFrerControl {
            CpuPortEgressFrerControl(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressFrerControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressFrerControl")
                .field("rtenc", &self.rtenc())
                .field("later", &self.later())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressFrerControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressFrerControl {{ rtenc: {=bool:?}, later: {=bool:?} }}",
                self.rtenc(),
                self.later()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressFrerFctrl(pub u32);
    impl CpuPortEgressFrerFctrl {
        #[doc = "TakeNoSequence (802.1CB 10.4.1.9)."]
        #[must_use]
        #[inline(always)]
        pub const fn tns(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TakeNoSequence (802.1CB 10.4.1.9)."]
        #[inline(always)]
        pub const fn set_tns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Individual function (802.1CB 10.4.1.10)."]
        #[must_use]
        #[inline(always)]
        pub const fn ind(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Individual function (802.1CB 10.4.1.10)."]
        #[inline(always)]
        pub const fn set_ind(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Latent error detection enable."]
        #[must_use]
        #[inline(always)]
        pub const fn laten(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Latent error detection enable."]
        #[inline(always)]
        pub const fn set_laten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Recovery function algorithm: 0 – Vector recovery algorithm 1 – Match recovery algorithm."]
        #[must_use]
        #[inline(always)]
        pub const fn algo(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Recovery function algorithm: 0 – Vector recovery algorithm 1 – Match recovery algorithm."]
        #[inline(always)]
        pub const fn set_algo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "History length (used by Vector recovery algorithm)."]
        #[must_use]
        #[inline(always)]
        pub const fn hlen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "History length (used by Vector recovery algorithm)."]
        #[inline(always)]
        pub const fn set_hlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Number of paths (used by latent error detection)."]
        #[must_use]
        #[inline(always)]
        pub const fn paths(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Number of paths (used by latent error detection)."]
        #[inline(always)]
        pub const fn set_paths(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Reset recovery function – self-resetting to 0."]
        #[must_use]
        #[inline(always)]
        pub const fn frset(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Reset recovery function – self-resetting to 0."]
        #[inline(always)]
        pub const fn set_frset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CpuPortEgressFrerFctrl {
        #[inline(always)]
        fn default() -> CpuPortEgressFrerFctrl {
            CpuPortEgressFrerFctrl(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressFrerFctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressFrerFctrl")
                .field("tns", &self.tns())
                .field("ind", &self.ind())
                .field("laten", &self.laten())
                .field("algo", &self.algo())
                .field("hlen", &self.hlen())
                .field("paths", &self.paths())
                .field("frset", &self.frset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressFrerFctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CpuPortEgressFrerFctrl {{ tns: {=bool:?}, ind: {=bool:?}, laten: {=bool:?}, algo: {=bool:?}, hlen: {=u8:?}, paths: {=u8:?}, frset: {=bool:?} }}" , self . tns () , self . ind () , self . laten () , self . algo () , self . hlen () , self . paths () , self . frset ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressFrerFselect(pub u32);
    impl CpuPortEgressFrerFselect {
        #[doc = "Recovery function selection for host access at offset 0x140+."]
        #[must_use]
        #[inline(always)]
        pub const fn fidx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Recovery function selection for host access at offset 0x140+."]
        #[inline(always)]
        pub const fn set_fidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CpuPortEgressFrerFselect {
        #[inline(always)]
        fn default() -> CpuPortEgressFrerFselect {
            CpuPortEgressFrerFselect(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressFrerFselect {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressFrerFselect")
                .field("fidx", &self.fidx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressFrerFselect {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressFrerFselect {{ fidx: {=u8:?} }}",
                self.fidx()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressFrerIrfunc(pub u32);
    impl CpuPortEgressFrerIrfunc {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn fidx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_fidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Individual recovery function: FEN – enable function for stream SIDSEL.SID. FIDX – function index for stream SIDSEL.SID If function does not exists (FIDX >= 2**FD), FEN will be set to 0."]
        #[must_use]
        #[inline(always)]
        pub const fn fen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Individual recovery function: FEN – enable function for stream SIDSEL.SID. FIDX – function index for stream SIDSEL.SID If function does not exists (FIDX >= 2**FD), FEN will be set to 0."]
        #[inline(always)]
        pub const fn set_fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CpuPortEgressFrerIrfunc {
        #[inline(always)]
        fn default() -> CpuPortEgressFrerIrfunc {
            CpuPortEgressFrerIrfunc(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressFrerIrfunc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressFrerIrfunc")
                .field("fidx", &self.fidx())
                .field("fen", &self.fen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressFrerIrfunc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressFrerIrfunc {{ fidx: {=u8:?}, fen: {=bool:?} }}",
                self.fidx(),
                self.fen()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressFrerLatErrCnt(pub u32);
    impl CpuPortEgressFrerLatErrCnt {
        #[doc = "Counter – latent error detect. Write any value to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn laterr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Counter – latent error detect. Write any value to clear."]
        #[inline(always)]
        pub const fn set_laterr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortEgressFrerLatErrCnt {
        #[inline(always)]
        fn default() -> CpuPortEgressFrerLatErrCnt {
            CpuPortEgressFrerLatErrCnt(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressFrerLatErrCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressFrerLatErrCnt")
                .field("laterr", &self.laterr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressFrerLatErrCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressFrerLatErrCnt {{ laterr: {=u32:?} }}",
                self.laterr()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressFrerLatErrDiffAlw(pub u32);
    impl CpuPortEgressFrerLatErrDiffAlw {
        #[doc = "frerSeqRcvyLatentErrorDifference (802.1CB 10.4.1.12.1)."]
        #[must_use]
        #[inline(always)]
        pub const fn fdiff(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "frerSeqRcvyLatentErrorDifference (802.1CB 10.4.1.12.1)."]
        #[inline(always)]
        pub const fn set_fdiff(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortEgressFrerLatErrDiffAlw {
        #[inline(always)]
        fn default() -> CpuPortEgressFrerLatErrDiffAlw {
            CpuPortEgressFrerLatErrDiffAlw(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressFrerLatErrDiffAlw {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressFrerLatErrDiffAlw")
                .field("fdiff", &self.fdiff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressFrerLatErrDiffAlw {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressFrerLatErrDiffAlw {{ fdiff: {=u32:?} }}",
                self.fdiff()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressFrerLatRsPeriod(pub u32);
    impl CpuPortEgressFrerLatRsPeriod {
        #[doc = "frerSeqRcvyLatentResetPeriod (802.1CB 10.4.1.12.4)."]
        #[must_use]
        #[inline(always)]
        pub const fn flatr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "frerSeqRcvyLatentResetPeriod (802.1CB 10.4.1.12.4)."]
        #[inline(always)]
        pub const fn set_flatr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortEgressFrerLatRsPeriod {
        #[inline(always)]
        fn default() -> CpuPortEgressFrerLatRsPeriod {
            CpuPortEgressFrerLatRsPeriod(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressFrerLatRsPeriod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressFrerLatRsPeriod")
                .field("flatr", &self.flatr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressFrerLatRsPeriod {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressFrerLatRsPeriod {{ flatr: {=u32:?} }}",
                self.flatr()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressFrerLatTestPeriod(pub u32);
    impl CpuPortEgressFrerLatTestPeriod {
        #[doc = "frerSeqRcvyLatentErrorPeriod (802.1CB 10.4.1.12.2)."]
        #[must_use]
        #[inline(always)]
        pub const fn flatt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "frerSeqRcvyLatentErrorPeriod (802.1CB 10.4.1.12.2)."]
        #[inline(always)]
        pub const fn set_flatt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortEgressFrerLatTestPeriod {
        #[inline(always)]
        fn default() -> CpuPortEgressFrerLatTestPeriod {
            CpuPortEgressFrerLatTestPeriod(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressFrerLatTestPeriod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressFrerLatTestPeriod")
                .field("flatt", &self.flatt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressFrerLatTestPeriod {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressFrerLatTestPeriod {{ flatt: {=u32:?} }}",
                self.flatt()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressFrerResetmsec(pub u32);
    impl CpuPortEgressFrerResetmsec {
        #[doc = "frerSeqRcvyResetMSec (802.1CB 10.4.1.7)."]
        #[must_use]
        #[inline(always)]
        pub const fn fsrms(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "frerSeqRcvyResetMSec (802.1CB 10.4.1.7)."]
        #[inline(always)]
        pub const fn set_fsrms(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortEgressFrerResetmsec {
        #[inline(always)]
        fn default() -> CpuPortEgressFrerResetmsec {
            CpuPortEgressFrerResetmsec(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressFrerResetmsec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressFrerResetmsec")
                .field("fsrms", &self.fsrms())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressFrerResetmsec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressFrerResetmsec {{ fsrms: {=u32:?} }}",
                self.fsrms()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressFrerSidsel(pub u32);
    impl CpuPortEgressFrerSidsel {
        #[doc = "Stream ID selection for host access to IRFUNC and SRFUNC."]
        #[must_use]
        #[inline(always)]
        pub const fn sid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Stream ID selection for host access to IRFUNC and SRFUNC."]
        #[inline(always)]
        pub const fn set_sid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CpuPortEgressFrerSidsel {
        #[inline(always)]
        fn default() -> CpuPortEgressFrerSidsel {
            CpuPortEgressFrerSidsel(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressFrerSidsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressFrerSidsel")
                .field("sid", &self.sid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressFrerSidsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CpuPortEgressFrerSidsel {{ sid: {=u8:?} }}", self.sid())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressFrerSrfunc(pub u32);
    impl CpuPortEgressFrerSrfunc {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn fidx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_fidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Sequence recovery function: FEN – enable function for stream SIDSEL.SID. FIDX – function index for stream SIDSEL.SID If function does not exists (FIDX >= 2**FD), FEN will be set to 0."]
        #[must_use]
        #[inline(always)]
        pub const fn fen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Sequence recovery function: FEN – enable function for stream SIDSEL.SID. FIDX – function index for stream SIDSEL.SID If function does not exists (FIDX >= 2**FD), FEN will be set to 0."]
        #[inline(always)]
        pub const fn set_fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CpuPortEgressFrerSrfunc {
        #[inline(always)]
        fn default() -> CpuPortEgressFrerSrfunc {
            CpuPortEgressFrerSrfunc(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressFrerSrfunc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressFrerSrfunc")
                .field("fidx", &self.fidx())
                .field("fen", &self.fen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressFrerSrfunc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressFrerSrfunc {{ fidx: {=u8:?}, fen: {=bool:?} }}",
                self.fidx(),
                self.fen()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressStmidAmachi(pub u32);
    impl CpuPortEgressStmidAmachi {
        #[doc = "Active Destination MAC, MAC-Address \\[47:32\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn amach(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Active Destination MAC, MAC-Address \\[47:32\\]."]
        #[inline(always)]
        pub const fn set_amach(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Active Destination MAC, VLAN ID."]
        #[must_use]
        #[inline(always)]
        pub const fn avid(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Active Destination MAC, VLAN ID."]
        #[inline(always)]
        pub const fn set_avid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Active Destination MAC, PCP."]
        #[must_use]
        #[inline(always)]
        pub const fn apcp(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Active Destination MAC, PCP."]
        #[inline(always)]
        pub const fn set_apcp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for CpuPortEgressStmidAmachi {
        #[inline(always)]
        fn default() -> CpuPortEgressStmidAmachi {
            CpuPortEgressStmidAmachi(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressStmidAmachi {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressStmidAmachi")
                .field("amach", &self.amach())
                .field("avid", &self.avid())
                .field("apcp", &self.apcp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressStmidAmachi {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressStmidAmachi {{ amach: {=u16:?}, avid: {=u16:?}, apcp: {=u8:?} }}",
                self.amach(),
                self.avid(),
                self.apcp()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressStmidControl(pub u32);
    impl CpuPortEgressStmidControl {
        #[doc = "Enable entry."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable entry."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Lookup mode. 1:Priority – a frame must be untagged or priority tagged ; 2:Tagged – a frame must have a VLAN tag ; 3:All – a frame can be tagged or untagged."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Lookup mode. 1:Priority – a frame must be untagged or priority tagged ; 2:Tagged – a frame must have a VLAN tag ; 3:All – a frame can be tagged or untagged."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "0: Lookup by Destination MAC 1: Lookup by Source MAC."]
        #[must_use]
        #[inline(always)]
        pub const fn smac(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "0: Lookup by Destination MAC 1: Lookup by Source MAC."]
        #[inline(always)]
        pub const fn set_smac(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Active Destination MAC – control. See Table 6-6."]
        #[must_use]
        #[inline(always)]
        pub const fn actctl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Active Destination MAC – control. See Table 6-6."]
        #[inline(always)]
        pub const fn set_actctl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Sequence number generation enable."]
        #[must_use]
        #[inline(always)]
        pub const fn seqgen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Sequence number generation enable."]
        #[inline(always)]
        pub const fn set_seqgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Stream ID – inserted to header on match."]
        #[must_use]
        #[inline(always)]
        pub const fn sid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Stream ID – inserted to header on match."]
        #[inline(always)]
        pub const fn set_sid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for CpuPortEgressStmidControl {
        #[inline(always)]
        fn default() -> CpuPortEgressStmidControl {
            CpuPortEgressStmidControl(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressStmidControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressStmidControl")
                .field("en", &self.en())
                .field("mode", &self.mode())
                .field("smac", &self.smac())
                .field("actctl", &self.actctl())
                .field("seqgen", &self.seqgen())
                .field("sid", &self.sid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressStmidControl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CpuPortEgressStmidControl {{ en: {=bool:?}, mode: {=u8:?}, smac: {=bool:?}, actctl: {=u8:?}, seqgen: {=bool:?}, sid: {=u8:?} }}" , self . en () , self . mode () , self . smac () , self . actctl () , self . seqgen () , self . sid ())
        }
    }
    #[doc = "Stream Identification."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressStmidEselect(pub u32);
    impl CpuPortEgressStmidEselect {
        #[doc = "Select entry. Selected entry mapped to 0x40 – 0x5C."]
        #[must_use]
        #[inline(always)]
        pub const fn esel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Select entry. Selected entry mapped to 0x40 – 0x5C."]
        #[inline(always)]
        pub const fn set_esel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CpuPortEgressStmidEselect {
        #[inline(always)]
        fn default() -> CpuPortEgressStmidEselect {
            CpuPortEgressStmidEselect(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressStmidEselect {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressStmidEselect")
                .field("esel", &self.esel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressStmidEselect {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressStmidEselect {{ esel: {=u8:?} }}",
                self.esel()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressStmidMachi(pub u32);
    impl CpuPortEgressStmidMachi {
        #[doc = "MAC-Address \\[47:31\\]
used by lookup."]
        #[must_use]
        #[inline(always)]
        pub const fn match_(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC-Address \\[47:31\\]
used by lookup."]
        #[inline(always)]
        pub const fn set_match_(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "VLAN ID used by lookup."]
        #[must_use]
        #[inline(always)]
        pub const fn vid(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "VLAN ID used by lookup."]
        #[inline(always)]
        pub const fn set_vid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for CpuPortEgressStmidMachi {
        #[inline(always)]
        fn default() -> CpuPortEgressStmidMachi {
            CpuPortEgressStmidMachi(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressStmidMachi {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressStmidMachi")
                .field("match_", &self.match_())
                .field("vid", &self.vid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressStmidMachi {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressStmidMachi {{ match_: {=u16:?}, vid: {=u16:?} }}",
                self.match_(),
                self.vid()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressStmidMaclo(pub u32);
    impl CpuPortEgressStmidMaclo {
        #[doc = "MAC-Address \\[31:0\\]
used by lookup."]
        #[must_use]
        #[inline(always)]
        pub const fn macl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC-Address \\[31:0\\]
used by lookup."]
        #[inline(always)]
        pub const fn set_macl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortEgressStmidMaclo {
        #[inline(always)]
        fn default() -> CpuPortEgressStmidMaclo {
            CpuPortEgressStmidMaclo(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressStmidMaclo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressStmidMaclo")
                .field("macl", &self.macl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressStmidMaclo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressStmidMaclo {{ macl: {=u32:?} }}",
                self.macl()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressStmidMatchcnt(pub u32);
    impl CpuPortEgressStmidMatchcnt {
        #[doc = "Entry match counter – any write access to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn match_(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Entry match counter – any write access to clear."]
        #[inline(always)]
        pub const fn set_match_(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortEgressStmidMatchcnt {
        #[inline(always)]
        fn default() -> CpuPortEgressStmidMatchcnt {
            CpuPortEgressStmidMatchcnt(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressStmidMatchcnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressStmidMatchcnt")
                .field("match_", &self.match_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressStmidMatchcnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressStmidMatchcnt {{ match_: {=u32:?} }}",
                self.match_()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortEgressStmidSeqno(pub u32);
    impl CpuPortEgressStmidSeqno {
        #[doc = "Sequence number – next number when generating,any write access to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn seqno(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Sequence number – next number when generating,any write access to clear."]
        #[inline(always)]
        pub const fn set_seqno(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CpuPortEgressStmidSeqno {
        #[inline(always)]
        fn default() -> CpuPortEgressStmidSeqno {
            CpuPortEgressStmidSeqno(0)
        }
    }
    impl core::fmt::Debug for CpuPortEgressStmidSeqno {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortEgressStmidSeqno")
                .field("seqno", &self.seqno())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortEgressStmidSeqno {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortEgressStmidSeqno {{ seqno: {=u16:?} }}",
                self.seqno()
            )
        }
    }
    #[doc = "Frame Replication and Elimination."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressFrerControl(pub u32);
    impl CpuPortIgressFrerControl {
        #[doc = "R-TAG encoding enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rtenc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "R-TAG encoding enable."]
        #[inline(always)]
        pub const fn set_rtenc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Latent error flag – write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn later(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Latent error flag – write 1 to clear."]
        #[inline(always)]
        pub const fn set_later(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for CpuPortIgressFrerControl {
        #[inline(always)]
        fn default() -> CpuPortIgressFrerControl {
            CpuPortIgressFrerControl(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressFrerControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressFrerControl")
                .field("rtenc", &self.rtenc())
                .field("later", &self.later())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressFrerControl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressFrerControl {{ rtenc: {=bool:?}, later: {=bool:?} }}",
                self.rtenc(),
                self.later()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressFrerFctrl(pub u32);
    impl CpuPortIgressFrerFctrl {
        #[doc = "TakeNoSequence (802.1CB 10.4.1.9)."]
        #[must_use]
        #[inline(always)]
        pub const fn tns(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TakeNoSequence (802.1CB 10.4.1.9)."]
        #[inline(always)]
        pub const fn set_tns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Individual function (802.1CB 10.4.1.10)."]
        #[must_use]
        #[inline(always)]
        pub const fn ind(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Individual function (802.1CB 10.4.1.10)."]
        #[inline(always)]
        pub const fn set_ind(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Latent error detection enable."]
        #[must_use]
        #[inline(always)]
        pub const fn laten(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Latent error detection enable."]
        #[inline(always)]
        pub const fn set_laten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Recovery function algorithm: 0 – Vector recovery algorithm 1 – Match recovery algorithm."]
        #[must_use]
        #[inline(always)]
        pub const fn algo(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Recovery function algorithm: 0 – Vector recovery algorithm 1 – Match recovery algorithm."]
        #[inline(always)]
        pub const fn set_algo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "History length (used by Vector recovery algorithm)."]
        #[must_use]
        #[inline(always)]
        pub const fn hlen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "History length (used by Vector recovery algorithm)."]
        #[inline(always)]
        pub const fn set_hlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Number of paths (used by latent error detection)."]
        #[must_use]
        #[inline(always)]
        pub const fn paths(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Number of paths (used by latent error detection)."]
        #[inline(always)]
        pub const fn set_paths(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Reset recovery function – self-resetting to 0."]
        #[must_use]
        #[inline(always)]
        pub const fn frset(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Reset recovery function – self-resetting to 0."]
        #[inline(always)]
        pub const fn set_frset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CpuPortIgressFrerFctrl {
        #[inline(always)]
        fn default() -> CpuPortIgressFrerFctrl {
            CpuPortIgressFrerFctrl(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressFrerFctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressFrerFctrl")
                .field("tns", &self.tns())
                .field("ind", &self.ind())
                .field("laten", &self.laten())
                .field("algo", &self.algo())
                .field("hlen", &self.hlen())
                .field("paths", &self.paths())
                .field("frset", &self.frset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressFrerFctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CpuPortIgressFrerFctrl {{ tns: {=bool:?}, ind: {=bool:?}, laten: {=bool:?}, algo: {=bool:?}, hlen: {=u8:?}, paths: {=u8:?}, frset: {=bool:?} }}" , self . tns () , self . ind () , self . laten () , self . algo () , self . hlen () , self . paths () , self . frset ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressFrerFselect(pub u32);
    impl CpuPortIgressFrerFselect {
        #[doc = "Recovery function selection for host access at offset 0x140+."]
        #[must_use]
        #[inline(always)]
        pub const fn fidx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Recovery function selection for host access at offset 0x140+."]
        #[inline(always)]
        pub const fn set_fidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CpuPortIgressFrerFselect {
        #[inline(always)]
        fn default() -> CpuPortIgressFrerFselect {
            CpuPortIgressFrerFselect(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressFrerFselect {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressFrerFselect")
                .field("fidx", &self.fidx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressFrerFselect {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressFrerFselect {{ fidx: {=u8:?} }}",
                self.fidx()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressFrerIrfunc(pub u32);
    impl CpuPortIgressFrerIrfunc {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn fidx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_fidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Individual recovery function: FEN – enable function for stream SIDSEL.SID. FIDX – function index for stream SIDSEL.SID If function does not exists (FIDX >= 2**FD), FEN will be set to 0."]
        #[must_use]
        #[inline(always)]
        pub const fn fen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Individual recovery function: FEN – enable function for stream SIDSEL.SID. FIDX – function index for stream SIDSEL.SID If function does not exists (FIDX >= 2**FD), FEN will be set to 0."]
        #[inline(always)]
        pub const fn set_fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CpuPortIgressFrerIrfunc {
        #[inline(always)]
        fn default() -> CpuPortIgressFrerIrfunc {
            CpuPortIgressFrerIrfunc(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressFrerIrfunc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressFrerIrfunc")
                .field("fidx", &self.fidx())
                .field("fen", &self.fen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressFrerIrfunc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressFrerIrfunc {{ fidx: {=u8:?}, fen: {=bool:?} }}",
                self.fidx(),
                self.fen()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressFrerLatErrCnt(pub u32);
    impl CpuPortIgressFrerLatErrCnt {
        #[doc = "Counter – latent error detect. Write any value to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn laterr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Counter – latent error detect. Write any value to clear."]
        #[inline(always)]
        pub const fn set_laterr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortIgressFrerLatErrCnt {
        #[inline(always)]
        fn default() -> CpuPortIgressFrerLatErrCnt {
            CpuPortIgressFrerLatErrCnt(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressFrerLatErrCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressFrerLatErrCnt")
                .field("laterr", &self.laterr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressFrerLatErrCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressFrerLatErrCnt {{ laterr: {=u32:?} }}",
                self.laterr()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressFrerLatErrDiffAlw(pub u32);
    impl CpuPortIgressFrerLatErrDiffAlw {
        #[doc = "frerSeqRcvyLatentErrorDifference (802.1CB 10.4.1.12.1)."]
        #[must_use]
        #[inline(always)]
        pub const fn fdiff(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "frerSeqRcvyLatentErrorDifference (802.1CB 10.4.1.12.1)."]
        #[inline(always)]
        pub const fn set_fdiff(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortIgressFrerLatErrDiffAlw {
        #[inline(always)]
        fn default() -> CpuPortIgressFrerLatErrDiffAlw {
            CpuPortIgressFrerLatErrDiffAlw(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressFrerLatErrDiffAlw {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressFrerLatErrDiffAlw")
                .field("fdiff", &self.fdiff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressFrerLatErrDiffAlw {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressFrerLatErrDiffAlw {{ fdiff: {=u32:?} }}",
                self.fdiff()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressFrerLatRsPeriod(pub u32);
    impl CpuPortIgressFrerLatRsPeriod {
        #[doc = "frerSeqRcvyLatentResetPeriod (802.1CB 10.4.1.12.4)."]
        #[must_use]
        #[inline(always)]
        pub const fn flatr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "frerSeqRcvyLatentResetPeriod (802.1CB 10.4.1.12.4)."]
        #[inline(always)]
        pub const fn set_flatr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortIgressFrerLatRsPeriod {
        #[inline(always)]
        fn default() -> CpuPortIgressFrerLatRsPeriod {
            CpuPortIgressFrerLatRsPeriod(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressFrerLatRsPeriod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressFrerLatRsPeriod")
                .field("flatr", &self.flatr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressFrerLatRsPeriod {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressFrerLatRsPeriod {{ flatr: {=u32:?} }}",
                self.flatr()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressFrerLatTestPeriod(pub u32);
    impl CpuPortIgressFrerLatTestPeriod {
        #[doc = "frerSeqRcvyLatentErrorPeriod (802.1CB 10.4.1.12.2)."]
        #[must_use]
        #[inline(always)]
        pub const fn flatt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "frerSeqRcvyLatentErrorPeriod (802.1CB 10.4.1.12.2)."]
        #[inline(always)]
        pub const fn set_flatt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortIgressFrerLatTestPeriod {
        #[inline(always)]
        fn default() -> CpuPortIgressFrerLatTestPeriod {
            CpuPortIgressFrerLatTestPeriod(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressFrerLatTestPeriod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressFrerLatTestPeriod")
                .field("flatt", &self.flatt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressFrerLatTestPeriod {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressFrerLatTestPeriod {{ flatt: {=u32:?} }}",
                self.flatt()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressFrerResetmsec(pub u32);
    impl CpuPortIgressFrerResetmsec {
        #[doc = "frerSeqRcvyResetMSec (802.1CB 10.4.1.7)."]
        #[must_use]
        #[inline(always)]
        pub const fn fsrms(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "frerSeqRcvyResetMSec (802.1CB 10.4.1.7)."]
        #[inline(always)]
        pub const fn set_fsrms(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortIgressFrerResetmsec {
        #[inline(always)]
        fn default() -> CpuPortIgressFrerResetmsec {
            CpuPortIgressFrerResetmsec(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressFrerResetmsec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressFrerResetmsec")
                .field("fsrms", &self.fsrms())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressFrerResetmsec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressFrerResetmsec {{ fsrms: {=u32:?} }}",
                self.fsrms()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressFrerSidsel(pub u32);
    impl CpuPortIgressFrerSidsel {
        #[doc = "Stream ID selection for host access to IRFUNC and SRFUNC."]
        #[must_use]
        #[inline(always)]
        pub const fn sid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Stream ID selection for host access to IRFUNC and SRFUNC."]
        #[inline(always)]
        pub const fn set_sid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CpuPortIgressFrerSidsel {
        #[inline(always)]
        fn default() -> CpuPortIgressFrerSidsel {
            CpuPortIgressFrerSidsel(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressFrerSidsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressFrerSidsel")
                .field("sid", &self.sid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressFrerSidsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CpuPortIgressFrerSidsel {{ sid: {=u8:?} }}", self.sid())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressFrerSrfunc(pub u32);
    impl CpuPortIgressFrerSrfunc {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn fidx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_fidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Sequence recovery function: FEN – enable function for stream SIDSEL.SID. FIDX – function index for stream SIDSEL.SID If function does not exists (FIDX >= 2**FD), FEN will be set to 0."]
        #[must_use]
        #[inline(always)]
        pub const fn fen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Sequence recovery function: FEN – enable function for stream SIDSEL.SID. FIDX – function index for stream SIDSEL.SID If function does not exists (FIDX >= 2**FD), FEN will be set to 0."]
        #[inline(always)]
        pub const fn set_fen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CpuPortIgressFrerSrfunc {
        #[inline(always)]
        fn default() -> CpuPortIgressFrerSrfunc {
            CpuPortIgressFrerSrfunc(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressFrerSrfunc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressFrerSrfunc")
                .field("fidx", &self.fidx())
                .field("fen", &self.fen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressFrerSrfunc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressFrerSrfunc {{ fidx: {=u8:?}, fen: {=bool:?} }}",
                self.fidx(),
                self.fen()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressRxFdfifoErrorFlag(pub u32);
    impl CpuPortIgressRxFdfifoErrorFlag {
        #[doc = "FD FIFO failure. Internal controller lost synchronization."]
        #[must_use]
        #[inline(always)]
        pub const fn desc_seq_err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO failure. Internal controller lost synchronization."]
        #[inline(always)]
        pub const fn set_desc_seq_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FD FIFO failure. Descriptor not received correctly."]
        #[must_use]
        #[inline(always)]
        pub const fn desc_nrdy_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO failure. Descriptor not received correctly."]
        #[inline(always)]
        pub const fn set_desc_nrdy_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Frame was dropped because the FIFO is full. Full by too much data."]
        #[must_use]
        #[inline(always)]
        pub const fn drop_full_mem(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Frame was dropped because the FIFO is full. Full by too much data."]
        #[inline(always)]
        pub const fn set_drop_full_mem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Frame was dropped because the internal descriptor FIFO is full. Full by too many frames."]
        #[must_use]
        #[inline(always)]
        pub const fn drop_full_desc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Frame was dropped because the internal descriptor FIFO is full. Full by too many frames."]
        #[inline(always)]
        pub const fn set_drop_full_desc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Frame was dropped because the FIFO was not ready. That can typically happen after a reset of the FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn drop_nrdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Frame was dropped because the FIFO was not ready. That can typically happen after a reset of the FIFO."]
        #[inline(always)]
        pub const fn set_drop_nrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Set if a frame is partially written into FIFO which had insufficient space. The frame is cut and frame error is set."]
        #[must_use]
        #[inline(always)]
        pub const fn wrfail_full(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Set if a frame is partially written into FIFO which had insufficient space. The frame is cut and frame error is set."]
        #[inline(always)]
        pub const fn set_wrfail_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LookUp Descriptor lost, because of unknown frame burst by MAC. If there is no MAC mailfunction then this flag will never be raised. FDFIFO requires reset."]
        #[must_use]
        #[inline(always)]
        pub const fn lu_desc_err(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LookUp Descriptor lost, because of unknown frame burst by MAC. If there is no MAC mailfunction then this flag will never be raised. FDFIFO requires reset."]
        #[inline(always)]
        pub const fn set_lu_desc_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for CpuPortIgressRxFdfifoErrorFlag {
        #[inline(always)]
        fn default() -> CpuPortIgressRxFdfifoErrorFlag {
            CpuPortIgressRxFdfifoErrorFlag(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressRxFdfifoErrorFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressRxFdfifoErrorFlag")
                .field("desc_seq_err", &self.desc_seq_err())
                .field("desc_nrdy_err", &self.desc_nrdy_err())
                .field("drop_full_mem", &self.drop_full_mem())
                .field("drop_full_desc", &self.drop_full_desc())
                .field("drop_nrdy", &self.drop_nrdy())
                .field("wrfail_full", &self.wrfail_full())
                .field("lu_desc_err", &self.lu_desc_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressRxFdfifoErrorFlag {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CpuPortIgressRxFdfifoErrorFlag {{ desc_seq_err: {=bool:?}, desc_nrdy_err: {=bool:?}, drop_full_mem: {=bool:?}, drop_full_desc: {=bool:?}, drop_nrdy: {=bool:?}, wrfail_full: {=bool:?}, lu_desc_err: {=bool:?} }}" , self . desc_seq_err () , self . desc_nrdy_err () , self . drop_full_mem () , self . drop_full_desc () , self . drop_nrdy () , self . wrfail_full () , self . lu_desc_err ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressRxFdfifoFdmemCntByte(pub u32);
    impl CpuPortIgressRxFdfifoFdmemCntByte {
        #[doc = "Number of bytes stored in frame drop FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn fdmem_cnt_byte(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of bytes stored in frame drop FIFO."]
        #[inline(always)]
        pub const fn set_fdmem_cnt_byte(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortIgressRxFdfifoFdmemCntByte {
        #[inline(always)]
        fn default() -> CpuPortIgressRxFdfifoFdmemCntByte {
            CpuPortIgressRxFdfifoFdmemCntByte(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressRxFdfifoFdmemCntByte {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressRxFdfifoFdmemCntByte")
                .field("fdmem_cnt_byte", &self.fdmem_cnt_byte())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressRxFdfifoFdmemCntByte {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressRxFdfifoFdmemCntByte {{ fdmem_cnt_byte: {=u32:?} }}",
                self.fdmem_cnt_byte()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressRxFdfifoFdmemSts(pub u32);
    impl CpuPortIgressRxFdfifoFdmemSts {
        #[doc = "FD FIFO empty."]
        #[must_use]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO empty."]
        #[inline(always)]
        pub const fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FD FIFO almost empty. Few bytes in FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn amst_empty(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO almost empty. Few bytes in FIFO."]
        #[inline(always)]
        pub const fn set_amst_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FD FIFO almost full. Less than 1600 Byte left."]
        #[must_use]
        #[inline(always)]
        pub const fn amst_full(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO almost full. Less than 1600 Byte left."]
        #[inline(always)]
        pub const fn set_amst_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FD FIFO full."]
        #[must_use]
        #[inline(always)]
        pub const fn full(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO full."]
        #[inline(always)]
        pub const fn set_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FD FIFO ready to work or working."]
        #[must_use]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO ready to work or working."]
        #[inline(always)]
        pub const fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FD FIFO processes data."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO processes data."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FD FIFO waits for more frame data."]
        #[must_use]
        #[inline(always)]
        pub const fn wait_for_frame(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO waits for more frame data."]
        #[inline(always)]
        pub const fn set_wait_for_frame(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "FD FIFO waits for LookUp information."]
        #[must_use]
        #[inline(always)]
        pub const fn wait_for_lu(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO waits for LookUp information."]
        #[inline(always)]
        pub const fn set_wait_for_lu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for CpuPortIgressRxFdfifoFdmemSts {
        #[inline(always)]
        fn default() -> CpuPortIgressRxFdfifoFdmemSts {
            CpuPortIgressRxFdfifoFdmemSts(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressRxFdfifoFdmemSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressRxFdfifoFdmemSts")
                .field("empty", &self.empty())
                .field("amst_empty", &self.amst_empty())
                .field("amst_full", &self.amst_full())
                .field("full", &self.full())
                .field("ready", &self.ready())
                .field("busy", &self.busy())
                .field("wait_for_frame", &self.wait_for_frame())
                .field("wait_for_lu", &self.wait_for_lu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressRxFdfifoFdmemSts {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CpuPortIgressRxFdfifoFdmemSts {{ empty: {=bool:?}, amst_empty: {=bool:?}, amst_full: {=bool:?}, full: {=bool:?}, ready: {=bool:?}, busy: {=bool:?}, wait_for_frame: {=bool:?}, wait_for_lu: {=bool:?} }}" , self . empty () , self . amst_empty () , self . amst_full () , self . full () , self . ready () , self . busy () , self . wait_for_frame () , self . wait_for_lu ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressRxFdfifoIeErrorFlag(pub u32);
    impl CpuPortIgressRxFdfifoIeErrorFlag {
        #[doc = "Interrupt enable of ERROR_FLAG."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Interrupt enable of ERROR_FLAG."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for CpuPortIgressRxFdfifoIeErrorFlag {
        #[inline(always)]
        fn default() -> CpuPortIgressRxFdfifoIeErrorFlag {
            CpuPortIgressRxFdfifoIeErrorFlag(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressRxFdfifoIeErrorFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressRxFdfifoIeErrorFlag")
                .field("ie", &self.ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressRxFdfifoIeErrorFlag {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressRxFdfifoIeErrorFlag {{ ie: {=u8:?} }}",
                self.ie()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressRxFdfifoInConfig(pub u32);
    impl CpuPortIgressRxFdfifoInConfig {
        #[doc = "FD_FIFO does not shorten frames which contain an error."]
        #[must_use]
        #[inline(always)]
        pub const fn nocut_error(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FD_FIFO does not shorten frames which contain an error."]
        #[inline(always)]
        pub const fn set_nocut_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for CpuPortIgressRxFdfifoInConfig {
        #[inline(always)]
        fn default() -> CpuPortIgressRxFdfifoInConfig {
            CpuPortIgressRxFdfifoInConfig(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressRxFdfifoInConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressRxFdfifoInConfig")
                .field("nocut_error", &self.nocut_error())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressRxFdfifoInConfig {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressRxFdfifoInConfig {{ nocut_error: {=bool:?} }}",
                self.nocut_error()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressRxFdfifoMirror(pub u32);
    impl CpuPortIgressRxFdfifoMirror {
        #[doc = "Mirror Port. If port mirroring is enabled TX/RX traffic will also be forwarded to this port. bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Mirror Port. If port mirroring is enabled TX/RX traffic will also be forwarded to this port. bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortIgressRxFdfifoMirror {
        #[inline(always)]
        fn default() -> CpuPortIgressRxFdfifoMirror {
            CpuPortIgressRxFdfifoMirror(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressRxFdfifoMirror {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressRxFdfifoMirror")
                .field("port", &self.port())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressRxFdfifoMirror {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressRxFdfifoMirror {{ port: {=u32:?} }}",
                self.port()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressRxFdfifoMirrorTx(pub u32);
    impl CpuPortIgressRxFdfifoMirrorTx {
        #[doc = "Mirror Selection TX. The destination of the frame is compared with this vector. All matching TX probe ports will be mirrored to MIRROR. It is necessary to configure all ingress ports to mirror the complete TX traffic. bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Mirror Selection TX. The destination of the frame is compared with this vector. All matching TX probe ports will be mirrored to MIRROR. It is necessary to configure all ingress ports to mirror the complete TX traffic. bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortIgressRxFdfifoMirrorTx {
        #[inline(always)]
        fn default() -> CpuPortIgressRxFdfifoMirrorTx {
            CpuPortIgressRxFdfifoMirrorTx(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressRxFdfifoMirrorTx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressRxFdfifoMirrorTx")
                .field("port", &self.port())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressRxFdfifoMirrorTx {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressRxFdfifoMirrorTx {{ port: {=u32:?} }}",
                self.port()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressRxFdfifoOutConfig(pub u32);
    impl CpuPortIgressRxFdfifoOutConfig {
        #[doc = "Switch between Cut-Through and Store&Forward mode. 0 - Cut-Through 1 - Store&Forward."]
        #[must_use]
        #[inline(always)]
        pub const fn mode_store_fw(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Switch between Cut-Through and Store&Forward mode. 0 - Cut-Through 1 - Store&Forward."]
        #[inline(always)]
        pub const fn set_mode_store_fw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Do not drop frame errors."]
        #[must_use]
        #[inline(always)]
        pub const fn nodrop_error(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Do not drop frame errors."]
        #[inline(always)]
        pub const fn set_nodrop_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Duplicate frames to CPU."]
        #[must_use]
        #[inline(always)]
        pub const fn mirror_to_cpu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Duplicate frames to CPU."]
        #[inline(always)]
        pub const fn set_mirror_to_cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Send error frames to CPU."]
        #[must_use]
        #[inline(always)]
        pub const fn error_to_cpu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Send error frames to CPU."]
        #[inline(always)]
        pub const fn set_error_to_cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Route all frames to DROP_DEST."]
        #[must_use]
        #[inline(always)]
        pub const fn drop_all(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Route all frames to DROP_DEST."]
        #[inline(always)]
        pub const fn set_drop_all(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Disable input of FD FIFO. Take care that also descriptor generation of LookUp is disabled. Remaining frames should be cleared with DROP_ALL."]
        #[must_use]
        #[inline(always)]
        pub const fn disable(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Disable input of FD FIFO. Take care that also descriptor generation of LookUp is disabled. Remaining frames should be cleared with DROP_ALL."]
        #[inline(always)]
        pub const fn set_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "If any Store&Forward option in RX_FDFIFO is set then this flag will still force preemptable traffic to be forwarded in Cut-Through mode. This is a useful option to save latency by double buffering if the used MAC/TSN-EP already does S&F."]
        #[must_use]
        #[inline(always)]
        pub const fn ct_fpe_ovrd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If any Store&Forward option in RX_FDFIFO is set then this flag will still force preemptable traffic to be forwarded in Cut-Through mode. This is a useful option to save latency by double buffering if the used MAC/TSN-EP already does S&F."]
        #[inline(always)]
        pub const fn set_ct_fpe_ovrd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Incoming frames of this port will be mirrored to the given destination in MIRROR_RX."]
        #[must_use]
        #[inline(always)]
        pub const fn mirror_rx_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Incoming frames of this port will be mirrored to the given destination in MIRROR_RX."]
        #[inline(always)]
        pub const fn set_mirror_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Incoming frames of this port will be mirrored to the given destination in MIRROR if their destination match with MIRROR_TX."]
        #[must_use]
        #[inline(always)]
        pub const fn mirror_tx_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Incoming frames of this port will be mirrored to the given destination in MIRROR if their destination match with MIRROR_TX."]
        #[inline(always)]
        pub const fn set_mirror_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Bit mapped Destination for dropped frames. Typically, frames are cleared at destination 0. Use another value to stream frames for analysis. Supports only max range of port\\[15:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn drop_dest(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Bit mapped Destination for dropped frames. Typically, frames are cleared at destination 0. Use another value to stream frames for analysis. Supports only max range of port\\[15:0\\]."]
        #[inline(always)]
        pub const fn set_drop_dest(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for CpuPortIgressRxFdfifoOutConfig {
        #[inline(always)]
        fn default() -> CpuPortIgressRxFdfifoOutConfig {
            CpuPortIgressRxFdfifoOutConfig(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressRxFdfifoOutConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressRxFdfifoOutConfig")
                .field("mode_store_fw", &self.mode_store_fw())
                .field("nodrop_error", &self.nodrop_error())
                .field("mirror_to_cpu", &self.mirror_to_cpu())
                .field("error_to_cpu", &self.error_to_cpu())
                .field("drop_all", &self.drop_all())
                .field("disable", &self.disable())
                .field("ct_fpe_ovrd", &self.ct_fpe_ovrd())
                .field("mirror_rx_en", &self.mirror_rx_en())
                .field("mirror_tx_en", &self.mirror_tx_en())
                .field("drop_dest", &self.drop_dest())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressRxFdfifoOutConfig {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CpuPortIgressRxFdfifoOutConfig {{ mode_store_fw: {=bool:?}, nodrop_error: {=bool:?}, mirror_to_cpu: {=bool:?}, error_to_cpu: {=bool:?}, drop_all: {=bool:?}, disable: {=bool:?}, ct_fpe_ovrd: {=bool:?}, mirror_rx_en: {=bool:?}, mirror_tx_en: {=bool:?}, drop_dest: {=u16:?} }}" , self . mode_store_fw () , self . nodrop_error () , self . mirror_to_cpu () , self . error_to_cpu () , self . drop_all () , self . disable () , self . ct_fpe_ovrd () , self . mirror_rx_en () , self . mirror_tx_en () , self . drop_dest ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressRxFdfifoParam(pub u32);
    impl CpuPortIgressRxFdfifoParam {
        #[doc = "Number of words (4byte) the Frame Drop FIFO can store."]
        #[must_use]
        #[inline(always)]
        pub const fn fd_fifo_desc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of words (4byte) the Frame Drop FIFO can store."]
        #[inline(always)]
        pub const fn set_fd_fifo_desc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Number of FD descriptors the FIFO can store. Two descriptors need to be stored per frame."]
        #[must_use]
        #[inline(always)]
        pub const fn fd_desc_fifo_desc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Number of FD descriptors the FIFO can store. Two descriptors need to be stored per frame."]
        #[inline(always)]
        pub const fn set_fd_desc_fifo_desc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Number of MAC lookup descriptors the FIFO can store."]
        #[must_use]
        #[inline(always)]
        pub const fn lu_fifo_depth(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Number of MAC lookup descriptors the FIFO can store."]
        #[inline(always)]
        pub const fn set_lu_fifo_depth(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for CpuPortIgressRxFdfifoParam {
        #[inline(always)]
        fn default() -> CpuPortIgressRxFdfifoParam {
            CpuPortIgressRxFdfifoParam(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressRxFdfifoParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressRxFdfifoParam")
                .field("fd_fifo_desc", &self.fd_fifo_desc())
                .field("fd_desc_fifo_desc", &self.fd_desc_fifo_desc())
                .field("lu_fifo_depth", &self.lu_fifo_depth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressRxFdfifoParam {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CpuPortIgressRxFdfifoParam {{ fd_fifo_desc: {=u16:?}, fd_desc_fifo_desc: {=u8:?}, lu_fifo_depth: {=u8:?} }}" , self . fd_fifo_desc () , self . fd_desc_fifo_desc () , self . lu_fifo_depth ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressRxFdfifoPortmask(pub u32);
    impl CpuPortIgressRxFdfifoPortmask {
        #[doc = "Port grouping via port mask. If the selected port is not set then the destination will be filtered out. This register allows the realization of port-based-VLAN (no VLAN tags required, only set it by ports). bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Port grouping via port mask. If the selected port is not set then the destination will be filtered out. This register allows the realization of port-based-VLAN (no VLAN tags required, only set it by ports). bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortIgressRxFdfifoPortmask {
        #[inline(always)]
        fn default() -> CpuPortIgressRxFdfifoPortmask {
            CpuPortIgressRxFdfifoPortmask(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressRxFdfifoPortmask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressRxFdfifoPortmask")
                .field("port", &self.port())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressRxFdfifoPortmask {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressRxFdfifoPortmask {{ port: {=u32:?} }}",
                self.port()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressRxFdfifoReset(pub u32);
    impl CpuPortIgressRxFdfifoReset {
        #[doc = "Write 1 to reset FD controller and memory pointers. Register Map content remains untouched."]
        #[must_use]
        #[inline(always)]
        pub const fn softrs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 to reset FD controller and memory pointers. Register Map content remains untouched."]
        #[inline(always)]
        pub const fn set_softrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for CpuPortIgressRxFdfifoReset {
        #[inline(always)]
        fn default() -> CpuPortIgressRxFdfifoReset {
            CpuPortIgressRxFdfifoReset(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressRxFdfifoReset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressRxFdfifoReset")
                .field("softrs", &self.softrs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressRxFdfifoReset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressRxFdfifoReset {{ softrs: {=bool:?} }}",
                self.softrs()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressRxFdfifoStrfwd(pub u32);
    impl CpuPortIgressRxFdfifoStrfwd {
        #[doc = "If selected port is set then the frame is transmitted in Store & Forward mode. This is necessary when the ingress rate of this port is slower than the egress rate of the transmitting port. In S&F, the ingress module is able to drop frames with bad CRC.bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "If selected port is set then the frame is transmitted in Store & Forward mode. This is necessary when the ingress rate of this port is slower than the egress rate of the transmitting port. In S&F, the ingress module is able to drop frames with bad CRC.bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortIgressRxFdfifoStrfwd {
        #[inline(always)]
        fn default() -> CpuPortIgressRxFdfifoStrfwd {
            CpuPortIgressRxFdfifoStrfwd(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressRxFdfifoStrfwd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressRxFdfifoStrfwd")
                .field("port", &self.port())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressRxFdfifoStrfwd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressRxFdfifoStrfwd {{ port: {=u32:?} }}",
                self.port()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressStmidAmachi(pub u32);
    impl CpuPortIgressStmidAmachi {
        #[doc = "Active Destination MAC, MAC-Address \\[47:32\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn amach(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Active Destination MAC, MAC-Address \\[47:32\\]."]
        #[inline(always)]
        pub const fn set_amach(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Active Destination MAC, VLAN ID."]
        #[must_use]
        #[inline(always)]
        pub const fn avid(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Active Destination MAC, VLAN ID."]
        #[inline(always)]
        pub const fn set_avid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[doc = "Active Destination MAC, PCP."]
        #[must_use]
        #[inline(always)]
        pub const fn apcp(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Active Destination MAC, PCP."]
        #[inline(always)]
        pub const fn set_apcp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for CpuPortIgressStmidAmachi {
        #[inline(always)]
        fn default() -> CpuPortIgressStmidAmachi {
            CpuPortIgressStmidAmachi(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressStmidAmachi {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressStmidAmachi")
                .field("amach", &self.amach())
                .field("avid", &self.avid())
                .field("apcp", &self.apcp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressStmidAmachi {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressStmidAmachi {{ amach: {=u16:?}, avid: {=u16:?}, apcp: {=u8:?} }}",
                self.amach(),
                self.avid(),
                self.apcp()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressStmidControl(pub u32);
    impl CpuPortIgressStmidControl {
        #[doc = "Enable entry."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable entry."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Lookup mode. 1:Priority – a frame must be untagged or priority tagged ; 2:Tagged – a frame must have a VLAN tag ; 3:All – a frame can be tagged or untagged."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Lookup mode. 1:Priority – a frame must be untagged or priority tagged ; 2:Tagged – a frame must have a VLAN tag ; 3:All – a frame can be tagged or untagged."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "0: Lookup by Destination MAC 1: Lookup by Source MAC."]
        #[must_use]
        #[inline(always)]
        pub const fn smac(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "0: Lookup by Destination MAC 1: Lookup by Source MAC."]
        #[inline(always)]
        pub const fn set_smac(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Active Destination MAC – control. See Table 6-6."]
        #[must_use]
        #[inline(always)]
        pub const fn actctl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Active Destination MAC – control. See Table 6-6."]
        #[inline(always)]
        pub const fn set_actctl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Sequence number generation enable."]
        #[must_use]
        #[inline(always)]
        pub const fn seqgen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Sequence number generation enable."]
        #[inline(always)]
        pub const fn set_seqgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Stream ID – inserted to header on match."]
        #[must_use]
        #[inline(always)]
        pub const fn sid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Stream ID – inserted to header on match."]
        #[inline(always)]
        pub const fn set_sid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for CpuPortIgressStmidControl {
        #[inline(always)]
        fn default() -> CpuPortIgressStmidControl {
            CpuPortIgressStmidControl(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressStmidControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressStmidControl")
                .field("en", &self.en())
                .field("mode", &self.mode())
                .field("smac", &self.smac())
                .field("actctl", &self.actctl())
                .field("seqgen", &self.seqgen())
                .field("sid", &self.sid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressStmidControl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CpuPortIgressStmidControl {{ en: {=bool:?}, mode: {=u8:?}, smac: {=bool:?}, actctl: {=u8:?}, seqgen: {=bool:?}, sid: {=u8:?} }}" , self . en () , self . mode () , self . smac () , self . actctl () , self . seqgen () , self . sid ())
        }
    }
    #[doc = "Stream Identification."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressStmidEselect(pub u32);
    impl CpuPortIgressStmidEselect {
        #[doc = "Select entry. Selected entry mapped to 0x40 – 0x5C."]
        #[must_use]
        #[inline(always)]
        pub const fn esel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Select entry. Selected entry mapped to 0x40 – 0x5C."]
        #[inline(always)]
        pub const fn set_esel(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for CpuPortIgressStmidEselect {
        #[inline(always)]
        fn default() -> CpuPortIgressStmidEselect {
            CpuPortIgressStmidEselect(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressStmidEselect {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressStmidEselect")
                .field("esel", &self.esel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressStmidEselect {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressStmidEselect {{ esel: {=u8:?} }}",
                self.esel()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressStmidMachi(pub u32);
    impl CpuPortIgressStmidMachi {
        #[doc = "MAC-Address \\[47:31\\]
used by lookup."]
        #[must_use]
        #[inline(always)]
        pub const fn match_(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC-Address \\[47:31\\]
used by lookup."]
        #[inline(always)]
        pub const fn set_match_(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "VLAN ID used by lookup."]
        #[must_use]
        #[inline(always)]
        pub const fn vid(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "VLAN ID used by lookup."]
        #[inline(always)]
        pub const fn set_vid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for CpuPortIgressStmidMachi {
        #[inline(always)]
        fn default() -> CpuPortIgressStmidMachi {
            CpuPortIgressStmidMachi(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressStmidMachi {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressStmidMachi")
                .field("match_", &self.match_())
                .field("vid", &self.vid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressStmidMachi {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressStmidMachi {{ match_: {=u16:?}, vid: {=u16:?} }}",
                self.match_(),
                self.vid()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressStmidMaclo(pub u32);
    impl CpuPortIgressStmidMaclo {
        #[doc = "MAC-Address \\[31:0\\]
used by lookup."]
        #[must_use]
        #[inline(always)]
        pub const fn macl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC-Address \\[31:0\\]
used by lookup."]
        #[inline(always)]
        pub const fn set_macl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortIgressStmidMaclo {
        #[inline(always)]
        fn default() -> CpuPortIgressStmidMaclo {
            CpuPortIgressStmidMaclo(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressStmidMaclo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressStmidMaclo")
                .field("macl", &self.macl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressStmidMaclo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressStmidMaclo {{ macl: {=u32:?} }}",
                self.macl()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressStmidMatchcnt(pub u32);
    impl CpuPortIgressStmidMatchcnt {
        #[doc = "Entry match counter – any write access to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn match_(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Entry match counter – any write access to clear."]
        #[inline(always)]
        pub const fn set_match_(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortIgressStmidMatchcnt {
        #[inline(always)]
        fn default() -> CpuPortIgressStmidMatchcnt {
            CpuPortIgressStmidMatchcnt(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressStmidMatchcnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressStmidMatchcnt")
                .field("match_", &self.match_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressStmidMatchcnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressStmidMatchcnt {{ match_: {=u32:?} }}",
                self.match_()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortIgressStmidSeqno(pub u32);
    impl CpuPortIgressStmidSeqno {
        #[doc = "Sequence number – next number when generating,any write access to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn seqno(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Sequence number – next number when generating,any write access to clear."]
        #[inline(always)]
        pub const fn set_seqno(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for CpuPortIgressStmidSeqno {
        #[inline(always)]
        fn default() -> CpuPortIgressStmidSeqno {
            CpuPortIgressStmidSeqno(0)
        }
    }
    impl core::fmt::Debug for CpuPortIgressStmidSeqno {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortIgressStmidSeqno")
                .field("seqno", &self.seqno())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortIgressStmidSeqno {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortIgressStmidSeqno {{ seqno: {=u16:?} }}",
                self.seqno()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorCtrl(pub u32);
    impl CpuPortMonitorCtrl {
        #[doc = "Enables counter. If deasserted the counter process stops and the counters hold their value."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enables counter. If deasserted the counter process stops and the counters hold their value."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for CpuPortMonitorCtrl {
        #[inline(always)]
        fn default() -> CpuPortMonitorCtrl {
            CpuPortMonitorCtrl(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorCtrl")
                .field("en", &self.en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "CpuPortMonitorCtrl {{ en: {=bool:?} }}", self.en())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorParam(pub u32);
    impl CpuPortMonitorParam {
        #[doc = "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available."]
        #[must_use]
        #[inline(always)]
        pub const fn cntw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available."]
        #[inline(always)]
        pub const fn set_cntw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cnt_en_vec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available."]
        #[inline(always)]
        pub const fn set_tx_cnt_en_vec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_cnt_en_vec(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available."]
        #[inline(always)]
        pub const fn set_rx_cnt_en_vec(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for CpuPortMonitorParam {
        #[inline(always)]
        fn default() -> CpuPortMonitorParam {
            CpuPortMonitorParam(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorParam")
                .field("cntw", &self.cntw())
                .field("tx_cnt_en_vec", &self.tx_cnt_en_vec())
                .field("rx_cnt_en_vec", &self.rx_cnt_en_vec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorParam {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CpuPortMonitorParam {{ cntw: {=u8:?}, tx_cnt_en_vec: {=u8:?}, rx_cnt_en_vec: {=u16:?} }}" , self . cntw () , self . tx_cnt_en_vec () , self . rx_cnt_en_vec ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorReset(pub u32);
    impl CpuPortMonitorReset {
        #[doc = "Write '1' to reset all TX&RX counters."]
        #[must_use]
        #[inline(always)]
        pub const fn rsall(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Write '1' to reset all TX&RX counters."]
        #[inline(always)]
        pub const fn set_rsall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write '1' to reset all TX counters."]
        #[must_use]
        #[inline(always)]
        pub const fn rstx(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write '1' to reset all TX counters."]
        #[inline(always)]
        pub const fn set_rstx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Write '1' to reset all RX counters."]
        #[must_use]
        #[inline(always)]
        pub const fn rsrx(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Write '1' to reset all RX counters."]
        #[inline(always)]
        pub const fn set_rsrx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for CpuPortMonitorReset {
        #[inline(always)]
        fn default() -> CpuPortMonitorReset {
            CpuPortMonitorReset(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorReset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorReset")
                .field("rsall", &self.rsall())
                .field("rstx", &self.rstx())
                .field("rsrx", &self.rsrx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorReset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorReset {{ rsall: {=bool:?}, rstx: {=bool:?}, rsrx: {=bool:?} }}",
                self.rsall(),
                self.rstx(),
                self.rsrx()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxBc(pub u32);
    impl CpuPortMonitorRxCounterRxBc {
        #[doc = "Number of Broadcast frames."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_bc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of Broadcast frames."]
        #[inline(always)]
        pub const fn set_rx_bc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxBc {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxBc {
            CpuPortMonitorRxCounterRxBc(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxBc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxBc")
                .field("rx_bc", &self.rx_bc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxBc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxBc {{ rx_bc: {=u32:?} }}",
                self.rx_bc()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxDropErr(pub u32);
    impl CpuPortMonitorRxCounterRxDropErr {
        #[doc = "Dropped frames with error by ingress. Possible in S&F mode or when frame is queued in ingress."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_drop_err(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Dropped frames with error by ingress. Possible in S&F mode or when frame is queued in ingress."]
        #[inline(always)]
        pub const fn set_rx_drop_err(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxDropErr {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxDropErr {
            CpuPortMonitorRxCounterRxDropErr(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxDropErr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxDropErr")
                .field("rx_drop_err", &self.rx_drop_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxDropErr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxDropErr {{ rx_drop_err: {=u32:?} }}",
                self.rx_drop_err()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxDropLu(pub u32);
    impl CpuPortMonitorRxCounterRxDropLu {
        #[doc = "Dropped frames by LookUp decision."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_drop_lu(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Dropped frames by LookUp decision."]
        #[inline(always)]
        pub const fn set_rx_drop_lu(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxDropLu {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxDropLu {
            CpuPortMonitorRxCounterRxDropLu(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxDropLu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxDropLu")
                .field("rx_drop_lu", &self.rx_drop_lu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxDropLu {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxDropLu {{ rx_drop_lu: {=u32:?} }}",
                self.rx_drop_lu()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxDropOvfl(pub u32);
    impl CpuPortMonitorRxCounterRxDropOvfl {
        #[doc = "Dropped frames by ingress overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_drop_ovfl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Dropped frames by ingress overflow."]
        #[inline(always)]
        pub const fn set_rx_drop_ovfl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxDropOvfl {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxDropOvfl {
            CpuPortMonitorRxCounterRxDropOvfl(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxDropOvfl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxDropOvfl")
                .field("rx_drop_ovfl", &self.rx_drop_ovfl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxDropOvfl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxDropOvfl {{ rx_drop_ovfl: {=u32:?} }}",
                self.rx_drop_ovfl()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxDropVlan(pub u32);
    impl CpuPortMonitorRxCounterRxDropVlan {
        #[doc = "Dropped frames by incompatible VLAN."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_drop_vlan(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Dropped frames by incompatible VLAN."]
        #[inline(always)]
        pub const fn set_rx_drop_vlan(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxDropVlan {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxDropVlan {
            CpuPortMonitorRxCounterRxDropVlan(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxDropVlan {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxDropVlan")
                .field("rx_drop_vlan", &self.rx_drop_vlan())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxDropVlan {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxDropVlan {{ rx_drop_vlan: {=u32:?} }}",
                self.rx_drop_vlan()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxFerror(pub u32);
    impl CpuPortMonitorRxCounterRxFerror {
        #[doc = "Bad received frame by ingress buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_ferror(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bad received frame by ingress buffer."]
        #[inline(always)]
        pub const fn set_rx_ferror(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxFerror {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxFerror {
            CpuPortMonitorRxCounterRxFerror(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxFerror {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxFerror")
                .field("rx_ferror", &self.rx_ferror())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxFerror {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxFerror {{ rx_ferror: {=u32:?} }}",
                self.rx_ferror()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxFgood(pub u32);
    impl CpuPortMonitorRxCounterRxFgood {
        #[doc = "Good received frame by ingress buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_fgood(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Good received frame by ingress buffer."]
        #[inline(always)]
        pub const fn set_rx_fgood(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxFgood {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxFgood {
            CpuPortMonitorRxCounterRxFgood(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxFgood {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxFgood")
                .field("rx_fgood", &self.rx_fgood())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxFgood {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxFgood {{ rx_fgood: {=u32:?} }}",
                self.rx_fgood()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxFpeFgood(pub u32);
    impl CpuPortMonitorRxCounterRxFpeFgood {
        #[doc = "Number of preemptable frames. Subset of RX_FGOOD."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_fpe_fgood(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of preemptable frames. Subset of RX_FGOOD."]
        #[inline(always)]
        pub const fn set_rx_fpe_fgood(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxFpeFgood {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxFpeFgood {
            CpuPortMonitorRxCounterRxFpeFgood(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxFpeFgood {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxFpeFgood")
                .field("rx_fpe_fgood", &self.rx_fpe_fgood())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxFpeFgood {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxFpeFgood {{ rx_fpe_fgood: {=u32:?} }}",
                self.rx_fpe_fgood()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxIntern(pub u32);
    impl CpuPortMonitorRxCounterRxIntern {
        #[doc = "Number of non-relay frames."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_intern(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of non-relay frames."]
        #[inline(always)]
        pub const fn set_rx_intern(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxIntern {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxIntern {
            CpuPortMonitorRxCounterRxIntern(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxIntern {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxIntern")
                .field("rx_intern", &self.rx_intern())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxIntern {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxIntern {{ rx_intern: {=u32:?} }}",
                self.rx_intern()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxKnown(pub u32);
    impl CpuPortMonitorRxCounterRxKnown {
        #[doc = "Number of frames passed ingress with hit by MAC Table. This includes Broadcast and non-relayed frames."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_known(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of frames passed ingress with hit by MAC Table. This includes Broadcast and non-relayed frames."]
        #[inline(always)]
        pub const fn set_rx_known(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxKnown {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxKnown {
            CpuPortMonitorRxCounterRxKnown(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxKnown {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxKnown")
                .field("rx_known", &self.rx_known())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxKnown {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxKnown {{ rx_known: {=u32:?} }}",
                self.rx_known()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxMulti(pub u32);
    impl CpuPortMonitorRxCounterRxMulti {
        #[doc = "Number of Multicast frames."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_multi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of Multicast frames."]
        #[inline(always)]
        pub const fn set_rx_multi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxMulti {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxMulti {
            CpuPortMonitorRxCounterRxMulti(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxMulti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxMulti")
                .field("rx_multi", &self.rx_multi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxMulti {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxMulti {{ rx_multi: {=u32:?} }}",
                self.rx_multi()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxUc(pub u32);
    impl CpuPortMonitorRxCounterRxUc {
        #[doc = "Number of unicast frames."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_uc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of unicast frames."]
        #[inline(always)]
        pub const fn set_rx_uc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxUc {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxUc {
            CpuPortMonitorRxCounterRxUc(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxUc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxUc")
                .field("rx_uc", &self.rx_uc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxUc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxUc {{ rx_uc: {=u32:?} }}",
                self.rx_uc()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxUnknown(pub u32);
    impl CpuPortMonitorRxCounterRxUnknown {
        #[doc = "Number of frames passed ingress without hit by MAC table."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_unknown(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of frames passed ingress without hit by MAC table."]
        #[inline(always)]
        pub const fn set_rx_unknown(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxUnknown {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxUnknown {
            CpuPortMonitorRxCounterRxUnknown(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxUnknown {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxUnknown")
                .field("rx_unknown", &self.rx_unknown())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxUnknown {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxUnknown {{ rx_unknown: {=u32:?} }}",
                self.rx_unknown()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorRxCounterRxVlan(pub u32);
    impl CpuPortMonitorRxCounterRxVlan {
        #[doc = "Number of VLAN tagged frames."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_vlan(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of VLAN tagged frames."]
        #[inline(always)]
        pub const fn set_rx_vlan(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorRxCounterRxVlan {
        #[inline(always)]
        fn default() -> CpuPortMonitorRxCounterRxVlan {
            CpuPortMonitorRxCounterRxVlan(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorRxCounterRxVlan {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorRxCounterRxVlan")
                .field("rx_vlan", &self.rx_vlan())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorRxCounterRxVlan {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorRxCounterRxVlan {{ rx_vlan: {=u32:?} }}",
                self.rx_vlan()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorTxCounterTxDropOvfl(pub u32);
    impl CpuPortMonitorTxCounterTxDropOvfl {
        #[doc = "Dropped frames by full queue of TSN-EP."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_drop_ovfl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Dropped frames by full queue of TSN-EP."]
        #[inline(always)]
        pub const fn set_tx_drop_ovfl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorTxCounterTxDropOvfl {
        #[inline(always)]
        fn default() -> CpuPortMonitorTxCounterTxDropOvfl {
            CpuPortMonitorTxCounterTxDropOvfl(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorTxCounterTxDropOvfl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorTxCounterTxDropOvfl")
                .field("tx_drop_ovfl", &self.tx_drop_ovfl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorTxCounterTxDropOvfl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorTxCounterTxDropOvfl {{ tx_drop_ovfl: {=u32:?} }}",
                self.tx_drop_ovfl()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorTxCounterTxFerror(pub u32);
    impl CpuPortMonitorTxCounterTxFerror {
        #[doc = "Transmitted Frames with Error to TX TSN-EP."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ferror(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmitted Frames with Error to TX TSN-EP."]
        #[inline(always)]
        pub const fn set_tx_ferror(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorTxCounterTxFerror {
        #[inline(always)]
        fn default() -> CpuPortMonitorTxCounterTxFerror {
            CpuPortMonitorTxCounterTxFerror(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorTxCounterTxFerror {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorTxCounterTxFerror")
                .field("tx_ferror", &self.tx_ferror())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorTxCounterTxFerror {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorTxCounterTxFerror {{ tx_ferror: {=u32:?} }}",
                self.tx_ferror()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortMonitorTxCounterTxFgood(pub u32);
    impl CpuPortMonitorTxCounterTxFgood {
        #[doc = "Good transmitted Frames to TX TSN-EP."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_fgood(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Good transmitted Frames to TX TSN-EP."]
        #[inline(always)]
        pub const fn set_tx_fgood(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for CpuPortMonitorTxCounterTxFgood {
        #[inline(always)]
        fn default() -> CpuPortMonitorTxCounterTxFgood {
            CpuPortMonitorTxCounterTxFgood(0)
        }
    }
    impl core::fmt::Debug for CpuPortMonitorTxCounterTxFgood {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortMonitorTxCounterTxFgood")
                .field("tx_fgood", &self.tx_fgood())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortMonitorTxCounterTxFgood {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortMonitorTxCounterTxFgood {{ tx_fgood: {=u32:?} }}",
                self.tx_fgood()
            )
        }
    }
    #[doc = "Port Module Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortPortMainEnnable(pub u32);
    impl CpuPortPortMainEnnable {
        #[doc = "if QCI is present at selected egress port, '1' to use QCI and '0' disable QCI. Changing during frame operation can lead to frame corruption."]
        #[must_use]
        #[inline(always)]
        pub const fn en_qci(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "if QCI is present at selected egress port, '1' to use QCI and '0' disable QCI. Changing during frame operation can lead to frame corruption."]
        #[inline(always)]
        pub const fn set_en_qci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "only applicable for CPU-Port at egress: '1' to use S&F FIFO and '0' disable S&F FIFO. Changing during frame operation can lead to frame corruption."]
        #[must_use]
        #[inline(always)]
        pub const fn en_sf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "only applicable for CPU-Port at egress: '1' to use S&F FIFO and '0' disable S&F FIFO. Changing during frame operation can lead to frame corruption."]
        #[inline(always)]
        pub const fn set_en_sf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for CpuPortPortMainEnnable {
        #[inline(always)]
        fn default() -> CpuPortPortMainEnnable {
            CpuPortPortMainEnnable(0)
        }
    }
    impl core::fmt::Debug for CpuPortPortMainEnnable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortPortMainEnnable")
                .field("en_qci", &self.en_qci())
                .field("en_sf", &self.en_sf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortPortMainEnnable {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CpuPortPortMainEnnable {{ en_qci: {=bool:?}, en_sf: {=bool:?} }}",
                self.en_qci(),
                self.en_sf()
            )
        }
    }
    #[doc = "PVID Tagging Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuPortPortMainTagging(pub u32);
    impl CpuPortPortMainTagging {
        #[doc = "Native VLAN of Port. Untagged traffic will be tagged with the native VLAN-ID By default the Port uses VLAN 1."]
        #[must_use]
        #[inline(always)]
        pub const fn pvid(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Native VLAN of Port. Untagged traffic will be tagged with the native VLAN-ID By default the Port uses VLAN 1."]
        #[inline(always)]
        pub const fn set_pvid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "VLAN-TCI: Drop Eligible Indicator, used when tagged."]
        #[must_use]
        #[inline(always)]
        pub const fn dei(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN-TCI: Drop Eligible Indicator, used when tagged."]
        #[inline(always)]
        pub const fn set_dei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "VLAN-TCI: Priority Code Point, used when tagged."]
        #[must_use]
        #[inline(always)]
        pub const fn pcp(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "VLAN-TCI: Priority Code Point, used when tagged."]
        #[inline(always)]
        pub const fn set_pcp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Every tagged frame not matching PVID is filtered out. Every untagged ingress frame will be tagged with PVID. Every egress frame with PVID will be untagged."]
        #[must_use]
        #[inline(always)]
        pub const fn access(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Every tagged frame not matching PVID is filtered out. Every untagged ingress frame will be tagged with PVID. Every egress frame with PVID will be untagged."]
        #[inline(always)]
        pub const fn set_access(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "The VLAN-TAG with PVID will be inserted in every frame from Host as their first VLAN-TAG. This can be used for double tagging of tagged/trunk ports."]
        #[must_use]
        #[inline(always)]
        pub const fn force(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "The VLAN-TAG with PVID will be inserted in every frame from Host as their first VLAN-TAG. This can be used for double tagging of tagged/trunk ports."]
        #[inline(always)]
        pub const fn set_force(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for CpuPortPortMainTagging {
        #[inline(always)]
        fn default() -> CpuPortPortMainTagging {
            CpuPortPortMainTagging(0)
        }
    }
    impl core::fmt::Debug for CpuPortPortMainTagging {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CpuPortPortMainTagging")
                .field("pvid", &self.pvid())
                .field("dei", &self.dei())
                .field("pcp", &self.pcp())
                .field("access", &self.access())
                .field("force", &self.force())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CpuPortPortMainTagging {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CpuPortPortMainTagging {{ pvid: {=u16:?}, dei: {=bool:?}, pcp: {=u8:?}, access: {=bool:?}, force: {=bool:?} }}" , self . pvid () , self . dei () , self . pcp () , self . access () , self . force ())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Egfrcnt(pub u32);
    impl Egfrcnt {
        #[doc = "Frame counters."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Frame counters."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Egfrcnt {
        #[inline(always)]
        fn default() -> Egfrcnt {
            Egfrcnt(0)
        }
    }
    impl core::fmt::Debug for Egfrcnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Egfrcnt")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Egfrcnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Egfrcnt {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "control register0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprCtrl0(pub u32);
    impl GprCtrl0 {
        #[doc = "delay value of txclk_delay_chain."]
        #[must_use]
        #[inline(always)]
        pub const fn txclk_dly_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "delay value of txclk_delay_chain."]
        #[inline(always)]
        pub const fn set_txclk_dly_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "delay value of rxclk_delay_chain."]
        #[must_use]
        #[inline(always)]
        pub const fn rxclk_dly_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "delay value of rxclk_delay_chain."]
        #[inline(always)]
        pub const fn set_rxclk_dly_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for GprCtrl0 {
        #[inline(always)]
        fn default() -> GprCtrl0 {
            GprCtrl0(0)
        }
    }
    impl core::fmt::Debug for GprCtrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GprCtrl0")
                .field("txclk_dly_sel", &self.txclk_dly_sel())
                .field("rxclk_dly_sel", &self.rxclk_dly_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GprCtrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "GprCtrl0 {{ txclk_dly_sel: {=u8:?}, rxclk_dly_sel: {=u8:?} }}",
                self.txclk_dly_sel(),
                self.rxclk_dly_sel()
            )
        }
    }
    #[doc = "control register2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GprCtrl2(pub u32);
    impl GprCtrl2 {
        #[doc = "txclk select control for RMII."]
        #[must_use]
        #[inline(always)]
        pub const fn rmii_txclk_sel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "txclk select control for RMII."]
        #[inline(always)]
        pub const fn set_rmii_txclk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "phy interface select."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_intf_sel(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "phy interface select."]
        #[inline(always)]
        pub const fn set_phy_intf_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "refclock output enable when rmii."]
        #[must_use]
        #[inline(always)]
        pub const fn pad_oe_eth_refclk(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "refclock output enable when rmii."]
        #[inline(always)]
        pub const fn set_pad_oe_eth_refclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "mac speed."]
        #[must_use]
        #[inline(always)]
        pub const fn mac_speed(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "mac speed."]
        #[inline(always)]
        pub const fn set_mac_speed(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for GprCtrl2 {
        #[inline(always)]
        fn default() -> GprCtrl2 {
            GprCtrl2(0)
        }
    }
    impl core::fmt::Debug for GprCtrl2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GprCtrl2")
                .field("rmii_txclk_sel", &self.rmii_txclk_sel())
                .field("phy_intf_sel", &self.phy_intf_sel())
                .field("pad_oe_eth_refclk", &self.pad_oe_eth_refclk())
                .field("mac_speed", &self.mac_speed())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GprCtrl2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GprCtrl2 {{ rmii_txclk_sel: {=bool:?}, phy_intf_sel: {=u8:?}, pad_oe_eth_refclk: {=bool:?}, mac_speed: {=u8:?} }}" , self . rmii_txclk_sel () , self . phy_intf_sel () , self . pad_oe_eth_refclk () , self . mac_speed ())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hitmem(pub u32);
    impl Hitmem {
        #[doc = "Every bit represents a lookup entry starting with bit 0 as entry 0. The memory can be written and cleared by the host system via common memory-mapped bus access."]
        #[must_use]
        #[inline(always)]
        pub const fn hitmem_reg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Every bit represents a lookup entry starting with bit 0 as entry 0. The memory can be written and cleared by the host system via common memory-mapped bus access."]
        #[inline(always)]
        pub const fn set_hitmem_reg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Hitmem {
        #[inline(always)]
        fn default() -> Hitmem {
            Hitmem(0)
        }
    }
    impl core::fmt::Debug for Hitmem {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hitmem")
                .field("hitmem_reg", &self.hitmem_reg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hitmem {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Hitmem {{ hitmem_reg: {=u32:?} }}", self.hitmem_reg())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idsel(pub u32);
    impl Idsel {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn fract(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_fract(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "CBS idle slope for traffic queue n (n = 0 – 7). Returns 0 when n > TQC. The register must only be written when TXSELi.CBE_EN=0. The idle slope value is defined as (INT + FRACT / 65536). The idle slope is set in bits per tick related to <tx_clk>."]
        #[must_use]
        #[inline(always)]
        pub const fn int(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "CBS idle slope for traffic queue n (n = 0 – 7). Returns 0 when n > TQC. The register must only be written when TXSELi.CBE_EN=0. The idle slope value is defined as (INT + FRACT / 65536). The idle slope is set in bits per tick related to <tx_clk>."]
        #[inline(always)]
        pub const fn set_int(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Idsel {
        #[inline(always)]
        fn default() -> Idsel {
            Idsel(0)
        }
    }
    impl core::fmt::Debug for Idsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idsel")
                .field("fract", &self.fract())
                .field("int", &self.int())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idsel {{ fract: {=u16:?}, int: {=u8:?} }}",
                self.fract(),
                self.int()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Igfrcnt(pub u32);
    impl Igfrcnt {
        #[doc = "Frame counters."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Frame counters."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Igfrcnt {
        #[inline(always)]
        fn default() -> Igfrcnt {
            Igfrcnt(0)
        }
    }
    impl core::fmt::Debug for Igfrcnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Igfrcnt")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Igfrcnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Igfrcnt {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "LU_MAIN low word of action data for broadcast frames."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LuMainBcAction(pub u32);
    impl LuMainBcAction {
        #[doc = "Select the destination ports of forwarded frame. It is coded in onehot/select way, where 0 is always route to null. Every bit is mapped to a port. 00000 – to null (frame to clear) 00001 – to port 0 (CPU Port) 00010 – to port 1 00100 – to port 2 01000 – to port 3."]
        #[must_use]
        #[inline(always)]
        pub const fn dest(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Select the destination ports of forwarded frame. It is coded in onehot/select way, where 0 is always route to null. Every bit is mapped to a port. 00000 – to null (frame to clear) 00001 – to port 0 (CPU Port) 00010 – to port 1 00100 – to port 2 01000 – to port 3."]
        #[inline(always)]
        pub const fn set_dest(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Select the Priority Queue for TSN TX, only used if QSEL=11."]
        #[must_use]
        #[inline(always)]
        pub const fn queue(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Select the Priority Queue for TSN TX, only used if QSEL=11."]
        #[inline(always)]
        pub const fn set_queue(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "1 if frame should be dropped."]
        #[must_use]
        #[inline(always)]
        pub const fn drop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "1 if frame should be dropped."]
        #[inline(always)]
        pub const fn set_drop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Define the traffic queue selection: 00 – use PCP field of VLAN, untagged frames use PCP of PVID 01 – use PCP field with global remapping list 10 – reserved 11 – use value QUEUE of Action List."]
        #[must_use]
        #[inline(always)]
        pub const fn qsel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Define the traffic queue selection: 00 – use PCP field of VLAN, untagged frames use PCP of PVID 01 – use PCP field with global remapping list 10 – reserved 11 – use value QUEUE of Action List."]
        #[inline(always)]
        pub const fn set_qsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "TSN user sideband information from ALMEM."]
        #[must_use]
        #[inline(always)]
        pub const fn utag(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x07;
            val as u8
        }
        #[doc = "TSN user sideband information from ALMEM."]
        #[inline(always)]
        pub const fn set_utag(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
        }
    }
    impl Default for LuMainBcAction {
        #[inline(always)]
        fn default() -> LuMainBcAction {
            LuMainBcAction(0)
        }
    }
    impl core::fmt::Debug for LuMainBcAction {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LuMainBcAction")
                .field("dest", &self.dest())
                .field("queue", &self.queue())
                .field("drop", &self.drop())
                .field("qsel", &self.qsel())
                .field("utag", &self.utag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LuMainBcAction {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "LuMainBcAction {{ dest: {=u16:?}, queue: {=u8:?}, drop: {=bool:?}, qsel: {=u8:?}, utag: {=u8:?} }}" , self . dest () , self . queue () , self . drop () , self . qsel () , self . utag ())
        }
    }
    #[doc = "LU_MAIN bypass."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LuMainBypass(pub u32);
    impl LuMainBypass {
        #[doc = "target destination ports of frame."]
        #[must_use]
        #[inline(always)]
        pub const fn dest(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "target destination ports of frame."]
        #[inline(always)]
        pub const fn set_dest(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "number of configured buffer depth."]
        #[must_use]
        #[inline(always)]
        pub const fn queue(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "number of configured buffer depth."]
        #[inline(always)]
        pub const fn set_queue(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "mark frame to be dropped."]
        #[must_use]
        #[inline(always)]
        pub const fn drop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "mark frame to be dropped."]
        #[inline(always)]
        pub const fn set_drop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "mark frame to be vlan-tagged."]
        #[must_use]
        #[inline(always)]
        pub const fn hit_vlan(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "mark frame to be vlan-tagged."]
        #[inline(always)]
        pub const fn set_hit_vlan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "set internal user tag field."]
        #[must_use]
        #[inline(always)]
        pub const fn utag(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "set internal user tag field."]
        #[inline(always)]
        pub const fn set_utag(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "set hit bit to frame, only for debugging."]
        #[must_use]
        #[inline(always)]
        pub const fn hit(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "set hit bit to frame, only for debugging."]
        #[inline(always)]
        pub const fn set_hit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for LuMainBypass {
        #[inline(always)]
        fn default() -> LuMainBypass {
            LuMainBypass(0)
        }
    }
    impl core::fmt::Debug for LuMainBypass {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LuMainBypass")
                .field("dest", &self.dest())
                .field("queue", &self.queue())
                .field("drop", &self.drop())
                .field("hit_vlan", &self.hit_vlan())
                .field("utag", &self.utag())
                .field("hit", &self.hit())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LuMainBypass {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "LuMainBypass {{ dest: {=u16:?}, queue: {=u8:?}, drop: {=bool:?}, hit_vlan: {=bool:?}, utag: {=u8:?}, hit: {=bool:?} }}" , self . dest () , self . queue () , self . drop () , self . hit_vlan () , self . utag () , self . hit ())
        }
    }
    #[doc = "LU_MAIN control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LuMainCtrl(pub u32);
    impl LuMainCtrl {
        #[doc = "MAC lookup bypass."]
        #[must_use]
        #[inline(always)]
        pub const fn byp_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MAC lookup bypass."]
        #[inline(always)]
        pub const fn set_byp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for LuMainCtrl {
        #[inline(always)]
        fn default() -> LuMainCtrl {
            LuMainCtrl(0)
        }
    }
    impl core::fmt::Debug for LuMainCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LuMainCtrl")
                .field("byp_en", &self.byp_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LuMainCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LuMainCtrl {{ byp_en: {=bool:?} }}", self.byp_en())
        }
    }
    #[doc = "LU_MAIN hit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LuMainHitmem(pub u32);
    impl LuMainHitmem {
        #[doc = "clears the hit memory."]
        #[must_use]
        #[inline(always)]
        pub const fn hitmemclr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "clears the hit memory."]
        #[inline(always)]
        pub const fn set_hitmemclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "clear the cam memory."]
        #[must_use]
        #[inline(always)]
        pub const fn cammemclr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clear the cam memory."]
        #[inline(always)]
        pub const fn set_cammemclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for LuMainHitmem {
        #[inline(always)]
        fn default() -> LuMainHitmem {
            LuMainHitmem(0)
        }
    }
    impl core::fmt::Debug for LuMainHitmem {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LuMainHitmem")
                .field("hitmemclr", &self.hitmemclr())
                .field("cammemclr", &self.cammemclr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LuMainHitmem {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LuMainHitmem {{ hitmemclr: {=bool:?}, cammemclr: {=bool:?} }}",
                self.hitmemclr(),
                self.cammemclr()
            )
        }
    }
    #[doc = "LU_MAIN low word of action data for internal frames."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LuMainIntfAction(pub u32);
    impl LuMainIntfAction {
        #[doc = "Select the destination ports of forwarded frame. It is coded in onehot/select way, where 0 is always route to null. Every bit is mapped to a port. 00000 – to null (frame to clear) 00001 – to port 0 (CPU Port) 00010 – to port 1 00100 – to port 2 01000 – to port 3."]
        #[must_use]
        #[inline(always)]
        pub const fn dest(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Select the destination ports of forwarded frame. It is coded in onehot/select way, where 0 is always route to null. Every bit is mapped to a port. 00000 – to null (frame to clear) 00001 – to port 0 (CPU Port) 00010 – to port 1 00100 – to port 2 01000 – to port 3."]
        #[inline(always)]
        pub const fn set_dest(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Select the Priority Queue for TSN TX, only used if QSEL=11."]
        #[must_use]
        #[inline(always)]
        pub const fn queue(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Select the Priority Queue for TSN TX, only used if QSEL=11."]
        #[inline(always)]
        pub const fn set_queue(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "1 if frame should be dropped."]
        #[must_use]
        #[inline(always)]
        pub const fn drop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "1 if frame should be dropped."]
        #[inline(always)]
        pub const fn set_drop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Define the traffic queue selection: 00 – use PCP field of VLAN, untagged frames use PCP of PVID 01 – use PCP field with global remapping list 10 – reserved 11 – use value QUEUE of Action List."]
        #[must_use]
        #[inline(always)]
        pub const fn qsel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Define the traffic queue selection: 00 – use PCP field of VLAN, untagged frames use PCP of PVID 01 – use PCP field with global remapping list 10 – reserved 11 – use value QUEUE of Action List."]
        #[inline(always)]
        pub const fn set_qsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "TSN user sideband information from ALMEM."]
        #[must_use]
        #[inline(always)]
        pub const fn utag(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x07;
            val as u8
        }
        #[doc = "TSN user sideband information from ALMEM."]
        #[inline(always)]
        pub const fn set_utag(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
        }
    }
    impl Default for LuMainIntfAction {
        #[inline(always)]
        fn default() -> LuMainIntfAction {
            LuMainIntfAction(0)
        }
    }
    impl core::fmt::Debug for LuMainIntfAction {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LuMainIntfAction")
                .field("dest", &self.dest())
                .field("queue", &self.queue())
                .field("drop", &self.drop())
                .field("qsel", &self.qsel())
                .field("utag", &self.utag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LuMainIntfAction {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "LuMainIntfAction {{ dest: {=u16:?}, queue: {=u8:?}, drop: {=bool:?}, qsel: {=u8:?}, utag: {=u8:?} }}" , self . dest () , self . queue () , self . drop () , self . qsel () , self . utag ())
        }
    }
    #[doc = "LU_MAIN low word of action data for unknown frames."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LuMainNnAction(pub u32);
    impl LuMainNnAction {
        #[doc = "Select the destination ports of forwarded frame. It is coded in onehot/select way, where 0 is always route to null. Every bit is mapped to a port. 00000 – to null (frame to clear) 00001 – to port 0 (CPU Port) 00010 – to port 1 00100 – to port 2 01000 – to port 3."]
        #[must_use]
        #[inline(always)]
        pub const fn dest(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Select the destination ports of forwarded frame. It is coded in onehot/select way, where 0 is always route to null. Every bit is mapped to a port. 00000 – to null (frame to clear) 00001 – to port 0 (CPU Port) 00010 – to port 1 00100 – to port 2 01000 – to port 3."]
        #[inline(always)]
        pub const fn set_dest(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Select the Priority Queue for TSN TX, only used if QSEL=11."]
        #[must_use]
        #[inline(always)]
        pub const fn queue(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Select the Priority Queue for TSN TX, only used if QSEL=11."]
        #[inline(always)]
        pub const fn set_queue(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "1 if frame should be dropped."]
        #[must_use]
        #[inline(always)]
        pub const fn drop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "1 if frame should be dropped."]
        #[inline(always)]
        pub const fn set_drop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Define the traffic queue selection: 00 – use PCP field of VLAN, untagged frames use PCP of PVID 01 – use PCP field with global remapping list 10 – reserved 11 – use value QUEUE of Action List."]
        #[must_use]
        #[inline(always)]
        pub const fn qsel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Define the traffic queue selection: 00 – use PCP field of VLAN, untagged frames use PCP of PVID 01 – use PCP field with global remapping list 10 – reserved 11 – use value QUEUE of Action List."]
        #[inline(always)]
        pub const fn set_qsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "TSN user sideband information from ALMEM."]
        #[must_use]
        #[inline(always)]
        pub const fn utag(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x07;
            val as u8
        }
        #[doc = "TSN user sideband information from ALMEM."]
        #[inline(always)]
        pub const fn set_utag(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
        }
    }
    impl Default for LuMainNnAction {
        #[inline(always)]
        fn default() -> LuMainNnAction {
            LuMainNnAction(0)
        }
    }
    impl core::fmt::Debug for LuMainNnAction {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LuMainNnAction")
                .field("dest", &self.dest())
                .field("queue", &self.queue())
                .field("drop", &self.drop())
                .field("qsel", &self.qsel())
                .field("utag", &self.utag())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LuMainNnAction {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "LuMainNnAction {{ dest: {=u16:?}, queue: {=u8:?}, drop: {=bool:?}, qsel: {=u8:?}, utag: {=u8:?} }}" , self . dest () , self . queue () , self . drop () , self . qsel () , self . utag ())
        }
    }
    #[doc = "LU_MAIN parameter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LuMainParam(pub u32);
    impl LuMainParam {
        #[doc = "bit width of entry address vector."]
        #[must_use]
        #[inline(always)]
        pub const fn addrw_entry(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "bit width of entry address vector."]
        #[inline(always)]
        pub const fn set_addrw_entry(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "number of supported streams."]
        #[must_use]
        #[inline(always)]
        pub const fn nstr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "number of supported streams."]
        #[inline(always)]
        pub const fn set_nstr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for LuMainParam {
        #[inline(always)]
        fn default() -> LuMainParam {
            LuMainParam(0)
        }
    }
    impl core::fmt::Debug for LuMainParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LuMainParam")
                .field("addrw_entry", &self.addrw_entry())
                .field("nstr", &self.nstr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LuMainParam {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LuMainParam {{ addrw_entry: {=u8:?}, nstr: {=u8:?} }}",
                self.addrw_entry(),
                self.nstr()
            )
        }
    }
    #[doc = "LU_MAIN PCP remap."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LuMainPcpRemap(pub u32);
    impl LuMainPcpRemap {
        #[doc = "queue value for PCP=0."]
        #[must_use]
        #[inline(always)]
        pub const fn pcp0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "queue value for PCP=0."]
        #[inline(always)]
        pub const fn set_pcp0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "queue value for PCP=1."]
        #[must_use]
        #[inline(always)]
        pub const fn pcp1(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "queue value for PCP=1."]
        #[inline(always)]
        pub const fn set_pcp1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "queue value for PCP=2."]
        #[must_use]
        #[inline(always)]
        pub const fn pcp2(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "queue value for PCP=2."]
        #[inline(always)]
        pub const fn set_pcp2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "queue value for PCP=3."]
        #[must_use]
        #[inline(always)]
        pub const fn pcp3(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "queue value for PCP=3."]
        #[inline(always)]
        pub const fn set_pcp3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "queue value for PCP=4."]
        #[must_use]
        #[inline(always)]
        pub const fn pcp4(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "queue value for PCP=4."]
        #[inline(always)]
        pub const fn set_pcp4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "queue value for PCP=5."]
        #[must_use]
        #[inline(always)]
        pub const fn pcp5(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x07;
            val as u8
        }
        #[doc = "queue value for PCP=5."]
        #[inline(always)]
        pub const fn set_pcp5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
        }
        #[doc = "queue value for PCP=6."]
        #[must_use]
        #[inline(always)]
        pub const fn pcp6(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[doc = "queue value for PCP=6."]
        #[inline(always)]
        pub const fn set_pcp6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
        #[doc = "queue value for PCP=7."]
        #[must_use]
        #[inline(always)]
        pub const fn pcp7(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "queue value for PCP=7."]
        #[inline(always)]
        pub const fn set_pcp7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
    }
    impl Default for LuMainPcpRemap {
        #[inline(always)]
        fn default() -> LuMainPcpRemap {
            LuMainPcpRemap(0)
        }
    }
    impl core::fmt::Debug for LuMainPcpRemap {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LuMainPcpRemap")
                .field("pcp0", &self.pcp0())
                .field("pcp1", &self.pcp1())
                .field("pcp2", &self.pcp2())
                .field("pcp3", &self.pcp3())
                .field("pcp4", &self.pcp4())
                .field("pcp5", &self.pcp5())
                .field("pcp6", &self.pcp6())
                .field("pcp7", &self.pcp7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LuMainPcpRemap {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "LuMainPcpRemap {{ pcp0: {=u8:?}, pcp1: {=u8:?}, pcp2: {=u8:?}, pcp3: {=u8:?}, pcp4: {=u8:?}, pcp5: {=u8:?}, pcp6: {=u8:?}, pcp7: {=u8:?} }}" , self . pcp0 () , self . pcp1 () , self . pcp2 () , self . pcp3 () , self . pcp4 () , self . pcp5 () , self . pcp6 () , self . pcp7 ())
        }
    }
    #[doc = "LU_MAIN version."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LuMainVersion(pub u32);
    impl LuMainVersion {
        #[doc = "revision number."]
        #[must_use]
        #[inline(always)]
        pub const fn ver_rev(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "revision number."]
        #[inline(always)]
        pub const fn set_ver_rev(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "minor version."]
        #[must_use]
        #[inline(always)]
        pub const fn ver_lo(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "minor version."]
        #[inline(always)]
        pub const fn set_ver_lo(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "major version."]
        #[must_use]
        #[inline(always)]
        pub const fn ver_hi(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "major version."]
        #[inline(always)]
        pub const fn set_ver_hi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for LuMainVersion {
        #[inline(always)]
        fn default() -> LuMainVersion {
            LuMainVersion(0)
        }
    }
    impl core::fmt::Debug for LuMainVersion {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LuMainVersion")
                .field("ver_rev", &self.ver_rev())
                .field("ver_lo", &self.ver_lo())
                .field("ver_hi", &self.ver_hi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LuMainVersion {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LuMainVersion {{ ver_rev: {=u8:?}, ver_lo: {=u8:?}, ver_hi: {=u8:?} }}",
                self.ver_rev(),
                self.ver_lo(),
                self.ver_hi()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacIrqCtrl(pub u32);
    impl MacIrqCtrl {
        #[doc = "MDIO Interrupt Enable 0 – Disabled 1 – Enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn mdie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDIO Interrupt Enable 0 – Disabled 1 – Enabled."]
        #[inline(always)]
        pub const fn set_mdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Safety warning interrupt enable 0 – SWIF disabled 1 – SWIF enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn swie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Safety warning interrupt enable 0 – SWIF disabled 1 – SWIF enabled."]
        #[inline(always)]
        pub const fn set_swie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clock activity interrupt enable 0 – CAIF disabled 1 – CAIF enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn caie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clock activity interrupt enable 0 – CAIF disabled 1 – CAIF enabled."]
        #[inline(always)]
        pub const fn set_caie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MDIO Interrupt Flag 1 – A transfer has been finished 0 – No transfer done."]
        #[must_use]
        #[inline(always)]
        pub const fn mdif(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MDIO Interrupt Flag 1 – A transfer has been finished 0 – No transfer done."]
        #[inline(always)]
        pub const fn set_mdif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Safety Error Interrupt Flag 0 – no interrupt 1 – interrupt pending If SEN=1 and if there is a mismatch between both instances of the logic core of LLEMAC-1G then this results in SEIF=1, TX_EN=0 and RX_EN=0."]
        #[must_use]
        #[inline(always)]
        pub const fn seif(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Safety Error Interrupt Flag 0 – no interrupt 1 – interrupt pending If SEN=1 and if there is a mismatch between both instances of the logic core of LLEMAC-1G then this results in SEIF=1, TX_EN=0 and RX_EN=0."]
        #[inline(always)]
        pub const fn set_seif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Safety warning interrupt flag 0 – no interrupt 1 – interrupt pending See Chapter 11.2.2 for details."]
        #[must_use]
        #[inline(always)]
        pub const fn swif(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Safety warning interrupt flag 0 – no interrupt 1 – interrupt pending See Chapter 11.2.2 for details."]
        #[inline(always)]
        pub const fn set_swif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Clock activity interrupt flag 0 – no interrupt 1 – interrupt pending See Chapter 11.2.3 for details."]
        #[must_use]
        #[inline(always)]
        pub const fn caif(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Clock activity interrupt flag 0 – no interrupt 1 – interrupt pending See Chapter 11.2.3 for details."]
        #[inline(always)]
        pub const fn set_caif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for MacIrqCtrl {
        #[inline(always)]
        fn default() -> MacIrqCtrl {
            MacIrqCtrl(0)
        }
    }
    impl core::fmt::Debug for MacIrqCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacIrqCtrl")
                .field("mdie", &self.mdie())
                .field("swie", &self.swie())
                .field("caie", &self.caie())
                .field("mdif", &self.mdif())
                .field("seif", &self.seif())
                .field("swif", &self.swif())
                .field("caif", &self.caif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacIrqCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MacIrqCtrl {{ mdie: {=bool:?}, swie: {=bool:?}, caie: {=bool:?}, mdif: {=bool:?}, seif: {=bool:?}, swif: {=bool:?}, caif: {=bool:?} }}" , self . mdie () , self . swie () , self . caie () , self . mdif () , self . seif () , self . swif () , self . caif ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacCtrl(pub u32);
    impl MacMacCtrl {
        #[doc = "Software reset of the statistic counters (see Table 3-8) 0 – no reset 1 – reset active RESSTAT will be automatically set to 0 after the counters have been reset."]
        #[must_use]
        #[inline(always)]
        pub const fn resstat(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset of the statistic counters (see Table 3-8) 0 – no reset 1 – reset active RESSTAT will be automatically set to 0 after the counters have been reset."]
        #[inline(always)]
        pub const fn set_resstat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RX path enable 0 – reception disabled – no frames fed to Avalon-ST RX path 1 – reception enabled RX_EN can be activated or deactivated at any time. Deactivation may take some time. If during deactivation there is a frame in reception, then this frame will be completed first. Afterwards bit RX_EN can be read as 0."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RX path enable 0 – reception disabled – no frames fed to Avalon-ST RX path 1 – reception enabled RX_EN can be activated or deactivated at any time. Deactivation may take some time. If during deactivation there is a frame in reception, then this frame will be completed first. Afterwards bit RX_EN can be read as 0."]
        #[inline(always)]
        pub const fn set_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TX path enable 0 – transmission disabled - Avalon-ST READY for the TX path will be set to 0. 1 – transmission enabled TX_EN can be activated or deactivated at any time. Deactivation may take some time. If during deactivation there is a frame in transmission, then this frame will be completed fist. Afterwards bit TX_EN can be read as 0. After the transmission is disabled there may be pending frames left, waiting at the TX stream interface."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TX path enable 0 – transmission disabled - Avalon-ST READY for the TX path will be set to 0. 1 – transmission enabled TX_EN can be activated or deactivated at any time. Deactivation may take some time. If during deactivation there is a frame in transmission, then this frame will be completed fist. Afterwards bit TX_EN can be read as 0. After the transmission is disabled there may be pending frames left, waiting at the TX stream interface."]
        #[inline(always)]
        pub const fn set_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Jumbo frame support 0 – jumbo frames not supported 1 – jumbo frame supported (not recommended) Jumbo frames are non-standard Ethernet frames with a size bigger than envelope frames (which contain 1982 payload bytes). If jumbo frames are not supported, then LLEMAC-1G generates the appropriate error signals (<tx_gmii_er> for the TX path and <rx_avst_err> for the RX path). Although jumbo frames typically contain up to 9000 bytes, the LLEMAC-1G can handle an infinite frame size. The problem of jumbo frames is the necessary storage space in transmission and reception buffers. LLEMAC-1G does not include storage buffers. JUMBO can be activated or deactivated at any time. The new setting becomes valid immediately after clock domain crossing."]
        #[must_use]
        #[inline(always)]
        pub const fn jumbo(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Jumbo frame support 0 – jumbo frames not supported 1 – jumbo frame supported (not recommended) Jumbo frames are non-standard Ethernet frames with a size bigger than envelope frames (which contain 1982 payload bytes). If jumbo frames are not supported, then LLEMAC-1G generates the appropriate error signals (<tx_gmii_er> for the TX path and <rx_avst_err> for the RX path). Although jumbo frames typically contain up to 9000 bytes, the LLEMAC-1G can handle an infinite frame size. The problem of jumbo frames is the necessary storage space in transmission and reception buffers. LLEMAC-1G does not include storage buffers. JUMBO can be activated or deactivated at any time. The new setting becomes valid immediately after clock domain crossing."]
        #[inline(always)]
        pub const fn set_jumbo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GMII mode / Ethernet speed selection (See Chapter 4.5.) 0 – MII: 10Mbit/s or 100Mbit/s 1 – GMII: 1GBit/s GMIIMODE can only be changed if RX_EN=0 and TX_EN=0. Deactivation delays of RX_EN and TX_EN have to be considered. GMIIMODE can only be changed, if these register bits can be read as 0. It is possible to change GMIIMODE together with the activation of RX_EN and TX_EN. GMIIMODE drives the outputs <tx_gmiimode> and <rx_gmiimode>."]
        #[must_use]
        #[inline(always)]
        pub const fn gmiimode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GMII mode / Ethernet speed selection (See Chapter 4.5.) 0 – MII: 10Mbit/s or 100Mbit/s 1 – GMII: 1GBit/s GMIIMODE can only be changed if RX_EN=0 and TX_EN=0. Deactivation delays of RX_EN and TX_EN have to be considered. GMIIMODE can only be changed, if these register bits can be read as 0. It is possible to change GMIIMODE together with the activation of RX_EN and TX_EN. GMIIMODE drives the outputs <tx_gmiimode> and <rx_gmiimode>."]
        #[inline(always)]
        pub const fn set_gmiimode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Selection of the PHY (See Chapter 4.6.) 00 – MII 01 – GMII 10 – RGMII 11 – reserved PHYSEL can only be changed if RX_EN=0 and TX_EN=0. Deactivation delays of RX_EN and TX_EN have to be considered. PHYSEL can only be changed, if these register bits can be read as 0. It is possible to change PHYSEL together with the activation of RX_EN and TX_EN. PHYSEL drives the output <rx_physel>."]
        #[must_use]
        #[inline(always)]
        pub const fn physel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Selection of the PHY (See Chapter 4.6.) 00 – MII 01 – GMII 10 – RGMII 11 – reserved PHYSEL can only be changed if RX_EN=0 and TX_EN=0. Deactivation delays of RX_EN and TX_EN have to be considered. PHYSEL can only be changed, if these register bits can be read as 0. It is possible to change PHYSEL together with the activation of RX_EN and TX_EN. PHYSEL drives the output <rx_physel>."]
        #[inline(always)]
        pub const fn set_physel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "TX path clock selector 000 – <mii_clk> 001 – <ref_clk> (recommended setting for this selection) 010 – <ref_clk> divided by 5 011 – <ref_clk> divided by 10 100 – <ref_clk> divided by 50 111 – <ref_clk> and enables modification of RCE and MCE others – <ref_clk> See Chapter 7 for further details. CLKSEL is write-locked if CSA=1."]
        #[must_use]
        #[inline(always)]
        pub const fn clksel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "TX path clock selector 000 – <mii_clk> 001 – <ref_clk> (recommended setting for this selection) 010 – <ref_clk> divided by 5 011 – <ref_clk> divided by 10 100 – <ref_clk> divided by 50 111 – <ref_clk> and enables modification of RCE and MCE others – <ref_clk> See Chapter 7 for further details. CLKSEL is write-locked if CSA=1."]
        #[inline(always)]
        pub const fn set_clksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "<mii_clk> enable 0 – disabled 1 – enabled MCE can only be modified if CLKSEL=111. See Chapter 7.3.3 for further details."]
        #[must_use]
        #[inline(always)]
        pub const fn mce(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "<mii_clk> enable 0 – disabled 1 – enabled MCE can only be modified if CLKSEL=111. See Chapter 7.3.3 for further details."]
        #[inline(always)]
        pub const fn set_mce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "<ref_clk> enable 0 – disabled 1 – enabled RCE can only be modified if CLKSEL=111. See Chapter 7.3.3 for further details."]
        #[must_use]
        #[inline(always)]
        pub const fn rce(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "<ref_clk> enable 0 – disabled 1 – enabled RCE can only be modified if CLKSEL=111. See Chapter 7.3.3 for further details."]
        #[inline(always)]
        pub const fn set_rce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Clock switching active (<tx_clk>) 0 – not active 1 – active Switching of <tx_clk> is commanded if CLKSEL or FSTIM (see Table 11-1) are written. Clock switching takes a few clock cycles and this is signaled with CSA=1. When CSA=1 then CLKSEL and FSTIM are write-locked and cannot be changed."]
        #[must_use]
        #[inline(always)]
        pub const fn csa(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Clock switching active (<tx_clk>) 0 – not active 1 – active Switching of <tx_clk> is commanded if CLKSEL or FSTIM (see Table 11-1) are written. Clock switching takes a few clock cycles and this is signaled with CSA=1. When CSA=1 then CLKSEL and FSTIM are write-locked and cannot be changed."]
        #[inline(always)]
        pub const fn set_csa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Safety Enable 0 – disabled 1 – enabled If enabled, then two instances of the logic core of LLEMAC-1G are compared at runtime to each other. SEN can only be changed if RX_EN and TX_EN can be read as 0. Deactivation delays of RX_EN and TX_EN have to be considered. It is possible to change SEN together with the activation of RX_EN and TX_EN."]
        #[must_use]
        #[inline(always)]
        pub const fn sen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Safety Enable 0 – disabled 1 – enabled If enabled, then two instances of the logic core of LLEMAC-1G are compared at runtime to each other. SEN can only be changed if RX_EN and TX_EN can be read as 0. Deactivation delays of RX_EN and TX_EN have to be considered. It is possible to change SEN together with the activation of RX_EN and TX_EN."]
        #[inline(always)]
        pub const fn set_sen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "<mii_clk> active 0 – not active 1 – active See chapter 11.2.3 for details."]
        #[must_use]
        #[inline(always)]
        pub const fn mca(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "<mii_clk> active 0 – not active 1 – active See chapter 11.2.3 for details."]
        #[inline(always)]
        pub const fn set_mca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "<ref_clk> active 0 – not active 1 – active See chapter 11.2.3 for details."]
        #[must_use]
        #[inline(always)]
        pub const fn rca(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "<ref_clk> active 0 – not active 1 – active See chapter 11.2.3 for details."]
        #[inline(always)]
        pub const fn set_rca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Fault Stimulation See Chapter 11.3, Table 11-1 for details. FSTIM is write-locked if CSA=1."]
        #[must_use]
        #[inline(always)]
        pub const fn fstim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Fault Stimulation See Chapter 11.3, Table 11-1 for details. FSTIM is write-locked if CSA=1."]
        #[inline(always)]
        pub const fn set_fstim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for MacMacCtrl {
        #[inline(always)]
        fn default() -> MacMacCtrl {
            MacMacCtrl(0)
        }
    }
    impl core::fmt::Debug for MacMacCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacCtrl")
                .field("resstat", &self.resstat())
                .field("rx_en", &self.rx_en())
                .field("tx_en", &self.tx_en())
                .field("jumbo", &self.jumbo())
                .field("gmiimode", &self.gmiimode())
                .field("physel", &self.physel())
                .field("clksel", &self.clksel())
                .field("mce", &self.mce())
                .field("rce", &self.rce())
                .field("csa", &self.csa())
                .field("sen", &self.sen())
                .field("mca", &self.mca())
                .field("rca", &self.rca())
                .field("fstim", &self.fstim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacMacCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MacMacCtrl {{ resstat: {=bool:?}, rx_en: {=bool:?}, tx_en: {=bool:?}, jumbo: {=bool:?}, gmiimode: {=bool:?}, physel: {=u8:?}, clksel: {=u8:?}, mce: {=bool:?}, rce: {=bool:?}, csa: {=bool:?}, sen: {=bool:?}, mca: {=bool:?}, rca: {=bool:?}, fstim: {=u8:?} }}" , self . resstat () , self . rx_en () , self . tx_en () , self . jumbo () , self . gmiimode () , self . physel () , self . clksel () , self . mce () , self . rce () , self . csa () , self . sen () , self . mca () , self . rca () , self . fstim ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacaddrH(pub u32);
    impl MacMacaddrH {
        #[doc = "MAC address (see Chapter 4.1) Upper bits of MAC address (47:32). MACADDR can only be modified if TX_EN=0 and RX_EN=0."]
        #[must_use]
        #[inline(always)]
        pub const fn macaddr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC address (see Chapter 4.1) Upper bits of MAC address (47:32). MACADDR can only be modified if TX_EN=0 and RX_EN=0."]
        #[inline(always)]
        pub const fn set_macaddr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "0 – disabled 1 – enabled If promiscuous mode is enabled, then reception of all frames independent from the Ethernet destination address is enabled. PROMISC can be changed at any time."]
        #[must_use]
        #[inline(always)]
        pub const fn promisc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "0 – disabled 1 – enabled If promiscuous mode is enabled, then reception of all frames independent from the Ethernet destination address is enabled. PROMISC can be changed at any time."]
        #[inline(always)]
        pub const fn set_promisc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for MacMacaddrH {
        #[inline(always)]
        fn default() -> MacMacaddrH {
            MacMacaddrH(0)
        }
    }
    impl core::fmt::Debug for MacMacaddrH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacaddrH")
                .field("macaddr", &self.macaddr())
                .field("promisc", &self.promisc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacMacaddrH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MacMacaddrH {{ macaddr: {=u16:?}, promisc: {=bool:?} }}",
                self.macaddr(),
                self.promisc()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMacaddrL(pub u32);
    impl MacMacaddrL {
        #[doc = "MAC address Lower bits of MAC address (31:0). MACADDR only be modified if TX_EN=0 and RX_EN=0."]
        #[must_use]
        #[inline(always)]
        pub const fn macaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC address Lower bits of MAC address (31:0). MACADDR only be modified if TX_EN=0 and RX_EN=0."]
        #[inline(always)]
        pub const fn set_macaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacMacaddrL {
        #[inline(always)]
        fn default() -> MacMacaddrL {
            MacMacaddrL(0)
        }
    }
    impl core::fmt::Debug for MacMacaddrL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMacaddrL")
                .field("macaddr", &self.macaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacMacaddrL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacMacaddrL {{ macaddr: {=u32:?} }}", self.macaddr())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMdioCfg(pub u32);
    impl MacMdioCfg {
        #[doc = "Clock Divider to configure MDC clock frequency. Refer to 10.1 Clock Divider for more details."]
        #[must_use]
        #[inline(always)]
        pub const fn mdc_clkdiv(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Clock Divider to configure MDC clock frequency. Refer to 10.1 Clock Divider for more details."]
        #[inline(always)]
        pub const fn set_mdc_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Enable the MDIO controller. If the controller is enabled then MDC will be toggled. ENABLE can only be read as 1 if a valid MDC_CLKDIV value is set."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the MDIO controller. If the controller is enabled then MDC will be toggled. ENABLE can only be read as 1 if a valid MDC_CLKDIV value is set."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "No Preamble With NPRE=1 the preamble generation is suppressed and frames are initiated with Start of Frame pattern directly. Suitable in case that all connected PHYs accept management frames without a preamble pattern. Recommended to be used if only one PHY is connected."]
        #[must_use]
        #[inline(always)]
        pub const fn npre(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "No Preamble With NPRE=1 the preamble generation is suppressed and frames are initiated with Start of Frame pattern directly. Suitable in case that all connected PHYs accept management frames without a preamble pattern. Recommended to be used if only one PHY is connected."]
        #[inline(always)]
        pub const fn set_npre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for MacMdioCfg {
        #[inline(always)]
        fn default() -> MacMdioCfg {
            MacMdioCfg(0)
        }
    }
    impl core::fmt::Debug for MacMdioCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMdioCfg")
                .field("mdc_clkdiv", &self.mdc_clkdiv())
                .field("enable", &self.enable())
                .field("npre", &self.npre())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacMdioCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MacMdioCfg {{ mdc_clkdiv: {=u8:?}, enable: {=bool:?}, npre: {=bool:?} }}",
                self.mdc_clkdiv(),
                self.enable(),
                self.npre()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMdioCtrl(pub u32);
    impl MacMdioCtrl {
        #[doc = "READY=1 indicates a finished transfer and also shows that the controller is ready for a new transfer. READY=1 is only possible if ENABLE=1. If READY=1 is signaled after a read transfer, then RD_DATA is valid until a new transfer is started."]
        #[must_use]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "READY=1 indicates a finished transfer and also shows that the controller is ready for a new transfer. READY=1 is only possible if ENABLE=1. If READY=1 is signaled after a read transfer, then RD_DATA is valid until a new transfer is started."]
        #[inline(always)]
        pub const fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "INIT=1 results in a MDIO write/read transfer if READY=1. If READY=0 while a transfer is already pending or if ENABLE=0 then settings INIT=1 has no effect and the current transaction is withdrawn."]
        #[must_use]
        #[inline(always)]
        pub const fn init(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "INIT=1 results in a MDIO write/read transfer if READY=1. If READY=0 while a transfer is already pending or if ENABLE=0 then settings INIT=1 has no effect and the current transaction is withdrawn."]
        #[inline(always)]
        pub const fn set_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Management Frame Register Address."]
        #[must_use]
        #[inline(always)]
        pub const fn regad(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Management Frame Register Address."]
        #[inline(always)]
        pub const fn set_regad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Management Frame PHY Address."]
        #[must_use]
        #[inline(always)]
        pub const fn phyad(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Management Frame PHY Address."]
        #[inline(always)]
        pub const fn set_phyad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[doc = "Opcode to determine transfer type 01 – Write Access 10 – Read Access."]
        #[must_use]
        #[inline(always)]
        pub const fn op(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Opcode to determine transfer type 01 – Write Access 10 – Read Access."]
        #[inline(always)]
        pub const fn set_op(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for MacMdioCtrl {
        #[inline(always)]
        fn default() -> MacMdioCtrl {
            MacMdioCtrl(0)
        }
    }
    impl core::fmt::Debug for MacMdioCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMdioCtrl")
                .field("ready", &self.ready())
                .field("init", &self.init())
                .field("regad", &self.regad())
                .field("phyad", &self.phyad())
                .field("op", &self.op())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacMdioCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MacMdioCtrl {{ ready: {=bool:?}, init: {=bool:?}, regad: {=u8:?}, phyad: {=u8:?}, op: {=u8:?} }}" , self . ready () , self . init () , self . regad () , self . phyad () , self . op ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMdioRdData(pub u32);
    impl MacMdioRdData {
        #[doc = "Read Data is available if READY=1 after a transfer has been started. RD_DATA represents the content of the management data field of the read transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Read Data is available if READY=1 after a transfer has been started. RD_DATA represents the content of the management data field of the read transfer."]
        #[inline(always)]
        pub const fn set_rd_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MacMdioRdData {
        #[inline(always)]
        fn default() -> MacMdioRdData {
            MacMdioRdData(0)
        }
    }
    impl core::fmt::Debug for MacMdioRdData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMdioRdData")
                .field("rd_data", &self.rd_data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacMdioRdData {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacMdioRdData {{ rd_data: {=u16:?} }}", self.rd_data())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacMdioWrData(pub u32);
    impl MacMdioWrData {
        #[doc = "Data is used for the management data field after a write transfer has been started."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Data is used for the management data field after a write transfer has been started."]
        #[inline(always)]
        pub const fn set_wr_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for MacMdioWrData {
        #[inline(always)]
        fn default() -> MacMdioWrData {
            MacMdioWrData(0)
        }
    }
    impl core::fmt::Debug for MacMdioWrData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacMdioWrData")
                .field("wr_data", &self.wr_data())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacMdioWrData {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacMdioWrData {{ wr_data: {=u16:?} }}", self.wr_data())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacRxFrames(pub u32);
    impl MacRxFrames {
        #[doc = "Number of successfully received frames."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_frames(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of successfully received frames."]
        #[inline(always)]
        pub const fn set_rx_frames(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacRxFrames {
        #[inline(always)]
        fn default() -> MacRxFrames {
            MacRxFrames(0)
        }
    }
    impl core::fmt::Debug for MacRxFrames {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacRxFrames")
                .field("rx_frames", &self.rx_frames())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacRxFrames {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacRxFrames {{ rx_frames: {=u32:?} }}", self.rx_frames())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacRxOctets(pub u32);
    impl MacRxOctets {
        #[doc = "Number of successfully received payload and padding octets."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_octets(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of successfully received payload and padding octets."]
        #[inline(always)]
        pub const fn set_rx_octets(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacRxOctets {
        #[inline(always)]
        fn default() -> MacRxOctets {
            MacRxOctets(0)
        }
    }
    impl core::fmt::Debug for MacRxOctets {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacRxOctets")
                .field("rx_octets", &self.rx_octets())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacRxOctets {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacRxOctets {{ rx_octets: {=u32:?} }}", self.rx_octets())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacTxFrames(pub u32);
    impl MacTxFrames {
        #[doc = "Number of successfully transmitted frames."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_frames(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of successfully transmitted frames."]
        #[inline(always)]
        pub const fn set_tx_frames(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacTxFrames {
        #[inline(always)]
        fn default() -> MacTxFrames {
            MacTxFrames(0)
        }
    }
    impl core::fmt::Debug for MacTxFrames {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacTxFrames")
                .field("tx_frames", &self.tx_frames())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacTxFrames {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacTxFrames {{ tx_frames: {=u32:?} }}", self.tx_frames())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacTxOctets(pub u32);
    impl MacTxOctets {
        #[doc = "Number of successfully transmitted payload and padding octets."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_octets(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of successfully transmitted payload and padding octets."]
        #[inline(always)]
        pub const fn set_tx_octets(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MacTxOctets {
        #[inline(always)]
        fn default() -> MacTxOctets {
            MacTxOctets(0)
        }
    }
    impl core::fmt::Debug for MacTxOctets {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacTxOctets")
                .field("tx_octets", &self.tx_octets())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacTxOctets {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacTxOctets {{ tx_octets: {=u32:?} }}", self.tx_octets())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacVer(pub u32);
    impl MacVer {
        #[doc = "Minor version number (lower part of the version)."]
        #[must_use]
        #[inline(always)]
        pub const fn ver_l(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Minor version number (lower part of the version)."]
        #[inline(always)]
        pub const fn set_ver_l(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Major version number (higher part of the version)."]
        #[must_use]
        #[inline(always)]
        pub const fn ver_h(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Major version number (higher part of the version)."]
        #[inline(always)]
        pub const fn set_ver_h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for MacVer {
        #[inline(always)]
        fn default() -> MacVer {
            MacVer(0)
        }
    }
    impl core::fmt::Debug for MacVer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MacVer")
                .field("ver_l", &self.ver_l())
                .field("ver_h", &self.ver_h())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MacVer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MacVer {{ ver_l: {=u16:?}, ver_h: {=u16:?} }}",
                self.ver_l(),
                self.ver_h()
            )
        }
    }
    #[doc = "mm2s axi address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mm2sAddrlo(pub u32);
    impl Mm2sAddrlo {
        #[doc = "axi address."]
        #[must_use]
        #[inline(always)]
        pub const fn addrlo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "axi address."]
        #[inline(always)]
        pub const fn set_addrlo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mm2sAddrlo {
        #[inline(always)]
        fn default() -> Mm2sAddrlo {
            Mm2sAddrlo(0)
        }
    }
    impl core::fmt::Debug for Mm2sAddrlo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mm2sAddrlo")
                .field("addrlo", &self.addrlo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mm2sAddrlo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mm2sAddrlo {{ addrlo: {=u32:?} }}", self.addrlo())
        }
    }
    #[doc = "mm2s command control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mm2sCtrl(pub u32);
    impl Mm2sCtrl {
        #[doc = "command id."]
        #[must_use]
        #[inline(always)]
        pub const fn id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "command id."]
        #[inline(always)]
        pub const fn set_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "no generation of TLAST."]
        #[must_use]
        #[inline(always)]
        pub const fn ngenlast(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "no generation of TLAST."]
        #[inline(always)]
        pub const fn set_ngenlast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "commit buffered descriptor to command queue."]
        #[must_use]
        #[inline(always)]
        pub const fn go(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "commit buffered descriptor to command queue."]
        #[inline(always)]
        pub const fn set_go(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Mm2sCtrl {
        #[inline(always)]
        fn default() -> Mm2sCtrl {
            Mm2sCtrl(0)
        }
    }
    impl core::fmt::Debug for Mm2sCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mm2sCtrl")
                .field("id", &self.id())
                .field("ngenlast", &self.ngenlast())
                .field("go", &self.go())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mm2sCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mm2sCtrl {{ id: {=u8:?}, ngenlast: {=bool:?}, go: {=bool:?} }}",
                self.id(),
                self.ngenlast(),
                self.go()
            )
        }
    }
    #[doc = "mm2s dma configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mm2sDmaCfg(pub u32);
    impl Mm2sDmaCfg {
        #[doc = "ip version."]
        #[must_use]
        #[inline(always)]
        pub const fn ver(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ip version."]
        #[inline(always)]
        pub const fn set_ver(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "axi data bus width."]
        #[must_use]
        #[inline(always)]
        pub const fn asize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "axi data bus width."]
        #[inline(always)]
        pub const fn set_asize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "enable support for 64 bit addressing."]
        #[must_use]
        #[inline(always)]
        pub const fn ena64(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "enable support for 64 bit addressing."]
        #[inline(always)]
        pub const fn set_ena64(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "command buffer depth."]
        #[must_use]
        #[inline(always)]
        pub const fn cbufd(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "command buffer depth."]
        #[inline(always)]
        pub const fn set_cbufd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "data buffer depth."]
        #[must_use]
        #[inline(always)]
        pub const fn dbufd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "data buffer depth."]
        #[inline(always)]
        pub const fn set_dbufd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Mm2sDmaCfg {
        #[inline(always)]
        fn default() -> Mm2sDmaCfg {
            Mm2sDmaCfg(0)
        }
    }
    impl core::fmt::Debug for Mm2sDmaCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mm2sDmaCfg")
                .field("ver", &self.ver())
                .field("asize", &self.asize())
                .field("ena64", &self.ena64())
                .field("cbufd", &self.cbufd())
                .field("dbufd", &self.dbufd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mm2sDmaCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mm2sDmaCfg {{ ver: {=u16:?}, asize: {=u8:?}, ena64: {=bool:?}, cbufd: {=u8:?}, dbufd: {=u8:?} }}" , self . ver () , self . asize () , self . ena64 () , self . cbufd () , self . dbufd ())
        }
    }
    #[doc = "mm2s control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mm2sDmaCr(pub u32);
    impl Mm2sDmaCr {
        #[doc = "run command from queue to data mover."]
        #[must_use]
        #[inline(always)]
        pub const fn run(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "run command from queue to data mover."]
        #[inline(always)]
        pub const fn set_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "stop on error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn soe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "stop on error flag."]
        #[inline(always)]
        pub const fn set_soe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "do reset when active."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "do reset when active."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "interrupt request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn irqen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt request enable."]
        #[inline(always)]
        pub const fn set_irqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "max axi burst size."]
        #[must_use]
        #[inline(always)]
        pub const fn mxlen(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "max axi burst size."]
        #[inline(always)]
        pub const fn set_mxlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Mm2sDmaCr {
        #[inline(always)]
        fn default() -> Mm2sDmaCr {
            Mm2sDmaCr(0)
        }
    }
    impl core::fmt::Debug for Mm2sDmaCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mm2sDmaCr")
                .field("run", &self.run())
                .field("soe", &self.soe())
                .field("reset", &self.reset())
                .field("irqen", &self.irqen())
                .field("mxlen", &self.mxlen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mm2sDmaCr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mm2sDmaCr {{ run: {=bool:?}, soe: {=bool:?}, reset: {=bool:?}, irqen: {=bool:?}, mxlen: {=u8:?} }}" , self . run () , self . soe () , self . reset () , self . irqen () , self . mxlen ())
        }
    }
    #[doc = "mm2s dma fill status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mm2sDmaFill(pub u32);
    impl Mm2sDmaFill {
        #[doc = "command buffer fill level."]
        #[must_use]
        #[inline(always)]
        pub const fn cfill(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "command buffer fill level."]
        #[inline(always)]
        pub const fn set_cfill(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "response buffer fill level."]
        #[must_use]
        #[inline(always)]
        pub const fn rfill(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "response buffer fill level."]
        #[inline(always)]
        pub const fn set_rfill(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Mm2sDmaFill {
        #[inline(always)]
        fn default() -> Mm2sDmaFill {
            Mm2sDmaFill(0)
        }
    }
    impl core::fmt::Debug for Mm2sDmaFill {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mm2sDmaFill")
                .field("cfill", &self.cfill())
                .field("rfill", &self.rfill())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mm2sDmaFill {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mm2sDmaFill {{ cfill: {=u16:?}, rfill: {=u16:?} }}",
                self.cfill(),
                self.rfill()
            )
        }
    }
    #[doc = "mm2s status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mm2sDmaSr(pub u32);
    impl Mm2sDmaSr {
        #[doc = "mm2s is stopped."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "mm2s is stopped."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "busy."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "busy."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "resetting status."]
        #[must_use]
        #[inline(always)]
        pub const fn rset(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "resetting status."]
        #[inline(always)]
        pub const fn set_rset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "interrupt request pending."]
        #[must_use]
        #[inline(always)]
        pub const fn irq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt request pending."]
        #[inline(always)]
        pub const fn set_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "command buffer empty."]
        #[must_use]
        #[inline(always)]
        pub const fn cbufe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "command buffer empty."]
        #[inline(always)]
        pub const fn set_cbufe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "command buffer full."]
        #[must_use]
        #[inline(always)]
        pub const fn cbuff(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "command buffer full."]
        #[inline(always)]
        pub const fn set_cbuff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "response buffer empty."]
        #[must_use]
        #[inline(always)]
        pub const fn rbufe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "response buffer empty."]
        #[inline(always)]
        pub const fn set_rbufe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "response buffer full."]
        #[must_use]
        #[inline(always)]
        pub const fn rbuff(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "response buffer full."]
        #[inline(always)]
        pub const fn set_rbuff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Mm2sDmaSr {
        #[inline(always)]
        fn default() -> Mm2sDmaSr {
            Mm2sDmaSr(0)
        }
    }
    impl core::fmt::Debug for Mm2sDmaSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mm2sDmaSr")
                .field("stop", &self.stop())
                .field("busy", &self.busy())
                .field("rset", &self.rset())
                .field("irq", &self.irq())
                .field("cbufe", &self.cbufe())
                .field("cbuff", &self.cbuff())
                .field("rbufe", &self.rbufe())
                .field("rbuff", &self.rbuff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mm2sDmaSr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mm2sDmaSr {{ stop: {=bool:?}, busy: {=bool:?}, rset: {=bool:?}, irq: {=bool:?}, cbufe: {=bool:?}, cbuff: {=bool:?}, rbufe: {=bool:?}, rbuff: {=bool:?} }}" , self . stop () , self . busy () , self . rset () , self . irq () , self . cbufe () , self . cbuff () , self . rbufe () , self . rbuff ())
        }
    }
    #[doc = "mm2s axi length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mm2sLength(pub u32);
    impl Mm2sLength {
        #[doc = "transfer request length in bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn length(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "transfer request length in bytes."]
        #[inline(always)]
        pub const fn set_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Mm2sLength {
        #[inline(always)]
        fn default() -> Mm2sLength {
            Mm2sLength(0)
        }
    }
    impl core::fmt::Debug for Mm2sLength {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mm2sLength")
                .field("length", &self.length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mm2sLength {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mm2sLength {{ length: {=u16:?} }}", self.length())
        }
    }
    #[doc = "mm2s response buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mm2sResp(pub u32);
    impl Mm2sResp {
        #[doc = "requested length of tansfer in bytes from command."]
        #[must_use]
        #[inline(always)]
        pub const fn length(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "requested length of tansfer in bytes from command."]
        #[inline(always)]
        pub const fn set_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "command ID feedback."]
        #[must_use]
        #[inline(always)]
        pub const fn id(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "command ID feedback."]
        #[inline(always)]
        pub const fn set_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "slave error."]
        #[must_use]
        #[inline(always)]
        pub const fn slverr(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "slave error."]
        #[inline(always)]
        pub const fn set_slverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "decode error."]
        #[must_use]
        #[inline(always)]
        pub const fn decerr(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "decode error."]
        #[inline(always)]
        pub const fn set_decerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "axi-stream with TLAST."]
        #[must_use]
        #[inline(always)]
        pub const fn last(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "axi-stream with TLAST."]
        #[inline(always)]
        pub const fn set_last(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Mm2sResp {
        #[inline(always)]
        fn default() -> Mm2sResp {
            Mm2sResp(0)
        }
    }
    impl core::fmt::Debug for Mm2sResp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mm2sResp")
                .field("length", &self.length())
                .field("id", &self.id())
                .field("slverr", &self.slverr())
                .field("decerr", &self.decerr())
                .field("last", &self.last())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mm2sResp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Mm2sResp {{ length: {=u16:?}, id: {=u8:?}, slverr: {=bool:?}, decerr: {=bool:?}, last: {=bool:?} }}" , self . length () , self . id () , self . slverr () , self . decerr () , self . last ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxBc(pub u32);
    impl MonitorRxCounterRxBc {
        #[doc = "Number of Broadcast frames."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_bc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of Broadcast frames."]
        #[inline(always)]
        pub const fn set_rx_bc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxBc {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxBc {
            MonitorRxCounterRxBc(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxBc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxBc")
                .field("rx_bc", &self.rx_bc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxBc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxBc {{ rx_bc: {=u32:?} }}",
                self.rx_bc()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxDropErr(pub u32);
    impl MonitorRxCounterRxDropErr {
        #[doc = "Dropped frames with error by ingress. Possible in S&F mode or when frame is queued in ingress."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_drop_err(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Dropped frames with error by ingress. Possible in S&F mode or when frame is queued in ingress."]
        #[inline(always)]
        pub const fn set_rx_drop_err(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxDropErr {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxDropErr {
            MonitorRxCounterRxDropErr(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxDropErr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxDropErr")
                .field("rx_drop_err", &self.rx_drop_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxDropErr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxDropErr {{ rx_drop_err: {=u32:?} }}",
                self.rx_drop_err()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxDropLu(pub u32);
    impl MonitorRxCounterRxDropLu {
        #[doc = "Dropped frames by LookUp decision."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_drop_lu(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Dropped frames by LookUp decision."]
        #[inline(always)]
        pub const fn set_rx_drop_lu(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxDropLu {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxDropLu {
            MonitorRxCounterRxDropLu(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxDropLu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxDropLu")
                .field("rx_drop_lu", &self.rx_drop_lu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxDropLu {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxDropLu {{ rx_drop_lu: {=u32:?} }}",
                self.rx_drop_lu()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxDropOvfl(pub u32);
    impl MonitorRxCounterRxDropOvfl {
        #[doc = "Dropped frames by ingress overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_drop_ovfl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Dropped frames by ingress overflow."]
        #[inline(always)]
        pub const fn set_rx_drop_ovfl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxDropOvfl {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxDropOvfl {
            MonitorRxCounterRxDropOvfl(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxDropOvfl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxDropOvfl")
                .field("rx_drop_ovfl", &self.rx_drop_ovfl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxDropOvfl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxDropOvfl {{ rx_drop_ovfl: {=u32:?} }}",
                self.rx_drop_ovfl()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxDropVlan(pub u32);
    impl MonitorRxCounterRxDropVlan {
        #[doc = "Dropped frames by incompatible VLAN."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_drop_vlan(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Dropped frames by incompatible VLAN."]
        #[inline(always)]
        pub const fn set_rx_drop_vlan(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxDropVlan {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxDropVlan {
            MonitorRxCounterRxDropVlan(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxDropVlan {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxDropVlan")
                .field("rx_drop_vlan", &self.rx_drop_vlan())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxDropVlan {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxDropVlan {{ rx_drop_vlan: {=u32:?} }}",
                self.rx_drop_vlan()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxFerror(pub u32);
    impl MonitorRxCounterRxFerror {
        #[doc = "Bad received frame by ingress buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_ferror(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bad received frame by ingress buffer."]
        #[inline(always)]
        pub const fn set_rx_ferror(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxFerror {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxFerror {
            MonitorRxCounterRxFerror(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxFerror {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxFerror")
                .field("rx_ferror", &self.rx_ferror())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxFerror {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxFerror {{ rx_ferror: {=u32:?} }}",
                self.rx_ferror()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxFgood(pub u32);
    impl MonitorRxCounterRxFgood {
        #[doc = "Good received frame by ingress buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_fgood(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Good received frame by ingress buffer."]
        #[inline(always)]
        pub const fn set_rx_fgood(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxFgood {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxFgood {
            MonitorRxCounterRxFgood(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxFgood {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxFgood")
                .field("rx_fgood", &self.rx_fgood())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxFgood {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxFgood {{ rx_fgood: {=u32:?} }}",
                self.rx_fgood()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxFpeFgood(pub u32);
    impl MonitorRxCounterRxFpeFgood {
        #[doc = "Number of preemptable frames. Subset of RX_FGOOD."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_fpe_fgood(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of preemptable frames. Subset of RX_FGOOD."]
        #[inline(always)]
        pub const fn set_rx_fpe_fgood(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxFpeFgood {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxFpeFgood {
            MonitorRxCounterRxFpeFgood(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxFpeFgood {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxFpeFgood")
                .field("rx_fpe_fgood", &self.rx_fpe_fgood())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxFpeFgood {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxFpeFgood {{ rx_fpe_fgood: {=u32:?} }}",
                self.rx_fpe_fgood()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxIntern(pub u32);
    impl MonitorRxCounterRxIntern {
        #[doc = "Number of non-relay frames."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_intern(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of non-relay frames."]
        #[inline(always)]
        pub const fn set_rx_intern(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxIntern {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxIntern {
            MonitorRxCounterRxIntern(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxIntern {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxIntern")
                .field("rx_intern", &self.rx_intern())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxIntern {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxIntern {{ rx_intern: {=u32:?} }}",
                self.rx_intern()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxKnown(pub u32);
    impl MonitorRxCounterRxKnown {
        #[doc = "Number of frames passed ingress with hit by MAC Table. This includes Broadcast and non-relayed frames."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_known(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of frames passed ingress with hit by MAC Table. This includes Broadcast and non-relayed frames."]
        #[inline(always)]
        pub const fn set_rx_known(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxKnown {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxKnown {
            MonitorRxCounterRxKnown(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxKnown {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxKnown")
                .field("rx_known", &self.rx_known())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxKnown {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxKnown {{ rx_known: {=u32:?} }}",
                self.rx_known()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxMulti(pub u32);
    impl MonitorRxCounterRxMulti {
        #[doc = "Number of Multicast frames."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_multi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of Multicast frames."]
        #[inline(always)]
        pub const fn set_rx_multi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxMulti {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxMulti {
            MonitorRxCounterRxMulti(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxMulti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxMulti")
                .field("rx_multi", &self.rx_multi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxMulti {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxMulti {{ rx_multi: {=u32:?} }}",
                self.rx_multi()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxUc(pub u32);
    impl MonitorRxCounterRxUc {
        #[doc = "Number of unicast frames."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_uc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of unicast frames."]
        #[inline(always)]
        pub const fn set_rx_uc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxUc {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxUc {
            MonitorRxCounterRxUc(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxUc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxUc")
                .field("rx_uc", &self.rx_uc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxUc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxUc {{ rx_uc: {=u32:?} }}",
                self.rx_uc()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxUnknown(pub u32);
    impl MonitorRxCounterRxUnknown {
        #[doc = "Number of frames passed ingress without hit by MAC table."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_unknown(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of frames passed ingress without hit by MAC table."]
        #[inline(always)]
        pub const fn set_rx_unknown(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxUnknown {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxUnknown {
            MonitorRxCounterRxUnknown(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxUnknown {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxUnknown")
                .field("rx_unknown", &self.rx_unknown())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxUnknown {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxUnknown {{ rx_unknown: {=u32:?} }}",
                self.rx_unknown()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorRxCounterRxVlan(pub u32);
    impl MonitorRxCounterRxVlan {
        #[doc = "Number of VLAN tagged frames."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_vlan(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of VLAN tagged frames."]
        #[inline(always)]
        pub const fn set_rx_vlan(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorRxCounterRxVlan {
        #[inline(always)]
        fn default() -> MonitorRxCounterRxVlan {
            MonitorRxCounterRxVlan(0)
        }
    }
    impl core::fmt::Debug for MonitorRxCounterRxVlan {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorRxCounterRxVlan")
                .field("rx_vlan", &self.rx_vlan())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorRxCounterRxVlan {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorRxCounterRxVlan {{ rx_vlan: {=u32:?} }}",
                self.rx_vlan()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorTxCounterTxDropOvfl(pub u32);
    impl MonitorTxCounterTxDropOvfl {
        #[doc = "Dropped frames by full queue of TSN-EP."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_drop_ovfl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Dropped frames by full queue of TSN-EP."]
        #[inline(always)]
        pub const fn set_tx_drop_ovfl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorTxCounterTxDropOvfl {
        #[inline(always)]
        fn default() -> MonitorTxCounterTxDropOvfl {
            MonitorTxCounterTxDropOvfl(0)
        }
    }
    impl core::fmt::Debug for MonitorTxCounterTxDropOvfl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorTxCounterTxDropOvfl")
                .field("tx_drop_ovfl", &self.tx_drop_ovfl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorTxCounterTxDropOvfl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorTxCounterTxDropOvfl {{ tx_drop_ovfl: {=u32:?} }}",
                self.tx_drop_ovfl()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorTxCounterTxFerror(pub u32);
    impl MonitorTxCounterTxFerror {
        #[doc = "Transmitted Frames with Error to TX TSN-EP."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_ferror(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmitted Frames with Error to TX TSN-EP."]
        #[inline(always)]
        pub const fn set_tx_ferror(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorTxCounterTxFerror {
        #[inline(always)]
        fn default() -> MonitorTxCounterTxFerror {
            MonitorTxCounterTxFerror(0)
        }
    }
    impl core::fmt::Debug for MonitorTxCounterTxFerror {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorTxCounterTxFerror")
                .field("tx_ferror", &self.tx_ferror())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorTxCounterTxFerror {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorTxCounterTxFerror {{ tx_ferror: {=u32:?} }}",
                self.tx_ferror()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MonitorTxCounterTxFgood(pub u32);
    impl MonitorTxCounterTxFgood {
        #[doc = "Good transmitted Frames to TX TSN-EP."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_fgood(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Good transmitted Frames to TX TSN-EP."]
        #[inline(always)]
        pub const fn set_tx_fgood(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for MonitorTxCounterTxFgood {
        #[inline(always)]
        fn default() -> MonitorTxCounterTxFgood {
            MonitorTxCounterTxFgood(0)
        }
    }
    impl core::fmt::Debug for MonitorTxCounterTxFgood {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MonitorTxCounterTxFgood")
                .field("tx_fgood", &self.tx_fgood())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MonitorTxCounterTxFgood {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MonitorTxCounterTxFgood {{ tx_fgood: {=u32:?} }}",
                self.tx_fgood()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mxsdu(pub u32);
    impl Mxsdu {
        #[doc = "Maximum SDU size for traffic queue n (n = 0 – 7)Returns 0 when n > TQC. Value is size in words (32 bit word size)."]
        #[must_use]
        #[inline(always)]
        pub const fn sdu(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Maximum SDU size for traffic queue n (n = 0 – 7)Returns 0 when n > TQC. Value is size in words (32 bit word size)."]
        #[inline(always)]
        pub const fn set_sdu(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Mxsdu {
        #[inline(always)]
        fn default() -> Mxsdu {
            Mxsdu(0)
        }
    }
    impl core::fmt::Debug for Mxsdu {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mxsdu").field("sdu", &self.sdu()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mxsdu {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mxsdu {{ sdu: {=u16:?} }}", self.sdu())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mxtk(pub u32);
    impl Mxtk {
        #[doc = "Maximum SDU size in clock ticks. MXTKi is only supported when TQC > i, otherwise read-only with value 0."]
        #[must_use]
        #[inline(always)]
        pub const fn tick(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Maximum SDU size in clock ticks. MXTKi is only supported when TQC > i, otherwise read-only with value 0."]
        #[inline(always)]
        pub const fn set_tick(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Mxtk {
        #[inline(always)]
        fn default() -> Mxtk {
            Mxtk(0)
        }
    }
    impl core::fmt::Debug for Mxtk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mxtk").field("tick", &self.tick()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mxtk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Mxtk {{ tick: {=u32:?} }}", self.tick())
        }
    }
    #[doc = "qch channel0 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Port1Qch0Cfg(pub u32);
    impl Port1Qch0Cfg {
        #[doc = "qch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "qch enable."]
        #[inline(always)]
        pub const fn set_cqf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "qch queue in select."]
        #[must_use]
        #[inline(always)]
        pub const fn axis_qch_en(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0xff;
            val as u8
        }
        #[doc = "qch queue in select."]
        #[inline(always)]
        pub const fn set_axis_qch_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
        }
        #[doc = "tas_gpio select."]
        #[must_use]
        #[inline(always)]
        pub const fn tas_gpio_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "tas_gpio select."]
        #[inline(always)]
        pub const fn set_tas_gpio_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "qch queue destination buffer select."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_num(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "qch queue destination buffer select."]
        #[inline(always)]
        pub const fn set_cqf_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "qch queue in error."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_in_err(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "qch queue in error."]
        #[inline(always)]
        pub const fn set_cqf_in_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Port1Qch0Cfg {
        #[inline(always)]
        fn default() -> Port1Qch0Cfg {
            Port1Qch0Cfg(0)
        }
    }
    impl core::fmt::Debug for Port1Qch0Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Port1Qch0Cfg")
                .field("cqf_en", &self.cqf_en())
                .field("axis_qch_en", &self.axis_qch_en())
                .field("tas_gpio_sel", &self.tas_gpio_sel())
                .field("cqf_num", &self.cqf_num())
                .field("cqf_in_err", &self.cqf_in_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Port1Qch0Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Port1Qch0Cfg {{ cqf_en: {=bool:?}, axis_qch_en: {=u8:?}, tas_gpio_sel: {=u8:?}, cqf_num: {=u8:?}, cqf_in_err: {=bool:?} }}" , self . cqf_en () , self . axis_qch_en () , self . tas_gpio_sel () , self . cqf_num () , self . cqf_in_err ())
        }
    }
    #[doc = "qch channel1 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Port1Qch1Cfg(pub u32);
    impl Port1Qch1Cfg {
        #[doc = "qch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "qch enable."]
        #[inline(always)]
        pub const fn set_cqf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "qch queue in select."]
        #[must_use]
        #[inline(always)]
        pub const fn axis_qch_en(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0xff;
            val as u8
        }
        #[doc = "qch queue in select."]
        #[inline(always)]
        pub const fn set_axis_qch_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
        }
        #[doc = "tas_gpio select."]
        #[must_use]
        #[inline(always)]
        pub const fn tas_gpio_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "tas_gpio select."]
        #[inline(always)]
        pub const fn set_tas_gpio_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "qch queue destination buffer select."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_num(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "qch queue destination buffer select."]
        #[inline(always)]
        pub const fn set_cqf_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "qch queue in error."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_in_err(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "qch queue in error."]
        #[inline(always)]
        pub const fn set_cqf_in_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Port1Qch1Cfg {
        #[inline(always)]
        fn default() -> Port1Qch1Cfg {
            Port1Qch1Cfg(0)
        }
    }
    impl core::fmt::Debug for Port1Qch1Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Port1Qch1Cfg")
                .field("cqf_en", &self.cqf_en())
                .field("axis_qch_en", &self.axis_qch_en())
                .field("tas_gpio_sel", &self.tas_gpio_sel())
                .field("cqf_num", &self.cqf_num())
                .field("cqf_in_err", &self.cqf_in_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Port1Qch1Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Port1Qch1Cfg {{ cqf_en: {=bool:?}, axis_qch_en: {=u8:?}, tas_gpio_sel: {=u8:?}, cqf_num: {=u8:?}, cqf_in_err: {=bool:?} }}" , self . cqf_en () , self . axis_qch_en () , self . tas_gpio_sel () , self . cqf_num () , self . cqf_in_err ())
        }
    }
    #[doc = "qch channel2 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Port1Qch2Cfg(pub u32);
    impl Port1Qch2Cfg {
        #[doc = "qch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "qch enable."]
        #[inline(always)]
        pub const fn set_cqf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "qch queue in select."]
        #[must_use]
        #[inline(always)]
        pub const fn axis_qch_en(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0xff;
            val as u8
        }
        #[doc = "qch queue in select."]
        #[inline(always)]
        pub const fn set_axis_qch_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
        }
        #[doc = "tas_gpio select."]
        #[must_use]
        #[inline(always)]
        pub const fn tas_gpio_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "tas_gpio select."]
        #[inline(always)]
        pub const fn set_tas_gpio_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "qch queue destination buffer select."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_num(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "qch queue destination buffer select."]
        #[inline(always)]
        pub const fn set_cqf_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "qch queue in error."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_in_err(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "qch queue in error."]
        #[inline(always)]
        pub const fn set_cqf_in_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Port1Qch2Cfg {
        #[inline(always)]
        fn default() -> Port1Qch2Cfg {
            Port1Qch2Cfg(0)
        }
    }
    impl core::fmt::Debug for Port1Qch2Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Port1Qch2Cfg")
                .field("cqf_en", &self.cqf_en())
                .field("axis_qch_en", &self.axis_qch_en())
                .field("tas_gpio_sel", &self.tas_gpio_sel())
                .field("cqf_num", &self.cqf_num())
                .field("cqf_in_err", &self.cqf_in_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Port1Qch2Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Port1Qch2Cfg {{ cqf_en: {=bool:?}, axis_qch_en: {=u8:?}, tas_gpio_sel: {=u8:?}, cqf_num: {=u8:?}, cqf_in_err: {=bool:?} }}" , self . cqf_en () , self . axis_qch_en () , self . tas_gpio_sel () , self . cqf_num () , self . cqf_in_err ())
        }
    }
    #[doc = "qch channel3 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Port1Qch3Cfg(pub u32);
    impl Port1Qch3Cfg {
        #[doc = "qch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "qch enable."]
        #[inline(always)]
        pub const fn set_cqf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "qch queue in select."]
        #[must_use]
        #[inline(always)]
        pub const fn axis_qch_en(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0xff;
            val as u8
        }
        #[doc = "qch queue in select."]
        #[inline(always)]
        pub const fn set_axis_qch_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
        }
        #[doc = "tas_gpio select."]
        #[must_use]
        #[inline(always)]
        pub const fn tas_gpio_sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "tas_gpio select."]
        #[inline(always)]
        pub const fn set_tas_gpio_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "qch queue destination buffer select."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_num(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "qch queue destination buffer select."]
        #[inline(always)]
        pub const fn set_cqf_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "qch queue in error."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_in_err(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "qch queue in error."]
        #[inline(always)]
        pub const fn set_cqf_in_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Port1Qch3Cfg {
        #[inline(always)]
        fn default() -> Port1Qch3Cfg {
            Port1Qch3Cfg(0)
        }
    }
    impl core::fmt::Debug for Port1Qch3Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Port1Qch3Cfg")
                .field("cqf_en", &self.cqf_en())
                .field("axis_qch_en", &self.axis_qch_en())
                .field("tas_gpio_sel", &self.tas_gpio_sel())
                .field("cqf_num", &self.cqf_num())
                .field("cqf_in_err", &self.cqf_in_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Port1Qch3Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Port1Qch3Cfg {{ cqf_en: {=bool:?}, axis_qch_en: {=u8:?}, tas_gpio_sel: {=u8:?}, cqf_num: {=u8:?}, cqf_in_err: {=bool:?} }}" , self . cqf_en () , self . axis_qch_en () , self . tas_gpio_sel () , self . cqf_num () , self . cqf_in_err ())
        }
    }
    #[doc = "qch clear."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Port1QchErrCfg(pub u32);
    impl Port1QchErrCfg {
        #[doc = "enable cqf buffer auto clear when error."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_clr_ctrl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable cqf buffer auto clear when error."]
        #[inline(always)]
        pub const fn set_cqf_clr_ctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "axis_qch_en config error."]
        #[must_use]
        #[inline(always)]
        pub const fn axis_qch_cfg_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "axis_qch_en config error."]
        #[inline(always)]
        pub const fn set_axis_qch_cfg_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "cqf_num config error."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_num_cfg_err(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "cqf_num config error."]
        #[inline(always)]
        pub const fn set_cqf_num_cfg_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "que gate error for each cqf."]
        #[must_use]
        #[inline(always)]
        pub const fn cqf_que_err(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "que gate error for each cqf."]
        #[inline(always)]
        pub const fn set_cqf_que_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Port1QchErrCfg {
        #[inline(always)]
        fn default() -> Port1QchErrCfg {
            Port1QchErrCfg(0)
        }
    }
    impl core::fmt::Debug for Port1QchErrCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Port1QchErrCfg")
                .field("cqf_clr_ctrl", &self.cqf_clr_ctrl())
                .field("axis_qch_cfg_err", &self.axis_qch_cfg_err())
                .field("cqf_num_cfg_err", &self.cqf_num_cfg_err())
                .field("cqf_que_err", &self.cqf_que_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Port1QchErrCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Port1QchErrCfg {{ cqf_clr_ctrl: {=bool:?}, axis_qch_cfg_err: {=bool:?}, cqf_num_cfg_err: {=bool:?}, cqf_que_err: {=u8:?} }}" , self . cqf_clr_ctrl () , self . axis_qch_cfg_err () , self . cqf_num_cfg_err () , self . cqf_que_err ())
        }
    }
    #[doc = "auxiliray read data seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtAtshi(pub u32);
    impl PtpEvtAtshi {
        #[doc = "auxiliary fifo read seconds info."]
        #[must_use]
        #[inline(always)]
        pub const fn stshi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "auxiliary fifo read seconds info."]
        #[inline(always)]
        pub const fn set_stshi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtAtshi {
        #[inline(always)]
        fn default() -> PtpEvtAtshi {
            PtpEvtAtshi(0)
        }
    }
    impl core::fmt::Debug for PtpEvtAtshi {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtAtshi")
                .field("stshi", &self.stshi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtAtshi {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PtpEvtAtshi {{ stshi: {=u32:?} }}", self.stshi())
        }
    }
    #[doc = "auxiliray read data sub seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtAtslo(pub u32);
    impl PtpEvtAtslo {
        #[doc = "auxiliary fifo read sub seconds info."]
        #[must_use]
        #[inline(always)]
        pub const fn stslo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "auxiliary fifo read sub seconds info."]
        #[inline(always)]
        pub const fn set_stslo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtAtslo {
        #[inline(always)]
        fn default() -> PtpEvtAtslo {
            PtpEvtAtslo(0)
        }
    }
    impl core::fmt::Debug for PtpEvtAtslo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtAtslo")
                .field("stslo", &self.stslo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtAtslo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PtpEvtAtslo {{ stslo: {=u32:?} }}", self.stslo())
        }
    }
    #[doc = "pps0 interval configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtPps0Interval(pub u32);
    impl PtpEvtPps0Interval {
        #[doc = "PPS0 output signal interval."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsint(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS0 output signal interval."]
        #[inline(always)]
        pub const fn set_ppsint(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtPps0Interval {
        #[inline(always)]
        fn default() -> PtpEvtPps0Interval {
            PtpEvtPps0Interval(0)
        }
    }
    impl core::fmt::Debug for PtpEvtPps0Interval {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtPps0Interval")
                .field("ppsint", &self.ppsint())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtPps0Interval {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PtpEvtPps0Interval {{ ppsint: {=u32:?} }}",
                self.ppsint()
            )
        }
    }
    #[doc = "pps0 width configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtPps0Width(pub u32);
    impl PtpEvtPps0Width {
        #[doc = "pps0 output signal width."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_width(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "pps0 output signal width."]
        #[inline(always)]
        pub const fn set_pps_width(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtPps0Width {
        #[inline(always)]
        fn default() -> PtpEvtPps0Width {
            PtpEvtPps0Width(0)
        }
    }
    impl core::fmt::Debug for PtpEvtPps0Width {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtPps0Width")
                .field("pps_width", &self.pps_width())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtPps0Width {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PtpEvtPps0Width {{ pps_width: {=u32:?} }}",
                self.pps_width()
            )
        }
    }
    #[doc = "pps1 interval configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtPps1Interval(pub u32);
    impl PtpEvtPps1Interval {
        #[doc = "PPS1 output signal interval."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsint(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS1 output signal interval."]
        #[inline(always)]
        pub const fn set_ppsint(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtPps1Interval {
        #[inline(always)]
        fn default() -> PtpEvtPps1Interval {
            PtpEvtPps1Interval(0)
        }
    }
    impl core::fmt::Debug for PtpEvtPps1Interval {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtPps1Interval")
                .field("ppsint", &self.ppsint())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtPps1Interval {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PtpEvtPps1Interval {{ ppsint: {=u32:?} }}",
                self.ppsint()
            )
        }
    }
    #[doc = "pps1 width configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtPps1Width(pub u32);
    impl PtpEvtPps1Width {
        #[doc = "pps1 output signal width."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_width(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "pps1 output signal width."]
        #[inline(always)]
        pub const fn set_pps_width(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtPps1Width {
        #[inline(always)]
        fn default() -> PtpEvtPps1Width {
            PtpEvtPps1Width(0)
        }
    }
    impl core::fmt::Debug for PtpEvtPps1Width {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtPps1Width")
                .field("pps_width", &self.pps_width())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtPps1Width {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PtpEvtPps1Width {{ pps_width: {=u32:?} }}",
                self.pps_width()
            )
        }
    }
    #[doc = "pps2 interval configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtPps2Interval(pub u32);
    impl PtpEvtPps2Interval {
        #[doc = "PPS2 output signal interval."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsint(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS2 output signal interval."]
        #[inline(always)]
        pub const fn set_ppsint(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtPps2Interval {
        #[inline(always)]
        fn default() -> PtpEvtPps2Interval {
            PtpEvtPps2Interval(0)
        }
    }
    impl core::fmt::Debug for PtpEvtPps2Interval {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtPps2Interval")
                .field("ppsint", &self.ppsint())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtPps2Interval {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PtpEvtPps2Interval {{ ppsint: {=u32:?} }}",
                self.ppsint()
            )
        }
    }
    #[doc = "pps2 width configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtPps2Width(pub u32);
    impl PtpEvtPps2Width {
        #[doc = "pps2 output signal width."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_width(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "pps2 output signal width."]
        #[inline(always)]
        pub const fn set_pps_width(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtPps2Width {
        #[inline(always)]
        fn default() -> PtpEvtPps2Width {
            PtpEvtPps2Width(0)
        }
    }
    impl core::fmt::Debug for PtpEvtPps2Width {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtPps2Width")
                .field("pps_width", &self.pps_width())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtPps2Width {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PtpEvtPps2Width {{ pps_width: {=u32:?} }}",
                self.pps_width()
            )
        }
    }
    #[doc = "pps3 interval configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtPps3Interval(pub u32);
    impl PtpEvtPps3Interval {
        #[doc = "PPS3 output signal interval."]
        #[must_use]
        #[inline(always)]
        pub const fn ppsint(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PPS3 output signal interval."]
        #[inline(always)]
        pub const fn set_ppsint(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtPps3Interval {
        #[inline(always)]
        fn default() -> PtpEvtPps3Interval {
            PtpEvtPps3Interval(0)
        }
    }
    impl core::fmt::Debug for PtpEvtPps3Interval {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtPps3Interval")
                .field("ppsint", &self.ppsint())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtPps3Interval {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PtpEvtPps3Interval {{ ppsint: {=u32:?} }}",
                self.ppsint()
            )
        }
    }
    #[doc = "pps3 width configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtPps3Width(pub u32);
    impl PtpEvtPps3Width {
        #[doc = "pps3 output signal width."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_width(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "pps3 output signal width."]
        #[inline(always)]
        pub const fn set_pps_width(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtPps3Width {
        #[inline(always)]
        fn default() -> PtpEvtPps3Width {
            PtpEvtPps3Width(0)
        }
    }
    impl core::fmt::Debug for PtpEvtPps3Width {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtPps3Width")
                .field("pps_width", &self.pps_width())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtPps3Width {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PtpEvtPps3Width {{ pps_width: {=u32:?} }}",
                self.pps_width()
            )
        }
    }
    #[doc = "pps command control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtPpsCmd(pub u32);
    impl PtpEvtPpsCmd {
        #[doc = "pps0 command."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_cmd0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "pps0 command."]
        #[inline(always)]
        pub const fn set_pps_cmd0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "flexible PPS0 output mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_en0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "flexible PPS0 output mode enable."]
        #[inline(always)]
        pub const fn set_pps_en0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Target Time Register Mode for PPS0 Output."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_mode0(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Target Time Register Mode for PPS0 Output."]
        #[inline(always)]
        pub const fn set_pps_mode0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "pps1 command."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_cmd1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "pps1 command."]
        #[inline(always)]
        pub const fn set_pps_cmd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Target Time Register Mode for PPS1 Output."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_mode1(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Target Time Register Mode for PPS1 Output."]
        #[inline(always)]
        pub const fn set_pps_mode1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "pps2 command."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_cmd2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "pps2 command."]
        #[inline(always)]
        pub const fn set_pps_cmd2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Target Time Register Mode for PPS2 Output."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_mode2(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[doc = "Target Time Register Mode for PPS2 Output."]
        #[inline(always)]
        pub const fn set_pps_mode2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[doc = "pps3 command."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_cmd3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "pps3 command."]
        #[inline(always)]
        pub const fn set_pps_cmd3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Target Time Register Mode for PPS3 Output."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_mode3(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "Target Time Register Mode for PPS3 Output."]
        #[inline(always)]
        pub const fn set_pps_mode3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for PtpEvtPpsCmd {
        #[inline(always)]
        fn default() -> PtpEvtPpsCmd {
            PtpEvtPpsCmd(0)
        }
    }
    impl core::fmt::Debug for PtpEvtPpsCmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtPpsCmd")
                .field("pps_cmd0", &self.pps_cmd0())
                .field("pps_en0", &self.pps_en0())
                .field("pps_mode0", &self.pps_mode0())
                .field("pps_cmd1", &self.pps_cmd1())
                .field("pps_mode1", &self.pps_mode1())
                .field("pps_cmd2", &self.pps_cmd2())
                .field("pps_mode2", &self.pps_mode2())
                .field("pps_cmd3", &self.pps_cmd3())
                .field("pps_mode3", &self.pps_mode3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtPpsCmd {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PtpEvtPpsCmd {{ pps_cmd0: {=u8:?}, pps_en0: {=bool:?}, pps_mode0: {=u8:?}, pps_cmd1: {=u8:?}, pps_mode1: {=u8:?}, pps_cmd2: {=u8:?}, pps_mode2: {=u8:?}, pps_cmd3: {=u8:?}, pps_mode3: {=u8:?} }}" , self . pps_cmd0 () , self . pps_en0 () , self . pps_mode0 () , self . pps_cmd1 () , self . pps_mode1 () , self . pps_cmd2 () , self . pps_mode2 () , self . pps_cmd3 () , self . pps_mode3 ())
        }
    }
    #[doc = "pps control 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtPpsCtrl0(pub u32);
    impl PtpEvtPpsCtrl0 {
        #[doc = "timer selection."]
        #[must_use]
        #[inline(always)]
        pub const fn time_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "timer selection."]
        #[inline(always)]
        pub const fn set_time_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "auxiliary snapshot fifo write interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_wr_intr_msk(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "auxiliary snapshot fifo write interrupt enable."]
        #[inline(always)]
        pub const fn set_fifo_wr_intr_msk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "target timmer interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn target_rac_intr_msk(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "target timmer interrupt mask."]
        #[inline(always)]
        pub const fn set_target_rac_intr_msk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "pps tod interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_tod_intr_msk(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "pps tod interrupt enable."]
        #[inline(always)]
        pub const fn set_pps_tod_intr_msk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for PtpEvtPpsCtrl0 {
        #[inline(always)]
        fn default() -> PtpEvtPpsCtrl0 {
            PtpEvtPpsCtrl0(0)
        }
    }
    impl core::fmt::Debug for PtpEvtPpsCtrl0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtPpsCtrl0")
                .field("time_sel", &self.time_sel())
                .field("fifo_wr_intr_msk", &self.fifo_wr_intr_msk())
                .field("target_rac_intr_msk", &self.target_rac_intr_msk())
                .field("pps_tod_intr_msk", &self.pps_tod_intr_msk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtPpsCtrl0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PtpEvtPpsCtrl0 {{ time_sel: {=bool:?}, fifo_wr_intr_msk: {=bool:?}, target_rac_intr_msk: {=bool:?}, pps_tod_intr_msk: {=bool:?} }}" , self . time_sel () , self . fifo_wr_intr_msk () , self . target_rac_intr_msk () , self . pps_tod_intr_msk ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtPpsSel(pub u32);
    impl PtpEvtPpsSel {
        #[doc = "pps selection for pps0."]
        #[must_use]
        #[inline(always)]
        pub const fn pps0_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "pps selection for pps0."]
        #[inline(always)]
        pub const fn set_pps0_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "pps selection for pps1."]
        #[must_use]
        #[inline(always)]
        pub const fn pps1_sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "pps selection for pps1."]
        #[inline(always)]
        pub const fn set_pps1_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "pps selection for pps2."]
        #[must_use]
        #[inline(always)]
        pub const fn pps2_sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "pps selection for pps2."]
        #[inline(always)]
        pub const fn set_pps2_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "pps selection for pps3."]
        #[must_use]
        #[inline(always)]
        pub const fn pps3_sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "pps selection for pps3."]
        #[inline(always)]
        pub const fn set_pps3_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for PtpEvtPpsSel {
        #[inline(always)]
        fn default() -> PtpEvtPpsSel {
            PtpEvtPpsSel(0)
        }
    }
    impl core::fmt::Debug for PtpEvtPpsSel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtPpsSel")
                .field("pps0_sel", &self.pps0_sel())
                .field("pps1_sel", &self.pps1_sel())
                .field("pps2_sel", &self.pps2_sel())
                .field("pps3_sel", &self.pps3_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtPpsSel {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PtpEvtPpsSel {{ pps0_sel: {=u8:?}, pps1_sel: {=u8:?}, pps2_sel: {=u8:?}, pps3_sel: {=u8:?} }}" , self . pps0_sel () , self . pps1_sel () , self . pps2_sel () , self . pps3_sel ())
        }
    }
    #[doc = "pps tod sun seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtPpsTodNs(pub u32);
    impl PtpEvtPpsTodNs {
        #[doc = "pps tod sub seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_tod_ns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "pps tod sub seconds."]
        #[inline(always)]
        pub const fn set_pps_tod_ns(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtPpsTodNs {
        #[inline(always)]
        fn default() -> PtpEvtPpsTodNs {
            PtpEvtPpsTodNs(0)
        }
    }
    impl core::fmt::Debug for PtpEvtPpsTodNs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtPpsTodNs")
                .field("pps_tod_ns", &self.pps_tod_ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtPpsTodNs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PtpEvtPpsTodNs {{ pps_tod_ns: {=u32:?} }}",
                self.pps_tod_ns()
            )
        }
    }
    #[doc = "pps tod seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtPpsTodSec(pub u32);
    impl PtpEvtPpsTodSec {
        #[doc = "pps tod seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_tod_sec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "pps tod seconds."]
        #[inline(always)]
        pub const fn set_pps_tod_sec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtPpsTodSec {
        #[inline(always)]
        fn default() -> PtpEvtPpsTodSec {
            PtpEvtPpsTodSec(0)
        }
    }
    impl core::fmt::Debug for PtpEvtPpsTodSec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtPpsTodSec")
                .field("pps_tod_sec", &self.pps_tod_sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtPpsTodSec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PtpEvtPpsTodSec {{ pps_tod_sec: {=u32:?} }}",
                self.pps_tod_sec()
            )
        }
    }
    #[doc = "target time sub seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtScpNs0(pub u32);
    impl PtpEvtScpNs0 {
        #[doc = "target time sub seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn scp_ns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "target time sub seconds."]
        #[inline(always)]
        pub const fn set_scp_ns(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtScpNs0 {
        #[inline(always)]
        fn default() -> PtpEvtScpNs0 {
            PtpEvtScpNs0(0)
        }
    }
    impl core::fmt::Debug for PtpEvtScpNs0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtScpNs0")
                .field("scp_ns", &self.scp_ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtScpNs0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PtpEvtScpNs0 {{ scp_ns: {=u32:?} }}", self.scp_ns())
        }
    }
    #[doc = "target time sub seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtScpNs1(pub u32);
    impl PtpEvtScpNs1 {
        #[doc = "target time sub seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn scp_ns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "target time sub seconds."]
        #[inline(always)]
        pub const fn set_scp_ns(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtScpNs1 {
        #[inline(always)]
        fn default() -> PtpEvtScpNs1 {
            PtpEvtScpNs1(0)
        }
    }
    impl core::fmt::Debug for PtpEvtScpNs1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtScpNs1")
                .field("scp_ns", &self.scp_ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtScpNs1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PtpEvtScpNs1 {{ scp_ns: {=u32:?} }}", self.scp_ns())
        }
    }
    #[doc = "target time sub seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtScpNs2(pub u32);
    impl PtpEvtScpNs2 {
        #[doc = "target time sub seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn scp_ns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "target time sub seconds."]
        #[inline(always)]
        pub const fn set_scp_ns(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtScpNs2 {
        #[inline(always)]
        fn default() -> PtpEvtScpNs2 {
            PtpEvtScpNs2(0)
        }
    }
    impl core::fmt::Debug for PtpEvtScpNs2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtScpNs2")
                .field("scp_ns", &self.scp_ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtScpNs2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PtpEvtScpNs2 {{ scp_ns: {=u32:?} }}", self.scp_ns())
        }
    }
    #[doc = "target time sub seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtScpNs3(pub u32);
    impl PtpEvtScpNs3 {
        #[doc = "target time sub seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn scp_ns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "target time sub seconds."]
        #[inline(always)]
        pub const fn set_scp_ns(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtScpNs3 {
        #[inline(always)]
        fn default() -> PtpEvtScpNs3 {
            PtpEvtScpNs3(0)
        }
    }
    impl core::fmt::Debug for PtpEvtScpNs3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtScpNs3")
                .field("scp_ns", &self.scp_ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtScpNs3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PtpEvtScpNs3 {{ scp_ns: {=u32:?} }}", self.scp_ns())
        }
    }
    #[doc = "target time seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtScpSec0(pub u32);
    impl PtpEvtScpSec0 {
        #[doc = "target time seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn scp_sec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "target time seconds."]
        #[inline(always)]
        pub const fn set_scp_sec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtScpSec0 {
        #[inline(always)]
        fn default() -> PtpEvtScpSec0 {
            PtpEvtScpSec0(0)
        }
    }
    impl core::fmt::Debug for PtpEvtScpSec0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtScpSec0")
                .field("scp_sec", &self.scp_sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtScpSec0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PtpEvtScpSec0 {{ scp_sec: {=u32:?} }}", self.scp_sec())
        }
    }
    #[doc = "target time seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtScpSec1(pub u32);
    impl PtpEvtScpSec1 {
        #[doc = "target time seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn scp_sec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "target time seconds."]
        #[inline(always)]
        pub const fn set_scp_sec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtScpSec1 {
        #[inline(always)]
        fn default() -> PtpEvtScpSec1 {
            PtpEvtScpSec1(0)
        }
    }
    impl core::fmt::Debug for PtpEvtScpSec1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtScpSec1")
                .field("scp_sec", &self.scp_sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtScpSec1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PtpEvtScpSec1 {{ scp_sec: {=u32:?} }}", self.scp_sec())
        }
    }
    #[doc = "target time seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtScpSec2(pub u32);
    impl PtpEvtScpSec2 {
        #[doc = "target time seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn scp_sec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "target time seconds."]
        #[inline(always)]
        pub const fn set_scp_sec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtScpSec2 {
        #[inline(always)]
        fn default() -> PtpEvtScpSec2 {
            PtpEvtScpSec2(0)
        }
    }
    impl core::fmt::Debug for PtpEvtScpSec2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtScpSec2")
                .field("scp_sec", &self.scp_sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtScpSec2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PtpEvtScpSec2 {{ scp_sec: {=u32:?} }}", self.scp_sec())
        }
    }
    #[doc = "target time seconds."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtScpSec3(pub u32);
    impl PtpEvtScpSec3 {
        #[doc = "target time seconds."]
        #[must_use]
        #[inline(always)]
        pub const fn scp_sec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "target time seconds."]
        #[inline(always)]
        pub const fn set_scp_sec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for PtpEvtScpSec3 {
        #[inline(always)]
        fn default() -> PtpEvtScpSec3 {
            PtpEvtScpSec3(0)
        }
    }
    impl core::fmt::Debug for PtpEvtScpSec3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtScpSec3")
                .field("scp_sec", &self.scp_sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtScpSec3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "PtpEvtScpSec3 {{ scp_sec: {=u32:?} }}", self.scp_sec())
        }
    }
    #[doc = "timer status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtTmrSts(pub u32);
    impl PtpEvtTmrSts {
        #[doc = "target time0 reached."]
        #[must_use]
        #[inline(always)]
        pub const fn target_time0_reach_intr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "target time0 reached."]
        #[inline(always)]
        pub const fn set_target_time0_reach_intr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "auxiliary timestamp trigger snapshot."]
        #[must_use]
        #[inline(always)]
        pub const fn ptp_fifo_wr_intr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "auxiliary timestamp trigger snapshot."]
        #[inline(always)]
        pub const fn set_ptp_fifo_wr_intr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "target time0 configure error."]
        #[must_use]
        #[inline(always)]
        pub const fn target_time0_cfg_err(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "target time0 configure error."]
        #[inline(always)]
        pub const fn set_target_time0_cfg_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "target time1 reached."]
        #[must_use]
        #[inline(always)]
        pub const fn target_time1_reach_intr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "target time1 reached."]
        #[inline(always)]
        pub const fn set_target_time1_reach_intr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "target time1 configure error."]
        #[must_use]
        #[inline(always)]
        pub const fn target_time1_cfg_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "target time1 configure error."]
        #[inline(always)]
        pub const fn set_target_time1_cfg_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "target time2 reached."]
        #[must_use]
        #[inline(always)]
        pub const fn target_time2_reach_intr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "target time2 reached."]
        #[inline(always)]
        pub const fn set_target_time2_reach_intr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "target time2 configure error."]
        #[must_use]
        #[inline(always)]
        pub const fn target_time2_cfg_err(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "target time2 configure error."]
        #[inline(always)]
        pub const fn set_target_time2_cfg_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "target time3 reached."]
        #[must_use]
        #[inline(always)]
        pub const fn target_time3_reach_intr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "target time3 reached."]
        #[inline(always)]
        pub const fn set_target_time3_reach_intr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "target time3 configure error."]
        #[must_use]
        #[inline(always)]
        pub const fn target_time3_cfg_err(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "target time3 configure error."]
        #[inline(always)]
        pub const fn set_target_time3_cfg_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "pps tod intrrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn pps_tod_intr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "pps tod intrrupt."]
        #[inline(always)]
        pub const fn set_pps_tod_intr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "auxiliary port."]
        #[must_use]
        #[inline(always)]
        pub const fn atport(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "auxiliary port."]
        #[inline(always)]
        pub const fn set_atport(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "auxiliary fifo full error."]
        #[must_use]
        #[inline(always)]
        pub const fn atsstm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "auxiliary fifo full error."]
        #[inline(always)]
        pub const fn set_atsstm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "fifo valid count."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_cnt(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "fifo valid count."]
        #[inline(always)]
        pub const fn set_rd_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
    }
    impl Default for PtpEvtTmrSts {
        #[inline(always)]
        fn default() -> PtpEvtTmrSts {
            PtpEvtTmrSts(0)
        }
    }
    impl core::fmt::Debug for PtpEvtTmrSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtTmrSts")
                .field("target_time0_reach_intr", &self.target_time0_reach_intr())
                .field("ptp_fifo_wr_intr", &self.ptp_fifo_wr_intr())
                .field("target_time0_cfg_err", &self.target_time0_cfg_err())
                .field("target_time1_reach_intr", &self.target_time1_reach_intr())
                .field("target_time1_cfg_err", &self.target_time1_cfg_err())
                .field("target_time2_reach_intr", &self.target_time2_reach_intr())
                .field("target_time2_cfg_err", &self.target_time2_cfg_err())
                .field("target_time3_reach_intr", &self.target_time3_reach_intr())
                .field("target_time3_cfg_err", &self.target_time3_cfg_err())
                .field("pps_tod_intr", &self.pps_tod_intr())
                .field("atport", &self.atport())
                .field("atsstm", &self.atsstm())
                .field("rd_cnt", &self.rd_cnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtTmrSts {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PtpEvtTmrSts {{ target_time0_reach_intr: {=bool:?}, ptp_fifo_wr_intr: {=bool:?}, target_time0_cfg_err: {=bool:?}, target_time1_reach_intr: {=bool:?}, target_time1_cfg_err: {=bool:?}, target_time2_reach_intr: {=bool:?}, target_time2_cfg_err: {=bool:?}, target_time3_reach_intr: {=bool:?}, target_time3_cfg_err: {=bool:?}, pps_tod_intr: {=bool:?}, atport: {=u8:?}, atsstm: {=bool:?}, rd_cnt: {=u8:?} }}" , self . target_time0_reach_intr () , self . ptp_fifo_wr_intr () , self . target_time0_cfg_err () , self . target_time1_reach_intr () , self . target_time1_cfg_err () , self . target_time2_reach_intr () , self . target_time2_cfg_err () , self . target_time3_reach_intr () , self . target_time3_cfg_err () , self . pps_tod_intr () , self . atport () , self . atsstm () , self . rd_cnt ())
        }
    }
    #[doc = "timestamp control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PtpEvtTsCtl(pub u32);
    impl PtpEvtTsCtl {
        #[doc = "timestamp interrupt trigger enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tstig(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "timestamp interrupt trigger enable."]
        #[inline(always)]
        pub const fn set_tstig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "auxiliary snapshot fifo clear."]
        #[must_use]
        #[inline(always)]
        pub const fn atsfc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "auxiliary snapshot fifo clear."]
        #[inline(always)]
        pub const fn set_atsfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "auxiliay snapshot enable."]
        #[must_use]
        #[inline(always)]
        pub const fn atsen(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x0f;
            val as u8
        }
        #[doc = "auxiliay snapshot enable."]
        #[inline(always)]
        pub const fn set_atsen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
        }
    }
    impl Default for PtpEvtTsCtl {
        #[inline(always)]
        fn default() -> PtpEvtTsCtl {
            PtpEvtTsCtl(0)
        }
    }
    impl core::fmt::Debug for PtpEvtTsCtl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PtpEvtTsCtl")
                .field("tstig", &self.tstig())
                .field("atsfc", &self.atsfc())
                .field("atsen", &self.atsen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PtpEvtTsCtl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PtpEvtTsCtl {{ tstig: {=bool:?}, atsfc: {=bool:?}, atsen: {=u8:?} }}",
                self.tstig(),
                self.atsfc(),
                self.atsen()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QciCnt(pub u32);
    impl QciCnt {
        #[doc = "Filter counter (see 802.1Qci 8.6.5.1.1 f) CNT0: Frames that matched filter CNT1: Frames that passed gate CNT2: Frames that did not pass gate CNT3: Frames that passed Maximum-SDU size check CNT4: Frames that did not pass size check CNT5: Frames discarded by Flow Meter operation Counters starting at value <0> after reset."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Filter counter (see 802.1Qci 8.6.5.1.1 f) CNT0: Frames that matched filter CNT1: Frames that passed gate CNT2: Frames that did not pass gate CNT3: Frames that passed Maximum-SDU size check CNT4: Frames that did not pass size check CNT5: Frames discarded by Flow Meter operation Counters starting at value <0> after reset."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for QciCnt {
        #[inline(always)]
        fn default() -> QciCnt {
            QciCnt(0)
        }
    }
    impl core::fmt::Debug for QciCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("QciCnt")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for QciCnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "QciCnt {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "ONLY IN PORT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcAlarmNs(pub u32);
    impl RtcAlarmNs {
        #[doc = "Alarm Time (nanoseconds part). Valid value range from 0 – 999999999."]
        #[must_use]
        #[inline(always)]
        pub const fn al_ns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Alarm Time (nanoseconds part). Valid value range from 0 – 999999999."]
        #[inline(always)]
        pub const fn set_al_ns(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for RtcAlarmNs {
        #[inline(always)]
        fn default() -> RtcAlarmNs {
            RtcAlarmNs(0)
        }
    }
    impl core::fmt::Debug for RtcAlarmNs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcAlarmNs")
                .field("al_ns", &self.al_ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcAlarmNs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcAlarmNs {{ al_ns: {=u32:?} }}", self.al_ns())
        }
    }
    #[doc = "ONLY IN PORT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcAlarmSh(pub u32);
    impl RtcAlarmSh {
        #[doc = "Alarm Time (seconds hi part)."]
        #[must_use]
        #[inline(always)]
        pub const fn al_sh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Alarm Time (seconds hi part)."]
        #[inline(always)]
        pub const fn set_al_sh(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for RtcAlarmSh {
        #[inline(always)]
        fn default() -> RtcAlarmSh {
            RtcAlarmSh(0)
        }
    }
    impl core::fmt::Debug for RtcAlarmSh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcAlarmSh")
                .field("al_sh", &self.al_sh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcAlarmSh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcAlarmSh {{ al_sh: {=u16:?} }}", self.al_sh())
        }
    }
    #[doc = "ONLY IN PORT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcAlarmSl(pub u32);
    impl RtcAlarmSl {
        #[doc = "Alarm Time (seconds lo part)."]
        #[must_use]
        #[inline(always)]
        pub const fn al_sl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Alarm Time (seconds lo part)."]
        #[inline(always)]
        pub const fn set_al_sl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RtcAlarmSl {
        #[inline(always)]
        fn default() -> RtcAlarmSl {
            RtcAlarmSl(0)
        }
    }
    impl core::fmt::Debug for RtcAlarmSl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcAlarmSl")
                .field("al_sl", &self.al_sl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcAlarmSl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcAlarmSl {{ al_sl: {=u32:?} }}", self.al_sl())
        }
    }
    #[doc = "ONLY IN PORT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcCr(pub u32);
    impl RtcCr {
        #[doc = "Alarm interrupt enable: alarm interrupt enabled when 1."]
        #[must_use]
        #[inline(always)]
        pub const fn alie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Alarm interrupt enable: alarm interrupt enabled when 1."]
        #[inline(always)]
        pub const fn set_alie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer A enable: timer enabled when 1."]
        #[must_use]
        #[inline(always)]
        pub const fn taen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A enable: timer enabled when 1."]
        #[inline(always)]
        pub const fn set_taen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer A interrupt enable: interrupt enabled when 1."]
        #[must_use]
        #[inline(always)]
        pub const fn taie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A interrupt enable: interrupt enabled when 1."]
        #[inline(always)]
        pub const fn set_taie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for RtcCr {
        #[inline(always)]
        fn default() -> RtcCr {
            RtcCr(0)
        }
    }
    impl core::fmt::Debug for RtcCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcCr")
                .field("alie", &self.alie())
                .field("taen", &self.taen())
                .field("taie", &self.taie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcCr {{ alie: {=bool:?}, taen: {=bool:?}, taie: {=bool:?} }}",
                self.alie(),
                self.taen(),
                self.taie()
            )
        }
    }
    #[doc = "ONLY IN PORT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcCtCurtimeNs(pub u32);
    impl RtcCtCurtimeNs {
        #[doc = "Local Time (nanosecond part): Update can be triggered by write access to this register. Value range from 0 – 999999999."]
        #[must_use]
        #[inline(always)]
        pub const fn ct_ns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Local Time (nanosecond part): Update can be triggered by write access to this register. Value range from 0 – 999999999."]
        #[inline(always)]
        pub const fn set_ct_ns(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for RtcCtCurtimeNs {
        #[inline(always)]
        fn default() -> RtcCtCurtimeNs {
            RtcCtCurtimeNs(0)
        }
    }
    impl core::fmt::Debug for RtcCtCurtimeNs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcCtCurtimeNs")
                .field("ct_ns", &self.ct_ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcCtCurtimeNs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcCtCurtimeNs {{ ct_ns: {=u32:?} }}", self.ct_ns())
        }
    }
    #[doc = "ONLY IN PORT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcCtCurtimeSec(pub u32);
    impl RtcCtCurtimeSec {
        #[doc = "Current Time (second part): Update can be triggered by write access to register CURTIME_NS."]
        #[must_use]
        #[inline(always)]
        pub const fn ct_sec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Current Time (second part): Update can be triggered by write access to register CURTIME_NS."]
        #[inline(always)]
        pub const fn set_ct_sec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RtcCtCurtimeSec {
        #[inline(always)]
        fn default() -> RtcCtCurtimeSec {
            RtcCtCurtimeSec(0)
        }
    }
    impl core::fmt::Debug for RtcCtCurtimeSec {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcCtCurtimeSec")
                .field("ct_sec", &self.ct_sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcCtCurtimeSec {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcCtCurtimeSec {{ ct_sec: {=u32:?} }}", self.ct_sec())
        }
    }
    #[doc = "ONLY IN PORT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcCtTimerIncr(pub u32);
    impl RtcCtTimerIncr {
        #[doc = "Local time increment – fractional ns, unsigned, in (1 / 2^24) n."]
        #[must_use]
        #[inline(always)]
        pub const fn fns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Local time increment – fractional ns, unsigned, in (1 / 2^24) n."]
        #[inline(always)]
        pub const fn set_fns(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "Local time increment – nanoseconds (integer)."]
        #[must_use]
        #[inline(always)]
        pub const fn ns(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Local time increment – nanoseconds (integer)."]
        #[inline(always)]
        pub const fn set_ns(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for RtcCtTimerIncr {
        #[inline(always)]
        fn default() -> RtcCtTimerIncr {
            RtcCtTimerIncr(0)
        }
    }
    impl core::fmt::Debug for RtcCtTimerIncr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcCtTimerIncr")
                .field("fns", &self.fns())
                .field("ns", &self.ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcCtTimerIncr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcCtTimerIncr {{ fns: {=u32:?}, ns: {=u8:?} }}",
                self.fns(),
                self.ns()
            )
        }
    }
    #[doc = "ONLY IN PORT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcOfsCh(pub u32);
    impl RtcOfsCh {
        #[doc = "Real Time Offset Change in fractional nanoseconds, signed value; value range from -2^23 / 2^24 to (2^23-1) / 2^24 nanoseconds."]
        #[must_use]
        #[inline(always)]
        pub const fn sfns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Real Time Offset Change in fractional nanoseconds, signed value; value range from -2^23 / 2^24 to (2^23-1) / 2^24 nanoseconds."]
        #[inline(always)]
        pub const fn set_sfns(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "Real Time Offset Change – sign extension of SFNS (Bit 23)."]
        #[must_use]
        #[inline(always)]
        pub const fn sext(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Real Time Offset Change – sign extension of SFNS (Bit 23)."]
        #[inline(always)]
        pub const fn set_sext(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for RtcOfsCh {
        #[inline(always)]
        fn default() -> RtcOfsCh {
            RtcOfsCh(0)
        }
    }
    impl core::fmt::Debug for RtcOfsCh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcOfsCh")
                .field("sfns", &self.sfns())
                .field("sext", &self.sext())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcOfsCh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcOfsCh {{ sfns: {=u32:?}, sext: {=u8:?} }}",
                self.sfns(),
                self.sext()
            )
        }
    }
    #[doc = "ONLY IN PORT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcOfsNs(pub u32);
    impl RtcOfsNs {
        #[doc = "Real Time Offset (nanoseconds part). Valid value range from 0 – 999999999."]
        #[must_use]
        #[inline(always)]
        pub const fn ofs_ns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Real Time Offset (nanoseconds part). Valid value range from 0 – 999999999."]
        #[inline(always)]
        pub const fn set_ofs_ns(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for RtcOfsNs {
        #[inline(always)]
        fn default() -> RtcOfsNs {
            RtcOfsNs(0)
        }
    }
    impl core::fmt::Debug for RtcOfsNs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcOfsNs")
                .field("ofs_ns", &self.ofs_ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcOfsNs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcOfsNs {{ ofs_ns: {=u32:?} }}", self.ofs_ns())
        }
    }
    #[doc = "ONLY IN PORT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcOfsSh(pub u32);
    impl RtcOfsSh {
        #[doc = "48 Bit Real Time Offset (seconds hi part)."]
        #[must_use]
        #[inline(always)]
        pub const fn ofs_sh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "48 Bit Real Time Offset (seconds hi part)."]
        #[inline(always)]
        pub const fn set_ofs_sh(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for RtcOfsSh {
        #[inline(always)]
        fn default() -> RtcOfsSh {
            RtcOfsSh(0)
        }
    }
    impl core::fmt::Debug for RtcOfsSh {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcOfsSh")
                .field("ofs_sh", &self.ofs_sh())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcOfsSh {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcOfsSh {{ ofs_sh: {=u16:?} }}", self.ofs_sh())
        }
    }
    #[doc = "ONLY IN PORT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcOfsSl(pub u32);
    impl RtcOfsSl {
        #[doc = "48 Bit Real Time Offset (seconds lo part)."]
        #[must_use]
        #[inline(always)]
        pub const fn ofs_sl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "48 Bit Real Time Offset (seconds lo part)."]
        #[inline(always)]
        pub const fn set_ofs_sl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RtcOfsSl {
        #[inline(always)]
        fn default() -> RtcOfsSl {
            RtcOfsSl(0)
        }
    }
    impl core::fmt::Debug for RtcOfsSl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcOfsSl")
                .field("ofs_sl", &self.ofs_sl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcOfsSl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "RtcOfsSl {{ ofs_sl: {=u32:?} }}", self.ofs_sl())
        }
    }
    #[doc = "ONLY IN PORT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcSr(pub u32);
    impl RtcSr {
        #[doc = "ALIS ro Alarm Interrupt Status: Always set while RTC-Time >= Alarm-Time."]
        #[must_use]
        #[inline(always)]
        pub const fn alis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ALIS ro Alarm Interrupt Status: Always set while RTC-Time >= Alarm-Time."]
        #[inline(always)]
        pub const fn set_alis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer A Interrupt Status: set at rising edge of “timer_clk_a”, write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn tais(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer A Interrupt Status: set at rising edge of “timer_clk_a”, write 1 to clear."]
        #[inline(always)]
        pub const fn set_tais(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for RtcSr {
        #[inline(always)]
        fn default() -> RtcSr {
            RtcSr(0)
        }
    }
    impl core::fmt::Debug for RtcSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcSr")
                .field("alis", &self.alis())
                .field("tais", &self.tais())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcSr {{ alis: {=bool:?}, tais: {=bool:?} }}",
                self.alis(),
                self.tais()
            )
        }
    }
    #[doc = "ONLY IN PORT1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtcTimerAPeriod(pub u32);
    impl RtcTimerAPeriod {
        #[doc = "Timer A Period in ns. This is the period of the timer until the next event, but the half-period of the signal “timer_a_clk”."]
        #[must_use]
        #[inline(always)]
        pub const fn period_ns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[doc = "Timer A Period in ns. This is the period of the timer until the next event, but the half-period of the signal “timer_a_clk”."]
        #[inline(always)]
        pub const fn set_period_ns(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
    }
    impl Default for RtcTimerAPeriod {
        #[inline(always)]
        fn default() -> RtcTimerAPeriod {
            RtcTimerAPeriod(0)
        }
    }
    impl core::fmt::Debug for RtcTimerAPeriod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtcTimerAPeriod")
                .field("period_ns", &self.period_ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RtcTimerAPeriod {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RtcTimerAPeriod {{ period_ns: {=u32:?} }}",
                self.period_ns()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rxdata(pub u32);
    impl Rxdata {
        #[doc = "RXBUF_DATA_WORD."]
        #[must_use]
        #[inline(always)]
        pub const fn rxbuf_data_word(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RXBUF_DATA_WORD."]
        #[inline(always)]
        pub const fn set_rxbuf_data_word(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rxdata {
        #[inline(always)]
        fn default() -> Rxdata {
            Rxdata(0)
        }
    }
    impl core::fmt::Debug for Rxdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rxdata")
                .field("rxbuf_data_word", &self.rxbuf_data_word())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rxdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rxdata {{ rxbuf_data_word: {=u32:?} }}",
                self.rxbuf_data_word()
            )
        }
    }
    #[doc = "s2mm axi address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S2mmAddrlo(pub u32);
    impl S2mmAddrlo {
        #[doc = "axi address."]
        #[must_use]
        #[inline(always)]
        pub const fn addrlo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "axi address."]
        #[inline(always)]
        pub const fn set_addrlo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for S2mmAddrlo {
        #[inline(always)]
        fn default() -> S2mmAddrlo {
            S2mmAddrlo(0)
        }
    }
    impl core::fmt::Debug for S2mmAddrlo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S2mmAddrlo")
                .field("addrlo", &self.addrlo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for S2mmAddrlo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "S2mmAddrlo {{ addrlo: {=u32:?} }}", self.addrlo())
        }
    }
    #[doc = "s2mm command control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S2mmCtrl(pub u32);
    impl S2mmCtrl {
        #[doc = "command id."]
        #[must_use]
        #[inline(always)]
        pub const fn id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "command id."]
        #[inline(always)]
        pub const fn set_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "commit buffered descriptor to command queue."]
        #[must_use]
        #[inline(always)]
        pub const fn go(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "commit buffered descriptor to command queue."]
        #[inline(always)]
        pub const fn set_go(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for S2mmCtrl {
        #[inline(always)]
        fn default() -> S2mmCtrl {
            S2mmCtrl(0)
        }
    }
    impl core::fmt::Debug for S2mmCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S2mmCtrl")
                .field("id", &self.id())
                .field("go", &self.go())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for S2mmCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "S2mmCtrl {{ id: {=u8:?}, go: {=bool:?} }}",
                self.id(),
                self.go()
            )
        }
    }
    #[doc = "s2mm dma config status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S2mmDmaCfg(pub u32);
    impl S2mmDmaCfg {
        #[doc = "IP version."]
        #[must_use]
        #[inline(always)]
        pub const fn ver(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "IP version."]
        #[inline(always)]
        pub const fn set_ver(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "axi data bus width."]
        #[must_use]
        #[inline(always)]
        pub const fn asize(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "axi data bus width."]
        #[inline(always)]
        pub const fn set_asize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "enabled support for 64 bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ena64(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "enabled support for 64 bit."]
        #[inline(always)]
        pub const fn set_ena64(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "command buffer depth."]
        #[must_use]
        #[inline(always)]
        pub const fn cbufd(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "command buffer depth."]
        #[inline(always)]
        pub const fn set_cbufd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "data buffer depth."]
        #[must_use]
        #[inline(always)]
        pub const fn dbufd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "data buffer depth."]
        #[inline(always)]
        pub const fn set_dbufd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for S2mmDmaCfg {
        #[inline(always)]
        fn default() -> S2mmDmaCfg {
            S2mmDmaCfg(0)
        }
    }
    impl core::fmt::Debug for S2mmDmaCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S2mmDmaCfg")
                .field("ver", &self.ver())
                .field("asize", &self.asize())
                .field("ena64", &self.ena64())
                .field("cbufd", &self.cbufd())
                .field("dbufd", &self.dbufd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for S2mmDmaCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "S2mmDmaCfg {{ ver: {=u16:?}, asize: {=u8:?}, ena64: {=bool:?}, cbufd: {=u8:?}, dbufd: {=u8:?} }}" , self . ver () , self . asize () , self . ena64 () , self . cbufd () , self . dbufd ())
        }
    }
    #[doc = "s2mm dma control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S2mmDmaCr(pub u32);
    impl S2mmDmaCr {
        #[doc = "run commands from queue to data mover."]
        #[must_use]
        #[inline(always)]
        pub const fn run(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "run commands from queue to data mover."]
        #[inline(always)]
        pub const fn set_run(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "stop on error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn soe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "stop on error flag."]
        #[inline(always)]
        pub const fn set_soe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "do reset when writing 1."]
        #[must_use]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "do reset when writing 1."]
        #[inline(always)]
        pub const fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "interrupt request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn irqen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt request enable."]
        #[inline(always)]
        pub const fn set_irqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "max axi burst size."]
        #[must_use]
        #[inline(always)]
        pub const fn mxlen(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "max axi burst size."]
        #[inline(always)]
        pub const fn set_mxlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for S2mmDmaCr {
        #[inline(always)]
        fn default() -> S2mmDmaCr {
            S2mmDmaCr(0)
        }
    }
    impl core::fmt::Debug for S2mmDmaCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S2mmDmaCr")
                .field("run", &self.run())
                .field("soe", &self.soe())
                .field("reset", &self.reset())
                .field("irqen", &self.irqen())
                .field("mxlen", &self.mxlen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for S2mmDmaCr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "S2mmDmaCr {{ run: {=bool:?}, soe: {=bool:?}, reset: {=bool:?}, irqen: {=bool:?}, mxlen: {=u8:?} }}" , self . run () , self . soe () , self . reset () , self . irqen () , self . mxlen ())
        }
    }
    #[doc = "s2mm buffer fill status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S2mmDmaFill(pub u32);
    impl S2mmDmaFill {
        #[doc = "command buffer fill level."]
        #[must_use]
        #[inline(always)]
        pub const fn cfill(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "command buffer fill level."]
        #[inline(always)]
        pub const fn set_cfill(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "response buffer fill level."]
        #[must_use]
        #[inline(always)]
        pub const fn rfill(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "response buffer fill level."]
        #[inline(always)]
        pub const fn set_rfill(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for S2mmDmaFill {
        #[inline(always)]
        fn default() -> S2mmDmaFill {
            S2mmDmaFill(0)
        }
    }
    impl core::fmt::Debug for S2mmDmaFill {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S2mmDmaFill")
                .field("cfill", &self.cfill())
                .field("rfill", &self.rfill())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for S2mmDmaFill {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "S2mmDmaFill {{ cfill: {=u16:?}, rfill: {=u16:?} }}",
                self.cfill(),
                self.rfill()
            )
        }
    }
    #[doc = "s2mm state."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S2mmDmaSr(pub u32);
    impl S2mmDmaSr {
        #[doc = "s2mm is stopped."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "s2mm is stopped."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "busy, issued command and outstanding response."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "busy, issued command and outstanding response."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "resetting status."]
        #[must_use]
        #[inline(always)]
        pub const fn rset(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "resetting status."]
        #[inline(always)]
        pub const fn set_rset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "interrupt request pending."]
        #[must_use]
        #[inline(always)]
        pub const fn irq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt request pending."]
        #[inline(always)]
        pub const fn set_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "command buffer empty."]
        #[must_use]
        #[inline(always)]
        pub const fn cbufe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "command buffer empty."]
        #[inline(always)]
        pub const fn set_cbufe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "command buffer full."]
        #[must_use]
        #[inline(always)]
        pub const fn cbuff(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "command buffer full."]
        #[inline(always)]
        pub const fn set_cbuff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "response buffer empty."]
        #[must_use]
        #[inline(always)]
        pub const fn rbufe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "response buffer empty."]
        #[inline(always)]
        pub const fn set_rbufe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "response buffer full."]
        #[must_use]
        #[inline(always)]
        pub const fn rbuff(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "response buffer full."]
        #[inline(always)]
        pub const fn set_rbuff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for S2mmDmaSr {
        #[inline(always)]
        fn default() -> S2mmDmaSr {
            S2mmDmaSr(0)
        }
    }
    impl core::fmt::Debug for S2mmDmaSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S2mmDmaSr")
                .field("stop", &self.stop())
                .field("busy", &self.busy())
                .field("rset", &self.rset())
                .field("irq", &self.irq())
                .field("cbufe", &self.cbufe())
                .field("cbuff", &self.cbuff())
                .field("rbufe", &self.rbufe())
                .field("rbuff", &self.rbuff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for S2mmDmaSr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "S2mmDmaSr {{ stop: {=bool:?}, busy: {=bool:?}, rset: {=bool:?}, irq: {=bool:?}, cbufe: {=bool:?}, cbuff: {=bool:?}, rbufe: {=bool:?}, rbuff: {=bool:?} }}" , self . stop () , self . busy () , self . rset () , self . irq () , self . cbufe () , self . cbuff () , self . rbufe () , self . rbuff ())
        }
    }
    #[doc = "s2mm axi length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S2mmLength(pub u32);
    impl S2mmLength {
        #[doc = "transfer request length in bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn length(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "transfer request length in bytes."]
        #[inline(always)]
        pub const fn set_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for S2mmLength {
        #[inline(always)]
        fn default() -> S2mmLength {
            S2mmLength(0)
        }
    }
    impl core::fmt::Debug for S2mmLength {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S2mmLength")
                .field("length", &self.length())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for S2mmLength {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "S2mmLength {{ length: {=u16:?} }}", self.length())
        }
    }
    #[doc = "s2mm response buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct S2mmResp(pub u32);
    impl S2mmResp {
        #[doc = "received packet size when terminated by TLAST."]
        #[must_use]
        #[inline(always)]
        pub const fn length(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "received packet size when terminated by TLAST."]
        #[inline(always)]
        pub const fn set_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "command ID feedback."]
        #[must_use]
        #[inline(always)]
        pub const fn id(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "command ID feedback."]
        #[inline(always)]
        pub const fn set_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "slave error."]
        #[must_use]
        #[inline(always)]
        pub const fn slverr(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "slave error."]
        #[inline(always)]
        pub const fn set_slverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "decode error."]
        #[must_use]
        #[inline(always)]
        pub const fn decerr(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "decode error."]
        #[inline(always)]
        pub const fn set_decerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "axi-stream with last."]
        #[must_use]
        #[inline(always)]
        pub const fn last(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "axi-stream with last."]
        #[inline(always)]
        pub const fn set_last(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for S2mmResp {
        #[inline(always)]
        fn default() -> S2mmResp {
            S2mmResp(0)
        }
    }
    impl core::fmt::Debug for S2mmResp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("S2mmResp")
                .field("length", &self.length())
                .field("id", &self.id())
                .field("slverr", &self.slverr())
                .field("decerr", &self.decerr())
                .field("last", &self.last())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for S2mmResp {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "S2mmResp {{ length: {=u16:?}, id: {=u8:?}, slverr: {=bool:?}, decerr: {=bool:?}, last: {=bool:?} }}" , self . length () , self . id () , self . slverr () , self . decerr () , self . last ())
        }
    }
    #[doc = "softer reset control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SoftRstCtrl(pub u32);
    impl SoftRstCtrl {
        #[doc = "port1 tx reset control."]
        #[must_use]
        #[inline(always)]
        pub const fn port1_tx_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "port1 tx reset control."]
        #[inline(always)]
        pub const fn set_port1_tx_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "port1 rx reset control."]
        #[must_use]
        #[inline(always)]
        pub const fn port1_rx_rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "port1 rx reset control."]
        #[inline(always)]
        pub const fn set_port1_rx_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "port2 tx reset control."]
        #[must_use]
        #[inline(always)]
        pub const fn port2_tx_rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "port2 tx reset control."]
        #[inline(always)]
        pub const fn set_port2_tx_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "port2 rx reset control."]
        #[must_use]
        #[inline(always)]
        pub const fn port2_rx_rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "port2 rx reset control."]
        #[inline(always)]
        pub const fn set_port2_rx_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "port3 tx reset control."]
        #[must_use]
        #[inline(always)]
        pub const fn port3_tx_rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "port3 tx reset control."]
        #[inline(always)]
        pub const fn set_port3_tx_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "port3 rx reset control."]
        #[must_use]
        #[inline(always)]
        pub const fn port3_rx_rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "port3 rx reset control."]
        #[inline(always)]
        pub const fn set_port3_rx_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "dma0 reset control."]
        #[must_use]
        #[inline(always)]
        pub const fn dma0_rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "dma0 reset control."]
        #[inline(always)]
        pub const fn set_dma0_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ptp event module reset control."]
        #[must_use]
        #[inline(always)]
        pub const fn ptp_evt_rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ptp event module reset control."]
        #[inline(always)]
        pub const fn set_ptp_evt_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "tsn core reset control."]
        #[must_use]
        #[inline(always)]
        pub const fn tsn_core_rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "tsn core reset control."]
        #[inline(always)]
        pub const fn set_tsn_core_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for SoftRstCtrl {
        #[inline(always)]
        fn default() -> SoftRstCtrl {
            SoftRstCtrl(0)
        }
    }
    impl core::fmt::Debug for SoftRstCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SoftRstCtrl")
                .field("port1_tx_rst", &self.port1_tx_rst())
                .field("port1_rx_rst", &self.port1_rx_rst())
                .field("port2_tx_rst", &self.port2_tx_rst())
                .field("port2_rx_rst", &self.port2_rx_rst())
                .field("port3_tx_rst", &self.port3_tx_rst())
                .field("port3_rx_rst", &self.port3_rx_rst())
                .field("dma0_rst", &self.dma0_rst())
                .field("ptp_evt_rst", &self.ptp_evt_rst())
                .field("tsn_core_rst", &self.tsn_core_rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SoftRstCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SoftRstCtrl {{ port1_tx_rst: {=bool:?}, port1_rx_rst: {=bool:?}, port2_tx_rst: {=bool:?}, port2_rx_rst: {=bool:?}, port3_tx_rst: {=bool:?}, port3_rx_rst: {=bool:?}, dma0_rst: {=bool:?}, ptp_evt_rst: {=bool:?}, tsn_core_rst: {=bool:?} }}" , self . port1_tx_rst () , self . port1_rx_rst () , self . port2_tx_rst () , self . port2_rx_rst () , self . port3_tx_rst () , self . port3_rx_rst () , self . dma0_rst () , self . ptp_evt_rst () , self . tsn_core_rst ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlEgressEcsrQdrop(pub u32);
    impl SwCtrlEgressEcsrQdrop {
        #[doc = "Enable/Disable drop in egress when TSN queue not free. 1 - drop enabled 0 - drop disabled TSN-SW: bit\\[i\\]
- from Port\\[i\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn en_vec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Enable/Disable drop in egress when TSN queue not free. 1 - drop enabled 0 - drop disabled TSN-SW: bit\\[i\\]
- from Port\\[i\\]."]
        #[inline(always)]
        pub const fn set_en_vec(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "disable drop for each queue when queue not free."]
        #[must_use]
        #[inline(always)]
        pub const fn dis_vec(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "disable drop for each queue when queue not free."]
        #[inline(always)]
        pub const fn set_dis_vec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for SwCtrlEgressEcsrQdrop {
        #[inline(always)]
        fn default() -> SwCtrlEgressEcsrQdrop {
            SwCtrlEgressEcsrQdrop(0)
        }
    }
    impl core::fmt::Debug for SwCtrlEgressEcsrQdrop {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlEgressEcsrQdrop")
                .field("en_vec", &self.en_vec())
                .field("dis_vec", &self.dis_vec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlEgressEcsrQdrop {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SwCtrlEgressEcsrQdrop {{ en_vec: {=u32:?}, dis_vec: {=u8:?} }}",
                self.en_vec(),
                self.dis_vec()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlIgressRxFdfifoEErrorFlag(pub u32);
    impl SwCtrlIgressRxFdfifoEErrorFlag {
        #[doc = "FD FIFO failure. Internal controller lost synchronization."]
        #[must_use]
        #[inline(always)]
        pub const fn desc_seq_err(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO failure. Internal controller lost synchronization."]
        #[inline(always)]
        pub const fn set_desc_seq_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FD FIFO failure. Descriptor not received correctly."]
        #[must_use]
        #[inline(always)]
        pub const fn desc_nrdy_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO failure. Descriptor not received correctly."]
        #[inline(always)]
        pub const fn set_desc_nrdy_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Frame was dropped because the FIFO is full. Full by too much data."]
        #[must_use]
        #[inline(always)]
        pub const fn drop_full_mem(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Frame was dropped because the FIFO is full. Full by too much data."]
        #[inline(always)]
        pub const fn set_drop_full_mem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Frame was dropped because the internal descriptor FIFO is full. Full by too many frames."]
        #[must_use]
        #[inline(always)]
        pub const fn drop_full_desc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Frame was dropped because the internal descriptor FIFO is full. Full by too many frames."]
        #[inline(always)]
        pub const fn set_drop_full_desc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Frame was dropped because the FIFO was not ready. That can typically happen after a reset of the FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn drop_nrdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Frame was dropped because the FIFO was not ready. That can typically happen after a reset of the FIFO."]
        #[inline(always)]
        pub const fn set_drop_nrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Set if a frame is partially written into FIFO which had insufficient space. The frame is cut and frame error is set."]
        #[must_use]
        #[inline(always)]
        pub const fn wrfail_full(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Set if a frame is partially written into FIFO which had insufficient space. The frame is cut and frame error is set."]
        #[inline(always)]
        pub const fn set_wrfail_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LookUp Descriptor lost, because of unknown frame burst by MAC. If there is no MAC mailfunction then this flag will never be raised. FDFIFO requires reset."]
        #[must_use]
        #[inline(always)]
        pub const fn lu_desc_err(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LookUp Descriptor lost, because of unknown frame burst by MAC. If there is no MAC mailfunction then this flag will never be raised. FDFIFO requires reset."]
        #[inline(always)]
        pub const fn set_lu_desc_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for SwCtrlIgressRxFdfifoEErrorFlag {
        #[inline(always)]
        fn default() -> SwCtrlIgressRxFdfifoEErrorFlag {
            SwCtrlIgressRxFdfifoEErrorFlag(0)
        }
    }
    impl core::fmt::Debug for SwCtrlIgressRxFdfifoEErrorFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlIgressRxFdfifoEErrorFlag")
                .field("desc_seq_err", &self.desc_seq_err())
                .field("desc_nrdy_err", &self.desc_nrdy_err())
                .field("drop_full_mem", &self.drop_full_mem())
                .field("drop_full_desc", &self.drop_full_desc())
                .field("drop_nrdy", &self.drop_nrdy())
                .field("wrfail_full", &self.wrfail_full())
                .field("lu_desc_err", &self.lu_desc_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlIgressRxFdfifoEErrorFlag {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SwCtrlIgressRxFdfifoEErrorFlag {{ desc_seq_err: {=bool:?}, desc_nrdy_err: {=bool:?}, drop_full_mem: {=bool:?}, drop_full_desc: {=bool:?}, drop_nrdy: {=bool:?}, wrfail_full: {=bool:?}, lu_desc_err: {=bool:?} }}" , self . desc_seq_err () , self . desc_nrdy_err () , self . drop_full_mem () , self . drop_full_desc () , self . drop_nrdy () , self . wrfail_full () , self . lu_desc_err ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlIgressRxFdfifoEFdmemCntByte(pub u32);
    impl SwCtrlIgressRxFdfifoEFdmemCntByte {
        #[doc = "Number of bytes stored in frame drop FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn fdmem_cnt_byte(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Number of bytes stored in frame drop FIFO."]
        #[inline(always)]
        pub const fn set_fdmem_cnt_byte(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SwCtrlIgressRxFdfifoEFdmemCntByte {
        #[inline(always)]
        fn default() -> SwCtrlIgressRxFdfifoEFdmemCntByte {
            SwCtrlIgressRxFdfifoEFdmemCntByte(0)
        }
    }
    impl core::fmt::Debug for SwCtrlIgressRxFdfifoEFdmemCntByte {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlIgressRxFdfifoEFdmemCntByte")
                .field("fdmem_cnt_byte", &self.fdmem_cnt_byte())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlIgressRxFdfifoEFdmemCntByte {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SwCtrlIgressRxFdfifoEFdmemCntByte {{ fdmem_cnt_byte: {=u32:?} }}",
                self.fdmem_cnt_byte()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlIgressRxFdfifoEFdmemSts(pub u32);
    impl SwCtrlIgressRxFdfifoEFdmemSts {
        #[doc = "FD FIFO empty."]
        #[must_use]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO empty."]
        #[inline(always)]
        pub const fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FD FIFO almost empty. Few bytes in FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn amst_empty(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO almost empty. Few bytes in FIFO."]
        #[inline(always)]
        pub const fn set_amst_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FD FIFO almost full. Less than 1600 Byte left."]
        #[must_use]
        #[inline(always)]
        pub const fn amst_full(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO almost full. Less than 1600 Byte left."]
        #[inline(always)]
        pub const fn set_amst_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FD FIFO full."]
        #[must_use]
        #[inline(always)]
        pub const fn full(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO full."]
        #[inline(always)]
        pub const fn set_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FD FIFO ready to work or working."]
        #[must_use]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO ready to work or working."]
        #[inline(always)]
        pub const fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FD FIFO processes data."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO processes data."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FD FIFO waits for more frame data."]
        #[must_use]
        #[inline(always)]
        pub const fn wait_for_frame(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO waits for more frame data."]
        #[inline(always)]
        pub const fn set_wait_for_frame(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "FD FIFO waits for LookUp information."]
        #[must_use]
        #[inline(always)]
        pub const fn wait_for_lu(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "FD FIFO waits for LookUp information."]
        #[inline(always)]
        pub const fn set_wait_for_lu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for SwCtrlIgressRxFdfifoEFdmemSts {
        #[inline(always)]
        fn default() -> SwCtrlIgressRxFdfifoEFdmemSts {
            SwCtrlIgressRxFdfifoEFdmemSts(0)
        }
    }
    impl core::fmt::Debug for SwCtrlIgressRxFdfifoEFdmemSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlIgressRxFdfifoEFdmemSts")
                .field("empty", &self.empty())
                .field("amst_empty", &self.amst_empty())
                .field("amst_full", &self.amst_full())
                .field("full", &self.full())
                .field("ready", &self.ready())
                .field("busy", &self.busy())
                .field("wait_for_frame", &self.wait_for_frame())
                .field("wait_for_lu", &self.wait_for_lu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlIgressRxFdfifoEFdmemSts {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SwCtrlIgressRxFdfifoEFdmemSts {{ empty: {=bool:?}, amst_empty: {=bool:?}, amst_full: {=bool:?}, full: {=bool:?}, ready: {=bool:?}, busy: {=bool:?}, wait_for_frame: {=bool:?}, wait_for_lu: {=bool:?} }}" , self . empty () , self . amst_empty () , self . amst_full () , self . full () , self . ready () , self . busy () , self . wait_for_frame () , self . wait_for_lu ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlIgressRxFdfifoEIeErrorFlag(pub u32);
    impl SwCtrlIgressRxFdfifoEIeErrorFlag {
        #[doc = "Interrupt enable of ERROR_FLAG."]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Interrupt enable of ERROR_FLAG."]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for SwCtrlIgressRxFdfifoEIeErrorFlag {
        #[inline(always)]
        fn default() -> SwCtrlIgressRxFdfifoEIeErrorFlag {
            SwCtrlIgressRxFdfifoEIeErrorFlag(0)
        }
    }
    impl core::fmt::Debug for SwCtrlIgressRxFdfifoEIeErrorFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlIgressRxFdfifoEIeErrorFlag")
                .field("ie", &self.ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlIgressRxFdfifoEIeErrorFlag {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SwCtrlIgressRxFdfifoEIeErrorFlag {{ ie: {=u8:?} }}",
                self.ie()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlIgressRxFdfifoEInConfig(pub u32);
    impl SwCtrlIgressRxFdfifoEInConfig {
        #[doc = "FD_FIFO does not shorten frames which contain an error."]
        #[must_use]
        #[inline(always)]
        pub const fn nocut_error(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FD_FIFO does not shorten frames which contain an error."]
        #[inline(always)]
        pub const fn set_nocut_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SwCtrlIgressRxFdfifoEInConfig {
        #[inline(always)]
        fn default() -> SwCtrlIgressRxFdfifoEInConfig {
            SwCtrlIgressRxFdfifoEInConfig(0)
        }
    }
    impl core::fmt::Debug for SwCtrlIgressRxFdfifoEInConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlIgressRxFdfifoEInConfig")
                .field("nocut_error", &self.nocut_error())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlIgressRxFdfifoEInConfig {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SwCtrlIgressRxFdfifoEInConfig {{ nocut_error: {=bool:?} }}",
                self.nocut_error()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlIgressRxFdfifoEMirror(pub u32);
    impl SwCtrlIgressRxFdfifoEMirror {
        #[doc = "Mirror Port. If port mirroring is enabled TX/RX traffic will also be forwarded to this port. bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Mirror Port. If port mirroring is enabled TX/RX traffic will also be forwarded to this port. bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for SwCtrlIgressRxFdfifoEMirror {
        #[inline(always)]
        fn default() -> SwCtrlIgressRxFdfifoEMirror {
            SwCtrlIgressRxFdfifoEMirror(0)
        }
    }
    impl core::fmt::Debug for SwCtrlIgressRxFdfifoEMirror {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlIgressRxFdfifoEMirror")
                .field("port", &self.port())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlIgressRxFdfifoEMirror {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SwCtrlIgressRxFdfifoEMirror {{ port: {=u32:?} }}",
                self.port()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlIgressRxFdfifoEMirrorTx(pub u32);
    impl SwCtrlIgressRxFdfifoEMirrorTx {
        #[doc = "Mirror Selection TX. The destination of the frame is compared with this vector. All matching TX probe ports will be mirrored to MIRROR. It is necessary to configure all ingress ports to mirror the complete TX traffic. bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Mirror Selection TX. The destination of the frame is compared with this vector. All matching TX probe ports will be mirrored to MIRROR. It is necessary to configure all ingress ports to mirror the complete TX traffic. bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for SwCtrlIgressRxFdfifoEMirrorTx {
        #[inline(always)]
        fn default() -> SwCtrlIgressRxFdfifoEMirrorTx {
            SwCtrlIgressRxFdfifoEMirrorTx(0)
        }
    }
    impl core::fmt::Debug for SwCtrlIgressRxFdfifoEMirrorTx {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlIgressRxFdfifoEMirrorTx")
                .field("port", &self.port())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlIgressRxFdfifoEMirrorTx {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SwCtrlIgressRxFdfifoEMirrorTx {{ port: {=u32:?} }}",
                self.port()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlIgressRxFdfifoEOutConfig(pub u32);
    impl SwCtrlIgressRxFdfifoEOutConfig {
        #[doc = "Switch between Cut-Through and Store&Forward mode. 0 - Cut-Through 1 - Store&Forward."]
        #[must_use]
        #[inline(always)]
        pub const fn mode_store_fw(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Switch between Cut-Through and Store&Forward mode. 0 - Cut-Through 1 - Store&Forward."]
        #[inline(always)]
        pub const fn set_mode_store_fw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Do not drop frame errors."]
        #[must_use]
        #[inline(always)]
        pub const fn nodrop_error(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Do not drop frame errors."]
        #[inline(always)]
        pub const fn set_nodrop_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Duplicate frames to CPU."]
        #[must_use]
        #[inline(always)]
        pub const fn mirror_to_cpu(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Duplicate frames to CPU."]
        #[inline(always)]
        pub const fn set_mirror_to_cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Send error frames to CPU."]
        #[must_use]
        #[inline(always)]
        pub const fn error_to_cpu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Send error frames to CPU."]
        #[inline(always)]
        pub const fn set_error_to_cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Route all frames to DROP_DEST."]
        #[must_use]
        #[inline(always)]
        pub const fn drop_all(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Route all frames to DROP_DEST."]
        #[inline(always)]
        pub const fn set_drop_all(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Disable input of FD FIFO. Take care that also descriptor generation of LookUp is disabled. Remaining frames should be cleared with DROP_ALL."]
        #[must_use]
        #[inline(always)]
        pub const fn disable(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Disable input of FD FIFO. Take care that also descriptor generation of LookUp is disabled. Remaining frames should be cleared with DROP_ALL."]
        #[inline(always)]
        pub const fn set_disable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "If any Store&Forward option in RX_FDFIFO is set then this flag will still force preemptable traffic to be forwarded in Cut-Through mode. This is a useful option to save latency by double buffering if the used MAC/TSN-EP already does S&F."]
        #[must_use]
        #[inline(always)]
        pub const fn ct_fpe_ovrd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "If any Store&Forward option in RX_FDFIFO is set then this flag will still force preemptable traffic to be forwarded in Cut-Through mode. This is a useful option to save latency by double buffering if the used MAC/TSN-EP already does S&F."]
        #[inline(always)]
        pub const fn set_ct_fpe_ovrd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Incoming frames of this port will be mirrored to the given destination in MIRROR_RX."]
        #[must_use]
        #[inline(always)]
        pub const fn mirror_rx_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Incoming frames of this port will be mirrored to the given destination in MIRROR_RX."]
        #[inline(always)]
        pub const fn set_mirror_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Incoming frames of this port will be mirrored to the given destination in MIRROR if their destination match with MIRROR_TX."]
        #[must_use]
        #[inline(always)]
        pub const fn mirror_tx_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Incoming frames of this port will be mirrored to the given destination in MIRROR if their destination match with MIRROR_TX."]
        #[inline(always)]
        pub const fn set_mirror_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Bit mapped Destination for dropped frames. Typically, frames are cleared at destination 0. Use another value to stream frames for analysis. Supports only max range of port\\[15:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn drop_dest(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Bit mapped Destination for dropped frames. Typically, frames are cleared at destination 0. Use another value to stream frames for analysis. Supports only max range of port\\[15:0\\]."]
        #[inline(always)]
        pub const fn set_drop_dest(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SwCtrlIgressRxFdfifoEOutConfig {
        #[inline(always)]
        fn default() -> SwCtrlIgressRxFdfifoEOutConfig {
            SwCtrlIgressRxFdfifoEOutConfig(0)
        }
    }
    impl core::fmt::Debug for SwCtrlIgressRxFdfifoEOutConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlIgressRxFdfifoEOutConfig")
                .field("mode_store_fw", &self.mode_store_fw())
                .field("nodrop_error", &self.nodrop_error())
                .field("mirror_to_cpu", &self.mirror_to_cpu())
                .field("error_to_cpu", &self.error_to_cpu())
                .field("drop_all", &self.drop_all())
                .field("disable", &self.disable())
                .field("ct_fpe_ovrd", &self.ct_fpe_ovrd())
                .field("mirror_rx_en", &self.mirror_rx_en())
                .field("mirror_tx_en", &self.mirror_tx_en())
                .field("drop_dest", &self.drop_dest())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlIgressRxFdfifoEOutConfig {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SwCtrlIgressRxFdfifoEOutConfig {{ mode_store_fw: {=bool:?}, nodrop_error: {=bool:?}, mirror_to_cpu: {=bool:?}, error_to_cpu: {=bool:?}, drop_all: {=bool:?}, disable: {=bool:?}, ct_fpe_ovrd: {=bool:?}, mirror_rx_en: {=bool:?}, mirror_tx_en: {=bool:?}, drop_dest: {=u16:?} }}" , self . mode_store_fw () , self . nodrop_error () , self . mirror_to_cpu () , self . error_to_cpu () , self . drop_all () , self . disable () , self . ct_fpe_ovrd () , self . mirror_rx_en () , self . mirror_tx_en () , self . drop_dest ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlIgressRxFdfifoEParam(pub u32);
    impl SwCtrlIgressRxFdfifoEParam {
        #[doc = "Number of words (4byte) the Frame Drop FIFO can store."]
        #[must_use]
        #[inline(always)]
        pub const fn fd_fifo_desc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of words (4byte) the Frame Drop FIFO can store."]
        #[inline(always)]
        pub const fn set_fd_fifo_desc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Number of FD descriptors the FIFO can store. Two descriptors need to be stored per frame."]
        #[must_use]
        #[inline(always)]
        pub const fn fd_desc_fifo_desc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Number of FD descriptors the FIFO can store. Two descriptors need to be stored per frame."]
        #[inline(always)]
        pub const fn set_fd_desc_fifo_desc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Number of MAC lookup descriptors the FIFO can store."]
        #[must_use]
        #[inline(always)]
        pub const fn lu_fifo_depth(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Number of MAC lookup descriptors the FIFO can store."]
        #[inline(always)]
        pub const fn set_lu_fifo_depth(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for SwCtrlIgressRxFdfifoEParam {
        #[inline(always)]
        fn default() -> SwCtrlIgressRxFdfifoEParam {
            SwCtrlIgressRxFdfifoEParam(0)
        }
    }
    impl core::fmt::Debug for SwCtrlIgressRxFdfifoEParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlIgressRxFdfifoEParam")
                .field("fd_fifo_desc", &self.fd_fifo_desc())
                .field("fd_desc_fifo_desc", &self.fd_desc_fifo_desc())
                .field("lu_fifo_depth", &self.lu_fifo_depth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlIgressRxFdfifoEParam {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SwCtrlIgressRxFdfifoEParam {{ fd_fifo_desc: {=u16:?}, fd_desc_fifo_desc: {=u8:?}, lu_fifo_depth: {=u8:?} }}" , self . fd_fifo_desc () , self . fd_desc_fifo_desc () , self . lu_fifo_depth ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlIgressRxFdfifoEPortmask(pub u32);
    impl SwCtrlIgressRxFdfifoEPortmask {
        #[doc = "Port grouping via port mask. If the selected port is not set then the destination will be filtered out. This register allows the realization of port-based-VLAN (no VLAN tags required, only set it by ports). bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Port grouping via port mask. If the selected port is not set then the destination will be filtered out. This register allows the realization of port-based-VLAN (no VLAN tags required, only set it by ports). bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for SwCtrlIgressRxFdfifoEPortmask {
        #[inline(always)]
        fn default() -> SwCtrlIgressRxFdfifoEPortmask {
            SwCtrlIgressRxFdfifoEPortmask(0)
        }
    }
    impl core::fmt::Debug for SwCtrlIgressRxFdfifoEPortmask {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlIgressRxFdfifoEPortmask")
                .field("port", &self.port())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlIgressRxFdfifoEPortmask {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SwCtrlIgressRxFdfifoEPortmask {{ port: {=u32:?} }}",
                self.port()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlIgressRxFdfifoEReset(pub u32);
    impl SwCtrlIgressRxFdfifoEReset {
        #[doc = "Write 1 to reset FD controller and memory pointers. Register Map content remains untouched."]
        #[must_use]
        #[inline(always)]
        pub const fn softrs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Write 1 to reset FD controller and memory pointers. Register Map content remains untouched."]
        #[inline(always)]
        pub const fn set_softrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SwCtrlIgressRxFdfifoEReset {
        #[inline(always)]
        fn default() -> SwCtrlIgressRxFdfifoEReset {
            SwCtrlIgressRxFdfifoEReset(0)
        }
    }
    impl core::fmt::Debug for SwCtrlIgressRxFdfifoEReset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlIgressRxFdfifoEReset")
                .field("softrs", &self.softrs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlIgressRxFdfifoEReset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SwCtrlIgressRxFdfifoEReset {{ softrs: {=bool:?} }}",
                self.softrs()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlIgressRxFdfifoEStrfwd(pub u32);
    impl SwCtrlIgressRxFdfifoEStrfwd {
        #[doc = "If selected port is set then the frame is transmitted in Store & Forward mode. This is necessary when the ingress rate of this port is slower than the egress rate of the transmitting port. In S&F, the ingress module is able to drop frames with bad CRC.bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[must_use]
        #[inline(always)]
        pub const fn port(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "If selected port is set then the frame is transmitted in Store & Forward mode. This is necessary when the ingress rate of this port is slower than the egress rate of the transmitting port. In S&F, the ingress module is able to drop frames with bad CRC.bit 0 - CPU-Port, bit 1 - Port 1, …."]
        #[inline(always)]
        pub const fn set_port(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for SwCtrlIgressRxFdfifoEStrfwd {
        #[inline(always)]
        fn default() -> SwCtrlIgressRxFdfifoEStrfwd {
            SwCtrlIgressRxFdfifoEStrfwd(0)
        }
    }
    impl core::fmt::Debug for SwCtrlIgressRxFdfifoEStrfwd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlIgressRxFdfifoEStrfwd")
                .field("port", &self.port())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlIgressRxFdfifoEStrfwd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SwCtrlIgressRxFdfifoEStrfwd {{ port: {=u32:?} }}",
                self.port()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlMonitorCtrl(pub u32);
    impl SwCtrlMonitorCtrl {
        #[doc = "Enables counter. If deasserted the counter process stops and the counters hold their value."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enables counter. If deasserted the counter process stops and the counters hold their value."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SwCtrlMonitorCtrl {
        #[inline(always)]
        fn default() -> SwCtrlMonitorCtrl {
            SwCtrlMonitorCtrl(0)
        }
    }
    impl core::fmt::Debug for SwCtrlMonitorCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlMonitorCtrl")
                .field("en", &self.en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlMonitorCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SwCtrlMonitorCtrl {{ en: {=bool:?} }}", self.en())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlMonitorParam(pub u32);
    impl SwCtrlMonitorParam {
        #[doc = "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available."]
        #[must_use]
        #[inline(always)]
        pub const fn cntw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available."]
        #[inline(always)]
        pub const fn set_cntw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_cnt_en_vec(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available."]
        #[inline(always)]
        pub const fn set_tx_cnt_en_vec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_cnt_en_vec(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Vector of implemented RX counters. E.g. 0x000F means only the first 4 RX counter are available."]
        #[inline(always)]
        pub const fn set_rx_cnt_en_vec(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SwCtrlMonitorParam {
        #[inline(always)]
        fn default() -> SwCtrlMonitorParam {
            SwCtrlMonitorParam(0)
        }
    }
    impl core::fmt::Debug for SwCtrlMonitorParam {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlMonitorParam")
                .field("cntw", &self.cntw())
                .field("tx_cnt_en_vec", &self.tx_cnt_en_vec())
                .field("rx_cnt_en_vec", &self.rx_cnt_en_vec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlMonitorParam {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SwCtrlMonitorParam {{ cntw: {=u8:?}, tx_cnt_en_vec: {=u8:?}, rx_cnt_en_vec: {=u16:?} }}" , self . cntw () , self . tx_cnt_en_vec () , self . rx_cnt_en_vec ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlMonitorReset(pub u32);
    impl SwCtrlMonitorReset {
        #[doc = "Write '1' to reset all TX&RX counters."]
        #[must_use]
        #[inline(always)]
        pub const fn rsall(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Write '1' to reset all TX&RX counters."]
        #[inline(always)]
        pub const fn set_rsall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write '1' to reset all TX counters."]
        #[must_use]
        #[inline(always)]
        pub const fn rstx(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write '1' to reset all TX counters."]
        #[inline(always)]
        pub const fn set_rstx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Write '1' to reset all RX counters."]
        #[must_use]
        #[inline(always)]
        pub const fn rsrx(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Write '1' to reset all RX counters."]
        #[inline(always)]
        pub const fn set_rsrx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for SwCtrlMonitorReset {
        #[inline(always)]
        fn default() -> SwCtrlMonitorReset {
            SwCtrlMonitorReset(0)
        }
    }
    impl core::fmt::Debug for SwCtrlMonitorReset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlMonitorReset")
                .field("rsall", &self.rsall())
                .field("rstx", &self.rstx())
                .field("rsrx", &self.rsrx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlMonitorReset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SwCtrlMonitorReset {{ rsall: {=bool:?}, rstx: {=bool:?}, rsrx: {=bool:?} }}",
                self.rsall(),
                self.rstx(),
                self.rsrx()
            )
        }
    }
    #[doc = "Port Module Enable Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlPortMainEnnable(pub u32);
    impl SwCtrlPortMainEnnable {
        #[doc = "if QCI is present at selected egress port, '1' to use QCI and '0' disable QCI. Changing during frame operation can lead to frame corruption."]
        #[must_use]
        #[inline(always)]
        pub const fn en_qci(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "if QCI is present at selected egress port, '1' to use QCI and '0' disable QCI. Changing during frame operation can lead to frame corruption."]
        #[inline(always)]
        pub const fn set_en_qci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "only applicable for CPU-Port at egress: '1' to use S&F FIFO and '0' disable S&F FIFO. Changing during frame operation can lead to frame corruption."]
        #[must_use]
        #[inline(always)]
        pub const fn en_sf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "only applicable for CPU-Port at egress: '1' to use S&F FIFO and '0' disable S&F FIFO. Changing during frame operation can lead to frame corruption."]
        #[inline(always)]
        pub const fn set_en_sf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SwCtrlPortMainEnnable {
        #[inline(always)]
        fn default() -> SwCtrlPortMainEnnable {
            SwCtrlPortMainEnnable(0)
        }
    }
    impl core::fmt::Debug for SwCtrlPortMainEnnable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlPortMainEnnable")
                .field("en_qci", &self.en_qci())
                .field("en_sf", &self.en_sf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlPortMainEnnable {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SwCtrlPortMainEnnable {{ en_qci: {=bool:?}, en_sf: {=bool:?} }}",
                self.en_qci(),
                self.en_sf()
            )
        }
    }
    #[doc = "PVID Tagging Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SwCtrlPortMainTagging(pub u32);
    impl SwCtrlPortMainTagging {
        #[doc = "Native VLAN of Port. Untagged traffic will be tagged with the native VLAN-ID By default the Port uses VLAN 1."]
        #[must_use]
        #[inline(always)]
        pub const fn pvid(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Native VLAN of Port. Untagged traffic will be tagged with the native VLAN-ID By default the Port uses VLAN 1."]
        #[inline(always)]
        pub const fn set_pvid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "VLAN-TCI: Drop Eligible Indicator, used when tagged."]
        #[must_use]
        #[inline(always)]
        pub const fn dei(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "VLAN-TCI: Drop Eligible Indicator, used when tagged."]
        #[inline(always)]
        pub const fn set_dei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "VLAN-TCI: Priority Code Point, used when tagged."]
        #[must_use]
        #[inline(always)]
        pub const fn pcp(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "VLAN-TCI: Priority Code Point, used when tagged."]
        #[inline(always)]
        pub const fn set_pcp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Every tagged frame not matching PVID is filtered out. Every untagged ingress frame will be tagged with PVID. Every egress frame with PVID will be untagged."]
        #[must_use]
        #[inline(always)]
        pub const fn access(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Every tagged frame not matching PVID is filtered out. Every untagged ingress frame will be tagged with PVID. Every egress frame with PVID will be untagged."]
        #[inline(always)]
        pub const fn set_access(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "The VLAN-TAG with PVID will be inserted in every frame from Host as their first VLAN-TAG. This can be used for double tagging of tagged/trunk ports."]
        #[must_use]
        #[inline(always)]
        pub const fn force(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "The VLAN-TAG with PVID will be inserted in every frame from Host as their first VLAN-TAG. This can be used for double tagging of tagged/trunk ports."]
        #[inline(always)]
        pub const fn set_force(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for SwCtrlPortMainTagging {
        #[inline(always)]
        fn default() -> SwCtrlPortMainTagging {
            SwCtrlPortMainTagging(0)
        }
    }
    impl core::fmt::Debug for SwCtrlPortMainTagging {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SwCtrlPortMainTagging")
                .field("pvid", &self.pvid())
                .field("dei", &self.dei())
                .field("pcp", &self.pcp())
                .field("access", &self.access())
                .field("force", &self.force())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwCtrlPortMainTagging {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SwCtrlPortMainTagging {{ pvid: {=u16:?}, dei: {=bool:?}, pcp: {=u8:?}, access: {=bool:?}, force: {=bool:?} }}" , self . pvid () , self . dei () , self . pcp () , self . access () , self . force ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpCtrl(pub u32);
    impl TsnEpCtrl {
        #[doc = "TxTimestampFifo interrupt enable; interrupt will be set when IE_TSF=<1> and TSF_SR.USED>0."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_tsf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TxTimestampFifo interrupt enable; interrupt will be set when IE_TSF=<1> and TSF_SR.USED>0."]
        #[inline(always)]
        pub const fn set_ie_tsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable PTPv2 1-step synchronization suppor."]
        #[must_use]
        #[inline(always)]
        pub const fn ptp_1s_en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Enable PTPv2 1-step synchronization suppor."]
        #[inline(always)]
        pub const fn set_ptp_1s_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Disable filtering of PTP frames (Ethertype = 0x88F7)."]
        #[must_use]
        #[inline(always)]
        pub const fn filtdis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Disable filtering of PTP frames (Ethertype = 0x88F7)."]
        #[inline(always)]
        pub const fn set_filtdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for TsnEpCtrl {
        #[inline(always)]
        fn default() -> TsnEpCtrl {
            TsnEpCtrl(0)
        }
    }
    impl core::fmt::Debug for TsnEpCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpCtrl")
                .field("ie_tsf", &self.ie_tsf())
                .field("ptp_1s_en", &self.ptp_1s_en())
                .field("filtdis", &self.filtdis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnEpCtrl {{ ie_tsf: {=bool:?}, ptp_1s_en: {=bool:?}, filtdis: {=bool:?} }}",
                self.ie_tsf(),
                self.ptp_1s_en(),
                self.filtdis()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpIpcfg(pub u32);
    impl TsnEpIpcfg {
        #[doc = "IP core parameter “INCL_1STEP”."]
        #[must_use]
        #[inline(always)]
        pub const fn incl_1step(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "IP core parameter “INCL_1STEP”."]
        #[inline(always)]
        pub const fn set_incl_1step(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "IP core parameter “INCL_TSYNC”."]
        #[must_use]
        #[inline(always)]
        pub const fn incl_tsync(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "IP core parameter “INCL_TSYNC”."]
        #[inline(always)]
        pub const fn set_incl_tsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "IP core parameter “INCL_TSF”."]
        #[must_use]
        #[inline(always)]
        pub const fn incl_tsf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "IP core parameter “INCL_TSF”."]
        #[inline(always)]
        pub const fn set_incl_tsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "IP core parameter “INCL_FPE”."]
        #[must_use]
        #[inline(always)]
        pub const fn incl_fpe(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "IP core parameter “INCL_FPE”."]
        #[inline(always)]
        pub const fn set_incl_fpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "IP core parameter “INCL_SHAPER”."]
        #[must_use]
        #[inline(always)]
        pub const fn incl_shap(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "IP core parameter “INCL_SHAPER”."]
        #[inline(always)]
        pub const fn set_incl_shap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "IP core parameter “INCL_RTC”."]
        #[must_use]
        #[inline(always)]
        pub const fn incl_rtc(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "IP core parameter “INCL_RTC”."]
        #[inline(always)]
        pub const fn set_incl_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for TsnEpIpcfg {
        #[inline(always)]
        fn default() -> TsnEpIpcfg {
            TsnEpIpcfg(0)
        }
    }
    impl core::fmt::Debug for TsnEpIpcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpIpcfg")
                .field("incl_1step", &self.incl_1step())
                .field("incl_tsync", &self.incl_tsync())
                .field("incl_tsf", &self.incl_tsf())
                .field("incl_fpe", &self.incl_fpe())
                .field("incl_shap", &self.incl_shap())
                .field("incl_rtc", &self.incl_rtc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpIpcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TsnEpIpcfg {{ incl_1step: {=bool:?}, incl_tsync: {=bool:?}, incl_tsf: {=bool:?}, incl_fpe: {=bool:?}, incl_shap: {=bool:?}, incl_rtc: {=bool:?} }}" , self . incl_1step () , self . incl_tsync () , self . incl_tsf () , self . incl_fpe () , self . incl_shap () , self . incl_rtc ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpMmsCtrl(pub u32);
    impl TsnEpMmsCtrl {
        #[doc = "Enable preemption."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable preemption."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Link error."]
        #[must_use]
        #[inline(always)]
        pub const fn link(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Link error."]
        #[inline(always)]
        pub const fn set_link(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Disable verification."]
        #[must_use]
        #[inline(always)]
        pub const fn disv(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Disable verification."]
        #[inline(always)]
        pub const fn set_disv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Minimum non-final fragment size: 64 x (1 + FRAGSZ) – 4 octets."]
        #[must_use]
        #[inline(always)]
        pub const fn fragsz(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Minimum non-final fragment size: 64 x (1 + FRAGSZ) – 4 octets."]
        #[inline(always)]
        pub const fn set_fragsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "MMS statistic counter selection, value can be read in register MMS_STAT <000>: Frame reassembly error counter (802.3br, 30.14.1.8) <001>: Frames rejected due to wrong SMD (802.3br, 30.14.1.9) <010>: Frame assembly ok counter (802.3br, 30.14.1.10) <011>: Fragment rx counter (802.3br, 30.14.1.11) <100>: Fragment tx counter (802.3br, 30.14.1.12) <101>: Hold request counter (802.3br, 30.14.1.13) otherwise: <0>."]
        #[must_use]
        #[inline(always)]
        pub const fn statsel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "MMS statistic counter selection, value can be read in register MMS_STAT <000>: Frame reassembly error counter (802.3br, 30.14.1.8) <001>: Frames rejected due to wrong SMD (802.3br, 30.14.1.9) <010>: Frame assembly ok counter (802.3br, 30.14.1.10) <011>: Fragment rx counter (802.3br, 30.14.1.11) <100>: Fragment tx counter (802.3br, 30.14.1.12) <101>: Hold request counter (802.3br, 30.14.1.13) otherwise: <0>."]
        #[inline(always)]
        pub const fn set_statsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
    }
    impl Default for TsnEpMmsCtrl {
        #[inline(always)]
        fn default() -> TsnEpMmsCtrl {
            TsnEpMmsCtrl(0)
        }
    }
    impl core::fmt::Debug for TsnEpMmsCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpMmsCtrl")
                .field("en", &self.en())
                .field("link", &self.link())
                .field("disv", &self.disv())
                .field("fragsz", &self.fragsz())
                .field("statsel", &self.statsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpMmsCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TsnEpMmsCtrl {{ en: {=bool:?}, link: {=bool:?}, disv: {=bool:?}, fragsz: {=u8:?}, statsel: {=u8:?} }}" , self . en () , self . link () , self . disv () , self . fragsz () , self . statsel ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpMmsStat(pub u32);
    impl TsnEpMmsStat {
        #[doc = "Statistic counter of MMS, selected by MMS_CTRL.STATSEL,any write access will clear selected counter."]
        #[must_use]
        #[inline(always)]
        pub const fn counter(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Statistic counter of MMS, selected by MMS_CTRL.STATSEL,any write access will clear selected counter."]
        #[inline(always)]
        pub const fn set_counter(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsnEpMmsStat {
        #[inline(always)]
        fn default() -> TsnEpMmsStat {
            TsnEpMmsStat(0)
        }
    }
    impl core::fmt::Debug for TsnEpMmsStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpMmsStat")
                .field("counter", &self.counter())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpMmsStat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsnEpMmsStat {{ counter: {=u32:?} }}", self.counter())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpMmsSts(pub u32);
    impl TsnEpMmsSts {
        #[doc = "HOLD-Signal."]
        #[must_use]
        #[inline(always)]
        pub const fn hld(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HOLD-Signal."]
        #[inline(always)]
        pub const fn set_hld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "802.3br verification state ok; verification is done when any bit VFAIL or VOK is <1>."]
        #[must_use]
        #[inline(always)]
        pub const fn vok(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "802.3br verification state ok; verification is done when any bit VFAIL or VOK is <1>."]
        #[inline(always)]
        pub const fn set_vok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "802.3br verification state failure; verification is done when any bit VFAIL or VOK is <1>."]
        #[must_use]
        #[inline(always)]
        pub const fn vfail(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "802.3br verification state failure; verification is done when any bit VFAIL or VOK is <1>."]
        #[inline(always)]
        pub const fn set_vfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for TsnEpMmsSts {
        #[inline(always)]
        fn default() -> TsnEpMmsSts {
            TsnEpMmsSts(0)
        }
    }
    impl core::fmt::Debug for TsnEpMmsSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpMmsSts")
                .field("hld", &self.hld())
                .field("vok", &self.vok())
                .field("vfail", &self.vfail())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpMmsSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnEpMmsSts {{ hld: {=bool:?}, vok: {=bool:?}, vfail: {=bool:?} }}",
                self.hld(),
                self.vok(),
                self.vfail()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpMmsVtime(pub u32);
    impl TsnEpMmsVtime {
        #[doc = "802.3br verification timeout counter in <sys_clk> cycles. Must be set by software in range of 1ms to 128ms."]
        #[must_use]
        #[inline(always)]
        pub const fn vtime(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "802.3br verification timeout counter in <sys_clk> cycles. Must be set by software in range of 1ms to 128ms."]
        #[inline(always)]
        pub const fn set_vtime(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsnEpMmsVtime {
        #[inline(always)]
        fn default() -> TsnEpMmsVtime {
            TsnEpMmsVtime(0)
        }
    }
    impl core::fmt::Debug for TsnEpMmsVtime {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpMmsVtime")
                .field("vtime", &self.vtime())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpMmsVtime {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsnEpMmsVtime {{ vtime: {=u32:?} }}", self.vtime())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpPtpSr(pub u32);
    impl TsnEpPtpSr {
        #[doc = "Measured value of the deviation of the early timestamping for PTP frames. This value is informational only. The deviation is already included to the corrected “correctionField”."]
        #[must_use]
        #[inline(always)]
        pub const fn meas_ns(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Measured value of the deviation of the early timestamping for PTP frames. This value is informational only. The deviation is already included to the corrected “correctionField”."]
        #[inline(always)]
        pub const fn set_meas_ns(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for TsnEpPtpSr {
        #[inline(always)]
        fn default() -> TsnEpPtpSr {
            TsnEpPtpSr(0)
        }
    }
    impl core::fmt::Debug for TsnEpPtpSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpPtpSr")
                .field("meas_ns", &self.meas_ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpPtpSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsnEpPtpSr {{ meas_ns: {=u16:?} }}", self.meas_ns())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpPtpUptmNs(pub u32);
    impl TsnEpPtpUptmNs {
        #[doc = "PTP SYNC frame “upstreamTxTime” in format “seconds.nanoseconds” as potentially received by another TSN-EP port. The correction field of a transmitted PTP SYNC frame is modified by (egressTimestamp –upstreamTxTime), relative to the LocalClock. The “rateRatio” to the Grandmaster Clock is not taken into account."]
        #[must_use]
        #[inline(always)]
        pub const fn uptm_ns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PTP SYNC frame “upstreamTxTime” in format “seconds.nanoseconds” as potentially received by another TSN-EP port. The correction field of a transmitted PTP SYNC frame is modified by (egressTimestamp –upstreamTxTime), relative to the LocalClock. The “rateRatio” to the Grandmaster Clock is not taken into account."]
        #[inline(always)]
        pub const fn set_uptm_ns(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsnEpPtpUptmNs {
        #[inline(always)]
        fn default() -> TsnEpPtpUptmNs {
            TsnEpPtpUptmNs(0)
        }
    }
    impl core::fmt::Debug for TsnEpPtpUptmNs {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpPtpUptmNs")
                .field("uptm_ns", &self.uptm_ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpPtpUptmNs {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsnEpPtpUptmNs {{ uptm_ns: {=u32:?} }}", self.uptm_ns())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpPtpUptmS(pub u32);
    impl TsnEpPtpUptmS {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn uptm_ns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_uptm_ns(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsnEpPtpUptmS {
        #[inline(always)]
        fn default() -> TsnEpPtpUptmS {
            TsnEpPtpUptmS(0)
        }
    }
    impl core::fmt::Debug for TsnEpPtpUptmS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpPtpUptmS")
                .field("uptm_ns", &self.uptm_ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpPtpUptmS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsnEpPtpUptmS {{ uptm_ns: {=u32:?} }}", self.uptm_ns())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpTsfD0(pub u32);
    impl TsnEpTsfD0 {
        #[doc = "Tx-Timestamp-Fifo, lower 32 bit part of local time (curtime) at the start of transmission of the packet. Usually nanoseconds part when used with included RTC."]
        #[must_use]
        #[inline(always)]
        pub const fn tsf_ns(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx-Timestamp-Fifo, lower 32 bit part of local time (curtime) at the start of transmission of the packet. Usually nanoseconds part when used with included RTC."]
        #[inline(always)]
        pub const fn set_tsf_ns(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsnEpTsfD0 {
        #[inline(always)]
        fn default() -> TsnEpTsfD0 {
            TsnEpTsfD0(0)
        }
    }
    impl core::fmt::Debug for TsnEpTsfD0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpTsfD0")
                .field("tsf_ns", &self.tsf_ns())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpTsfD0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsnEpTsfD0 {{ tsf_ns: {=u32:?} }}", self.tsf_ns())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpTsfD1(pub u32);
    impl TsnEpTsfD1 {
        #[doc = "Tx-Timestamp-Fifo, upper 32 bit part of the local time (curtime) at the start of the transmission of the packet. Usually seconds part when used with included RTC."]
        #[must_use]
        #[inline(always)]
        pub const fn tsf_sec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Tx-Timestamp-Fifo, upper 32 bit part of the local time (curtime) at the start of the transmission of the packet. Usually seconds part when used with included RTC."]
        #[inline(always)]
        pub const fn set_tsf_sec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsnEpTsfD1 {
        #[inline(always)]
        fn default() -> TsnEpTsfD1 {
            TsnEpTsfD1(0)
        }
    }
    impl core::fmt::Debug for TsnEpTsfD1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpTsfD1")
                .field("tsf_sec", &self.tsf_sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpTsfD1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsnEpTsfD1 {{ tsf_sec: {=u32:?} }}", self.tsf_sec())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpTsfD2(pub u32);
    impl TsnEpTsfD2 {
        #[doc = "Tx-Timestamp-Fifo, user sideband <tx_tuser> of sent packet; Note: any read to register will remove actual value from FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn tsf_usr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Tx-Timestamp-Fifo, user sideband <tx_tuser> of sent packet; Note: any read to register will remove actual value from FIFO."]
        #[inline(always)]
        pub const fn set_tsf_usr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Tx-Timestamp-Fifo, traffic queue <tx_tqueue> of sent packet."]
        #[must_use]
        #[inline(always)]
        pub const fn tsf_tq(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[doc = "Tx-Timestamp-Fifo, traffic queue <tx_tqueue> of sent packet."]
        #[inline(always)]
        pub const fn set_tsf_tq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for TsnEpTsfD2 {
        #[inline(always)]
        fn default() -> TsnEpTsfD2 {
            TsnEpTsfD2(0)
        }
    }
    impl core::fmt::Debug for TsnEpTsfD2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpTsfD2")
                .field("tsf_usr", &self.tsf_usr())
                .field("tsf_tq", &self.tsf_tq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpTsfD2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnEpTsfD2 {{ tsf_usr: {=u8:?}, tsf_tq: {=u8:?} }}",
                self.tsf_usr(),
                self.tsf_tq()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpTsfSr(pub u32);
    impl TsnEpTsfSr {
        #[doc = "Tx-Timestamp-Fifo currently used entries counter; reading of TSF_Dx is only valid if field value > 0. Any read from TSF_D2 will decrement counter (unless already 0)."]
        #[must_use]
        #[inline(always)]
        pub const fn tsf_used(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Tx-Timestamp-Fifo currently used entries counter; reading of TSF_Dx is only valid if field value > 0. Any read from TSF_D2 will decrement counter (unless already 0)."]
        #[inline(always)]
        pub const fn set_tsf_used(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Overflow of Tx-Timestamp-Fifo. At least one transmitted packet has been sent and timestamp was not stored; write bit to clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tsf_ov(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Overflow of Tx-Timestamp-Fifo. At least one transmitted packet has been sent and timestamp was not stored; write bit to clear flag."]
        #[inline(always)]
        pub const fn set_tsf_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for TsnEpTsfSr {
        #[inline(always)]
        fn default() -> TsnEpTsfSr {
            TsnEpTsfSr(0)
        }
    }
    impl core::fmt::Debug for TsnEpTsfSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpTsfSr")
                .field("tsf_used", &self.tsf_used())
                .field("tsf_ov", &self.tsf_ov())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpTsfSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnEpTsfSr {{ tsf_used: {=u8:?}, tsf_ov: {=bool:?} }}",
                self.tsf_used(),
                self.tsf_ov()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpTxuf(pub u32);
    impl TsnEpTxuf {
        #[doc = "TX buffer underflow counter; incremented when any MAC runs out of data during transmission. The counter is cleared at any write access. The counter is shared by pMAC and eMAC. If underflow event occurs at the same time for pMAC and eMAC, it will be counted as one event."]
        #[must_use]
        #[inline(always)]
        pub const fn counter(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "TX buffer underflow counter; incremented when any MAC runs out of data during transmission. The counter is cleared at any write access. The counter is shared by pMAC and eMAC. If underflow event occurs at the same time for pMAC and eMAC, it will be counted as one event."]
        #[inline(always)]
        pub const fn set_counter(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsnEpTxuf {
        #[inline(always)]
        fn default() -> TsnEpTxuf {
            TsnEpTxuf(0)
        }
    }
    impl core::fmt::Debug for TsnEpTxuf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpTxuf")
                .field("counter", &self.counter())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpTxuf {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsnEpTxuf {{ counter: {=u32:?} }}", self.counter())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnEpVer(pub u32);
    impl TsnEpVer {
        #[doc = "revision number."]
        #[must_use]
        #[inline(always)]
        pub const fn ver_rev(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "revision number."]
        #[inline(always)]
        pub const fn set_ver_rev(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "minor version number."]
        #[must_use]
        #[inline(always)]
        pub const fn ver_lo(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "minor version number."]
        #[inline(always)]
        pub const fn set_ver_lo(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "major version number."]
        #[must_use]
        #[inline(always)]
        pub const fn ver_hi(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "major version number."]
        #[inline(always)]
        pub const fn set_ver_hi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for TsnEpVer {
        #[inline(always)]
        fn default() -> TsnEpVer {
            TsnEpVer(0)
        }
    }
    impl core::fmt::Debug for TsnEpVer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnEpVer")
                .field("ver_rev", &self.ver_rev())
                .field("ver_lo", &self.ver_lo())
                .field("ver_hi", &self.ver_hi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnEpVer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnEpVer {{ ver_rev: {=u8:?}, ver_lo: {=u8:?}, ver_hi: {=u8:?} }}",
                self.ver_rev(),
                self.ver_lo(),
                self.ver_hi()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperAclistEntry0H(pub u32);
    impl TsnShaperAclistEntry0H {
        #[doc = "Time interval, entry execution in in host clock ticks (<sys_clk>)."]
        #[must_use]
        #[inline(always)]
        pub const fn time(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Time interval, entry execution in in host clock ticks (<sys_clk>)."]
        #[inline(always)]
        pub const fn set_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsnShaperAclistEntry0H {
        #[inline(always)]
        fn default() -> TsnShaperAclistEntry0H {
            TsnShaperAclistEntry0H(0)
        }
    }
    impl core::fmt::Debug for TsnShaperAclistEntry0H {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperAclistEntry0H")
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperAclistEntry0H {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnShaperAclistEntry0H {{ time: {=u32:?} }}",
                self.time()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperAclistEntry0L(pub u32);
    impl TsnShaperAclistEntry0L {
        #[doc = "gate state vector; 1 – Gate is open."]
        #[must_use]
        #[inline(always)]
        pub const fn state(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "gate state vector; 1 – Gate is open."]
        #[inline(always)]
        pub const fn set_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "gate operation: 0 – SetGateStates 1 – Set-And-Hold-MAC 2 – Set-And-Release-MAC 3 – undefined."]
        #[must_use]
        #[inline(always)]
        pub const fn op(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "gate operation: 0 – SetGateStates 1 – Set-And-Hold-MAC 2 – Set-And-Release-MAC 3 – undefined."]
        #[inline(always)]
        pub const fn set_op(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "gate states for qch and ptp event source."]
        #[must_use]
        #[inline(always)]
        pub const fn tas_gpio(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0xff;
            val as u8
        }
        #[doc = "gate states for qch and ptp event source."]
        #[inline(always)]
        pub const fn set_tas_gpio(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 10usize)) | (((val as u32) & 0xff) << 10usize);
        }
    }
    impl Default for TsnShaperAclistEntry0L {
        #[inline(always)]
        fn default() -> TsnShaperAclistEntry0L {
            TsnShaperAclistEntry0L(0)
        }
    }
    impl core::fmt::Debug for TsnShaperAclistEntry0L {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperAclistEntry0L")
                .field("state", &self.state())
                .field("op", &self.op())
                .field("tas_gpio", &self.tas_gpio())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperAclistEntry0L {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnShaperAclistEntry0L {{ state: {=u8:?}, op: {=u8:?}, tas_gpio: {=u8:?} }}",
                self.state(),
                self.op(),
                self.tas_gpio()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperFpst(pub u32);
    impl TsnShaperFpst {
        #[doc = "Frame Preemption Status Table, Bit\\[i\\]
= 1: Preemptable traffic in TQ\\[i\\], otherwise Express traffic (default)."]
        #[must_use]
        #[inline(always)]
        pub const fn table(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Frame Preemption Status Table, Bit\\[i\\]
= 1: Preemptable traffic in TQ\\[i\\], otherwise Express traffic (default)."]
        #[inline(always)]
        pub const fn set_table(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TsnShaperFpst {
        #[inline(always)]
        fn default() -> TsnShaperFpst {
            TsnShaperFpst(0)
        }
    }
    impl core::fmt::Debug for TsnShaperFpst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperFpst")
                .field("table", &self.table())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperFpst {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsnShaperFpst {{ table: {=u8:?} }}", self.table())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperHoldadv(pub u32);
    impl TsnShaperHoldadv {
        #[doc = "holdAdvance time for TAS operation Set-And-Hold-MAC in <sys_clk> cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "holdAdvance time for TAS operation Set-And-Hold-MAC in <sys_clk> cycles."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for TsnShaperHoldadv {
        #[inline(always)]
        fn default() -> TsnShaperHoldadv {
            TsnShaperHoldadv(0)
        }
    }
    impl core::fmt::Debug for TsnShaperHoldadv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperHoldadv")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperHoldadv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsnShaperHoldadv {{ value: {=u16:?} }}", self.value())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperHwcfg1(pub u32);
    impl TsnShaperHwcfg1 {
        #[doc = "Traffic queue data width (Bytes); fixed to value 4 within IP core."]
        #[must_use]
        #[inline(always)]
        pub const fn dw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Traffic queue data width (Bytes); fixed to value 4 within IP core."]
        #[inline(always)]
        pub const fn set_dw(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Traffic queue depth (IP core parameter TQD)."]
        #[must_use]
        #[inline(always)]
        pub const fn tqd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Traffic queue depth (IP core parameter TQD)."]
        #[inline(always)]
        pub const fn set_tqd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Traffic queue count (IP core parameter TQC)."]
        #[must_use]
        #[inline(always)]
        pub const fn tqc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Traffic queue count (IP core parameter TQC)."]
        #[inline(always)]
        pub const fn set_tqc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Scheduler list address width (IP core parameter LWIDTH)."]
        #[must_use]
        #[inline(always)]
        pub const fn lwidth(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Scheduler list address width (IP core parameter LWIDTH)."]
        #[inline(always)]
        pub const fn set_lwidth(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for TsnShaperHwcfg1 {
        #[inline(always)]
        fn default() -> TsnShaperHwcfg1 {
            TsnShaperHwcfg1(0)
        }
    }
    impl core::fmt::Debug for TsnShaperHwcfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperHwcfg1")
                .field("dw", &self.dw())
                .field("tqd", &self.tqd())
                .field("tqc", &self.tqc())
                .field("lwidth", &self.lwidth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperHwcfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnShaperHwcfg1 {{ dw: {=u8:?}, tqd: {=u8:?}, tqc: {=u8:?}, lwidth: {=u8:?} }}",
                self.dw(),
                self.tqd(),
                self.tqc(),
                self.lwidth()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperMmct(pub u32);
    impl TsnShaperMmct {
        #[doc = "Request HOLD-Signal hold operation. Will be automatically set to <0>."]
        #[must_use]
        #[inline(always)]
        pub const fn rqhld(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Request HOLD-Signal hold operation. Will be automatically set to <0>."]
        #[inline(always)]
        pub const fn set_rqhld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Request HOLD-Signal release operation. Will be automatically set to <0>."]
        #[must_use]
        #[inline(always)]
        pub const fn rqrel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Request HOLD-Signal release operation. Will be automatically set to <0>."]
        #[inline(always)]
        pub const fn set_rqrel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for TsnShaperMmct {
        #[inline(always)]
        fn default() -> TsnShaperMmct {
            TsnShaperMmct(0)
        }
    }
    impl core::fmt::Debug for TsnShaperMmct {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperMmct")
                .field("rqhld", &self.rqhld())
                .field("rqrel", &self.rqrel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperMmct {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnShaperMmct {{ rqhld: {=bool:?}, rqrel: {=bool:?} }}",
                self.rqhld(),
                self.rqrel()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperTasAbasetmH(pub u32);
    impl TsnShaperTasAbasetmH {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn basetm_h(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_basetm_h(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsnShaperTasAbasetmH {
        #[inline(always)]
        fn default() -> TsnShaperTasAbasetmH {
            TsnShaperTasAbasetmH(0)
        }
    }
    impl core::fmt::Debug for TsnShaperTasAbasetmH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperTasAbasetmH")
                .field("basetm_h", &self.basetm_h())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperTasAbasetmH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnShaperTasAbasetmH {{ basetm_h: {=u32:?} }}",
                self.basetm_h()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperTasAbasetmL(pub u32);
    impl TsnShaperTasAbasetmL {
        #[doc = "Admin basetime – nanoseconds and seconds part."]
        #[must_use]
        #[inline(always)]
        pub const fn basetm_l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Admin basetime – nanoseconds and seconds part."]
        #[inline(always)]
        pub const fn set_basetm_l(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for TsnShaperTasAbasetmL {
        #[inline(always)]
        fn default() -> TsnShaperTasAbasetmL {
            TsnShaperTasAbasetmL(0)
        }
    }
    impl core::fmt::Debug for TsnShaperTasAbasetmL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperTasAbasetmL")
                .field("basetm_l", &self.basetm_l())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperTasAbasetmL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnShaperTasAbasetmL {{ basetm_l: {=u32:?} }}",
                self.basetm_l()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperTasAcycletm(pub u32);
    impl TsnShaperTasAcycletm {
        #[doc = "Admin cycletime in nanoseconds."]
        #[must_use]
        #[inline(always)]
        pub const fn ctime(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Admin cycletime in nanoseconds."]
        #[inline(always)]
        pub const fn set_ctime(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for TsnShaperTasAcycletm {
        #[inline(always)]
        fn default() -> TsnShaperTasAcycletm {
            TsnShaperTasAcycletm(0)
        }
    }
    impl core::fmt::Debug for TsnShaperTasAcycletm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperTasAcycletm")
                .field("ctime", &self.ctime())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperTasAcycletm {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnShaperTasAcycletm {{ ctime: {=u32:?} }}",
                self.ctime()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperTasCrsr(pub u32);
    impl TsnShaperTasCrsr {
        #[doc = "Enable time aware scheduling."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable time aware scheduling."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Switch configuration; Bit is automatically reset to 0; Setting Bit=1 triggers configuration change event."]
        #[must_use]
        #[inline(always)]
        pub const fn cfgchg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Switch configuration; Bit is automatically reset to 0; Setting Bit=1 triggers configuration change event."]
        #[inline(always)]
        pub const fn set_cfgchg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Configuration error."]
        #[must_use]
        #[inline(always)]
        pub const fn cfgerr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration error."]
        #[inline(always)]
        pub const fn set_cfgerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Configuration change is pending – Admin basetime not yet reached."]
        #[must_use]
        #[inline(always)]
        pub const fn cfgpend(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration change is pending – Admin basetime not yet reached."]
        #[inline(always)]
        pub const fn set_cfgpend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "operational tas gpio gate status of TQ\\[i\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn tas_gpio_sta(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "operational tas gpio gate status of TQ\\[i\\]."]
        #[inline(always)]
        pub const fn set_tas_gpio_sta(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Operational gate states of TQ\\[i\\]
(i = 0 – TQC-1) Bit\\[i\\]=0 – Gate is closed; no start of frame TX possible Bit\\[i\\]=1 – Gate is open."]
        #[must_use]
        #[inline(always)]
        pub const fn opergs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Operational gate states of TQ\\[i\\]
(i = 0 – TQC-1) Bit\\[i\\]=0 – Gate is closed; no start of frame TX possible Bit\\[i\\]=1 – Gate is open."]
        #[inline(always)]
        pub const fn set_opergs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Admin gate states, fixed 0xFF. Gate states when TAS is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn admings(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Admin gate states, fixed 0xFF. Gate states when TAS is disabled."]
        #[inline(always)]
        pub const fn set_admings(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for TsnShaperTasCrsr {
        #[inline(always)]
        fn default() -> TsnShaperTasCrsr {
            TsnShaperTasCrsr(0)
        }
    }
    impl core::fmt::Debug for TsnShaperTasCrsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperTasCrsr")
                .field("en", &self.en())
                .field("cfgchg", &self.cfgchg())
                .field("cfgerr", &self.cfgerr())
                .field("cfgpend", &self.cfgpend())
                .field("tas_gpio_sta", &self.tas_gpio_sta())
                .field("opergs", &self.opergs())
                .field("admings", &self.admings())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperTasCrsr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TsnShaperTasCrsr {{ en: {=bool:?}, cfgchg: {=bool:?}, cfgerr: {=bool:?}, cfgpend: {=bool:?}, tas_gpio_sta: {=u8:?}, opergs: {=u8:?}, admings: {=u8:?} }}" , self . en () , self . cfgchg () , self . cfgerr () , self . cfgpend () , self . tas_gpio_sta () , self . opergs () , self . admings ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperTasListlen(pub u32);
    impl TsnShaperTasListlen {
        #[doc = "Admin list length."]
        #[must_use]
        #[inline(always)]
        pub const fn alistlen(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Admin list length."]
        #[inline(always)]
        pub const fn set_alistlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Oper list length."]
        #[must_use]
        #[inline(always)]
        pub const fn olistlen(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Oper list length."]
        #[inline(always)]
        pub const fn set_olistlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for TsnShaperTasListlen {
        #[inline(always)]
        fn default() -> TsnShaperTasListlen {
            TsnShaperTasListlen(0)
        }
    }
    impl core::fmt::Debug for TsnShaperTasListlen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperTasListlen")
                .field("alistlen", &self.alistlen())
                .field("olistlen", &self.olistlen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperTasListlen {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnShaperTasListlen {{ alistlen: {=u8:?}, olistlen: {=u8:?} }}",
                self.alistlen(),
                self.olistlen()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperTasObasetmH(pub u32);
    impl TsnShaperTasObasetmH {
        #[doc = "No description available."]
        #[must_use]
        #[inline(always)]
        pub const fn basetm_h(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "No description available."]
        #[inline(always)]
        pub const fn set_basetm_h(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsnShaperTasObasetmH {
        #[inline(always)]
        fn default() -> TsnShaperTasObasetmH {
            TsnShaperTasObasetmH(0)
        }
    }
    impl core::fmt::Debug for TsnShaperTasObasetmH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperTasObasetmH")
                .field("basetm_h", &self.basetm_h())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperTasObasetmH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnShaperTasObasetmH {{ basetm_h: {=u32:?} }}",
                self.basetm_h()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperTasObasetmL(pub u32);
    impl TsnShaperTasObasetmL {
        #[doc = "Operational basetime – nanoseconds and seconds part. The operational basetime might occasionally have a non-normalized value (ns >= 10^9) for one clock cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn basetm_l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Operational basetime – nanoseconds and seconds part. The operational basetime might occasionally have a non-normalized value (ns >= 10^9) for one clock cycle."]
        #[inline(always)]
        pub const fn set_basetm_l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsnShaperTasObasetmL {
        #[inline(always)]
        fn default() -> TsnShaperTasObasetmL {
            TsnShaperTasObasetmL(0)
        }
    }
    impl core::fmt::Debug for TsnShaperTasObasetmL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperTasObasetmL")
                .field("basetm_l", &self.basetm_l())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperTasObasetmL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnShaperTasObasetmL {{ basetm_l: {=u32:?} }}",
                self.basetm_l()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperTasOcycletm(pub u32);
    impl TsnShaperTasOcycletm {
        #[doc = "Operational cycletime in nanoseconds."]
        #[must_use]
        #[inline(always)]
        pub const fn ctime(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Operational cycletime in nanoseconds."]
        #[inline(always)]
        pub const fn set_ctime(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
        }
    }
    impl Default for TsnShaperTasOcycletm {
        #[inline(always)]
        fn default() -> TsnShaperTasOcycletm {
            TsnShaperTasOcycletm(0)
        }
    }
    impl core::fmt::Debug for TsnShaperTasOcycletm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperTasOcycletm")
                .field("ctime", &self.ctime())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperTasOcycletm {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnShaperTasOcycletm {{ ctime: {=u32:?} }}",
                self.ctime()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperTqav(pub u32);
    impl TsnShaperTqav {
        #[doc = "Traffic queue buffer space available for complete packet of size MaxSDU (register MXSDUi) Bit\\[i\\]
= 1: space available Bit\\[i\\]
= 0: no space available or TQ not implemented (I >= TQC)."]
        #[must_use]
        #[inline(always)]
        pub const fn avail(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Traffic queue buffer space available for complete packet of size MaxSDU (register MXSDUi) Bit\\[i\\]
= 1: space available Bit\\[i\\]
= 0: no space available or TQ not implemented (I >= TQC)."]
        #[inline(always)]
        pub const fn set_avail(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Traffic queue interrupt enable on buffer space available, one bit per traffic queue Bit\\[i\\]
= 0: no interrupt Bit\\[i\\]
= 1: interrupt, when AVAIL\\[i\\]=1."]
        #[must_use]
        #[inline(always)]
        pub const fn avie(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Traffic queue interrupt enable on buffer space available, one bit per traffic queue Bit\\[i\\]
= 0: no interrupt Bit\\[i\\]
= 1: interrupt, when AVAIL\\[i\\]=1."]
        #[inline(always)]
        pub const fn set_avie(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for TsnShaperTqav {
        #[inline(always)]
        fn default() -> TsnShaperTqav {
            TsnShaperTqav(0)
        }
    }
    impl core::fmt::Debug for TsnShaperTqav {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperTqav")
                .field("avail", &self.avail())
                .field("avie", &self.avie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperTqav {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsnShaperTqav {{ avail: {=u8:?}, avie: {=u8:?} }}",
                self.avail(),
                self.avie()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsnShaperTqem(pub u32);
    impl TsnShaperTqem {
        #[doc = "Traffic queue empty Bit\\[i\\]
= 1: traffic queue i is empty."]
        #[must_use]
        #[inline(always)]
        pub const fn empty(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Traffic queue empty Bit\\[i\\]
= 1: traffic queue i is empty."]
        #[inline(always)]
        pub const fn set_empty(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TsnShaperTqem {
        #[inline(always)]
        fn default() -> TsnShaperTqem {
            TsnShaperTqem(0)
        }
    }
    impl core::fmt::Debug for TsnShaperTqem {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsnShaperTqem")
                .field("empty", &self.empty())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsnShaperTqem {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsnShaperTqem {{ empty: {=u8:?} }}", self.empty())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsynCr(pub u32);
    impl TsynCr {
        #[doc = "Tx Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Interrupt Enable."]
        #[inline(always)]
        pub const fn set_txie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Rx Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Rx Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rxie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tmrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer Interrupt Enable."]
        #[inline(always)]
        pub const fn set_tmrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer Enable: every bit corresponds to Timer 0 – 4."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_en(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Timer Enable: every bit corresponds to Timer 0 – 4."]
        #[inline(always)]
        pub const fn set_tmr_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Timer Auto Load: automatic reloading of timer when reaching 0. Done flag stays set after countdown. Used for periodic events, when following event shall not be delayed by host interaction."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_ald(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Timer Auto Load: automatic reloading of timer when reaching 0. Done flag stays set after countdown. Used for periodic events, when following event shall not be delayed by host interaction."]
        #[inline(always)]
        pub const fn set_tmr_ald(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for TsynCr {
        #[inline(always)]
        fn default() -> TsynCr {
            TsynCr(0)
        }
    }
    impl core::fmt::Debug for TsynCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsynCr")
                .field("txie", &self.txie())
                .field("rxie", &self.rxie())
                .field("tmrie", &self.tmrie())
                .field("tmr_en", &self.tmr_en())
                .field("tmr_ald", &self.tmr_ald())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsynCr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TsynCr {{ txie: {=bool:?}, rxie: {=bool:?}, tmrie: {=bool:?}, tmr_en: {=u8:?}, tmr_ald: {=u8:?} }}" , self . txie () , self . rxie () , self . tmrie () , self . tmr_en () , self . tmr_ald ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsynHclkdiv(pub u32);
    impl TsynHclkdiv {
        #[doc = "Period in host clocks <sys_clk>. Host clock shall be scaled to ticks of 1/1024th second. Ticks are used by timer TMR0 – TMR4."]
        #[must_use]
        #[inline(always)]
        pub const fn period(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Period in host clocks <sys_clk>. Host clock shall be scaled to ticks of 1/1024th second. Ticks are used by timer TMR0 – TMR4."]
        #[inline(always)]
        pub const fn set_period(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for TsynHclkdiv {
        #[inline(always)]
        fn default() -> TsynHclkdiv {
            TsynHclkdiv(0)
        }
    }
    impl core::fmt::Debug for TsynHclkdiv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsynHclkdiv")
                .field("period", &self.period())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsynHclkdiv {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsynHclkdiv {{ period: {=u32:?} }}", self.period())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsynPtpRxSts(pub u32);
    impl TsynPtpRxSts {
        #[doc = "Current selected RX buffer for reading (0-7). Can be used to determine when RX buffer has been switched after setting PTP_RX_STS.NXT."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Current selected RX buffer for reading (0-7). Can be used to determine when RX buffer has been switched after setting PTP_RX_STS.NXT."]
        #[inline(always)]
        pub const fn set_rx_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Read access: buffer data available – reading data from RX_BUF is valid. Write access: switch to next RX buffer – shall only be done when buffer not empty (AV=1). Use field RX_SEL as indication when rx buffer switch has been done."]
        #[must_use]
        #[inline(always)]
        pub const fn av_nxt(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Read access: buffer data available – reading data from RX_BUF is valid. Write access: switch to next RX buffer – shall only be done when buffer not empty (AV=1). Use field RX_SEL as indication when rx buffer switch has been done."]
        #[inline(always)]
        pub const fn set_av_nxt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "FIFO overflow flag. PTP frame has been received and there was no free buffer available. Data has been lost."]
        #[must_use]
        #[inline(always)]
        pub const fn ov(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow flag. PTP frame has been received and there was no free buffer available. Data has been lost."]
        #[inline(always)]
        pub const fn set_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for TsynPtpRxSts {
        #[inline(always)]
        fn default() -> TsynPtpRxSts {
            TsynPtpRxSts(0)
        }
    }
    impl core::fmt::Debug for TsynPtpRxSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsynPtpRxSts")
                .field("rx_sel", &self.rx_sel())
                .field("av_nxt", &self.av_nxt())
                .field("ov", &self.ov())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsynPtpRxSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsynPtpRxSts {{ rx_sel: {=u8:?}, av_nxt: {=bool:?}, ov: {=bool:?} }}",
                self.rx_sel(),
                self.av_nxt(),
                self.ov()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsynPtpTxDone(pub u32);
    impl TsynPtpTxDone {
        #[doc = "Transmission done status of PTP TX bin n (bit 0 – 7 correspond to tx bin 0 – 7). 1: transmission done. Writing a ‘1’ clears corresponding bit.."]
        #[must_use]
        #[inline(always)]
        pub const fn done(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Transmission done status of PTP TX bin n (bit 0 – 7 correspond to tx bin 0 – 7). 1: transmission done. Writing a ‘1’ clears corresponding bit.."]
        #[inline(always)]
        pub const fn set_done(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TsynPtpTxDone {
        #[inline(always)]
        fn default() -> TsynPtpTxDone {
            TsynPtpTxDone(0)
        }
    }
    impl core::fmt::Debug for TsynPtpTxDone {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsynPtpTxDone")
                .field("done", &self.done())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsynPtpTxDone {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsynPtpTxDone {{ done: {=u8:?} }}", self.done())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsynPtpTxSts(pub u32);
    impl TsynPtpTxSts {
        #[doc = "Transmission status of PTP TX bin n (bit 0 – 7 correspond to tx bin 0 – 7). 1: transmission pending."]
        #[must_use]
        #[inline(always)]
        pub const fn sts(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Transmission status of PTP TX bin n (bit 0 – 7 correspond to tx bin 0 – 7). 1: transmission pending."]
        #[inline(always)]
        pub const fn set_sts(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TsynPtpTxSts {
        #[inline(always)]
        fn default() -> TsynPtpTxSts {
            TsynPtpTxSts(0)
        }
    }
    impl core::fmt::Debug for TsynPtpTxSts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsynPtpTxSts")
                .field("sts", &self.sts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsynPtpTxSts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsynPtpTxSts {{ sts: {=u8:?} }}", self.sts())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsynPtpTxTrig(pub u32);
    impl TsynPtpTxTrig {
        #[doc = "Trigger PTP TX bin n (bit 0 – 7 correspond to tx bin 0 –7). Writing ‘1’ will trigger transmission. Corresponding bit PTP_TX_STS.STS(n) will be set immediately."]
        #[must_use]
        #[inline(always)]
        pub const fn trig(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Trigger PTP TX bin n (bit 0 – 7 correspond to tx bin 0 –7). Writing ‘1’ will trigger transmission. Corresponding bit PTP_TX_STS.STS(n) will be set immediately."]
        #[inline(always)]
        pub const fn set_trig(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for TsynPtpTxTrig {
        #[inline(always)]
        fn default() -> TsynPtpTxTrig {
            TsynPtpTxTrig(0)
        }
    }
    impl core::fmt::Debug for TsynPtpTxTrig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsynPtpTxTrig")
                .field("trig", &self.trig())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsynPtpTxTrig {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TsynPtpTxTrig {{ trig: {=u8:?} }}", self.trig())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsynRxbufRxFrameLengthBytes(pub u32);
    impl TsynRxbufRxFrameLengthBytes {
        #[doc = "RX frame length bytes \\[11:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_frame_length_bytes(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "RX frame length bytes \\[11:0\\]."]
        #[inline(always)]
        pub const fn set_rx_frame_length_bytes(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for TsynRxbufRxFrameLengthBytes {
        #[inline(always)]
        fn default() -> TsynRxbufRxFrameLengthBytes {
            TsynRxbufRxFrameLengthBytes(0)
        }
    }
    impl core::fmt::Debug for TsynRxbufRxFrameLengthBytes {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsynRxbufRxFrameLengthBytes")
                .field("rx_frame_length_bytes", &self.rx_frame_length_bytes())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsynRxbufRxFrameLengthBytes {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsynRxbufRxFrameLengthBytes {{ rx_frame_length_bytes: {=u16:?} }}",
                self.rx_frame_length_bytes()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsynRxbufRxTimeStampH(pub u32);
    impl TsynRxbufRxTimeStampH {
        #[doc = "RX Timestamp \\[63:32\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_timestamp_high(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RX Timestamp \\[63:32\\]."]
        #[inline(always)]
        pub const fn set_rx_timestamp_high(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsynRxbufRxTimeStampH {
        #[inline(always)]
        fn default() -> TsynRxbufRxTimeStampH {
            TsynRxbufRxTimeStampH(0)
        }
    }
    impl core::fmt::Debug for TsynRxbufRxTimeStampH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsynRxbufRxTimeStampH")
                .field("rx_timestamp_high", &self.rx_timestamp_high())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsynRxbufRxTimeStampH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsynRxbufRxTimeStampH {{ rx_timestamp_high: {=u32:?} }}",
                self.rx_timestamp_high()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsynRxbufRxTimeStampL(pub u32);
    impl TsynRxbufRxTimeStampL {
        #[doc = "RX Timestamp \\[31:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_timestamp_low(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RX Timestamp \\[31:0\\]."]
        #[inline(always)]
        pub const fn set_rx_timestamp_low(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsynRxbufRxTimeStampL {
        #[inline(always)]
        fn default() -> TsynRxbufRxTimeStampL {
            TsynRxbufRxTimeStampL(0)
        }
    }
    impl core::fmt::Debug for TsynRxbufRxTimeStampL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsynRxbufRxTimeStampL")
                .field("rx_timestamp_low", &self.rx_timestamp_low())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsynRxbufRxTimeStampL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsynRxbufRxTimeStampL {{ rx_timestamp_low: {=u32:?} }}",
                self.rx_timestamp_low()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsynSr(pub u32);
    impl TsynSr {
        #[doc = "Tx Done Interrupt Status: OR’ed PTP_TX_DONE."]
        #[must_use]
        #[inline(always)]
        pub const fn txis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tx Done Interrupt Status: OR’ed PTP_TX_DONE."]
        #[inline(always)]
        pub const fn set_txis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Rx Interrupt Status, RX buffer data available equal to PTP_RX_STS.AV)."]
        #[must_use]
        #[inline(always)]
        pub const fn rxis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Rx Interrupt Status, RX buffer data available equal to PTP_RX_STS.AV)."]
        #[inline(always)]
        pub const fn set_rxis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer Interrupt Status: OR’ed (TMR_DN AND TMR_EN) flags. 1 when timer is enabled and countdown is done."]
        #[must_use]
        #[inline(always)]
        pub const fn tmris(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer Interrupt Status: OR’ed (TMR_DN AND TMR_EN) flags. 1 when timer is enabled and countdown is done."]
        #[inline(always)]
        pub const fn set_tmris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer Done: 1 when timer reached 0."]
        #[must_use]
        #[inline(always)]
        pub const fn tmr_dn(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Timer Done: 1 when timer reached 0."]
        #[inline(always)]
        pub const fn set_tmr_dn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for TsynSr {
        #[inline(always)]
        fn default() -> TsynSr {
            TsynSr(0)
        }
    }
    impl core::fmt::Debug for TsynSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsynSr")
                .field("txis", &self.txis())
                .field("rxis", &self.rxis())
                .field("tmris", &self.tmris())
                .field("tmr_dn", &self.tmr_dn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsynSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsynSr {{ txis: {=bool:?}, rxis: {=bool:?}, tmris: {=bool:?}, tmr_dn: {=u8:?} }}",
                self.txis(),
                self.rxis(),
                self.tmris(),
                self.tmr_dn()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsynTxbufBin0TqueAndTxLen(pub u32);
    impl TsynTxbufBin0TqueAndTxLen {
        #[doc = "TXBUF_BIN0_TX_LEN."]
        #[must_use]
        #[inline(always)]
        pub const fn txbuf_bin0_tx_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "TXBUF_BIN0_TX_LEN."]
        #[inline(always)]
        pub const fn set_txbuf_bin0_tx_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "TXBUF_BIN0_TQUE."]
        #[must_use]
        #[inline(always)]
        pub const fn txbuf_bin0_tque(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "TXBUF_BIN0_TQUE."]
        #[inline(always)]
        pub const fn set_txbuf_bin0_tque(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
    }
    impl Default for TsynTxbufBin0TqueAndTxLen {
        #[inline(always)]
        fn default() -> TsynTxbufBin0TqueAndTxLen {
            TsynTxbufBin0TqueAndTxLen(0)
        }
    }
    impl core::fmt::Debug for TsynTxbufBin0TqueAndTxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsynTxbufBin0TqueAndTxLen")
                .field("txbuf_bin0_tx_len", &self.txbuf_bin0_tx_len())
                .field("txbuf_bin0_tque", &self.txbuf_bin0_tque())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsynTxbufBin0TqueAndTxLen {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TsynTxbufBin0TqueAndTxLen {{ txbuf_bin0_tx_len: {=u8:?}, txbuf_bin0_tque: {=u8:?} }}" , self . txbuf_bin0_tx_len () , self . txbuf_bin0_tque ())
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsynTxbufBin0TxTimestampH(pub u32);
    impl TsynTxbufBin0TxTimestampH {
        #[doc = "TXBUF_BIN0TX_TIMESTAMP_H."]
        #[must_use]
        #[inline(always)]
        pub const fn txbuf_bin0_tx_timestamp_h(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "TXBUF_BIN0TX_TIMESTAMP_H."]
        #[inline(always)]
        pub const fn set_txbuf_bin0_tx_timestamp_h(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsynTxbufBin0TxTimestampH {
        #[inline(always)]
        fn default() -> TsynTxbufBin0TxTimestampH {
            TsynTxbufBin0TxTimestampH(0)
        }
    }
    impl core::fmt::Debug for TsynTxbufBin0TxTimestampH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsynTxbufBin0TxTimestampH")
                .field(
                    "txbuf_bin0_tx_timestamp_h",
                    &self.txbuf_bin0_tx_timestamp_h(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsynTxbufBin0TxTimestampH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsynTxbufBin0TxTimestampH {{ txbuf_bin0_tx_timestamp_h: {=u32:?} }}",
                self.txbuf_bin0_tx_timestamp_h()
            )
        }
    }
    #[doc = "No description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TsynTxbufBin0TxTimestampL(pub u32);
    impl TsynTxbufBin0TxTimestampL {
        #[doc = "TXBUF_BIN0_TX_TIMESTAMP_L."]
        #[must_use]
        #[inline(always)]
        pub const fn txbuf_bin0_tx_timestamp_l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "TXBUF_BIN0_TX_TIMESTAMP_L."]
        #[inline(always)]
        pub const fn set_txbuf_bin0_tx_timestamp_l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TsynTxbufBin0TxTimestampL {
        #[inline(always)]
        fn default() -> TsynTxbufBin0TxTimestampL {
            TsynTxbufBin0TxTimestampL(0)
        }
    }
    impl core::fmt::Debug for TsynTxbufBin0TxTimestampL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TsynTxbufBin0TxTimestampL")
                .field(
                    "txbuf_bin0_tx_timestamp_l",
                    &self.txbuf_bin0_tx_timestamp_l(),
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TsynTxbufBin0TxTimestampL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "TsynTxbufBin0TxTimestampL {{ txbuf_bin0_tx_timestamp_l: {=u32:?} }}",
                self.txbuf_bin0_tx_timestamp_l()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsyntmr(pub u32);
    impl Tsyntmr {
        #[doc = "Period in ticks, ticks based on register HCLKDIV and host clock <sys_clk>."]
        #[must_use]
        #[inline(always)]
        pub const fn period(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Period in ticks, ticks based on register HCLKDIV and host clock <sys_clk>."]
        #[inline(always)]
        pub const fn set_period(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Tsyntmr {
        #[inline(always)]
        fn default() -> Tsyntmr {
            Tsyntmr(0)
        }
    }
    impl core::fmt::Debug for Tsyntmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tsyntmr")
                .field("period", &self.period())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tsyntmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tsyntmr {{ period: {=u32:?} }}", self.period())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txdata(pub u32);
    impl Txdata {
        #[doc = "TXBUF_BIN0_DATA_WORD0."]
        #[must_use]
        #[inline(always)]
        pub const fn txbuf_bin0_data_word0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "TXBUF_BIN0_DATA_WORD0."]
        #[inline(always)]
        pub const fn set_txbuf_bin0_data_word0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txdata {
        #[inline(always)]
        fn default() -> Txdata {
            Txdata(0)
        }
    }
    impl core::fmt::Debug for Txdata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txdata")
                .field("txbuf_bin0_data_word0", &self.txbuf_bin0_data_word0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txdata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Txdata {{ txbuf_bin0_data_word0: {=u32:?} }}",
                self.txbuf_bin0_data_word0()
            )
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txov(pub u32);
    impl Txov {
        #[doc = "Transmission overrun counter; increments on transmission when gate is closed; any write access will clear register to 0. TXOVi is only supported when TQC > i."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmission overrun counter; increments on transmission when gate is closed; any write access will clear register to 0. TXOVi is only supported when TQC > i."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Txov {
        #[inline(always)]
        fn default() -> Txov {
            Txov(0)
        }
    }
    impl core::fmt::Debug for Txov {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txov")
                .field("value", &self.value())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txov {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txov {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "no description available."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Txsel(pub u32);
    impl Txsel {
        #[doc = "CBS enable traffic queue n (n = 0 – 7). Returns 0 when n > TQC. Must be 0 when changing register IDSLPi."]
        #[must_use]
        #[inline(always)]
        pub const fn cbs_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "CBS enable traffic queue n (n = 0 – 7). Returns 0 when n > TQC. Must be 0 when changing register IDSLPi."]
        #[inline(always)]
        pub const fn set_cbs_en(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Txsel {
        #[inline(always)]
        fn default() -> Txsel {
            Txsel(0)
        }
    }
    impl core::fmt::Debug for Txsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Txsel")
                .field("cbs_en", &self.cbs_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Txsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Txsel {{ cbs_en: {=u16:?} }}", self.cbs_en())
        }
    }
}
